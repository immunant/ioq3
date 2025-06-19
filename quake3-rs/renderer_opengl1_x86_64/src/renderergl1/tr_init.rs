use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::intptr_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qgl_h::ClearDepthproc;
pub use crate::qgl_h::Color4fproc;
pub use crate::qgl_h::CullFaceproc;
pub use crate::qgl_h::DepthFuncproc;
pub use crate::qgl_h::DepthMaskproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::EnableClientStateproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::GetErrorproc;
pub use crate::qgl_h::GetIntegervproc;
pub use crate::qgl_h::GetStringiproc;
pub use crate::qgl_h::PolygonModeproc;
pub use crate::qgl_h::ReadPixelsproc;
pub use crate::qgl_h::ShadeModelproc;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::fontInfo_t;
pub use crate::src::qcommon::q_shared::glyphInfo_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::markFragment_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
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
pub use crate::src::renderercommon::tr_font::RE_RegisterFont;
pub use crate::src::renderercommon::tr_font::R_DoneFreeType;
pub use crate::src::renderercommon::tr_font::R_InitFreeType;
pub use crate::src::renderercommon::tr_image_jpg::RE_SaveJPG;
pub use crate::src::renderercommon::tr_image_jpg::RE_SaveJPGToBuffer;
pub use crate::src::renderercommon::tr_noise::R_NoiseInit;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::backEndData;
pub use crate::src::renderergl1::tr_backend::GL_SelectTexture;
pub use crate::src::renderergl1::tr_backend::GL_TexEnv;
pub use crate::src::renderergl1::tr_backend::RB_ShowImages;
pub use crate::src::renderergl1::tr_backend::RE_StretchRaw;
pub use crate::src::renderergl1::tr_backend::RE_UploadCinematic;
pub use crate::src::renderergl1::tr_bsp::RE_LoadWorldMap;
pub use crate::src::renderergl1::tr_bsp::RE_SetWorldVisData;
pub use crate::src::renderergl1::tr_bsp::R_GetEntityToken;
pub use crate::src::renderergl1::tr_cmds::RE_BeginFrame;
pub use crate::src::renderergl1::tr_cmds::RE_EndFrame;
pub use crate::src::renderergl1::tr_cmds::RE_SetColor;
pub use crate::src::renderergl1::tr_cmds::RE_StretchPic;
pub use crate::src::renderergl1::tr_cmds::RE_TakeVideoFrame;
pub use crate::src::renderergl1::tr_cmds::R_GetCommandBuffer;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_image::GL_TextureMode;
pub use crate::src::renderergl1::tr_image::RE_RegisterSkin;
pub use crate::src::renderergl1::tr_image::R_DeleteTextures;
pub use crate::src::renderergl1::tr_image::R_GammaCorrect;
pub use crate::src::renderergl1::tr_image::R_ImageList_f;
pub use crate::src::renderergl1::tr_image::R_InitFogTable;
pub use crate::src::renderergl1::tr_image::R_InitImages;
pub use crate::src::renderergl1::tr_image::R_InitSkins;
pub use crate::src::renderergl1::tr_image::R_SkinList_f;
pub use crate::src::renderergl1::tr_light::R_LightForPoint;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_marks::R_MarkFragments;
pub use crate::src::renderergl1::tr_model::RE_BeginRegistration;
pub use crate::src::renderergl1::tr_model::RE_RegisterModel;
pub use crate::src::renderergl1::tr_model::R_LerpTag;
pub use crate::src::renderergl1::tr_model::R_ModelBounds;
pub use crate::src::renderergl1::tr_model::R_ModelInit;
pub use crate::src::renderergl1::tr_model::R_Modellist_f;
pub use crate::src::renderergl1::tr_scene::RE_AddAdditiveLightToScene;
pub use crate::src::renderergl1::tr_scene::RE_AddLightToScene;
pub use crate::src::renderergl1::tr_scene::RE_AddPolyToScene;
pub use crate::src::renderergl1::tr_scene::RE_AddRefEntityToScene;
pub use crate::src::renderergl1::tr_scene::RE_ClearScene;
pub use crate::src::renderergl1::tr_scene::RE_RenderScene;
pub use crate::src::renderergl1::tr_scene::R_InitNextFrame;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shader::RE_RegisterShader;
pub use crate::src::renderergl1::tr_shader::RE_RegisterShaderNoMip;
pub use crate::src::renderergl1::tr_shader::R_InitShaders;
pub use crate::src::renderergl1::tr_shader::R_RemapShader;
pub use crate::src::renderergl1::tr_shader::R_ShaderList_f;
pub use crate::src::renderergl1::tr_world::R_inPVS;
pub use crate::src::sdl::sdl_glimp::qglActiveTextureARB;
pub use crate::src::sdl::sdl_glimp::qglClearDepth;
pub use crate::src::sdl::sdl_glimp::qglColor4f;
pub use crate::src::sdl::sdl_glimp::qglCullFace;
pub use crate::src::sdl::sdl_glimp::qglDepthFunc;
pub use crate::src::sdl::sdl_glimp::qglDepthMask;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglEnable;
pub use crate::src::sdl::sdl_glimp::qglEnableClientState;
pub use crate::src::sdl::sdl_glimp::qglGetError;
pub use crate::src::sdl::sdl_glimp::qglGetIntegerv;
pub use crate::src::sdl::sdl_glimp::qglGetStringi;
pub use crate::src::sdl::sdl_glimp::qglLockArraysEXT;
pub use crate::src::sdl::sdl_glimp::qglPolygonMode;
pub use crate::src::sdl::sdl_glimp::qglReadPixels;
pub use crate::src::sdl::sdl_glimp::qglShadeModel;
pub use crate::src::sdl::sdl_glimp::GLimp_Init;
pub use crate::src::sdl::sdl_glimp::GLimp_Minimize;
pub use crate::src::sdl::sdl_glimp::GLimp_Shutdown;

pub use crate::stdlib::GLboolean;
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
pub use crate::tr_local_h::backEndData_t;
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
pub use crate::tr_local_h::drawSurf_t;
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
pub use crate::tr_local_h::screenshotCommand_t;
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
pub use crate::tr_local_h::surfaceType_t;
pub use crate::tr_local_h::texCoordGen_t;
pub use crate::tr_local_h::texModInfo_t;
pub use crate::tr_local_h::texMod_t;
pub use crate::tr_local_h::textureBundle_t;
pub use crate::tr_local_h::trGlobals_t;
pub use crate::tr_local_h::trRefEntity_t;
pub use crate::tr_local_h::trRefdef_t;
pub use crate::tr_local_h::videoFrameCommand_t;
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
pub use crate::tr_public_h::refexport_t;
pub use crate::tr_public_h::refimport_t;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
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

/*
** R_GetModeInfo
*/

pub type vidmode_t = vidmode_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vidmode_s {
    pub description: *const libc::c_char,
    pub width: i32,
    pub height: i32,
    pub pixelAspect: f32,
}
// pixel width / height
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
// tr_init.c -- functions that are not called every frame
#[no_mangle]

pub static mut glConfig: crate::tr_types_h::glconfig_t = crate::tr_types_h::glconfig_t {
    renderer_string: [0; 1024],
    vendor_string: [0; 1024],
    version_string: [0; 1024],
    extensions_string: [0; 8192],
    maxTextureSize: 0,
    numTextureUnits: 0,
    colorBits: 0,
    depthBits: 0,
    stencilBits: 0,
    driverType: crate::tr_types_h::GLDRV_ICD,
    hardwareType: crate::tr_types_h::GLHW_GENERIC,
    deviceSupportsGamma: crate::src::qcommon::q_shared::qfalse,
    textureCompression: crate::tr_types_h::TC_NONE,
    textureEnvAddAvailable: crate::src::qcommon::q_shared::qfalse,
    vidWidth: 0,
    vidHeight: 0,
    windowAspect: 0.,
    displayFrequency: 0,
    isFullscreen: crate::src::qcommon::q_shared::qfalse,
    stereoEnabled: crate::src::qcommon::q_shared::qfalse,
    smpActive: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut textureFilterAnisotropic: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut maxAnisotropy: i32 = 0 as i32;
#[no_mangle]

pub static mut displayAspect: f32 = 0.0f32;
#[no_mangle]

pub static mut glState: crate::tr_local_h::glstate_t = crate::tr_local_h::glstate_t {
    currenttextures: [0; 2],
    currenttmu: 0,
    finishCalled: crate::src::qcommon::q_shared::qfalse,
    texEnv: [0; 2],
    faceCulling: 0,
    glStateBits: 0,
};
#[no_mangle]

pub static mut com_altivec: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_flareSize: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_flareFade: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_flareCoeff: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_railWidth: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_railCoreWidth: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_railSegmentLength: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ignoreFastPath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_verbose: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ignore: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_displayRefresh: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_detailTextures: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_znear: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_zproj: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_stereoSeparation: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_skipBackEnd: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_stereoEnabled: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_anaglyphMode: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_greyscale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ignorehwgamma: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_measureOverdraw: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_inGameVideo: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_fastsky: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_drawSun: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_dynamiclight: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_dlightBacks: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_lodbias: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_lodscale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_norefresh: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_drawentities: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_drawworld: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_speeds: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_fullbright: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_novis: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_nocull: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_facePlaneCull: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_showcluster: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_nocurves: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_allowExtensions: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_compressed_textures: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_multitexture: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_compiled_vertex_array: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_texture_env_add: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_texture_filter_anisotropic: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_max_anisotropy: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ignoreGLErrors: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_logFile: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_stencilbits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_depthbits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_colorbits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_primitives: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_texturebits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ext_multisample: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_drawBuffer: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_lightmap: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_vertexLight: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_uiFullScreen: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_shadows: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_flares: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_mode: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_nobind: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_singleShader: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_roundImagesDown: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_colorMipLevels: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_picmip: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_showtris: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_showsky: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_shownormals: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_finish: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_clear: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_swapInterval: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_textureMode: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_offsetFactor: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_offsetUnits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_gamma: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_intensity: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_lockpvs: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_noportals: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_portalOnly: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_subdivisions: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_lodCurveError: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_fullscreen: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_noborder: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_customwidth: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_customheight: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_customPixelAspect: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_overBrightBits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_mapOverBrightBits: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_debugSurface: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_simpleMipMaps: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_showImages: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_ambientScale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_directedScale: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_debugLight: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_debugSort: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_printShaders: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_saveFontData: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_marksOnTriangleMeshes: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_aviMotionJpegQuality: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_screenshotJpegQuality: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut r_maxpolys: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut max_polys: i32 = 0;
#[no_mangle]

pub static mut r_maxpolyverts: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut max_polyverts: i32 = 0;
/*
** InitOpenGL
**
** This function is responsible for initializing a valid OpenGL subsystem.  This
** is done by calling GLimp_Init (which gives us a working OGL subsystem) then
** setting variables, checking GL constants, and reporting the gfx system config
** to the user.
*/

unsafe extern "C" fn InitOpenGL() {
    //
    // initialize OS specific portions of the renderer
    //
    // GLimp_Init directly or indirectly references the following cvars:
    //		- r_fullscreen
    //		- r_mode
    //		- r_(color|depth|stencil)bits
    //		- r_ignorehwgamma
    //		- r_gamma
    //
    if glConfig.vidWidth == 0 as i32 {
        let mut temp: crate::stdlib::GLint = 0;
        crate::src::sdl::sdl_glimp::GLimp_Init(crate::src::qcommon::q_shared::qtrue);
        // OpenGL driver constants
        crate::src::sdl::sdl_glimp::qglGetIntegerv.expect("non-null function pointer")(
            0xd33 as i32 as crate::stdlib::GLenum,
            &mut temp,
        );
        glConfig.maxTextureSize = temp;
        // stubbed or broken drivers may have reported 0...
        if glConfig.maxTextureSize <= 0 as i32 {
            glConfig.maxTextureSize = 0 as i32
        }
    }
    // set default state
    GL_SetDefaultState();
}
/*
==================
GL_CheckErrors
==================
*/
#[no_mangle]

pub unsafe extern "C" fn GL_CheckErrors() {
    let mut err: i32 = 0;
    let mut s: [libc::c_char; 64] = [0; 64];
    err = crate::src::sdl::sdl_glimp::qglGetError.expect("non-null function pointer")()
        as i32;
    if err == 0 as i32 {
        return;
    }
    if (*r_ignoreGLErrors).integer != 0 {
        return;
    }
    match err {
        1280 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_INVALID_ENUM\x00" as *const u8 as *const libc::c_char,
            );
        }
        1281 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_INVALID_VALUE\x00" as *const u8 as *const libc::c_char,
            );
        }
        1282 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_INVALID_OPERATION\x00" as *const u8 as *const libc::c_char,
            );
        }
        1283 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_STACK_OVERFLOW\x00" as *const u8 as *const libc::c_char,
            );
        }
        1284 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_STACK_UNDERFLOW\x00" as *const u8 as *const libc::c_char,
            );
        }
        1285 => {
            ::libc::strcpy(
                s.as_mut_ptr(),
                b"GL_OUT_OF_MEMORY\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            crate::src::qcommon::q_shared::Com_sprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                err,
            );
        }
    }
    crate::src::renderergl1::tr_main::ri
        .Error
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::ERR_FATAL as i32,
        b"GL_CheckErrors: %s\x00" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
    );
}
#[no_mangle]

pub static mut r_vidModes: [vidmode_t; 12] = [
    {
        let mut init = vidmode_s {
            description: b"Mode  0: 320x240\x00" as *const u8 as *const libc::c_char,
            width: 320 as i32,
            height: 240 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  1: 400x300\x00" as *const u8 as *const libc::c_char,
            width: 400 as i32,
            height: 300 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  2: 512x384\x00" as *const u8 as *const libc::c_char,
            width: 512 as i32,
            height: 384 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  3: 640x480\x00" as *const u8 as *const libc::c_char,
            width: 640 as i32,
            height: 480 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  4: 800x600\x00" as *const u8 as *const libc::c_char,
            width: 800 as i32,
            height: 600 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  5: 960x720\x00" as *const u8 as *const libc::c_char,
            width: 960 as i32,
            height: 720 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  6: 1024x768\x00" as *const u8 as *const libc::c_char,
            width: 1024 as i32,
            height: 768 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  7: 1152x864\x00" as *const u8 as *const libc::c_char,
            width: 1152 as i32,
            height: 864 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  8: 1280x1024\x00" as *const u8 as *const libc::c_char,
            width: 1280 as i32,
            height: 1024 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode  9: 1600x1200\x00" as *const u8 as *const libc::c_char,
            width: 1600 as i32,
            height: 1200 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode 10: 2048x1536\x00" as *const u8 as *const libc::c_char,
            width: 2048 as i32,
            height: 1536 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
    {
        let mut init = vidmode_s {
            description: b"Mode 11: 856x480 (wide)\x00" as *const u8 as *const libc::c_char,
            width: 856 as i32,
            height: 480 as i32,
            pixelAspect: 1 as i32 as f32,
        };
        init
    },
];
// Initialized in run_static_initializers

static mut s_numVidModes: i32 = 0;
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
// for color, lightmap, diffuse, and specular
// normals are swizzled, deluxe are not
// game path, including extension
// source image
// after power of two and picmip but not including clamp to MAX_TEXTURE_SIZE
// gl texture binding
// for texture usage in frame statistics
// only needed for voodoo2
// any change in the LIGHTMAP_* defines here MUST be reflected in
// R_FindShader() in tr_bsp.c
// shader is for 2D rendering
// pre-lit triangle models
// outside of TR since it shouldn't be cleared during ref re-init
// These variables should live inside glConfig but can't because of
// compatibility issues to the original ID vms.  If you release a stand-alone
// game and your mod uses tr_types.h from this build you can safely move them
// to the glconfig_t struct.
//
// cvars
//
// number of desired stencil bits
// number of desired depth bits
// number of desired color bits, only relevant for fullscreen
// number of desired texture bits
// 0 = use framebuffer depth
// 16 = use 16-bit textures
// 32 = use 32-bit textures
// all else = error
// video mode
// overrides hardware gamma capabilities
// global enable/disable of OpenGL extensions
// these control use of specific extensions
#[no_mangle]

pub unsafe extern "C" fn R_GetModeInfo(
    mut width: *mut i32,
    mut height: *mut i32,
    mut windowAspect: *mut f32,
    mut mode: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut vm: *mut vidmode_t = 0 as *mut vidmode_t;
    let mut pixelAspect: f32 = 0.;
    if mode < -(1 as i32) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if mode >= s_numVidModes {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if mode == -(1 as i32) {
        *width = (*r_customwidth).integer;
        *height = (*r_customheight).integer;
        pixelAspect = (*r_customPixelAspect).value
    } else {
        vm = &mut *r_vidModes.as_mut_ptr().offset(mode as isize) as *mut vidmode_t;
        *width = (*vm).width;
        *height = (*vm).height;
        pixelAspect = (*vm).pixelAspect
    }
    *windowAspect = *width as f32 / (*height as f32 * pixelAspect);
    return crate::src::qcommon::q_shared::qtrue;
}
/*
** R_ModeList_f
*/

unsafe extern "C" fn R_ModeList_f() {
    let mut i: i32 = 0;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as i32;
    while i < s_numVidModes {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            r_vidModes[i as usize].description,
        );
        i += 1
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==============================================================================

                        SCREEN SHOTS

NOTE TTimo
some thoughts about the screenshots system:
screenshots get written in fs_homepath + fs_gamedir
vanilla q3 .. baseq3/screenshots/ *.tga
team arena .. missionpack/screenshots/ *.tga

two commands: "screenshot" and "screenshotJPEG"
we use statics to store a count and start writing the first screenshot/screenshot????.tga (.jpg) available
(with FS_FileExists / FS_FOpenFileWrite calls)
FIXME: the statics don't get a reinit between fs_game changes

==============================================================================
*/
/*
==================
RB_ReadPixels

Reads an image but takes care of alignment issues for reading RGB images.

Reads a minimum offset for where the RGB data starts in the image from
integer stored at pointer offset. When the function has returned the actual
offset was written back to address offset. This address will always have an
alignment of packAlign to ensure efficient copying.

Stores the length of padding after a line of pixels to address padlen

Return value must be freed with ri.Hunk_FreeTempMemory()
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ReadPixels(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut offset: *mut crate::stddef_h::size_t,
    mut padlen: *mut i32,
) -> *mut crate::src::qcommon::q_shared::byte {
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bufstart: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut padwidth: i32 = 0;
    let mut linelen: i32 = 0;
    let mut packAlign: crate::stdlib::GLint = 0;
    crate::src::sdl::sdl_glimp::qglGetIntegerv.expect("non-null function pointer")(
        0xd05 as i32 as crate::stdlib::GLenum,
        &mut packAlign,
    );
    linelen = width * 3 as i32;
    padwidth = linelen + packAlign - 1 as i32 & !(packAlign - 1 as i32);
    // Allocate a few more bytes so that we can choose an alignment we like
    buffer = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(
        ((padwidth * height) as libc::c_ulong)
            .wrapping_add(*offset)
            .wrapping_add(packAlign as libc::c_ulong)
            .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
    ) as *mut crate::src::qcommon::q_shared::byte;
    bufstart = ((buffer as crate::stdlib::intptr_t as libc::c_ulong).wrapping_add(*offset)
        as crate::stdlib::intptr_t
        + packAlign as libc::c_long
        - 1 as i32 as libc::c_long
        & !(packAlign - 1 as i32) as libc::c_long) as *mut libc::c_void
        as *mut crate::src::qcommon::q_shared::byte;
    crate::src::sdl::sdl_glimp::qglReadPixels.expect("non-null function pointer")(
        x,
        y,
        width,
        height,
        0x1907 as i32 as crate::stdlib::GLenum,
        0x1401 as i32 as crate::stdlib::GLenum,
        bufstart as *mut libc::c_void,
    );
    *offset = bufstart.offset_from(buffer) as libc::c_long as crate::stddef_h::size_t;
    *padlen = padwidth - linelen;
    return buffer;
}
/*
==================
RB_TakeScreenshot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_TakeScreenshot(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut fileName: *mut libc::c_char,
) {
    let mut allbuf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte; // uncompressed type
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte; // pixel size
    let mut srcptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut destptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut endline: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut endmem: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut temp: crate::src::qcommon::q_shared::byte = 0;
    let mut linelen: i32 = 0;
    let mut padlen: i32 = 0;
    let mut offset: crate::stddef_h::size_t = 18 as i32 as crate::stddef_h::size_t;
    let mut memcount: crate::stddef_h::size_t = 0;
    allbuf = RB_ReadPixels(x, y, width, height, &mut offset, &mut padlen);
    buffer = allbuf
        .offset(offset as isize)
        .offset(-(18 as i32 as isize));
    crate::stdlib::memset(
        buffer as *mut libc::c_void,
        0 as i32,
        18 as i32 as libc::c_ulong,
    );
    *buffer.offset(2 as i32 as isize) =
        2 as i32 as crate::src::qcommon::q_shared::byte;
    *buffer.offset(12 as i32 as isize) =
        (width & 255 as i32) as crate::src::qcommon::q_shared::byte;
    *buffer.offset(13 as i32 as isize) =
        (width >> 8 as i32) as crate::src::qcommon::q_shared::byte;
    *buffer.offset(14 as i32 as isize) =
        (height & 255 as i32) as crate::src::qcommon::q_shared::byte;
    *buffer.offset(15 as i32 as isize) =
        (height >> 8 as i32) as crate::src::qcommon::q_shared::byte;
    *buffer.offset(16 as i32 as isize) =
        24 as i32 as crate::src::qcommon::q_shared::byte;
    // swap rgb to bgr and remove padding from line endings
    linelen = width * 3 as i32;
    destptr = allbuf.offset(offset as isize);
    srcptr = destptr;
    endmem = srcptr.offset(((linelen + padlen) * height) as isize);
    while srcptr < endmem {
        endline = srcptr.offset(linelen as isize);
        while srcptr < endline {
            temp = *srcptr.offset(0 as i32 as isize);
            let fresh0 = destptr;
            destptr = destptr.offset(1);
            *fresh0 = *srcptr.offset(2 as i32 as isize);
            let fresh1 = destptr;
            destptr = destptr.offset(1);
            *fresh1 = *srcptr.offset(1 as i32 as isize);
            let fresh2 = destptr;
            destptr = destptr.offset(1);
            *fresh2 = temp;
            srcptr = srcptr.offset(3 as i32 as isize)
        }
        // Skip the pad
        srcptr = srcptr.offset(padlen as isize)
    }
    memcount = (linelen * height) as crate::stddef_h::size_t;
    // gamma correct
    if glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::renderergl1::tr_image::R_GammaCorrect(
            allbuf.offset(offset as isize),
            memcount as i32,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .FS_WriteFile
        .expect("non-null function pointer")(
        fileName,
        buffer as *const libc::c_void,
        memcount.wrapping_add(18 as i32 as libc::c_ulong) as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(allbuf as *mut libc::c_void);
}
/*
==================
RB_TakeScreenshotJPEG
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_TakeScreenshotJPEG(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut fileName: *mut libc::c_char,
) {
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut offset: crate::stddef_h::size_t = 0 as i32 as crate::stddef_h::size_t;
    let mut memcount: crate::stddef_h::size_t = 0;
    let mut padlen: i32 = 0;
    buffer = RB_ReadPixels(x, y, width, height, &mut offset, &mut padlen);
    memcount = ((width * 3 as i32 + padlen) * height) as crate::stddef_h::size_t;
    // gamma correct
    if glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::renderergl1::tr_image::R_GammaCorrect(
            buffer.offset(offset as isize),
            memcount as i32,
        );
    }
    crate::src::renderercommon::tr_image_jpg::RE_SaveJPG(
        fileName,
        (*r_screenshotJpegQuality).integer,
        width,
        height,
        buffer.offset(offset as isize),
        padlen,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(buffer as *mut libc::c_void);
}
/*
==================
RB_TakeScreenshotCmd
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_TakeScreenshotCmd(
    mut data: *const libc::c_void,
) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::screenshotCommand_t =
        0 as *const crate::tr_local_h::screenshotCommand_t;
    cmd = data as *const crate::tr_local_h::screenshotCommand_t;
    if (*cmd).jpeg as u64 != 0 {
        RB_TakeScreenshotJPEG(
            (*cmd).x,
            (*cmd).y,
            (*cmd).width,
            (*cmd).height,
            (*cmd).fileName,
        );
    } else {
        RB_TakeScreenshot(
            (*cmd).x,
            (*cmd).y,
            (*cmd).width,
            (*cmd).height,
            (*cmd).fileName,
        );
    }
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
/*
==================
R_TakeScreenshot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_TakeScreenshot(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut name: *mut libc::c_char,
    mut jpeg: crate::src::qcommon::q_shared::qboolean,
) {
    static mut fileName: [libc::c_char; 4096] = [0; 4096]; // bad things if two screenshots per frame?
    let mut cmd: *mut crate::tr_local_h::screenshotCommand_t =
        0 as *mut crate::tr_local_h::screenshotCommand_t;
    cmd = crate::src::renderergl1::tr_cmds::R_GetCommandBuffer(::std::mem::size_of::<
        crate::tr_local_h::screenshotCommand_t,
    >() as libc::c_ulong
        as i32) as *mut crate::tr_local_h::screenshotCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = crate::tr_local_h::RC_SCREENSHOT as i32;
    (*cmd).x = x;
    (*cmd).y = y;
    (*cmd).width = width;
    (*cmd).height = height;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fileName.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    (*cmd).fileName = fileName.as_mut_ptr();
    (*cmd).jpeg = jpeg;
}
/*
==================
R_ScreenshotFilename
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ScreenshotFilename(
    mut lastNumber: i32,
    mut fileName: *mut libc::c_char,
) {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    if lastNumber < 0 as i32 || lastNumber > 9999 as i32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            fileName,
            4096 as i32,
            b"screenshots/shot9999.tga\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    a = lastNumber / 1000 as i32;
    lastNumber -= a * 1000 as i32;
    b = lastNumber / 100 as i32;
    lastNumber -= b * 100 as i32;
    c = lastNumber / 10 as i32;
    lastNumber -= c * 10 as i32;
    d = lastNumber;
    crate::src::qcommon::q_shared::Com_sprintf(
        fileName,
        4096 as i32,
        b"screenshots/shot%i%i%i%i.tga\x00" as *const u8 as *const libc::c_char,
        a,
        b,
        c,
        d,
    );
}
/*
==================
R_ScreenshotFilename
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ScreenshotFilenameJPEG(
    mut lastNumber: i32,
    mut fileName: *mut libc::c_char,
) {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    if lastNumber < 0 as i32 || lastNumber > 9999 as i32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            fileName,
            4096 as i32,
            b"screenshots/shot9999.jpg\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    a = lastNumber / 1000 as i32;
    lastNumber -= a * 1000 as i32;
    b = lastNumber / 100 as i32;
    lastNumber -= b * 100 as i32;
    c = lastNumber / 10 as i32;
    lastNumber -= c * 10 as i32;
    d = lastNumber;
    crate::src::qcommon::q_shared::Com_sprintf(
        fileName,
        4096 as i32,
        b"screenshots/shot%i%i%i%i.jpg\x00" as *const u8 as *const libc::c_char,
        a,
        b,
        c,
        d,
    );
}
/*
====================
R_LevelShot

levelshots are specialized 128*128 thumbnails for
the menu system, sampled down from full screen distorted images
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LevelShot() {
    let mut checkname: [libc::c_char; 4096] = [0; 4096]; // uncompressed type
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte; // pixel size
    let mut source: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut allsource: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut src: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut dst: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut offset: crate::stddef_h::size_t = 0 as i32 as crate::stddef_h::size_t;
    let mut padlen: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut r: i32 = 0;
    let mut g: i32 = 0;
    let mut b: i32 = 0;
    let mut xScale: f32 = 0.;
    let mut yScale: f32 = 0.;
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    crate::src::qcommon::q_shared::Com_sprintf(
        checkname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
        (*crate::src::renderergl1::tr_main::tr.world)
            .baseName
            .as_mut_ptr(),
    );
    allsource = RB_ReadPixels(
        0 as i32,
        0 as i32,
        glConfig.vidWidth,
        glConfig.vidHeight,
        &mut offset,
        &mut padlen,
    );
    source = allsource.offset(offset as isize);
    buffer = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(
        128 as i32 * 128 as i32 * 3 as i32 + 18 as i32,
    ) as *mut crate::src::qcommon::q_shared::byte;
    crate::stdlib::memset(
        buffer as *mut libc::c_void,
        0 as i32,
        18 as i32 as libc::c_ulong,
    );
    *buffer.offset(2 as i32 as isize) =
        2 as i32 as crate::src::qcommon::q_shared::byte;
    *buffer.offset(12 as i32 as isize) =
        128 as i32 as crate::src::qcommon::q_shared::byte;
    *buffer.offset(14 as i32 as isize) =
        128 as i32 as crate::src::qcommon::q_shared::byte;
    *buffer.offset(16 as i32 as isize) =
        24 as i32 as crate::src::qcommon::q_shared::byte;
    // resample from source
    xScale = glConfig.vidWidth as f32 / 512.0f32;
    yScale = glConfig.vidHeight as f32 / 384.0f32;
    y = 0 as i32;
    while y < 128 as i32 {
        x = 0 as i32;
        while x < 128 as i32 {
            b = 0 as i32;
            g = b;
            r = g;
            yy = 0 as i32;
            while yy < 3 as i32 {
                xx = 0 as i32;
                while xx < 4 as i32 {
                    src = source
                        .offset(
                            ((3 as i32 * glConfig.vidWidth + padlen)
                                * ((y * 3 as i32 + yy) as f32 * yScale)
                                    as i32) as isize,
                        )
                        .offset(
                            (3 as i32
                                * ((x * 4 as i32 + xx) as f32 * xScale)
                                    as i32) as isize,
                        );
                    r += *src.offset(0 as i32 as isize) as i32;
                    g += *src.offset(1 as i32 as isize) as i32;
                    b += *src.offset(2 as i32 as isize) as i32;
                    xx += 1
                }
                yy += 1
            }
            dst = buffer
                .offset(18 as i32 as isize)
                .offset((3 as i32 * (y * 128 as i32 + x)) as isize);
            *dst.offset(0 as i32 as isize) =
                (b / 12 as i32) as crate::src::qcommon::q_shared::byte;
            *dst.offset(1 as i32 as isize) =
                (g / 12 as i32) as crate::src::qcommon::q_shared::byte;
            *dst.offset(2 as i32 as isize) =
                (r / 12 as i32) as crate::src::qcommon::q_shared::byte;
            x += 1
        }
        y += 1
    }
    // gamma correct
    if glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::renderergl1::tr_image::R_GammaCorrect(
            buffer.offset(18 as i32 as isize),
            128 as i32 * 128 as i32 * 3 as i32,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .FS_WriteFile
        .expect("non-null function pointer")(
        checkname.as_mut_ptr(),
        buffer as *const libc::c_void,
        128 as i32 * 128 as i32 * 3 as i32 + 18 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(buffer as *mut libc::c_void);
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(allsource as *mut libc::c_void);
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"Wrote %s\n\x00" as *const u8 as *const libc::c_char,
        checkname.as_mut_ptr(),
    );
}
/*
==================
R_ScreenShot_f

screenshot
screenshot [silent]
screenshot [levelshot]
screenshot [filename]

Doesn't print the pacifier message if there is a second arg
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ScreenShot_f() {
    let mut checkname: [libc::c_char; 4096] = [0; 4096];
    static mut lastNumber: i32 = -(1 as i32);
    let mut silent: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if ::libc::strcmp(
        crate::src::renderergl1::tr_main::ri
            .Cmd_Argv
            .expect("non-null function pointer")(1 as i32),
        b"levelshot\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        R_LevelShot();
        return;
    }
    if ::libc::strcmp(
        crate::src::renderergl1::tr_main::ri
            .Cmd_Argv
            .expect("non-null function pointer")(1 as i32),
        b"silent\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        silent = crate::src::qcommon::q_shared::qtrue
    } else {
        silent = crate::src::qcommon::q_shared::qfalse
    }
    if crate::src::renderergl1::tr_main::ri
        .Cmd_Argc
        .expect("non-null function pointer")()
        == 2 as i32
        && silent as u64 == 0
    {
        // explicit filename
        crate::src::qcommon::q_shared::Com_sprintf(
            checkname.as_mut_ptr(),
            4096 as i32,
            b"screenshots/%s.tga\x00" as *const u8 as *const libc::c_char,
            crate::src::renderergl1::tr_main::ri
                .Cmd_Argv
                .expect("non-null function pointer")(1 as i32),
        );
    } else {
        // scan for a free filename
        // if we have saved a previous screenshot, don't scan
        // again, because recording demo avis can involve
        // thousands of shots
        if lastNumber == -(1 as i32) {
            lastNumber = 0 as i32
        }
        // scan for a free number
        while lastNumber <= 9999 as i32 {
            R_ScreenshotFilename(lastNumber, checkname.as_mut_ptr());
            if crate::src::renderergl1::tr_main::ri
                .FS_FileExists
                .expect("non-null function pointer")(checkname.as_mut_ptr()) as u64
                == 0
            {
                break;
            }
            lastNumber += 1
        }
        if lastNumber >= 9999 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ScreenShot: Couldn\'t create a file\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        lastNumber += 1
    }
    R_TakeScreenshot(
        0 as i32,
        0 as i32,
        glConfig.vidWidth,
        glConfig.vidHeight,
        checkname.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
    );
    if silent as u64 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Wrote %s\n\x00" as *const u8 as *const libc::c_char,
            checkname.as_mut_ptr(),
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn R_ScreenShotJPEG_f() {
    let mut checkname: [libc::c_char; 4096] = [0; 4096];
    static mut lastNumber: i32 = -(1 as i32);
    let mut silent: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if ::libc::strcmp(
        crate::src::renderergl1::tr_main::ri
            .Cmd_Argv
            .expect("non-null function pointer")(1 as i32),
        b"levelshot\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        R_LevelShot();
        return;
    }
    if ::libc::strcmp(
        crate::src::renderergl1::tr_main::ri
            .Cmd_Argv
            .expect("non-null function pointer")(1 as i32),
        b"silent\x00" as *const u8 as *const libc::c_char,
    ) == 0
    {
        silent = crate::src::qcommon::q_shared::qtrue
    } else {
        silent = crate::src::qcommon::q_shared::qfalse
    }
    if crate::src::renderergl1::tr_main::ri
        .Cmd_Argc
        .expect("non-null function pointer")()
        == 2 as i32
        && silent as u64 == 0
    {
        // explicit filename
        crate::src::qcommon::q_shared::Com_sprintf(
            checkname.as_mut_ptr(),
            4096 as i32,
            b"screenshots/%s.jpg\x00" as *const u8 as *const libc::c_char,
            crate::src::renderergl1::tr_main::ri
                .Cmd_Argv
                .expect("non-null function pointer")(1 as i32),
        );
    } else {
        // scan for a free filename
        // if we have saved a previous screenshot, don't scan
        // again, because recording demo avis can involve
        // thousands of shots
        if lastNumber == -(1 as i32) {
            lastNumber = 0 as i32
        }
        // scan for a free number
        while lastNumber <= 9999 as i32 {
            R_ScreenshotFilenameJPEG(lastNumber, checkname.as_mut_ptr());
            if crate::src::renderergl1::tr_main::ri
                .FS_FileExists
                .expect("non-null function pointer")(checkname.as_mut_ptr()) as u64
                == 0
            {
                break;
            }
            lastNumber += 1
        }
        if lastNumber == 10000 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"ScreenShot: Couldn\'t create a file\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        lastNumber += 1
    }
    R_TakeScreenshot(
        0 as i32,
        0 as i32,
        glConfig.vidWidth,
        glConfig.vidHeight,
        checkname.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
    );
    if silent as u64 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Wrote %s\n\x00" as *const u8 as *const libc::c_char,
            checkname.as_mut_ptr(),
        );
    };
}
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
//============================================================================
/*
==================
RB_TakeVideoFrameCmd
==================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_TakeVideoFrameCmd(
    mut data: *const libc::c_void,
) -> *const libc::c_void {
    let mut cmd: *const crate::tr_local_h::videoFrameCommand_t =
        0 as *const crate::tr_local_h::videoFrameCommand_t;
    let mut cBuf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut memcount: crate::stddef_h::size_t = 0;
    let mut linelen: crate::stddef_h::size_t = 0;
    let mut padwidth: i32 = 0;
    let mut avipadwidth: i32 = 0;
    let mut padlen: i32 = 0;
    let mut avipadlen: i32 = 0;
    let mut packAlign: crate::stdlib::GLint = 0;
    cmd = data as *const crate::tr_local_h::videoFrameCommand_t;
    crate::src::sdl::sdl_glimp::qglGetIntegerv.expect("non-null function pointer")(
        0xd05 as i32 as crate::stdlib::GLenum,
        &mut packAlign,
    );
    linelen = ((*cmd).width * 3 as i32) as crate::stddef_h::size_t;
    // Alignment stuff for glReadPixels
    padwidth = (linelen
        .wrapping_add(packAlign as libc::c_ulong)
        .wrapping_sub(1 as i32 as libc::c_ulong)
        & !(packAlign - 1 as i32) as libc::c_ulong) as i32;
    padlen = (padwidth as libc::c_ulong).wrapping_sub(linelen) as i32;
    // AVI line padding
    avipadwidth = (linelen
        .wrapping_add(4 as i32 as libc::c_ulong)
        .wrapping_sub(1 as i32 as libc::c_ulong)
        & !(4 as i32 - 1 as i32) as libc::c_ulong) as i32;
    avipadlen = (avipadwidth as libc::c_ulong).wrapping_sub(linelen) as i32;
    cBuf = ((*cmd).captureBuffer as crate::stdlib::intptr_t + packAlign as libc::c_long
        - 1 as i32 as libc::c_long
        & !(packAlign - 1 as i32) as libc::c_long) as *mut libc::c_void
        as *mut crate::src::qcommon::q_shared::byte;
    crate::src::sdl::sdl_glimp::qglReadPixels.expect("non-null function pointer")(
        0 as i32,
        0 as i32,
        (*cmd).width,
        (*cmd).height,
        0x1907 as i32 as crate::stdlib::GLenum,
        0x1401 as i32 as crate::stdlib::GLenum,
        cBuf as *mut libc::c_void,
    );
    memcount = (padwidth * (*cmd).height) as crate::stddef_h::size_t;
    // gamma correct
    if glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::renderergl1::tr_image::R_GammaCorrect(cBuf, memcount as i32);
    }
    if (*cmd).motionJpeg as u64 != 0 {
        memcount = crate::src::renderercommon::tr_image_jpg::RE_SaveJPGToBuffer(
            (*cmd).encodeBuffer,
            linelen.wrapping_mul((*cmd).height as libc::c_ulong),
            (*r_aviMotionJpegQuality).integer,
            (*cmd).width,
            (*cmd).height,
            cBuf,
            padlen,
        );
        crate::src::renderergl1::tr_main::ri
            .CL_WriteAVIVideoFrame
            .expect("non-null function pointer")(
            (*cmd).encodeBuffer, memcount as i32
        );
    } else {
        let mut lineend: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut memend: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut srcptr: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        let mut destptr: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        srcptr = cBuf;
        destptr = (*cmd).encodeBuffer;
        memend = srcptr.offset(memcount as isize);
        // swap R and B and remove line paddings
        while srcptr < memend {
            lineend = srcptr.offset(linelen as isize);
            while srcptr < lineend {
                let fresh3 = destptr;
                destptr = destptr.offset(1);
                *fresh3 = *srcptr.offset(2 as i32 as isize);
                let fresh4 = destptr;
                destptr = destptr.offset(1);
                *fresh4 = *srcptr.offset(1 as i32 as isize);
                let fresh5 = destptr;
                destptr = destptr.offset(1);
                *fresh5 = *srcptr.offset(0 as i32 as isize);
                srcptr = srcptr.offset(3 as i32 as isize)
            }
            crate::stdlib::memset(
                destptr as *mut libc::c_void,
                '\u{0}' as i32,
                avipadlen as libc::c_ulong,
            );
            destptr = destptr.offset(avipadlen as isize);
            srcptr = srcptr.offset(padlen as isize)
        }
        crate::src::renderergl1::tr_main::ri
            .CL_WriteAVIVideoFrame
            .expect("non-null function pointer")(
            (*cmd).encodeBuffer, avipadwidth * (*cmd).height
        );
    }
    return cmd.offset(1 as i32 as isize) as *const libc::c_void;
}
//============================================================================
/*
** GL_SetDefaultState
*/
#[no_mangle]

pub unsafe extern "C" fn GL_SetDefaultState() {
    crate::src::sdl::sdl_glimp::qglClearDepth.expect("non-null function pointer")(
        1.0f32 as crate::stdlib::GLclampd,
    );
    crate::src::sdl::sdl_glimp::qglCullFace.expect("non-null function pointer")(
        0x404 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglColor4f.expect("non-null function pointer")(
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
        1 as i32 as crate::stdlib::GLfloat,
    );
    // initialize downstream texture unit if we're running
    // in a multitexture environment
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() {
        crate::src::renderergl1::tr_backend::GL_SelectTexture(1 as i32);
        crate::src::renderergl1::tr_image::GL_TextureMode((*r_textureMode).string);
        crate::src::renderergl1::tr_backend::GL_TexEnv(0x2100 as i32);
        crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
            0xde1 as i32 as crate::stdlib::GLenum,
        );
        crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as i32);
    }
    crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
        0xde1 as i32 as crate::stdlib::GLenum,
    );
    crate::src::renderergl1::tr_image::GL_TextureMode((*r_textureMode).string);
    crate::src::renderergl1::tr_backend::GL_TexEnv(0x2100 as i32);
    crate::src::sdl::sdl_glimp::qglShadeModel.expect("non-null function pointer")(
        0x1d01 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDepthFunc.expect("non-null function pointer")(
        0x203 as i32 as crate::stdlib::GLenum,
    );
    // the vertex array is always enabled, but the color and texture
    // arrays are enabled and disabled around the compiled vertex array call
    crate::src::sdl::sdl_glimp::qglEnableClientState.expect("non-null function pointer")(
        0x8074 as i32 as crate::stdlib::GLenum,
    );
    //
    // make sure our GL state vector is set correctly
    //
    glState.glStateBits = (0x10000 as i32 | 0x100 as i32) as libc::c_ulong;
    crate::src::sdl::sdl_glimp::qglPolygonMode.expect("non-null function pointer")(
        0x408 as i32 as crate::stdlib::GLenum,
        0x1b02 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDepthMask.expect("non-null function pointer")(
        1 as i32 as crate::stdlib::GLboolean,
    );
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0xb71 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglEnable.expect("non-null function pointer")(
        0xc11 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0xb44 as i32 as crate::stdlib::GLenum,
    );
    crate::src::sdl::sdl_glimp::qglDisable.expect("non-null function pointer")(
        0xbe2 as i32 as crate::stdlib::GLenum,
    );
}
/*
================
R_PrintLongString

Workaround for ri.Printf's 1024 characters buffer limit.
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_PrintLongString(mut string: *const libc::c_char) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: i32 = crate::stdlib::strlen(string) as i32;
    p = string;
    while size > 0 as i32 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            buffer.as_mut_ptr(),
            p,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        );
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr(),
        );
        p = p.offset(1023 as i32 as isize);
        size -= 1023 as i32
    }
}
/*
================
GfxInfo_f
================
*/

unsafe extern "C" fn GfxInfo_f() {
    let mut enablestrings: [*const libc::c_char; 2] = [
        b"disabled\x00" as *const u8 as *const libc::c_char,
        b"enabled\x00" as *const u8 as *const libc::c_char,
    ];
    let mut fsstrings: [*const libc::c_char; 2] = [
        b"windowed\x00" as *const u8 as *const libc::c_char,
        b"fullscreen\x00" as *const u8 as *const libc::c_char,
    ];
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\nGL_VENDOR: %s\n\x00" as *const u8 as *const libc::c_char,
        glConfig.vendor_string.as_mut_ptr(),
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_RENDERER: %s\n\x00" as *const u8 as *const libc::c_char,
        glConfig.renderer_string.as_mut_ptr(),
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_VERSION: %s\n\x00" as *const u8 as *const libc::c_char,
        glConfig.version_string.as_mut_ptr(),
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_EXTENSIONS: \x00" as *const u8 as *const libc::c_char,
    );
    if crate::src::sdl::sdl_glimp::qglGetStringi.is_some() {
        let mut numExtensions: crate::stdlib::GLint = 0;
        let mut i: i32 = 0;
        crate::src::sdl::sdl_glimp::qglGetIntegerv.expect("non-null function pointer")(
            0x821d as i32 as crate::stdlib::GLenum,
            &mut numExtensions,
        );
        i = 0 as i32;
        while i < numExtensions {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as i32,
                b"%s \x00" as *const u8 as *const libc::c_char,
                crate::src::sdl::sdl_glimp::qglGetStringi.expect("non-null function pointer")(
                    0x1f03 as i32 as crate::stdlib::GLenum,
                    i as crate::stdlib::GLuint,
                ),
            );
            i += 1
        }
    } else {
        R_PrintLongString(glConfig.extensions_string.as_mut_ptr());
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_MAX_TEXTURE_SIZE: %d\n\x00" as *const u8 as *const libc::c_char,
        glConfig.maxTextureSize,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"GL_MAX_TEXTURE_UNITS_ARB: %d\n\x00" as *const u8 as *const libc::c_char,
        glConfig.numTextureUnits,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"\nPIXELFORMAT: color(%d-bits) Z(%d-bit) stencil(%d-bits)\n\x00" as *const u8
            as *const libc::c_char,
        glConfig.colorBits,
        glConfig.depthBits,
        glConfig.stencilBits,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"MODE: %d, %d x %d %s hz:\x00" as *const u8 as *const libc::c_char,
        (*r_mode).integer,
        glConfig.vidWidth,
        glConfig.vidHeight,
        fsstrings[((*r_fullscreen).integer == 1 as i32) as i32 as usize],
    );
    if glConfig.displayFrequency != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%d\n\x00" as *const u8 as *const libc::c_char,
            glConfig.displayFrequency,
        );
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"N/A\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"GAMMA: hardware w/ %d overbright bits\n\x00" as *const u8 as *const libc::c_char,
            crate::src::renderergl1::tr_main::tr.overbrightBits,
        );
    } else {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"GAMMA: software w/ %d overbright bits\n\x00" as *const u8 as *const libc::c_char,
            crate::src::renderergl1::tr_main::tr.overbrightBits,
        );
    }
    // rendering primitives
    let mut primitives: i32 = 0;
    // default is to use triangles if compiled vertex arrays are present
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"rendering primitives: \x00" as *const u8 as *const libc::c_char,
    );
    primitives = (*r_primitives).integer;
    if primitives == 0 as i32 {
        if crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() {
            primitives = 2 as i32
        } else {
            primitives = 1 as i32
        }
    }
    if primitives == -(1 as i32) {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"none\n\x00" as *const u8 as *const libc::c_char,
        );
    } else if primitives == 2 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"single glDrawElements\n\x00" as *const u8 as *const libc::c_char,
        );
    } else if primitives == 1 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"multiple glArrayElement\n\x00" as *const u8 as *const libc::c_char,
        );
    } else if primitives == 3 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"multiple glColor4ubv + glTexCoord2fv + glVertex3fv\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"texturemode: %s\n\x00" as *const u8 as *const libc::c_char,
        (*r_textureMode).string,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"picmip: %d\n\x00" as *const u8 as *const libc::c_char,
        (*r_picmip).integer,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"texture bits: %d\n\x00" as *const u8 as *const libc::c_char,
        (*r_texturebits).integer,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"multitexture: %s\n\x00" as *const u8 as *const libc::c_char,
        enablestrings
            [crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() as i32 as usize],
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"compiled vertex arrays: %s\n\x00" as *const u8 as *const libc::c_char,
        enablestrings
            [crate::src::sdl::sdl_glimp::qglLockArraysEXT.is_some() as i32 as usize],
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"texenv add: %s\n\x00" as *const u8 as *const libc::c_char,
        enablestrings[(glConfig.textureEnvAddAvailable as u32
            != 0 as i32 as u32) as i32 as usize],
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"compressed textures: %s\n\x00" as *const u8 as *const libc::c_char,
        enablestrings[(glConfig.textureCompression as u32
            != crate::tr_types_h::TC_NONE as i32 as u32)
            as i32 as usize],
    );
    if (*r_vertexLight).integer != 0
        || glConfig.hardwareType as u32
            == crate::tr_types_h::GLHW_PERMEDIA2 as i32 as u32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"HACK: using vertex lightmap approximation\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if glConfig.hardwareType as u32
        == crate::tr_types_h::GLHW_RAGEPRO as i32 as u32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"HACK: ragePro approximations\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if glConfig.hardwareType as u32
        == crate::tr_types_h::GLHW_RIVA128 as i32 as u32
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"HACK: riva128 approximations\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*r_finish).integer != 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Forcing glFinish\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
===============
R_Register
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_Register() {
    com_altivec = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"com_altivec\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    //
    // latched and archived variables
    //
    r_allowExtensions = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_allowExtensions\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_compressed_textures = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_compressed_textures\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_multitexture = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_multitexture\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_compiled_vertex_array = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_compiled_vertex_array\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_texture_env_add = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_texture_env_add\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_texture_filter_anisotropic = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_texture_filter_anisotropic\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_max_anisotropy = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_max_anisotropy\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_picmip = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_picmip\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_roundImagesDown = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_roundImagesDown\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_colorMipLevels = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_colorMipLevels\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cvar_CheckRange
        .expect("non-null function pointer")(
        r_picmip,
        0 as i32 as f32,
        16 as i32 as f32,
        crate::src::qcommon::q_shared::qtrue,
    );
    r_detailTextures = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_detailtextures\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_texturebits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_texturebits\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_colorbits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_colorbits\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_stencilbits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_depthbits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_depthbits\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ext_multisample = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ext_multisample\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cvar_CheckRange
        .expect("non-null function pointer")(
        r_ext_multisample,
        0 as i32 as f32,
        4 as i32 as f32,
        crate::src::qcommon::q_shared::qtrue,
    );
    r_overBrightBits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_overBrightBits\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ignorehwgamma = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ignorehwgamma\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_mode = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_mode\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_fullscreen = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_noborder = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_noborder\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_customwidth = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_customwidth\x00" as *const u8 as *const libc::c_char,
        b"1600\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_customheight = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_customheight\x00" as *const u8 as *const libc::c_char,
        b"1024\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_customPixelAspect = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_customPixelAspect\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_simpleMipMaps = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_simpleMipMaps\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_vertexLight = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_vertexLight\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_uiFullScreen = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_uifullscreen\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    );
    r_subdivisions = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_stereoEnabled = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_stereoEnabled\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_ignoreFastPath = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ignoreFastPath\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_greyscale = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_greyscale\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cvar_CheckRange
        .expect("non-null function pointer")(
        r_greyscale,
        0 as i32 as f32,
        1 as i32 as f32,
        crate::src::qcommon::q_shared::qfalse,
    );
    //
    // temporary latched variables that can only change over a restart
    //
    r_displayRefresh = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_displayRefresh\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cvar_CheckRange
        .expect("non-null function pointer")(
        r_displayRefresh,
        0 as i32 as f32,
        200 as i32 as f32,
        crate::src::qcommon::q_shared::qtrue,
    );
    r_fullbright = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_fullbright\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x20 as i32 | 0x200 as i32,
    );
    r_mapOverBrightBits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_mapOverBrightBits\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    );
    r_intensity = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_intensity\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x20 as i32,
    );
    r_singleShader = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_singleShader\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32 | 0x20 as i32,
    );
    //
    // archived variables that can change at any time
    //
    r_lodCurveError = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_lodCurveError\x00" as *const u8 as *const libc::c_char,
        b"250\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x200 as i32,
    );
    r_lodbias = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_lodbias\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_flares = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_flares\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_znear = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_znear\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cvar_CheckRange
        .expect("non-null function pointer")(
        r_znear,
        0.001f32,
        200 as i32 as f32,
        crate::src::qcommon::q_shared::qfalse,
    );
    r_zproj = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_zproj\x00" as *const u8 as *const libc::c_char,
        b"64\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_stereoSeparation = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_stereoSeparation\x00" as *const u8 as *const libc::c_char,
        b"64\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_ignoreGLErrors = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ignoreGLErrors\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_fastsky = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_fastsky\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_inGameVideo = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_inGameVideo\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_drawSun = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_drawSun\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_dynamiclight = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_dynamiclight\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_dlightBacks = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_dlightBacks\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_finish = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_finish\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_textureMode = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_textureMode\x00" as *const u8 as *const libc::c_char,
        b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_swapInterval = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_swapInterval\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32 | 0x20 as i32,
    );
    r_gamma = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_gamma\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_facePlaneCull = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_facePlaneCull\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_railWidth = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_railWidth\x00" as *const u8 as *const libc::c_char,
        b"16\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_railCoreWidth = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_railCoreWidth\x00" as *const u8 as *const libc::c_char,
        b"6\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_railSegmentLength = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_railSegmentLength\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_primitives = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_primitives\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_ambientScale = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ambientScale\x00" as *const u8 as *const libc::c_char,
        b"0.6\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_directedScale = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_directedScale\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_anaglyphMode = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_anaglyphMode\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    //
    // temporary variables that can change at any time
    //
    r_showImages = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_showImages\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    );
    r_debugLight = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_debuglight\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as i32,
    );
    r_debugSort = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_debugSort\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_printShaders = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_printShaders\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    );
    r_saveFontData = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_saveFontData\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    );
    r_nocurves = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_nocurves\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_drawworld = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_drawworld\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_lightmap = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_lightmap\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    );
    r_portalOnly = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_portalOnly\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_flareSize = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_flareSize\x00" as *const u8 as *const libc::c_char,
        b"40\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_flareFade = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_flareFade\x00" as *const u8 as *const libc::c_char,
        b"7\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_flareCoeff = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_flareCoeff\x00" as *const u8 as *const libc::c_char,
        b"150\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_skipBackEnd = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_skipBackEnd\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_measureOverdraw = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_measureOverdraw\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_lodscale = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_lodscale\x00" as *const u8 as *const libc::c_char,
        b"5\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_norefresh = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_norefresh\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_drawentities = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_drawentities\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_ignore = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_ignore\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_nocull = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_nocull\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_novis = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_novis\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_showcluster = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_showcluster\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_speeds = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_speeds\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_verbose = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_verbose\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_logFile = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_logFile\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_debugSurface = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_debugSurface\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_nobind = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_nobind\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_showtris = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_showtris\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_showsky = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_showsky\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_shownormals = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_shownormals\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_clear = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_clear\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_offsetFactor = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_offsetfactor\x00" as *const u8 as *const libc::c_char,
        b"-1\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_offsetUnits = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_offsetunits\x00" as *const u8 as *const libc::c_char,
        b"-2\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_drawBuffer = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_drawBuffer\x00" as *const u8 as *const libc::c_char,
        b"GL_BACK\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_lockpvs = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_lockpvs\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_noportals = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_noportals\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as i32,
    );
    r_shadows = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"cg_shadows\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    );
    r_marksOnTriangleMeshes = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_marksOnTriangleMeshes\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_aviMotionJpegQuality = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_aviMotionJpegQuality\x00" as *const u8 as *const libc::c_char,
        b"90\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_screenshotJpegQuality = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_screenshotJpegQuality\x00" as *const u8 as *const libc::c_char,
        b"90\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    );
    r_maxpolys = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_maxpolys\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            600 as i32,
        ),
        0 as i32,
    );
    r_maxpolyverts = crate::src::renderergl1::tr_main::ri
        .Cvar_Get
        .expect("non-null function pointer")(
        b"r_maxpolyverts\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            3000 as i32,
        ),
        0 as i32,
    );
    // make sure all the commands added here are also
    // removed in R_Shutdown
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"imagelist\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::renderergl1::tr_image::R_ImageList_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"shaderlist\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::renderergl1::tr_shader::R_ShaderList_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"skinlist\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::renderergl1::tr_image::R_SkinList_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"modellist\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::renderergl1::tr_model::R_Modellist_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"modelist\x00" as *const u8 as *const libc::c_char,
        Some(R_ModeList_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"screenshot\x00" as *const u8 as *const libc::c_char,
        Some(R_ScreenShot_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"screenshotJPEG\x00" as *const u8 as *const libc::c_char,
        Some(R_ScreenShotJPEG_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"gfxinfo\x00" as *const u8 as *const libc::c_char,
        Some(GfxInfo_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_AddCommand
        .expect("non-null function pointer")(
        b"minimize\x00" as *const u8 as *const libc::c_char,
        Some(crate::src::sdl::sdl_glimp::GLimp_Minimize as unsafe extern "C" fn() -> ()),
    );
}
/*
===============
R_Init
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_Init() {
    let mut err: i32 = 0;
    let mut i: i32 = 0;
    let mut ptr: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"----- R_Init -----\n\x00" as *const u8 as *const libc::c_char,
    );
    // clear all our internal state
    crate::stdlib::memset(
        &mut crate::src::renderergl1::tr_main::tr as *mut crate::tr_local_h::trGlobals_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_local_h::trGlobals_t>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut crate::src::renderergl1::tr_backend::backEnd as *mut crate::tr_local_h::backEndState_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_local_h::backEndState_t>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut crate::src::renderergl1::tr_shade::tess as *mut crate::tr_local_h::shaderCommands_t
            as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_local_h::shaderCommands_t>() as libc::c_ulong,
    );
    if ::std::mem::size_of::<crate::tr_types_h::glconfig_t>() as libc::c_ulong
        != 11332 as i32 as libc::c_ulong
    {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Mod ABI incompatible: sizeof(glconfig_t) == %u != 11332\x00" as *const u8
                as *const libc::c_char,
            ::std::mem::size_of::<crate::tr_types_h::glconfig_t>() as libc::c_ulong as u32,
        );
    }
    //	Swap_Init();
    if crate::src::renderergl1::tr_shade::tess.xyz.as_mut_ptr() as crate::stdlib::intptr_t
        & 15 as i32 as libc::c_long
        != 0
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"tess.xyz not 16 byte aligned\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memset(
        crate::src::renderergl1::tr_shade::tess
            .constantColor255
            .as_mut_ptr() as *mut libc::c_void,
        255 as i32,
        ::std::mem::size_of::<[crate::tr_local_h::color4ub_t; 1000]>() as libc::c_ulong,
    );
    //
    // init function tables
    //
    i = 0 as i32;
    while i < 1024 as i32 {
        crate::src::renderergl1::tr_main::tr.sinTable[i as usize] = crate::stdlib::sin(
            (i as f32 * 360.0f32
                / (1024 as i32 - 1 as i32) as f32)
                as f64
                * 3.14159265358979323846f64
                / 180.0f32 as f64,
        ) as f32;
        crate::src::renderergl1::tr_main::tr.squareTable[i as usize] =
            if i < 1024 as i32 / 2 as i32 {
                1.0f32
            } else {
                -1.0f32
            };
        crate::src::renderergl1::tr_main::tr.sawToothTable[i as usize] =
            i as f32 / 1024 as i32 as f32;
        crate::src::renderergl1::tr_main::tr.inverseSawToothTable[i as usize] =
            1.0f32 - crate::src::renderergl1::tr_main::tr.sawToothTable[i as usize];
        if i < 1024 as i32 / 2 as i32 {
            if i < 1024 as i32 / 4 as i32 {
                crate::src::renderergl1::tr_main::tr.triangleTable[i as usize] =
                    i as f32 / (1024 as i32 / 4 as i32) as f32
            } else {
                crate::src::renderergl1::tr_main::tr.triangleTable[i as usize] = 1.0f32
                    - crate::src::renderergl1::tr_main::tr.triangleTable
                        [(i - 1024 as i32 / 4 as i32) as usize]
            }
        } else {
            crate::src::renderergl1::tr_main::tr.triangleTable[i as usize] =
                -crate::src::renderergl1::tr_main::tr.triangleTable
                    [(i - 1024 as i32 / 2 as i32) as usize]
        }
        i += 1
    }
    crate::src::renderergl1::tr_image::R_InitFogTable();
    crate::src::renderercommon::tr_noise::R_NoiseInit();
    R_Register();
    max_polys = (*r_maxpolys).integer;
    if max_polys < 600 as i32 {
        max_polys = 600 as i32
    }
    max_polyverts = (*r_maxpolyverts).integer;
    if max_polyverts < 3000 as i32 {
        max_polyverts = 3000 as i32
    }
    ptr = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        (::std::mem::size_of::<crate::tr_local_h::backEndData_t>() as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<crate::tr_local_h::srfPoly_t>() as libc::c_ulong)
                    .wrapping_mul(max_polys as libc::c_ulong),
            )
            .wrapping_add(
                (::std::mem::size_of::<crate::tr_types_h::polyVert_t>() as libc::c_ulong)
                    .wrapping_mul(max_polyverts as libc::c_ulong),
            ) as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::src::qcommon::q_shared::byte;
    crate::src::renderergl1::tr_backend::backEndData = ptr as *mut crate::tr_local_h::backEndData_t;
    (*crate::src::renderergl1::tr_backend::backEndData).polys = (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<crate::tr_local_h::backEndData_t>() as libc::c_ulong as isize)
        as *mut crate::tr_local_h::srfPoly_t;
    (*crate::src::renderergl1::tr_backend::backEndData).polyVerts = (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<crate::tr_local_h::backEndData_t>() as libc::c_ulong as isize)
        .offset(
            (::std::mem::size_of::<crate::tr_local_h::srfPoly_t>() as libc::c_ulong)
                .wrapping_mul(max_polys as libc::c_ulong) as isize,
        )
        as *mut crate::tr_types_h::polyVert_t;
    crate::src::renderergl1::tr_scene::R_InitNextFrame();
    InitOpenGL();
    crate::src::renderergl1::tr_image::R_InitImages();
    crate::src::renderergl1::tr_shader::R_InitShaders();
    crate::src::renderergl1::tr_image::R_InitSkins();
    crate::src::renderergl1::tr_model::R_ModelInit();
    crate::src::renderercommon::tr_font::R_InitFreeType();
    err = crate::src::sdl::sdl_glimp::qglGetError.expect("non-null function pointer")()
        as i32;
    if err != 0 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"glGetError() = 0x%x\n\x00" as *const u8 as *const libc::c_char,
            err,
        );
    }
    // print info
    GfxInfo_f();
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"----- finished R_Init -----\n\x00" as *const u8 as *const libc::c_char,
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
===============
RE_Shutdown
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_Shutdown(mut destroyWindow: crate::src::qcommon::q_shared::qboolean) {
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"RE_Shutdown( %i )\n\x00" as *const u8 as *const libc::c_char,
        destroyWindow as u32,
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"imagelist\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"shaderlist\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"skinlist\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"modellist\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"modelist\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"screenshot\x00" as *const u8 as *const libc::c_char
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"screenshotJPEG\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(b"gfxinfo\x00" as *const u8 as *const libc::c_char);
    crate::src::renderergl1::tr_main::ri
        .Cmd_RemoveCommand
        .expect("non-null function pointer")(
        b"minimize\x00" as *const u8 as *const libc::c_char
    );
    if crate::src::renderergl1::tr_main::tr.registered as u64 != 0 {
        crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
        crate::src::renderergl1::tr_image::R_DeleteTextures();
    }
    crate::src::renderercommon::tr_font::R_DoneFreeType();
    // shut down platform specific OpenGL stuff
    if destroyWindow as u64 != 0 {
        crate::src::sdl::sdl_glimp::GLimp_Shutdown();
        crate::stdlib::memset(
            &mut glConfig as *mut crate::tr_types_h::glconfig_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::tr_types_h::glconfig_t>() as libc::c_ulong,
        );
        textureFilterAnisotropic = crate::src::qcommon::q_shared::qfalse;
        maxAnisotropy = 0 as i32;
        displayAspect = 0.0f32;
        crate::stdlib::memset(
            &mut glState as *mut crate::tr_local_h::glstate_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::tr_local_h::glstate_t>() as libc::c_ulong,
        );
    }
    crate::src::renderergl1::tr_main::tr.registered = crate::src::qcommon::q_shared::qfalse;
}
/*
=============
RE_EndRegistration

Touch all images to make sure they are resident
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_EndRegistration() {
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    if crate::src::renderergl1::tr_main::ri
        .Sys_LowPhysicalMemory
        .expect("non-null function pointer")() as u64
        == 0
    {
        crate::src::renderergl1::tr_backend::RB_ShowImages();
    };
}
/*
@@@@@@@@@@@@@@@@@@@@@
GetRefAPI

@@@@@@@@@@@@@@@@@@@@@
*/
#[no_mangle]

pub unsafe extern "C" fn GetRefAPI(
    mut apiVersion: i32,
    mut rimp: *mut crate::tr_public_h::refimport_t,
) -> *mut crate::tr_public_h::refexport_t {
    static mut re: crate::tr_public_h::refexport_t = crate::tr_public_h::refexport_t {
        Shutdown: None,
        BeginRegistration: None,
        RegisterModel: None,
        RegisterSkin: None,
        RegisterShader: None,
        RegisterShaderNoMip: None,
        LoadWorld: None,
        SetWorldVisData: None,
        EndRegistration: None,
        ClearScene: None,
        AddRefEntityToScene: None,
        AddPolyToScene: None,
        LightForPoint: None,
        AddLightToScene: None,
        AddAdditiveLightToScene: None,
        RenderScene: None,
        SetColor: None,
        DrawStretchPic: None,
        DrawStretchRaw: None,
        UploadCinematic: None,
        BeginFrame: None,
        EndFrame: None,
        MarkFragments: None,
        LerpTag: None,
        ModelBounds: None,
        RegisterFont: None,
        RemapShader: None,
        GetEntityToken: None,
        inPVS: None,
        TakeVideoFrame: None,
    };
    crate::src::renderergl1::tr_main::ri = *rimp;
    crate::stdlib::memset(
        &mut re as *mut crate::tr_public_h::refexport_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::tr_public_h::refexport_t>() as libc::c_ulong,
    );
    if apiVersion != 8 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Mismatched REF_API_VERSION: expected %i, got %i\n\x00" as *const u8
                as *const libc::c_char,
            8 as i32,
            apiVersion,
        );
        return 0 as *mut crate::tr_public_h::refexport_t;
    }
    // the RE_ functions are Renderer Entry points
    re.Shutdown =
        Some(RE_Shutdown as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> ());
    re.BeginRegistration = Some(
        crate::src::renderergl1::tr_model::RE_BeginRegistration
            as unsafe extern "C" fn(_: *mut crate::tr_types_h::glconfig_t) -> (),
    );
    re.RegisterModel = Some(
        crate::src::renderergl1::tr_model::RE_RegisterModel
            as unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
    );
    re.RegisterSkin = Some(
        crate::src::renderergl1::tr_image::RE_RegisterSkin
            as unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
    );
    re.RegisterShader = Some(
        crate::src::renderergl1::tr_shader::RE_RegisterShader
            as unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
    );
    re.RegisterShaderNoMip = Some(
        crate::src::renderergl1::tr_shader::RE_RegisterShaderNoMip
            as unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
    );
    re.LoadWorld = Some(
        crate::src::renderergl1::tr_bsp::RE_LoadWorldMap
            as unsafe extern "C" fn(_: *const libc::c_char) -> (),
    );
    re.SetWorldVisData = Some(
        crate::src::renderergl1::tr_bsp::RE_SetWorldVisData
            as unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::byte) -> (),
    );
    re.EndRegistration = Some(RE_EndRegistration as unsafe extern "C" fn() -> ());
    re.BeginFrame = Some(
        crate::src::renderergl1::tr_cmds::RE_BeginFrame
            as unsafe extern "C" fn(_: crate::tr_types_h::stereoFrame_t) -> (),
    );
    re.EndFrame = Some(
        crate::src::renderergl1::tr_cmds::RE_EndFrame
            as unsafe extern "C" fn(_: *mut i32, _: *mut i32) -> (),
    );
    re.MarkFragments = Some(
        crate::src::renderergl1::tr_marks::R_MarkFragments
            as unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec3_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::markFragment_t,
            ) -> i32,
    );
    re.LerpTag = Some(
        crate::src::renderergl1::tr_model::R_LerpTag
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::orientation_t,
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: i32,
                _: i32,
                _: f32,
                _: *const libc::c_char,
            ) -> i32,
    );
    re.ModelBounds = Some(
        crate::src::renderergl1::tr_model::R_ModelBounds
            as unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
    );
    re.ClearScene =
        Some(crate::src::renderergl1::tr_scene::RE_ClearScene as unsafe extern "C" fn() -> ());
    re.AddRefEntityToScene = Some(
        crate::src::renderergl1::tr_scene::RE_AddRefEntityToScene
            as unsafe extern "C" fn(_: *const crate::tr_types_h::refEntity_t) -> (),
    );
    re.AddPolyToScene = Some(
        crate::src::renderergl1::tr_scene::RE_AddPolyToScene
            as unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: i32,
                _: *const crate::tr_types_h::polyVert_t,
                _: i32,
            ) -> (),
    );
    re.LightForPoint = Some(
        crate::src::renderergl1::tr_light::R_LightForPoint
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
    );
    re.AddLightToScene = Some(
        crate::src::renderergl1::tr_scene::RE_AddLightToScene
            as unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
            ) -> (),
    );
    re.AddAdditiveLightToScene = Some(
        crate::src::renderergl1::tr_scene::RE_AddAdditiveLightToScene
            as unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
            ) -> (),
    );
    re.RenderScene = Some(
        crate::src::renderergl1::tr_scene::RE_RenderScene
            as unsafe extern "C" fn(_: *const crate::tr_types_h::refdef_t) -> (),
    );
    re.SetColor = Some(
        crate::src::renderergl1::tr_cmds::RE_SetColor
            as unsafe extern "C" fn(_: *const f32) -> (),
    );
    re.DrawStretchPic = Some(
        crate::src::renderergl1::tr_cmds::RE_StretchPic
            as unsafe extern "C" fn(
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: crate::src::qcommon::q_shared::qhandle_t,
            ) -> (),
    );
    re.DrawStretchRaw = Some(
        crate::src::renderergl1::tr_backend::RE_StretchRaw
            as unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: i32,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
    );
    re.UploadCinematic = Some(
        crate::src::renderergl1::tr_backend::RE_UploadCinematic
            as unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: i32,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
    );
    re.RegisterFont = Some(
        crate::src::renderercommon::tr_font::RE_RegisterFont
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::fontInfo_t,
            ) -> (),
    );
    re.RemapShader = Some(
        crate::src::renderergl1::tr_shader::R_RemapShader
            as unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> (),
    );
    re.GetEntityToken = Some(
        crate::src::renderergl1::tr_bsp::R_GetEntityToken
            as unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: i32,
            ) -> crate::src::qcommon::q_shared::qboolean,
    );
    re.inPVS = Some(
        crate::src::renderergl1::tr_world::R_inPVS
            as unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
            ) -> crate::src::qcommon::q_shared::qboolean,
    );
    re.TakeVideoFrame = Some(
        crate::src::renderergl1::tr_cmds::RE_TakeVideoFrame
            as unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::byte,
                _: *mut crate::src::qcommon::q_shared::byte,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
    );
    return &mut re;
}
unsafe extern "C" fn run_static_initializers() {
    s_numVidModes = (::std::mem::size_of::<[vidmode_t; 12]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<vidmode_t>() as libc::c_ulong)
        as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

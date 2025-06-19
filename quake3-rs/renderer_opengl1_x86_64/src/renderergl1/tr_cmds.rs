use ::libc;

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
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
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

pub use crate::qgl_h::ClearColorproc;
pub use crate::qgl_h::ClearStencilproc;
pub use crate::qgl_h::Clearproc;
pub use crate::qgl_h::ColorMaskproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::DrawBufferproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::GetErrorproc;
pub use crate::qgl_h::StencilFuncproc;
pub use crate::qgl_h::StencilMaskproc;
pub use crate::qgl_h::StencilOpproc;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::backEndData;
pub use crate::src::renderergl1::tr_backend::RB_ExecuteRenderCommands;
pub use crate::src::renderergl1::tr_image::GL_TextureMode;
pub use crate::src::renderergl1::tr_image::R_SetColorMappings;
pub use crate::src::renderergl1::tr_image::R_SumOfUsedImages;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::r_anaglyphMode;
pub use crate::src::renderergl1::tr_init::r_drawBuffer;
pub use crate::src::renderergl1::tr_init::r_gamma;
pub use crate::src::renderergl1::tr_init::r_ignoreGLErrors;
pub use crate::src::renderergl1::tr_init::r_measureOverdraw;
pub use crate::src::renderergl1::tr_init::r_shadows;
pub use crate::src::renderergl1::tr_init::r_skipBackEnd;
pub use crate::src::renderergl1::tr_init::r_speeds;
pub use crate::src::renderergl1::tr_init::r_textureMode;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_scene::R_InitNextFrame;
pub use crate::src::renderergl1::tr_shader::R_GetShaderByHandle;
pub use crate::src::sdl::sdl_glimp::qglClear;
pub use crate::src::sdl::sdl_glimp::qglClearColor;
pub use crate::src::sdl::sdl_glimp::qglClearStencil;
pub use crate::src::sdl::sdl_glimp::qglColorMask;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglDrawBuffer;
pub use crate::src::sdl::sdl_glimp::qglEnable;
pub use crate::src::sdl::sdl_glimp::qglGetError;
pub use crate::src::sdl::sdl_glimp::qglStencilFunc;
pub use crate::src::sdl::sdl_glimp::qglStencilMask;
pub use crate::src::sdl::sdl_glimp::qglStencilOp;

pub use crate::stdlib::GLbitfield;
pub use crate::stdlib::GLboolean;
pub use crate::stdlib::GLclampf;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLint;
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
pub use crate::tr_local_h::backEndData_t;
pub use crate::tr_local_h::backEndState_t;
pub use crate::tr_local_h::bmodel_t;
pub use crate::tr_local_h::clearDepthCommand_t;
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
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfPoly_t;
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
/*
=====================
R_PerformanceCounters
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_PerformanceCounters() {
    if (*r_speeds).integer == 0 {
        // clear the counters even if we aren't printing
        crate::stdlib::memset(
            &mut tr.pc as *mut frontEndCounters_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<frontEndCounters_t>() as usize,
        );
        crate::stdlib::memset(
            &mut backEnd.pc as *mut backEndCounters_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<backEndCounters_t>() as usize,
        );
        return;
    }
    if (*r_speeds).integer == 1 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"%i/%i shaders/surfs %i leafs %i verts %i/%i tris %.2f mtex %.2f dc\n\x00" as *const u8
                as *const libc::c_char,
            backEnd.pc.c_shaders,
            backEnd.pc.c_surfaces,
            tr.pc.c_leafs,
            backEnd.pc.c_vertexes,
            backEnd.pc.c_indexes / 3 as i32,
            backEnd.pc.c_totalIndexes / 3 as i32,
            (R_SumOfUsedImages() as f32 / 1000000.0f32) as f64,
            (backEnd.pc.c_overDraw / (glConfig.vidWidth * glConfig.vidHeight) as f32) as f64,
        );
    } else if (*r_speeds).integer == 2 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"(patch) %i sin %i sclip  %i sout %i bin %i bclip %i bout\n\x00" as *const u8
                as *const libc::c_char,
            tr.pc.c_sphere_cull_patch_in,
            tr.pc.c_sphere_cull_patch_clip,
            tr.pc.c_sphere_cull_patch_out,
            tr.pc.c_box_cull_patch_in,
            tr.pc.c_box_cull_patch_clip,
            tr.pc.c_box_cull_patch_out,
        );
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"(md3) %i sin %i sclip  %i sout %i bin %i bclip %i bout\n\x00" as *const u8
                as *const libc::c_char,
            tr.pc.c_sphere_cull_md3_in,
            tr.pc.c_sphere_cull_md3_clip,
            tr.pc.c_sphere_cull_md3_out,
            tr.pc.c_box_cull_md3_in,
            tr.pc.c_box_cull_md3_clip,
            tr.pc.c_box_cull_md3_out,
        );
    } else if (*r_speeds).integer == 3 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"viewcluster: %i\n\x00" as *const u8 as *const libc::c_char,
            tr.viewCluster,
        );
    } else if (*r_speeds).integer == 4 as i32 {
        if backEnd.pc.c_dlightVertexes != 0 {
            ri.Printf.expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"dlight srf:%i  culled:%i  verts:%i  tris:%i\n\x00" as *const u8
                    as *const libc::c_char,
                tr.pc.c_dlightSurfaces,
                tr.pc.c_dlightSurfacesCulled,
                backEnd.pc.c_dlightVertexes,
                backEnd.pc.c_dlightIndexes / 3 as i32,
            );
        }
    } else if (*r_speeds).integer == 5 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"zFar: %.0f\n\x00" as *const u8 as *const libc::c_char,
            tr.viewParms.zFar as f64,
        );
    } else if (*r_speeds).integer == 6 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_ALL as i32,
            b"flare adds:%i tests:%i renders:%i\n\x00" as *const u8 as *const libc::c_char,
            backEnd.pc.c_flareAdds,
            backEnd.pc.c_flareTests,
            backEnd.pc.c_flareRenders,
        );
    }
    crate::stdlib::memset(
        &mut tr.pc as *mut frontEndCounters_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<frontEndCounters_t>() as usize,
    );
    crate::stdlib::memset(
        &mut backEnd.pc as *mut backEndCounters_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<backEndCounters_t>() as usize,
    );
}
/*
====================
R_IssueRenderCommands
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_IssueRenderCommands(mut runPerformanceCounters: qboolean) {
    let mut cmdList: *mut renderCommandList_t = 0 as *mut renderCommandList_t;
    cmdList = &mut (*backEndData).commands;
    // add an end-of-list command
    *((*cmdList)
        .cmds
        .as_mut_ptr()
        .offset((*cmdList).used as isize) as *mut i32) = RC_END_OF_LIST as i32;
    // clear it out, in case this is a sync and not a buffer flip
    (*cmdList).used = 0 as i32;
    if runPerformanceCounters as u64 != 0 {
        R_PerformanceCounters();
    }
    // actually start the commands going
    if (*r_skipBackEnd).integer == 0 {
        // let it start on the new batch
        RB_ExecuteRenderCommands((*cmdList).cmds.as_mut_ptr() as *const libc::c_void);
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
/*
====================
R_IssuePendingRenderCommands

Issue any pending commands and wait for them to complete.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_IssuePendingRenderCommands() {
    if tr.registered as u64 == 0 {
        return;
    }
    R_IssueRenderCommands(qfalse);
}
/*
============
R_GetCommandBufferReserved

make sure there is enough command space
============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetCommandBufferReserved(
    mut bytes: i32,
    mut reservedBytes: i32,
) -> *mut libc::c_void {
    let mut cmdList: *mut renderCommandList_t = 0 as *mut renderCommandList_t;
    cmdList = &mut (*backEndData).commands;
    bytes = ((bytes as usize)
        .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as usize)
        .wrapping_sub(1 as i32 as usize)
        & !(::std::mem::size_of::<*mut libc::c_void>() as usize)
            .wrapping_sub(1 as i32 as usize)) as i32;
    // always leave room for the end of list command
    if (((*cmdList).used + bytes) as usize)
        .wrapping_add(::std::mem::size_of::<i32>() as usize)
        .wrapping_add(reservedBytes as usize)
        > 0x40000 as i32 as usize
    {
        if bytes as usize
            > (0x40000 as i32 as usize)
                .wrapping_sub(::std::mem::size_of::<i32>() as usize)
        {
            ri.Error.expect("non-null function pointer")(
                ERR_FATAL as i32,
                b"R_GetCommandBuffer: bad size %i\x00" as *const u8 as *const libc::c_char,
                bytes,
            );
        }
        // if we run out of room, just start dropping commands
        return 0 as *mut libc::c_void;
    }
    (*cmdList).used += bytes;
    return (*cmdList)
        .cmds
        .as_mut_ptr()
        .offset((*cmdList).used as isize)
        .offset(-(bytes as isize)) as *mut libc::c_void;
}
/*
=============
R_GetCommandBuffer

returns NULL if there is not enough space for important commands
=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetCommandBuffer(mut bytes: i32) -> *mut libc::c_void {
    return R_GetCommandBufferReserved(
        bytes,
        ((::std::mem::size_of::<swapBuffersCommand_t>() as usize)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as usize)
            .wrapping_sub(1 as i32 as usize)
            & !(::std::mem::size_of::<*mut libc::c_void>() as usize)
                .wrapping_sub(1 as i32 as usize)) as i32,
    );
}
/*
=============
R_AddDrawSurfCmd

=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddDrawSurfCmd(mut drawSurfs: *mut drawSurf_t, mut numDrawSurfs: i32) {
    let mut cmd: *mut drawSurfsCommand_t = 0 as *mut drawSurfsCommand_t;
    cmd = R_GetCommandBuffer(::std::mem::size_of::<drawSurfsCommand_t>() as usize as i32)
        as *mut drawSurfsCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = RC_DRAW_SURFS as i32;
    (*cmd).drawSurfs = drawSurfs;
    (*cmd).numDrawSurfs = numDrawSurfs;
    (*cmd).refdef = tr.refdef;
    (*cmd).viewParms = tr.viewParms;
}
/*
=============
RE_SetColor

Passing NULL will set the color to white
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_SetColor(mut rgba: *const f32) {
    let mut cmd: *mut setColorCommand_t = 0 as *mut setColorCommand_t;
    if tr.registered as u64 == 0 {
        return;
    }
    cmd = R_GetCommandBuffer(::std::mem::size_of::<setColorCommand_t>() as usize as i32)
        as *mut setColorCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = RC_SET_COLOR as i32;
    if rgba.is_null() {
        static mut colorWhite: [f32; 4] = [
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
            1 as i32 as f32,
        ];
        rgba = colorWhite.as_mut_ptr()
    }
    (*cmd).color[0 as i32 as usize] = *rgba.offset(0 as i32 as isize);
    (*cmd).color[1 as i32 as usize] = *rgba.offset(1 as i32 as isize);
    (*cmd).color[2 as i32 as usize] = *rgba.offset(2 as i32 as isize);
    (*cmd).color[3 as i32 as usize] = *rgba.offset(3 as i32 as isize);
}
/*
=============
RE_StretchPic
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_StretchPic(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut s1: f32,
    mut t1: f32,
    mut s2: f32,
    mut t2: f32,
    mut hShader: qhandle_t,
) {
    let mut cmd: *mut stretchPicCommand_t = 0 as *mut stretchPicCommand_t;
    if tr.registered as u64 == 0 {
        return;
    }
    cmd = R_GetCommandBuffer(::std::mem::size_of::<stretchPicCommand_t>() as usize as i32)
        as *mut stretchPicCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = RC_STRETCH_PIC as i32;
    (*cmd).shader = R_GetShaderByHandle(hShader) as *mut shader_s;
    (*cmd).x = x;
    (*cmd).y = y;
    (*cmd).w = w;
    (*cmd).h = h;
    (*cmd).s1 = s1;
    (*cmd).t1 = t1;
    (*cmd).s2 = s2;
    (*cmd).t2 = t2;
}
#[no_mangle]

pub unsafe extern "C" fn R_SetColorMode(
    mut rgba: *mut GLboolean,
    mut stereoFrame: stereoFrame_t,
    mut colormode: i32,
) {
    let ref mut fresh0 = *rgba.offset(3 as i32 as isize);
    *fresh0 = 1 as i32 as GLboolean;
    let ref mut fresh1 = *rgba.offset(2 as i32 as isize);
    *fresh1 = *fresh0;
    let ref mut fresh2 = *rgba.offset(1 as i32 as isize);
    *fresh2 = *fresh1;
    *rgba.offset(0 as i32 as isize) = *fresh2;
    if colormode > 4 as i32 {
        if stereoFrame as u32 == STEREO_LEFT as i32 as u32 {
            stereoFrame = STEREO_RIGHT
        } else if stereoFrame as u32 == STEREO_RIGHT as i32 as u32 {
            stereoFrame = STEREO_LEFT
        }
        colormode -= 4 as i32
    }
    if colormode == 4 as i32 {
        if stereoFrame as u32 == STEREO_LEFT as i32 as u32 {
            let ref mut fresh3 = *rgba.offset(2 as i32 as isize);
            *fresh3 = 0 as i32 as GLboolean;
            *rgba.offset(0 as i32 as isize) = *fresh3
        } else if stereoFrame as u32 == STEREO_RIGHT as i32 as u32 {
            *rgba.offset(1 as i32 as isize) = 0 as i32 as GLboolean
        }
    } else if stereoFrame as u32 == STEREO_LEFT as i32 as u32 {
        let ref mut fresh4 = *rgba.offset(2 as i32 as isize);
        *fresh4 = 0 as i32 as GLboolean;
        *rgba.offset(1 as i32 as isize) = *fresh4
    } else if stereoFrame as u32 == STEREO_RIGHT as i32 as u32 {
        *rgba.offset(0 as i32 as isize) = 0 as i32 as GLboolean;
        if colormode == 2 as i32 {
            *rgba.offset(1 as i32 as isize) = 0 as i32 as GLboolean
        } else if colormode == 3 as i32 {
            *rgba.offset(2 as i32 as isize) = 0 as i32 as GLboolean
        }
    };
}
/*
====================
RE_BeginFrame

If running in stereo, RE_BeginFrame will be called twice
for each RE_EndFrame
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_BeginFrame(mut stereoFrame: stereoFrame_t) {
    let mut cmd: *mut drawBufferCommand_t = 0 as *mut drawBufferCommand_t;
    let mut colcmd: *mut colorMaskCommand_t = 0 as *mut colorMaskCommand_t;
    if tr.registered as u64 == 0 {
        return;
    }
    glState.finishCalled = qfalse;
    tr.frameCount += 1;
    tr.frameSceneNum = 0 as i32;
    //
    // do overdraw measurement
    //
    if (*r_measureOverdraw).integer != 0 {
        if glConfig.stencilBits < 4 as i32 {
            ri.Printf.expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"Warning: not enough stencil bits to measure overdraw: %d\n\x00" as *const u8
                    as *const libc::c_char,
                glConfig.stencilBits,
            );
            ri.Cvar_Set.expect("non-null function pointer")(
                b"r_measureOverdraw\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
            (*r_measureOverdraw).modified = qfalse
        } else if (*r_shadows).integer == 2 as i32 {
            ri.Printf.expect("non-null function pointer")(
                PRINT_ALL as i32,
                b"Warning: stencil shadows and overdraw measurement are mutually exclusive\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            ri.Cvar_Set.expect("non-null function pointer")(
                b"r_measureOverdraw\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
            (*r_measureOverdraw).modified = qfalse
        } else {
            R_IssuePendingRenderCommands();
            qglEnable.expect("non-null function pointer")(0xb90 as i32 as GLenum);
            qglStencilMask.expect("non-null function pointer")(!(0 as u32));
            qglClearStencil.expect("non-null function pointer")(0 as u32 as GLint);
            qglStencilFunc.expect("non-null function pointer")(
                0x207 as i32 as GLenum,
                0 as u32 as GLint,
                !(0 as u32),
            );
            qglStencilOp.expect("non-null function pointer")(
                0x1e00 as i32 as GLenum,
                0x1e02 as i32 as GLenum,
                0x1e02 as i32 as GLenum,
            );
        }
        (*r_measureOverdraw).modified = qfalse
    } else {
        // this is only reached if it was on and is now off
        if (*r_measureOverdraw).modified as u64 != 0 {
            R_IssuePendingRenderCommands();
            qglDisable.expect("non-null function pointer")(0xb90 as i32 as GLenum);
        }
        (*r_measureOverdraw).modified = qfalse
    }
    //
    // texturemode stuff
    //
    if (*r_textureMode).modified as u64 != 0 {
        R_IssuePendingRenderCommands();
        GL_TextureMode((*r_textureMode).string);
        (*r_textureMode).modified = qfalse
    }
    //
    // gamma stuff
    //
    if (*r_gamma).modified as u64 != 0 {
        (*r_gamma).modified = qfalse;
        R_IssuePendingRenderCommands();
        R_SetColorMappings();
    }
    // check for errors
    if (*r_ignoreGLErrors).integer == 0 {
        let mut err: i32 = 0;
        R_IssuePendingRenderCommands();
        err = qglGetError.expect("non-null function pointer")() as i32;
        if err != 0 as i32 {
            ri.Error.expect("non-null function pointer")(
                ERR_FATAL as i32,
                b"RE_BeginFrame() - glGetError() failed (0x%x)!\x00" as *const u8
                    as *const libc::c_char,
                err,
            );
        }
    }
    if glConfig.stereoEnabled as u64 != 0 {
        cmd = R_GetCommandBuffer(
            ::std::mem::size_of::<drawBufferCommand_t>() as usize as i32,
        ) as *mut drawBufferCommand_t;
        if cmd.is_null() {
            return;
        }
        (*cmd).commandId = RC_DRAW_BUFFER as i32;
        if stereoFrame as u32 == STEREO_LEFT as i32 as u32 {
            (*cmd).buffer = 0x402 as i32
        } else if stereoFrame as u32 == STEREO_RIGHT as i32 as u32 {
            (*cmd).buffer = 0x403 as i32
        } else {
            ri.Error.expect("non-null function pointer")(
                ERR_FATAL as i32,
                b"RE_BeginFrame: Stereo is enabled, but stereoFrame was %i\x00" as *const u8
                    as *const libc::c_char,
                stereoFrame as u32,
            );
        }
    } else {
        if (*r_anaglyphMode).integer != 0 {
            if (*r_anaglyphMode).modified as u64 != 0 {
                // clear both, front and backbuffer.
                qglColorMask.expect("non-null function pointer")(
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                );
                qglClearColor.expect("non-null function pointer")(0.0f32, 0.0f32, 0.0f32, 1.0f32);
                qglDrawBuffer.expect("non-null function pointer")(0x404 as i32 as GLenum);
                qglClear.expect("non-null function pointer")(0x4000 as i32 as GLbitfield);
                qglDrawBuffer.expect("non-null function pointer")(0x405 as i32 as GLenum);
                qglClear.expect("non-null function pointer")(0x4000 as i32 as GLbitfield);
                (*r_anaglyphMode).modified = qfalse
            }
            if stereoFrame as u32 == STEREO_LEFT as i32 as u32 {
                cmd = R_GetCommandBuffer(
                    ::std::mem::size_of::<drawBufferCommand_t>() as usize as i32
                ) as *mut drawBufferCommand_t;
                if cmd.is_null() {
                    return;
                }
                colcmd = R_GetCommandBuffer(
                    ::std::mem::size_of::<colorMaskCommand_t>() as usize as i32
                ) as *mut colorMaskCommand_t;
                if colcmd.is_null() {
                    return;
                }
            } else if stereoFrame as u32 == STEREO_RIGHT as i32 as u32 {
                let mut cldcmd: *mut clearDepthCommand_t = 0 as *mut clearDepthCommand_t;
                cldcmd = R_GetCommandBuffer(
                    ::std::mem::size_of::<clearDepthCommand_t>() as usize as i32
                ) as *mut clearDepthCommand_t;
                if cldcmd.is_null() {
                    return;
                }
                (*cldcmd).commandId = RC_CLEARDEPTH as i32;
                colcmd = R_GetCommandBuffer(
                    ::std::mem::size_of::<colorMaskCommand_t>() as usize as i32
                ) as *mut colorMaskCommand_t;
                if colcmd.is_null() {
                    return;
                }
            } else {
                ri.Error.expect("non-null function pointer")(
                    ERR_FATAL as i32,
                    b"RE_BeginFrame: Stereo is enabled, but stereoFrame was %i\x00" as *const u8
                        as *const libc::c_char,
                    stereoFrame as u32,
                );
            }
            R_SetColorMode(
                (*colcmd).rgba.as_mut_ptr(),
                stereoFrame,
                (*r_anaglyphMode).integer,
            );
            (*colcmd).commandId = RC_COLORMASK as i32
        } else {
            if stereoFrame as u32 != STEREO_CENTER as i32 as u32 {
                ri.Error.expect("non-null function pointer")(
                    ERR_FATAL as i32,
                    b"RE_BeginFrame: Stereo is disabled, but stereoFrame was %i\x00" as *const u8
                        as *const libc::c_char,
                    stereoFrame as u32,
                );
            }
            cmd = R_GetCommandBuffer(
                ::std::mem::size_of::<drawBufferCommand_t>() as usize as i32,
            ) as *mut drawBufferCommand_t;
            if cmd.is_null() {
                return;
            }
        }
        if !cmd.is_null() {
            (*cmd).commandId = RC_DRAW_BUFFER as i32;
            if (*r_anaglyphMode).modified as u64 != 0 {
                qglColorMask.expect("non-null function pointer")(
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                    1 as i32 as GLboolean,
                );
                (*r_anaglyphMode).modified = qfalse
            }
            if Q_stricmp(
                (*r_drawBuffer).string,
                b"GL_FRONT\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*cmd).buffer = 0x404 as i32
            } else {
                (*cmd).buffer = 0x405 as i32
            }
        }
    }
    tr.refdef.stereoFrame = stereoFrame;
}
/*
=============
RE_EndFrame

Returns the number of msec spent in the back end
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_EndFrame(mut frontEndMsec: *mut i32, mut backEndMsec: *mut i32) {
    let mut cmd: *mut swapBuffersCommand_t = 0 as *mut swapBuffersCommand_t;
    if tr.registered as u64 == 0 {
        return;
    }
    cmd = R_GetCommandBufferReserved(
        ::std::mem::size_of::<swapBuffersCommand_t>() as usize as i32,
        0 as i32,
    ) as *mut swapBuffersCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = RC_SWAP_BUFFERS as i32;
    R_IssueRenderCommands(qtrue);
    R_InitNextFrame();
    if !frontEndMsec.is_null() {
        *frontEndMsec = tr.frontEndMsec
    }
    tr.frontEndMsec = 0 as i32;
    if !backEndMsec.is_null() {
        *backEndMsec = backEnd.pc.msec
    }
    backEnd.pc.msec = 0 as i32;
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
=============
RE_TakeVideoFrame
=============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_TakeVideoFrame(
    mut width: i32,
    mut height: i32,
    mut captureBuffer: *mut byte,
    mut encodeBuffer: *mut byte,
    mut motionJpeg: qboolean,
) {
    let mut cmd: *mut videoFrameCommand_t = 0 as *mut videoFrameCommand_t;
    if tr.registered as u64 == 0 {
        return;
    }
    cmd = R_GetCommandBuffer(::std::mem::size_of::<videoFrameCommand_t>() as usize as i32)
        as *mut videoFrameCommand_t;
    if cmd.is_null() {
        return;
    }
    (*cmd).commandId = RC_VIDEOFRAME as i32;
    (*cmd).width = width;
    (*cmd).height = height;
    (*cmd).captureBuffer = captureBuffer;
    (*cmd).encodeBuffer = encodeBuffer;
    (*cmd).motionJpeg = motionJpeg;
}

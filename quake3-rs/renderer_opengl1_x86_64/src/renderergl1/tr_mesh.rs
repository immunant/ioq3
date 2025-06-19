use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Frame_s;
pub use crate::qfiles_h::md3Frame_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qfiles_h::md3Shader_t;
pub use crate::qfiles_h::md3Surface_t;
pub use crate::qfiles_h::mdrBone_t;
pub use crate::qfiles_h::mdrFrame_t;
pub use crate::qfiles_h::mdrHeader_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::RadiusFromBounds;
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

pub use crate::src::renderergl1::tr_image::R_GetSkinByHandle;
pub use crate::src::renderergl1::tr_init::r_lodbias;
pub use crate::src::renderergl1::tr_init::r_lodscale;
pub use crate::src::renderergl1::tr_init::r_shadows;
pub use crate::src::renderergl1::tr_light::R_SetupEntityLighting;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_AddDrawSurf;
pub use crate::src::renderergl1::tr_main::R_CullLocalBox;
pub use crate::src::renderergl1::tr_main::R_CullLocalPointAndRadius;
pub use crate::src::renderergl1::tr_shader::R_GetShaderByHandle;

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
pub use crate::tr_local_h::srfPoly_s;
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
// tr_mesh.c: triangle model functions

unsafe extern "C" fn ProjectRadius(mut r: f32, mut location: *mut vec_t) -> f32 {
    let mut pr: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut c: f32 = 0.;
    let mut p: vec3_t = [0.; 3];
    let mut projected: [f32; 4] = [0.; 4];
    c = tr.viewParms.or.axis[0 as i32 as usize][0 as i32 as usize]
        * tr.viewParms.or.origin[0 as i32 as usize]
        + tr.viewParms.or.axis[0 as i32 as usize][1 as i32 as usize]
            * tr.viewParms.or.origin[1 as i32 as usize]
        + tr.viewParms.or.axis[0 as i32 as usize][2 as i32 as usize]
            * tr.viewParms.or.origin[2 as i32 as usize];
    dist = tr.viewParms.or.axis[0 as i32 as usize][0 as i32 as usize]
        * *location.offset(0 as i32 as isize)
        + tr.viewParms.or.axis[0 as i32 as usize][1 as i32 as usize]
            * *location.offset(1 as i32 as isize)
        + tr.viewParms.or.axis[0 as i32 as usize][2 as i32 as usize]
            * *location.offset(2 as i32 as isize)
        - c;
    if dist <= 0 as i32 as f32 {
        return 0 as i32 as f32;
    }
    p[0 as i32 as usize] = 0 as i32 as vec_t;
    p[1 as i32 as usize] = crate::stdlib::fabs(r as f64) as vec_t;
    p[2 as i32 as usize] = -dist;
    projected[0 as i32 as usize] = p[0 as i32 as usize]
        * tr.viewParms.projectionMatrix[0 as i32 as usize]
        + p[1 as i32 as usize] * tr.viewParms.projectionMatrix[4 as i32 as usize]
        + p[2 as i32 as usize] * tr.viewParms.projectionMatrix[8 as i32 as usize]
        + tr.viewParms.projectionMatrix[12 as i32 as usize];
    projected[1 as i32 as usize] = p[0 as i32 as usize]
        * tr.viewParms.projectionMatrix[1 as i32 as usize]
        + p[1 as i32 as usize] * tr.viewParms.projectionMatrix[5 as i32 as usize]
        + p[2 as i32 as usize] * tr.viewParms.projectionMatrix[9 as i32 as usize]
        + tr.viewParms.projectionMatrix[13 as i32 as usize];
    projected[2 as i32 as usize] = p[0 as i32 as usize]
        * tr.viewParms.projectionMatrix[2 as i32 as usize]
        + p[1 as i32 as usize] * tr.viewParms.projectionMatrix[6 as i32 as usize]
        + p[2 as i32 as usize] * tr.viewParms.projectionMatrix[10 as i32 as usize]
        + tr.viewParms.projectionMatrix[14 as i32 as usize];
    projected[3 as i32 as usize] = p[0 as i32 as usize]
        * tr.viewParms.projectionMatrix[3 as i32 as usize]
        + p[1 as i32 as usize] * tr.viewParms.projectionMatrix[7 as i32 as usize]
        + p[2 as i32 as usize] * tr.viewParms.projectionMatrix[11 as i32 as usize]
        + tr.viewParms.projectionMatrix[15 as i32 as usize];
    pr = projected[1 as i32 as usize] / projected[3 as i32 as usize];
    if pr > 1.0f32 {
        pr = 1.0f32
    }
    return pr;
}
/*
=============
R_CullModel
=============
*/

unsafe extern "C" fn R_CullModel(mut header: *mut md3Header_t, mut ent: *mut trRefEntity_t) -> i32 {
    let mut bounds: [vec3_t; 2] = [[0.; 3]; 2];
    let mut oldFrame: *mut md3Frame_t = 0 as *mut md3Frame_t;
    let mut newFrame: *mut md3Frame_t = 0 as *mut md3Frame_t;
    let mut i: i32 = 0;
    // compute frame pointers
    newFrame = ((header as *mut byte).offset((*header).ofsFrames as isize) as *mut md3Frame_t)
        .offset((*ent).e.frame as isize);
    oldFrame = ((header as *mut byte).offset((*header).ofsFrames as isize) as *mut md3Frame_t)
        .offset((*ent).e.oldframe as isize);
    // cull bounding sphere ONLY if this is not an upscaled entity
    if (*ent).e.nonNormalizedAxes as u64 == 0 {
        if (*ent).e.frame == (*ent).e.oldframe {
            match R_CullLocalPointAndRadius(
                (*newFrame).localOrigin.as_mut_ptr(),
                (*newFrame).radius,
            ) {
                2 => {
                    tr.pc.c_sphere_cull_md3_out += 1;
                    return 2 as i32;
                }
                0 => {
                    tr.pc.c_sphere_cull_md3_in += 1;
                    return 0 as i32;
                }
                1 => tr.pc.c_sphere_cull_md3_clip += 1,
                _ => {}
            }
        } else {
            let mut sphereCull: i32 = 0;
            let mut sphereCullB: i32 = 0;
            sphereCull =
                R_CullLocalPointAndRadius((*newFrame).localOrigin.as_mut_ptr(), (*newFrame).radius);
            if newFrame == oldFrame {
                sphereCullB = sphereCull
            } else {
                sphereCullB = R_CullLocalPointAndRadius(
                    (*oldFrame).localOrigin.as_mut_ptr(),
                    (*oldFrame).radius,
                )
            }
            if sphereCull == sphereCullB {
                if sphereCull == 2 as i32 {
                    tr.pc.c_sphere_cull_md3_out += 1;
                    return 2 as i32;
                } else {
                    if sphereCull == 0 as i32 {
                        tr.pc.c_sphere_cull_md3_in += 1;
                        return 0 as i32;
                    } else {
                        tr.pc.c_sphere_cull_md3_clip += 1
                    }
                }
            }
        }
    }
    // calculate a bounding box in the current coordinate system
    i = 0 as i32;
    while i < 3 as i32 {
        bounds[0 as i32 as usize][i as usize] = if (*oldFrame).bounds[0 as i32 as usize][i as usize]
            < (*newFrame).bounds[0 as i32 as usize][i as usize]
        {
            (*oldFrame).bounds[0 as i32 as usize][i as usize]
        } else {
            (*newFrame).bounds[0 as i32 as usize][i as usize]
        };
        bounds[1 as i32 as usize][i as usize] = if (*oldFrame).bounds[1 as i32 as usize][i as usize]
            > (*newFrame).bounds[1 as i32 as usize][i as usize]
        {
            (*oldFrame).bounds[1 as i32 as usize][i as usize]
        } else {
            (*newFrame).bounds[1 as i32 as usize][i as usize]
        };
        i += 1
    }
    match R_CullLocalBox(bounds.as_mut_ptr()) {
        0 => {
            tr.pc.c_box_cull_md3_in += 1;
            return 0 as i32;
        }
        1 => {
            tr.pc.c_box_cull_md3_clip += 1;
            return 1 as i32;
        }
        2 | _ => {
            tr.pc.c_box_cull_md3_out += 1;
            return 2 as i32;
        }
    };
}
// completely unclipped
// clipped by one or more planes
// completely outside the clipping planes
/*
** GL wrapper/helper functions
*/
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
/*
=================
R_ComputeLOD

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ComputeLOD(mut ent: *mut trRefEntity_t) -> i32 {
    let mut radius: f32 = 0.;
    let mut flod: f32 = 0.;
    let mut lodscale: f32 = 0.;
    let mut projectedRadius: f32 = 0.;
    let mut frame: *mut md3Frame_t = 0 as *mut md3Frame_t;
    let mut mdr: *mut mdrHeader_t = 0 as *mut mdrHeader_t;
    let mut mdrframe: *mut mdrFrame_t = 0 as *mut mdrFrame_t;
    let mut lod: i32 = 0;
    if (*tr.currentModel).numLods < 2 as i32 {
        // model has only 1 LOD level, skip computations and bias
        lod = 0 as i32
    } else {
        // multiple LODs exist, so compute projected bounding sphere
        // and use that as a criteria for selecting LOD
        if (*tr.currentModel).type_0 as u32 == MOD_MDR as i32 as u32 {
            let mut frameSize: i32 = 0;
            mdr = (*tr.currentModel).modelData as *mut mdrHeader_t;
            frameSize = &mut *(*(0 as *mut mdrFrame_t))
                .bones
                .as_mut_ptr()
                .offset((*mdr).numBones as isize) as *mut mdrBone_t
                as size_t as i32;
            mdrframe = (mdr as *mut byte)
                .offset((*mdr).ofsFrames as isize)
                .offset((frameSize * (*ent).e.frame) as isize)
                as *mut mdrFrame_t;
            radius = RadiusFromBounds(
                (*mdrframe).bounds[0 as i32 as usize].as_mut_ptr() as *const vec_t,
                (*mdrframe).bounds[1 as i32 as usize].as_mut_ptr() as *const vec_t,
            )
        } else {
            frame = ((*tr.currentModel).md3[0 as i32 as usize] as *mut u8)
                .offset((*(*tr.currentModel).md3[0 as i32 as usize]).ofsFrames as isize)
                as *mut md3Frame_t;
            frame = frame.offset((*ent).e.frame as isize);
            radius = RadiusFromBounds(
                (*frame).bounds[0 as i32 as usize].as_mut_ptr() as *const vec_t,
                (*frame).bounds[1 as i32 as usize].as_mut_ptr() as *const vec_t,
            )
        }
        projectedRadius = ProjectRadius(radius, (*ent).e.origin.as_mut_ptr());
        if projectedRadius != 0 as i32 as f32 {
            lodscale = (*r_lodscale).value;
            if lodscale > 20 as i32 as f32 {
                lodscale = 20 as i32 as f32
            }
            flod = 1.0f32 - projectedRadius * lodscale
        } else {
            // object intersects near view plane, e.g. view weapon
            flod = 0 as i32 as f32
        }
        flod *= (*tr.currentModel).numLods as f32;
        lod = ri.ftol.expect("non-null function pointer")(flod) as i32;
        if lod < 0 as i32 {
            lod = 0 as i32
        } else if lod >= (*tr.currentModel).numLods {
            lod = (*tr.currentModel).numLods - 1 as i32
        }
    }
    lod += (*r_lodbias).integer;
    if lod >= (*tr.currentModel).numLods {
        lod = (*tr.currentModel).numLods - 1 as i32
    }
    if lod < 0 as i32 {
        lod = 0 as i32
    }
    return lod;
}
/*
=================
R_ComputeFogNum

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ComputeFogNum(
    mut header: *mut md3Header_t,
    mut ent: *mut trRefEntity_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut fog: *mut fog_t = 0 as *mut fog_t;
    let mut md3Frame: *mut md3Frame_t = 0 as *mut md3Frame_t;
    let mut localOrigin: vec3_t = [0.; 3];
    if tr.refdef.rdflags & 0x1 as i32 != 0 {
        return 0 as i32;
    }
    // FIXME: non-normalized axis issues
    md3Frame = ((header as *mut byte).offset((*header).ofsFrames as isize) as *mut md3Frame_t)
        .offset((*ent).e.frame as isize);
    localOrigin[0 as i32 as usize] =
        (*ent).e.origin[0 as i32 as usize] + (*md3Frame).localOrigin[0 as i32 as usize];
    localOrigin[1 as i32 as usize] =
        (*ent).e.origin[1 as i32 as usize] + (*md3Frame).localOrigin[1 as i32 as usize];
    localOrigin[2 as i32 as usize] =
        (*ent).e.origin[2 as i32 as usize] + (*md3Frame).localOrigin[2 as i32 as usize];
    i = 1 as i32;
    while i < (*tr.world).numfogs {
        fog = &mut *(*tr.world).fogs.offset(i as isize) as *mut fog_t;
        j = 0 as i32;
        while j < 3 as i32 {
            if localOrigin[j as usize] - (*md3Frame).radius
                >= (*fog).bounds[1 as i32 as usize][j as usize]
            {
                break;
            }
            if localOrigin[j as usize] + (*md3Frame).radius
                <= (*fog).bounds[0 as i32 as usize][j as usize]
            {
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
/*
=================
R_AddMD3Surfaces

=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddMD3Surfaces(mut ent: *mut trRefEntity_t) {
    let mut i: i32 = 0;
    let mut header: *mut md3Header_t = 0 as *mut md3Header_t;
    let mut surface: *mut md3Surface_t = 0 as *mut md3Surface_t;
    let mut md3Shader: *mut md3Shader_t = 0 as *mut md3Shader_t;
    let mut shader: *mut shader_t = 0 as *mut shader_t;
    let mut cull: i32 = 0;
    let mut lod: i32 = 0;
    let mut fogNum: i32 = 0;
    let mut personalModel: qboolean = qfalse;
    // don't add third_person objects if not in a portal
    personalModel = ((*ent).e.renderfx & 0x2 as i32 != 0 && tr.viewParms.isPortal as u64 == 0)
        as i32 as qboolean;
    if (*ent).e.renderfx & 0x200 as i32 != 0 {
        (*ent).e.frame %= (*(*tr.currentModel).md3[0 as i32 as usize]).numFrames;
        (*ent).e.oldframe %= (*(*tr.currentModel).md3[0 as i32 as usize]).numFrames
    }
    //
    // Validate the frames so there is no chance of a crash.
    // This will write directly into the entity structure, so
    // when the surfaces are rendered, they don't need to be
    // range checked again.
    //
    if (*ent).e.frame >= (*(*tr.currentModel).md3[0 as i32 as usize]).numFrames
        || (*ent).e.frame < 0 as i32
        || (*ent).e.oldframe >= (*(*tr.currentModel).md3[0 as i32 as usize]).numFrames
        || (*ent).e.oldframe < 0 as i32
    {
        ri.Printf.expect("non-null function pointer")(
            PRINT_DEVELOPER as i32,
            b"R_AddMD3Surfaces: no such frame %d to %d for \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            (*ent).e.oldframe,
            (*ent).e.frame,
            (*tr.currentModel).name.as_mut_ptr(),
        );
        (*ent).e.frame = 0 as i32;
        (*ent).e.oldframe = 0 as i32
    }
    //
    // compute LOD
    //
    lod = R_ComputeLOD(ent);
    header = (*tr.currentModel).md3[lod as usize];
    //
    // cull the entire model if merged bounding box of both frames
    // is outside the view frustum.
    //
    cull = R_CullModel(header, ent);
    if cull == 2 as i32 {
        return;
    }
    //
    // set up lighting now that we know we aren't culled
    //
    if personalModel as u64 == 0 || (*r_shadows).integer > 1 as i32 {
        R_SetupEntityLighting(
            &mut tr.refdef as *mut _ as *const trRefdef_t,
            ent as *mut trRefEntity_t,
        );
    }
    //
    // see if we are in a fog volume
    //
    fogNum = R_ComputeFogNum(header, ent);
    //
    // draw all surfaces
    //
    surface = (header as *mut byte).offset((*header).ofsSurfaces as isize) as *mut md3Surface_t;
    i = 0 as i32;
    while i < (*header).numSurfaces {
        if (*ent).e.customShader != 0 {
            shader = R_GetShaderByHandle((*ent).e.customShader) as *mut shader_s
        } else if (*ent).e.customSkin > 0 as i32 && (*ent).e.customSkin < tr.numSkins {
            let mut skin: *mut skin_t = 0 as *mut skin_t;
            let mut j: i32 = 0;
            skin = R_GetSkinByHandle((*ent).e.customSkin) as *mut skin_s;
            // match the surface name to something in the skin file
            shader = tr.defaultShader;
            j = 0 as i32;
            while j < (*skin).numSurfaces {
                // the names have both been lowercased
                if libc::strcmp(
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
            if shader == tr.defaultShader {
                ri.Printf.expect("non-null function pointer")(
                    PRINT_DEVELOPER as i32,
                    b"WARNING: no shader for surface %s in skin %s\n\x00" as *const u8
                        as *const libc::c_char,
                    (*surface).name.as_mut_ptr(),
                    (*skin).name.as_mut_ptr(),
                );
            } else if (*shader).defaultShader as u64 != 0 {
                ri.Printf.expect("non-null function pointer")(
                    PRINT_DEVELOPER as i32,
                    b"WARNING: shader %s in skin %s not found\n\x00" as *const u8
                        as *const libc::c_char,
                    (*shader).name.as_mut_ptr(),
                    (*skin).name.as_mut_ptr(),
                );
            }
        } else if (*surface).numShaders <= 0 as i32 {
            shader = tr.defaultShader
        } else {
            md3Shader =
                (surface as *mut byte).offset((*surface).ofsShaders as isize) as *mut md3Shader_t;
            md3Shader = md3Shader.offset(((*ent).e.skinNum % (*surface).numShaders) as isize);
            shader = tr.shaders[(*md3Shader).shaderIndex as usize]
        }
        // we will add shadows even if the main object isn't visible in the view
        // stencil shadows can't do personal models unless I polyhedron clip
        if personalModel as u64 == 0
            && (*r_shadows).integer == 2 as i32
            && fogNum == 0 as i32
            && (*ent).e.renderfx & (0x40 as i32 | 0x8 as i32) == 0
            && (*shader).sort == SS_OPAQUE as i32 as f32
        {
            R_AddDrawSurf(
                surface as *mut libc::c_void as *mut surfaceType_t,
                tr.shadowShader as *mut shader_s,
                0 as i32,
                qfalse as i32,
            );
        }
        // projection shadows work fine with personal models
        if (*r_shadows).integer == 3 as i32
            && fogNum == 0 as i32
            && (*ent).e.renderfx & 0x100 as i32 != 0
            && (*shader).sort == SS_OPAQUE as i32 as f32
        {
            R_AddDrawSurf(
                surface as *mut libc::c_void as *mut surfaceType_t,
                tr.projectionShadowShader as *mut shader_s,
                0 as i32,
                qfalse as i32,
            );
        }
        // don't add third_person objects if not viewing through a portal
        if personalModel as u64 == 0 {
            R_AddDrawSurf(
                surface as *mut libc::c_void as *mut surfaceType_t,
                shader as *mut shader_s,
                fogNum,
                qfalse as i32,
            );
        }
        surface = (surface as *mut byte).offset((*surface).ofsEnd as isize) as *mut md3Surface_t;
        i += 1
    }
}

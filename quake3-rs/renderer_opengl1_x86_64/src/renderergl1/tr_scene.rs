use ::libc;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::Q_isnan;
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
pub use crate::stdlib::GLuint;
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

pub use crate::src::renderergl1::tr_backend::backEndData;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::max_polys;
pub use crate::src::renderergl1::tr_init::max_polyverts;
pub use crate::src::renderergl1::tr_init::r_dynamiclight;
pub use crate::src::renderergl1::tr_init::r_norefresh;
pub use crate::src::renderergl1::tr_init::r_vertexLight;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_main::R_AddDrawSurf;
pub use crate::src::renderergl1::tr_main::R_RenderView;
pub use crate::src::renderergl1::tr_shader::R_GetShaderByHandle;
pub use crate::src::sdl::sdl_glimp::GLimp_LogComment;

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
pub use crate::tr_local_h::backEndData_t;
pub use crate::tr_local_h::bmodel_t;
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
pub use crate::tr_local_h::mnode_s;
pub use crate::tr_local_h::mnode_t;
pub use crate::tr_local_h::model_s;
pub use crate::tr_local_h::model_t;
pub use crate::tr_local_h::modtype_t;
pub use crate::tr_local_h::msurface_s;
pub use crate::tr_local_h::msurface_t;
pub use crate::tr_local_h::orientationr_t;
pub use crate::tr_local_h::renderCommandList_t;
pub use crate::tr_local_h::shaderStage_t;
pub use crate::tr_local_h::shader_s;
pub use crate::tr_local_h::shader_t;
pub use crate::tr_local_h::skinSurface_t;
pub use crate::tr_local_h::skin_s;
pub use crate::tr_local_h::skin_t;
pub use crate::tr_local_h::skyParms_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfPoly_t;
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
#[no_mangle]

pub static mut r_firstSceneDrawSurf: i32 = 0;
#[no_mangle]

pub static mut r_numdlights: i32 = 0;
#[no_mangle]

pub static mut r_firstSceneDlight: i32 = 0;
#[no_mangle]

pub static mut r_numentities: i32 = 0;
#[no_mangle]

pub static mut r_firstSceneEntity: i32 = 0;
#[no_mangle]

pub static mut r_numpolys: i32 = 0;
#[no_mangle]

pub static mut r_firstScenePoly: i32 = 0;
#[no_mangle]

pub static mut r_numpolyverts: i32 = 0;
/*
====================
R_InitNextFrame

====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitNextFrame() {
    (*backEndData).commands.used = 0 as i32;
    r_firstSceneDrawSurf = 0 as i32;
    r_numdlights = 0 as i32;
    r_firstSceneDlight = 0 as i32;
    r_numentities = 0 as i32;
    r_firstSceneEntity = 0 as i32;
    r_numpolys = 0 as i32;
    r_firstScenePoly = 0 as i32;
    r_numpolyverts = 0 as i32;
}
/*
====================
RE_ClearScene

====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_ClearScene() {
    r_firstSceneDlight = r_numdlights;
    r_firstSceneEntity = r_numentities;
    r_firstScenePoly = r_numpolys;
}
/*
===========================================================================

DISCRETE POLYS

===========================================================================
*/
/*
=====================
R_AddPolygonSurfaces

Adds all the scene's polys into this view's drawsurf list
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_AddPolygonSurfaces() {
    let mut i: i32 = 0;
    let mut sh: *mut shader_t = 0 as *mut shader_t;
    let mut poly: *mut srfPoly_t = 0 as *mut srfPoly_t;
    tr.currentEntityNum = ((1 as i32) << 10 as i32) - 1 as i32;
    tr.shiftedEntityNum = tr.currentEntityNum << 7 as i32;
    i = 0 as i32;
    poly = tr.refdef.polys;
    while i < tr.refdef.numPolys {
        sh = R_GetShaderByHandle((*poly).hShader) as *mut shader_s;
        R_AddDrawSurf(
            poly as *mut libc::c_void as *mut surfaceType_t,
            sh as *mut shader_s,
            (*poly).fogIndex,
            qfalse as i32,
        );
        i += 1;
        poly = poly.offset(1)
    }
}
/*
=====================
RE_AddPolyToScene

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_AddPolyToScene(
    mut hShader: qhandle_t,
    mut numVerts: i32,
    mut verts: *const polyVert_t,
    mut numPolys: i32,
) {
    let mut poly: *mut srfPoly_t = 0 as *mut srfPoly_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut fogIndex: i32 = 0;
    let mut fog: *mut fog_t = 0 as *mut fog_t;
    let mut bounds: [vec3_t; 2] = [[0.; 3]; 2];
    if tr.registered as u64 == 0 {
        return;
    }
    if hShader == 0 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_WARNING as i32,
            b"WARNING: RE_AddPolyToScene: NULL poly shader\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    j = 0 as i32;
    while j < numPolys {
        if r_numpolyverts + numVerts > max_polyverts || r_numpolys >= max_polys {
            /*
            NOTE TTimo this was initially a PRINT_WARNING
            but it happens a lot with high fighting scenes and particles
            since we don't plan on changing the const and making for room for those effects
            simply cut this message to developer only
            */
            ri.Printf.expect("non-null function pointer")(
                PRINT_DEVELOPER as i32,
                b"WARNING: RE_AddPolyToScene: r_max_polys or r_max_polyverts reached\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        poly = &mut *(*backEndData).polys.offset(r_numpolys as isize) as *mut srfPoly_t;
        (*poly).surfaceType = SF_POLY;
        (*poly).hShader = hShader;
        (*poly).numVerts = numVerts;
        (*poly).verts =
            &mut *(*backEndData).polyVerts.offset(r_numpolyverts as isize) as *mut polyVert_t;
        crate::stdlib::memcpy(
            (*poly).verts as *mut libc::c_void,
            &*verts.offset((numVerts * j) as isize) as *const polyVert_t as *const libc::c_void,
            (numVerts as usize)
                .wrapping_mul(::std::mem::size_of::<polyVert_t>() as usize),
        );
        if glConfig.hardwareType as u32 == GLHW_RAGEPRO as i32 as u32 {
            (*(*poly).verts).modulate[0 as i32 as usize] = 255 as i32 as byte;
            (*(*poly).verts).modulate[1 as i32 as usize] = 255 as i32 as byte;
            (*(*poly).verts).modulate[2 as i32 as usize] = 255 as i32 as byte;
            (*(*poly).verts).modulate[3 as i32 as usize] = 255 as i32 as byte
        }
        // done.
        r_numpolys += 1;
        r_numpolyverts += numVerts;
        // if no world is loaded
        if tr.world.is_null() {
            fogIndex = 0 as i32
        } else if (*tr.world).numfogs == 1 as i32 {
            fogIndex = 0 as i32
        } else {
            // see if it is in a fog volume
            // find which fog volume the poly is in
            bounds[0 as i32 as usize][0 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[0 as i32 as usize];
            bounds[0 as i32 as usize][1 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[1 as i32 as usize];
            bounds[0 as i32 as usize][2 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[2 as i32 as usize];
            bounds[1 as i32 as usize][0 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[0 as i32 as usize];
            bounds[1 as i32 as usize][1 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[1 as i32 as usize];
            bounds[1 as i32 as usize][2 as i32 as usize] =
                (*(*poly).verts.offset(0 as i32 as isize)).xyz[2 as i32 as usize];
            i = 1 as i32;
            while i < (*poly).numVerts {
                AddPointToBounds(
                    (*(*poly).verts.offset(i as isize)).xyz.as_mut_ptr() as *const vec_t,
                    bounds[0 as i32 as usize].as_mut_ptr(),
                    bounds[1 as i32 as usize].as_mut_ptr(),
                );
                i += 1
            }
            fogIndex = 1 as i32;
            while fogIndex < (*tr.world).numfogs {
                fog = &mut *(*tr.world).fogs.offset(fogIndex as isize) as *mut fog_t;
                if bounds[1 as i32 as usize][0 as i32 as usize]
                    >= (*fog).bounds[0 as i32 as usize][0 as i32 as usize]
                    && bounds[1 as i32 as usize][1 as i32 as usize]
                        >= (*fog).bounds[0 as i32 as usize][1 as i32 as usize]
                    && bounds[1 as i32 as usize][2 as i32 as usize]
                        >= (*fog).bounds[0 as i32 as usize][2 as i32 as usize]
                    && bounds[0 as i32 as usize][0 as i32 as usize]
                        <= (*fog).bounds[1 as i32 as usize][0 as i32 as usize]
                    && bounds[0 as i32 as usize][1 as i32 as usize]
                        <= (*fog).bounds[1 as i32 as usize][1 as i32 as usize]
                    && bounds[0 as i32 as usize][2 as i32 as usize]
                        <= (*fog).bounds[1 as i32 as usize][2 as i32 as usize]
                {
                    break;
                }
                fogIndex += 1
            }
            if fogIndex == (*tr.world).numfogs {
                fogIndex = 0 as i32
            }
        }
        (*poly).fogIndex = fogIndex;
        j += 1
    }
}
//=================================================================================
/*
=====================
RE_AddRefEntityToScene

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_AddRefEntityToScene(mut ent: *const refEntity_t) {
    if tr.registered as u64 == 0 {
        return;
    }
    if r_numentities >= ((1 as i32) << 10 as i32) - 1 as i32 {
        ri.Printf.expect("non-null function pointer")(
            PRINT_DEVELOPER as i32,
            b"RE_AddRefEntityToScene: Dropping refEntity, reached MAX_REFENTITIES\n\x00"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Q_isnan((*ent).origin[0 as i32 as usize]) != 0
        || Q_isnan((*ent).origin[1 as i32 as usize]) != 0
        || Q_isnan((*ent).origin[2 as i32 as usize]) != 0
    {
        static mut firstTime: qboolean = qtrue;
        if firstTime as u64 != 0 {
            firstTime = qfalse;
            ri.Printf.expect("non-null function pointer")(PRINT_WARNING as
                                                              i32,
                                                          b"RE_AddRefEntityToScene passed a refEntity which has an origin with a NaN component\n\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
        }
        return;
    }
    if ((*ent).reType as i32) < 0 as i32
        || (*ent).reType as u32 >= RT_MAX_REF_ENTITY_TYPE as i32 as u32
    {
        ri.Error.expect("non-null function pointer")(
            ERR_DROP as i32,
            b"RE_AddRefEntityToScene: bad reType %i\x00" as *const u8 as *const libc::c_char,
            (*ent).reType as u32,
        );
    }
    (*backEndData).entities[r_numentities as usize].e = *ent;
    (*backEndData).entities[r_numentities as usize].lightingCalculated = qfalse;
    r_numentities += 1;
}
/*
=====================
RE_AddDynamicLightToScene

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_AddDynamicLightToScene(
    mut org: *const vec_t,
    mut intensity: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
    mut additive: i32,
) {
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if tr.registered as u64 == 0 {
        return;
    }
    if r_numdlights >= 32 as i32 {
        return;
    }
    if intensity <= 0 as i32 as f32 {
        return;
    }
    // these cards don't have the correct blend mode
    if glConfig.hardwareType as u32 == GLHW_RIVA128 as i32 as u32
        || glConfig.hardwareType as u32 == GLHW_PERMEDIA2 as i32 as u32
    {
        return;
    }
    let fresh0 = r_numdlights;
    r_numdlights = r_numdlights + 1;
    dl = &mut *(*backEndData).dlights.as_mut_ptr().offset(fresh0 as isize) as *mut dlight_t;
    (*dl).origin[0 as i32 as usize] = *org.offset(0 as i32 as isize);
    (*dl).origin[1 as i32 as usize] = *org.offset(1 as i32 as isize);
    (*dl).origin[2 as i32 as usize] = *org.offset(2 as i32 as isize);
    (*dl).radius = intensity;
    (*dl).color[0 as i32 as usize] = r;
    (*dl).color[1 as i32 as usize] = g;
    (*dl).color[2 as i32 as usize] = b;
    (*dl).additive = additive;
}
/*
=====================
RE_AddLightToScene

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_AddLightToScene(
    mut org: *const vec_t,
    mut intensity: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
) {
    RE_AddDynamicLightToScene(org, intensity, r, g, b, qfalse as i32);
}
/*
=====================
RE_AddAdditiveLightToScene

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_AddAdditiveLightToScene(
    mut org: *const vec_t,
    mut intensity: f32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
) {
    RE_AddDynamicLightToScene(org, intensity, r, g, b, qtrue as i32);
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
@@@@@@@@@@@@@@@@@@@@@
RE_RenderScene

Draw a 3D view into a part of the window, then return
to 2D drawing.

Rendering a scene may require multiple views to be rendered
to handle mirrors,
@@@@@@@@@@@@@@@@@@@@@
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RenderScene(mut fd: *const refdef_t) {
    let mut parms: viewParms_t = viewParms_t {
        or: orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        world: orientationr_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
            viewOrigin: [0.; 3],
            modelMatrix: [0.; 16],
        },
        pvsOrigin: [0.; 3],
        isPortal: qfalse,
        isMirror: qfalse,
        frameSceneNum: 0,
        frameCount: 0,
        portalPlane: cplane_t {
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
        frustum: [cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        }; 4],
        visBounds: [[0.; 3]; 2],
        zFar: 0.,
        stereoFrame: STEREO_CENTER,
    };
    let mut startTime: i32 = 0;
    if tr.registered as u64 == 0 {
        return;
    }
    GLimp_LogComment(
        b"====== RE_RenderScene =====\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*r_norefresh).integer != 0 {
        return;
    }
    startTime = ri.Milliseconds.expect("non-null function pointer")();
    if tr.world.is_null() && (*fd).rdflags & 0x1 as i32 == 0 {
        ri.Error.expect("non-null function pointer")(
            ERR_DROP as i32,
            b"R_RenderScene: NULL worldmodel\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memcpy(
        tr.refdef.text.as_mut_ptr() as *mut libc::c_void,
        (*fd).text.as_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[libc::c_char; 32]; 8]>() as usize,
    );
    tr.refdef.x = (*fd).x;
    tr.refdef.y = (*fd).y;
    tr.refdef.width = (*fd).width;
    tr.refdef.height = (*fd).height;
    tr.refdef.fov_x = (*fd).fov_x;
    tr.refdef.fov_y = (*fd).fov_y;
    tr.refdef.vieworg[0 as i32 as usize] = (*fd).vieworg[0 as i32 as usize];
    tr.refdef.vieworg[1 as i32 as usize] = (*fd).vieworg[1 as i32 as usize];
    tr.refdef.vieworg[2 as i32 as usize] = (*fd).vieworg[2 as i32 as usize];
    tr.refdef.viewaxis[0 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][0 as i32 as usize];
    tr.refdef.viewaxis[0 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][1 as i32 as usize];
    tr.refdef.viewaxis[0 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][2 as i32 as usize];
    tr.refdef.viewaxis[1 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][0 as i32 as usize];
    tr.refdef.viewaxis[1 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][1 as i32 as usize];
    tr.refdef.viewaxis[1 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][2 as i32 as usize];
    tr.refdef.viewaxis[2 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][0 as i32 as usize];
    tr.refdef.viewaxis[2 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][1 as i32 as usize];
    tr.refdef.viewaxis[2 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][2 as i32 as usize];
    tr.refdef.time = (*fd).time;
    tr.refdef.rdflags = (*fd).rdflags;
    // copy the areamask data over and note if it has changed, which
    // will force a reset of the visible leafs even if the view hasn't moved
    tr.refdef.areamaskModified = qfalse;
    if tr.refdef.rdflags & 0x1 as i32 == 0 {
        let mut areaDiff: i32 = 0;
        let mut i: i32 = 0;
        // compare the area bits
        areaDiff = 0 as i32;
        i = 0 as i32;
        while i < 32 as i32 / 4 as i32 {
            areaDiff |= *(tr.refdef.areamask.as_mut_ptr() as *mut i32).offset(i as isize)
                ^ *((*fd).areamask.as_ptr() as *mut i32).offset(i as isize);
            *(tr.refdef.areamask.as_mut_ptr() as *mut i32).offset(i as isize) =
                *((*fd).areamask.as_ptr() as *mut i32).offset(i as isize);
            i += 1
        }
        if areaDiff != 0 {
            // a door just opened or something
            tr.refdef.areamaskModified = qtrue
        }
    }
    // derived info
    tr.refdef.floatTime = tr.refdef.time as f64 * 0.001f64;
    tr.refdef.numDrawSurfs = r_firstSceneDrawSurf;
    tr.refdef.drawSurfs = (*backEndData).drawSurfs.as_mut_ptr();
    tr.refdef.num_entities = r_numentities - r_firstSceneEntity;
    tr.refdef.entities = &mut *(*backEndData)
        .entities
        .as_mut_ptr()
        .offset(r_firstSceneEntity as isize) as *mut trRefEntity_t;
    tr.refdef.num_dlights = r_numdlights - r_firstSceneDlight;
    tr.refdef.dlights = &mut *(*backEndData)
        .dlights
        .as_mut_ptr()
        .offset(r_firstSceneDlight as isize) as *mut dlight_t;
    tr.refdef.numPolys = r_numpolys - r_firstScenePoly;
    tr.refdef.polys =
        &mut *(*backEndData).polys.offset(r_firstScenePoly as isize) as *mut srfPoly_t;
    // turn off dynamic lighting globally by clearing all the
    // dlights if it needs to be disabled or if vertex lighting is enabled
    if (*r_dynamiclight).integer == 0 as i32
        || (*r_vertexLight).integer == 1 as i32
        || glConfig.hardwareType as u32 == GLHW_PERMEDIA2 as i32 as u32
    {
        tr.refdef.num_dlights = 0 as i32
    }
    // a single frame may have multiple scenes draw inside it --
    // a 3D game view, 3D status bar renderings, 3D menus, etc.
    // They need to be distinguished by the light flare code, because
    // the visibility state for a given surface may be different in
    // each scene / view.
    tr.frameSceneNum += 1;
    tr.sceneCount += 1;
    // setup view parms for the initial view
    //
    // set up viewport
    // The refdef takes 0-at-the-top y coordinates, so
    // convert to GL's 0-at-the-bottom space
    //
    crate::stdlib::memset(
        &mut parms as *mut viewParms_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<viewParms_t>() as usize,
    );
    parms.viewportX = tr.refdef.x;
    parms.viewportY = glConfig.vidHeight - (tr.refdef.y + tr.refdef.height);
    parms.viewportWidth = tr.refdef.width;
    parms.viewportHeight = tr.refdef.height;
    parms.isPortal = qfalse;
    parms.fovX = tr.refdef.fov_x;
    parms.fovY = tr.refdef.fov_y;
    parms.stereoFrame = tr.refdef.stereoFrame;
    parms.or.origin[0 as i32 as usize] = (*fd).vieworg[0 as i32 as usize];
    parms.or.origin[1 as i32 as usize] = (*fd).vieworg[1 as i32 as usize];
    parms.or.origin[2 as i32 as usize] = (*fd).vieworg[2 as i32 as usize];
    parms.or.axis[0 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][0 as i32 as usize];
    parms.or.axis[0 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][1 as i32 as usize];
    parms.or.axis[0 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[0 as i32 as usize][2 as i32 as usize];
    parms.or.axis[1 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][0 as i32 as usize];
    parms.or.axis[1 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][1 as i32 as usize];
    parms.or.axis[1 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[1 as i32 as usize][2 as i32 as usize];
    parms.or.axis[2 as i32 as usize][0 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][0 as i32 as usize];
    parms.or.axis[2 as i32 as usize][1 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][1 as i32 as usize];
    parms.or.axis[2 as i32 as usize][2 as i32 as usize] =
        (*fd).viewaxis[2 as i32 as usize][2 as i32 as usize];
    parms.pvsOrigin[0 as i32 as usize] = (*fd).vieworg[0 as i32 as usize];
    parms.pvsOrigin[1 as i32 as usize] = (*fd).vieworg[1 as i32 as usize];
    parms.pvsOrigin[2 as i32 as usize] = (*fd).vieworg[2 as i32 as usize];
    R_RenderView(&mut parms as *mut _ as *mut viewParms_t);
    // the next scene rendered in this frame will tack on after this one
    r_firstSceneDrawSurf = tr.refdef.numDrawSurfs;
    r_firstSceneEntity = r_numentities;
    r_firstSceneDlight = r_numdlights;
    r_firstScenePoly = r_numpolys;
    tr.frontEndMsec += ri.Milliseconds.expect("non-null function pointer")() - startTime;
}

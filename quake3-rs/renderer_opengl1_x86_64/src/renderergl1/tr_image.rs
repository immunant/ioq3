use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;

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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_GetExtension;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strlwr;
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

pub use crate::qgl_h::BindTextureproc;
pub use crate::qgl_h::DeleteTexturesproc;
pub use crate::qgl_h::GenTexturesproc;
pub use crate::qgl_h::TexImage2Dproc;
pub use crate::qgl_h::TexParameterfproc;
pub use crate::qgl_h::TexParameteriproc;
pub use crate::src::renderercommon::tr_image_bmp::R_LoadBMP;
pub use crate::src::renderercommon::tr_image_jpg::R_LoadJPG;
pub use crate::src::renderercommon::tr_image_pcx::R_LoadPCX;
pub use crate::src::renderercommon::tr_image_png::R_LoadPNG;
pub use crate::src::renderercommon::tr_image_tga::R_LoadTGA;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_SelectTexture;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_image::ctype_h::tolower;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::glState;
pub use crate::src::renderergl1::tr_init::maxAnisotropy;
pub use crate::src::renderergl1::tr_init::r_colorMipLevels;
pub use crate::src::renderergl1::tr_init::r_ext_max_anisotropy;
pub use crate::src::renderergl1::tr_init::r_gamma;
pub use crate::src::renderergl1::tr_init::r_greyscale;
pub use crate::src::renderergl1::tr_init::r_intensity;
pub use crate::src::renderergl1::tr_init::r_overBrightBits;
pub use crate::src::renderergl1::tr_init::r_picmip;
pub use crate::src::renderergl1::tr_init::r_roundImagesDown;
pub use crate::src::renderergl1::tr_init::r_simpleMipMaps;
pub use crate::src::renderergl1::tr_init::r_texturebits;
pub use crate::src::renderergl1::tr_init::textureFilterAnisotropic;
pub use crate::src::renderergl1::tr_init::GL_CheckErrors;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shader::R_FindShader;
pub use crate::src::sdl::sdl_gamma::GLimp_SetGamma;
pub use crate::src::sdl::sdl_glimp::qglActiveTextureARB;
pub use crate::src::sdl::sdl_glimp::qglBindTexture;
pub use crate::src::sdl::sdl_glimp::qglDeleteTextures;
pub use crate::src::sdl::sdl_glimp::qglGenTextures;
pub use crate::src::sdl::sdl_glimp::qglTexImage2D;
pub use crate::src::sdl::sdl_glimp::qglTexParameterf;
pub use crate::src::sdl::sdl_glimp::qglTexParameteri;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
pub use crate::stdlib::GLint;
pub use crate::stdlib::GLsizei;
pub use crate::stdlib::GLuint;
pub use crate::stdlib::GLvoid;
pub use crate::stdlib::__ctype_tolower_loc;

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
pub use crate::tr_local_h::glstate_t;
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
pub struct imageExtToLoaderMap_t {
    pub ext: *mut libc::c_char,
    pub ImageLoader: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut *mut libc::c_uchar,
            _: *mut libc::c_int,
            _: *mut libc::c_int,
        ) -> (),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct textureMode_t {
    pub name: *mut libc::c_char,
    pub minimize: libc::c_int,
    pub maximize: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_108 {
    pub c: *mut libc::c_char,
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
// tr_image.c

static mut s_intensitytable: [crate::src::qcommon::q_shared::byte; 256] = [0; 256];

static mut s_gammatable: [libc::c_uchar; 256] = [0; 256];
#[no_mangle]

pub static mut gl_filter_min: libc::c_int = 0x2701 as libc::c_int;
#[no_mangle]

pub static mut gl_filter_max: libc::c_int = 0x2601 as libc::c_int;

static mut hashTable: [*mut crate::tr_common_h::image_t; 1024] =
    [0 as *const crate::tr_common_h::image_t as *mut crate::tr_common_h::image_t; 1024];
/*
** R_GammaCorrect
*/
#[no_mangle]

pub unsafe extern "C" fn R_GammaCorrect(
    mut buffer: *mut crate::src::qcommon::q_shared::byte,
    mut bufSize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < bufSize {
        *buffer.offset(i as isize) = s_gammatable[*buffer.offset(i as isize) as usize];
        i += 1
    }
}
#[no_mangle]

pub static mut modes: [textureMode_t; 6] = [
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            minimize: 0x2600 as libc::c_int,
            maximize: 0x2600 as libc::c_int,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            minimize: 0x2601 as libc::c_int,
            maximize: 0x2601 as libc::c_int,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            minimize: 0x2700 as libc::c_int,
            maximize: 0x2600 as libc::c_int,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            minimize: 0x2701 as libc::c_int,
            maximize: 0x2601 as libc::c_int,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_NEAREST_MIPMAP_LINEAR\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            minimize: 0x2702 as libc::c_int,
            maximize: 0x2600 as libc::c_int,
        };
        init
    },
    {
        let mut init = textureMode_t {
            name: b"GL_LINEAR_MIPMAP_LINEAR\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            minimize: 0x2703 as libc::c_int,
            maximize: 0x2601 as libc::c_int,
        };
        init
    },
];
/*
================
return a hash value for the filename
================
*/

unsafe extern "C" fn generateHashValue(mut fname: *const libc::c_char) -> libc::c_long {
    let mut i: libc::c_int = 0; // don't include extension
    let mut hash: libc::c_long = 0; // damn path names
    let mut letter: libc::c_char = 0;
    hash = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while *fname.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        letter = ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *fname.offset(i as isize) as libc::c_int;
                    __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*fname.offset(i as isize) as libc::c_int)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc())
                    .offset(*fname.offset(i as isize) as libc::c_int as isize)
            }
            __res
        }) as libc::c_char;
        if letter as libc::c_int == '.' as i32 {
            break;
        }
        if letter as libc::c_int == '\\' as i32 {
            letter = '/' as i32 as libc::c_char
        }
        hash += letter as libc::c_long * (i + 119 as libc::c_int) as libc::c_long;
        i += 1
    }
    hash &= (1024 as libc::c_int - 1 as libc::c_int) as libc::c_long;
    return hash;
}
/*
===============
GL_TextureMode
===============
*/
#[no_mangle]

pub unsafe extern "C" fn GL_TextureMode(mut string: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut glt: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if crate::src::qcommon::q_shared::Q_stricmp(modes[i as usize].name, string) == 0 {
            break;
        }
        i += 1
    }
    // hack to prevent trilinear from being set on voodoo,
    // because their driver freaks...
    if i == 5 as libc::c_int
        && crate::src::renderergl1::tr_init::glConfig.hardwareType as libc::c_uint
            == crate::tr_types_h::GLHW_3DFX_2D3D as libc::c_int as libc::c_uint
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b"Refusing to set trilinear on a voodoo.\n\x00" as *const u8 as *const libc::c_char,
        );
        i = 3 as libc::c_int
    }
    if i == 6 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b"bad filter name\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    gl_filter_min = modes[i as usize].minimize;
    gl_filter_max = modes[i as usize].maximize;
    // change all the existing mipmap texture objects
    i = 0 as libc::c_int;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        glt = crate::src::renderergl1::tr_main::tr.images[i as usize];
        if (*glt).flags as libc::c_uint
            & crate::tr_common_h::IMGFLAG_MIPMAP as libc::c_int as libc::c_uint
            != 0
        {
            crate::src::renderergl1::tr_backend::GL_Bind(glt as *mut crate::tr_common_h::image_s);
            crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0x2801 as libc::c_int as crate::stdlib::GLenum,
                gl_filter_min as crate::stdlib::GLfloat,
            );
            crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0x2800 as libc::c_int as crate::stdlib::GLenum,
                gl_filter_max as crate::stdlib::GLfloat,
            );
        }
        i += 1
    }
}
/*
===============
R_SumOfUsedImages
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SumOfUsedImages() -> libc::c_int {
    let mut total: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    total = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        if (*crate::src::renderergl1::tr_main::tr.images[i as usize]).frameUsed
            == crate::src::renderergl1::tr_main::tr.frameCount
        {
            total += (*crate::src::renderergl1::tr_main::tr.images[i as usize]).uploadWidth
                * (*crate::src::renderergl1::tr_main::tr.images[i as usize]).uploadHeight
        }
        i += 1
    }
    return total;
}
/*
===============
R_ImageList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_ImageList_f() {
    let mut i: libc::c_int = 0;
    let mut estTotalSize: libc::c_int = 0 as libc::c_int;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b"\n      -w-- -h-- type  -size- --name-------\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        let mut image: *mut crate::tr_common_h::image_t =
            crate::src::renderergl1::tr_main::tr.images[i as usize];
        let mut format: *mut libc::c_char =
            b"???? \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        let mut sizeSuffix: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut estSize: libc::c_int = 0;
        let mut displaySize: libc::c_int = 0;
        estSize = (*image).uploadHeight * (*image).uploadWidth;
        match (*image).internalFormat {
            35917 => {
                format = b"sDXT1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 64 bits per 16 pixels, so 4 bits per pixel
                estSize /= 2 as libc::c_int
            }
            35919 => format = b"sDXT5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            36493 => format = b"sBPTC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            35954 => format = b"LATC \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            33777 => {
                format = b"DXT1 \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 64 bits per 16 pixels, so 4 bits per pixel
                estSize /= 2 as libc::c_int
            }
            33779 => format = b"DXT5 \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            36492 => format = b"BPTC \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            33697 => {
                format = b"S3TC \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // same as DXT1?
                estSize /= 2 as libc::c_int
            }
            32854 | 32856 | 6408 => {
                format = b"RGBA \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 4 bytes per pixel
                estSize *= 4 as libc::c_int
            }
            32832 | 6409 => {
                format = b"L    \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            32848 | 32849 | 6407 => {
                format = b"RGB  \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 3 bytes per pixel?
                estSize *= 3 as libc::c_int
            }
            32837 | 6410 => {
                format = b"LA   \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 2 bytes per pixel?
                estSize *= 2 as libc::c_int
            }
            35904 | 35905 => {
                format = b"sRGB \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 3 bytes per pixel?
                estSize *= 3 as libc::c_int
            }
            35906 | 35907 => {
                format = b"sRGBA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 4 bytes per pixel?
                estSize *= 4 as libc::c_int
            }
            35910 | 35911 => {
                format = b"sL   \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            35908 | 35909 => {
                format = b"sLA  \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                // 2 byte per pixel?
                estSize *= 2 as libc::c_int
            }
            _ => {}
        }
        // mipmap adds about 50%
        if (*image).flags as libc::c_uint
            & crate::tr_common_h::IMGFLAG_MIPMAP as libc::c_int as libc::c_uint
            != 0
        {
            estSize += estSize / 2 as libc::c_int
        }
        sizeSuffix = b"b \x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        displaySize = estSize;
        if displaySize > 1024 as libc::c_int {
            displaySize /= 1024 as libc::c_int;
            sizeSuffix = b"kb\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        if displaySize > 1024 as libc::c_int {
            displaySize /= 1024 as libc::c_int;
            sizeSuffix = b"Mb\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        if displaySize > 1024 as libc::c_int {
            displaySize /= 1024 as libc::c_int;
            sizeSuffix = b"Gb\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b"%4i: %4ix%4i %s %4i%s %s\n\x00" as *const u8 as *const libc::c_char,
            i,
            (*image).uploadWidth,
            (*image).uploadHeight,
            format,
            displaySize,
            sizeSuffix,
            (*image).imgName.as_mut_ptr(),
        );
        estTotalSize += estSize;
        i += 1
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b" ---------\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b" approx %i bytes\n\x00" as *const u8 as *const libc::c_char,
        estTotalSize,
    );
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b" %i total images\n\n\x00" as *const u8 as *const libc::c_char,
        crate::src::renderergl1::tr_main::tr.numImages,
    );
}
//=======================================================================
/*
================
ResampleTexture

Used to resample images in a more general than quartering fashion.

This will only be filtered properly if the resampled size
is greater than half the original size.

If a larger shrinking is needed, use the mipmap function
before or after.
================
*/

unsafe extern "C" fn ResampleTexture(
    mut in_0: *mut libc::c_uint,
    mut inwidth: libc::c_int,
    mut inheight: libc::c_int,
    mut out: *mut libc::c_uint,
    mut outwidth: libc::c_int,
    mut outheight: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut inrow: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut inrow2: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut frac: libc::c_uint = 0;
    let mut fracstep: libc::c_uint = 0;
    let mut p1: [libc::c_uint; 2048] = [0; 2048];
    let mut p2: [libc::c_uint; 2048] = [0; 2048];
    let mut pix1: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix2: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix3: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut pix4: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    if outwidth > 2048 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ResampleTexture: max width\x00" as *const u8 as *const libc::c_char,
        );
    }
    fracstep = (inwidth * 0x10000 as libc::c_int / outwidth) as libc::c_uint;
    frac = fracstep >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outwidth {
        p1[i as usize] = (4 as libc::c_int as libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = frac.wrapping_add(fracstep);
        i += 1
    }
    frac = (3 as libc::c_int as libc::c_uint).wrapping_mul(fracstep >> 2 as libc::c_int);
    i = 0 as libc::c_int;
    while i < outwidth {
        p2[i as usize] = (4 as libc::c_int as libc::c_uint).wrapping_mul(frac >> 16 as libc::c_int);
        frac = frac.wrapping_add(fracstep);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < outheight {
        inrow = in_0.offset(
            (inwidth
                * ((i as libc::c_double + 0.25f64) * inheight as libc::c_double
                    / outheight as libc::c_double) as libc::c_int) as isize,
        );
        inrow2 = in_0.offset(
            (inwidth
                * ((i as libc::c_double + 0.75f64) * inheight as libc::c_double
                    / outheight as libc::c_double) as libc::c_int) as isize,
        );
        j = 0 as libc::c_int;
        while j < outwidth {
            pix1 =
                (inrow as *mut crate::src::qcommon::q_shared::byte).offset(p1[j as usize] as isize);
            pix2 =
                (inrow as *mut crate::src::qcommon::q_shared::byte).offset(p2[j as usize] as isize);
            pix3 = (inrow2 as *mut crate::src::qcommon::q_shared::byte)
                .offset(p1[j as usize] as isize);
            pix4 = (inrow2 as *mut crate::src::qcommon::q_shared::byte)
                .offset(p2[j as usize] as isize);
            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte)
                .offset(0 as libc::c_int as isize) =
                (*pix1.offset(0 as libc::c_int as isize) as libc::c_int
                    + *pix2.offset(0 as libc::c_int as isize) as libc::c_int
                    + *pix3.offset(0 as libc::c_int as isize) as libc::c_int
                    + *pix4.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte)
                .offset(1 as libc::c_int as isize) =
                (*pix1.offset(1 as libc::c_int as isize) as libc::c_int
                    + *pix2.offset(1 as libc::c_int as isize) as libc::c_int
                    + *pix3.offset(1 as libc::c_int as isize) as libc::c_int
                    + *pix4.offset(1 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte)
                .offset(2 as libc::c_int as isize) =
                (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
                    + *pix2.offset(2 as libc::c_int as isize) as libc::c_int
                    + *pix3.offset(2 as libc::c_int as isize) as libc::c_int
                    + *pix4.offset(2 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *(out.offset(j as isize) as *mut crate::src::qcommon::q_shared::byte)
                .offset(3 as libc::c_int as isize) =
                (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
                    + *pix2.offset(3 as libc::c_int as isize) as libc::c_int
                    + *pix3.offset(3 as libc::c_int as isize) as libc::c_int
                    + *pix4.offset(3 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            j += 1
        }
        i += 1;
        out = out.offset(outwidth as isize)
    }
}
/*
================
R_LightScaleTexture

Scale up the pixel values in a texture to increase the
lighting range
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LightScaleTexture(
    mut in_0: *mut libc::c_uint,
    mut inwidth: libc::c_int,
    mut inheight: libc::c_int,
    mut only_gamma: crate::src::qcommon::q_shared::qboolean,
) {
    if only_gamma as u64 != 0 {
        if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 == 0 {
            let mut i: libc::c_int = 0;
            let mut c: libc::c_int = 0;
            let mut p: *mut crate::src::qcommon::q_shared::byte =
                0 as *mut crate::src::qcommon::q_shared::byte;
            p = in_0 as *mut crate::src::qcommon::q_shared::byte;
            c = inwidth * inheight;
            i = 0 as libc::c_int;
            while i < c {
                *p.offset(0 as libc::c_int as isize) =
                    s_gammatable[*p.offset(0 as libc::c_int as isize) as usize];
                *p.offset(1 as libc::c_int as isize) =
                    s_gammatable[*p.offset(1 as libc::c_int as isize) as usize];
                *p.offset(2 as libc::c_int as isize) =
                    s_gammatable[*p.offset(2 as libc::c_int as isize) as usize];
                i += 1;
                p = p.offset(4 as libc::c_int as isize)
            }
        }
    } else {
        let mut i_0: libc::c_int = 0;
        let mut c_0: libc::c_int = 0;
        let mut p_0: *mut crate::src::qcommon::q_shared::byte =
            0 as *mut crate::src::qcommon::q_shared::byte;
        p_0 = in_0 as *mut crate::src::qcommon::q_shared::byte;
        c_0 = inwidth * inheight;
        if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 != 0 {
            i_0 = 0 as libc::c_int;
            while i_0 < c_0 {
                *p_0.offset(0 as libc::c_int as isize) =
                    s_intensitytable[*p_0.offset(0 as libc::c_int as isize) as usize];
                *p_0.offset(1 as libc::c_int as isize) =
                    s_intensitytable[*p_0.offset(1 as libc::c_int as isize) as usize];
                *p_0.offset(2 as libc::c_int as isize) =
                    s_intensitytable[*p_0.offset(2 as libc::c_int as isize) as usize];
                i_0 += 1;
                p_0 = p_0.offset(4 as libc::c_int as isize)
            }
        } else {
            i_0 = 0 as libc::c_int;
            while i_0 < c_0 {
                *p_0.offset(0 as libc::c_int as isize) = s_gammatable
                    [s_intensitytable[*p_0.offset(0 as libc::c_int as isize) as usize] as usize];
                *p_0.offset(1 as libc::c_int as isize) = s_gammatable
                    [s_intensitytable[*p_0.offset(1 as libc::c_int as isize) as usize] as usize];
                *p_0.offset(2 as libc::c_int as isize) = s_gammatable
                    [s_intensitytable[*p_0.offset(2 as libc::c_int as isize) as usize] as usize];
                i_0 += 1;
                p_0 = p_0.offset(4 as libc::c_int as isize)
            }
        }
    };
}
/*
================
R_MipMap2

Operates in place, quartering the size of the texture
Proper linear filter
================
*/

unsafe extern "C" fn R_MipMap2(
    mut in_0: *mut libc::c_uint,
    mut inWidth: libc::c_int,
    mut inHeight: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut outpix: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut inWidthMask: libc::c_int = 0;
    let mut inHeightMask: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut outWidth: libc::c_int = 0;
    let mut outHeight: libc::c_int = 0;
    let mut temp: *mut libc::c_uint = 0 as *mut libc::c_uint;
    outWidth = inWidth >> 1 as libc::c_int;
    outHeight = inHeight >> 1 as libc::c_int;
    temp = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(outWidth * outHeight * 4 as libc::c_int)
        as *mut libc::c_uint;
    inWidthMask = inWidth - 1 as libc::c_int;
    inHeightMask = inHeight - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outHeight {
        j = 0 as libc::c_int;
        while j < outWidth {
            outpix = temp.offset((i * outWidth) as isize).offset(j as isize)
                as *mut crate::src::qcommon::q_shared::byte;
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                total = 1 as libc::c_int
                    * *(&mut *in_0.offset(
                        ((i * 2 as libc::c_int - 1 as libc::c_int & inHeightMask) * inWidth
                            + (j * 2 as libc::c_int - 1 as libc::c_int & inWidthMask))
                            as isize,
                    ) as *mut libc::c_uint
                        as *mut crate::src::qcommon::q_shared::byte)
                        .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int - 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int - 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 1 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int - 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int - 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 4 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 4 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int - 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 4 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 4 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 1 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 1 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int - 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 2 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 1 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int
                    + 1 as libc::c_int
                        * *(&mut *in_0.offset(
                            ((i * 2 as libc::c_int + 2 as libc::c_int & inHeightMask) * inWidth
                                + (j * 2 as libc::c_int + 2 as libc::c_int & inWidthMask))
                                as isize,
                        ) as *mut libc::c_uint
                            as *mut crate::src::qcommon::q_shared::byte)
                            .offset(k as isize) as libc::c_int;
                *outpix.offset(k as isize) =
                    (total / 36 as libc::c_int) as crate::src::qcommon::q_shared::byte;
                k += 1
            }
            j += 1
        }
        i += 1
    }
    crate::stdlib::memcpy(
        in_0 as *mut libc::c_void,
        temp as *const libc::c_void,
        (outWidth * outHeight * 4 as libc::c_int) as libc::c_ulong,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(temp as *mut libc::c_void);
}
/*
================
R_MipMap

Operates in place, quartering the size of the texture
================
*/

unsafe extern "C" fn R_MipMap(
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i: libc::c_int = 0; // get largest
    let mut j: libc::c_int = 0;
    let mut out: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut row: libc::c_int = 0;
    if (*crate::src::renderergl1::tr_init::r_simpleMipMaps).integer == 0 {
        R_MipMap2(in_0 as *mut libc::c_uint, width, height);
        return;
    }
    if width == 1 as libc::c_int && height == 1 as libc::c_int {
        return;
    }
    row = width * 4 as libc::c_int;
    out = in_0;
    width >>= 1 as libc::c_int;
    height >>= 1 as libc::c_int;
    if width == 0 as libc::c_int || height == 0 as libc::c_int {
        width += height;
        i = 0 as libc::c_int;
        while i < width {
            *out.offset(0 as libc::c_int as isize) =
                (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(4 as libc::c_int as isize) as libc::c_int
                    >> 1 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(1 as libc::c_int as isize) =
                (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(5 as libc::c_int as isize) as libc::c_int
                    >> 1 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(2 as libc::c_int as isize) =
                (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(6 as libc::c_int as isize) as libc::c_int
                    >> 1 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(3 as libc::c_int as isize) =
                (*in_0.offset(3 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(7 as libc::c_int as isize) as libc::c_int
                    >> 1 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            i += 1;
            out = out.offset(4 as libc::c_int as isize);
            in_0 = in_0.offset(8 as libc::c_int as isize)
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < height {
        j = 0 as libc::c_int;
        while j < width {
            *out.offset(0 as libc::c_int as isize) =
                (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(4 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset((row + 0 as libc::c_int) as isize) as libc::c_int
                    + *in_0.offset((row + 4 as libc::c_int) as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(1 as libc::c_int as isize) =
                (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(5 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset((row + 1 as libc::c_int) as isize) as libc::c_int
                    + *in_0.offset((row + 5 as libc::c_int) as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(2 as libc::c_int as isize) =
                (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(6 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset((row + 2 as libc::c_int) as isize) as libc::c_int
                    + *in_0.offset((row + 6 as libc::c_int) as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            *out.offset(3 as libc::c_int as isize) =
                (*in_0.offset(3 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset(7 as libc::c_int as isize) as libc::c_int
                    + *in_0.offset((row + 3 as libc::c_int) as isize) as libc::c_int
                    + *in_0.offset((row + 7 as libc::c_int) as isize) as libc::c_int
                    >> 2 as libc::c_int) as crate::src::qcommon::q_shared::byte;
            j += 1;
            out = out.offset(4 as libc::c_int as isize);
            in_0 = in_0.offset(8 as libc::c_int as isize)
        }
        i += 1;
        in_0 = in_0.offset(row as isize)
    }
}
/*
==================
R_BlendOverTexture

Apply a color blend over a set of pixels
==================
*/

unsafe extern "C" fn R_BlendOverTexture(
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut pixelCount: libc::c_int,
    mut blend: *mut crate::src::qcommon::q_shared::byte,
) {
    let mut i: libc::c_int = 0;
    let mut inverseAlpha: libc::c_int = 0;
    let mut premult: [libc::c_int; 3] = [0; 3];
    inverseAlpha = 255 as libc::c_int - *blend.offset(3 as libc::c_int as isize) as libc::c_int;
    premult[0 as libc::c_int as usize] = *blend.offset(0 as libc::c_int as isize) as libc::c_int
        * *blend.offset(3 as libc::c_int as isize) as libc::c_int;
    premult[1 as libc::c_int as usize] = *blend.offset(1 as libc::c_int as isize) as libc::c_int
        * *blend.offset(3 as libc::c_int as isize) as libc::c_int;
    premult[2 as libc::c_int as usize] = *blend.offset(2 as libc::c_int as isize) as libc::c_int
        * *blend.offset(3 as libc::c_int as isize) as libc::c_int;
    i = 0 as libc::c_int;
    while i < pixelCount {
        *data.offset(0 as libc::c_int as isize) =
            (*data.offset(0 as libc::c_int as isize) as libc::c_int * inverseAlpha
                + premult[0 as libc::c_int as usize]
                >> 9 as libc::c_int) as crate::src::qcommon::q_shared::byte;
        *data.offset(1 as libc::c_int as isize) =
            (*data.offset(1 as libc::c_int as isize) as libc::c_int * inverseAlpha
                + premult[1 as libc::c_int as usize]
                >> 9 as libc::c_int) as crate::src::qcommon::q_shared::byte;
        *data.offset(2 as libc::c_int as isize) =
            (*data.offset(2 as libc::c_int as isize) as libc::c_int * inverseAlpha
                + premult[2 as libc::c_int as usize]
                >> 9 as libc::c_int) as crate::src::qcommon::q_shared::byte;
        i += 1;
        data = data.offset(4 as libc::c_int as isize)
    }
}
#[no_mangle]

pub static mut mipBlendColors: [[crate::src::qcommon::q_shared::byte; 4]; 16] = [
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
    [
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        0 as libc::c_int as crate::src::qcommon::q_shared::byte,
        255 as libc::c_int as crate::src::qcommon::q_shared::byte,
        128 as libc::c_int as crate::src::qcommon::q_shared::byte,
    ],
];
/*
===============
Upload32

===============
*/

unsafe extern "C" fn Upload32(
    mut data: *mut libc::c_uint,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut mipmap: crate::src::qcommon::q_shared::qboolean,
    mut picmip: crate::src::qcommon::q_shared::qboolean,
    mut lightMap: crate::src::qcommon::q_shared::qboolean,
    mut allowCompression: crate::src::qcommon::q_shared::qboolean,
    mut format: *mut libc::c_int,
    mut pUploadWidth: *mut libc::c_int,
    mut pUploadHeight: *mut libc::c_int,
) {
    let mut current_block: u64;
    let mut samples: libc::c_int = 0;
    let mut scaledBuffer: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut resampledBuffer: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut scaled_width: libc::c_int = 0;
    let mut scaled_height: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut scan: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut internalFormat: crate::stdlib::GLenum = 0x1907 as libc::c_int as crate::stdlib::GLenum;
    let mut rMax: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut gMax: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut bMax: libc::c_float = 0 as libc::c_int as libc::c_float;
    //
    // convert to exact power of 2 sizes
    //
    scaled_width = 1 as libc::c_int;
    while scaled_width < width {
        scaled_width <<= 1 as libc::c_int
    }
    scaled_height = 1 as libc::c_int;
    while scaled_height < height {
        scaled_height <<= 1 as libc::c_int
    }
    if (*crate::src::renderergl1::tr_init::r_roundImagesDown).integer != 0 && scaled_width > width {
        scaled_width >>= 1 as libc::c_int
    }
    if (*crate::src::renderergl1::tr_init::r_roundImagesDown).integer != 0 && scaled_height > height
    {
        scaled_height >>= 1 as libc::c_int
    }
    if scaled_width != width || scaled_height != height {
        resampledBuffer = crate::src::renderergl1::tr_main::ri
            .Hunk_AllocateTempMemory
            .expect("non-null function pointer")(
            scaled_width * scaled_height * 4 as libc::c_int
        ) as *mut libc::c_uint;
        ResampleTexture(
            data,
            width,
            height,
            resampledBuffer,
            scaled_width,
            scaled_height,
        );
        data = resampledBuffer;
        width = scaled_width;
        height = scaled_height
    }
    //
    // perform optional picmip operation
    //
    if picmip as u64 != 0 {
        scaled_width >>= (*crate::src::renderergl1::tr_init::r_picmip).integer;
        scaled_height >>= (*crate::src::renderergl1::tr_init::r_picmip).integer
    }
    //
    // clamp to minimum size
    //
    if scaled_width < 1 as libc::c_int {
        scaled_width = 1 as libc::c_int
    }
    if scaled_height < 1 as libc::c_int {
        scaled_height = 1 as libc::c_int
    }
    //
    // clamp to the current upper OpenGL limit
    // scale both axis down equally so we don't have to
    // deal with a half mip resampling
    //
    while scaled_width > crate::src::renderergl1::tr_init::glConfig.maxTextureSize
        || scaled_height > crate::src::renderergl1::tr_init::glConfig.maxTextureSize
    {
        scaled_width >>= 1 as libc::c_int;
        scaled_height >>= 1 as libc::c_int
    }
    scaledBuffer = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(
        (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            .wrapping_mul(scaled_width as libc::c_ulong)
            .wrapping_mul(scaled_height as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_uint;
    //
    // scan the texture for each channel's max values
    // and verify if the alpha channel is being used or not
    //
    c = width * height;
    scan = data as *mut crate::src::qcommon::q_shared::byte;
    samples = 3 as libc::c_int;
    if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
        i = 0 as libc::c_int;
        while i < c {
            let mut luma: crate::src::qcommon::q_shared::byte = (0.2126f32
                * *scan.offset((i * 4 as libc::c_int) as isize) as libc::c_int as libc::c_float
                + 0.7152f32
                    * *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int as libc::c_float
                + 0.0722f32
                    * *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int as libc::c_float)
                as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4 as libc::c_int) as isize) = luma;
            *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) = luma;
            *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize) = luma;
            i += 1
        }
    } else if (*crate::src::renderergl1::tr_init::r_greyscale).value != 0. {
        i = 0 as libc::c_int;
        while i < c {
            let mut luma_0: libc::c_float = 0.2126f32
                * *scan.offset((i * 4 as libc::c_int) as isize) as libc::c_int as libc::c_float
                + 0.7152f32
                    * *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize)
                        as libc::c_int as libc::c_float
                + 0.0722f32
                    * *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
                        as libc::c_int as libc::c_float;
            *scan.offset((i * 4 as libc::c_int) as isize) =
                (*scan.offset((i * 4 as libc::c_int) as isize) as libc::c_int as libc::c_float
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) =
                (*scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int
                    as libc::c_float
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize) =
                (*scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int
                    as libc::c_float
                    * (1.0f32 - (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    + luma_0 * (*crate::src::renderergl1::tr_init::r_greyscale).value)
                    as crate::src::qcommon::q_shared::byte;
            i += 1
        }
    }
    if lightMap as u64 != 0 {
        if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
            internalFormat = 0x1909 as libc::c_int as crate::stdlib::GLenum
        } else {
            internalFormat = 0x1907 as libc::c_int as crate::stdlib::GLenum
        }
    } else {
        i = 0 as libc::c_int;
        while i < c {
            if *scan.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int
                as libc::c_float
                > rMax
            {
                rMax = *scan.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
                    as libc::c_float
            }
            if *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int
                as libc::c_float
                > gMax
            {
                gMax = *scan.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize)
                    as libc::c_float
            }
            if *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int
                as libc::c_float
                > bMax
            {
                bMax = *scan.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
                    as libc::c_float
            }
            if *scan.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int
                != 255 as libc::c_int
            {
                samples = 4 as libc::c_int;
                break;
            } else {
                i += 1
            }
        }
        // select proper internal format
        if samples == 3 as libc::c_int {
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16 as libc::c_int
                    || (*crate::src::renderergl1::tr_init::r_texturebits).integer
                        == 32 as libc::c_int
                {
                    internalFormat = 0x8040 as libc::c_int as crate::stdlib::GLenum
                } else {
                    internalFormat = 0x1909 as libc::c_int as crate::stdlib::GLenum
                }
            } else if allowCompression as libc::c_uint != 0
                && crate::src::renderergl1::tr_init::glConfig.textureCompression as libc::c_uint
                    == crate::tr_types_h::TC_S3TC_ARB as libc::c_int as libc::c_uint
            {
                internalFormat = 0x83f1 as libc::c_int as crate::stdlib::GLenum
            } else if allowCompression as libc::c_uint != 0
                && crate::src::renderergl1::tr_init::glConfig.textureCompression as libc::c_uint
                    == crate::tr_types_h::TC_S3TC as libc::c_int as libc::c_uint
            {
                internalFormat = 0x83a1 as libc::c_int as crate::stdlib::GLenum
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer
                == 16 as libc::c_int
            {
                internalFormat = 0x8050 as libc::c_int as crate::stdlib::GLenum
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer
                == 32 as libc::c_int
            {
                internalFormat = 0x8051 as libc::c_int as crate::stdlib::GLenum
            } else {
                internalFormat = 0x1907 as libc::c_int as crate::stdlib::GLenum
            }
        } else if samples == 4 as libc::c_int {
            if (*crate::src::renderergl1::tr_init::r_greyscale).integer != 0 {
                if (*crate::src::renderergl1::tr_init::r_texturebits).integer == 16 as libc::c_int
                    || (*crate::src::renderergl1::tr_init::r_texturebits).integer
                        == 32 as libc::c_int
                {
                    internalFormat = 0x8045 as libc::c_int as crate::stdlib::GLenum
                } else {
                    internalFormat = 0x190a as libc::c_int as crate::stdlib::GLenum
                }
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer
                == 16 as libc::c_int
            {
                internalFormat = 0x8056 as libc::c_int as crate::stdlib::GLenum
            } else if (*crate::src::renderergl1::tr_init::r_texturebits).integer
                == 32 as libc::c_int
            {
                internalFormat = 0x8058 as libc::c_int as crate::stdlib::GLenum
            } else {
                internalFormat = 0x1908 as libc::c_int as crate::stdlib::GLenum
            }
        }
    }
    // copy or resample data as appropriate for first MIP level
    if scaled_width == width && scaled_height == height {
        if mipmap as u64 == 0 {
            crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0 as libc::c_int,
                internalFormat as crate::stdlib::GLint,
                scaled_width,
                scaled_height,
                0 as libc::c_int,
                0x1908 as libc::c_int as crate::stdlib::GLenum,
                0x1401 as libc::c_int as crate::stdlib::GLenum,
                data as *const libc::c_void,
            );
            *pUploadWidth = scaled_width;
            *pUploadHeight = scaled_height;
            *format = internalFormat as libc::c_int;
            current_block = 1652156287349356005;
        } else {
            crate::stdlib::memcpy(
                scaledBuffer as *mut libc::c_void,
                data as *const libc::c_void,
                (width * height * 4 as libc::c_int) as libc::c_ulong,
            );
            current_block = 7244994750255146185;
        }
    } else {
        // use the normal mip-mapping function to go down from here
        while width > scaled_width || height > scaled_height {
            R_MipMap(
                data as *mut crate::src::qcommon::q_shared::byte,
                width,
                height,
            );
            width >>= 1 as libc::c_int;
            height >>= 1 as libc::c_int;
            if width < 1 as libc::c_int {
                width = 1 as libc::c_int
            }
            if height < 1 as libc::c_int {
                height = 1 as libc::c_int
            }
        }
        crate::stdlib::memcpy(
            scaledBuffer as *mut libc::c_void,
            data as *const libc::c_void,
            (width * height * 4 as libc::c_int) as libc::c_ulong,
        );
        current_block = 7244994750255146185;
    }
    match current_block {
        7244994750255146185 => {
            R_LightScaleTexture(
                scaledBuffer,
                scaled_width,
                scaled_height,
                (mipmap as u64 == 0) as libc::c_int as crate::src::qcommon::q_shared::qboolean,
            );
            *pUploadWidth = scaled_width;
            *pUploadHeight = scaled_height;
            *format = internalFormat as libc::c_int;
            crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0 as libc::c_int,
                internalFormat as crate::stdlib::GLint,
                scaled_width,
                scaled_height,
                0 as libc::c_int,
                0x1908 as libc::c_int as crate::stdlib::GLenum,
                0x1401 as libc::c_int as crate::stdlib::GLenum,
                scaledBuffer as *const libc::c_void,
            );
            if mipmap as u64 != 0 {
                let mut miplevel: libc::c_int = 0;
                miplevel = 0 as libc::c_int;
                while scaled_width > 1 as libc::c_int || scaled_height > 1 as libc::c_int {
                    R_MipMap(
                        scaledBuffer as *mut crate::src::qcommon::q_shared::byte,
                        scaled_width,
                        scaled_height,
                    );
                    scaled_width >>= 1 as libc::c_int;
                    scaled_height >>= 1 as libc::c_int;
                    if scaled_width < 1 as libc::c_int {
                        scaled_width = 1 as libc::c_int
                    }
                    if scaled_height < 1 as libc::c_int {
                        scaled_height = 1 as libc::c_int
                    }
                    miplevel += 1;
                    if (*crate::src::renderergl1::tr_init::r_colorMipLevels).integer != 0 {
                        R_BlendOverTexture(
                            scaledBuffer as *mut crate::src::qcommon::q_shared::byte,
                            scaled_width * scaled_height,
                            mipBlendColors[miplevel as usize].as_mut_ptr(),
                        );
                    }
                    crate::src::sdl::sdl_glimp::qglTexImage2D.expect("non-null function pointer")(
                        0xde1 as libc::c_int as crate::stdlib::GLenum,
                        miplevel,
                        internalFormat as crate::stdlib::GLint,
                        scaled_width,
                        scaled_height,
                        0 as libc::c_int,
                        0x1908 as libc::c_int as crate::stdlib::GLenum,
                        0x1401 as libc::c_int as crate::stdlib::GLenum,
                        scaledBuffer as *const libc::c_void,
                    );
                }
            }
        }
        _ => {}
    }
    if mipmap as u64 != 0 {
        if crate::src::renderergl1::tr_init::textureFilterAnisotropic as u64 != 0 {
            crate::src::sdl::sdl_glimp::qglTexParameteri.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0x84fe as libc::c_int as crate::stdlib::GLenum,
                crate::src::qcommon::q_shared::Com_Clamp(
                    1 as libc::c_int as libc::c_float,
                    crate::src::renderergl1::tr_init::maxAnisotropy as libc::c_float,
                    (*crate::src::renderergl1::tr_init::r_ext_max_anisotropy).integer
                        as libc::c_float,
                ) as crate::stdlib::GLint,
            );
        }
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0x2801 as libc::c_int as crate::stdlib::GLenum,
            gl_filter_min as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0x2800 as libc::c_int as crate::stdlib::GLenum,
            gl_filter_max as crate::stdlib::GLfloat,
        );
    } else {
        if crate::src::renderergl1::tr_init::textureFilterAnisotropic as u64 != 0 {
            crate::src::sdl::sdl_glimp::qglTexParameteri.expect("non-null function pointer")(
                0xde1 as libc::c_int as crate::stdlib::GLenum,
                0x84fe as libc::c_int as crate::stdlib::GLenum,
                1 as libc::c_int,
            );
        }
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0x2801 as libc::c_int as crate::stdlib::GLenum,
            0x2601 as libc::c_int as crate::stdlib::GLfloat,
        );
        crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0x2800 as libc::c_int as crate::stdlib::GLenum,
            0x2601 as libc::c_int as crate::stdlib::GLfloat,
        );
    }
    crate::src::renderergl1::tr_init::GL_CheckErrors();
    if !scaledBuffer.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Hunk_FreeTempMemory
            .expect("non-null function pointer")(scaledBuffer as *mut libc::c_void);
    }
    if !resampledBuffer.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Hunk_FreeTempMemory
            .expect("non-null function pointer")(resampledBuffer as *mut libc::c_void);
    };
}
/*
================
R_CreateImage

This is the only way any image_t are created
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CreateImage(
    mut name: *const libc::c_char,
    mut pic: *mut crate::src::qcommon::q_shared::byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut type_0: crate::tr_common_h::imgType_t,
    mut flags: crate::tr_common_h::imgFlags_t,
    mut _internalFormat: libc::c_int,
) -> *mut crate::tr_common_h::image_t {
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut isLightmap: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut hash: libc::c_long = 0;
    let mut glWrapClampMode: libc::c_int = 0;
    if crate::stdlib::strlen(name) >= 64 as libc::c_int as libc::c_ulong {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"R_CreateImage: \"%s\" is too long\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    if crate::stdlib::strncmp(
        name,
        b"*lightmap\x00" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        isLightmap = crate::src::qcommon::q_shared::qtrue
    }
    if crate::src::renderergl1::tr_main::tr.numImages == 2048 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Error
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"R_CreateImage: MAX_DRAWIMAGES hit\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::renderergl1::tr_main::tr.images
        [crate::src::renderergl1::tr_main::tr.numImages as usize] =
        crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_common_h::image_t>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_common_h::image_t;
    image = crate::src::renderergl1::tr_main::tr.images
        [crate::src::renderergl1::tr_main::tr.numImages as usize];
    crate::src::sdl::sdl_glimp::qglGenTextures.expect("non-null function pointer")(
        1 as libc::c_int,
        &mut (*image).texnum,
    );
    crate::src::renderergl1::tr_main::tr.numImages += 1;
    (*image).type_0 = type_0;
    (*image).flags = flags;
    ::libc::strcpy((*image).imgName.as_mut_ptr(), name);
    (*image).width = width;
    (*image).height = height;
    if flags as libc::c_uint
        & crate::tr_common_h::IMGFLAG_CLAMPTOEDGE as libc::c_int as libc::c_uint
        != 0
    {
        glWrapClampMode = 0x812f as libc::c_int
    } else {
        glWrapClampMode = 0x2901 as libc::c_int
    }
    // lightmaps are always allocated on TMU 1
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() && isLightmap as libc::c_uint != 0
    {
        (*image).TMU = 1 as libc::c_int
    } else {
        (*image).TMU = 0 as libc::c_int
    }
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() {
        crate::src::renderergl1::tr_backend::GL_SelectTexture((*image).TMU);
    }
    crate::src::renderergl1::tr_backend::GL_Bind(image as *mut crate::tr_common_h::image_s);
    Upload32(
        pic as *mut libc::c_uint,
        (*image).width,
        (*image).height,
        ((*image).flags as libc::c_uint
            & crate::tr_common_h::IMGFLAG_MIPMAP as libc::c_int as libc::c_uint)
            as crate::src::qcommon::q_shared::qboolean,
        ((*image).flags as libc::c_uint
            & crate::tr_common_h::IMGFLAG_PICMIP as libc::c_int as libc::c_uint)
            as crate::src::qcommon::q_shared::qboolean,
        isLightmap,
        ((*image).flags as libc::c_uint
            & crate::tr_common_h::IMGFLAG_NO_COMPRESSION as libc::c_int as libc::c_uint
            == 0) as libc::c_int as crate::src::qcommon::q_shared::qboolean,
        &mut (*image).internalFormat,
        &mut (*image).uploadWidth,
        &mut (*image).uploadHeight,
    );
    crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
        0xde1 as libc::c_int as crate::stdlib::GLenum,
        0x2802 as libc::c_int as crate::stdlib::GLenum,
        glWrapClampMode as crate::stdlib::GLfloat,
    );
    crate::src::sdl::sdl_glimp::qglTexParameterf.expect("non-null function pointer")(
        0xde1 as libc::c_int as crate::stdlib::GLenum,
        0x2803 as libc::c_int as crate::stdlib::GLenum,
        glWrapClampMode as crate::stdlib::GLfloat,
    );
    crate::src::renderergl1::tr_init::glState.currenttextures
        [crate::src::renderergl1::tr_init::glState.currenttmu as usize] = 0 as libc::c_int;
    crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
        0xde1 as libc::c_int as crate::stdlib::GLenum,
        0 as libc::c_int as crate::stdlib::GLuint,
    );
    if (*image).TMU == 1 as libc::c_int {
        crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as libc::c_int);
    }
    hash = generateHashValue(name);
    (*image).next = hashTable[hash as usize];
    hashTable[hash as usize] = image;
    return image;
}
// Note that the ordering indicates the order of preference used
// when there are multiple images of different formats available

static mut imageLoaders: [imageExtToLoaderMap_t; 6] = {
    [
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"tga\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_tga::R_LoadTGA
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"jpg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_jpg::R_LoadJPG
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"jpeg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_jpg::R_LoadJPG
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"png\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_png::R_LoadPNG
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"pcx\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_pcx::R_LoadPCX
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = imageExtToLoaderMap_t {
                ext: b"bmp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ImageLoader: Some(
                    crate::src::renderercommon::tr_image_bmp::R_LoadBMP
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut *mut crate::src::qcommon::q_shared::byte,
                            _: *mut libc::c_int,
                            _: *mut libc::c_int,
                        ) -> (),
                ),
            };
            init
        },
    ]
};
// Initialized in run_static_initializers

static mut numImageLoaders: libc::c_int = 0;
/*
=================
R_LoadImage

Loads any of the supported image types into a canonical
32 bit format.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LoadImage(
    mut name: *const libc::c_char,
    mut pic: *mut *mut crate::src::qcommon::q_shared::byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut orgNameFailed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut orgLoader: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut localName: [libc::c_char; 64] = [0; 64];
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut altName: *mut libc::c_char = 0 as *mut libc::c_char;
    *pic = 0 as *mut crate::src::qcommon::q_shared::byte;
    *width = 0 as libc::c_int;
    *height = 0 as libc::c_int;
    crate::src::qcommon::q_shared::Q_strncpyz(localName.as_mut_ptr(), name, 64 as libc::c_int);
    ext = crate::src::qcommon::q_shared::COM_GetExtension(localName.as_mut_ptr());
    if *ext != 0 {
        // Look for the correct loader and use it
        i = 0 as libc::c_int;
        while i < numImageLoaders {
            if crate::src::qcommon::q_shared::Q_stricmp(ext, imageLoaders[i as usize].ext) == 0 {
                // Load
                imageLoaders[i as usize]
                    .ImageLoader
                    .expect("non-null function pointer")(
                    localName.as_mut_ptr(),
                    pic,
                    width,
                    height,
                );
                break;
            } else {
                i += 1
            }
        }
        // A loader was found
        if i < numImageLoaders {
            if (*pic).is_null() {
                // Loader failed, most likely because the file isn't there;
                // try again without the extension
                orgNameFailed = crate::src::qcommon::q_shared::qtrue;
                orgLoader = i;
                crate::src::qcommon::q_shared::COM_StripExtension(
                    name,
                    localName.as_mut_ptr(),
                    64 as libc::c_int,
                );
            } else {
                // Something loaded
                return;
            }
        }
    }
    // Try and find a suitable match using all
    // the image formats supported
    i = 0 as libc::c_int;
    while i < numImageLoaders {
        if !(i == orgLoader) {
            altName = crate::src::qcommon::q_shared::va(
                b"%s.%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                localName.as_mut_ptr(),
                imageLoaders[i as usize].ext,
            );
            // Load
            imageLoaders[i as usize]
                .ImageLoader
                .expect("non-null function pointer")(altName, pic, width, height);
            if !(*pic).is_null() {
                if orgNameFailed as u64 != 0 {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int,
                        b"WARNING: %s not present, using %s instead\n\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        altName,
                    );
                }
                break;
            }
        }
        i += 1
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
===============
R_FindImageFile

Finds or loads the given image.
Returns NULL if it fails, not a default image.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn R_FindImageFile(
    mut name: *const libc::c_char,
    mut type_0: crate::tr_common_h::imgType_t,
    mut flags: crate::tr_common_h::imgFlags_t,
) -> *mut crate::tr_common_h::image_t {
    let mut image: *mut crate::tr_common_h::image_t = 0 as *mut crate::tr_common_h::image_t;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut pic: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut hash: libc::c_long = 0;
    if name.is_null() {
        return 0 as *mut crate::tr_common_h::image_t;
    }
    hash = generateHashValue(name);
    //
    // see if the image is already loaded
    //
    image = hashTable[hash as usize];
    while !image.is_null() {
        if ::libc::strcmp(name, (*image).imgName.as_mut_ptr()) == 0 {
            // the white image can be used with any set of parms, but other mismatches are errors
            if ::libc::strcmp(name, b"*white\x00" as *const u8 as *const libc::c_char) != 0 {
                if (*image).flags as libc::c_uint != flags as libc::c_uint {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int,
                        b"WARNING: reused image %s with mixed flags (%i vs %i)\n\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        (*image).flags as libc::c_uint,
                        flags as libc::c_uint,
                    );
                }
            }
            return image;
        }
        image = (*image).next
    }
    //
    // load the pic from disk
    //
    R_LoadImage(name, &mut pic, &mut width, &mut height);
    if pic.is_null() {
        return 0 as *mut crate::tr_common_h::image_t;
    }
    image = R_CreateImage(
        name as *mut libc::c_char,
        pic,
        width,
        height,
        type_0,
        flags,
        0 as libc::c_int,
    );
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(pic as *mut libc::c_void);
    return image;
}

unsafe extern "C" fn R_CreateDlightImage() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    let mut b: libc::c_int = 0;
    // make a centered inverse-square falloff blob for dynamic lighting
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int {
            let mut d: libc::c_float = 0.;
            d = ((16 as libc::c_int / 2 as libc::c_int) as libc::c_float
                - 0.5f32
                - x as libc::c_float)
                * ((16 as libc::c_int / 2 as libc::c_int) as libc::c_float
                    - 0.5f32
                    - x as libc::c_float)
                + ((16 as libc::c_int / 2 as libc::c_int) as libc::c_float
                    - 0.5f32
                    - y as libc::c_float)
                    * ((16 as libc::c_int / 2 as libc::c_int) as libc::c_float
                        - 0.5f32
                        - y as libc::c_float);
            b = (4000 as libc::c_int as libc::c_float / d) as libc::c_int;
            if b > 255 as libc::c_int {
                b = 255 as libc::c_int
            } else if b < 75 as libc::c_int {
                b = 0 as libc::c_int
            }
            data[y as usize][x as usize][2 as libc::c_int as usize] =
                b as crate::src::qcommon::q_shared::byte;
            data[y as usize][x as usize][1 as libc::c_int as usize] =
                data[y as usize][x as usize][2 as libc::c_int as usize];
            data[y as usize][x as usize][0 as libc::c_int as usize] =
                data[y as usize][x as usize][1 as libc::c_int as usize];
            data[y as usize][x as usize][3 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            y += 1
        }
        x += 1
    }
    crate::src::renderergl1::tr_main::tr.dlightImage = R_CreateImage(
        b"*dlight\x00" as *const u8 as *const libc::c_char,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        16 as libc::c_int,
        16 as libc::c_int,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
        0 as libc::c_int,
    );
}
/*
=================
R_InitFogTable
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitFogTable() {
    let mut i: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut exp: libc::c_float = 0.;
    exp = 0.5f64 as libc::c_float;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        d = crate::stdlib::pow(
            (i as libc::c_float / (256 as libc::c_int - 1 as libc::c_int) as libc::c_float)
                as libc::c_double,
            exp as libc::c_double,
        ) as libc::c_float;
        crate::src::renderergl1::tr_main::tr.fogTable[i as usize] = d;
        i += 1
    }
}
/*
================
R_FogFactor

Returns a 0.0 to 1.0 fog density value
This is called for each texel of the fog texture on startup
and for each vertex of transparent shaders in fog dynamically
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FogFactor(mut s: libc::c_float, mut t: libc::c_float) -> libc::c_float {
    let mut d: libc::c_float = 0.;
    s = (s as libc::c_double - 1.0f64 / 512 as libc::c_int as libc::c_double) as libc::c_float;
    if s < 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int as libc::c_float;
    }
    if (t as libc::c_double) < 1.0f64 / 32 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_float;
    }
    if (t as libc::c_double) < 31.0f64 / 32 as libc::c_int as libc::c_double {
        s *= (t - 1.0f32 / 32.0f32) / (30.0f32 / 32.0f32)
    }
    // we need to leave a lot of clamp range
    s *= 8 as libc::c_int as libc::c_float;
    if s as libc::c_double > 1.0f64 {
        s = 1.0f64 as libc::c_float
    }
    d = crate::src::renderergl1::tr_main::tr.fogTable
        [(s * (256 as libc::c_int - 1 as libc::c_int) as libc::c_float) as libc::c_int as usize];
    return d;
}

unsafe extern "C" fn R_CreateFogImage() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut data: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut d: libc::c_float = 0.;
    data = crate::src::renderergl1::tr_main::ri
        .Hunk_AllocateTempMemory
        .expect("non-null function pointer")(
        256 as libc::c_int * 32 as libc::c_int * 4 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::byte;
    // S is distance, T is depth
    x = 0 as libc::c_int;
    while x < 256 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 32 as libc::c_int {
            d = R_FogFactor(
                (x as libc::c_float + 0.5f32) / 256 as libc::c_int as libc::c_float,
                (y as libc::c_float + 0.5f32) / 32 as libc::c_int as libc::c_float,
            );
            let ref mut fresh0 = *data.offset(
                ((y * 256 as libc::c_int + x) * 4 as libc::c_int + 2 as libc::c_int) as isize,
            );
            *fresh0 = 255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            let ref mut fresh1 = *data.offset(
                ((y * 256 as libc::c_int + x) * 4 as libc::c_int + 1 as libc::c_int) as isize,
            );
            *fresh1 = *fresh0;
            *data.offset(
                ((y * 256 as libc::c_int + x) * 4 as libc::c_int + 0 as libc::c_int) as isize,
            ) = *fresh1;
            *data.offset(
                ((y * 256 as libc::c_int + x) * 4 as libc::c_int + 3 as libc::c_int) as isize,
            ) = (255 as libc::c_int as libc::c_float * d) as crate::src::qcommon::q_shared::byte;
            y += 1
        }
        x += 1
    }
    crate::src::renderergl1::tr_main::tr.fogImage = R_CreateImage(
        b"*fog\x00" as *const u8 as *const libc::c_char,
        data,
        256 as libc::c_int,
        32 as libc::c_int,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_CLAMPTOEDGE,
        0 as libc::c_int,
    );
    crate::src::renderergl1::tr_main::ri
        .Hunk_FreeTempMemory
        .expect("non-null function pointer")(data as *mut libc::c_void);
}

unsafe extern "C" fn R_CreateDefaultImage() {
    let mut x: libc::c_int = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    // the default image will be a box, to allow you to see the mapping coordinates
    crate::stdlib::memset(
        data.as_mut_ptr() as *mut libc::c_void,
        32 as libc::c_int,
        ::std::mem::size_of::<[[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16]>()
            as libc::c_ulong,
    );
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        data[0 as libc::c_int as usize][x as usize][3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        data[0 as libc::c_int as usize][x as usize][2 as libc::c_int as usize] =
            data[0 as libc::c_int as usize][x as usize][3 as libc::c_int as usize];
        data[0 as libc::c_int as usize][x as usize][1 as libc::c_int as usize] =
            data[0 as libc::c_int as usize][x as usize][2 as libc::c_int as usize];
        data[0 as libc::c_int as usize][x as usize][0 as libc::c_int as usize] =
            data[0 as libc::c_int as usize][x as usize][1 as libc::c_int as usize];
        data[x as usize][0 as libc::c_int as usize][3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        data[x as usize][0 as libc::c_int as usize][2 as libc::c_int as usize] =
            data[x as usize][0 as libc::c_int as usize][3 as libc::c_int as usize];
        data[x as usize][0 as libc::c_int as usize][1 as libc::c_int as usize] =
            data[x as usize][0 as libc::c_int as usize][2 as libc::c_int as usize];
        data[x as usize][0 as libc::c_int as usize][0 as libc::c_int as usize] =
            data[x as usize][0 as libc::c_int as usize][1 as libc::c_int as usize];
        data[(16 as libc::c_int - 1 as libc::c_int) as usize][x as usize]
            [3 as libc::c_int as usize] = 255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        data[(16 as libc::c_int - 1 as libc::c_int) as usize][x as usize]
            [2 as libc::c_int as usize] = data[(16 as libc::c_int - 1 as libc::c_int) as usize]
            [x as usize][3 as libc::c_int as usize];
        data[(16 as libc::c_int - 1 as libc::c_int) as usize][x as usize]
            [1 as libc::c_int as usize] = data[(16 as libc::c_int - 1 as libc::c_int) as usize]
            [x as usize][2 as libc::c_int as usize];
        data[(16 as libc::c_int - 1 as libc::c_int) as usize][x as usize]
            [0 as libc::c_int as usize] = data[(16 as libc::c_int - 1 as libc::c_int) as usize]
            [x as usize][1 as libc::c_int as usize];
        data[x as usize][(16 as libc::c_int - 1 as libc::c_int) as usize]
            [3 as libc::c_int as usize] = 255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        data[x as usize][(16 as libc::c_int - 1 as libc::c_int) as usize]
            [2 as libc::c_int as usize] = data[x as usize]
            [(16 as libc::c_int - 1 as libc::c_int) as usize][3 as libc::c_int as usize];
        data[x as usize][(16 as libc::c_int - 1 as libc::c_int) as usize]
            [1 as libc::c_int as usize] = data[x as usize]
            [(16 as libc::c_int - 1 as libc::c_int) as usize][2 as libc::c_int as usize];
        data[x as usize][(16 as libc::c_int - 1 as libc::c_int) as usize]
            [0 as libc::c_int as usize] = data[x as usize]
            [(16 as libc::c_int - 1 as libc::c_int) as usize][1 as libc::c_int as usize];
        x += 1
    }
    crate::src::renderergl1::tr_main::tr.defaultImage = R_CreateImage(
        b"*default\x00" as *const u8 as *const libc::c_char,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        16 as libc::c_int,
        16 as libc::c_int,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_MIPMAP,
        0 as libc::c_int,
    );
}
/*
==================
R_CreateBuiltinImages
==================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CreateBuiltinImages() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut data: [[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16] = [[[0; 4]; 16]; 16];
    R_CreateDefaultImage();
    // we use a solid white image instead of disabling texturing
    crate::stdlib::memset(
        data.as_mut_ptr() as *mut libc::c_void,
        255 as libc::c_int,
        ::std::mem::size_of::<[[[crate::src::qcommon::q_shared::byte; 4]; 16]; 16]>()
            as libc::c_ulong,
    );
    crate::src::renderergl1::tr_main::tr.whiteImage = R_CreateImage(
        b"*white\x00" as *const u8 as *const libc::c_char,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        8 as libc::c_int,
        8 as libc::c_int,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_NONE,
        0 as libc::c_int,
    );
    // with overbright bits active, we need an image which is some fraction of full color,
    // for default lightmaps, etc
    x = 0 as libc::c_int;
    while x < 16 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int {
            data[y as usize][x as usize][2 as libc::c_int as usize] =
                crate::src::renderergl1::tr_main::tr.identityLightByte
                    as crate::src::qcommon::q_shared::byte;
            data[y as usize][x as usize][1 as libc::c_int as usize] =
                data[y as usize][x as usize][2 as libc::c_int as usize];
            data[y as usize][x as usize][0 as libc::c_int as usize] =
                data[y as usize][x as usize][1 as libc::c_int as usize];
            data[y as usize][x as usize][3 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            y += 1
        }
        x += 1
    }
    crate::src::renderergl1::tr_main::tr.identityLightImage = R_CreateImage(
        b"*identityLight\x00" as *const u8 as *const libc::c_char,
        data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        8 as libc::c_int,
        8 as libc::c_int,
        crate::tr_common_h::IMGTYPE_COLORALPHA,
        crate::tr_common_h::IMGFLAG_NONE,
        0 as libc::c_int,
    );
    x = 0 as libc::c_int;
    while x < 32 as libc::c_int {
        // scratchimage is usually used for cinematic drawing
        crate::src::renderergl1::tr_main::tr.scratchImage[x as usize] = R_CreateImage(
            b"*scratch\x00" as *const u8 as *const libc::c_char,
            data.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
            16 as libc::c_int,
            16 as libc::c_int,
            crate::tr_common_h::IMGTYPE_COLORALPHA,
            (crate::tr_common_h::IMGFLAG_PICMIP as libc::c_int
                | crate::tr_common_h::IMGFLAG_CLAMPTOEDGE as libc::c_int)
                as crate::tr_common_h::imgFlags_t,
            0 as libc::c_int,
        );
        x += 1
    }
    R_CreateDlightImage();
    R_CreateFogImage();
}
/*
===============
R_SetColorMappings
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetColorMappings() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut g: libc::c_float = 0.;
    let mut inf: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    // setup the overbright lighting
    crate::src::renderergl1::tr_main::tr.overbrightBits =
        (*crate::src::renderergl1::tr_init::r_overBrightBits).integer;
    if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 == 0 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0 as libc::c_int
        // need hardware gamma for overbright
    }
    // never overbright in windowed mode
    if crate::src::renderergl1::tr_init::glConfig.isFullscreen as u64 == 0 {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0 as libc::c_int
    }
    // allow 2 overbright bits in 24 bit, but only 1 in 16 bit
    if crate::src::renderergl1::tr_init::glConfig.colorBits > 16 as libc::c_int {
        if crate::src::renderergl1::tr_main::tr.overbrightBits > 2 as libc::c_int {
            crate::src::renderergl1::tr_main::tr.overbrightBits = 2 as libc::c_int
        }
    } else if crate::src::renderergl1::tr_main::tr.overbrightBits > 1 as libc::c_int {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 1 as libc::c_int
    }
    if crate::src::renderergl1::tr_main::tr.overbrightBits < 0 as libc::c_int {
        crate::src::renderergl1::tr_main::tr.overbrightBits = 0 as libc::c_int
    }
    crate::src::renderergl1::tr_main::tr.identityLight = 1.0f32
        / ((1 as libc::c_int) << crate::src::renderergl1::tr_main::tr.overbrightBits)
            as libc::c_float;
    crate::src::renderergl1::tr_main::tr.identityLightByte = (255 as libc::c_int as libc::c_float
        * crate::src::renderergl1::tr_main::tr.identityLight)
        as libc::c_int;
    if (*crate::src::renderergl1::tr_init::r_intensity).value <= 1 as libc::c_int as libc::c_float {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_intensity\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*crate::src::renderergl1::tr_init::r_gamma).value < 0.5f32 {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_gamma\x00" as *const u8 as *const libc::c_char,
            b"0.5\x00" as *const u8 as *const libc::c_char,
        );
    } else if (*crate::src::renderergl1::tr_init::r_gamma).value > 3.0f32 {
        crate::src::renderergl1::tr_main::ri
            .Cvar_Set
            .expect("non-null function pointer")(
            b"r_gamma\x00" as *const u8 as *const libc::c_char,
            b"3.0\x00" as *const u8 as *const libc::c_char,
        );
    }
    g = (*crate::src::renderergl1::tr_init::r_gamma).value;
    shift = crate::src::renderergl1::tr_main::tr.overbrightBits;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if g == 1 as libc::c_int as libc::c_float {
            inf = i
        } else {
            inf = (255 as libc::c_int as libc::c_double
                * crate::stdlib::pow(
                    (i as libc::c_float / 255.0f32) as libc::c_double,
                    (1.0f32 / g) as libc::c_double,
                )
                + 0.5f32 as libc::c_double) as libc::c_int
        }
        inf <<= shift;
        if inf < 0 as libc::c_int {
            inf = 0 as libc::c_int
        }
        if inf > 255 as libc::c_int {
            inf = 255 as libc::c_int
        }
        s_gammatable[i as usize] = inf as libc::c_uchar;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        j = (i as libc::c_float * (*crate::src::renderergl1::tr_init::r_intensity).value)
            as libc::c_int;
        if j > 255 as libc::c_int {
            j = 255 as libc::c_int
        }
        s_intensitytable[i as usize] = j as crate::src::qcommon::q_shared::byte;
        i += 1
    }
    if crate::src::renderergl1::tr_init::glConfig.deviceSupportsGamma as u64 != 0 {
        crate::src::sdl::sdl_gamma::GLimp_SetGamma(
            s_gammatable.as_mut_ptr(),
            s_gammatable.as_mut_ptr(),
            s_gammatable.as_mut_ptr(),
        );
    };
}
/*
===============
R_InitImages
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitImages() {
    crate::stdlib::memset(
        hashTable.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut crate::tr_common_h::image_t; 1024]>() as libc::c_ulong,
    );
    // build brightness translation tables
    R_SetColorMappings();
    // create default texture and white texture
    R_CreateBuiltinImages();
}
/*
===============
R_DeleteTextures
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_DeleteTextures() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < crate::src::renderergl1::tr_main::tr.numImages {
        crate::src::sdl::sdl_glimp::qglDeleteTextures.expect("non-null function pointer")(
            1 as libc::c_int,
            &mut (**crate::src::renderergl1::tr_main::tr
                .images
                .as_mut_ptr()
                .offset(i as isize))
            .texnum,
        );
        i += 1
    }
    crate::stdlib::memset(
        crate::src::renderergl1::tr_main::tr.images.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut crate::tr_common_h::image_t; 2048]>() as libc::c_ulong,
    );
    crate::src::renderergl1::tr_main::tr.numImages = 0 as libc::c_int;
    crate::stdlib::memset(
        crate::src::renderergl1::tr_init::glState
            .currenttextures
            .as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
    );
    if crate::src::sdl::sdl_glimp::qglActiveTextureARB.is_some() {
        crate::src::renderergl1::tr_backend::GL_SelectTexture(1 as libc::c_int);
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0 as libc::c_int as crate::stdlib::GLuint,
        );
        crate::src::renderergl1::tr_backend::GL_SelectTexture(0 as libc::c_int);
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0 as libc::c_int as crate::stdlib::GLuint,
        );
    } else {
        crate::src::sdl::sdl_glimp::qglBindTexture.expect("non-null function pointer")(
            0xde1 as libc::c_int as crate::stdlib::GLenum,
            0 as libc::c_int as crate::stdlib::GLuint,
        );
    };
}
/*
============================================================================

SKINS

============================================================================
*/
/*
==================
CommaParse

This is unfortunate, but the skin files aren't
compatible with our normal parsing rules.
==================
*/

unsafe extern "C" fn CommaParse(mut data_p: *mut *mut libc::c_char) -> *mut libc::c_char {
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut com_token: [libc::c_char; 1024] = [0; 1024];
    data = *data_p;
    len = 0 as libc::c_int;
    com_token[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    // make sure incoming data is valid
    if data.is_null() {
        *data_p = 0 as *mut libc::c_char;
        return com_token.as_mut_ptr();
    }
    loop {
        loop
        // skip whitespace
        {
            c = *data as libc::c_int;
            if !(c <= ' ' as i32) {
                break;
            }
            if c == 0 {
                break;
            }
            data = data.offset(1)
        }
        c = *data as libc::c_int;
        // skip double slash comments
        if c == '/' as i32 && *data.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
            data = data.offset(2 as libc::c_int as isize);
            while *data as libc::c_int != 0 && *data as libc::c_int != '\n' as i32 {
                data = data.offset(1)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32
                && *data.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)
            {
                break;
            }
            data = data.offset(2 as libc::c_int as isize);
            while *data as libc::c_int != 0
                && (*data as libc::c_int != '*' as i32
                    || *data.offset(1 as libc::c_int as isize) as libc::c_int != '/' as i32)
            {
                data = data.offset(1)
            }
            if *data != 0 {
                data = data.offset(2 as libc::c_int as isize)
            }
        }
    }
    if c == 0 as libc::c_int {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    // handle quoted strings
    if c == '\"' as i32 {
        data = data.offset(1);
        loop {
            let fresh2 = data;
            data = data.offset(1);
            c = *fresh2 as libc::c_int;
            if c == '\"' as i32 || c == 0 {
                com_token[len as usize] = 0 as libc::c_int as libc::c_char;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if len < 1024 as libc::c_int - 1 as libc::c_int {
                com_token[len as usize] = c as libc::c_char;
                len += 1
            }
        }
    }
    loop
    // parse a regular word
    {
        if len < 1024 as libc::c_int - 1 as libc::c_int {
            com_token[len as usize] = c as libc::c_char;
            len += 1
        }
        data = data.offset(1);
        c = *data as libc::c_int;
        if !(c > 32 as libc::c_int && c != ',' as i32) {
            break;
        }
    }
    com_token[len as usize] = 0 as libc::c_int as libc::c_char;
    *data_p = data;
    return com_token.as_mut_ptr();
}
/*
===============
RE_RegisterSkin

===============
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterSkin(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut parseSurfaces: [crate::tr_local_h::skinSurface_t; 256] =
        [crate::tr_local_h::skinSurface_t {
            name: [0; 64],
            shader: 0 as *mut crate::tr_local_h::shader_t,
        }; 256];
    let mut hSkin: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    let mut surf: *mut crate::tr_local_h::skinSurface_t =
        0 as *mut crate::tr_local_h::skinSurface_t;
    let mut text: C2RustUnnamed_108 = C2RustUnnamed_108 {
        c: 0 as *mut libc::c_char,
    };
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut surfName: [libc::c_char; 64] = [0; 64];
    let mut totalSurfaces: libc::c_int = 0;
    if name.is_null() || *name.offset(0 as libc::c_int as isize) == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int,
            b"Empty name passed to RE_RegisterSkin\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if crate::stdlib::strlen(name) >= 64 as libc::c_int as libc::c_ulong {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_DEVELOPER as libc::c_int,
            b"Skin name exceeds MAX_QPATH\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    // see if the skin is already loaded
    hSkin = 1 as libc::c_int;
    while hSkin < crate::src::renderergl1::tr_main::tr.numSkins {
        skin = crate::src::renderergl1::tr_main::tr.skins[hSkin as usize];
        if crate::src::qcommon::q_shared::Q_stricmp((*skin).name.as_mut_ptr(), name) == 0 {
            if (*skin).numSurfaces == 0 as libc::c_int {
                return 0 as libc::c_int;
                // default skin
            }
            return hSkin;
        }
        hSkin += 1
    }
    // allocate a new skin
    if crate::src::renderergl1::tr_main::tr.numSkins == 1024 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as libc::c_int,
            b"WARNING: RE_RegisterSkin( \'%s\' ) MAX_SKINS hit\n\x00" as *const u8
                as *const libc::c_char,
            name,
        );
        return 0 as libc::c_int;
    }
    crate::src::renderergl1::tr_main::tr.numSkins += 1;
    skin = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::skin_t>() as libc::c_ulong as libc::c_int,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::tr.skins[hSkin as usize] = skin;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*skin).name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    (*skin).numSurfaces = 0 as libc::c_int;
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    // If not a .skin file, load as a single shader
    if ::libc::strcmp(
        name.offset(crate::stdlib::strlen(name) as isize)
            .offset(-(5 as libc::c_int as isize)),
        b".skin\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        (*skin).numSurfaces = 1 as libc::c_int;
        (*skin).surfaces = crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_local_h::skinSurface_t>() as libc::c_ulong
                as libc::c_int,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_local_h::skinSurface_t;
        let ref mut fresh3 = (*(*skin).surfaces.offset(0 as libc::c_int as isize)).shader;
        *fresh3 = crate::src::renderergl1::tr_shader::R_FindShader(
            name,
            -(1 as libc::c_int),
            crate::src::qcommon::q_shared::qtrue,
        ) as *mut crate::tr_local_h::shader_s;
        return hSkin;
    }
    // load and parse the skin file
    crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(name, &mut text.v);
    if text.c.is_null() {
        return 0 as libc::c_int;
    }
    totalSurfaces = 0 as libc::c_int;
    text_p = text.c;
    while !text_p.is_null() && *text_p as libc::c_int != 0 {
        // get surface name
        token = CommaParse(&mut text_p);
        crate::src::qcommon::q_shared::Q_strncpyz(
            surfName.as_mut_ptr(),
            token,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        if *token.offset(0 as libc::c_int as isize) == 0 {
            break;
        }
        // lowercase the surface name so skin compares are faster
        crate::src::qcommon::q_shared::Q_strlwr(surfName.as_mut_ptr());
        if *text_p as libc::c_int == ',' as i32 {
            text_p = text_p.offset(1)
        }
        if !::libc::strstr(token, b"tag_\x00" as *const u8 as *const libc::c_char).is_null() {
            continue;
        }
        // parse the shader name
        token = CommaParse(&mut text_p);
        if (*skin).numSurfaces < 256 as libc::c_int {
            surf = &mut *parseSurfaces
                .as_mut_ptr()
                .offset((*skin).numSurfaces as isize)
                as *mut crate::tr_local_h::skinSurface_t;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*surf).name.as_mut_ptr(),
                surfName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            (*surf).shader = crate::src::renderergl1::tr_shader::R_FindShader(
                token,
                -(1 as libc::c_int),
                crate::src::qcommon::q_shared::qtrue,
            ) as *mut crate::tr_local_h::shader_s;
            (*skin).numSurfaces += 1
        }
        totalSurfaces += 1
    }
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(text.v);
    if totalSurfaces > 256 as libc::c_int {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as libc::c_int,
            b"WARNING: Ignoring excess surfaces (found %d, max is %d) in skin \'%s\'!\n\x00"
                as *const u8 as *const libc::c_char,
            totalSurfaces,
            256 as libc::c_int,
            name,
        );
    }
    // never let a skin have 0 shaders
    if (*skin).numSurfaces == 0 as libc::c_int {
        return 0 as libc::c_int;
        // use default skin
    }
    // copy surfaces to skin
    (*skin).surfaces =
        crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ((*skin).numSurfaces as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::tr_local_h::skinSurface_t,
            >() as libc::c_ulong) as libc::c_int,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_local_h::skinSurface_t;
    crate::stdlib::memcpy(
        (*skin).surfaces as *mut libc::c_void,
        parseSurfaces.as_mut_ptr() as *const libc::c_void,
        ((*skin).numSurfaces as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::tr_local_h::skinSurface_t,
        >() as libc::c_ulong),
    );
    return hSkin;
}
/*
===============
R_InitSkins
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_InitSkins() {
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::tr.numSkins = 1 as libc::c_int;
    // make the default skin have all default shaders
    crate::src::renderergl1::tr_main::tr.skins[0 as libc::c_int as usize] =
        crate::src::renderergl1::tr_main::ri
            .Hunk_Alloc
            .expect("non-null function pointer")(
            ::std::mem::size_of::<crate::tr_local_h::skin_t>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::q_shared::h_low,
        ) as *mut crate::tr_local_h::skin_t;
    skin = crate::src::renderergl1::tr_main::tr.skins[0 as libc::c_int as usize];
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*skin).name.as_mut_ptr(),
        b"<default skin>\x00" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    (*skin).numSurfaces = 1 as libc::c_int;
    (*skin).surfaces = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::skinSurface_t>() as libc::c_ulong as libc::c_int,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::skinSurface_t;
    let ref mut fresh4 = (*(*skin).surfaces.offset(0 as libc::c_int as isize)).shader;
    *fresh4 = crate::src::renderergl1::tr_main::tr.defaultShader;
}
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
/*
===============
R_GetSkinByHandle
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetSkinByHandle(
    mut hSkin: crate::src::qcommon::q_shared::qhandle_t,
) -> *mut crate::tr_local_h::skin_t {
    if hSkin < 1 as libc::c_int || hSkin >= crate::src::renderergl1::tr_main::tr.numSkins {
        return crate::src::renderergl1::tr_main::tr.skins[0 as libc::c_int as usize];
    }
    return crate::src::renderergl1::tr_main::tr.skins[hSkin as usize];
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
R_SkinList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_SkinList_f() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut skin: *mut crate::tr_local_h::skin_t = 0 as *mut crate::tr_local_h::skin_t;
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b"------------------\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < crate::src::renderergl1::tr_main::tr.numSkins {
        skin = crate::src::renderergl1::tr_main::tr.skins[i as usize];
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
            b"%3i:%s (%d surfaces)\n\x00" as *const u8 as *const libc::c_char,
            i,
            (*skin).name.as_mut_ptr(),
            (*skin).numSurfaces,
        );
        j = 0 as libc::c_int;
        while j < (*skin).numSurfaces {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
                b"       %s = %s\n\x00" as *const u8 as *const libc::c_char,
                (*(*skin).surfaces.offset(j as isize)).name.as_mut_ptr(),
                (*(*(*skin).surfaces.offset(j as isize)).shader)
                    .name
                    .as_mut_ptr(),
            );
            j += 1
        }
        i += 1
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as libc::c_int,
        b"------------------\n\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn run_static_initializers() {
    numImageLoaders = (::std::mem::size_of::<[imageExtToLoaderMap_t; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<imageExtToLoaderMap_t>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

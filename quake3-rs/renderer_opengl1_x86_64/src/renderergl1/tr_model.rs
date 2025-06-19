use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Frame_s;
pub use crate::qfiles_h::md3Frame_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qfiles_h::md3Shader_t;
pub use crate::qfiles_h::md3St_t;
pub use crate::qfiles_h::md3Surface_t;
pub use crate::qfiles_h::md3Tag_s;
pub use crate::qfiles_h::md3Tag_t;
pub use crate::qfiles_h::md3Triangle_t;
pub use crate::qfiles_h::md3XyzNormal_t;
pub use crate::qfiles_h::mdrBone_t;
pub use crate::qfiles_h::mdrCompBone_t;
pub use crate::qfiles_h::mdrCompFrame_t;
pub use crate::qfiles_h::mdrFrame_t;
pub use crate::qfiles_h::mdrHeader_t;
pub use crate::qfiles_h::mdrLOD_t;
pub use crate::qfiles_h::mdrSurface_t;
pub use crate::qfiles_h::mdrTag_t;
pub use crate::qfiles_h::mdrTriangle_t;
pub use crate::qfiles_h::mdrVertex_t;
pub use crate::qfiles_h::mdrWeight_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::AxisClear;
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
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_GetExtension;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strlwr;
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

pub use crate::src::renderergl1::tr_animation::MC_UnCompress;
pub use crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands;
pub use crate::src::renderergl1::tr_flares::R_ClearFlares;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::R_Init;
pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_model_iqm::R_IQMLerpTag;
pub use crate::src::renderergl1::tr_model_iqm::R_LoadIQM;
pub use crate::src::renderergl1::tr_scene::RE_ClearScene;
pub use crate::src::renderergl1::tr_shader::R_FindShader;

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
pub use crate::tr_local_h::iqmData_t;
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
pub use crate::tr_local_h::srfIQModel_s;
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
pub struct modelExtToLoaderMap_t {
    pub ext: *mut libc::c_char,
    pub ModelLoader: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: *mut crate::tr_local_h::model_t,
        ) -> crate::src::qcommon::q_shared::qhandle_t,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_120 {
    pub u: *mut u32,
    pub v: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_121 {
    pub u: *mut u32,
    pub v: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_122 {
    pub u: *mut u32,
    pub v: *mut libc::c_void,
}
/*
====================
R_RegisterMD3
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RegisterMD3(
    mut name: *const libc::c_char,
    mut mod_0: *mut crate::tr_local_h::model_t,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut buf: C2RustUnnamed_120 = C2RustUnnamed_120 { u: 0 as *mut u32 };
    let mut lod: i32 = 0;
    let mut ident: i32 = 0;
    let mut loaded: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut numLoaded: i32 = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut namebuf: [libc::c_char; 84] = [0; 84];
    let mut fext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut defex: [libc::c_char; 4] =
        *::std::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"md3\x00");
    numLoaded = 0 as i32;
    ::libc::strcpy(filename.as_mut_ptr(), name);
    fext = ::libc::strchr(filename.as_mut_ptr(), '.' as i32);
    if fext.is_null() {
        fext = defex.as_mut_ptr()
    } else {
        *fext = '\u{0}' as i32 as libc::c_char;
        fext = fext.offset(1)
    }
    lod = 3 as i32 - 1 as i32;
    while lod >= 0 as i32 {
        if lod != 0 {
            crate::src::qcommon::q_shared::Com_sprintf(
                namebuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 84]>() as libc::c_ulong as i32,
                b"%s_%d.%s\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
                lod,
                fext,
            );
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                namebuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 84]>() as libc::c_ulong as i32,
                b"%s.%s\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
                fext,
            );
        }
        crate::src::renderergl1::tr_main::ri
            .FS_ReadFile
            .expect("non-null function pointer")(namebuf.as_mut_ptr(), &mut buf.v);
        if !buf.u.is_null() {
            ident = *buf.u as i32;
            if ident
                == (('3' as i32) << 24 as i32)
                    + (('P' as i32) << 16 as i32)
                    + (('D' as i32) << 8 as i32)
                    + 'I' as i32
            {
                loaded = R_LoadMD3(mod_0, lod, buf.u as *mut libc::c_void, name)
            } else {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_RegisterMD3: unknown fileid for %s\n\x00" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
            crate::src::renderergl1::tr_main::ri
                .FS_FreeFile
                .expect("non-null function pointer")(buf.v);
            if !(loaded as u64 != 0) {
                break;
            }
            (*mod_0).numLods += 1;
            numLoaded += 1
        }
        lod -= 1
    }
    if numLoaded != 0 {
        // duplicate into higher lod spots that weren't
        // loaded, in case the user changes r_lodbias on the fly
        lod -= 1;
        while lod >= 0 as i32 {
            (*mod_0).numLods += 1;
            (*mod_0).md3[lod as usize] = (*mod_0).md3[(lod + 1 as i32) as usize];
            lod -= 1
        }
        return (*mod_0).index;
    }
    (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
    return 0 as i32;
}
/*
====================
R_RegisterMDR
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RegisterMDR(
    mut name: *const libc::c_char,
    mut mod_0: *mut crate::tr_local_h::model_t,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut buf: C2RustUnnamed_121 = C2RustUnnamed_121 { u: 0 as *mut u32 };
    let mut ident: i32 = 0;
    let mut loaded: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut filesize: i32 = 0;
    filesize = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(
        name, &mut buf.v as *mut *mut libc::c_void
    ) as i32;
    if buf.u.is_null() {
        (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
        return 0 as i32;
    }
    ident = *buf.u as i32;
    if ident
        == (('5' as i32) << 24 as i32)
            + (('M' as i32) << 16 as i32)
            + (('D' as i32) << 8 as i32)
            + 'R' as i32
    {
        loaded = R_LoadMDR(mod_0, buf.u as *mut libc::c_void, filesize, name)
    }
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(buf.v);
    if loaded as u64 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_RegisterMDR: couldn\'t load mdr file %s\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
        return 0 as i32;
    }
    return (*mod_0).index;
}
/*
====================
R_RegisterIQM
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_RegisterIQM(
    mut name: *const libc::c_char,
    mut mod_0: *mut crate::tr_local_h::model_t,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut buf: C2RustUnnamed_122 = C2RustUnnamed_122 { u: 0 as *mut u32 };
    let mut loaded: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut filesize: i32 = 0;
    filesize = crate::src::renderergl1::tr_main::ri
        .FS_ReadFile
        .expect("non-null function pointer")(
        name, &mut buf.v as *mut *mut libc::c_void
    ) as i32;
    if buf.u.is_null() {
        (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
        return 0 as i32;
    }
    loaded = crate::src::renderergl1::tr_model_iqm::R_LoadIQM(
        mod_0 as *mut crate::tr_local_h::model_s,
        buf.u as *mut libc::c_void,
        filesize,
        name,
    );
    crate::src::renderergl1::tr_main::ri
        .FS_FreeFile
        .expect("non-null function pointer")(buf.v);
    if loaded as u64 == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_RegisterIQM: couldn\'t load iqm file %s\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
        return 0 as i32;
    }
    return (*mod_0).index;
}
// Note that the ordering indicates the order of preference used
// when there are multiple models of different formats available

static mut modelLoaders: [modelExtToLoaderMap_t; 3] = {
    [
        {
            let mut init = modelExtToLoaderMap_t {
                ext: b"iqm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ModelLoader: Some(
                    R_RegisterIQM
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut crate::tr_local_h::model_t,
                        )
                            -> crate::src::qcommon::q_shared::qhandle_t,
                ),
            };
            init
        },
        {
            let mut init = modelExtToLoaderMap_t {
                ext: b"mdr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ModelLoader: Some(
                    R_RegisterMDR
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut crate::tr_local_h::model_t,
                        )
                            -> crate::src::qcommon::q_shared::qhandle_t,
                ),
            };
            init
        },
        {
            let mut init = modelExtToLoaderMap_t {
                ext: b"md3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ModelLoader: Some(
                    R_RegisterMD3
                        as unsafe extern "C" fn(
                            _: *const libc::c_char,
                            _: *mut crate::tr_local_h::model_t,
                        )
                            -> crate::src::qcommon::q_shared::qhandle_t,
                ),
            };
            init
        },
    ]
};
// Initialized in run_static_initializers

static mut numModelLoaders: i32 = 0;
//===============================================================================
/*
** R_GetModelByHandle
*/
#[no_mangle]

pub unsafe extern "C" fn R_GetModelByHandle(
    mut index: crate::src::qcommon::q_shared::qhandle_t,
) -> *mut crate::tr_local_h::model_t {
    let mut mod_0: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    // out of range gets the defualt model
    if index < 1 as i32 || index >= crate::src::renderergl1::tr_main::tr.numModels {
        return crate::src::renderergl1::tr_main::tr.models[0 as i32 as usize];
    }
    mod_0 = crate::src::renderergl1::tr_main::tr.models[index as usize];
    return mod_0;
}
//===============================================================================
/*
** R_AllocModel
*/
#[no_mangle]

pub unsafe extern "C" fn R_AllocModel() -> *mut crate::tr_local_h::model_t {
    let mut mod_0: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    if crate::src::renderergl1::tr_main::tr.numModels == 1024 as i32 {
        return 0 as *mut crate::tr_local_h::model_t;
    }
    mod_0 = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        ::std::mem::size_of::<crate::tr_local_h::model_t>() as libc::c_ulong as i32,
        crate::src::qcommon::q_shared::h_low,
    ) as *mut crate::tr_local_h::model_t;
    (*mod_0).index = crate::src::renderergl1::tr_main::tr.numModels;
    crate::src::renderergl1::tr_main::tr.models
        [crate::src::renderergl1::tr_main::tr.numModels as usize] = mod_0;
    crate::src::renderergl1::tr_main::tr.numModels += 1;
    return mod_0;
}
/*
====================
RE_RegisterModel

Loads in a model for the given name

Zero will be returned if the model fails to load.
An entry will be retained for failed models as an
optimization to prevent disk rescanning if they are
asked for again.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn RE_RegisterModel(
    mut name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut mod_0: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    let mut hModel: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut orgNameFailed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut orgLoader: i32 = -(1 as i32);
    let mut i: i32 = 0;
    let mut localName: [libc::c_char; 64] = [0; 64];
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut altName: [libc::c_char; 64] = [0; 64];
    if name.is_null() || *name.offset(0 as i32 as isize) == 0 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"RE_RegisterModel: NULL name\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as i32;
    }
    if crate::stdlib::strlen(name) >= 64 as i32 as libc::c_ulong {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"Model name exceeds MAX_QPATH\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as i32;
    }
    //
    // search the currently loaded models
    //
    hModel = 1 as i32;
    while hModel < crate::src::renderergl1::tr_main::tr.numModels {
        mod_0 = crate::src::renderergl1::tr_main::tr.models[hModel as usize];
        if ::libc::strcmp((*mod_0).name.as_mut_ptr(), name) == 0 {
            if (*mod_0).type_0 as u32 == crate::tr_local_h::MOD_BAD as i32 as u32 {
                return 0 as i32;
            }
            return hModel;
        }
        hModel += 1
    }
    // allocate a new model_t
    mod_0 = R_AllocModel();
    if mod_0.is_null() {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"RE_RegisterModel: R_AllocModel() failed for \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            name,
        );
        return 0 as i32;
    }
    // only set the name after the model has been successfully loaded
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*mod_0).name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
    (*mod_0).numLods = 0 as i32;
    //
    // load the files
    //
    crate::src::qcommon::q_shared::Q_strncpyz(localName.as_mut_ptr(), name, 64 as i32);
    ext = crate::src::qcommon::q_shared::COM_GetExtension(localName.as_mut_ptr());
    if *ext != 0 {
        // Look for the correct loader and use it
        i = 0 as i32;
        while i < numModelLoaders {
            if crate::src::qcommon::q_shared::Q_stricmp(ext, modelLoaders[i as usize].ext) == 0 {
                // Load
                hModel = modelLoaders[i as usize]
                    .ModelLoader
                    .expect("non-null function pointer")(
                    localName.as_mut_ptr(), mod_0
                );
                break;
            } else {
                i += 1
            }
        }
        // A loader was found
        if i < numModelLoaders {
            if hModel == 0 {
                // Loader failed, most likely because the file isn't there;
                // try again without the extension
                orgNameFailed = crate::src::qcommon::q_shared::qtrue;
                orgLoader = i;
                crate::src::qcommon::q_shared::COM_StripExtension(
                    name,
                    localName.as_mut_ptr(),
                    64 as i32,
                );
            } else {
                // Something loaded
                return (*mod_0).index;
            }
        }
    }
    // Try and find a suitable match using all
    // the model formats supported
    i = 0 as i32;
    while i < numModelLoaders {
        if !(i == orgLoader) {
            crate::src::qcommon::q_shared::Com_sprintf(
                altName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
                b"%s.%s\x00" as *const u8 as *const libc::c_char,
                localName.as_mut_ptr(),
                modelLoaders[i as usize].ext,
            );
            // Load
            hModel = modelLoaders[i as usize]
                .ModelLoader
                .expect("non-null function pointer")(
                altName.as_mut_ptr(), mod_0
            );
            if hModel != 0 {
                if orgNameFailed as u64 != 0 {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_DEVELOPER as i32,
                        b"WARNING: %s not present, using %s instead\n\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        altName.as_mut_ptr(),
                    );
                }
                break;
            }
        }
        i += 1
    }
    return hModel;
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
// tr_models.c -- model loading and caching
/*
=================
R_LoadMD3
=================
*/

unsafe extern "C" fn R_LoadMD3(
    mut mod_0: *mut crate::tr_local_h::model_t,
    mut lod: i32,
    mut buffer: *mut libc::c_void,
    mut mod_name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pinmodel: *mut crate::qfiles_h::md3Header_t = 0 as *mut crate::qfiles_h::md3Header_t;
    let mut frame: *mut crate::qfiles_h::md3Frame_t = 0 as *mut crate::qfiles_h::md3Frame_t;
    let mut surf: *mut crate::qfiles_h::md3Surface_t = 0 as *mut crate::qfiles_h::md3Surface_t;
    let mut shader: *mut crate::qfiles_h::md3Shader_t = 0 as *mut crate::qfiles_h::md3Shader_t;
    let mut tri: *mut crate::qfiles_h::md3Triangle_t = 0 as *mut crate::qfiles_h::md3Triangle_t;
    let mut st: *mut crate::qfiles_h::md3St_t = 0 as *mut crate::qfiles_h::md3St_t;
    let mut xyz: *mut crate::qfiles_h::md3XyzNormal_t = 0 as *mut crate::qfiles_h::md3XyzNormal_t;
    let mut tag: *mut crate::qfiles_h::md3Tag_t = 0 as *mut crate::qfiles_h::md3Tag_t;
    let mut version: i32 = 0;
    let mut size: i32 = 0;
    pinmodel = buffer as *mut crate::qfiles_h::md3Header_t;
    version = (*pinmodel).version;
    if version != 15 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMD3: %s has wrong version (%i should be %i)\n\x00" as *const u8
                as *const libc::c_char,
            mod_name,
            version,
            15 as i32,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*mod_0).type_0 = crate::tr_local_h::MOD_MESH;
    size = (*pinmodel).ofsEnd;
    (*mod_0).dataSize += size;
    (*mod_0).md3[lod as usize] = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(
        size, crate::src::qcommon::q_shared::h_low
    ) as *mut crate::qfiles_h::md3Header_t;
    crate::stdlib::memcpy(
        (*mod_0).md3[lod as usize] as *mut libc::c_void,
        buffer,
        (*pinmodel).ofsEnd as libc::c_ulong,
    );
    (*(*mod_0).md3[lod as usize]).ident = (*(*mod_0).md3[lod as usize]).ident;
    (*(*mod_0).md3[lod as usize]).version = (*(*mod_0).md3[lod as usize]).version;
    (*(*mod_0).md3[lod as usize]).numFrames = (*(*mod_0).md3[lod as usize]).numFrames;
    (*(*mod_0).md3[lod as usize]).numTags = (*(*mod_0).md3[lod as usize]).numTags;
    (*(*mod_0).md3[lod as usize]).numSurfaces = (*(*mod_0).md3[lod as usize]).numSurfaces;
    (*(*mod_0).md3[lod as usize]).ofsFrames = (*(*mod_0).md3[lod as usize]).ofsFrames;
    (*(*mod_0).md3[lod as usize]).ofsTags = (*(*mod_0).md3[lod as usize]).ofsTags;
    (*(*mod_0).md3[lod as usize]).ofsSurfaces = (*(*mod_0).md3[lod as usize]).ofsSurfaces;
    (*(*mod_0).md3[lod as usize]).ofsEnd = (*(*mod_0).md3[lod as usize]).ofsEnd;
    if (*(*mod_0).md3[lod as usize]).numFrames < 1 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMD3: %s has no frames\n\x00" as *const u8 as *const libc::c_char,
            mod_name,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // swap all the frames
    frame = ((*mod_0).md3[lod as usize] as *mut crate::src::qcommon::q_shared::byte)
        .offset((*(*mod_0).md3[lod as usize]).ofsFrames as isize)
        as *mut crate::qfiles_h::md3Frame_t;
    i = 0 as i32;
    while i < (*(*mod_0).md3[lod as usize]).numFrames {
        (*frame).radius = (*frame).radius;
        j = 0 as i32;
        while j < 3 as i32 {
            (*frame).bounds[0 as i32 as usize][j as usize] =
                (*frame).bounds[0 as i32 as usize][j as usize];
            (*frame).bounds[1 as i32 as usize][j as usize] =
                (*frame).bounds[1 as i32 as usize][j as usize];
            (*frame).localOrigin[j as usize] = (*frame).localOrigin[j as usize];
            j += 1
        }
        i += 1;
        frame = frame.offset(1)
    }
    // swap all the tags
    tag = ((*mod_0).md3[lod as usize] as *mut crate::src::qcommon::q_shared::byte)
        .offset((*(*mod_0).md3[lod as usize]).ofsTags as isize)
        as *mut crate::qfiles_h::md3Tag_t;
    i = 0 as i32;
    while i < (*(*mod_0).md3[lod as usize]).numTags * (*(*mod_0).md3[lod as usize]).numFrames {
        j = 0 as i32;
        while j < 3 as i32 {
            (*tag).origin[j as usize] = (*tag).origin[j as usize];
            (*tag).axis[0 as i32 as usize][j as usize] = (*tag).axis[0 as i32 as usize][j as usize];
            (*tag).axis[1 as i32 as usize][j as usize] = (*tag).axis[1 as i32 as usize][j as usize];
            (*tag).axis[2 as i32 as usize][j as usize] = (*tag).axis[2 as i32 as usize][j as usize];
            j += 1
        }
        i += 1;
        tag = tag.offset(1)
    }
    // swap all the surfaces
    surf = ((*mod_0).md3[lod as usize] as *mut crate::src::qcommon::q_shared::byte)
        .offset((*(*mod_0).md3[lod as usize]).ofsSurfaces as isize)
        as *mut crate::qfiles_h::md3Surface_t;
    i = 0 as i32;
    while i < (*(*mod_0).md3[lod as usize]).numSurfaces {
        (*surf).ident = (*surf).ident;
        (*surf).flags = (*surf).flags;
        (*surf).numFrames = (*surf).numFrames;
        (*surf).numShaders = (*surf).numShaders;
        (*surf).numTriangles = (*surf).numTriangles;
        (*surf).ofsTriangles = (*surf).ofsTriangles;
        (*surf).numVerts = (*surf).numVerts;
        (*surf).ofsShaders = (*surf).ofsShaders;
        (*surf).ofsSt = (*surf).ofsSt;
        (*surf).ofsXyzNormals = (*surf).ofsXyzNormals;
        (*surf).ofsEnd = (*surf).ofsEnd;
        if (*surf).numVerts >= 1000 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"R_LoadMD3: %s has more than %i verts on %s (%i).\n\x00" as *const u8
                    as *const libc::c_char,
                mod_name,
                1000 as i32 - 1 as i32,
                if (*surf).name[0 as i32 as usize] as i32 != 0 {
                    (*surf).name.as_mut_ptr() as *const libc::c_char
                } else {
                    b"a surface\x00" as *const u8 as *const libc::c_char
                },
                (*surf).numVerts,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        if (*surf).numTriangles * 3 as i32 >= 6 as i32 * 1000 as i32 {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"R_LoadMD3: %s has more than %i triangles on %s (%i).\n\x00" as *const u8
                    as *const libc::c_char,
                mod_name,
                6 as i32 * 1000 as i32 / 3 as i32 - 1 as i32,
                if (*surf).name[0 as i32 as usize] as i32 != 0 {
                    (*surf).name.as_mut_ptr() as *const libc::c_char
                } else {
                    b"a surface\x00" as *const u8 as *const libc::c_char
                },
                (*surf).numTriangles,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        // change to surface identifier
        (*surf).ident = crate::tr_local_h::SF_MD3 as i32;
        // lowercase the surface name so skin compares are faster
        crate::src::qcommon::q_shared::Q_strlwr((*surf).name.as_mut_ptr());
        // strip off a trailing _1 or _2
        // this is a crutch for q3data being a mess
        j = crate::stdlib::strlen((*surf).name.as_mut_ptr()) as i32;
        if j > 2 as i32 && (*surf).name[(j - 2 as i32) as usize] as i32 == '_' as i32 {
            (*surf).name[(j - 2 as i32) as usize] = 0 as i32 as libc::c_char
        }
        // register the shaders
        shader = (surf as *mut crate::src::qcommon::q_shared::byte)
            .offset((*surf).ofsShaders as isize)
            as *mut crate::qfiles_h::md3Shader_t;
        j = 0 as i32;
        while j < (*surf).numShaders {
            let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
            sh = crate::src::renderergl1::tr_shader::R_FindShader(
                (*shader).name.as_mut_ptr(),
                -(1 as i32),
                crate::src::qcommon::q_shared::qtrue,
            ) as *mut crate::tr_local_h::shader_s;
            if (*sh).defaultShader as u64 != 0 {
                (*shader).shaderIndex = 0 as i32
            } else {
                (*shader).shaderIndex = (*sh).index
            }
            j += 1;
            shader = shader.offset(1)
        }
        // swap all the triangles
        tri = (surf as *mut crate::src::qcommon::q_shared::byte)
            .offset((*surf).ofsTriangles as isize)
            as *mut crate::qfiles_h::md3Triangle_t;
        j = 0 as i32;
        while j < (*surf).numTriangles {
            (*tri).indexes[0 as i32 as usize] = (*tri).indexes[0 as i32 as usize];
            (*tri).indexes[1 as i32 as usize] = (*tri).indexes[1 as i32 as usize];
            (*tri).indexes[2 as i32 as usize] = (*tri).indexes[2 as i32 as usize];
            j += 1;
            tri = tri.offset(1)
        }
        // swap all the ST
        st = (surf as *mut crate::src::qcommon::q_shared::byte).offset((*surf).ofsSt as isize)
            as *mut crate::qfiles_h::md3St_t;
        j = 0 as i32;
        while j < (*surf).numVerts {
            (*st).st[0 as i32 as usize] = (*st).st[0 as i32 as usize];
            (*st).st[1 as i32 as usize] = (*st).st[1 as i32 as usize];
            j += 1;
            st = st.offset(1)
        }
        // swap all the XyzNormals
        xyz = (surf as *mut crate::src::qcommon::q_shared::byte)
            .offset((*surf).ofsXyzNormals as isize)
            as *mut crate::qfiles_h::md3XyzNormal_t;
        j = 0 as i32;
        while j < (*surf).numVerts * (*surf).numFrames {
            (*xyz).xyz[0 as i32 as usize] = (*xyz).xyz[0 as i32 as usize];
            (*xyz).xyz[1 as i32 as usize] = (*xyz).xyz[1 as i32 as usize];
            (*xyz).xyz[2 as i32 as usize] = (*xyz).xyz[2 as i32 as usize];
            (*xyz).normal = (*xyz).normal;
            j += 1;
            xyz = xyz.offset(1)
        }
        // find the next surface
        surf = (surf as *mut crate::src::qcommon::q_shared::byte).offset((*surf).ofsEnd as isize)
            as *mut crate::qfiles_h::md3Surface_t;
        i += 1
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
R_LoadMDR
=================
*/

unsafe extern "C" fn R_LoadMDR(
    mut mod_0: *mut crate::tr_local_h::model_t,
    mut buffer: *mut libc::c_void,
    mut filesize: i32,
    mut mod_name: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut pinmodel: *mut crate::qfiles_h::mdrHeader_t = 0 as *mut crate::qfiles_h::mdrHeader_t;
    let mut mdr: *mut crate::qfiles_h::mdrHeader_t = 0 as *mut crate::qfiles_h::mdrHeader_t;
    let mut frame: *mut crate::qfiles_h::mdrFrame_t = 0 as *mut crate::qfiles_h::mdrFrame_t;
    let mut lod: *mut crate::qfiles_h::mdrLOD_t = 0 as *mut crate::qfiles_h::mdrLOD_t;
    let mut curlod: *mut crate::qfiles_h::mdrLOD_t = 0 as *mut crate::qfiles_h::mdrLOD_t;
    let mut surf: *mut crate::qfiles_h::mdrSurface_t = 0 as *mut crate::qfiles_h::mdrSurface_t;
    let mut cursurf: *mut crate::qfiles_h::mdrSurface_t = 0 as *mut crate::qfiles_h::mdrSurface_t;
    let mut tri: *mut crate::qfiles_h::mdrTriangle_t = 0 as *mut crate::qfiles_h::mdrTriangle_t;
    let mut curtri: *mut crate::qfiles_h::mdrTriangle_t = 0 as *mut crate::qfiles_h::mdrTriangle_t;
    let mut v: *mut crate::qfiles_h::mdrVertex_t = 0 as *mut crate::qfiles_h::mdrVertex_t;
    let mut curv: *mut crate::qfiles_h::mdrVertex_t = 0 as *mut crate::qfiles_h::mdrVertex_t;
    let mut weight: *mut crate::qfiles_h::mdrWeight_t = 0 as *mut crate::qfiles_h::mdrWeight_t;
    let mut curweight: *mut crate::qfiles_h::mdrWeight_t = 0 as *mut crate::qfiles_h::mdrWeight_t;
    let mut tag: *mut crate::qfiles_h::mdrTag_t = 0 as *mut crate::qfiles_h::mdrTag_t;
    let mut curtag: *mut crate::qfiles_h::mdrTag_t = 0 as *mut crate::qfiles_h::mdrTag_t;
    let mut size: i32 = 0;
    let mut sh: *mut crate::tr_local_h::shader_t = 0 as *mut crate::tr_local_h::shader_t;
    pinmodel = buffer as *mut crate::qfiles_h::mdrHeader_t;
    (*pinmodel).version = (*pinmodel).version;
    if (*pinmodel).version != 2 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMDR: %s has wrong version (%i should be %i)\n\x00" as *const u8
                as *const libc::c_char,
            mod_name,
            (*pinmodel).version,
            2 as i32,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    size = (*pinmodel).ofsEnd;
    if size > filesize {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMDR: Header of %s is broken. Wrong filesize declared!\n\x00" as *const u8
                as *const libc::c_char,
            mod_name,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*mod_0).type_0 = crate::tr_local_h::MOD_MDR;
    (*pinmodel).numFrames = (*pinmodel).numFrames;
    (*pinmodel).numBones = (*pinmodel).numBones;
    (*pinmodel).ofsFrames = (*pinmodel).ofsFrames;
    // This is a model that uses some type of compressed Bones. We don't want to uncompress every bone for each rendered frame
    // over and over again, we'll uncompress it in this function already, so we must adjust the size of the target mdr.
    if (*pinmodel).ofsFrames < 0 as i32 {
        // mdrFrame_t is larger than mdrCompFrame_t:
        size = (size as libc::c_ulong).wrapping_add(
            ((*pinmodel).numFrames as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong),
        ) as i32 as i32;
        // now add enough space for the uncompressed bones.
        size = (size as libc::c_ulong).wrapping_add(
            (((*pinmodel).numFrames * (*pinmodel).numBones) as libc::c_ulong).wrapping_mul(
                (::std::mem::size_of::<crate::qfiles_h::mdrBone_t>() as libc::c_ulong)
                    .wrapping_sub(
                        ::std::mem::size_of::<crate::qfiles_h::mdrCompBone_t>() as libc::c_ulong
                    ),
            ),
        ) as i32 as i32
    }
    // simple bounds check
    if (*pinmodel).numBones < 0 as i32
        || (::std::mem::size_of::<crate::qfiles_h::mdrHeader_t>() as libc::c_ulong).wrapping_add(
            ((*pinmodel).numFrames as libc::c_ulong).wrapping_mul(
                (::std::mem::size_of::<crate::qfiles_h::mdrFrame_t>() as libc::c_ulong)
                    .wrapping_add(
                        (((*pinmodel).numBones - 1 as i32) as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::mdrBone_t>()
                                as libc::c_ulong),
                    ),
            ),
        ) > size as libc::c_ulong
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8 as *const libc::c_char,
            mod_name,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    (*mod_0).dataSize += size;
    mdr = crate::src::renderergl1::tr_main::ri
        .Hunk_Alloc
        .expect("non-null function pointer")(size, crate::src::qcommon::q_shared::h_low)
        as *mut crate::qfiles_h::mdrHeader_t;
    (*mod_0).modelData = mdr as *mut libc::c_void;
    // Copy all the values over from the file and fix endian issues in the process, if necessary.
    (*mdr).ident = (*pinmodel).ident; // Don't need to swap byte order on this one, we already did above.
    (*mdr).version = (*pinmodel).version;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*mdr).name.as_mut_ptr(),
        (*pinmodel).name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    (*mdr).numFrames = (*pinmodel).numFrames;
    (*mdr).numBones = (*pinmodel).numBones;
    (*mdr).numLODs = (*pinmodel).numLODs;
    (*mdr).numTags = (*pinmodel).numTags;
    // We don't care about the other offset values, we'll generate them ourselves while loading.
    (*mod_0).numLods = (*mdr).numLODs;
    if (*mdr).numFrames < 1 as i32 {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMDR: %s has no frames\n\x00" as *const u8 as *const libc::c_char,
            mod_name,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    /* The first frame will be put into the first free space after the header */
    frame = mdr.offset(1 as i32 as isize) as *mut crate::qfiles_h::mdrFrame_t;
    (*mdr).ofsFrames = (frame as *mut crate::src::qcommon::q_shared::byte)
        .offset_from(mdr as *mut crate::src::qcommon::q_shared::byte)
        as isize as i32;
    if (*pinmodel).ofsFrames < 0 as i32 {
        let mut cframe: *mut crate::qfiles_h::mdrCompFrame_t =
            0 as *mut crate::qfiles_h::mdrCompFrame_t;
        // compressed model...
        cframe = (pinmodel as *mut crate::src::qcommon::q_shared::byte)
            .offset(-((*pinmodel).ofsFrames as isize))
            as *mut crate::qfiles_h::mdrCompFrame_t; // No name supplied in the compressed version.
        i = 0 as i32;
        while i < (*mdr).numFrames {
            j = 0 as i32;
            while j < 3 as i32 {
                (*frame).bounds[0 as i32 as usize][j as usize] =
                    (*cframe).bounds[0 as i32 as usize][j as usize];
                (*frame).bounds[1 as i32 as usize][j as usize] =
                    (*cframe).bounds[1 as i32 as usize][j as usize];
                (*frame).localOrigin[j as usize] = (*cframe).localOrigin[j as usize];
                j += 1
            }
            (*frame).radius = (*cframe).radius;
            (*frame).name[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            j = 0 as i32;
            while j < (*mdr).numBones {
                k = 0 as i32;
                while (k as libc::c_ulong)
                    < (::std::mem::size_of::<[u8; 24]>() as libc::c_ulong)
                        .wrapping_div(2 as i32 as libc::c_ulong)
                {
                    // Do swapping for the uncompressing functions. They seem to use shorts
                    // values only, so I assume this will work. Never tested it on other
                    // platforms, though.
                    *((*(*cframe).bones.as_mut_ptr().offset(j as isize))
                        .Comp
                        .as_mut_ptr() as *mut u16)
                        .offset(k as isize) = *((*(*cframe).bones.as_mut_ptr().offset(j as isize))
                        .Comp
                        .as_mut_ptr() as *mut u16)
                        .offset(k as isize);
                    k += 1
                }
                /* Now do the actual uncompressing */
                crate::src::renderergl1::tr_animation::MC_UnCompress(
                    (*(*frame).bones.as_mut_ptr().offset(j as isize))
                        .matrix
                        .as_mut_ptr(),
                    (*(*cframe).bones.as_mut_ptr().offset(j as isize))
                        .Comp
                        .as_mut_ptr(),
                );
                j += 1
            }
            // Next Frame...
            cframe = &mut *(*cframe).bones.as_mut_ptr().offset(j as isize)
                as *mut crate::qfiles_h::mdrCompBone_t
                as *mut crate::qfiles_h::mdrCompFrame_t;
            frame = &mut *(*frame).bones.as_mut_ptr().offset(j as isize)
                as *mut crate::qfiles_h::mdrBone_t
                as *mut crate::qfiles_h::mdrFrame_t;
            i += 1
        }
    } else {
        let mut curframe: *mut crate::qfiles_h::mdrFrame_t = 0 as *mut crate::qfiles_h::mdrFrame_t;
        // uncompressed model...
        //
        curframe = (pinmodel as *mut crate::src::qcommon::q_shared::byte)
            .offset((*pinmodel).ofsFrames as isize)
            as *mut crate::qfiles_h::mdrFrame_t;
        // swap all the frames
        i = 0 as i32;
        while i < (*mdr).numFrames {
            j = 0 as i32;
            while j < 3 as i32 {
                (*frame).bounds[0 as i32 as usize][j as usize] =
                    (*curframe).bounds[0 as i32 as usize][j as usize];
                (*frame).bounds[1 as i32 as usize][j as usize] =
                    (*curframe).bounds[1 as i32 as usize][j as usize];
                (*frame).localOrigin[j as usize] = (*curframe).localOrigin[j as usize];
                j += 1
            }
            (*frame).radius = (*curframe).radius;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*frame).name.as_mut_ptr(),
                (*curframe).name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as i32,
            );
            j = 0 as i32;
            while j
                < ((*mdr).numBones as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::qfiles_h::mdrBone_t>() as libc::c_ulong
                    )
                    .wrapping_div(4 as i32 as libc::c_ulong) as i32
            {
                *((*frame).bones.as_mut_ptr() as *mut f32).offset(j as isize) =
                    *((*curframe).bones.as_mut_ptr() as *mut f32).offset(j as isize);
                j += 1
            }
            curframe = &mut *(*curframe)
                .bones
                .as_mut_ptr()
                .offset((*mdr).numBones as isize)
                as *mut crate::qfiles_h::mdrBone_t
                as *mut crate::qfiles_h::mdrFrame_t;
            frame = &mut *(*frame).bones.as_mut_ptr().offset((*mdr).numBones as isize)
                as *mut crate::qfiles_h::mdrBone_t
                as *mut crate::qfiles_h::mdrFrame_t;
            i += 1
        }
    }
    // frame should now point to the first free address after all frames.
    lod = frame as *mut crate::qfiles_h::mdrLOD_t;
    (*mdr).ofsLODs = (lod as *mut crate::src::qcommon::q_shared::byte)
        .offset_from(mdr as *mut crate::src::qcommon::q_shared::byte)
        as isize as i32;
    curlod = (pinmodel as *mut crate::src::qcommon::q_shared::byte)
        .offset((*pinmodel).ofsLODs as isize) as *mut crate::qfiles_h::mdrLOD_t;
    // swap all the LOD's
    l = 0 as i32;
    while l < (*mdr).numLODs {
        // simple bounds check
        if lod.offset(1 as i32 as isize) as *mut crate::src::qcommon::q_shared::byte
            > (mdr as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
        {
            crate::src::renderergl1::tr_main::ri
                .Printf
                .expect("non-null function pointer")(
                crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8 as *const libc::c_char,
                mod_name,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        (*lod).numSurfaces = (*curlod).numSurfaces;
        // swap all the surfaces
        surf = lod.offset(1 as i32 as isize) as *mut crate::qfiles_h::mdrSurface_t;
        (*lod).ofsSurfaces = (surf as *mut crate::src::qcommon::q_shared::byte)
            .offset_from(lod as *mut crate::src::qcommon::q_shared::byte)
            as isize as i32;
        cursurf = (curlod as *mut crate::src::qcommon::q_shared::byte)
            .offset((*curlod).ofsSurfaces as isize)
            as *mut crate::qfiles_h::mdrSurface_t;
        i = 0 as i32;
        while i < (*lod).numSurfaces {
            // simple bounds check
            if surf.offset(1 as i32 as isize) as *mut crate::src::qcommon::q_shared::byte
                > (mdr as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
            {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            // first do some copying stuff
            (*surf).ident = crate::tr_local_h::SF_MDR as i32;
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*surf).name.as_mut_ptr(),
                (*cursurf).name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*surf).shader.as_mut_ptr(),
                (*cursurf).shader.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            (*surf).ofsHeader = (mdr as *mut crate::src::qcommon::q_shared::byte)
                .offset_from(surf as *mut crate::src::qcommon::q_shared::byte)
                as isize as i32;
            (*surf).numVerts = (*cursurf).numVerts;
            (*surf).numTriangles = (*cursurf).numTriangles;
            // numBoneReferences and BoneReferences generally seem to be unused
            // now do the checks that may fail.
            if (*surf).numVerts >= 1000 as i32 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadMDR: %s has more than %i verts on %s (%i).\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                    1000 as i32 - 1 as i32,
                    if (*surf).name[0 as i32 as usize] as i32 != 0 {
                        (*surf).name.as_mut_ptr() as *const libc::c_char
                    } else {
                        b"a surface\x00" as *const u8 as *const libc::c_char
                    },
                    (*surf).numVerts,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            if (*surf).numTriangles * 3 as i32 >= 6 as i32 * 1000 as i32 {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadMDR: %s has more than %i triangles on %s (%i).\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                    6 as i32 * 1000 as i32 / 3 as i32 - 1 as i32,
                    if (*surf).name[0 as i32 as usize] as i32 != 0 {
                        (*surf).name.as_mut_ptr() as *const libc::c_char
                    } else {
                        b"a surface\x00" as *const u8 as *const libc::c_char
                    },
                    (*surf).numTriangles,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            // lowercase the surface name so skin compares are faster
            crate::src::qcommon::q_shared::Q_strlwr((*surf).name.as_mut_ptr());
            // register the shaders
            sh = crate::src::renderergl1::tr_shader::R_FindShader(
                (*surf).shader.as_mut_ptr(),
                -(1 as i32),
                crate::src::qcommon::q_shared::qtrue,
            ) as *mut crate::tr_local_h::shader_s;
            if (*sh).defaultShader as u64 != 0 {
                (*surf).shaderIndex = 0 as i32
            } else {
                (*surf).shaderIndex = (*sh).index
            }
            // now copy the vertexes.
            v = surf.offset(1 as i32 as isize) as *mut crate::qfiles_h::mdrVertex_t;
            (*surf).ofsVerts = (v as *mut crate::src::qcommon::q_shared::byte)
                .offset_from(surf as *mut crate::src::qcommon::q_shared::byte)
                as isize as i32;
            curv = (cursurf as *mut crate::src::qcommon::q_shared::byte)
                .offset((*cursurf).ofsVerts as isize)
                as *mut crate::qfiles_h::mdrVertex_t;
            j = 0 as i32;
            while j < (*surf).numVerts {
                (*curv).numWeights = (*curv).numWeights;
                // simple bounds check
                if (*curv).numWeights < 0 as i32
                    || (v.offset(1 as i32 as isize) as *mut crate::src::qcommon::q_shared::byte)
                        .offset(
                            (((*curv).numWeights - 1 as i32) as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::mdrWeight_t>()
                                    as libc::c_ulong) as isize,
                        )
                        > (mdr as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
                {
                    crate::src::renderergl1::tr_main::ri
                        .Printf
                        .expect("non-null function pointer")(
                        crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                        b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8
                            as *const libc::c_char,
                        mod_name,
                    );
                    return crate::src::qcommon::q_shared::qfalse;
                }
                (*v).normal[0 as i32 as usize] = (*curv).normal[0 as i32 as usize];
                (*v).normal[1 as i32 as usize] = (*curv).normal[1 as i32 as usize];
                (*v).normal[2 as i32 as usize] = (*curv).normal[2 as i32 as usize];
                (*v).texCoords[0 as i32 as usize] = (*curv).texCoords[0 as i32 as usize];
                (*v).texCoords[1 as i32 as usize] = (*curv).texCoords[1 as i32 as usize];
                (*v).numWeights = (*curv).numWeights;
                weight = &mut *(*v).weights.as_mut_ptr().offset(0 as i32 as isize)
                    as *mut crate::qfiles_h::mdrWeight_t;
                curweight = &mut *(*curv).weights.as_mut_ptr().offset(0 as i32 as isize)
                    as *mut crate::qfiles_h::mdrWeight_t;
                // Now copy all the weights
                k = 0 as i32;
                while k < (*v).numWeights {
                    (*weight).boneIndex = (*curweight).boneIndex;
                    (*weight).boneWeight = (*curweight).boneWeight;
                    (*weight).offset[0 as i32 as usize] = (*curweight).offset[0 as i32 as usize];
                    (*weight).offset[1 as i32 as usize] = (*curweight).offset[1 as i32 as usize];
                    (*weight).offset[2 as i32 as usize] = (*curweight).offset[2 as i32 as usize];
                    weight = weight.offset(1);
                    curweight = curweight.offset(1);
                    k += 1
                }
                v = weight as *mut crate::qfiles_h::mdrVertex_t;
                curv = curweight as *mut crate::qfiles_h::mdrVertex_t;
                j += 1
            }
            // we know the offset to the triangles now:
            tri = v as *mut crate::qfiles_h::mdrTriangle_t;
            (*surf).ofsTriangles = (tri as *mut crate::src::qcommon::q_shared::byte)
                .offset_from(surf as *mut crate::src::qcommon::q_shared::byte)
                as isize as i32;
            curtri = (cursurf as *mut crate::src::qcommon::q_shared::byte)
                .offset((*cursurf).ofsTriangles as isize)
                as *mut crate::qfiles_h::mdrTriangle_t;
            // simple bounds check
            if (*surf).numTriangles < 0 as i32
                || tri.offset((*surf).numTriangles as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    > (mdr as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
            {
                crate::src::renderergl1::tr_main::ri
                    .Printf
                    .expect("non-null function pointer")(
                    crate::src::qcommon::q_shared::PRINT_WARNING as i32,
                    b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8
                        as *const libc::c_char,
                    mod_name,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            j = 0 as i32;
            while j < (*surf).numTriangles {
                (*tri).indexes[0 as i32 as usize] = (*curtri).indexes[0 as i32 as usize];
                (*tri).indexes[1 as i32 as usize] = (*curtri).indexes[1 as i32 as usize];
                (*tri).indexes[2 as i32 as usize] = (*curtri).indexes[2 as i32 as usize];
                tri = tri.offset(1);
                curtri = curtri.offset(1);
                j += 1
            }
            // tri now points to the end of the surface.
            (*surf).ofsEnd = (tri as *mut crate::src::qcommon::q_shared::byte)
                .offset_from(surf as *mut crate::src::qcommon::q_shared::byte)
                as isize as i32;
            surf = tri as *mut crate::qfiles_h::mdrSurface_t;
            // find the next surface.
            cursurf = (cursurf as *mut crate::src::qcommon::q_shared::byte)
                .offset((*cursurf).ofsEnd as isize)
                as *mut crate::qfiles_h::mdrSurface_t;
            i += 1
        }
        // surf points to the next lod now.
        (*lod).ofsEnd = (surf as *mut crate::src::qcommon::q_shared::byte)
            .offset_from(lod as *mut crate::src::qcommon::q_shared::byte)
            as isize as i32;
        lod = surf as *mut crate::qfiles_h::mdrLOD_t;
        // find the next LOD.
        curlod = (curlod as *mut crate::src::qcommon::q_shared::byte)
            .offset((*curlod).ofsEnd as isize) as *mut crate::qfiles_h::mdrLOD_t;
        l += 1
    }
    // lod points to the first tag now, so update the offset too.
    tag = lod as *mut crate::qfiles_h::mdrTag_t;
    (*mdr).ofsTags = (tag as *mut crate::src::qcommon::q_shared::byte)
        .offset_from(mdr as *mut crate::src::qcommon::q_shared::byte)
        as isize as i32;
    curtag = (pinmodel as *mut crate::src::qcommon::q_shared::byte)
        .offset((*pinmodel).ofsTags as isize) as *mut crate::qfiles_h::mdrTag_t;
    // simple bounds check
    if (*mdr).numTags < 0 as i32
        || tag.offset((*mdr).numTags as isize) as *mut crate::src::qcommon::q_shared::byte
            > (mdr as *mut crate::src::qcommon::q_shared::byte).offset(size as isize)
    {
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_WARNING as i32,
            b"R_LoadMDR: %s has broken structure.\n\x00" as *const u8 as *const libc::c_char,
            mod_name,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    i = 0 as i32;
    while i < (*mdr).numTags {
        (*tag).boneIndex = (*curtag).boneIndex;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*tag).name.as_mut_ptr(),
            (*curtag).name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as i32,
        );
        tag = tag.offset(1);
        curtag = curtag.offset(1);
        i += 1
    }
    // And finally we know the real offset to the end.
    (*mdr).ofsEnd = (tag as *mut crate::src::qcommon::q_shared::byte)
        .offset_from(mdr as *mut crate::src::qcommon::q_shared::byte)
        as isize as i32;
    // phew! we're done.
    return crate::src::qcommon::q_shared::qtrue;
}
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
//=============================================================================
/*
** RE_BeginRegistration
*/
#[no_mangle]

pub unsafe extern "C" fn RE_BeginRegistration(mut glconfigOut: *mut crate::tr_types_h::glconfig_t) {
    crate::src::renderergl1::tr_init::R_Init(); // force markleafs to regenerate
    *glconfigOut = crate::src::renderergl1::tr_init::glConfig;
    crate::src::renderergl1::tr_cmds::R_IssuePendingRenderCommands();
    crate::src::renderergl1::tr_main::tr.viewCluster = -(1 as i32);
    crate::src::renderergl1::tr_flares::R_ClearFlares();
    crate::src::renderergl1::tr_scene::RE_ClearScene();
    crate::src::renderergl1::tr_main::tr.registered = crate::src::qcommon::q_shared::qtrue;
}
//=============================================================================
/*
===============
R_ModelInit
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_ModelInit() {
    let mut mod_0: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    // leave a space for NULL model
    crate::src::renderergl1::tr_main::tr.numModels = 0 as i32;
    mod_0 = R_AllocModel();
    (*mod_0).type_0 = crate::tr_local_h::MOD_BAD;
}
/*
================
R_Modellist_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_Modellist_f() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut mod_0: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    let mut total: i32 = 0;
    let mut lods: i32 = 0;
    total = 0 as i32;
    i = 1 as i32;
    while i < crate::src::renderergl1::tr_main::tr.numModels {
        mod_0 = crate::src::renderergl1::tr_main::tr.models[i as usize];
        lods = 1 as i32;
        j = 1 as i32;
        while j < 3 as i32 {
            if !(*mod_0).md3[j as usize].is_null()
                && (*mod_0).md3[j as usize] != (*mod_0).md3[(j - 1 as i32) as usize]
            {
                lods += 1
            }
            j += 1
        }
        crate::src::renderergl1::tr_main::ri
            .Printf
            .expect("non-null function pointer")(
            crate::src::qcommon::q_shared::PRINT_ALL as i32,
            b"%8i : (%i) %s\n\x00" as *const u8 as *const libc::c_char,
            (*mod_0).dataSize,
            lods,
            (*mod_0).name.as_mut_ptr(),
        );
        total += (*mod_0).dataSize;
        i += 1
    }
    crate::src::renderergl1::tr_main::ri
        .Printf
        .expect("non-null function pointer")(
        crate::src::qcommon::q_shared::PRINT_ALL as i32,
        b"%8i : Total models\n\x00" as *const u8 as *const libc::c_char,
        total,
    );
    // not working right with new hunk
}
//=============================================================================
/*
================
R_GetTag
================
*/

unsafe extern "C" fn R_GetTag(
    mut mod_0: *mut crate::qfiles_h::md3Header_t,
    mut frame: i32,
    mut tagName: *const libc::c_char,
) -> *mut crate::qfiles_h::md3Tag_t {
    let mut tag: *mut crate::qfiles_h::md3Tag_t = 0 as *mut crate::qfiles_h::md3Tag_t;
    let mut i: i32 = 0;
    if frame >= (*mod_0).numFrames {
        // it is possible to have a bad frame while changing models, so don't error
        frame = (*mod_0).numFrames - 1 as i32
    }
    tag = ((mod_0 as *mut crate::src::qcommon::q_shared::byte).offset((*mod_0).ofsTags as isize)
        as *mut crate::qfiles_h::md3Tag_t)
        .offset((frame * (*mod_0).numTags) as isize);
    i = 0 as i32;
    while i < (*mod_0).numTags {
        if ::libc::strcmp((*tag).name.as_mut_ptr(), tagName) == 0 {
            return tag;
            // found it
        }
        i += 1;
        tag = tag.offset(1)
    }
    return 0 as *mut crate::qfiles_h::md3Tag_t;
}
#[no_mangle]

pub unsafe extern "C" fn R_GetAnimTag(
    mut mod_0: *mut crate::qfiles_h::mdrHeader_t,
    mut framenum: i32,
    mut tagName: *const libc::c_char,
    mut dest: *mut crate::qfiles_h::md3Tag_t,
) -> *mut crate::qfiles_h::md3Tag_t {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut frameSize: i32 = 0;
    let mut frame: *mut crate::qfiles_h::mdrFrame_t = 0 as *mut crate::qfiles_h::mdrFrame_t;
    let mut tag: *mut crate::qfiles_h::mdrTag_t = 0 as *mut crate::qfiles_h::mdrTag_t;
    if framenum >= (*mod_0).numFrames {
        // it is possible to have a bad frame while changing models, so don't error
        framenum = (*mod_0).numFrames - 1 as i32
    }
    tag = (mod_0 as *mut crate::src::qcommon::q_shared::byte).offset((*mod_0).ofsTags as isize)
        as *mut crate::qfiles_h::mdrTag_t;
    i = 0 as i32;
    while i < (*mod_0).numTags {
        if ::libc::strcmp((*tag).name.as_mut_ptr(), tagName) == 0 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*dest).name.as_mut_ptr(),
                (*tag).name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
            // uncompressed model...
            //
            frameSize = &mut *(*(0 as *mut crate::qfiles_h::mdrFrame_t))
                .bones
                .as_mut_ptr()
                .offset((*mod_0).numBones as isize)
                as *mut crate::qfiles_h::mdrBone_t
                as crate::stdlib::intptr_t as i32;
            frame = (mod_0 as *mut crate::src::qcommon::q_shared::byte)
                .offset((*mod_0).ofsFrames as isize)
                .offset((framenum * frameSize) as isize)
                as *mut crate::qfiles_h::mdrFrame_t;
            j = 0 as i32;
            while j < 3 as i32 {
                k = 0 as i32;
                while k < 3 as i32 {
                    (*dest).axis[j as usize][k as usize] = (*(*frame)
                        .bones
                        .as_mut_ptr()
                        .offset((*tag).boneIndex as isize))
                    .matrix[k as usize][j as usize];
                    k += 1
                }
                j += 1
            }
            (*dest).origin[0 as i32 as usize] = (*(*frame)
                .bones
                .as_mut_ptr()
                .offset((*tag).boneIndex as isize))
            .matrix[0 as i32 as usize][3 as i32 as usize];
            (*dest).origin[1 as i32 as usize] = (*(*frame)
                .bones
                .as_mut_ptr()
                .offset((*tag).boneIndex as isize))
            .matrix[1 as i32 as usize][3 as i32 as usize];
            (*dest).origin[2 as i32 as usize] = (*(*frame)
                .bones
                .as_mut_ptr()
                .offset((*tag).boneIndex as isize))
            .matrix[2 as i32 as usize][3 as i32 as usize];
            return dest;
        }
        i += 1;
        tag = tag.offset(1)
    }
    return 0 as *mut crate::qfiles_h::md3Tag_t;
}
/*
================
R_LerpTag
================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LerpTag(
    mut tag: *mut crate::src::qcommon::q_shared::orientation_t,
    mut handle: crate::src::qcommon::q_shared::qhandle_t,
    mut startFrame: i32,
    mut endFrame: i32,
    mut frac: f32,
    mut tagName: *const libc::c_char,
) -> i32 {
    let mut start: *mut crate::qfiles_h::md3Tag_t = 0 as *mut crate::qfiles_h::md3Tag_t;
    let mut end: *mut crate::qfiles_h::md3Tag_t = 0 as *mut crate::qfiles_h::md3Tag_t;
    let mut start_space: crate::qfiles_h::md3Tag_t = crate::qfiles_h::md3Tag_t {
        name: [0; 64],
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    let mut end_space: crate::qfiles_h::md3Tag_t = crate::qfiles_h::md3Tag_t {
        name: [0; 64],
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    let mut i: i32 = 0;
    let mut frontLerp: f32 = 0.;
    let mut backLerp: f32 = 0.;
    let mut model: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    model = R_GetModelByHandle(handle);
    if (*model).md3[0 as i32 as usize].is_null() {
        if (*model).type_0 as u32 == crate::tr_local_h::MOD_MDR as i32 as u32 {
            start = R_GetAnimTag(
                (*model).modelData as *mut crate::qfiles_h::mdrHeader_t,
                startFrame,
                tagName,
                &mut start_space,
            );
            end = R_GetAnimTag(
                (*model).modelData as *mut crate::qfiles_h::mdrHeader_t,
                endFrame,
                tagName,
                &mut end_space,
            )
        } else if (*model).type_0 as u32 == crate::tr_local_h::MOD_IQM as i32 as u32 {
            return crate::src::renderergl1::tr_model_iqm::R_IQMLerpTag(
                tag as *mut crate::src::qcommon::q_shared::orientation_t,
                (*model).modelData as *mut crate::tr_local_h::iqmData_t
                    as *mut crate::tr_local_h::iqmData_t,
                startFrame,
                endFrame,
                frac,
                tagName,
            );
        } else {
            end = 0 as *mut crate::qfiles_h::md3Tag_t;
            start = end
        }
    } else {
        start = R_GetTag((*model).md3[0 as i32 as usize], startFrame, tagName);
        end = R_GetTag((*model).md3[0 as i32 as usize], endFrame, tagName)
    }
    if start.is_null() || end.is_null() {
        crate::src::qcommon::q_math::AxisClear((*tag).axis.as_mut_ptr());
        (*tag).origin[2 as i32 as usize] = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
        (*tag).origin[1 as i32 as usize] = (*tag).origin[2 as i32 as usize];
        (*tag).origin[0 as i32 as usize] = (*tag).origin[1 as i32 as usize];
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    frontLerp = frac;
    backLerp = 1.0f32 - frac;
    i = 0 as i32;
    while i < 3 as i32 {
        (*tag).origin[i as usize] =
            (*start).origin[i as usize] * backLerp + (*end).origin[i as usize] * frontLerp;
        (*tag).axis[0 as i32 as usize][i as usize] = (*start).axis[0 as i32 as usize][i as usize]
            * backLerp
            + (*end).axis[0 as i32 as usize][i as usize] * frontLerp;
        (*tag).axis[1 as i32 as usize][i as usize] = (*start).axis[1 as i32 as usize][i as usize]
            * backLerp
            + (*end).axis[1 as i32 as usize][i as usize] * frontLerp;
        (*tag).axis[2 as i32 as usize][i as usize] = (*start).axis[2 as i32 as usize][i as usize]
            * backLerp
            + (*end).axis[2 as i32 as usize][i as usize] * frontLerp;
        i += 1
    }
    crate::src::qcommon::q_math::VectorNormalize((*tag).axis[0 as i32 as usize].as_mut_ptr());
    crate::src::qcommon::q_math::VectorNormalize((*tag).axis[1 as i32 as usize].as_mut_ptr());
    crate::src::qcommon::q_math::VectorNormalize((*tag).axis[2 as i32 as usize].as_mut_ptr());
    return crate::src::qcommon::q_shared::qtrue as i32;
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
/*
====================
R_ModelBounds
====================
*/
#[no_mangle]

pub unsafe extern "C" fn R_ModelBounds(
    mut handle: crate::src::qcommon::q_shared::qhandle_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut model: *mut crate::tr_local_h::model_t = 0 as *mut crate::tr_local_h::model_t;
    model = R_GetModelByHandle(handle);
    if (*model).type_0 as u32 == crate::tr_local_h::MOD_BRUSH as i32 as u32 {
        *mins.offset(0 as i32 as isize) =
            (*(*model).bmodel).bounds[0 as i32 as usize][0 as i32 as usize];
        *mins.offset(1 as i32 as isize) =
            (*(*model).bmodel).bounds[0 as i32 as usize][1 as i32 as usize];
        *mins.offset(2 as i32 as isize) =
            (*(*model).bmodel).bounds[0 as i32 as usize][2 as i32 as usize];
        *maxs.offset(0 as i32 as isize) =
            (*(*model).bmodel).bounds[1 as i32 as usize][0 as i32 as usize];
        *maxs.offset(1 as i32 as isize) =
            (*(*model).bmodel).bounds[1 as i32 as usize][1 as i32 as usize];
        *maxs.offset(2 as i32 as isize) =
            (*(*model).bmodel).bounds[1 as i32 as usize][2 as i32 as usize];
        return;
    } else {
        if (*model).type_0 as u32 == crate::tr_local_h::MOD_MESH as i32 as u32 {
            let mut header: *mut crate::qfiles_h::md3Header_t =
                0 as *mut crate::qfiles_h::md3Header_t;
            let mut frame: *mut crate::qfiles_h::md3Frame_t = 0 as *mut crate::qfiles_h::md3Frame_t;
            header = (*model).md3[0 as i32 as usize];
            frame = (header as *mut crate::src::qcommon::q_shared::byte)
                .offset((*header).ofsFrames as isize)
                as *mut crate::qfiles_h::md3Frame_t;
            *mins.offset(0 as i32 as isize) = (*frame).bounds[0 as i32 as usize][0 as i32 as usize];
            *mins.offset(1 as i32 as isize) = (*frame).bounds[0 as i32 as usize][1 as i32 as usize];
            *mins.offset(2 as i32 as isize) = (*frame).bounds[0 as i32 as usize][2 as i32 as usize];
            *maxs.offset(0 as i32 as isize) = (*frame).bounds[1 as i32 as usize][0 as i32 as usize];
            *maxs.offset(1 as i32 as isize) = (*frame).bounds[1 as i32 as usize][1 as i32 as usize];
            *maxs.offset(2 as i32 as isize) = (*frame).bounds[1 as i32 as usize][2 as i32 as usize];
            return;
        } else {
            if (*model).type_0 as u32 == crate::tr_local_h::MOD_MDR as i32 as u32 {
                let mut header_0: *mut crate::qfiles_h::mdrHeader_t =
                    0 as *mut crate::qfiles_h::mdrHeader_t;
                let mut frame_0: *mut crate::qfiles_h::mdrFrame_t =
                    0 as *mut crate::qfiles_h::mdrFrame_t;
                header_0 = (*model).modelData as *mut crate::qfiles_h::mdrHeader_t;
                frame_0 = (header_0 as *mut crate::src::qcommon::q_shared::byte)
                    .offset((*header_0).ofsFrames as isize)
                    as *mut crate::qfiles_h::mdrFrame_t;
                *mins.offset(0 as i32 as isize) =
                    (*frame_0).bounds[0 as i32 as usize][0 as i32 as usize];
                *mins.offset(1 as i32 as isize) =
                    (*frame_0).bounds[0 as i32 as usize][1 as i32 as usize];
                *mins.offset(2 as i32 as isize) =
                    (*frame_0).bounds[0 as i32 as usize][2 as i32 as usize];
                *maxs.offset(0 as i32 as isize) =
                    (*frame_0).bounds[1 as i32 as usize][0 as i32 as usize];
                *maxs.offset(1 as i32 as isize) =
                    (*frame_0).bounds[1 as i32 as usize][1 as i32 as usize];
                *maxs.offset(2 as i32 as isize) =
                    (*frame_0).bounds[1 as i32 as usize][2 as i32 as usize];
                return;
            } else {
                if (*model).type_0 as u32 == crate::tr_local_h::MOD_IQM as i32 as u32 {
                    let mut iqmData: *mut crate::tr_local_h::iqmData_t =
                        0 as *mut crate::tr_local_h::iqmData_t;
                    iqmData = (*model).modelData as *mut crate::tr_local_h::iqmData_t;
                    if !(*iqmData).bounds.is_null() {
                        *mins.offset(0 as i32 as isize) =
                            *(*iqmData).bounds.offset(0 as i32 as isize);
                        *mins.offset(1 as i32 as isize) =
                            *(*iqmData).bounds.offset(1 as i32 as isize);
                        *mins.offset(2 as i32 as isize) =
                            *(*iqmData).bounds.offset(2 as i32 as isize);
                        *maxs.offset(0 as i32 as isize) = *(*iqmData)
                            .bounds
                            .offset(3 as i32 as isize)
                            .offset(0 as i32 as isize);
                        *maxs.offset(1 as i32 as isize) = *(*iqmData)
                            .bounds
                            .offset(3 as i32 as isize)
                            .offset(1 as i32 as isize);
                        *maxs.offset(2 as i32 as isize) = *(*iqmData)
                            .bounds
                            .offset(3 as i32 as isize)
                            .offset(2 as i32 as isize);
                        return;
                    }
                }
            }
        }
    }
    let ref mut fresh0 = *mins.offset(2 as i32 as isize);
    *fresh0 = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh1 = *mins.offset(1 as i32 as isize);
    *fresh1 = *fresh0;
    *mins.offset(0 as i32 as isize) = *fresh1;
    let ref mut fresh2 = *maxs.offset(2 as i32 as isize);
    *fresh2 = 0 as i32 as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh3 = *maxs.offset(1 as i32 as isize);
    *fresh3 = *fresh2;
    *maxs.offset(0 as i32 as isize) = *fresh3;
}
unsafe extern "C" fn run_static_initializers() {
    numModelLoaders = (::std::mem::size_of::<[modelExtToLoaderMap_t; 3]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<modelExtToLoaderMap_t>() as libc::c_ulong)
        as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

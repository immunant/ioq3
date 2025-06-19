use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as i32 as isize) = *v1.offset(1 as i32 as isize)
            * *v2.offset(2 as i32 as isize)
            - *v1.offset(2 as i32 as isize) * *v2.offset(1 as i32 as isize);
        *cross.offset(1 as i32 as isize) = *v1.offset(2 as i32 as isize)
            * *v2.offset(0 as i32 as isize)
            - *v1.offset(0 as i32 as isize) * *v2.offset(2 as i32 as isize);
        *cross.offset(2 as i32 as isize) = *v1.offset(0 as i32 as isize)
            * *v2.offset(1 as i32 as isize)
            - *v1.offset(1 as i32 as isize) * *v2.offset(0 as i32 as isize);
    }
    // __Q_SHARED_H
}

pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::qgl_h::Beginproc;
pub use crate::qgl_h::Color3fproc;
pub use crate::qgl_h::Color4fproc;
pub use crate::qgl_h::ColorMaskproc;
pub use crate::qgl_h::Disableproc;
pub use crate::qgl_h::Enableproc;
pub use crate::qgl_h::Endproc;
pub use crate::qgl_h::GetBooleanvproc;
pub use crate::qgl_h::LoadIdentityproc;
pub use crate::qgl_h::StencilFuncproc;
pub use crate::qgl_h::StencilOpproc;
pub use crate::qgl_h::Vertex3fproc;
pub use crate::qgl_h::Vertex3fvproc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec2_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::renderergl1::tr_backend::backEnd;
pub use crate::src::renderergl1::tr_backend::GL_Bind;
pub use crate::src::renderergl1::tr_backend::GL_Cull;
pub use crate::src::renderergl1::tr_backend::GL_State;
pub use crate::src::renderergl1::tr_init::glConfig;
pub use crate::src::renderergl1::tr_init::r_shadows;
pub use crate::src::renderergl1::tr_main::tr;
pub use crate::src::renderergl1::tr_shade::tess;
pub use crate::src::renderergl1::tr_shadows::q_shared_h::CrossProduct;
pub use crate::src::sdl::sdl_glimp::qglBegin;
pub use crate::src::sdl::sdl_glimp::qglColor3f;
pub use crate::src::sdl::sdl_glimp::qglColor4f;
pub use crate::src::sdl::sdl_glimp::qglColorMask;
pub use crate::src::sdl::sdl_glimp::qglDisable;
pub use crate::src::sdl::sdl_glimp::qglEnable;
pub use crate::src::sdl::sdl_glimp::qglEnd;
pub use crate::src::sdl::sdl_glimp::qglGetBooleanv;
pub use crate::src::sdl::sdl_glimp::qglLoadIdentity;
pub use crate::src::sdl::sdl_glimp::qglStencilFunc;
pub use crate::src::sdl::sdl_glimp::qglStencilOp;
pub use crate::src::sdl::sdl_glimp::qglVertex3f;
pub use crate::src::sdl::sdl_glimp::qglVertex3fv;

pub use crate::stdlib::GLboolean;
pub use crate::stdlib::GLenum;
pub use crate::stdlib::GLfloat;
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edgeDef_t {
    pub i2: i32,
    pub facing: i32,
}

static mut edgeDefs: [[edgeDef_t; 32]; 1000] = [[edgeDef_t { i2: 0, facing: 0 }; 32]; 1000];

static mut numEdgeDefs: [i32; 1000] = [0; 1000];

static mut facing: [i32; 2000] = [0; 2000];

static mut shadowXyz: [vec3_t; 1000] = [[0.; 3]; 1000];
#[no_mangle]

pub unsafe extern "C" fn R_AddEdgeDef(mut i1: i32, mut i2: i32, mut facing_0: i32) {
    let mut c: i32 = 0;
    c = numEdgeDefs[i1 as usize];
    if c == 32 as i32 {
        return;
        // overflow
    }
    edgeDefs[i1 as usize][c as usize].i2 = i2;
    edgeDefs[i1 as usize][c as usize].facing = facing_0;
    numEdgeDefs[i1 as usize] += 1;
}
#[no_mangle]

pub unsafe extern "C" fn R_RenderShadowEdges() {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut c2: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut i2: i32 = 0;
    let mut _c_edges: i32 = 0;
    let mut _c_rejected: i32 = 0;
    let mut hit: [i32; 2] = [0; 2];
    // an edge is NOT a silhouette edge if its face doesn't face the light,
    // or if it has a reverse paired edge that also faces the light.
    // A well behaved polyhedron would have exactly two faces for each edge,
    // but lots of models have dangling edges or overfanned edges
    _c_edges = 0 as i32;
    _c_rejected = 0 as i32;
    i = 0 as i32;
    while i < tess.numVertexes {
        c = numEdgeDefs[i as usize];
        j = 0 as i32;
        while j < c {
            if !(edgeDefs[i as usize][j as usize].facing == 0) {
                hit[0 as i32 as usize] = 0 as i32;
                hit[1 as i32 as usize] = 0 as i32;
                i2 = edgeDefs[i as usize][j as usize].i2;
                c2 = numEdgeDefs[i2 as usize];
                k = 0 as i32;
                while k < c2 {
                    if edgeDefs[i2 as usize][k as usize].i2 == i {
                        hit[edgeDefs[i2 as usize][k as usize].facing as usize] += 1
                    }
                    k += 1
                }
                // if it doesn't share the edge with another front facing
                // triangle, it is a sil edge
                if hit[1 as i32 as usize] == 0 as i32 {
                    qglBegin.expect("non-null function pointer")(0x5 as i32 as GLenum);
                    qglVertex3fv.expect("non-null function pointer")(
                        tess.xyz[i as usize].as_mut_ptr(),
                    );
                    qglVertex3fv.expect("non-null function pointer")(
                        shadowXyz[i as usize].as_mut_ptr(),
                    );
                    qglVertex3fv.expect("non-null function pointer")(
                        tess.xyz[i2 as usize].as_mut_ptr(),
                    );
                    qglVertex3fv.expect("non-null function pointer")(
                        shadowXyz[i2 as usize].as_mut_ptr(),
                    );
                    qglEnd.expect("non-null function pointer")();
                    _c_edges += 1
                } else {
                    _c_rejected += 1
                }
            }
            j += 1
        }
        i += 1
    }
}
/*
=================
RB_ShadowTessEnd

triangleFromEdge[ v1 ][ v2 ]


  set triangle from edge( v1, v2, tri )
  if ( facing[ triangleFromEdge[ v1 ][ v2 ] ] && !facing[ triangleFromEdge[ v2 ][ v1 ] ) {
  }
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ShadowTessEnd() {
    let mut i: i32 = 0;
    let mut numTris: i32 = 0;
    let mut lightDir: vec3_t = [0.; 3];
    let mut rgba: [GLboolean; 4] = [0; 4];
    if glConfig.stencilBits < 4 as i32 {
        return;
    }
    lightDir[0 as i32 as usize] = (*backEnd.currentEntity).lightDir[0 as i32 as usize];
    lightDir[1 as i32 as usize] = (*backEnd.currentEntity).lightDir[1 as i32 as usize];
    lightDir[2 as i32 as usize] = (*backEnd.currentEntity).lightDir[2 as i32 as usize];
    // project vertexes away from light direction
    i = 0 as i32;
    while i < tess.numVertexes {
        shadowXyz[i as usize][0 as i32 as usize] = tess.xyz[i as usize][0 as i32 as usize]
            + lightDir[0 as i32 as usize] * -(512 as i32) as f32;
        shadowXyz[i as usize][1 as i32 as usize] = tess.xyz[i as usize][1 as i32 as usize]
            + lightDir[1 as i32 as usize] * -(512 as i32) as f32;
        shadowXyz[i as usize][2 as i32 as usize] = tess.xyz[i as usize][2 as i32 as usize]
            + lightDir[2 as i32 as usize] * -(512 as i32) as f32;
        i += 1
    }
    // decide which triangles face the light
    crate::stdlib::memset(
        numEdgeDefs.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (4 as i32 * tess.numVertexes) as libc::c_ulong,
    );
    numTris = tess.numIndexes / 3 as i32;
    i = 0 as i32;
    while i < numTris {
        let mut i1: i32 = 0;
        let mut i2: i32 = 0;
        let mut i3: i32 = 0;
        let mut d1: vec3_t = [0.; 3];
        let mut d2: vec3_t = [0.; 3];
        let mut normal: vec3_t = [0.; 3];
        let mut v1: *mut f32 = 0 as *mut f32;
        let mut v2: *mut f32 = 0 as *mut f32;
        let mut v3: *mut f32 = 0 as *mut f32;
        let mut d: f32 = 0.;
        i1 = tess.indexes[(i * 3 as i32 + 0 as i32) as usize] as i32;
        i2 = tess.indexes[(i * 3 as i32 + 1 as i32) as usize] as i32;
        i3 = tess.indexes[(i * 3 as i32 + 2 as i32) as usize] as i32;
        v1 = tess.xyz[i1 as usize].as_mut_ptr();
        v2 = tess.xyz[i2 as usize].as_mut_ptr();
        v3 = tess.xyz[i3 as usize].as_mut_ptr();
        d1[0 as i32 as usize] = *v2.offset(0 as i32 as isize) - *v1.offset(0 as i32 as isize);
        d1[1 as i32 as usize] = *v2.offset(1 as i32 as isize) - *v1.offset(1 as i32 as isize);
        d1[2 as i32 as usize] = *v2.offset(2 as i32 as isize) - *v1.offset(2 as i32 as isize);
        d2[0 as i32 as usize] = *v3.offset(0 as i32 as isize) - *v1.offset(0 as i32 as isize);
        d2[1 as i32 as usize] = *v3.offset(1 as i32 as isize) - *v1.offset(1 as i32 as isize);
        d2[2 as i32 as usize] = *v3.offset(2 as i32 as isize) - *v1.offset(2 as i32 as isize);
        CrossProduct(
            d1.as_mut_ptr() as *const vec_t,
            d2.as_mut_ptr() as *const vec_t,
            normal.as_mut_ptr(),
        );
        d = normal[0 as i32 as usize] * lightDir[0 as i32 as usize]
            + normal[1 as i32 as usize] * lightDir[1 as i32 as usize]
            + normal[2 as i32 as usize] * lightDir[2 as i32 as usize];
        if d > 0 as i32 as f32 {
            facing[i as usize] = 1 as i32
        } else {
            facing[i as usize] = 0 as i32
        }
        // create the edges
        R_AddEdgeDef(i1, i2, facing[i as usize]);
        R_AddEdgeDef(i2, i3, facing[i as usize]);
        R_AddEdgeDef(i3, i1, facing[i as usize]);
        i += 1
    }
    // draw the silhouette edges
    GL_Bind(tr.whiteImage as *mut image_s);
    GL_State((0x2 as i32 | 0x10 as i32) as libc::c_ulong);
    qglColor3f.expect("non-null function pointer")(0.2f32, 0.2f32, 0.2f32);
    // don't write to the color buffer
    qglGetBooleanv.expect("non-null function pointer")(0xc23 as i32 as GLenum, rgba.as_mut_ptr());
    qglColorMask.expect("non-null function pointer")(
        0 as i32 as GLboolean,
        0 as i32 as GLboolean,
        0 as i32 as GLboolean,
        0 as i32 as GLboolean,
    );
    qglEnable.expect("non-null function pointer")(0xb90 as i32 as GLenum);
    qglStencilFunc.expect("non-null function pointer")(
        0x207 as i32 as GLenum,
        1 as i32,
        255 as i32 as GLuint,
    );
    GL_Cull(CT_BACK_SIDED as i32);
    qglStencilOp.expect("non-null function pointer")(
        0x1e00 as i32 as GLenum,
        0x1e00 as i32 as GLenum,
        0x1e02 as i32 as GLenum,
    );
    R_RenderShadowEdges();
    GL_Cull(CT_FRONT_SIDED as i32);
    qglStencilOp.expect("non-null function pointer")(
        0x1e00 as i32 as GLenum,
        0x1e00 as i32 as GLenum,
        0x1e03 as i32 as GLenum,
    );
    R_RenderShadowEdges();
    // reenable writing to the color buffer
    qglColorMask.expect("non-null function pointer")(
        rgba[0 as i32 as usize],
        rgba[1 as i32 as usize],
        rgba[2 as i32 as usize],
        rgba[3 as i32 as usize],
    );
}
/*
=================
RB_ShadowFinish

Darken everything that is is a shadow volume.
We have to delay this until everything has been shadowed,
because otherwise shadows from different body parts would
overlap and double darken.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ShadowFinish() {
    if (*r_shadows).integer != 2 as i32 {
        return;
    }
    if glConfig.stencilBits < 4 as i32 {
        return;
    }
    qglEnable.expect("non-null function pointer")(0xb90 as i32 as GLenum);
    qglStencilFunc.expect("non-null function pointer")(
        0x205 as i32 as GLenum,
        0 as i32,
        255 as i32 as GLuint,
    );
    qglDisable.expect("non-null function pointer")(0x3000 as i32 as GLenum);
    GL_Cull(CT_TWO_SIDED as i32);
    GL_Bind(tr.whiteImage as *mut image_s);
    qglLoadIdentity.expect("non-null function pointer")();
    qglColor3f.expect("non-null function pointer")(0.6f32, 0.6f32, 0.6f32);
    GL_State((0x100 as i32 | 0x3 as i32 | 0x10 as i32) as libc::c_ulong);
    //	qglColor3f( 1, 0, 0 );
    //	GL_State( GLS_DEPTHMASK_TRUE | GLS_SRCBLEND_ONE | GLS_DSTBLEND_ZERO );
    qglBegin.expect("non-null function pointer")(0x7 as i32 as GLenum);
    qglVertex3f.expect("non-null function pointer")(
        -(100 as i32) as GLfloat,
        100 as i32 as GLfloat,
        -(10 as i32) as GLfloat,
    );
    qglVertex3f.expect("non-null function pointer")(
        100 as i32 as GLfloat,
        100 as i32 as GLfloat,
        -(10 as i32) as GLfloat,
    );
    qglVertex3f.expect("non-null function pointer")(
        100 as i32 as GLfloat,
        -(100 as i32) as GLfloat,
        -(10 as i32) as GLfloat,
    );
    qglVertex3f.expect("non-null function pointer")(
        -(100 as i32) as GLfloat,
        -(100 as i32) as GLfloat,
        -(10 as i32) as GLfloat,
    );
    qglEnd.expect("non-null function pointer")();
    qglColor4f.expect("non-null function pointer")(
        1 as i32 as GLfloat,
        1 as i32 as GLfloat,
        1 as i32 as GLfloat,
        1 as i32 as GLfloat,
    );
    qglDisable.expect("non-null function pointer")(0xb90 as i32 as GLenum);
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
=================
RB_ProjectionShadowDeform

=================
*/
#[no_mangle]

pub unsafe extern "C" fn RB_ProjectionShadowDeform() {
    let mut xyz: *mut f32 = 0 as *mut f32;
    let mut i: i32 = 0;
    let mut h: f32 = 0.;
    let mut ground: vec3_t = [0.; 3];
    let mut light: vec3_t = [0.; 3];
    let mut groundDist: f32 = 0.;
    let mut d: f32 = 0.;
    let mut lightDir: vec3_t = [0.; 3];
    xyz = tess.xyz.as_mut_ptr() as *mut f32;
    ground[0 as i32 as usize] = backEnd.or.axis[0 as i32 as usize][2 as i32 as usize];
    ground[1 as i32 as usize] = backEnd.or.axis[1 as i32 as usize][2 as i32 as usize];
    ground[2 as i32 as usize] = backEnd.or.axis[2 as i32 as usize][2 as i32 as usize];
    groundDist = backEnd.or.origin[2 as i32 as usize] - (*backEnd.currentEntity).e.shadowPlane;
    lightDir[0 as i32 as usize] = (*backEnd.currentEntity).lightDir[0 as i32 as usize];
    lightDir[1 as i32 as usize] = (*backEnd.currentEntity).lightDir[1 as i32 as usize];
    lightDir[2 as i32 as usize] = (*backEnd.currentEntity).lightDir[2 as i32 as usize];
    d = lightDir[0 as i32 as usize] * ground[0 as i32 as usize]
        + lightDir[1 as i32 as usize] * ground[1 as i32 as usize]
        + lightDir[2 as i32 as usize] * ground[2 as i32 as usize];
    // don't let the shadows get too long or go negative
    if (d as f64) < 0.5f64 {
        lightDir[0 as i32 as usize] = (lightDir[0 as i32 as usize] as f64
            + ground[0 as i32 as usize] as f64 * (0.5f64 - d as f64))
            as vec_t;
        lightDir[1 as i32 as usize] = (lightDir[1 as i32 as usize] as f64
            + ground[1 as i32 as usize] as f64 * (0.5f64 - d as f64))
            as vec_t;
        lightDir[2 as i32 as usize] = (lightDir[2 as i32 as usize] as f64
            + ground[2 as i32 as usize] as f64 * (0.5f64 - d as f64))
            as vec_t;
        d = lightDir[0 as i32 as usize] * ground[0 as i32 as usize]
            + lightDir[1 as i32 as usize] * ground[1 as i32 as usize]
            + lightDir[2 as i32 as usize] * ground[2 as i32 as usize]
    }
    d = (1.0f64 / d as f64) as f32;
    light[0 as i32 as usize] = lightDir[0 as i32 as usize] * d;
    light[1 as i32 as usize] = lightDir[1 as i32 as usize] * d;
    light[2 as i32 as usize] = lightDir[2 as i32 as usize] * d;
    i = 0 as i32;
    while i < tess.numVertexes {
        h = *xyz.offset(0 as i32 as isize) * ground[0 as i32 as usize]
            + *xyz.offset(1 as i32 as isize) * ground[1 as i32 as usize]
            + *xyz.offset(2 as i32 as isize) * ground[2 as i32 as usize]
            + groundDist;
        *xyz.offset(0 as i32 as isize) -= light[0 as i32 as usize] * h;
        *xyz.offset(1 as i32 as isize) -= light[1 as i32 as usize] * h;
        *xyz.offset(2 as i32 as isize) -= light[2 as i32 as usize] * h;
        i += 1;
        xyz = xyz.offset(4 as i32 as isize)
    }
}

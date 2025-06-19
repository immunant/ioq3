use ::libc;

pub mod q_shared_h {

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

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as i32 as isize) * *v.offset(0 as i32 as isize)
                + *v.offset(1 as i32 as isize) * *v.offset(1 as i32 as isize)
                + *v.offset(2 as i32 as isize) * *v.offset(2 as i32 as isize)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }

    // __Q_SHARED_H
}

pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::md3Header_t;
pub use crate::src::jpeg_8c::jerror::C2RustUnnamed_1;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
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
pub use crate::src::renderergl1::tr_light::q_shared_h::VectorLength;
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

pub use crate::src::renderergl1::tr_main::ri;
pub use crate::src::renderergl1::tr_main::tr;

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
pub use crate::tr_local_h::dlight_t;
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
pub use crate::tr_local_h::srfGridMesh_s;
pub use crate::tr_local_h::srfGridMesh_t;
pub use crate::tr_local_h::srfPoly_s;
pub use crate::tr_local_h::srfSurfaceFace_t;
pub use crate::tr_local_h::srfTriangles_t;
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
extern "C" {
    /*
    =============================================================================

    LIGHT SAMPLING

    =============================================================================
    */
    #[no_mangle]
    pub static mut r_ambientScale: *mut cvar_t;
    #[no_mangle]
    pub static mut r_directedScale: *mut cvar_t;
    #[no_mangle]
    pub static mut r_debugLight: *mut cvar_t;
}
// never calculate a range less than this to prevent huge light numbers
/*
===============
R_TransformDlights

Transforms the origins of an array of dlights.
Used by both the front end (for DlightBmodel) and
the back end (before doing the lighting calculation)
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_TransformDlights(
    mut count: i32,
    mut dl: *mut dlight_t,
    mut or: *mut orientationr_t,
) {
    let mut i: i32 = 0;
    let mut temp: vec3_t = [0.; 3];
    i = 0 as i32;
    while i < count {
        temp[0 as i32 as usize] = (*dl).origin[0 as i32 as usize] - (*or).origin[0 as i32 as usize];
        temp[1 as i32 as usize] = (*dl).origin[1 as i32 as usize] - (*or).origin[1 as i32 as usize];
        temp[2 as i32 as usize] = (*dl).origin[2 as i32 as usize] - (*or).origin[2 as i32 as usize];
        (*dl).transformed[0 as i32 as usize] = temp[0 as i32 as usize]
            * (*or).axis[0 as i32 as usize][0 as i32 as usize]
            + temp[1 as i32 as usize] * (*or).axis[0 as i32 as usize][1 as i32 as usize]
            + temp[2 as i32 as usize] * (*or).axis[0 as i32 as usize][2 as i32 as usize];
        (*dl).transformed[1 as i32 as usize] = temp[0 as i32 as usize]
            * (*or).axis[1 as i32 as usize][0 as i32 as usize]
            + temp[1 as i32 as usize] * (*or).axis[1 as i32 as usize][1 as i32 as usize]
            + temp[2 as i32 as usize] * (*or).axis[1 as i32 as usize][2 as i32 as usize];
        (*dl).transformed[2 as i32 as usize] = temp[0 as i32 as usize]
            * (*or).axis[2 as i32 as usize][0 as i32 as usize]
            + temp[1 as i32 as usize] * (*or).axis[2 as i32 as usize][1 as i32 as usize]
            + temp[2 as i32 as usize] * (*or).axis[2 as i32 as usize][2 as i32 as usize];
        i += 1;
        dl = dl.offset(1)
    }
}
/*
=============
R_DlightBmodel

Determine which dynamic lights may effect this bmodel
=============
*/
#[no_mangle]

pub unsafe extern "C" fn R_DlightBmodel(mut bmodel: *mut bmodel_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut mask: i32 = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    // transform all the lights
    R_TransformDlights(
        tr.refdef.num_dlights,
        tr.refdef.dlights,
        &mut tr.or,
    );
    mask = 0 as i32;
    i = 0 as i32;
    while i < tr.refdef.num_dlights {
        dl = &mut *tr
            .refdef
            .dlights
            .offset(i as isize) as *mut dlight_s;
        // see if the point is close enough to the bounds to matter
        j = 0 as i32;
        while j < 3 as i32 {
            if (*dl).transformed[j as usize] - (*bmodel).bounds[1 as i32 as usize][j as usize]
                > (*dl).radius
            {
                break;
            }
            if (*bmodel).bounds[0 as i32 as usize][j as usize] - (*dl).transformed[j as usize]
                > (*dl).radius
            {
                break;
            }
            j += 1
        }
        if !(j < 3 as i32) {
            // we need to check this light
            mask |= (1 as i32) << i
        }
        i += 1
    }
    (*tr.currentEntity).needDlights =
        (mask != 0 as i32) as i32 as qboolean;
    // set the dlight bits in all the surfaces
    i = 0 as i32;
    while i < (*bmodel).numSurfaces {
        surf = (*bmodel).firstSurface.offset(i as isize);
        if *(*surf).data as u32 == SF_FACE as i32 as u32 {
            (*((*surf).data as *mut srfSurfaceFace_t)).dlightBits = mask
        } else if *(*surf).data as u32 == SF_GRID as i32 as u32 {
            (*((*surf).data as *mut srfGridMesh_t)).dlightBits = mask
        } else if *(*surf).data as u32 == SF_TRIANGLES as i32 as u32 {
            (*((*surf).data as *mut srfTriangles_t)).dlightBits = mask
        }
        i += 1
    }
}
/*
=================
R_SetupEntityLightingGrid

=================
*/

unsafe extern "C" fn R_SetupEntityLightingGrid(mut ent: *mut trRefEntity_t) {
    let mut lightOrigin: vec3_t = [0.; 3];
    let mut pos: [i32; 3] = [0; 3];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut gridData: *mut byte =
        0 as *mut byte;
    let mut frac: [f32; 3] = [0.; 3];
    let mut gridStep: [i32; 3] = [0; 3];
    let mut direction: vec3_t = [0.; 3];
    let mut totalFactor: f32 = 0.;
    if (*ent).e.renderfx & 0x80 as i32 != 0 {
        // separate lightOrigins are needed so an object that is
        // sinking into the ground can still be lit, and so
        // multi-part models can be lit identically
        lightOrigin[0 as i32 as usize] = (*ent).e.lightingOrigin[0 as i32 as usize];
        lightOrigin[1 as i32 as usize] = (*ent).e.lightingOrigin[1 as i32 as usize];
        lightOrigin[2 as i32 as usize] = (*ent).e.lightingOrigin[2 as i32 as usize]
    } else {
        lightOrigin[0 as i32 as usize] = (*ent).e.origin[0 as i32 as usize];
        lightOrigin[1 as i32 as usize] = (*ent).e.origin[1 as i32 as usize];
        lightOrigin[2 as i32 as usize] = (*ent).e.origin[2 as i32 as usize]
    }
    lightOrigin[0 as i32 as usize] = lightOrigin[0 as i32 as usize]
        - (*tr.world).lightGridOrigin[0 as i32 as usize];
    lightOrigin[1 as i32 as usize] = lightOrigin[1 as i32 as usize]
        - (*tr.world).lightGridOrigin[1 as i32 as usize];
    lightOrigin[2 as i32 as usize] = lightOrigin[2 as i32 as usize]
        - (*tr.world).lightGridOrigin[2 as i32 as usize];
    i = 0 as i32;
    while i < 3 as i32 {
        let mut v: f32 = 0.;
        v = lightOrigin[i as usize]
            * (*tr.world).lightGridInverseSize[i as usize];
        pos[i as usize] = crate::stdlib::floor(v as f64) as i32;
        frac[i as usize] = v - pos[i as usize] as f32;
        if pos[i as usize] < 0 as i32 {
            pos[i as usize] = 0 as i32
        } else if pos[i as usize]
            > (*tr.world).lightGridBounds[i as usize] - 1 as i32
        {
            pos[i as usize] =
                (*tr.world).lightGridBounds[i as usize] - 1 as i32
        }
        i += 1
    }
    (*ent).ambientLight[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).ambientLight[1 as i32 as usize] = (*ent).ambientLight[2 as i32 as usize];
    (*ent).ambientLight[0 as i32 as usize] = (*ent).ambientLight[1 as i32 as usize];
    (*ent).directedLight[2 as i32 as usize] = 0 as i32 as vec_t;
    (*ent).directedLight[1 as i32 as usize] = (*ent).directedLight[2 as i32 as usize];
    (*ent).directedLight[0 as i32 as usize] = (*ent).directedLight[1 as i32 as usize];
    direction[2 as i32 as usize] = 0 as i32 as vec_t;
    direction[1 as i32 as usize] = direction[2 as i32 as usize];
    direction[0 as i32 as usize] = direction[1 as i32 as usize];
    // NULL with -nolight maps
    // trilerp the light value
    gridStep[0 as i32 as usize] = 8 as i32;
    gridStep[1 as i32 as usize] =
        8 as i32 * (*tr.world).lightGridBounds[0 as i32 as usize];
    gridStep[2 as i32 as usize] = 8 as i32
        * (*tr.world).lightGridBounds[0 as i32 as usize]
        * (*tr.world).lightGridBounds[1 as i32 as usize];
    gridData = (*tr.world)
        .lightGridData
        .offset((pos[0 as i32 as usize] * gridStep[0 as i32 as usize]) as isize)
        .offset((pos[1 as i32 as usize] * gridStep[1 as i32 as usize]) as isize)
        .offset((pos[2 as i32 as usize] * gridStep[2 as i32 as usize]) as isize);
    totalFactor = 0 as i32 as f32;
    i = 0 as i32;
    while i < 8 as i32 {
        let mut factor: f32 = 0.;
        let mut data: *mut byte =
            0 as *mut byte;
        let mut lat: i32 = 0;
        let mut lng: i32 = 0;
        let mut normal: vec3_t = [0.; 3];
        factor = 1.0f64 as f32;
        data = gridData;
        j = 0 as i32;
        while j < 3 as i32 {
            if i & (1 as i32) << j != 0 {
                if pos[j as usize] + 1 as i32
                    > (*tr.world).lightGridBounds[j as usize]
                        - 1 as i32
                {
                    break;
                }
                factor *= frac[j as usize];
                data = data.offset(gridStep[j as usize] as isize)
            } else {
                factor *= 1.0f32 - frac[j as usize]
            }
            j += 1
        }
        if !(j != 3 as i32) {
            if !(*data.offset(0 as i32 as isize) as i32
                + *data.offset(1 as i32 as isize) as i32
                + *data.offset(2 as i32 as isize) as i32
                == 0)
            {
                totalFactor += factor;
                (*ent).ambientLight[0 as i32 as usize] +=
                    factor * *data.offset(0 as i32 as isize) as i32 as f32;
                (*ent).ambientLight[1 as i32 as usize] +=
                    factor * *data.offset(1 as i32 as isize) as i32 as f32;
                (*ent).ambientLight[2 as i32 as usize] +=
                    factor * *data.offset(2 as i32 as isize) as i32 as f32;
                (*ent).directedLight[0 as i32 as usize] +=
                    factor * *data.offset(3 as i32 as isize) as i32 as f32;
                (*ent).directedLight[1 as i32 as usize] +=
                    factor * *data.offset(4 as i32 as isize) as i32 as f32;
                (*ent).directedLight[2 as i32 as usize] +=
                    factor * *data.offset(5 as i32 as isize) as i32 as f32;
                lat = *data.offset(7 as i32 as isize) as i32;
                lng = *data.offset(6 as i32 as isize) as i32;
                lat *= 1024 as i32 / 256 as i32;
                lng *= 1024 as i32 / 256 as i32;
                // decode X as cos( lat ) * sin( long )
                // decode Y as sin( lat ) * sin( long )
                // decode Z as cos( long )
                normal[0 as i32 as usize] = tr.sinTable
                    [(lat + 1024 as i32 / 4 as i32 & 1024 as i32 - 1 as i32) as usize]
                    * tr.sinTable[lng as usize];
                normal[1 as i32 as usize] = tr.sinTable
                    [lat as usize]
                    * tr.sinTable[lng as usize];
                normal[2 as i32 as usize] = tr.sinTable
                    [(lng + 1024 as i32 / 4 as i32 & 1024 as i32 - 1 as i32) as usize];
                direction[0 as i32 as usize] =
                    direction[0 as i32 as usize] + normal[0 as i32 as usize] * factor;
                direction[1 as i32 as usize] =
                    direction[1 as i32 as usize] + normal[1 as i32 as usize] * factor;
                direction[2 as i32 as usize] =
                    direction[2 as i32 as usize] + normal[2 as i32 as usize] * factor
            }
        }
        i += 1
        // ignore samples in walls
    }
    if totalFactor > 0 as i32 as f32 && (totalFactor as f64) < 0.99f64 {
        totalFactor = 1.0f32 / totalFactor;
        (*ent).ambientLight[0 as i32 as usize] =
            (*ent).ambientLight[0 as i32 as usize] * totalFactor;
        (*ent).ambientLight[1 as i32 as usize] =
            (*ent).ambientLight[1 as i32 as usize] * totalFactor;
        (*ent).ambientLight[2 as i32 as usize] =
            (*ent).ambientLight[2 as i32 as usize] * totalFactor;
        (*ent).directedLight[0 as i32 as usize] =
            (*ent).directedLight[0 as i32 as usize] * totalFactor;
        (*ent).directedLight[1 as i32 as usize] =
            (*ent).directedLight[1 as i32 as usize] * totalFactor;
        (*ent).directedLight[2 as i32 as usize] =
            (*ent).directedLight[2 as i32 as usize] * totalFactor
    }
    (*ent).ambientLight[0 as i32 as usize] =
        (*ent).ambientLight[0 as i32 as usize] * (*r_ambientScale).value;
    (*ent).ambientLight[1 as i32 as usize] =
        (*ent).ambientLight[1 as i32 as usize] * (*r_ambientScale).value;
    (*ent).ambientLight[2 as i32 as usize] =
        (*ent).ambientLight[2 as i32 as usize] * (*r_ambientScale).value;
    (*ent).directedLight[0 as i32 as usize] =
        (*ent).directedLight[0 as i32 as usize] * (*r_directedScale).value;
    (*ent).directedLight[1 as i32 as usize] =
        (*ent).directedLight[1 as i32 as usize] * (*r_directedScale).value;
    (*ent).directedLight[2 as i32 as usize] =
        (*ent).directedLight[2 as i32 as usize] * (*r_directedScale).value;
    VectorNormalize2(
        direction.as_mut_ptr() as *const vec_t,
        (*ent).lightDir.as_mut_ptr(),
    );
}
/*
===============
LogLight
===============
*/

unsafe extern "C" fn LogLight(mut ent: *mut trRefEntity_t) {
    let mut max1: i32 = 0;
    let mut max2: i32 = 0;
    if (*ent).e.renderfx & 0x4 as i32 == 0 {
        return;
    }
    max1 = (*ent).ambientLight[0 as i32 as usize] as i32;
    if (*ent).ambientLight[1 as i32 as usize] > max1 as f32 {
        max1 = (*ent).ambientLight[1 as i32 as usize] as i32
    } else if (*ent).ambientLight[2 as i32 as usize] > max1 as f32 {
        max1 = (*ent).ambientLight[2 as i32 as usize] as i32
    }
    max2 = (*ent).directedLight[0 as i32 as usize] as i32;
    if (*ent).directedLight[1 as i32 as usize] > max2 as f32 {
        max2 = (*ent).directedLight[1 as i32 as usize] as i32
    } else if (*ent).directedLight[2 as i32 as usize] > max2 as f32 {
        max2 = (*ent).directedLight[2 as i32 as usize] as i32
    }
    ri
        .Printf
        .expect("non-null function pointer")(
        PRINT_ALL as i32,
        b"amb:%i  dir:%i\n\x00" as *const u8 as *const libc::c_char,
        max1,
        max2,
    );
}
/*
=================
R_SetupEntityLighting

Calculates all the lighting values that will be used
by the Calc_* functions
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_SetupEntityLighting(
    mut refdef: *const trRefdef_t,
    mut ent: *mut trRefEntity_t,
) {
    let mut i: i32 = 0;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut power: f32 = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    let mut lightDir: vec3_t = [0.; 3];
    let mut lightOrigin: vec3_t = [0.; 3];
    // lighting calculations
    if (*ent).lightingCalculated as u64 != 0 {
        return;
    }
    (*ent).lightingCalculated = qtrue;
    //
    // trace a sample point down to find ambient light
    //
    if (*ent).e.renderfx & 0x80 as i32 != 0 {
        // separate lightOrigins are needed so an object that is
        // sinking into the ground can still be lit, and so
        // multi-part models can be lit identically
        lightOrigin[0 as i32 as usize] = (*ent).e.lightingOrigin[0 as i32 as usize];
        lightOrigin[1 as i32 as usize] = (*ent).e.lightingOrigin[1 as i32 as usize];
        lightOrigin[2 as i32 as usize] = (*ent).e.lightingOrigin[2 as i32 as usize]
    } else {
        lightOrigin[0 as i32 as usize] = (*ent).e.origin[0 as i32 as usize];
        lightOrigin[1 as i32 as usize] = (*ent).e.origin[1 as i32 as usize];
        lightOrigin[2 as i32 as usize] = (*ent).e.origin[2 as i32 as usize]
    }
    // if NOWORLDMODEL, only use dynamic lights (menu system, etc)
    if (*refdef).rdflags & 0x1 as i32 == 0
        && !(*tr.world)
            .lightGridData
            .is_null()
    {
        R_SetupEntityLightingGrid(ent);
    } else {
        (*ent).ambientLight[2 as i32 as usize] =
            tr.identityLight * 150 as i32 as f32;
        (*ent).ambientLight[1 as i32 as usize] = (*ent).ambientLight[2 as i32 as usize];
        (*ent).ambientLight[0 as i32 as usize] = (*ent).ambientLight[1 as i32 as usize];
        (*ent).directedLight[2 as i32 as usize] =
            tr.identityLight * 150 as i32 as f32;
        (*ent).directedLight[1 as i32 as usize] = (*ent).directedLight[2 as i32 as usize];
        (*ent).directedLight[0 as i32 as usize] = (*ent).directedLight[1 as i32 as usize];
        (*ent).lightDir[0 as i32 as usize] =
            tr.sunDirection[0 as i32 as usize];
        (*ent).lightDir[1 as i32 as usize] =
            tr.sunDirection[1 as i32 as usize];
        (*ent).lightDir[2 as i32 as usize] =
            tr.sunDirection[2 as i32 as usize]
    }
    // bonus items and view weapons have a fixed minimum add
    /* ent->e.renderfx & RF_MINLIGHT */
    // give everything a minimum light add
    (*ent).ambientLight[0 as i32 as usize] +=
        tr.identityLight * 32 as i32 as f32;
    (*ent).ambientLight[1 as i32 as usize] +=
        tr.identityLight * 32 as i32 as f32;
    (*ent).ambientLight[2 as i32 as usize] +=
        tr.identityLight * 32 as i32 as f32;
    //
    // modify the light by dynamic lights
    //
    d = VectorLength(
        (*ent).directedLight.as_mut_ptr() as *const vec_t
    );
    lightDir[0 as i32 as usize] = (*ent).lightDir[0 as i32 as usize] * d;
    lightDir[1 as i32 as usize] = (*ent).lightDir[1 as i32 as usize] * d;
    lightDir[2 as i32 as usize] = (*ent).lightDir[2 as i32 as usize] * d;
    i = 0 as i32;
    while i < (*refdef).num_dlights {
        dl = &mut *(*refdef).dlights.offset(i as isize) as *mut dlight_s;
        dir[0 as i32 as usize] = (*dl).origin[0 as i32 as usize] - lightOrigin[0 as i32 as usize];
        dir[1 as i32 as usize] = (*dl).origin[1 as i32 as usize] - lightOrigin[1 as i32 as usize];
        dir[2 as i32 as usize] = (*dl).origin[2 as i32 as usize] - lightOrigin[2 as i32 as usize];
        d = VectorNormalize(dir.as_mut_ptr());
        power = 16 as i32 as f32 * ((*dl).radius * (*dl).radius);
        if d < 16 as i32 as f32 {
            d = 16 as i32 as f32
        }
        d = power / (d * d);
        (*ent).directedLight[0 as i32 as usize] =
            (*ent).directedLight[0 as i32 as usize] + (*dl).color[0 as i32 as usize] * d;
        (*ent).directedLight[1 as i32 as usize] =
            (*ent).directedLight[1 as i32 as usize] + (*dl).color[1 as i32 as usize] * d;
        (*ent).directedLight[2 as i32 as usize] =
            (*ent).directedLight[2 as i32 as usize] + (*dl).color[2 as i32 as usize] * d;
        lightDir[0 as i32 as usize] = lightDir[0 as i32 as usize] + dir[0 as i32 as usize] * d;
        lightDir[1 as i32 as usize] = lightDir[1 as i32 as usize] + dir[1 as i32 as usize] * d;
        lightDir[2 as i32 as usize] = lightDir[2 as i32 as usize] + dir[2 as i32 as usize] * d;
        i += 1
    }
    // clamp ambient
    i = 0 as i32;
    while i < 3 as i32 {
        if (*ent).ambientLight[i as usize]
            > tr.identityLightByte as f32
        {
            (*ent).ambientLight[i as usize] = tr.identityLightByte
                as vec_t
        }
        i += 1
    }
    if (*r_debugLight).integer != 0 {
        LogLight(ent);
    }
    // save out the byte packet version
    *(&mut (*ent).ambientLightInt as *mut i32 as *mut byte)
        .offset(0 as i32 as isize) = ri
        .ftol
        .expect("non-null function pointer")(
        (*ent).ambientLight[0 as i32 as usize]
    ) as byte;
    *(&mut (*ent).ambientLightInt as *mut i32 as *mut byte)
        .offset(1 as i32 as isize) = ri
        .ftol
        .expect("non-null function pointer")(
        (*ent).ambientLight[1 as i32 as usize]
    ) as byte;
    *(&mut (*ent).ambientLightInt as *mut i32 as *mut byte)
        .offset(2 as i32 as isize) = ri
        .ftol
        .expect("non-null function pointer")(
        (*ent).ambientLight[2 as i32 as usize]
    ) as byte;
    *(&mut (*ent).ambientLightInt as *mut i32 as *mut byte)
        .offset(3 as i32 as isize) = 0xff as i32 as byte;
    // transform the direction to local space
    VectorNormalize(lightDir.as_mut_ptr());
    (*ent).lightDir[0 as i32 as usize] = lightDir[0 as i32 as usize]
        * (*ent).e.axis[0 as i32 as usize][0 as i32 as usize]
        + lightDir[1 as i32 as usize] * (*ent).e.axis[0 as i32 as usize][1 as i32 as usize]
        + lightDir[2 as i32 as usize] * (*ent).e.axis[0 as i32 as usize][2 as i32 as usize];
    (*ent).lightDir[1 as i32 as usize] = lightDir[0 as i32 as usize]
        * (*ent).e.axis[1 as i32 as usize][0 as i32 as usize]
        + lightDir[1 as i32 as usize] * (*ent).e.axis[1 as i32 as usize][1 as i32 as usize]
        + lightDir[2 as i32 as usize] * (*ent).e.axis[1 as i32 as usize][2 as i32 as usize];
    (*ent).lightDir[2 as i32 as usize] = lightDir[0 as i32 as usize]
        * (*ent).e.axis[2 as i32 as usize][0 as i32 as usize]
        + lightDir[1 as i32 as usize] * (*ent).e.axis[2 as i32 as usize][1 as i32 as usize]
        + lightDir[2 as i32 as usize] * (*ent).e.axis[2 as i32 as usize][2 as i32 as usize];
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
=================
R_LightForPoint
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_LightForPoint(
    mut point: *mut vec_t,
    mut ambientLight: *mut vec_t,
    mut directedLight: *mut vec_t,
    mut lightDir: *mut vec_t,
) -> i32 {
    let mut ent: trRefEntity_t = trRefEntity_t {
        e: refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
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
        needDlights: qfalse,
        lightingCalculated: qfalse,
        lightDir: [0.; 3],
        ambientLight: [0.; 3],
        ambientLightInt: 0,
        directedLight: [0.; 3],
    };
    if (*tr.world)
        .lightGridData
        .is_null()
    {
        return qfalse as i32;
    }
    crate::stdlib::memset(
        &mut ent as *mut trRefEntity_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<trRefEntity_t>() as libc::c_ulong,
    );
    ent.e.origin[0 as i32 as usize] = *point.offset(0 as i32 as isize);
    ent.e.origin[1 as i32 as usize] = *point.offset(1 as i32 as isize);
    ent.e.origin[2 as i32 as usize] = *point.offset(2 as i32 as isize);
    R_SetupEntityLightingGrid(&mut ent);
    *ambientLight.offset(0 as i32 as isize) = ent.ambientLight[0 as i32 as usize];
    *ambientLight.offset(1 as i32 as isize) = ent.ambientLight[1 as i32 as usize];
    *ambientLight.offset(2 as i32 as isize) = ent.ambientLight[2 as i32 as usize];
    *directedLight.offset(0 as i32 as isize) = ent.directedLight[0 as i32 as usize];
    *directedLight.offset(1 as i32 as isize) = ent.directedLight[1 as i32 as usize];
    *directedLight.offset(2 as i32 as isize) = ent.directedLight[2 as i32 as usize];
    *lightDir.offset(0 as i32 as isize) = ent.lightDir[0 as i32 as usize];
    *lightDir.offset(1 as i32 as isize) = ent.lightDir[1 as i32 as usize];
    *lightDir.offset(2 as i32 as isize) = ent.lightDir[2 as i32 as usize];
    return qtrue as i32;
}

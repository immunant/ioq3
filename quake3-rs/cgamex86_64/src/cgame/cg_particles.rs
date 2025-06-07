use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn Distance(
        mut p1: *const crate::src::qcommon::q_shared::vec_t,
        mut p2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        v[0 as libc::c_int as usize] =
            *p2.offset(0 as libc::c_int as isize) - *p1.offset(0 as libc::c_int as isize);
        v[1 as libc::c_int as usize] =
            *p2.offset(1 as libc::c_int as isize) - *p1.offset(1 as libc::c_int as isize);
        v[2 as libc::c_int as usize] =
            *p2.offset(2 as libc::c_int as isize) - *p1.offset(2 as libc::c_int as isize);
        return VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
        return ::libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_particles::q_shared_h::Distance;
pub use crate::src::cgame::cg_particles::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
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
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_particles::stdlib_float_h::atof;
pub use crate::src::cgame::cg_particles::stdlib_h::atoi;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShader;

pub use ::libc::rand;
pub use ::libc::strtod;
pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct particle_s {
    pub next: *mut particle_s,
    pub time: libc::c_float,
    pub endtime: libc::c_float,
    pub org: crate::src::qcommon::q_shared::vec3_t,
    pub vel: crate::src::qcommon::q_shared::vec3_t,
    pub accel: crate::src::qcommon::q_shared::vec3_t,
    pub color: libc::c_int,
    pub colorvel: libc::c_float,
    pub alpha: libc::c_float,
    pub alphavel: libc::c_float,
    pub type_0: libc::c_int,
    pub pshader: crate::src::qcommon::q_shared::qhandle_t,
    pub height: libc::c_float,
    pub width: libc::c_float,
    pub endheight: libc::c_float,
    pub endwidth: libc::c_float,
    pub start: libc::c_float,
    pub end: libc::c_float,
    pub startfade: libc::c_float,
    pub rotate: crate::src::qcommon::q_shared::qboolean,
    pub snum: libc::c_int,
    pub link: crate::src::qcommon::q_shared::qboolean,
    pub shaderAnim: libc::c_int,
    pub roll: libc::c_int,
    pub accumroll: libc::c_int,
}

pub type cparticle_t = particle_s;

pub const P_WEATHER_FLURRY: C2RustUnnamed_25 = 11;

pub const P_WEATHER_TURBULENT: C2RustUnnamed_25 = 5;

pub const P_WEATHER: C2RustUnnamed_25 = 1;

pub const P_ANIM: C2RustUnnamed_25 = 6;

pub const P_FLAT: C2RustUnnamed_25 = 2;

pub const P_FLAT_SCALEUP: C2RustUnnamed_25 = 9;

pub const P_BLEED: C2RustUnnamed_25 = 8;

pub const P_SMOKE_IMPACT: C2RustUnnamed_25 = 12;

pub const P_SMOKE: C2RustUnnamed_25 = 3;

pub const P_SPRITE: C2RustUnnamed_25 = 15;

pub const P_BUBBLE_TURBULENT: C2RustUnnamed_25 = 14;

pub const P_BUBBLE: C2RustUnnamed_25 = 13;

pub const P_BAT: C2RustUnnamed_25 = 7;

pub const P_FLAT_SCALEUP_FADE: C2RustUnnamed_25 = 10;

pub type C2RustUnnamed_25 = libc::c_uint;

pub const P_ROTATE: C2RustUnnamed_25 = 4;

pub const P_NONE: C2RustUnnamed_25 = 0;

static mut shaderAnimNames: [*mut libc::c_char; 32] = [
    b"explode1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

static mut shaderAnims: [[crate::src::qcommon::q_shared::qhandle_t; 64]; 32] = [[0; 64]; 32];

static mut shaderAnimCounts: [libc::c_int; 32] = [
    23 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

static mut shaderAnimSTRatio: [libc::c_float; 32] = [
    1.0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    0., 0., 0., 0., 0., 0., 0., 0., 0.,
];

static mut numShaderAnims: libc::c_int = 0;
#[no_mangle]

pub static mut active_particles: *mut cparticle_t = 0 as *const cparticle_t as *mut cparticle_t;
#[no_mangle]

pub static mut free_particles: *mut cparticle_t = 0 as *const cparticle_t as *mut cparticle_t;
#[no_mangle]

pub static mut particles: [cparticle_t; 1024] = [cparticle_t {
    next: 0 as *const particle_s as *mut particle_s,
    time: 0.,
    endtime: 0.,
    org: [0.; 3],
    vel: [0.; 3],
    accel: [0.; 3],
    color: 0,
    colorvel: 0.,
    alpha: 0.,
    alphavel: 0.,
    type_0: 0,
    pshader: 0,
    height: 0.,
    width: 0.,
    endheight: 0.,
    endwidth: 0.,
    start: 0.,
    end: 0.,
    startfade: 0.,
    rotate: crate::src::qcommon::q_shared::qfalse,
    snum: 0,
    link: crate::src::qcommon::q_shared::qfalse,
    shaderAnim: 0,
    roll: 0,
    accumroll: 0,
}; 1024];
#[no_mangle]

pub static mut cl_numparticles: libc::c_int = 1024 as libc::c_int;
#[no_mangle]

pub static mut initparticles: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut vforward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut vright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rforward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut rup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
#[no_mangle]

pub static mut oldtime: libc::c_float = 0.;
// Ridah
/*
===============
CL_ClearParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ClearParticles() {
    let mut i: libc::c_int = 0;
    crate::stdlib::memset(
        particles.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[cparticle_t; 1024]>() as libc::c_ulong,
    );
    free_particles =
        &mut *particles.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut cparticle_t;
    active_particles = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < cl_numparticles {
        particles[i as usize].next = &mut *particles
            .as_mut_ptr()
            .offset((i + 1 as libc::c_int) as isize)
            as *mut cparticle_t;
        particles[i as usize].type_0 = 0 as libc::c_int;
        i += 1
    }
    particles[(cl_numparticles - 1 as libc::c_int) as usize].next = 0 as *mut particle_s;
    oldtime = crate::src::cgame::cg_main::cg.time as libc::c_float;
    // Ridah, init the shaderAnims
    i = 0 as libc::c_int;
    while !shaderAnimNames[i as usize].is_null() {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < shaderAnimCounts[i as usize] {
            shaderAnims[i as usize][j as usize] =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    crate::src::qcommon::q_shared::va(
                        b"%s%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        shaderAnimNames[i as usize],
                        j + 1 as libc::c_int,
                    ),
                );
            j += 1
        }
        i += 1
    }
    numShaderAnims = i;
    // done.
    initparticles = crate::src::qcommon::q_shared::qtrue;
}
/*
=====================
CG_AddParticleToScene
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleToScene(
    mut p: *mut cparticle_t,
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut _alpha: libc::c_float,
) {
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut verts: [crate::tr_types_h::polyVert_t; 4] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut width: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut time2: libc::c_float = 0.;
    let mut ratio: libc::c_float = 0.;
    let mut invratio: libc::c_float = 0.;
    let mut color: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut TRIverts: [crate::tr_types_h::polyVert_t; 3] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 3];
    let mut rright2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rup2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*p).type_0 == P_WEATHER as libc::c_int
        || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        || (*p).type_0 == P_WEATHER_FLURRY as libc::c_int
        || (*p).type_0 == P_BUBBLE as libc::c_int
        || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
    {
        // create a front facing polygon
        if (*p).type_0 != P_WEATHER_FLURRY as libc::c_int {
            if (*p).type_0 == P_BUBBLE as libc::c_int
                || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
            {
                if *org.offset(2 as libc::c_int as isize) > (*p).end {
                    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float; // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize); // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
                    (*p).org[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
                    (*p).org[2 as libc::c_int as usize] = ((*p).start as libc::c_double
                        + 2.0f64
                            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                                / 0x7fff as libc::c_int as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4 as libc::c_int as libc::c_double)
                        as crate::src::qcommon::q_shared::vec_t;
                    if (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int {
                        (*p).vel[0 as libc::c_int as usize] = (2.0f64
                            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                                / 0x7fff as libc::c_int as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4 as libc::c_int as libc::c_double)
                            as crate::src::qcommon::q_shared::vec_t;
                        (*p).vel[1 as libc::c_int as usize] = (2.0f64
                            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                                / 0x7fff as libc::c_int as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4 as libc::c_int as libc::c_double)
                            as crate::src::qcommon::q_shared::vec_t
                    }
                }
            } else if *org.offset(2 as libc::c_int as isize) < (*p).end {
                (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
                (*p).org[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize);
                (*p).org[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
                (*p).org[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
                while (*p).org[2 as libc::c_int as usize] < (*p).end {
                    (*p).org[2 as libc::c_int as usize] += (*p).start - (*p).end
                }
                if (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int {
                    (*p).vel[0 as libc::c_int as usize] = (2.0f64
                        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 16 as libc::c_int as libc::c_double)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*p).vel[1 as libc::c_int as usize] = (2.0f64
                        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 16 as libc::c_int as libc::c_double)
                        as crate::src::qcommon::q_shared::vec_t
                }
            }
            // Rafael snow pvs check
            if (*p).link as u64 == 0 {
                return;
            }
            (*p).alpha = 1 as libc::c_int as libc::c_float
        }
        // Ridah, had to do this or MAX_POLYS is being exceeded in village1.bsp
        if Distance(
            (*crate::src::cgame::cg_main::cg.snap)
                .ps
                .origin
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            org as *const crate::src::qcommon::q_shared::vec_t,
        ) > 1024 as libc::c_int as libc::c_float
        {
            return;
        }
        // done.
        if (*p).type_0 == P_BUBBLE as libc::c_int
            || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
        {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * -(*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * -(*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * -(*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width;
            verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte;
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * -(*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * -(*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * -(*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * (*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * (*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * (*p).width;
            verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte;
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * (*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * (*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * (*p).width;
            verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte;
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width;
            verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte
        } else {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * -(*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * -(*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * -(*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width;
            TRIverts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            TRIverts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            TRIverts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            TRIverts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            TRIverts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            TRIverts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte;
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width;
            TRIverts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            TRIverts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            TRIverts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            TRIverts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            TRIverts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            TRIverts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte;
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * (*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * (*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * (*p).width;
            TRIverts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize];
            TRIverts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize];
            TRIverts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize];
            TRIverts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
                0 as libc::c_int as libc::c_float;
            TRIverts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
                1 as libc::c_int as libc::c_float;
            TRIverts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
                255 as libc::c_int as crate::src::qcommon::q_shared::byte;
            TRIverts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
                (255 as libc::c_int as libc::c_float * (*p).alpha)
                    as crate::src::qcommon::q_shared::byte
        }
    } else if (*p).type_0 == P_SPRITE as libc::c_int {
        let mut rr: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        color[0 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
        color[1 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
        color[2 as libc::c_int as usize] = 0.5f64 as crate::src::qcommon::q_shared::vec_t;
        time = crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang.as_mut_ptr(),
            );
            rotate_ang[2 as libc::c_int as usize] += (*p).roll as libc::c_float;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr.as_mut_ptr(),
                ru.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + ru[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + ru[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + ru[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rr[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rr[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rr[2 as libc::c_int as usize] * -width
        } else {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + vup[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + vup[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + vup[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -width
        }
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + ru[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + ru[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + ru[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vup[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vup[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vup[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height)
        }
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + rr[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + rr[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + rr[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vright[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vright[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vright[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width)
        }
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + ru[0 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + ru[1 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + ru[2 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vup[0 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vup[1 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vup[2 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height)
        }
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_SMOKE as libc::c_int || (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
    {
        // create a front rotating facing polygon
        if (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
            && Distance(
                (*crate::src::cgame::cg_main::cg.snap)
                    .ps
                    .origin
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                org as *const crate::src::qcommon::q_shared::vec_t,
            ) > 1024 as libc::c_int as libc::c_float
        {
            return;
        }
        if (*p).color == 2 as libc::c_int {
            color[0 as libc::c_int as usize] = 0.22f32;
            color[1 as libc::c_int as usize] = 0.0f32;
            color[2 as libc::c_int as usize] = 0.0f32
        } else if (*p).color == 4 as libc::c_int {
            let mut len: libc::c_float = 0.;
            let mut greyit: libc::c_float = 0.;
            let mut val: libc::c_float = 0.;
            len = Distance(
                (*crate::src::cgame::cg_main::cg.snap)
                    .ps
                    .origin
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                org as *const crate::src::qcommon::q_shared::vec_t,
            );
            if len == 0. {
                len = 1 as libc::c_int as libc::c_float
            }
            val = 4096 as libc::c_int as libc::c_float / len;
            greyit = (0.25f64 * val as libc::c_double) as libc::c_float;
            if greyit as libc::c_double > 0.5f64 {
                greyit = 0.5f64 as libc::c_float
            }
            color[0 as libc::c_int as usize] = greyit;
            color[1 as libc::c_int as usize] = greyit;
            color[2 as libc::c_int as usize] = greyit
        } else {
            color[0 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
            color[1 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t;
            color[2 as libc::c_int as usize] = 1.0f64 as crate::src::qcommon::q_shared::vec_t
        }
        time = crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if crate::src::cgame::cg_main::cg.time as libc::c_float > (*p).startfade {
            invratio = 1 as libc::c_int as libc::c_float
                - (crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).startfade)
                    / ((*p).endtime - (*p).startfade);
            if (*p).color == 3 as libc::c_int {
                let mut fval: libc::c_float = 0.;
                fval = invratio * invratio;
                if fval < 0 as libc::c_int as libc::c_float {
                    fval = 0 as libc::c_int as libc::c_float
                }
                color[0 as libc::c_int as usize] = fval;
                color[1 as libc::c_int as usize] = fval;
                color[2 as libc::c_int as usize] = fval
            }
            invratio *= (*p).alpha
        } else {
            invratio = 1 as libc::c_int as libc::c_float * (*p).alpha
        }
        if crate::src::cgame::cg_main::cgs.glconfig.hardwareType as libc::c_uint
            == crate::tr_types_h::GLHW_RAGEPRO as libc::c_int as libc::c_uint
        {
            invratio = 1 as libc::c_int as libc::c_float
        }
        if invratio > 1 as libc::c_int as libc::c_float {
            invratio = 1 as libc::c_int as libc::c_float
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).type_0 != P_SMOKE_IMPACT as libc::c_int {
            let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
            crate::src::qcommon::q_math::vectoangles(
                rforward.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                temp.as_mut_ptr(),
            );
            (*p).accumroll += (*p).roll;
            temp[2 as libc::c_int as usize] = (temp[2 as libc::c_int as usize] as libc::c_double
                + (*p).accumroll as libc::c_double * 0.1f64)
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::qcommon::q_math::AngleVectors(
                temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rright2.as_mut_ptr(),
                rup2.as_mut_ptr(),
            );
        } else {
            rright2[0 as libc::c_int as usize] = rright[0 as libc::c_int as usize];
            rright2[1 as libc::c_int as usize] = rright[1 as libc::c_int as usize];
            rright2[2 as libc::c_int as usize] = rright[2 as libc::c_int as usize];
            rup2[0 as libc::c_int as usize] = rup[0 as libc::c_int as usize];
            rup2[1 as libc::c_int as usize] = rup[1 as libc::c_int as usize];
            rup2[2 as libc::c_int as usize] = rup[2 as libc::c_int as usize]
        }
        if (*p).rotate as u64 != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + rup2[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + rup2[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + rup2[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rright2[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rright2[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rright2[2 as libc::c_int as usize] * -width
        } else {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * -(*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * -(*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * -(*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width
        }
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + rup2[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + rup2[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + rup2[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rright2[0 as libc::c_int as usize] * width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rright2[1 as libc::c_int as usize] * width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rright2[2 as libc::c_int as usize] * width
        } else {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * -(*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * -(*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * -(*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * (*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * (*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * (*p).width
        }
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + rup2[0 as libc::c_int as usize] * height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + rup2[1 as libc::c_int as usize] * height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + rup2[2 as libc::c_int as usize] * height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rright2[0 as libc::c_int as usize] * width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rright2[1 as libc::c_int as usize] * width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rright2[2 as libc::c_int as usize] * width
        } else {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * (*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * (*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * (*p).width
        }
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * invratio) as crate::src::qcommon::q_shared::byte;
        if (*p).rotate as u64 != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + rup2[0 as libc::c_int as usize] * height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + rup2[1 as libc::c_int as usize] * height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + rup2[2 as libc::c_int as usize] * height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rright2[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rright2[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rright2[2 as libc::c_int as usize] * -width
        } else {
            point[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize)
                + vup[0 as libc::c_int as usize] * (*p).height;
            point[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize)
                + vup[1 as libc::c_int as usize] * (*p).height;
            point[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize)
                + vup[2 as libc::c_int as usize] * (*p).height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -(*p).width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -(*p).width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -(*p).width
        }
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * invratio) as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_BLEED as libc::c_int {
        let mut rr_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut alpha_0: libc::c_float = 0.;
        alpha_0 = (*p).alpha;
        if crate::src::cgame::cg_main::cgs.glconfig.hardwareType as libc::c_uint
            == crate::tr_types_h::GLHW_RAGEPRO as libc::c_int as libc::c_uint
        {
            alpha_0 = 1 as libc::c_int as libc::c_float
        }
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang_0.as_mut_ptr(),
            );
            rotate_ang_0[2 as libc::c_int as usize] += (*p).roll as libc::c_float;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang_0.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr_0.as_mut_ptr(),
                ru_0.as_mut_ptr(),
            );
        } else {
            ru_0[0 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
            ru_0[1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
            ru_0[2 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
            rr_0[0 as libc::c_int as usize] = vright[0 as libc::c_int as usize];
            rr_0[1 as libc::c_int as usize] = vright[1 as libc::c_int as usize];
            rr_0[2 as libc::c_int as usize] = vright[2 as libc::c_int as usize]
        }
        point[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) + ru_0[0 as libc::c_int as usize] * -(*p).height;
        point[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) + ru_0[1 as libc::c_int as usize] * -(*p).height;
        point[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) + ru_0[2 as libc::c_int as usize] * -(*p).height;
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + rr_0[0 as libc::c_int as usize] * -(*p).width;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + rr_0[1 as libc::c_int as usize] * -(*p).width;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + rr_0[2 as libc::c_int as usize] * -(*p).width;
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            111 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            19 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            9 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) + ru_0[0 as libc::c_int as usize] * -(*p).height;
        point[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) + ru_0[1 as libc::c_int as usize] * -(*p).height;
        point[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) + ru_0[2 as libc::c_int as usize] * -(*p).height;
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + rr_0[0 as libc::c_int as usize] * (*p).width;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + rr_0[1 as libc::c_int as usize] * (*p).width;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + rr_0[2 as libc::c_int as usize] * (*p).width;
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            111 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            19 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            9 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) + ru_0[0 as libc::c_int as usize] * (*p).height;
        point[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) + ru_0[1 as libc::c_int as usize] * (*p).height;
        point[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) + ru_0[2 as libc::c_int as usize] * (*p).height;
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + rr_0[0 as libc::c_int as usize] * (*p).width;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + rr_0[1 as libc::c_int as usize] * (*p).width;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + rr_0[2 as libc::c_int as usize] * (*p).width;
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            111 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            19 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            9 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * alpha_0) as crate::src::qcommon::q_shared::byte;
        point[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize) + ru_0[0 as libc::c_int as usize] * (*p).height;
        point[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize) + ru_0[1 as libc::c_int as usize] * (*p).height;
        point[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize) + ru_0[2 as libc::c_int as usize] * (*p).height;
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + rr_0[0 as libc::c_int as usize] * -(*p).width;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + rr_0[1 as libc::c_int as usize] * -(*p).width;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + rr_0[2 as libc::c_int as usize] * -(*p).width;
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            111 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            19 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            9 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * alpha_0) as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_FLAT_SCALEUP as libc::c_int {
        let mut sinR: libc::c_float = 0.;
        let mut cosR: libc::c_float = 0.;
        if (*p).color == 2 as libc::c_int {
            color[0 as libc::c_int as usize] =
                1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            color[1 as libc::c_int as usize] =
                1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            color[2 as libc::c_int as usize] =
                1 as libc::c_int as crate::src::qcommon::q_shared::vec_t
        } else {
            color[0 as libc::c_int as usize] = 0.5f64 as crate::src::qcommon::q_shared::vec_t;
            color[1 as libc::c_int as usize] = 0.5f64 as crate::src::qcommon::q_shared::vec_t;
            color[2 as libc::c_int as usize] = 0.5f64 as crate::src::qcommon::q_shared::vec_t
        }
        time = crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if width > (*p).endwidth {
            width = (*p).endwidth
        }
        if height > (*p).endheight {
            height = (*p).endheight
        }
        sinR = (height as libc::c_double
            * crate::stdlib::sin(
                (*p).roll as libc::c_double * 3.14159265358979323846f64
                    / 180.0f32 as libc::c_double,
            )
            * crate::stdlib::sqrt(2 as libc::c_int as libc::c_double))
            as libc::c_float;
        cosR = (width as libc::c_double
            * crate::stdlib::cos(
                (*p).roll as libc::c_double * 3.14159265358979323846f64
                    / 180.0f32 as libc::c_double,
            )
            * crate::stdlib::sqrt(2 as libc::c_int as libc::c_double))
            as libc::c_float;
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] -= sinR;
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] -= cosR;
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] -= cosR;
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] += sinR;
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] += sinR;
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] += cosR;
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] += cosR;
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] -= sinR;
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[0 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[1 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            (255 as libc::c_int as libc::c_float * color[2 as libc::c_int as usize])
                as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_FLAT as libc::c_int {
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] -= (*p).height;
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] -= (*p).width;
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] -= (*p).height;
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] += (*p).width;
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] += (*p).height;
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] += (*p).width;
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            *org.offset(0 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            *org.offset(1 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            *org.offset(2 as libc::c_int as isize);
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] += (*p).height;
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] -= (*p).width;
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte
    } else if (*p).type_0 == P_ANIM as libc::c_int {
        let mut rr_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut ru_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut rotate_ang_1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        time = crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if ratio >= 1.0f32 {
            ratio = 0.9999f32
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        // Ridah
        // if we are "inside" this sprite, don't draw
        if (Distance(
            (*crate::src::cgame::cg_main::cg.snap)
                .ps
                .origin
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            org as *const crate::src::qcommon::q_shared::vec_t,
        ) as libc::c_double)
            < width as libc::c_double / 1.5f64
        {
            return;
        }
        i = (*p).shaderAnim;
        j = crate::stdlib::floor(
            (ratio * shaderAnimCounts[(*p).shaderAnim as usize] as libc::c_float) as libc::c_double,
        ) as libc::c_int;
        (*p).pshader = shaderAnims[i as usize][j as usize];
        if (*p).roll != 0 {
            crate::src::qcommon::q_math::vectoangles(
                crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize]
                    .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                rotate_ang_1.as_mut_ptr(),
            );
            rotate_ang_1[2 as libc::c_int as usize] += (*p).roll as libc::c_float;
            crate::src::qcommon::q_math::AngleVectors(
                rotate_ang_1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                rr_1.as_mut_ptr(),
                ru_1.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + ru_1[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + ru_1[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + ru_1[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + rr_1[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + rr_1[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + rr_1[2 as libc::c_int as usize] * -width
        } else {
            point[0 as libc::c_int as usize] =
                *org.offset(0 as libc::c_int as isize) + vup[0 as libc::c_int as usize] * -height;
            point[1 as libc::c_int as usize] =
                *org.offset(1 as libc::c_int as isize) + vup[1 as libc::c_int as usize] * -height;
            point[2 as libc::c_int as usize] =
                *org.offset(2 as libc::c_int as isize) + vup[2 as libc::c_int as usize] * -height;
            point[0 as libc::c_int as usize] =
                point[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize] * -width;
            point[1 as libc::c_int as usize] =
                point[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize] * -width;
            point[2 as libc::c_int as usize] =
                point[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize] * -width
        }
        verts[0 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[0 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[0 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[0 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + ru_1[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + ru_1[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + ru_1[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vup[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vup[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vup[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * height)
        }
        verts[1 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[1 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[1 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[1 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + rr_1[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + rr_1[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + rr_1[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vright[0 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vright[1 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vright[2 as libc::c_int as usize] * (2 as libc::c_int as libc::c_float * width)
        }
        verts[2 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[2 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[2 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[2 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        if (*p).roll != 0 {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + ru_1[0 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + ru_1[1 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + ru_1[2 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height)
        } else {
            point[0 as libc::c_int as usize] = point[0 as libc::c_int as usize]
                + vup[0 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[1 as libc::c_int as usize] = point[1 as libc::c_int as usize]
                + vup[1 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height);
            point[2 as libc::c_int as usize] = point[2 as libc::c_int as usize]
                + vup[2 as libc::c_int as usize] * (-(2 as libc::c_int) as libc::c_float * height)
        }
        verts[3 as libc::c_int as usize].xyz[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].xyz[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize];
        verts[3 as libc::c_int as usize].st[0 as libc::c_int as usize] =
            1 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].st[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_float;
        verts[3 as libc::c_int as usize].modulate[0 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[1 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[2 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte;
        verts[3 as libc::c_int as usize].modulate[3 as libc::c_int as usize] =
            255 as libc::c_int as crate::src::qcommon::q_shared::byte
    }
    // done.
    if (*p).pshader == 0 {
        // (SA) temp commented out for DM
        //		CG_Printf ("CG_AddParticleToScene type %d p->pshader == ZERO\n", p->type);
        return;
    }
    if (*p).type_0 == P_WEATHER as libc::c_int
        || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        || (*p).type_0 == P_WEATHER_FLURRY as libc::c_int
    {
        crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
            (*p).pshader,
            3 as libc::c_int,
            TRIverts.as_mut_ptr() as *const crate::tr_types_h::polyVert_t,
        );
    } else {
        crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
            (*p).pshader,
            4 as libc::c_int,
            verts.as_mut_ptr() as *const crate::tr_types_h::polyVert_t,
        );
    };
}
// Ridah, made this static so it doesn't interfere with other files

static mut roll: libc::c_float = 0.0f64 as libc::c_float;
/*
===============
CG_AddParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticles() {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut alpha: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut time2: libc::c_float = 0.;
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut active: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut tail: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut rotate_ang: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if initparticles as u64 == 0 {
        CG_ClearParticles();
    }
    vforward[0 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [0 as libc::c_int as usize][0 as libc::c_int as usize];
    vforward[1 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [0 as libc::c_int as usize][1 as libc::c_int as usize];
    vforward[2 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [0 as libc::c_int as usize][2 as libc::c_int as usize];
    vright[0 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [1 as libc::c_int as usize][0 as libc::c_int as usize];
    vright[1 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [1 as libc::c_int as usize][1 as libc::c_int as usize];
    vright[2 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [1 as libc::c_int as usize][2 as libc::c_int as usize];
    vup[0 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [2 as libc::c_int as usize][0 as libc::c_int as usize];
    vup[1 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [2 as libc::c_int as usize][1 as libc::c_int as usize];
    vup[2 as libc::c_int as usize] = crate::src::cgame::cg_main::cg.refdef.viewaxis
        [2 as libc::c_int as usize][2 as libc::c_int as usize];
    crate::src::qcommon::q_math::vectoangles(
        crate::src::cgame::cg_main::cg.refdef.viewaxis[0 as libc::c_int as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        rotate_ang.as_mut_ptr(),
    );
    roll = (roll as libc::c_double
        + (crate::src::cgame::cg_main::cg.time as libc::c_float - oldtime) as libc::c_double
            * 0.1f64) as libc::c_float;
    rotate_ang[2 as libc::c_int as usize] =
        (rotate_ang[2 as libc::c_int as usize] as libc::c_double + roll as libc::c_double * 0.9f64)
            as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        rotate_ang.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        rforward.as_mut_ptr(),
        rright.as_mut_ptr(),
        rup.as_mut_ptr(),
    );
    oldtime = crate::src::cgame::cg_main::cg.time as libc::c_float;
    active = 0 as *mut cparticle_t;
    tail = 0 as *mut cparticle_t;
    let mut current_block_54: u64;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        time = ((crate::src::cgame::cg_main::cg.time as libc::c_float - (*p).time)
            as libc::c_double
            * 0.001f64) as libc::c_float;
        alpha = (*p).alpha + time * (*p).alphavel;
        if alpha <= 0 as libc::c_int as libc::c_float {
            // faded out
            (*p).next = free_particles;
            free_particles = p;
            (*p).type_0 = 0 as libc::c_int;
            (*p).color = 0 as libc::c_int;
            (*p).alpha = 0 as libc::c_int as libc::c_float
        } else {
            if (*p).type_0 == P_SMOKE as libc::c_int
                || (*p).type_0 == P_ANIM as libc::c_int
                || (*p).type_0 == P_BLEED as libc::c_int
                || (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
            {
                if crate::src::cgame::cg_main::cg.time as libc::c_float > (*p).endtime {
                    (*p).next = free_particles;
                    free_particles = p;
                    (*p).type_0 = 0 as libc::c_int;
                    (*p).color = 0 as libc::c_int;
                    (*p).alpha = 0 as libc::c_int as libc::c_float;
                    current_block_54 = 12599329904712511516;
                } else {
                    current_block_54 = 11459959175219260272;
                }
            } else {
                current_block_54 = 11459959175219260272;
            }
            match current_block_54 {
                12599329904712511516 => {}
                _ => {
                    if (*p).type_0 == P_WEATHER_FLURRY as libc::c_int {
                        if crate::src::cgame::cg_main::cg.time as libc::c_float > (*p).endtime {
                            (*p).next = free_particles;
                            free_particles = p;
                            (*p).type_0 = 0 as libc::c_int;
                            (*p).color = 0 as libc::c_int;
                            (*p).alpha = 0 as libc::c_int as libc::c_float;
                            current_block_54 = 12599329904712511516;
                        } else {
                            current_block_54 = 5529461102203738653;
                        }
                    } else {
                        current_block_54 = 5529461102203738653;
                    }
                    match current_block_54 {
                        12599329904712511516 => {}
                        _ => {
                            if (*p).type_0 == P_FLAT_SCALEUP_FADE as libc::c_int {
                                if crate::src::cgame::cg_main::cg.time as libc::c_float
                                    > (*p).endtime
                                {
                                    (*p).next = free_particles;
                                    free_particles = p;
                                    (*p).type_0 = 0 as libc::c_int;
                                    (*p).color = 0 as libc::c_int;
                                    (*p).alpha = 0 as libc::c_int as libc::c_float;
                                    current_block_54 = 12599329904712511516;
                                } else {
                                    current_block_54 = 6717214610478484138;
                                }
                            } else {
                                current_block_54 = 6717214610478484138;
                            }
                            match current_block_54 {
                                12599329904712511516 => {}
                                _ => {
                                    if ((*p).type_0 == P_BAT as libc::c_int
                                        || (*p).type_0 == P_SPRITE as libc::c_int)
                                        && (*p).endtime < 0 as libc::c_int as libc::c_float
                                    {
                                        // temporary sprite
                                        CG_AddParticleToScene(p, (*p).org.as_mut_ptr(), alpha);
                                        (*p).next = free_particles;
                                        free_particles = p;
                                        (*p).type_0 = 0 as libc::c_int;
                                        (*p).color = 0 as libc::c_int;
                                        (*p).alpha = 0 as libc::c_int as libc::c_float
                                    } else {
                                        (*p).next = 0 as *mut particle_s;
                                        if tail.is_null() {
                                            tail = p;
                                            active = tail
                                        } else {
                                            (*tail).next = p;
                                            tail = p
                                        }
                                        if alpha as libc::c_double > 1.0f64 {
                                            alpha = 1 as libc::c_int as libc::c_float
                                        }
                                        time2 = time * time;
                                        org[0 as libc::c_int as usize] = (*p).org
                                            [0 as libc::c_int as usize]
                                            + (*p).vel[0 as libc::c_int as usize] * time
                                            + (*p).accel[0 as libc::c_int as usize] * time2;
                                        org[1 as libc::c_int as usize] = (*p).org
                                            [1 as libc::c_int as usize]
                                            + (*p).vel[1 as libc::c_int as usize] * time
                                            + (*p).accel[1 as libc::c_int as usize] * time2;
                                        org[2 as libc::c_int as usize] = (*p).org
                                            [2 as libc::c_int as usize]
                                            + (*p).vel[2 as libc::c_int as usize] * time
                                            + (*p).accel[2 as libc::c_int as usize] * time2;
                                        CG_AddParticleToScene(p, org.as_mut_ptr(), alpha);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        p = next
    }
    active_particles = active;
}
/*
======================
CG_AddParticles
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSnowFlurry(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut turb: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qtrue;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnowFlurry pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).color = 0 as libc::c_int;
    (*p).alpha = 0.90f32;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).start = (*cent).currentState.origin2[0 as libc::c_int as usize];
    (*p).end = (*cent).currentState.origin2[1 as libc::c_int as usize];
    (*p).endtime =
        (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time) as libc::c_float;
    (*p).startfade =
        (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time2) as libc::c_float;
    (*p).pshader = pshader;
    if ::libc::rand() % 100 as libc::c_int > 90 as libc::c_int {
        (*p).height = 32 as libc::c_int as libc::c_float;
        (*p).width = 32 as libc::c_int as libc::c_float;
        (*p).alpha = 0.10f32
    } else {
        (*p).height = 1 as libc::c_int as libc::c_float;
        (*p).width = 1 as libc::c_int as libc::c_float
    }
    (*p).vel[2 as libc::c_int as usize] =
        -(20 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*p).type_0 = P_WEATHER_FLURRY as libc::c_int;
    if turb as u64 != 0 {
        (*p).vel[2 as libc::c_int as usize] =
            -(10 as libc::c_int) as crate::src::qcommon::q_shared::vec_t
    }
    (*p).org[0 as libc::c_int as usize] = (*cent).currentState.origin[0 as libc::c_int as usize];
    (*p).org[1 as libc::c_int as usize] = (*cent).currentState.origin[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = (*cent).currentState.origin[2 as libc::c_int as usize];
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = (*p).vel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).vel[0 as libc::c_int as usize] = ((*p).vel[0 as libc::c_int as usize] as libc::c_double
        + (((*cent).currentState.angles[0 as libc::c_int as usize]
            * 32 as libc::c_int as libc::c_float) as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 16 as libc::c_int as libc::c_double))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = ((*p).vel[1 as libc::c_int as usize] as libc::c_double
        + (((*cent).currentState.angles[1 as libc::c_int as usize]
            * 32 as libc::c_int as libc::c_float) as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 16 as libc::c_int as libc::c_double))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] += (*cent).currentState.angles[2 as libc::c_int as usize];
    if turb as u64 != 0 {
        (*p).accel[0 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 16 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 16 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t
    };
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSnow(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut turb: libc::c_int,
    mut range: libc::c_float,
    mut snum: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).color = 0 as libc::c_int;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).start = *origin.offset(2 as libc::c_int as isize);
    (*p).end = *origin2.offset(2 as libc::c_int as isize);
    (*p).pshader = pshader;
    (*p).height = 1 as libc::c_int as libc::c_float;
    (*p).width = 1 as libc::c_int as libc::c_float;
    (*p).vel[2 as libc::c_int as usize] =
        -(50 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    if turb != 0 {
        (*p).type_0 = P_WEATHER_TURBULENT as libc::c_int;
        (*p).vel[2 as libc::c_int as usize] = (-(50 as libc::c_int) as libc::c_double * 1.3f64)
            as crate::src::qcommon::q_shared::vec_t
    } else {
        (*p).type_0 = P_WEATHER as libc::c_int
    }
    (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*p).org[0 as libc::c_int as usize] = ((*p).org[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * range as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1 as libc::c_int as usize] = ((*p).org[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * range as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[2 as libc::c_int as usize] = ((*p).org[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * ((*p).start - (*p).end) as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = (*p).vel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    if turb != 0 {
        (*p).vel[0 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 16 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 16 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBubble(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut turb: libc::c_int,
    mut range: libc::c_float,
    mut snum: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut randsize: libc::c_float = 0.;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).color = 0 as libc::c_int;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).start = *origin.offset(2 as libc::c_int as isize);
    (*p).end = *origin2.offset(2 as libc::c_int as isize);
    (*p).pshader = pshader;
    randsize = (1 as libc::c_int as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 0.5f64) as libc::c_float;
    (*p).height = randsize;
    (*p).width = randsize;
    (*p).vel[2 as libc::c_int as usize] = (50 as libc::c_int as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 10 as libc::c_int as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    if turb != 0 {
        (*p).type_0 = P_BUBBLE_TURBULENT as libc::c_int;
        (*p).vel[2 as libc::c_int as usize] =
            (50 as libc::c_int as libc::c_double * 1.3f64) as crate::src::qcommon::q_shared::vec_t
    } else {
        (*p).type_0 = P_BUBBLE as libc::c_int
    }
    (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*p).org[0 as libc::c_int as usize] = ((*p).org[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * range as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1 as libc::c_int as usize] = ((*p).org[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * range as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[2 as libc::c_int as usize] = ((*p).org[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * ((*p).start - (*p).end) as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = (*p).vel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    if turb != 0 {
        (*p).vel[0 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 4 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 4 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSmoke(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    // using cent->density = enttime
    //		 cent->frame = startfade
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleSmoke == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).endtime =
        (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time) as libc::c_float;
    (*p).startfade =
        (crate::src::cgame::cg_main::cg.time + (*cent).currentState.time2) as libc::c_float;
    (*p).color = 0 as libc::c_int;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).start = (*cent).currentState.origin[2 as libc::c_int as usize];
    (*p).end = (*cent).currentState.origin2[2 as libc::c_int as usize];
    (*p).pshader = pshader;
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).height = 8 as libc::c_int as libc::c_float;
    (*p).width = 8 as libc::c_int as libc::c_float;
    (*p).endheight = 32 as libc::c_int as libc::c_float;
    (*p).endwidth = 32 as libc::c_int as libc::c_float;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = (*cent).currentState.origin[0 as libc::c_int as usize];
    (*p).org[1 as libc::c_int as usize] = (*cent).currentState.origin[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = (*cent).currentState.origin[2 as libc::c_int as usize];
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = (*p).vel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).vel[2 as libc::c_int as usize] = 5 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    if (*cent).currentState.frame == 1 as libc::c_int {
        // reverse gravity
        (*p).vel[2 as libc::c_int as usize] *= -(1 as libc::c_int) as libc::c_float
    }
    (*p).roll = (8 as libc::c_int as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 4 as libc::c_int as libc::c_double) as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBulletDebris(
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as libc::c_float;
    (*p).startfade =
        (crate::src::cgame::cg_main::cg.time + duration / 2 as libc::c_int) as libc::c_float;
    (*p).color = 3 as libc::c_int;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).height = 0.5f64 as libc::c_float;
    (*p).width = 0.5f64 as libc::c_float;
    (*p).endheight = 0.5f64 as libc::c_float;
    (*p).endwidth = 0.5f64 as libc::c_float;
    (*p).pshader = crate::src::cgame::cg_main::cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
    (*p).vel[0 as libc::c_int as usize] = *vel.offset(0 as libc::c_int as isize);
    (*p).vel[1 as libc::c_int as usize] = *vel.offset(1 as libc::c_int as isize);
    (*p).vel[2 as libc::c_int as usize] = *vel.offset(2 as libc::c_int as isize);
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        -(60 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] += -(20 as libc::c_int) as libc::c_float;
}
/*
======================
CG_ParticleExplosion
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleExplosion(
    mut animStr: *mut libc::c_char,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: libc::c_int,
    mut sizeStart: libc::c_int,
    mut sizeEnd: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut anim: libc::c_int = 0;
    if animStr < 10 as libc::c_int as *mut libc::c_char {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ParticleExplosion: animStr is probably an index rather than a string\x00"
                as *const u8 as *const libc::c_char,
        );
    }
    // find the animation string
    anim = 0 as libc::c_int; // for sprites that are stretch in either direction
    while !shaderAnimNames[anim as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(animStr, shaderAnimNames[anim as usize]) == 0 {
            break;
        }
        anim += 1
    }
    if shaderAnimNames[anim as usize].is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_ParticleExplosion: unknown animation string: %s\x00" as *const u8
                as *const libc::c_char,
            animStr,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).alpha = 0.5f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    if duration < 0 as libc::c_int {
        duration *= -(1 as libc::c_int);
        (*p).roll = 0 as libc::c_int
    } else {
        (*p).roll = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 179 as libc::c_int as libc::c_double) as libc::c_int
    }
    (*p).shaderAnim = anim;
    (*p).width = sizeStart as libc::c_float;
    (*p).height = sizeStart as libc::c_float * shaderAnimSTRatio[anim as usize];
    (*p).endheight = sizeEnd as libc::c_float;
    (*p).endwidth = sizeEnd as libc::c_float * shaderAnimSTRatio[anim as usize];
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as libc::c_float;
    (*p).type_0 = P_ANIM as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*p).vel[0 as libc::c_int as usize] = *vel.offset(0 as libc::c_int as isize);
    (*p).vel[1 as libc::c_int as usize] = *vel.offset(1 as libc::c_int as isize);
    (*p).vel[2 as libc::c_int as usize] = *vel.offset(2 as libc::c_int as isize);
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
}
// Rafael Shrapnel
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleShrapnel(mut _le: *mut crate::cg_local_h::localEntity_t) {}
// done.
#[no_mangle]

pub unsafe extern "C" fn CG_NewParticleArea(mut num: libc::c_int) -> libc::c_int {
    // const char *str;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut range: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut turb: libc::c_int = 0;
    let mut numparticles: libc::c_int = 0;
    let mut snum: libc::c_int = 0;
    str = crate::src::cgame::cg_main::CG_ConfigString(num) as *mut libc::c_char;
    if *str.offset(0 as libc::c_int as isize) == 0 {
        return 0 as libc::c_int;
    }
    // returns type 128 64 or 32
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    type_0 = atoi(token);
    if type_0 == 1 as libc::c_int {
        range = 128 as libc::c_int as libc::c_float
    } else if type_0 == 2 as libc::c_int {
        range = 64 as libc::c_int as libc::c_float
    } else if type_0 == 3 as libc::c_int {
        range = 32 as libc::c_int as libc::c_float
    } else if type_0 == 0 as libc::c_int {
        range = 256 as libc::c_int as libc::c_float
    } else if type_0 == 4 as libc::c_int {
        range = 8 as libc::c_int as libc::c_float
    } else if type_0 == 5 as libc::c_int {
        range = 16 as libc::c_int as libc::c_float
    } else if type_0 == 6 as libc::c_int {
        range = 32 as libc::c_int as libc::c_float
    } else if type_0 == 7 as libc::c_int {
        range = 64 as libc::c_int as libc::c_float
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
        origin[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
        origin2[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    numparticles = atoi(token);
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    turb = atoi(token);
    token = crate::src::qcommon::q_shared::COM_Parse(&mut str);
    snum = atoi(token);
    i = 0 as libc::c_int;
    while i < numparticles {
        if type_0 >= 4 as libc::c_int {
            CG_ParticleBubble(
                crate::src::cgame::cg_main::cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        } else {
            CG_ParticleSnow(
                crate::src::cgame::cg_main::cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        }
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn CG_SnowLink(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut particleOn: crate::src::qcommon::q_shared::qboolean,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: libc::c_int = 0;
    id = (*cent).currentState.frame;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_WEATHER as libc::c_int
            || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        {
            if (*p).snum == id {
                if particleOn as u64 != 0 {
                    (*p).link = crate::src::qcommon::q_shared::qtrue
                } else {
                    (*p).link = crate::src::qcommon::q_shared::qfalse
                }
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleImpactSmokePuff(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).alpha = 0.25f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = (2.0f64
        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
            - 0.5f64)
        * 179 as libc::c_int as libc::c_double) as libc::c_int;
    (*p).pshader = pshader;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 1000 as libc::c_int) as libc::c_float;
    (*p).startfade = (crate::src::cgame::cg_main::cg.time + 100 as libc::c_int) as libc::c_float;
    (*p).width = (::libc::rand() % 4 as libc::c_int + 8 as libc::c_int) as libc::c_float;
    (*p).height = (::libc::rand() % 4 as libc::c_int + 8 as libc::c_int) as libc::c_float;
    (*p).endheight = (*p).height * 2 as libc::c_int as libc::c_float;
    (*p).endwidth = (*p).width * 2 as libc::c_int as libc::c_float;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 500 as libc::c_int) as libc::c_float;
    (*p).type_0 = P_SMOKE_IMPACT as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*p).vel[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] = 20 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[0 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[2 as libc::c_int as usize] =
        20 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).rotate = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_Bleed(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut _dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut fleshEntityNum: libc::c_int,
    mut duration: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_Bleed pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = 0 as libc::c_int;
    (*p).pshader = pshader;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as libc::c_float;
    if fleshEntityNum != 0 {
        (*p).startfade = crate::src::cgame::cg_main::cg.time as libc::c_float
    } else {
        (*p).startfade = (crate::src::cgame::cg_main::cg.time + 100 as libc::c_int) as libc::c_float
    }
    (*p).width = 4 as libc::c_int as libc::c_float;
    (*p).height = 4 as libc::c_int as libc::c_float;
    (*p).endheight = (4 as libc::c_int + ::libc::rand() % 3 as libc::c_int) as libc::c_float;
    (*p).endwidth = (*p).endheight;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*p).vel[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] =
        -(20 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = ::libc::rand() % 179 as libc::c_int;
    (*p).color = 2 as libc::c_int;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilParticle(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut time: libc::c_int = 0;
    let mut time2: libc::c_int = 0;
    let mut ratio: libc::c_float = 0.;
    let mut duration: libc::c_float = 1500 as libc::c_int as libc::c_float;
    time = crate::src::cgame::cg_main::cg.time;
    time2 = crate::src::cgame::cg_main::cg.time + (*cent).currentState.time;
    ratio = 1 as libc::c_int as libc::c_float - time as libc::c_float / time2 as libc::c_float;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_OilParticle == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = 0 as libc::c_int;
    (*p).pshader = pshader;
    (*p).endtime = crate::src::cgame::cg_main::cg.time as libc::c_float + duration;
    (*p).startfade = (*p).endtime;
    (*p).width = 1 as libc::c_int as libc::c_float;
    (*p).height = 3 as libc::c_int as libc::c_float;
    (*p).endheight = 3 as libc::c_int as libc::c_float;
    (*p).endwidth = 1 as libc::c_int as libc::c_float;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = (*cent).currentState.origin[0 as libc::c_int as usize];
    (*p).org[1 as libc::c_int as usize] = (*cent).currentState.origin[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = (*cent).currentState.origin[2 as libc::c_int as usize];
    (*p).vel[0 as libc::c_int as usize] = (*cent).currentState.origin2[0 as libc::c_int as usize]
        * (16 as libc::c_int as libc::c_float * ratio);
    (*p).vel[1 as libc::c_int as usize] = (*cent).currentState.origin2[1 as libc::c_int as usize]
        * (16 as libc::c_int as libc::c_float * ratio);
    (*p).vel[2 as libc::c_int as usize] = (*cent).currentState.origin2[2 as libc::c_int as usize];
    (*p).snum = 1.0f32 as libc::c_int;
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).accel[2 as libc::c_int as usize] =
        -(20 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = ::libc::rand() % 179 as libc::c_int;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilSlick(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut cent: *mut crate::cg_local_h::centity_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_Particle_OilSlick == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    if (*cent).currentState.angles2[2 as libc::c_int as usize] != 0. {
        (*p).endtime = crate::src::cgame::cg_main::cg.time as libc::c_float
            + (*cent).currentState.angles2[2 as libc::c_int as usize]
    } else {
        (*p).endtime = (crate::src::cgame::cg_main::cg.time + 60000 as libc::c_int) as libc::c_float
    }
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = 0 as libc::c_int;
    (*p).pshader = pshader;
    if (*cent).currentState.angles2[0 as libc::c_int as usize] != 0.
        || (*cent).currentState.angles2[1 as libc::c_int as usize] != 0.
    {
        (*p).width = (*cent).currentState.angles2[0 as libc::c_int as usize];
        (*p).height = (*cent).currentState.angles2[0 as libc::c_int as usize];
        (*p).endheight = (*cent).currentState.angles2[1 as libc::c_int as usize];
        (*p).endwidth = (*cent).currentState.angles2[1 as libc::c_int as usize]
    } else {
        (*p).width = 8 as libc::c_int as libc::c_float;
        (*p).height = 8 as libc::c_int as libc::c_float;
        (*p).endheight = 16 as libc::c_int as libc::c_float;
        (*p).endwidth = 16 as libc::c_int as libc::c_float
    }
    (*p).type_0 = P_FLAT_SCALEUP as libc::c_int;
    (*p).snum = 1.0f64 as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = (*cent).currentState.origin[0 as libc::c_int as usize];
    (*p).org[1 as libc::c_int as usize] = (*cent).currentState.origin[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = (*cent).currentState.origin[2 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = ((*p).org[2 as libc::c_int as usize] as libc::c_double
        + (0.55f64
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 0.5f64))
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = ::libc::rand() % 179 as libc::c_int;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn CG_OilSlickRemove(mut _cent: *mut crate::cg_local_h::centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: libc::c_int = 0;
    id = 1.0f32 as libc::c_int;
    if id == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_OilSlickRevove NULL id\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_FLAT_SCALEUP as libc::c_int {
            if (*p).snum == id {
                (*p).endtime =
                    (crate::src::cgame::cg_main::cg.time + 100 as libc::c_int) as libc::c_float;
                (*p).startfade = (*p).endtime;
                (*p).type_0 = P_FLAT_SCALEUP_FADE as libc::c_int
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn ValidBloodPool(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut this_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut x_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut center_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end_pos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut fwidth: libc::c_int = 0;
    let mut fheight: libc::c_int = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    fwidth = 16 as libc::c_int;
    fheight = 16 as libc::c_int;
    normal[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    normal[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    normal[2 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::vectoangles(
        normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    center_pos[0 as libc::c_int as usize] = (*start.offset(0 as libc::c_int as isize)
        as libc::c_double
        + normal[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
        as crate::src::qcommon::q_shared::vec_t;
    center_pos[1 as libc::c_int as usize] = (*start.offset(1 as libc::c_int as isize)
        as libc::c_double
        + normal[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
        as crate::src::qcommon::q_shared::vec_t;
    center_pos[2 as libc::c_int as usize] = (*start.offset(2 as libc::c_int as isize)
        as libc::c_double
        + normal[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
        as crate::src::qcommon::q_shared::vec_t;
    x = -fwidth / 2 as libc::c_int;
    while x < fwidth {
        x_pos[0 as libc::c_int as usize] = center_pos[0 as libc::c_int as usize]
            + right[0 as libc::c_int as usize] * x as libc::c_float;
        x_pos[1 as libc::c_int as usize] = center_pos[1 as libc::c_int as usize]
            + right[1 as libc::c_int as usize] * x as libc::c_float;
        x_pos[2 as libc::c_int as usize] = center_pos[2 as libc::c_int as usize]
            + right[2 as libc::c_int as usize] * x as libc::c_float;
        y = -fheight / 2 as libc::c_int;
        while y < fheight {
            this_pos[0 as libc::c_int as usize] = x_pos[0 as libc::c_int as usize]
                + up[0 as libc::c_int as usize] * y as libc::c_float;
            this_pos[1 as libc::c_int as usize] = x_pos[1 as libc::c_int as usize]
                + up[1 as libc::c_int as usize] * y as libc::c_float;
            this_pos[2 as libc::c_int as usize] = x_pos[2 as libc::c_int as usize]
                + up[2 as libc::c_int as usize] * y as libc::c_float;
            end_pos[0 as libc::c_int as usize] = (this_pos[0 as libc::c_int as usize]
                as libc::c_double
                + normal[0 as libc::c_int as usize] as libc::c_double
                    * (-0.5f64 * 2 as libc::c_int as libc::c_double))
                as crate::src::qcommon::q_shared::vec_t;
            end_pos[1 as libc::c_int as usize] = (this_pos[1 as libc::c_int as usize]
                as libc::c_double
                + normal[1 as libc::c_int as usize] as libc::c_double
                    * (-0.5f64 * 2 as libc::c_int as libc::c_double))
                as crate::src::qcommon::q_shared::vec_t;
            end_pos[2 as libc::c_int as usize] = (this_pos[2 as libc::c_int as usize]
                as libc::c_double
                + normal[2 as libc::c_int as usize] as libc::c_double
                    * (-0.5f64 * 2 as libc::c_int as libc::c_double))
                as crate::src::qcommon::q_shared::vec_t;
            crate::src::cgame::cg_predict::CG_Trace(
                &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
                this_pos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0 as *const crate::src::qcommon::q_shared::vec_t,
                0 as *const crate::src::qcommon::q_shared::vec_t,
                end_pos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                -(1 as libc::c_int),
                1 as libc::c_int,
            );
            if trace.entityNum < ((1 as libc::c_int) << 10 as libc::c_int) - 2 as libc::c_int {
                // may only land on world
                return crate::src::qcommon::q_shared::qfalse;
            }
            if !(trace.startsolid as u64 == 0 && trace.fraction < 1 as libc::c_int as libc::c_float)
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            y += fheight
        }
        x += fwidth
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_BloodPool(
    mut _le: *mut crate::cg_local_h::localEntity_t,
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut tr: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut legit: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rndSize: libc::c_float = 0.;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_BloodPool pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    start[0 as libc::c_int as usize] = (*tr).endpos[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*tr).endpos[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*tr).endpos[2 as libc::c_int as usize];
    legit = ValidBloodPool(start.as_mut_ptr());
    if legit as u64 == 0 {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + 3000 as libc::c_int) as libc::c_float;
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = 0 as libc::c_int;
    (*p).pshader = pshader;
    rndSize = (0.4f64
        + ((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
            * 0.6f64) as libc::c_float;
    (*p).width = 8 as libc::c_int as libc::c_float * rndSize;
    (*p).height = 8 as libc::c_int as libc::c_float * rndSize;
    (*p).endheight = 16 as libc::c_int as libc::c_float * rndSize;
    (*p).endwidth = 16 as libc::c_int as libc::c_float * rndSize;
    (*p).type_0 = P_FLAT_SCALEUP as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
    (*p).org[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
    (*p).org[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
    (*p).vel[0 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
    (*p).roll = ::libc::rand() % 179 as libc::c_int;
    (*p).alpha = 0.75f64 as libc::c_float;
    (*p).color = 2 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBloodCloud(
    mut _cent: *mut crate::cg_local_h::centity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut length: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut crittersize: libc::c_float = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: libc::c_int = 0;
    dist = 0 as libc::c_int as libc::c_float;
    length = VectorLength(dir as *const crate::src::qcommon::q_shared::vec_t);
    crate::src::qcommon::q_math::vectoangles(
        dir as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    crittersize = 32 as libc::c_int as libc::c_float;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1 as libc::c_int as libc::c_float {
        dist = 1 as libc::c_int as libc::c_float
    }
    point[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while (i as libc::c_float) < dist {
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + forward[0 as libc::c_int as usize] * crittersize;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + forward[1 as libc::c_int as usize] * crittersize;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + forward[2 as libc::c_int as usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = 0 as libc::c_int as libc::c_float;
        (*p).roll = 0 as libc::c_int;
        (*p).pshader = crate::src::cgame::cg_main::cgs.media.smokePuffShader;
        (*p).endtime = ((crate::src::cgame::cg_main::cg.time + 350 as libc::c_int)
            as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                    - 0.5f64)
                * 100 as libc::c_int as libc::c_double) as libc::c_float;
        (*p).startfade = crate::src::cgame::cg_main::cg.time as libc::c_float;
        (*p).width = 32 as libc::c_int as libc::c_float;
        (*p).height = 32 as libc::c_int as libc::c_float;
        (*p).endheight = 32 as libc::c_int as libc::c_float;
        (*p).endwidth = 32 as libc::c_int as libc::c_float;
        (*p).type_0 = P_SMOKE as libc::c_int;
        (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
        (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
        (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
        (*p).vel[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[2 as libc::c_int as usize] =
            -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
        (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
        (*p).rotate = crate::src::qcommon::q_shared::qfalse;
        (*p).roll = ::libc::rand() % 179 as libc::c_int;
        (*p).color = 2 as libc::c_int;
        (*p).alpha = 0.75f64 as libc::c_float;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSparks(
    mut org: *mut crate::src::qcommon::q_shared::vec_t,
    mut vel: *mut crate::src::qcommon::q_shared::vec_t,
    mut duration: libc::c_int,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut speed: libc::c_float,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as libc::c_float;
    (*p).startfade =
        (crate::src::cgame::cg_main::cg.time + duration / 2 as libc::c_int) as libc::c_float;
    (*p).color = 3 as libc::c_int;
    (*p).alpha = 0.4f32;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).height = 0.5f64 as libc::c_float;
    (*p).width = 0.5f64 as libc::c_float;
    (*p).endheight = 0.5f64 as libc::c_float;
    (*p).endwidth = 0.5f64 as libc::c_float;
    (*p).pshader = crate::src::cgame::cg_main::cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
    (*p).org[0 as libc::c_int as usize] = ((*p).org[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * x as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).org[1 as libc::c_int as usize] = ((*p).org[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * y as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[0 as libc::c_int as usize] = *vel.offset(0 as libc::c_int as isize);
    (*p).vel[1 as libc::c_int as usize] = *vel.offset(1 as libc::c_int as isize);
    (*p).vel[2 as libc::c_int as usize] = *vel.offset(2 as libc::c_int as isize);
    (*p).accel[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
    (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
    (*p).vel[0 as libc::c_int as usize] = ((*p).vel[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 4 as libc::c_int as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[1 as libc::c_int as usize] = ((*p).vel[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 4 as libc::c_int as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).vel[2 as libc::c_int as usize] = ((*p).vel[2 as libc::c_int as usize] as libc::c_double
        + (20 as libc::c_int as libc::c_double
            + 2.0f64
                * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 10 as libc::c_int as libc::c_double)
            * speed as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[0 as libc::c_int as usize] = (2.0f64
        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
            - 0.5f64)
        * 4 as libc::c_int as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
    (*p).accel[1 as libc::c_int as usize] = (2.0f64
        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
            - 0.5f64)
        * 4 as libc::c_int as libc::c_double)
        as crate::src::qcommon::q_shared::vec_t;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleDust(
    mut _cent: *mut crate::cg_local_h::centity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut length: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut crittersize: libc::c_float = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: libc::c_int = 0;
    dist = 0 as libc::c_int as libc::c_float;
    *dir.offset(0 as libc::c_int as isize) = -*dir.offset(0 as libc::c_int as isize);
    *dir.offset(1 as libc::c_int as isize) = -*dir.offset(1 as libc::c_int as isize);
    *dir.offset(2 as libc::c_int as isize) = -*dir.offset(2 as libc::c_int as isize);
    length = VectorLength(dir as *const crate::src::qcommon::q_shared::vec_t);
    crate::src::qcommon::q_math::vectoangles(
        dir as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    crittersize = 32 as libc::c_int as libc::c_float;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1 as libc::c_int as libc::c_float {
        dist = 1 as libc::c_int as libc::c_float
    }
    point[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    point[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    point[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while (i as libc::c_float) < dist {
        point[0 as libc::c_int as usize] =
            point[0 as libc::c_int as usize] + forward[0 as libc::c_int as usize] * crittersize;
        point[1 as libc::c_int as usize] =
            point[1 as libc::c_int as usize] + forward[1 as libc::c_int as usize] * crittersize;
        point[2 as libc::c_int as usize] =
            point[2 as libc::c_int as usize] + forward[2 as libc::c_int as usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
        (*p).alpha = 5.0f64 as libc::c_float;
        (*p).alphavel = 0 as libc::c_int as libc::c_float;
        (*p).roll = 0 as libc::c_int;
        (*p).pshader = crate::src::cgame::cg_main::cgs.media.smokePuffShader;
        // RF, stay around for long enough to expand and dissipate naturally
        if length != 0. {
            (*p).endtime =
                ((crate::src::cgame::cg_main::cg.time + 4500 as libc::c_int) as libc::c_double
                    + 2.0f64
                        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 3500 as libc::c_int as libc::c_double) as libc::c_float
        } else {
            (*p).endtime =
                ((crate::src::cgame::cg_main::cg.time + 750 as libc::c_int) as libc::c_double
                    + 2.0f64
                        * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                            / 0x7fff as libc::c_int as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 500 as libc::c_int as libc::c_double) as libc::c_float
        }
        (*p).startfade = crate::src::cgame::cg_main::cg.time as libc::c_float;
        (*p).width = 32 as libc::c_int as libc::c_float;
        (*p).height = 32 as libc::c_int as libc::c_float;
        // RF, expand while falling
        (*p).endheight = (32 as libc::c_int as libc::c_double * 3.0f64) as libc::c_float;
        (*p).endwidth = (32 as libc::c_int as libc::c_double * 3.0f64) as libc::c_float;
        if length == 0. {
            (*p).width *= 0.2f32;
            (*p).height *= 0.2f32;
            (*p).endheight = 16 as libc::c_int as libc::c_float;
            (*p).endwidth = 16 as libc::c_int as libc::c_float
        }
        (*p).type_0 = P_SMOKE as libc::c_int;
        (*p).org[0 as libc::c_int as usize] = point[0 as libc::c_int as usize];
        (*p).org[1 as libc::c_int as usize] = point[1 as libc::c_int as usize];
        (*p).org[2 as libc::c_int as usize] = point[2 as libc::c_int as usize];
        (*p).vel[0 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 6 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[1 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 6 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).vel[2 as libc::c_int as usize] = (::libc::rand() & 0x7fff as libc::c_int)
            as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float
            * 20 as libc::c_int as libc::c_float;
        // RF, add some gravity/randomness
        (*p).accel[0 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 3 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1 as libc::c_int as usize] = (2.0f64
            * (((::libc::rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                - 0.5f64)
            * 3 as libc::c_int as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[2 as libc::c_int as usize] = (-(40 as libc::c_int) as libc::c_double * 0.4f64)
            as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).accel[1 as libc::c_int as usize] = (*p).accel[2 as libc::c_int as usize];
        (*p).accel[0 as libc::c_int as usize] = (*p).accel[1 as libc::c_int as usize];
        (*p).rotate = crate::src::qcommon::q_shared::qfalse;
        (*p).roll = ::libc::rand() % 179 as libc::c_int;
        (*p).alpha = 0.75f64 as libc::c_float;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleMisc(
    mut pshader: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: libc::c_int,
    mut duration: libc::c_int,
    mut _alpha: libc::c_float,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0 as libc::c_int as libc::c_float;
    (*p).roll = ::libc::rand() % 179 as libc::c_int;
    (*p).pshader = pshader;
    if duration > 0 as libc::c_int {
        (*p).endtime = (crate::src::cgame::cg_main::cg.time + duration) as libc::c_float
    } else {
        (*p).endtime = duration as libc::c_float
    }
    (*p).startfade = crate::src::cgame::cg_main::cg.time as libc::c_float;
    (*p).width = size as libc::c_float;
    (*p).height = size as libc::c_float;
    (*p).endheight = size as libc::c_float;
    (*p).endwidth = size as libc::c_float;
    (*p).type_0 = P_SPRITE as libc::c_int;
    (*p).org[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*p).org[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*p).org[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*p).rotate = crate::src::qcommon::q_shared::qfalse;
}

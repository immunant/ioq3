use ::libc;

pub mod q_shared_h {

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
    #[inline]

    pub unsafe extern "C" fn Distance(
        mut p1: *const crate::src::qcommon::q_shared::vec_t,
        mut p2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        v[0 as i32 as usize] = *p2.offset(0 as i32 as isize) - *p1.offset(0 as i32 as isize);
        v[1 as i32 as usize] = *p2.offset(1 as i32 as isize) - *p1.offset(1 as i32 as isize);
        v[2 as i32 as usize] = *p2.offset(2 as i32 as isize) - *p1.offset(2 as i32 as isize);
        return VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> f64 {
        return libc::strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    }
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
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
    pub time: f32,
    pub endtime: f32,
    pub org: vec3_t,
    pub vel: vec3_t,
    pub accel: vec3_t,
    pub color: i32,
    pub colorvel: f32,
    pub alpha: f32,
    pub alphavel: f32,
    pub type_0: i32,
    pub pshader: qhandle_t,
    pub height: f32,
    pub width: f32,
    pub endheight: f32,
    pub endwidth: f32,
    pub start: f32,
    pub end: f32,
    pub startfade: f32,
    pub rotate: qboolean,
    pub snum: i32,
    pub link: qboolean,
    pub shaderAnim: i32,
    pub roll: i32,
    pub accumroll: i32,
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

pub type C2RustUnnamed_25 = u32;

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

static mut shaderAnims: [[qhandle_t; 64]; 32] = [[0; 64]; 32];

static mut shaderAnimCounts: [i32; 32] = [
    23 as i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

static mut shaderAnimSTRatio: [f32; 32] = [
    1.0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    0., 0., 0., 0., 0., 0., 0., 0., 0.,
];

static mut numShaderAnims: i32 = 0;
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
    rotate: qfalse,
    snum: 0,
    link: qfalse,
    shaderAnim: 0,
    roll: 0,
    accumroll: 0,
}; 1024];
#[no_mangle]

pub static mut cl_numparticles: i32 = 1024 as i32;
#[no_mangle]

pub static mut initparticles: qboolean = qfalse;
#[no_mangle]

pub static mut vforward: vec3_t = [0.; 3];
#[no_mangle]

pub static mut vright: vec3_t = [0.; 3];
#[no_mangle]

pub static mut vup: vec3_t = [0.; 3];
#[no_mangle]

pub static mut rforward: vec3_t = [0.; 3];
#[no_mangle]

pub static mut rright: vec3_t = [0.; 3];
#[no_mangle]

pub static mut rup: vec3_t = [0.; 3];
#[no_mangle]

pub static mut oldtime: f32 = 0.;
// Ridah
/*
===============
CL_ClearParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ClearParticles() {
    let mut i: i32 = 0;
    crate::stdlib::memset(
        particles.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[cparticle_t; 1024]>() as libc::c_ulong,
    );
    free_particles = &mut *particles.as_mut_ptr().offset(0 as i32 as isize) as *mut cparticle_t;
    active_particles = 0 as *mut cparticle_t;
    i = 0 as i32;
    while i < cl_numparticles {
        particles[i as usize].next =
            &mut *particles.as_mut_ptr().offset((i + 1 as i32) as isize) as *mut cparticle_t;
        particles[i as usize].type_0 = 0 as i32;
        i += 1
    }
    particles[(cl_numparticles - 1 as i32) as usize].next = 0 as *mut particle_s;
    oldtime = cg.time as f32;
    // Ridah, init the shaderAnims
    i = 0 as i32;
    while !shaderAnimNames[i as usize].is_null() {
        let mut j: i32 = 0;
        j = 0 as i32;
        while j < shaderAnimCounts[i as usize] {
            shaderAnims[i as usize][j as usize] = trap_R_RegisterShader(va(
                b"%s%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                shaderAnimNames[i as usize],
                j + 1 as i32,
            ));
            j += 1
        }
        i += 1
    }
    numShaderAnims = i;
    // done.
    initparticles = qtrue;
}
/*
=====================
CG_AddParticleToScene
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleToScene(
    mut p: *mut cparticle_t,
    mut org: *mut vec_t,
    mut _alpha: f32,
) {
    let mut point: vec3_t = [0.; 3];
    let mut verts: [polyVert_t; 4] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    let mut time: f32 = 0.;
    let mut time2: f32 = 0.;
    let mut ratio: f32 = 0.;
    let mut invratio: f32 = 0.;
    let mut color: vec3_t = [0.; 3];
    let mut TRIverts: [polyVert_t; 3] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 3];
    let mut rright2: vec3_t = [0.; 3];
    let mut rup2: vec3_t = [0.; 3];
    if (*p).type_0 == P_WEATHER as i32
        || (*p).type_0 == P_WEATHER_TURBULENT as i32
        || (*p).type_0 == P_WEATHER_FLURRY as i32
        || (*p).type_0 == P_BUBBLE as i32
        || (*p).type_0 == P_BUBBLE_TURBULENT as i32
    {
        // create a front facing polygon
        if (*p).type_0 != P_WEATHER_FLURRY as i32 {
            if (*p).type_0 == P_BUBBLE as i32 || (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
                if *org.offset(2 as i32 as isize) > (*p).end {
                    (*p).time = cg.time as f32; // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[0 as i32 as usize] = *org.offset(0 as i32 as isize); // Ridah, fixes rare snow flakes that flicker on the ground
                    (*p).org[1 as i32 as usize] = *org.offset(1 as i32 as isize);
                    (*p).org[2 as i32 as usize] = *org.offset(2 as i32 as isize);
                    (*p).org[2 as i32 as usize] = ((*p).start as f64
                        + 2.0f64
                            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                                - 0.5f64)
                            * 4 as i32 as f64)
                        as vec_t;
                    if (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
                        (*p).vel[0 as i32 as usize] = (2.0f64
                            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                                - 0.5f64)
                            * 4 as i32 as f64)
                            as vec_t;
                        (*p).vel[1 as i32 as usize] = (2.0f64
                            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                                - 0.5f64)
                            * 4 as i32 as f64)
                            as vec_t
                    }
                }
            } else if *org.offset(2 as i32 as isize) < (*p).end {
                (*p).time = cg.time as f32;
                (*p).org[0 as i32 as usize] = *org.offset(0 as i32 as isize);
                (*p).org[1 as i32 as usize] = *org.offset(1 as i32 as isize);
                (*p).org[2 as i32 as usize] = *org.offset(2 as i32 as isize);
                while (*p).org[2 as i32 as usize] < (*p).end {
                    (*p).org[2 as i32 as usize] += (*p).start - (*p).end
                }
                if (*p).type_0 == P_WEATHER_TURBULENT as i32 {
                    (*p).vel[0 as i32 as usize] = (2.0f64
                        * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                            - 0.5f64)
                        * 16 as i32 as f64)
                        as vec_t;
                    (*p).vel[1 as i32 as usize] = (2.0f64
                        * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64
                            - 0.5f64)
                        * 16 as i32 as f64)
                        as vec_t
                }
            }
            // Rafael snow pvs check
            if (*p).link as u64 == 0 {
                return;
            }
            (*p).alpha = 1 as i32 as f32
        }
        // Ridah, had to do this or MAX_POLYS is being exceeded in village1.bsp
        if Distance(
            (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
            org as *const vec_t,
        ) > 1024 as i32 as f32
        {
            return;
        }
        // done.
        if (*p).type_0 == P_BUBBLE as i32 || (*p).type_0 == P_BUBBLE_TURBULENT as i32 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -(*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -(*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -(*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width;
            verts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            verts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            verts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
            verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
            verts[0 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            verts[0 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            verts[0 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            verts[0 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte;
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -(*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -(*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -(*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (*p).width;
            verts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            verts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            verts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
            verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
            verts[1 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            verts[1 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            verts[1 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            verts[1 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte;
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (*p).width;
            verts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            verts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            verts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
            verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
            verts[2 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            verts[2 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            verts[2 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            verts[2 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte;
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width;
            verts[3 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            verts[3 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            verts[3 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
            verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
            verts[3 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            verts[3 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            verts[3 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            verts[3 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -(*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -(*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -(*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width;
            TRIverts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            TRIverts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            TRIverts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            TRIverts[0 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
            TRIverts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
            TRIverts[0 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            TRIverts[0 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            TRIverts[0 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            TRIverts[0 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte;
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width;
            TRIverts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            TRIverts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            TRIverts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            TRIverts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
            TRIverts[1 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
            TRIverts[1 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            TRIverts[1 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            TRIverts[1 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            TRIverts[1 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte;
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (*p).width;
            TRIverts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
            TRIverts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
            TRIverts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
            TRIverts[2 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
            TRIverts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
            TRIverts[2 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
            TRIverts[2 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
            TRIverts[2 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
            TRIverts[2 as i32 as usize].modulate[3 as i32 as usize] =
                (255 as i32 as f32 * (*p).alpha) as byte
        }
    } else if (*p).type_0 == P_SPRITE as i32 {
        let mut rr: vec3_t = [0.; 3];
        let mut ru: vec3_t = [0.; 3];
        let mut rotate_ang: vec3_t = [0.; 3];
        color[0 as i32 as usize] = 1.0f64 as vec_t;
        color[1 as i32 as usize] = 1.0f64 as vec_t;
        color[2 as i32 as usize] = 0.5f64 as vec_t;
        time = cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).roll != 0 {
            vectoangles(
                cg.refdef.viewaxis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
                rotate_ang.as_mut_ptr(),
            );
            rotate_ang[2 as i32 as usize] += (*p).roll as f32;
            AngleVectors(
                rotate_ang.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr.as_mut_ptr(),
                ru.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + ru[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + ru[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + ru[2 as i32 as usize] * -height;
            point[0 as i32 as usize] = point[0 as i32 as usize] + rr[0 as i32 as usize] * -width;
            point[1 as i32 as usize] = point[1 as i32 as usize] + rr[1 as i32 as usize] * -width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + rr[2 as i32 as usize] * -width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + vright[2 as i32 as usize] * -width
        }
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + ru[0 as i32 as usize] * (2 as i32 as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + ru[1 as i32 as usize] * (2 as i32 as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + ru[2 as i32 as usize] * (2 as i32 as f32 * height)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vup[0 as i32 as usize] * (2 as i32 as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vup[1 as i32 as usize] * (2 as i32 as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vup[2 as i32 as usize] * (2 as i32 as f32 * height)
        }
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rr[0 as i32 as usize] * (2 as i32 as f32 * width);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rr[1 as i32 as usize] * (2 as i32 as f32 * width);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + rr[2 as i32 as usize] * (2 as i32 as f32 * width)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (2 as i32 as f32 * width);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (2 as i32 as f32 * width);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (2 as i32 as f32 * width)
        }
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + ru[0 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + ru[1 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + ru[2 as i32 as usize] * (-(2 as i32) as f32 * height)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vup[0 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vup[1 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vup[2 as i32 as usize] * (-(2 as i32) as f32 * height)
        }
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte
    } else if (*p).type_0 == P_SMOKE as i32 || (*p).type_0 == P_SMOKE_IMPACT as i32 {
        // create a front rotating facing polygon
        if (*p).type_0 == P_SMOKE_IMPACT as i32
            && Distance(
                (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
                org as *const vec_t,
            ) > 1024 as i32 as f32
        {
            return;
        }
        if (*p).color == 2 as i32 {
            color[0 as i32 as usize] = 0.22f32;
            color[1 as i32 as usize] = 0.0f32;
            color[2 as i32 as usize] = 0.0f32
        } else if (*p).color == 4 as i32 {
            let mut len: f32 = 0.;
            let mut greyit: f32 = 0.;
            let mut val: f32 = 0.;
            len = Distance(
                (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
                org as *const vec_t,
            );
            if len == 0. {
                len = 1 as i32 as f32
            }
            val = 4096 as i32 as f32 / len;
            greyit = (0.25f64 * val as f64) as f32;
            if greyit as f64 > 0.5f64 {
                greyit = 0.5f64 as f32
            }
            color[0 as i32 as usize] = greyit;
            color[1 as i32 as usize] = greyit;
            color[2 as i32 as usize] = greyit
        } else {
            color[0 as i32 as usize] = 1.0f64 as vec_t;
            color[1 as i32 as usize] = 1.0f64 as vec_t;
            color[2 as i32 as usize] = 1.0f64 as vec_t
        }
        time = cg.time as f32 - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if cg.time as f32 > (*p).startfade {
            invratio = 1 as i32 as f32
                - (cg.time as f32 - (*p).startfade) / ((*p).endtime - (*p).startfade);
            if (*p).color == 3 as i32 {
                let mut fval: f32 = 0.;
                fval = invratio * invratio;
                if fval < 0 as i32 as f32 {
                    fval = 0 as i32 as f32
                }
                color[0 as i32 as usize] = fval;
                color[1 as i32 as usize] = fval;
                color[2 as i32 as usize] = fval
            }
            invratio *= (*p).alpha
        } else {
            invratio = 1 as i32 as f32 * (*p).alpha
        }
        if cgs.glconfig.hardwareType as u32 == GLHW_RAGEPRO as i32 as u32 {
            invratio = 1 as i32 as f32
        }
        if invratio > 1 as i32 as f32 {
            invratio = 1 as i32 as f32
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).type_0 != P_SMOKE_IMPACT as i32 {
            let mut temp: vec3_t = [0.; 3];
            vectoangles(rforward.as_mut_ptr() as *const vec_t, temp.as_mut_ptr());
            (*p).accumroll += (*p).roll;
            temp[2 as i32 as usize] =
                (temp[2 as i32 as usize] as f64 + (*p).accumroll as f64 * 0.1f64) as vec_t;
            AngleVectors(
                temp.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rright2.as_mut_ptr(),
                rup2.as_mut_ptr(),
            );
        } else {
            rright2[0 as i32 as usize] = rright[0 as i32 as usize];
            rright2[1 as i32 as usize] = rright[1 as i32 as usize];
            rright2[2 as i32 as usize] = rright[2 as i32 as usize];
            rup2[0 as i32 as usize] = rup[0 as i32 as usize];
            rup2[1 as i32 as usize] = rup[1 as i32 as usize];
            rup2[2 as i32 as usize] = rup[2 as i32 as usize]
        }
        if (*p).rotate as u64 != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + rup2[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + rup2[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + rup2[2 as i32 as usize] * -height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rright2[0 as i32 as usize] * -width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rright2[1 as i32 as usize] * -width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + rright2[2 as i32 as usize] * -width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -(*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -(*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -(*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width
        }
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * invratio) as byte;
        if (*p).rotate as u64 != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + rup2[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + rup2[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + rup2[2 as i32 as usize] * -height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rright2[0 as i32 as usize] * width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rright2[1 as i32 as usize] * width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + rright2[2 as i32 as usize] * width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -(*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -(*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -(*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (*p).width
        }
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * invratio) as byte;
        if (*p).rotate as u64 != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + rup2[0 as i32 as usize] * height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + rup2[1 as i32 as usize] * height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + rup2[2 as i32 as usize] * height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rright2[0 as i32 as usize] * width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rright2[1 as i32 as usize] * width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + rright2[2 as i32 as usize] * width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (*p).width
        }
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * invratio) as byte;
        if (*p).rotate as u64 != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + rup2[0 as i32 as usize] * height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + rup2[1 as i32 as usize] * height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + rup2[2 as i32 as usize] * height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rright2[0 as i32 as usize] * -width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rright2[1 as i32 as usize] * -width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + rright2[2 as i32 as usize] * -width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * (*p).height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * (*p).height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * (*p).height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -(*p).width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -(*p).width;
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * -(*p).width
        }
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * invratio) as byte
    } else if (*p).type_0 == P_BLEED as i32 {
        let mut rr_0: vec3_t = [0.; 3];
        let mut ru_0: vec3_t = [0.; 3];
        let mut rotate_ang_0: vec3_t = [0.; 3];
        let mut alpha_0: f32 = 0.;
        alpha_0 = (*p).alpha;
        if cgs.glconfig.hardwareType as u32 == GLHW_RAGEPRO as i32 as u32 {
            alpha_0 = 1 as i32 as f32
        }
        if (*p).roll != 0 {
            vectoangles(
                cg.refdef.viewaxis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
                rotate_ang_0.as_mut_ptr(),
            );
            rotate_ang_0[2 as i32 as usize] += (*p).roll as f32;
            AngleVectors(
                rotate_ang_0.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr_0.as_mut_ptr(),
                ru_0.as_mut_ptr(),
            );
        } else {
            ru_0[0 as i32 as usize] = vup[0 as i32 as usize];
            ru_0[1 as i32 as usize] = vup[1 as i32 as usize];
            ru_0[2 as i32 as usize] = vup[2 as i32 as usize];
            rr_0[0 as i32 as usize] = vright[0 as i32 as usize];
            rr_0[1 as i32 as usize] = vright[1 as i32 as usize];
            rr_0[2 as i32 as usize] = vright[2 as i32 as usize]
        }
        point[0 as i32 as usize] =
            *org.offset(0 as i32 as isize) + ru_0[0 as i32 as usize] * -(*p).height;
        point[1 as i32 as usize] =
            *org.offset(1 as i32 as isize) + ru_0[1 as i32 as usize] * -(*p).height;
        point[2 as i32 as usize] =
            *org.offset(2 as i32 as isize) + ru_0[2 as i32 as usize] * -(*p).height;
        point[0 as i32 as usize] = point[0 as i32 as usize] + rr_0[0 as i32 as usize] * -(*p).width;
        point[1 as i32 as usize] = point[1 as i32 as usize] + rr_0[1 as i32 as usize] * -(*p).width;
        point[2 as i32 as usize] = point[2 as i32 as usize] + rr_0[2 as i32 as usize] * -(*p).width;
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] = 111 as i32 as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] = 19 as i32 as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] = 9 as i32 as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * alpha_0) as byte;
        point[0 as i32 as usize] =
            *org.offset(0 as i32 as isize) + ru_0[0 as i32 as usize] * -(*p).height;
        point[1 as i32 as usize] =
            *org.offset(1 as i32 as isize) + ru_0[1 as i32 as usize] * -(*p).height;
        point[2 as i32 as usize] =
            *org.offset(2 as i32 as isize) + ru_0[2 as i32 as usize] * -(*p).height;
        point[0 as i32 as usize] = point[0 as i32 as usize] + rr_0[0 as i32 as usize] * (*p).width;
        point[1 as i32 as usize] = point[1 as i32 as usize] + rr_0[1 as i32 as usize] * (*p).width;
        point[2 as i32 as usize] = point[2 as i32 as usize] + rr_0[2 as i32 as usize] * (*p).width;
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] = 111 as i32 as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] = 19 as i32 as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] = 9 as i32 as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * alpha_0) as byte;
        point[0 as i32 as usize] =
            *org.offset(0 as i32 as isize) + ru_0[0 as i32 as usize] * (*p).height;
        point[1 as i32 as usize] =
            *org.offset(1 as i32 as isize) + ru_0[1 as i32 as usize] * (*p).height;
        point[2 as i32 as usize] =
            *org.offset(2 as i32 as isize) + ru_0[2 as i32 as usize] * (*p).height;
        point[0 as i32 as usize] = point[0 as i32 as usize] + rr_0[0 as i32 as usize] * (*p).width;
        point[1 as i32 as usize] = point[1 as i32 as usize] + rr_0[1 as i32 as usize] * (*p).width;
        point[2 as i32 as usize] = point[2 as i32 as usize] + rr_0[2 as i32 as usize] * (*p).width;
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] = 111 as i32 as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] = 19 as i32 as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] = 9 as i32 as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] =
            (255 as i32 as f32 * alpha_0) as byte;
        point[0 as i32 as usize] =
            *org.offset(0 as i32 as isize) + ru_0[0 as i32 as usize] * (*p).height;
        point[1 as i32 as usize] =
            *org.offset(1 as i32 as isize) + ru_0[1 as i32 as usize] * (*p).height;
        point[2 as i32 as usize] =
            *org.offset(2 as i32 as isize) + ru_0[2 as i32 as usize] * (*p).height;
        point[0 as i32 as usize] = point[0 as i32 as usize] + rr_0[0 as i32 as usize] * -(*p).width;
        point[1 as i32 as usize] = point[1 as i32 as usize] + rr_0[1 as i32 as usize] * -(*p).width;
        point[2 as i32 as usize] = point[2 as i32 as usize] + rr_0[2 as i32 as usize] * -(*p).width;
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] = 111 as i32 as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] = 19 as i32 as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] = 9 as i32 as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] = (255 as i32 as f32 * alpha_0) as byte
    } else if (*p).type_0 == P_FLAT_SCALEUP as i32 {
        let mut sinR: f32 = 0.;
        let mut cosR: f32 = 0.;
        if (*p).color == 2 as i32 {
            color[0 as i32 as usize] = 1 as i32 as vec_t;
            color[1 as i32 as usize] = 1 as i32 as vec_t;
            color[2 as i32 as usize] = 1 as i32 as vec_t
        } else {
            color[0 as i32 as usize] = 0.5f64 as vec_t;
            color[1 as i32 as usize] = 0.5f64 as vec_t;
            color[2 as i32 as usize] = 0.5f64 as vec_t
        }
        time = cg.time as f32 - (*p).time;
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
        sinR = (height as f64
            * crate::stdlib::sin((*p).roll as f64 * 3.14159265358979323846f64 / 180.0f32 as f64)
            * crate::stdlib::sqrt(2 as i32 as f64)) as f32;
        cosR = (width as f64
            * crate::stdlib::cos((*p).roll as f64 * 3.14159265358979323846f64 / 180.0f32 as f64)
            * crate::stdlib::sqrt(2 as i32 as f64)) as f32;
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[0 as i32 as usize].xyz[0 as i32 as usize] -= sinR;
        verts[0 as i32 as usize].xyz[1 as i32 as usize] -= cosR;
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[1 as i32 as usize].xyz[0 as i32 as usize] -= cosR;
        verts[1 as i32 as usize].xyz[1 as i32 as usize] += sinR;
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[2 as i32 as usize].xyz[0 as i32 as usize] += sinR;
        verts[2 as i32 as usize].xyz[1 as i32 as usize] += cosR;
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[3 as i32 as usize].xyz[0 as i32 as usize] += cosR;
        verts[3 as i32 as usize].xyz[1 as i32 as usize] -= sinR;
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] =
            (255 as i32 as f32 * color[0 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] =
            (255 as i32 as f32 * color[1 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] =
            (255 as i32 as f32 * color[2 as i32 as usize]) as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte
    } else if (*p).type_0 == P_FLAT as i32 {
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[0 as i32 as usize].xyz[0 as i32 as usize] -= (*p).height;
        verts[0 as i32 as usize].xyz[1 as i32 as usize] -= (*p).width;
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[1 as i32 as usize].xyz[0 as i32 as usize] -= (*p).height;
        verts[1 as i32 as usize].xyz[1 as i32 as usize] += (*p).width;
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[2 as i32 as usize].xyz[0 as i32 as usize] += (*p).height;
        verts[2 as i32 as usize].xyz[1 as i32 as usize] += (*p).width;
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = *org.offset(0 as i32 as isize);
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = *org.offset(1 as i32 as isize);
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = *org.offset(2 as i32 as isize);
        verts[3 as i32 as usize].xyz[0 as i32 as usize] += (*p).height;
        verts[3 as i32 as usize].xyz[1 as i32 as usize] -= (*p).width;
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte
    } else if (*p).type_0 == P_ANIM as i32 {
        let mut rr_1: vec3_t = [0.; 3];
        let mut ru_1: vec3_t = [0.; 3];
        let mut rotate_ang_1: vec3_t = [0.; 3];
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        time = cg.time as f32 - (*p).time;
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
            (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
            org as *const vec_t,
        ) as f64)
            < width as f64 / 1.5f64
        {
            return;
        }
        i = (*p).shaderAnim;
        j = crate::stdlib::floor((ratio * shaderAnimCounts[(*p).shaderAnim as usize] as f32) as f64)
            as i32;
        (*p).pshader = shaderAnims[i as usize][j as usize];
        if (*p).roll != 0 {
            vectoangles(
                cg.refdef.viewaxis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
                rotate_ang_1.as_mut_ptr(),
            );
            rotate_ang_1[2 as i32 as usize] += (*p).roll as f32;
            AngleVectors(
                rotate_ang_1.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr_1.as_mut_ptr(),
                ru_1.as_mut_ptr(),
            );
        }
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + ru_1[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + ru_1[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + ru_1[2 as i32 as usize] * -height;
            point[0 as i32 as usize] = point[0 as i32 as usize] + rr_1[0 as i32 as usize] * -width;
            point[1 as i32 as usize] = point[1 as i32 as usize] + rr_1[1 as i32 as usize] * -width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + rr_1[2 as i32 as usize] * -width
        } else {
            point[0 as i32 as usize] =
                *org.offset(0 as i32 as isize) + vup[0 as i32 as usize] * -height;
            point[1 as i32 as usize] =
                *org.offset(1 as i32 as isize) + vup[1 as i32 as usize] * -height;
            point[2 as i32 as usize] =
                *org.offset(2 as i32 as isize) + vup[2 as i32 as usize] * -height;
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * -width;
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * -width;
            point[2 as i32 as usize] = point[2 as i32 as usize] + vright[2 as i32 as usize] * -width
        }
        verts[0 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[0 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[0 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[0 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[0 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[0 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + ru_1[0 as i32 as usize] * (2 as i32 as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + ru_1[1 as i32 as usize] * (2 as i32 as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + ru_1[2 as i32 as usize] * (2 as i32 as f32 * height)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vup[0 as i32 as usize] * (2 as i32 as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vup[1 as i32 as usize] * (2 as i32 as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vup[2 as i32 as usize] * (2 as i32 as f32 * height)
        }
        verts[1 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[1 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[1 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[1 as i32 as usize].st[0 as i32 as usize] = 0 as i32 as f32;
        verts[1 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[1 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[1 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + rr_1[0 as i32 as usize] * (2 as i32 as f32 * width);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + rr_1[1 as i32 as usize] * (2 as i32 as f32 * width);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + rr_1[2 as i32 as usize] * (2 as i32 as f32 * width)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vright[0 as i32 as usize] * (2 as i32 as f32 * width);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vright[1 as i32 as usize] * (2 as i32 as f32 * width);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vright[2 as i32 as usize] * (2 as i32 as f32 * width)
        }
        verts[2 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[2 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[2 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[2 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].st[1 as i32 as usize] = 1 as i32 as f32;
        verts[2 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[2 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte;
        if (*p).roll != 0 {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + ru_1[0 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + ru_1[1 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + ru_1[2 as i32 as usize] * (-(2 as i32) as f32 * height)
        } else {
            point[0 as i32 as usize] =
                point[0 as i32 as usize] + vup[0 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[1 as i32 as usize] =
                point[1 as i32 as usize] + vup[1 as i32 as usize] * (-(2 as i32) as f32 * height);
            point[2 as i32 as usize] =
                point[2 as i32 as usize] + vup[2 as i32 as usize] * (-(2 as i32) as f32 * height)
        }
        verts[3 as i32 as usize].xyz[0 as i32 as usize] = point[0 as i32 as usize];
        verts[3 as i32 as usize].xyz[1 as i32 as usize] = point[1 as i32 as usize];
        verts[3 as i32 as usize].xyz[2 as i32 as usize] = point[2 as i32 as usize];
        verts[3 as i32 as usize].st[0 as i32 as usize] = 1 as i32 as f32;
        verts[3 as i32 as usize].st[1 as i32 as usize] = 0 as i32 as f32;
        verts[3 as i32 as usize].modulate[0 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[1 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[2 as i32 as usize] = 255 as i32 as byte;
        verts[3 as i32 as usize].modulate[3 as i32 as usize] = 255 as i32 as byte
    }
    // done.
    if (*p).pshader == 0 {
        // (SA) temp commented out for DM
        //		CG_Printf ("CG_AddParticleToScene type %d p->pshader == ZERO\n", p->type);
        return;
    }
    if (*p).type_0 == P_WEATHER as i32
        || (*p).type_0 == P_WEATHER_TURBULENT as i32
        || (*p).type_0 == P_WEATHER_FLURRY as i32
    {
        trap_R_AddPolyToScene(
            (*p).pshader,
            3 as i32,
            TRIverts.as_mut_ptr() as *const polyVert_t,
        );
    } else {
        trap_R_AddPolyToScene(
            (*p).pshader,
            4 as i32,
            verts.as_mut_ptr() as *const polyVert_t,
        );
    };
}
// Ridah, made this static so it doesn't interfere with other files

static mut roll: f32 = 0.0f64 as f32;
/*
===============
CG_AddParticles
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticles() {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut alpha: f32 = 0.;
    let mut time: f32 = 0.;
    let mut time2: f32 = 0.;
    let mut org: vec3_t = [0.; 3];
    let mut active: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut tail: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut rotate_ang: vec3_t = [0.; 3];
    if initparticles as u64 == 0 {
        CG_ClearParticles();
    }
    vforward[0 as i32 as usize] = cg.refdef.viewaxis[0 as i32 as usize][0 as i32 as usize];
    vforward[1 as i32 as usize] = cg.refdef.viewaxis[0 as i32 as usize][1 as i32 as usize];
    vforward[2 as i32 as usize] = cg.refdef.viewaxis[0 as i32 as usize][2 as i32 as usize];
    vright[0 as i32 as usize] = cg.refdef.viewaxis[1 as i32 as usize][0 as i32 as usize];
    vright[1 as i32 as usize] = cg.refdef.viewaxis[1 as i32 as usize][1 as i32 as usize];
    vright[2 as i32 as usize] = cg.refdef.viewaxis[1 as i32 as usize][2 as i32 as usize];
    vup[0 as i32 as usize] = cg.refdef.viewaxis[2 as i32 as usize][0 as i32 as usize];
    vup[1 as i32 as usize] = cg.refdef.viewaxis[2 as i32 as usize][1 as i32 as usize];
    vup[2 as i32 as usize] = cg.refdef.viewaxis[2 as i32 as usize][2 as i32 as usize];
    vectoangles(
        cg.refdef.viewaxis[0 as i32 as usize].as_mut_ptr() as *const vec_t,
        rotate_ang.as_mut_ptr(),
    );
    roll = (roll as f64 + (cg.time as f32 - oldtime) as f64 * 0.1f64) as f32;
    rotate_ang[2 as i32 as usize] =
        (rotate_ang[2 as i32 as usize] as f64 + roll as f64 * 0.9f64) as vec_t;
    AngleVectors(
        rotate_ang.as_mut_ptr() as *const vec_t,
        rforward.as_mut_ptr(),
        rright.as_mut_ptr(),
        rup.as_mut_ptr(),
    );
    oldtime = cg.time as f32;
    active = 0 as *mut cparticle_t;
    tail = 0 as *mut cparticle_t;
    let mut current_block_54: u64;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        time = ((cg.time as f32 - (*p).time) as f64 * 0.001f64) as f32;
        alpha = (*p).alpha + time * (*p).alphavel;
        if alpha <= 0 as i32 as f32 {
            // faded out
            (*p).next = free_particles;
            free_particles = p;
            (*p).type_0 = 0 as i32;
            (*p).color = 0 as i32;
            (*p).alpha = 0 as i32 as f32
        } else {
            if (*p).type_0 == P_SMOKE as i32
                || (*p).type_0 == P_ANIM as i32
                || (*p).type_0 == P_BLEED as i32
                || (*p).type_0 == P_SMOKE_IMPACT as i32
            {
                if cg.time as f32 > (*p).endtime {
                    (*p).next = free_particles;
                    free_particles = p;
                    (*p).type_0 = 0 as i32;
                    (*p).color = 0 as i32;
                    (*p).alpha = 0 as i32 as f32;
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
                    if (*p).type_0 == P_WEATHER_FLURRY as i32 {
                        if cg.time as f32 > (*p).endtime {
                            (*p).next = free_particles;
                            free_particles = p;
                            (*p).type_0 = 0 as i32;
                            (*p).color = 0 as i32;
                            (*p).alpha = 0 as i32 as f32;
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
                            if (*p).type_0 == P_FLAT_SCALEUP_FADE as i32 {
                                if cg.time as f32 > (*p).endtime {
                                    (*p).next = free_particles;
                                    free_particles = p;
                                    (*p).type_0 = 0 as i32;
                                    (*p).color = 0 as i32;
                                    (*p).alpha = 0 as i32 as f32;
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
                                    if ((*p).type_0 == P_BAT as i32
                                        || (*p).type_0 == P_SPRITE as i32)
                                        && (*p).endtime < 0 as i32 as f32
                                    {
                                        // temporary sprite
                                        CG_AddParticleToScene(p, (*p).org.as_mut_ptr(), alpha);
                                        (*p).next = free_particles;
                                        free_particles = p;
                                        (*p).type_0 = 0 as i32;
                                        (*p).color = 0 as i32;
                                        (*p).alpha = 0 as i32 as f32
                                    } else {
                                        (*p).next = 0 as *mut particle_s;
                                        if tail.is_null() {
                                            tail = p;
                                            active = tail
                                        } else {
                                            (*tail).next = p;
                                            tail = p
                                        }
                                        if alpha as f64 > 1.0f64 {
                                            alpha = 1 as i32 as f32
                                        }
                                        time2 = time * time;
                                        org[0 as i32 as usize] = (*p).org[0 as i32 as usize]
                                            + (*p).vel[0 as i32 as usize] * time
                                            + (*p).accel[0 as i32 as usize] * time2;
                                        org[1 as i32 as usize] = (*p).org[1 as i32 as usize]
                                            + (*p).vel[1 as i32 as usize] * time
                                            + (*p).accel[1 as i32 as usize] * time2;
                                        org[2 as i32 as usize] = (*p).org[2 as i32 as usize]
                                            + (*p).vel[2 as i32 as usize] * time
                                            + (*p).accel[2 as i32 as usize] * time2;
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

pub unsafe extern "C" fn CG_ParticleSnowFlurry(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut turb: qboolean = qtrue;
    if pshader == 0 {
        CG_Printf(
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
    (*p).time = cg.time as f32;
    (*p).color = 0 as i32;
    (*p).alpha = 0.90f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).start = (*cent).currentState.origin2[0 as i32 as usize];
    (*p).end = (*cent).currentState.origin2[1 as i32 as usize];
    (*p).endtime = (cg.time + (*cent).currentState.time) as f32;
    (*p).startfade = (cg.time + (*cent).currentState.time2) as f32;
    (*p).pshader = pshader;
    if rand() % 100 as i32 > 90 as i32 {
        (*p).height = 32 as i32 as f32;
        (*p).width = 32 as i32 as f32;
        (*p).alpha = 0.10f32
    } else {
        (*p).height = 1 as i32 as f32;
        (*p).width = 1 as i32 as f32
    }
    (*p).vel[2 as i32 as usize] = -(20 as i32) as vec_t;
    (*p).type_0 = P_WEATHER_FLURRY as i32;
    if turb as u64 != 0 {
        (*p).vel[2 as i32 as usize] = -(10 as i32) as vec_t
    }
    (*p).org[0 as i32 as usize] = (*cent).currentState.origin[0 as i32 as usize];
    (*p).org[1 as i32 as usize] = (*cent).currentState.origin[1 as i32 as usize];
    (*p).org[2 as i32 as usize] = (*cent).currentState.origin[2 as i32 as usize];
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[0 as i32 as usize] = (*p).vel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).vel[0 as i32 as usize] = ((*p).vel[0 as i32 as usize] as f64
        + (((*cent).currentState.angles[0 as i32 as usize] * 32 as i32 as f32) as f64
            + 2.0f64
                * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 16 as i32 as f64)) as vec_t;
    (*p).vel[1 as i32 as usize] = ((*p).vel[1 as i32 as usize] as f64
        + (((*cent).currentState.angles[1 as i32 as usize] * 32 as i32 as f32) as f64
            + 2.0f64
                * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 16 as i32 as f64)) as vec_t;
    (*p).vel[2 as i32 as usize] += (*cent).currentState.angles[2 as i32 as usize];
    if turb as u64 != 0 {
        (*p).accel[0 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 16 as i32 as f64) as vec_t;
        (*p).accel[1 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 16 as i32 as f64) as vec_t
    };
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSnow(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut turb: i32,
    mut range: f32,
    mut snum: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).color = 0 as i32;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).start = *origin.offset(2 as i32 as isize);
    (*p).end = *origin2.offset(2 as i32 as isize);
    (*p).pshader = pshader;
    (*p).height = 1 as i32 as f32;
    (*p).width = 1 as i32 as f32;
    (*p).vel[2 as i32 as usize] = -(50 as i32) as vec_t;
    if turb != 0 {
        (*p).type_0 = P_WEATHER_TURBULENT as i32;
        (*p).vel[2 as i32 as usize] = (-(50 as i32) as f64 * 1.3f64) as vec_t
    } else {
        (*p).type_0 = P_WEATHER as i32
    }
    (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*p).org[0 as i32 as usize] = ((*p).org[0 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * range as f64) as vec_t;
    (*p).org[1 as i32 as usize] = ((*p).org[1 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * range as f64) as vec_t;
    (*p).org[2 as i32 as usize] = ((*p).org[2 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * ((*p).start - (*p).end) as f64) as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[0 as i32 as usize] = (*p).vel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    if turb != 0 {
        (*p).vel[0 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 16 as i32 as f64) as vec_t;
        (*p).vel[1 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 16 as i32 as f64) as vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBubble(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut turb: i32,
    mut range: f32,
    mut snum: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut randsize: f32 = 0.;
    if pshader == 0 {
        CG_Printf(b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).color = 0 as i32;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).start = *origin.offset(2 as i32 as isize);
    (*p).end = *origin2.offset(2 as i32 as isize);
    (*p).pshader = pshader;
    randsize = (1 as i32 as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 0.5f64) as f32;
    (*p).height = randsize;
    (*p).width = randsize;
    (*p).vel[2 as i32 as usize] = (50 as i32 as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 10 as i32 as f64) as vec_t;
    if turb != 0 {
        (*p).type_0 = P_BUBBLE_TURBULENT as i32;
        (*p).vel[2 as i32 as usize] = (50 as i32 as f64 * 1.3f64) as vec_t
    } else {
        (*p).type_0 = P_BUBBLE as i32
    }
    (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*p).org[0 as i32 as usize] = ((*p).org[0 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * range as f64) as vec_t;
    (*p).org[1 as i32 as usize] = ((*p).org[1 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * range as f64) as vec_t;
    (*p).org[2 as i32 as usize] = ((*p).org[2 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * ((*p).start - (*p).end) as f64) as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[0 as i32 as usize] = (*p).vel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    if turb != 0 {
        (*p).vel[0 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 4 as i32 as f64) as vec_t;
        (*p).vel[1 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 4 as i32 as f64) as vec_t
    }
    // Rafael snow pvs check
    (*p).snum = snum;
    (*p).link = qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSmoke(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    // using cent->density = enttime
    //		 cent->frame = startfade
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(b"CG_ParticleSmoke == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).endtime = (cg.time + (*cent).currentState.time) as f32;
    (*p).startfade = (cg.time + (*cent).currentState.time2) as f32;
    (*p).color = 0 as i32;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).start = (*cent).currentState.origin[2 as i32 as usize];
    (*p).end = (*cent).currentState.origin2[2 as i32 as usize];
    (*p).pshader = pshader;
    (*p).rotate = qfalse;
    (*p).height = 8 as i32 as f32;
    (*p).width = 8 as i32 as f32;
    (*p).endheight = 32 as i32 as f32;
    (*p).endwidth = 32 as i32 as f32;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0 as i32 as usize] = (*cent).currentState.origin[0 as i32 as usize];
    (*p).org[1 as i32 as usize] = (*cent).currentState.origin[1 as i32 as usize];
    (*p).org[2 as i32 as usize] = (*cent).currentState.origin[2 as i32 as usize];
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[0 as i32 as usize] = (*p).vel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).vel[2 as i32 as usize] = 5 as i32 as vec_t;
    if (*cent).currentState.frame == 1 as i32 {
        // reverse gravity
        (*p).vel[2 as i32 as usize] *= -(1 as i32) as f32
    }
    (*p).roll = (8 as i32 as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 4 as i32 as f64) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBulletDebris(
    mut org: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).endtime = (cg.time + duration) as f32;
    (*p).startfade = (cg.time + duration / 2 as i32) as f32;
    (*p).color = 3 as i32;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).height = 0.5f64 as f32;
    (*p).width = 0.5f64 as f32;
    (*p).endheight = 0.5f64 as f32;
    (*p).endwidth = 0.5f64 as f32;
    (*p).pshader = cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0 as i32 as usize] = *org.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *org.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *org.offset(2 as i32 as isize);
    (*p).vel[0 as i32 as usize] = *vel.offset(0 as i32 as isize);
    (*p).vel[1 as i32 as usize] = *vel.offset(1 as i32 as isize);
    (*p).vel[2 as i32 as usize] = *vel.offset(2 as i32 as isize);
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = -(60 as i32) as vec_t;
    (*p).vel[2 as i32 as usize] += -(20 as i32) as f32;
}
/*
======================
CG_ParticleExplosion
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleExplosion(
    mut animStr: *mut libc::c_char,
    mut origin: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: i32,
    mut sizeStart: i32,
    mut sizeEnd: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut anim: i32 = 0;
    if animStr < 10 as i32 as *mut libc::c_char {
        CG_Error(
            b"CG_ParticleExplosion: animStr is probably an index rather than a string\x00"
                as *const u8 as *const libc::c_char,
        );
    }
    // find the animation string
    anim = 0 as i32; // for sprites that are stretch in either direction
    while !shaderAnimNames[anim as usize].is_null() {
        if Q_stricmp(animStr, shaderAnimNames[anim as usize]) == 0 {
            break;
        }
        anim += 1
    }
    if shaderAnimNames[anim as usize].is_null() {
        CG_Error(
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
    (*p).time = cg.time as f32;
    (*p).alpha = 0.5f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    if duration < 0 as i32 {
        duration *= -(1 as i32);
        (*p).roll = 0 as i32
    } else {
        (*p).roll = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 179 as i32 as f64) as i32
    }
    (*p).shaderAnim = anim;
    (*p).width = sizeStart as f32;
    (*p).height = sizeStart as f32 * shaderAnimSTRatio[anim as usize];
    (*p).endheight = sizeEnd as f32;
    (*p).endwidth = sizeEnd as f32 * shaderAnimSTRatio[anim as usize];
    (*p).endtime = (cg.time + duration) as f32;
    (*p).type_0 = P_ANIM as i32;
    (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*p).vel[0 as i32 as usize] = *vel.offset(0 as i32 as isize);
    (*p).vel[1 as i32 as usize] = *vel.offset(1 as i32 as isize);
    (*p).vel[2 as i32 as usize] = *vel.offset(2 as i32 as isize);
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
}
// Rafael Shrapnel
#[no_mangle]

pub unsafe extern "C" fn CG_AddParticleShrapnel(mut _le: *mut localEntity_t) {}
// done.
#[no_mangle]

pub unsafe extern "C" fn CG_NewParticleArea(mut num: i32) -> i32 {
    // const char *str;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: i32 = 0;
    let mut origin: vec3_t = [0.; 3];
    let mut origin2: vec3_t = [0.; 3];
    let mut i: i32 = 0;
    let mut range: f32 = 0 as i32 as f32;
    let mut turb: i32 = 0;
    let mut numparticles: i32 = 0;
    let mut snum: i32 = 0;
    str = CG_ConfigString(num) as *mut libc::c_char;
    if *str.offset(0 as i32 as isize) == 0 {
        return 0 as i32;
    }
    // returns type 128 64 or 32
    token = COM_Parse(&mut str);
    type_0 = atoi(token);
    if type_0 == 1 as i32 {
        range = 128 as i32 as f32
    } else if type_0 == 2 as i32 {
        range = 64 as i32 as f32
    } else if type_0 == 3 as i32 {
        range = 32 as i32 as f32
    } else if type_0 == 0 as i32 {
        range = 256 as i32 as f32
    } else if type_0 == 4 as i32 {
        range = 8 as i32 as f32
    } else if type_0 == 5 as i32 {
        range = 16 as i32 as f32
    } else if type_0 == 6 as i32 {
        range = 32 as i32 as f32
    } else if type_0 == 7 as i32 {
        range = 64 as i32 as f32
    }
    i = 0 as i32;
    while i < 3 as i32 {
        token = COM_Parse(&mut str);
        origin[i as usize] = atof(token) as vec_t;
        i += 1
    }
    i = 0 as i32;
    while i < 3 as i32 {
        token = COM_Parse(&mut str);
        origin2[i as usize] = atof(token) as vec_t;
        i += 1
    }
    token = COM_Parse(&mut str);
    numparticles = atoi(token);
    token = COM_Parse(&mut str);
    turb = atoi(token);
    token = COM_Parse(&mut str);
    snum = atoi(token);
    i = 0 as i32;
    while i < numparticles {
        if type_0 >= 4 as i32 {
            CG_ParticleBubble(
                cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        } else {
            CG_ParticleSnow(
                cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        }
        i += 1
    }
    return 1 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_SnowLink(mut cent: *mut centity_t, mut particleOn: qboolean) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: i32 = 0;
    id = (*cent).currentState.frame;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_WEATHER as i32 || (*p).type_0 == P_WEATHER_TURBULENT as i32 {
            if (*p).snum == id {
                if particleOn as u64 != 0 {
                    (*p).link = qtrue
                } else {
                    (*p).link = qfalse
                }
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleImpactSmokePuff(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(
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
    (*p).time = cg.time as f32;
    (*p).alpha = 0.25f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = (2.0f64
        * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
        * 179 as i32 as f64) as i32;
    (*p).pshader = pshader;
    (*p).endtime = (cg.time + 1000 as i32) as f32;
    (*p).startfade = (cg.time + 100 as i32) as f32;
    (*p).width = (rand() % 4 as i32 + 8 as i32) as f32;
    (*p).height = (rand() % 4 as i32 + 8 as i32) as f32;
    (*p).endheight = (*p).height * 2 as i32 as f32;
    (*p).endwidth = (*p).width * 2 as i32 as f32;
    (*p).endtime = (cg.time + 500 as i32) as f32;
    (*p).type_0 = P_SMOKE_IMPACT as i32;
    (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*p).vel[0 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[2 as i32 as usize] = 20 as i32 as vec_t;
    (*p).accel[0 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[2 as i32 as usize] = 20 as i32 as vec_t;
    (*p).rotate = qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_Bleed(
    mut pshader: qhandle_t,
    mut start: *mut vec_t,
    mut _dir: *mut vec_t,
    mut fleshEntityNum: i32,
    mut duration: i32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(b"CG_Particle_Bleed pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = 0 as i32;
    (*p).pshader = pshader;
    (*p).endtime = (cg.time + duration) as f32;
    if fleshEntityNum != 0 {
        (*p).startfade = cg.time as f32
    } else {
        (*p).startfade = (cg.time + 100 as i32) as f32
    }
    (*p).width = 4 as i32 as f32;
    (*p).height = 4 as i32 as f32;
    (*p).endheight = (4 as i32 + rand() % 3 as i32) as f32;
    (*p).endwidth = (*p).endheight;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0 as i32 as usize] = *start.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *start.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *start.offset(2 as i32 as isize);
    (*p).vel[0 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[2 as i32 as usize] = -(20 as i32) as vec_t;
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179 as i32;
    (*p).color = 2 as i32;
    (*p).alpha = 0.75f64 as f32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilParticle(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut time: i32 = 0;
    let mut time2: i32 = 0;
    let mut ratio: f32 = 0.;
    let mut duration: f32 = 1500 as i32 as f32;
    time = cg.time;
    time2 = cg.time + (*cent).currentState.time;
    ratio = 1 as i32 as f32 - time as f32 / time2 as f32;
    if pshader == 0 {
        CG_Printf(b"CG_Particle_OilParticle == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = 0 as i32;
    (*p).pshader = pshader;
    (*p).endtime = cg.time as f32 + duration;
    (*p).startfade = (*p).endtime;
    (*p).width = 1 as i32 as f32;
    (*p).height = 3 as i32 as f32;
    (*p).endheight = 3 as i32 as f32;
    (*p).endwidth = 1 as i32 as f32;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0 as i32 as usize] = (*cent).currentState.origin[0 as i32 as usize];
    (*p).org[1 as i32 as usize] = (*cent).currentState.origin[1 as i32 as usize];
    (*p).org[2 as i32 as usize] = (*cent).currentState.origin[2 as i32 as usize];
    (*p).vel[0 as i32 as usize] =
        (*cent).currentState.origin2[0 as i32 as usize] * (16 as i32 as f32 * ratio);
    (*p).vel[1 as i32 as usize] =
        (*cent).currentState.origin2[1 as i32 as usize] * (16 as i32 as f32 * ratio);
    (*p).vel[2 as i32 as usize] = (*cent).currentState.origin2[2 as i32 as usize];
    (*p).snum = 1.0f32 as i32;
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).accel[2 as i32 as usize] = -(20 as i32) as vec_t;
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179 as i32;
    (*p).alpha = 0.75f64 as f32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_Particle_OilSlick(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(b"CG_Particle_OilSlick == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    if (*cent).currentState.angles2[2 as i32 as usize] != 0. {
        (*p).endtime = cg.time as f32 + (*cent).currentState.angles2[2 as i32 as usize]
    } else {
        (*p).endtime = (cg.time + 60000 as i32) as f32
    }
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = 0 as i32;
    (*p).pshader = pshader;
    if (*cent).currentState.angles2[0 as i32 as usize] != 0.
        || (*cent).currentState.angles2[1 as i32 as usize] != 0.
    {
        (*p).width = (*cent).currentState.angles2[0 as i32 as usize];
        (*p).height = (*cent).currentState.angles2[0 as i32 as usize];
        (*p).endheight = (*cent).currentState.angles2[1 as i32 as usize];
        (*p).endwidth = (*cent).currentState.angles2[1 as i32 as usize]
    } else {
        (*p).width = 8 as i32 as f32;
        (*p).height = 8 as i32 as f32;
        (*p).endheight = 16 as i32 as f32;
        (*p).endwidth = 16 as i32 as f32
    }
    (*p).type_0 = P_FLAT_SCALEUP as i32;
    (*p).snum = 1.0f64 as i32;
    (*p).org[0 as i32 as usize] = (*cent).currentState.origin[0 as i32 as usize];
    (*p).org[1 as i32 as usize] = (*cent).currentState.origin[1 as i32 as usize];
    (*p).org[2 as i32 as usize] = (*cent).currentState.origin[2 as i32 as usize];
    (*p).org[2 as i32 as usize] = ((*p).org[2 as i32 as usize] as f64
        + (0.55f64
            + 2.0f64
                * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 0.5f64)) as vec_t;
    (*p).vel[0 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179 as i32;
    (*p).alpha = 0.75f64 as f32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_OilSlickRemove(mut _cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: i32 = 0;
    id = 1.0f32 as i32;
    if id == 0 {
        CG_Printf(b"CG_OilSlickRevove NULL id\n\x00" as *const u8 as *const libc::c_char);
    }
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_FLAT_SCALEUP as i32 {
            if (*p).snum == id {
                (*p).endtime = (cg.time + 100 as i32) as f32;
                (*p).startfade = (*p).endtime;
                (*p).type_0 = P_FLAT_SCALEUP_FADE as i32
            }
        }
        p = next
    }
}
#[no_mangle]

pub unsafe extern "C" fn ValidBloodPool(mut start: *mut vec_t) -> qboolean {
    let mut angles: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut this_pos: vec3_t = [0.; 3];
    let mut x_pos: vec3_t = [0.; 3];
    let mut center_pos: vec3_t = [0.; 3];
    let mut end_pos: vec3_t = [0.; 3];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut fwidth: i32 = 0;
    let mut fheight: i32 = 0;
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
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
    let mut normal: vec3_t = [0.; 3];
    fwidth = 16 as i32;
    fheight = 16 as i32;
    normal[0 as i32 as usize] = 0 as i32 as vec_t;
    normal[1 as i32 as usize] = 0 as i32 as vec_t;
    normal[2 as i32 as usize] = 1 as i32 as vec_t;
    vectoangles(normal.as_mut_ptr() as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        0 as *mut vec_t,
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    center_pos[0 as i32 as usize] = (*start.offset(0 as i32 as isize) as f64
        + normal[0 as i32 as usize] as f64 * 0.5f64) as vec_t;
    center_pos[1 as i32 as usize] = (*start.offset(1 as i32 as isize) as f64
        + normal[1 as i32 as usize] as f64 * 0.5f64) as vec_t;
    center_pos[2 as i32 as usize] = (*start.offset(2 as i32 as isize) as f64
        + normal[2 as i32 as usize] as f64 * 0.5f64) as vec_t;
    x = -fwidth / 2 as i32;
    while x < fwidth {
        x_pos[0 as i32 as usize] =
            center_pos[0 as i32 as usize] + right[0 as i32 as usize] * x as f32;
        x_pos[1 as i32 as usize] =
            center_pos[1 as i32 as usize] + right[1 as i32 as usize] * x as f32;
        x_pos[2 as i32 as usize] =
            center_pos[2 as i32 as usize] + right[2 as i32 as usize] * x as f32;
        y = -fheight / 2 as i32;
        while y < fheight {
            this_pos[0 as i32 as usize] =
                x_pos[0 as i32 as usize] + up[0 as i32 as usize] * y as f32;
            this_pos[1 as i32 as usize] =
                x_pos[1 as i32 as usize] + up[1 as i32 as usize] * y as f32;
            this_pos[2 as i32 as usize] =
                x_pos[2 as i32 as usize] + up[2 as i32 as usize] * y as f32;
            end_pos[0 as i32 as usize] = (this_pos[0 as i32 as usize] as f64
                + normal[0 as i32 as usize] as f64 * (-0.5f64 * 2 as i32 as f64))
                as vec_t;
            end_pos[1 as i32 as usize] = (this_pos[1 as i32 as usize] as f64
                + normal[1 as i32 as usize] as f64 * (-0.5f64 * 2 as i32 as f64))
                as vec_t;
            end_pos[2 as i32 as usize] = (this_pos[2 as i32 as usize] as f64
                + normal[2 as i32 as usize] as f64 * (-0.5f64 * 2 as i32 as f64))
                as vec_t;
            CG_Trace(
                &mut trace as *mut _ as *mut trace_t,
                this_pos.as_mut_ptr() as *const vec_t,
                0 as *const vec_t,
                0 as *const vec_t,
                end_pos.as_mut_ptr() as *const vec_t,
                -(1 as i32),
                1 as i32,
            );
            if trace.entityNum < ((1 as i32) << 10 as i32) - 2 as i32 {
                // may only land on world
                return qfalse;
            }
            if !(trace.startsolid as u64 == 0 && trace.fraction < 1 as i32 as f32) {
                return qfalse;
            }
            y += fheight
        }
        x += fwidth
    }
    return qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn CG_BloodPool(
    mut _le: *mut localEntity_t,
    mut pshader: qhandle_t,
    mut tr: *mut trace_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut legit: qboolean = qfalse;
    let mut start: vec3_t = [0.; 3];
    let mut rndSize: f32 = 0.;
    if pshader == 0 {
        CG_Printf(b"CG_BloodPool pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    start[0 as i32 as usize] = (*tr).endpos[0 as i32 as usize];
    start[1 as i32 as usize] = (*tr).endpos[1 as i32 as usize];
    start[2 as i32 as usize] = (*tr).endpos[2 as i32 as usize];
    legit = ValidBloodPool(start.as_mut_ptr());
    if legit as u64 == 0 {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).endtime = (cg.time + 3000 as i32) as f32;
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = 0 as i32;
    (*p).pshader = pshader;
    rndSize =
        (0.4f64 + ((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 * 0.6f64) as f32;
    (*p).width = 8 as i32 as f32 * rndSize;
    (*p).height = 8 as i32 as f32 * rndSize;
    (*p).endheight = 16 as i32 as f32 * rndSize;
    (*p).endwidth = 16 as i32 as f32 * rndSize;
    (*p).type_0 = P_FLAT_SCALEUP as i32;
    (*p).org[0 as i32 as usize] = start[0 as i32 as usize];
    (*p).org[1 as i32 as usize] = start[1 as i32 as usize];
    (*p).org[2 as i32 as usize] = start[2 as i32 as usize];
    (*p).vel[0 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
    (*p).vel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179 as i32;
    (*p).alpha = 0.75f64 as f32;
    (*p).color = 2 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleBloodCloud(
    mut _cent: *mut centity_t,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
) {
    let mut length: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut crittersize: f32 = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: i32 = 0;
    dist = 0 as i32 as f32;
    length = VectorLength(dir as *const vec_t);
    vectoangles(dir as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    crittersize = 32 as i32 as f32;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1 as i32 as f32 {
        dist = 1 as i32 as f32
    }
    point[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    point[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    point[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    i = 0 as i32;
    while (i as f32) < dist {
        point[0 as i32 as usize] =
            point[0 as i32 as usize] + forward[0 as i32 as usize] * crittersize;
        point[1 as i32 as usize] =
            point[1 as i32 as usize] + forward[1 as i32 as usize] * crittersize;
        point[2 as i32 as usize] =
            point[2 as i32 as usize] + forward[2 as i32 as usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = cg.time as f32;
        (*p).alpha = 1.0f64 as f32;
        (*p).alphavel = 0 as i32 as f32;
        (*p).roll = 0 as i32;
        (*p).pshader = cgs.media.smokePuffShader;
        (*p).endtime = ((cg.time + 350 as i32) as f64
            + 2.0f64
                * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 100 as i32 as f64) as f32;
        (*p).startfade = cg.time as f32;
        (*p).width = 32 as i32 as f32;
        (*p).height = 32 as i32 as f32;
        (*p).endheight = 32 as i32 as f32;
        (*p).endwidth = 32 as i32 as f32;
        (*p).type_0 = P_SMOKE as i32;
        (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
        (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
        (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
        (*p).vel[0 as i32 as usize] = 0 as i32 as vec_t;
        (*p).vel[1 as i32 as usize] = 0 as i32 as vec_t;
        (*p).vel[2 as i32 as usize] = -(1 as i32) as vec_t;
        (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
        (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
        (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
        (*p).rotate = qfalse;
        (*p).roll = rand() % 179 as i32;
        (*p).color = 2 as i32;
        (*p).alpha = 0.75f64 as f32;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleSparks(
    mut org: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: i32,
    mut x: f32,
    mut y: f32,
    mut speed: f32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as f32;
    (*p).endtime = (cg.time + duration) as f32;
    (*p).startfade = (cg.time + duration / 2 as i32) as f32;
    (*p).color = 3 as i32;
    (*p).alpha = 0.4f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).height = 0.5f64 as f32;
    (*p).width = 0.5f64 as f32;
    (*p).endheight = 0.5f64 as f32;
    (*p).endwidth = 0.5f64 as f32;
    (*p).pshader = cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as i32;
    (*p).org[0 as i32 as usize] = *org.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *org.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *org.offset(2 as i32 as isize);
    (*p).org[0 as i32 as usize] = ((*p).org[0 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * x as f64) as vec_t;
    (*p).org[1 as i32 as usize] = ((*p).org[1 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * y as f64) as vec_t;
    (*p).vel[0 as i32 as usize] = *vel.offset(0 as i32 as isize);
    (*p).vel[1 as i32 as usize] = *vel.offset(1 as i32 as isize);
    (*p).vel[2 as i32 as usize] = *vel.offset(2 as i32 as isize);
    (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
    (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
    (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
    (*p).vel[0 as i32 as usize] = ((*p).vel[0 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 4 as i32 as f64) as vec_t;
    (*p).vel[1 as i32 as usize] = ((*p).vel[1 as i32 as usize] as f64
        + 2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 4 as i32 as f64) as vec_t;
    (*p).vel[2 as i32 as usize] = ((*p).vel[2 as i32 as usize] as f64
        + (20 as i32 as f64
            + 2.0f64
                * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                * 10 as i32 as f64)
            * speed as f64) as vec_t;
    (*p).accel[0 as i32 as usize] = (2.0f64
        * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
        * 4 as i32 as f64) as vec_t;
    (*p).accel[1 as i32 as usize] = (2.0f64
        * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
        * 4 as i32 as f64) as vec_t;
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleDust(
    mut _cent: *mut centity_t,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
) {
    let mut length: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut crittersize: f32 = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: i32 = 0;
    dist = 0 as i32 as f32;
    *dir.offset(0 as i32 as isize) = -*dir.offset(0 as i32 as isize);
    *dir.offset(1 as i32 as isize) = -*dir.offset(1 as i32 as isize);
    *dir.offset(2 as i32 as isize) = -*dir.offset(2 as i32 as isize);
    length = VectorLength(dir as *const vec_t);
    vectoangles(dir as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    crittersize = 32 as i32 as f32;
    if length != 0. {
        dist = length / crittersize
    }
    if dist < 1 as i32 as f32 {
        dist = 1 as i32 as f32
    }
    point[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    point[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    point[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    i = 0 as i32;
    while (i as f32) < dist {
        point[0 as i32 as usize] =
            point[0 as i32 as usize] + forward[0 as i32 as usize] * crittersize;
        point[1 as i32 as usize] =
            point[1 as i32 as usize] + forward[1 as i32 as usize] * crittersize;
        point[2 as i32 as usize] =
            point[2 as i32 as usize] + forward[2 as i32 as usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = cg.time as f32;
        (*p).alpha = 5.0f64 as f32;
        (*p).alphavel = 0 as i32 as f32;
        (*p).roll = 0 as i32;
        (*p).pshader = cgs.media.smokePuffShader;
        // RF, stay around for long enough to expand and dissipate naturally
        if length != 0. {
            (*p).endtime = ((cg.time + 4500 as i32) as f64
                + 2.0f64
                    * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                    * 3500 as i32 as f64) as f32
        } else {
            (*p).endtime = ((cg.time + 750 as i32) as f64
                + 2.0f64
                    * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
                    * 500 as i32 as f64) as f32
        }
        (*p).startfade = cg.time as f32;
        (*p).width = 32 as i32 as f32;
        (*p).height = 32 as i32 as f32;
        // RF, expand while falling
        (*p).endheight = (32 as i32 as f64 * 3.0f64) as f32;
        (*p).endwidth = (32 as i32 as f64 * 3.0f64) as f32;
        if length == 0. {
            (*p).width *= 0.2f32;
            (*p).height *= 0.2f32;
            (*p).endheight = 16 as i32 as f32;
            (*p).endwidth = 16 as i32 as f32
        }
        (*p).type_0 = P_SMOKE as i32;
        (*p).org[0 as i32 as usize] = point[0 as i32 as usize];
        (*p).org[1 as i32 as usize] = point[1 as i32 as usize];
        (*p).org[2 as i32 as usize] = point[2 as i32 as usize];
        (*p).vel[0 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 6 as i32 as f64) as vec_t;
        (*p).vel[1 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 6 as i32 as f64) as vec_t;
        (*p).vel[2 as i32 as usize] =
            (rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32 * 20 as i32 as f32;
        // RF, add some gravity/randomness
        (*p).accel[0 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 3 as i32 as f64) as vec_t;
        (*p).accel[1 as i32 as usize] = (2.0f64
            * (((rand() & 0x7fff as i32) as f32 / 0x7fff as i32 as f32) as f64 - 0.5f64)
            * 3 as i32 as f64) as vec_t;
        (*p).accel[2 as i32 as usize] = (-(40 as i32) as f64 * 0.4f64) as vec_t;
        (*p).accel[2 as i32 as usize] = 0 as i32 as vec_t;
        (*p).accel[1 as i32 as usize] = (*p).accel[2 as i32 as usize];
        (*p).accel[0 as i32 as usize] = (*p).accel[1 as i32 as usize];
        (*p).rotate = qfalse;
        (*p).roll = rand() % 179 as i32;
        (*p).alpha = 0.75f64 as f32;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn CG_ParticleMisc(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut size: i32,
    mut duration: i32,
    mut _alpha: f32,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if pshader == 0 {
        CG_Printf(
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
    (*p).time = cg.time as f32;
    (*p).alpha = 1.0f64 as f32;
    (*p).alphavel = 0 as i32 as f32;
    (*p).roll = rand() % 179 as i32;
    (*p).pshader = pshader;
    if duration > 0 as i32 {
        (*p).endtime = (cg.time + duration) as f32
    } else {
        (*p).endtime = duration as f32
    }
    (*p).startfade = cg.time as f32;
    (*p).width = size as f32;
    (*p).height = size as f32;
    (*p).endheight = size as f32;
    (*p).endwidth = size as f32;
    (*p).type_0 = P_SPRITE as i32;
    (*p).org[0 as i32 as usize] = *origin.offset(0 as i32 as isize);
    (*p).org[1 as i32 as usize] = *origin.offset(1 as i32 as isize);
    (*p).org[2 as i32 as usize] = *origin.offset(2 as i32 as isize);
    (*p).rotate = qfalse;
}

pub type bot_input_t = bot_input_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_input_s {
    pub thinktime: libc::c_float,
    pub dir: crate::src::qcommon::q_shared::vec3_t,
    pub speed: libc::c_float,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub actionflags: libc::c_int,
    pub weapon: libc::c_int,
}
//origin of the goal

//area number of the goal

//mins and maxs of the goal

//number of the goal entity

//goal number

//goal flags

//item information

//bsp_trace_t hit surface
pub type bsp_surface_t = bsp_surface_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_surface_s {
    pub name: [libc::c_char; 16],
    pub flags: libc::c_int,
    pub value: libc::c_int,
}
//remove the bsp_trace_s structure definition l8r on

//a trace is returned when a box is swept through the world
pub type bsp_trace_t = bsp_trace_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_trace_s {
    pub allsolid: crate::src::qcommon::q_shared::qboolean,
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: libc::c_float,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub exp_dist: libc::c_float,
    pub sidenum: libc::c_int,
    pub surface: bsp_surface_t,
    pub contents: libc::c_int,
    pub ent: libc::c_int,
}
//time since last output (in seconds)

//movement direction

//speed in the range [0, 400]

//the view angles

//one of the ACTION_? flags

//weapon to use

// BSPTRACE

//entity state
pub type bot_entitystate_t = bot_entitystate_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_entitystate_s {
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub old_origin: crate::src::qcommon::q_shared::vec3_t,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub groundent: libc::c_int,
    pub solid: libc::c_int,
    pub modelindex: libc::c_int,
    pub modelindex2: libc::c_int,
    pub frame: libc::c_int,
    pub event: libc::c_int,
    pub eventParm: libc::c_int,
    pub powerups: libc::c_int,
    pub weapon: libc::c_int,
    pub legsAnim: libc::c_int,
    pub torsoAnim: libc::c_int,
}

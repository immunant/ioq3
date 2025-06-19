pub type bot_input_t = bot_input_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_input_s {
    pub thinktime: f32,
    pub dir: crate::src::qcommon::q_shared::vec3_t,
    pub speed: f32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub actionflags: i32,
    pub weapon: i32,
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
    pub flags: i32,
    pub value: i32,
}
//remove the bsp_trace_s structure definition l8r on

//a trace is returned when a box is swept through the world
pub type bsp_trace_t = bsp_trace_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_trace_s {
    pub allsolid: crate::src::qcommon::q_shared::qboolean,
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: f32,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub exp_dist: f32,
    pub sidenum: i32,
    pub surface: bsp_surface_t,
    pub contents: i32,
    pub ent: i32,
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
    pub type_0: i32,
    pub flags: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub old_origin: crate::src::qcommon::q_shared::vec3_t,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub groundent: i32,
    pub solid: i32,
    pub modelindex: i32,
    pub modelindex2: i32,
    pub frame: i32,
    pub event: i32,
    pub eventParm: i32,
    pub powerups: i32,
    pub weapon: i32,
    pub legsAnim: i32,
    pub torsoAnim: i32,
}

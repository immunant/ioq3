#[repr(C)]
#[derive(Copy, Clone)]
pub struct pml_t {
    pub forward: crate::src::qcommon::q_shared::vec3_t,
    pub right: crate::src::qcommon::q_shared::vec3_t,
    pub up: crate::src::qcommon::q_shared::vec3_t,
    pub frametime: f32,
    pub msec: i32,
    pub walking: crate::src::qcommon::q_shared::qboolean,
    pub groundPlane: crate::src::qcommon::q_shared::qboolean,
    pub groundTrace: crate::src::qcommon::q_shared::trace_t,
    pub impactSpeed: f32,
    pub previous_origin: crate::src::qcommon::q_shared::vec3_t,
    pub previous_velocity: crate::src::qcommon::q_shared::vec3_t,
    pub previous_waterlevel: i32,
}

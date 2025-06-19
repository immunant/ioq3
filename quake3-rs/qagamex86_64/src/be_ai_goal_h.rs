pub type bot_goal_t = bot_goal_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_goal_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: libc::c_int,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: libc::c_int,
    pub number: libc::c_int,
    pub flags: libc::c_int,
    pub iteminfo: libc::c_int,
}

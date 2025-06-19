pub type bot_goal_t = bot_goal_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_goal_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: i32,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: i32,
    pub number: i32,
    pub flags: i32,
    pub iteminfo: i32,
}

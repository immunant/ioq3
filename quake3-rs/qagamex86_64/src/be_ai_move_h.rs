pub type bot_initmove_t = bot_initmove_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_initmove_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub viewoffset: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: i32,
    pub client: i32,
    pub thinktime: f32,
    pub presencetype: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub or_moveflags: i32,
}
//origin of the goal

//area number of the goal

//mins and maxs of the goal

//number of the goal entity

//goal number

//goal flags

//item information

//NOTE: the ideal_viewangles are only valid if MFL_MOVEMENTVIEW is set
pub type bot_moveresult_t = bot_moveresult_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_moveresult_s {
    pub failure: i32,
    pub type_0: i32,
    pub blocked: i32,
    pub blockentity: i32,
    pub traveltype: i32,
    pub flags: i32,
    pub weapon: i32,
    pub movedir: crate::src::qcommon::q_shared::vec3_t,
    pub ideal_viewangles: crate::src::qcommon::q_shared::vec3_t,
}

pub type bot_initmove_t = bot_initmove_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_initmove_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub viewoffset: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: libc::c_int,
    pub client: libc::c_int,
    pub thinktime: libc::c_float,
    pub presencetype: libc::c_int,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub or_moveflags: libc::c_int,
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
    pub failure: libc::c_int,
    pub type_0: libc::c_int,
    pub blocked: libc::c_int,
    pub blockentity: libc::c_int,
    pub traveltype: libc::c_int,
    pub flags: libc::c_int,
    pub weapon: libc::c_int,
    pub movedir: crate::src::qcommon::q_shared::vec3_t,
    pub ideal_viewangles: crate::src::qcommon::q_shared::vec3_t,
}

pub type projectileinfo_t = projectileinfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct projectileinfo_s {
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub flags: libc::c_int,
    pub gravity: libc::c_float,
    pub damage: libc::c_int,
    pub radius: libc::c_float,
    pub visdamage: libc::c_int,
    pub damagetype: libc::c_int,
    pub healthinc: libc::c_int,
    pub push: libc::c_float,
    pub detonation: libc::c_float,
    pub bounce: libc::c_float,
    pub bouncefric: libc::c_float,
    pub bouncestop: libc::c_float,
}
pub type weaponinfo_t = weaponinfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponinfo_s {
    pub valid: libc::c_int,
    pub number: libc::c_int,
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub level: libc::c_int,
    pub weaponindex: libc::c_int,
    pub flags: libc::c_int,
    pub projectile: [libc::c_char; 80],
    pub numprojectiles: libc::c_int,
    pub hspread: libc::c_float,
    pub vspread: libc::c_float,
    pub speed: libc::c_float,
    pub acceleration: libc::c_float,
    pub recoil: crate::src::qcommon::q_shared::vec3_t,
    pub offset: crate::src::qcommon::q_shared::vec3_t,
    pub angleoffset: crate::src::qcommon::q_shared::vec3_t,
    pub extrazvelocity: libc::c_float,
    pub ammoamount: libc::c_int,
    pub ammoindex: libc::c_int,
    pub activate: libc::c_float,
    pub reload: libc::c_float,
    pub spinup: libc::c_float,
    pub spindown: libc::c_float,
    pub proj: projectileinfo_t,
}

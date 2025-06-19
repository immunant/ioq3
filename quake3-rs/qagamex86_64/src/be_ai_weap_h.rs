pub type projectileinfo_t = projectileinfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct projectileinfo_s {
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub flags: i32,
    pub gravity: f32,
    pub damage: i32,
    pub radius: f32,
    pub visdamage: i32,
    pub damagetype: i32,
    pub healthinc: i32,
    pub push: f32,
    pub detonation: f32,
    pub bounce: f32,
    pub bouncefric: f32,
    pub bouncestop: f32,
}
pub type weaponinfo_t = weaponinfo_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct weaponinfo_s {
    pub valid: i32,
    pub number: i32,
    pub name: [libc::c_char; 80],
    pub model: [libc::c_char; 80],
    pub level: i32,
    pub weaponindex: i32,
    pub flags: i32,
    pub projectile: [libc::c_char; 80],
    pub numprojectiles: i32,
    pub hspread: f32,
    pub vspread: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub recoil: crate::src::qcommon::q_shared::vec3_t,
    pub offset: crate::src::qcommon::q_shared::vec3_t,
    pub angleoffset: crate::src::qcommon::q_shared::vec3_t,
    pub extrazvelocity: f32,
    pub ammoamount: i32,
    pub ammoindex: i32,
    pub activate: f32,
    pub reload: f32,
    pub spinup: f32,
    pub spindown: f32,
    pub proj: projectileinfo_t,
}

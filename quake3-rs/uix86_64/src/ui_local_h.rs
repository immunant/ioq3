pub type menuframework_s = _tag_menuframework;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tag_menuframework {
    pub cursor: i32,
    pub cursor_prev: i32,
    pub nitems: i32,
    pub items: [*mut libc::c_void; 64],
    pub draw: Option<unsafe extern "C" fn() -> ()>,
    pub key: Option<unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t>,
    pub wrapAround: crate::src::qcommon::q_shared::qboolean,
    pub fullscreen: crate::src::qcommon::q_shared::qboolean,
    pub showlogo: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menucommon_s {
    pub type_0: i32,
    pub name: *const libc::c_char,
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub parent: *mut menuframework_s,
    pub menuPosition: i32,
    pub flags: u32,
    pub callback: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ()>,
    pub statusbar: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub ownerdraw: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfield_t {
    pub cursor: i32,
    pub scroll: i32,
    pub widthInChars: i32,
    pub buffer: [libc::c_char; 256],
    pub maxchars: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menufield_s {
    pub generic: menucommon_s,
    pub field: mfield_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuslider_s {
    pub generic: menucommon_s,
    pub minvalue: f32,
    pub maxvalue: f32,
    pub curvalue: f32,
    pub range: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menulist_s {
    pub generic: menucommon_s,
    pub oldvalue: i32,
    pub curvalue: i32,
    pub numitems: i32,
    pub top: i32,
    pub itemnames: *mut *const libc::c_char,
    pub width: i32,
    pub height: i32,
    pub columns: i32,
    pub separation: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuaction_s {
    pub generic: menucommon_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menuradiobutton_s {
    pub generic: menucommon_s,
    pub curvalue: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menubitmap_s {
    pub generic: menucommon_s,
    pub focuspic: *mut libc::c_char,
    pub errorpic: *mut libc::c_char,
    pub shader: crate::src::qcommon::q_shared::qhandle_t,
    pub focusshader: crate::src::qcommon::q_shared::qhandle_t,
    pub width: i32,
    pub height: i32,
    pub focuscolor: *mut f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menutext_s {
    pub generic: menucommon_s,
    pub string: *mut libc::c_char,
    pub style: i32,
    pub color: *mut f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lerpFrame_t {
    pub oldFrame: i32,
    pub oldFrameTime: i32,
    pub frame: i32,
    pub frameTime: i32,
    pub backlerp: f32,
    pub yawAngle: f32,
    pub yawing: crate::src::qcommon::q_shared::qboolean,
    pub pitchAngle: f32,
    pub pitching: crate::src::qcommon::q_shared::qboolean,
    pub animationNumber: i32,
    pub animation: *mut crate::bg_public_h::animation_t,
    pub animationTime: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerInfo_t {
    pub legsModel: crate::src::qcommon::q_shared::qhandle_t,
    pub legsSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub legs: lerpFrame_t,
    pub torsoModel: crate::src::qcommon::q_shared::qhandle_t,
    pub torsoSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub torso: lerpFrame_t,
    pub headModel: crate::src::qcommon::q_shared::qhandle_t,
    pub headSkin: crate::src::qcommon::q_shared::qhandle_t,
    pub animations: [crate::bg_public_h::animation_t; 31],
    pub fixedlegs: crate::src::qcommon::q_shared::qboolean,
    pub fixedtorso: crate::src::qcommon::q_shared::qboolean,
    pub weaponModel: crate::src::qcommon::q_shared::qhandle_t,
    pub barrelModel: crate::src::qcommon::q_shared::qhandle_t,
    pub flashModel: crate::src::qcommon::q_shared::qhandle_t,
    pub flashDlightColor: crate::src::qcommon::q_shared::vec3_t,
    pub muzzleFlashTime: i32,
    pub color1: crate::src::qcommon::q_shared::vec3_t,
    pub c1RGBA: [crate::src::qcommon::q_shared::byte; 4],
    pub viewAngles: crate::src::qcommon::q_shared::vec3_t,
    pub moveAngles: crate::src::qcommon::q_shared::vec3_t,
    pub currentWeapon: crate::bg_public_h::weapon_t,
    pub legsAnim: i32,
    pub torsoAnim: i32,
    pub weapon: crate::bg_public_h::weapon_t,
    pub lastWeapon: crate::bg_public_h::weapon_t,
    pub pendingWeapon: crate::bg_public_h::weapon_t,
    pub weaponTimer: i32,
    pub pendingLegsAnim: i32,
    pub torsoAnimationTimer: i32,
    pub pendingTorsoAnim: i32,
    pub legsAnimationTimer: i32,
    pub chat: crate::src::qcommon::q_shared::qboolean,
    pub newModel: crate::src::qcommon::q_shared::qboolean,
    pub barrelSpinning: crate::src::qcommon::q_shared::qboolean,
    pub barrelAngle: f32,
    pub barrelTime: i32,
    pub realWeapon: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uiStatic_t {
    pub frametime: i32,
    pub realtime: i32,
    pub cursorx: i32,
    pub cursory: i32,
    pub menusp: i32,
    pub activemenu: *mut menuframework_s,
    pub stack: [*mut menuframework_s; 8],
    pub glconfig: crate::tr_types_h::glconfig_t,
    pub debug: crate::src::qcommon::q_shared::qboolean,
    pub whiteShader: crate::src::qcommon::q_shared::qhandle_t,
    pub menuBackShader: crate::src::qcommon::q_shared::qhandle_t,
    pub menuBackNoLogoShader: crate::src::qcommon::q_shared::qhandle_t,
    pub charset: crate::src::qcommon::q_shared::qhandle_t,
    pub charsetProp: crate::src::qcommon::q_shared::qhandle_t,
    pub charsetPropGlow: crate::src::qcommon::q_shared::qhandle_t,
    pub charsetPropB: crate::src::qcommon::q_shared::qhandle_t,
    pub cursor: crate::src::qcommon::q_shared::qhandle_t,
    pub rb_on: crate::src::qcommon::q_shared::qhandle_t,
    pub rb_off: crate::src::qcommon::q_shared::qhandle_t,
    pub xscale: f32,
    pub yscale: f32,
    pub bias: f32,
    pub demoversion: crate::src::qcommon::q_shared::qboolean,
    pub firstdraw: crate::src::qcommon::q_shared::qboolean,
}
pub const AWARD_ACCURACY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const AWARD_IMPRESSIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const AWARD_EXCELLENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const AWARD_GAUNTLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const AWARD_FRAGS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const AWARD_PERFECT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;

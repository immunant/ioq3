pub type menuframework_s = _tag_menuframework;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tag_menuframework {
    pub cursor: libc::c_int,
    pub cursor_prev: libc::c_int,
    pub nitems: libc::c_int,
    pub items: [*mut libc::c_void; 64],
    pub draw: Option<unsafe extern "C" fn() -> ()>,
    pub key:
        Option<unsafe extern "C" fn(_: libc::c_int) -> crate::src::qcommon::q_shared::sfxHandle_t>,
    pub wrapAround: crate::src::qcommon::q_shared::qboolean,
    pub fullscreen: crate::src::qcommon::q_shared::qboolean,
    pub showlogo: crate::src::qcommon::q_shared::qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menucommon_s {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub id: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
    pub parent: *mut menuframework_s,
    pub menuPosition: libc::c_int,
    pub flags: libc::c_uint,
    pub callback: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()>,
    pub statusbar: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub ownerdraw: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfield_t {
    pub cursor: libc::c_int,
    pub scroll: libc::c_int,
    pub widthInChars: libc::c_int,
    pub buffer: [libc::c_char; 256],
    pub maxchars: libc::c_int,
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
    pub minvalue: libc::c_float,
    pub maxvalue: libc::c_float,
    pub curvalue: libc::c_float,
    pub range: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menulist_s {
    pub generic: menucommon_s,
    pub oldvalue: libc::c_int,
    pub curvalue: libc::c_int,
    pub numitems: libc::c_int,
    pub top: libc::c_int,
    pub itemnames: *mut *const libc::c_char,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub columns: libc::c_int,
    pub separation: libc::c_int,
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
    pub curvalue: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menubitmap_s {
    pub generic: menucommon_s,
    pub focuspic: *mut libc::c_char,
    pub errorpic: *mut libc::c_char,
    pub shader: crate::src::qcommon::q_shared::qhandle_t,
    pub focusshader: crate::src::qcommon::q_shared::qhandle_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub focuscolor: *mut libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct menutext_s {
    pub generic: menucommon_s,
    pub string: *mut libc::c_char,
    pub style: libc::c_int,
    pub color: *mut libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lerpFrame_t {
    pub oldFrame: libc::c_int,
    pub oldFrameTime: libc::c_int,
    pub frame: libc::c_int,
    pub frameTime: libc::c_int,
    pub backlerp: libc::c_float,
    pub yawAngle: libc::c_float,
    pub yawing: crate::src::qcommon::q_shared::qboolean,
    pub pitchAngle: libc::c_float,
    pub pitching: crate::src::qcommon::q_shared::qboolean,
    pub animationNumber: libc::c_int,
    pub animation: *mut crate::bg_public_h::animation_t,
    pub animationTime: libc::c_int,
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
    pub muzzleFlashTime: libc::c_int,
    pub color1: crate::src::qcommon::q_shared::vec3_t,
    pub c1RGBA: [crate::src::qcommon::q_shared::byte; 4],
    pub viewAngles: crate::src::qcommon::q_shared::vec3_t,
    pub moveAngles: crate::src::qcommon::q_shared::vec3_t,
    pub currentWeapon: crate::bg_public_h::weapon_t,
    pub legsAnim: libc::c_int,
    pub torsoAnim: libc::c_int,
    pub weapon: crate::bg_public_h::weapon_t,
    pub lastWeapon: crate::bg_public_h::weapon_t,
    pub pendingWeapon: crate::bg_public_h::weapon_t,
    pub weaponTimer: libc::c_int,
    pub pendingLegsAnim: libc::c_int,
    pub torsoAnimationTimer: libc::c_int,
    pub pendingTorsoAnim: libc::c_int,
    pub legsAnimationTimer: libc::c_int,
    pub chat: crate::src::qcommon::q_shared::qboolean,
    pub newModel: crate::src::qcommon::q_shared::qboolean,
    pub barrelSpinning: crate::src::qcommon::q_shared::qboolean,
    pub barrelAngle: libc::c_float,
    pub barrelTime: libc::c_int,
    pub realWeapon: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uiStatic_t {
    pub frametime: libc::c_int,
    pub realtime: libc::c_int,
    pub cursorx: libc::c_int,
    pub cursory: libc::c_int,
    pub menusp: libc::c_int,
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
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub bias: libc::c_float,
    pub demoversion: crate::src::qcommon::q_shared::qboolean,
    pub firstdraw: crate::src::qcommon::q_shared::qboolean,
}
pub const AWARD_ACCURACY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const AWARD_IMPRESSIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const AWARD_EXCELLENT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const AWARD_GAUNTLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const AWARD_FRAGS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const AWARD_PERFECT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;

extern "C" {
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;

    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn acos(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn cos(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn sin(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn tan(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn sqrt(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn fabs(_: libc::c_double) -> libc::c_double;

    #[no_mangle]
    pub fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    pub fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    pub fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

    #[no_mangle]
    pub fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;

    #[no_mangle]
    pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
// =============== BEGIN ctype_h ================
pub const _ISupper: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 256;
pub const _ISlower: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 512;
pub const _ISalpha: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1024;
pub const _ISdigit: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2048;
pub const _ISxdigit: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4096;
pub const _ISspace: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8192;
pub const _ISprint: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16384;
pub const _ISgraph: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32768;
pub const _ISblank: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const _IScntrl: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const _ISpunct: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const _ISalnum: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
// ================ END ctype_h ================
// =============== BEGIN stdint_h ================
pub type intptr_t = libc::c_long;
// ================ END stdint_h ================
// =============== BEGIN types_h ================
pub type __int32_t = libc::c_int;

pub type __builtin_va_list = [__va_list_tag; 1];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

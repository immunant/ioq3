pub type backing_store_ptr = *mut backing_store_struct;
pub type backing_store_info = backing_store_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backing_store_struct {
    pub read_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: backing_store_ptr,
            _: *mut libc::c_void,
            _: libc::c_long,
            _: libc::c_long,
        ) -> (),
    >,
    pub write_backing_store: Option<
        unsafe extern "C" fn(
            _: crate::jpeglib_h::j_common_ptr,
            _: backing_store_ptr,
            _: *mut libc::c_void,
            _: libc::c_long,
            _: libc::c_long,
        ) -> (),
    >,
    pub close_backing_store:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: backing_store_ptr) -> ()>,
    pub temp_file: *mut crate::stdlib::FILE,
    pub temp_name: [libc::c_char; 64],
}

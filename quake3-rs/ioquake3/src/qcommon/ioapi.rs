// =============== BEGIN ioapi_h ================
pub type open_file_func = Option<
    unsafe extern "C" fn(
        _: crate::zconf_h::voidpf,
        _: *const libc::c_char,
        _: i32,
    ) -> crate::zconf_h::voidpf,
>;

pub type read_file_func = Option<
    unsafe extern "C" fn(
        _: crate::zconf_h::voidpf,
        _: crate::zconf_h::voidpf,
        _: *mut libc::c_void,
        _: crate::zconf_h::uLong,
    ) -> crate::zconf_h::uLong,
>;

pub type write_file_func = Option<
    unsafe extern "C" fn(
        _: crate::zconf_h::voidpf,
        _: crate::zconf_h::voidpf,
        _: *const libc::c_void,
        _: crate::zconf_h::uLong,
    ) -> crate::zconf_h::uLong,
>;

pub type tell_file_func = Option<
    unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> isize,
>;

pub type seek_file_func = Option<
    unsafe extern "C" fn(
        _: crate::zconf_h::voidpf,
        _: crate::zconf_h::voidpf,
        _: crate::zconf_h::uLong,
        _: i32,
    ) -> isize,
>;

pub type close_file_func =
    Option<unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> i32>;

pub type testerror_file_func =
    Option<unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> i32>;

pub type zlib_filefunc_def = crate::src::qcommon::ioapi::zlib_filefunc_def_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zlib_filefunc_def_s {
    pub zopen_file: crate::src::qcommon::ioapi::open_file_func,
    pub zread_file: crate::src::qcommon::ioapi::read_file_func,
    pub zwrite_file: crate::src::qcommon::ioapi::write_file_func,
    pub ztell_file: crate::src::qcommon::ioapi::tell_file_func,
    pub zseek_file: crate::src::qcommon::ioapi::seek_file_func,
    pub zclose_file: crate::src::qcommon::ioapi::close_file_func,
    pub zerror_file: crate::src::qcommon::ioapi::testerror_file_func,
    pub opaque: crate::zconf_h::voidpf,
}
use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::zconf_h::uLong;
pub use crate::zconf_h::voidpf;
/* ioapi.c -- IO base function header for compress/uncompress .zip
   files using zlib + zip or unzip API

   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant
*/
/* I've found an old Unix (a SunOS 4.1.3_U1) without all SEEK_* defined.... */
#[no_mangle]

pub unsafe extern "C" fn fopen_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut filename: *const libc::c_char,
    mut mode: i32,
) -> crate::zconf_h::voidpf {
    let mut file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut mode_fopen: *const libc::c_char = 0 as *const libc::c_char;
    if mode & 3 as i32 == 1 as i32 {
        mode_fopen = b"rb\x00" as *const u8 as *const libc::c_char
    } else if mode & 4 as i32 != 0 {
        mode_fopen = b"r+b\x00" as *const u8 as *const libc::c_char
    } else if mode & 8 as i32 != 0 {
        mode_fopen = b"wb\x00" as *const u8 as *const libc::c_char
    }
    if !filename.is_null() && !mode_fopen.is_null() {
        file = crate::stdlib::fopen(filename, mode_fopen)
    }
    return file as crate::zconf_h::voidpf;
}
#[no_mangle]

pub unsafe extern "C" fn fread_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
    mut buf: *mut libc::c_void,
    mut size: crate::zconf_h::uLong,
) -> crate::zconf_h::uLong {
    let mut ret: crate::zconf_h::uLong = 0;
    ret = crate::stdlib::fread(
        buf,
        1 as i32 as libc::c_ulong,
        size,
        stream as *mut crate::stdlib::FILE,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fwrite_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
    mut buf: *const libc::c_void,
    mut size: crate::zconf_h::uLong,
) -> crate::zconf_h::uLong {
    let mut ret: crate::zconf_h::uLong = 0;
    ret = crate::stdlib::fwrite(
        buf,
        1 as i32 as libc::c_ulong,
        size,
        stream as *mut crate::stdlib::FILE,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ftell_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
) -> isize {
    let mut ret: isize = 0;
    ret = crate::stdlib::ftell(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fseek_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
    mut offset: crate::zconf_h::uLong,
    mut origin: i32,
) -> isize {
    let mut fseek_origin: i32 = 0 as i32;
    let mut ret: isize = 0;
    match origin {
        1 => fseek_origin = 1 as i32,
        2 => fseek_origin = 2 as i32,
        0 => fseek_origin = 0 as i32,
        _ => return -(1 as i32) as isize,
    }
    ret = 0 as i32 as isize;
    crate::stdlib::fseek(
        stream as *mut crate::stdlib::FILE,
        offset as isize,
        fseek_origin,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fclose_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
) -> i32 {
    let mut ret: i32 = 0;
    ret = crate::stdlib::fclose(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ferror_file_func(
    mut _opaque: crate::zconf_h::voidpf,
    mut stream: crate::zconf_h::voidpf,
) -> i32 {
    let mut ret: i32 = 0;
    ret = crate::stdlib::ferror(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fill_fopen_filefunc(
    mut pzlib_filefunc_def: *mut crate::src::qcommon::ioapi::zlib_filefunc_def,
) {
    (*pzlib_filefunc_def).zopen_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::zconf_h::voidpf>,
        crate::src::qcommon::ioapi::open_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::zconf_h::voidpf,
            _: *const libc::c_char,
            _: i32,
        ) -> crate::zconf_h::voidpf,
        unsafe extern "C" fn() -> crate::zconf_h::voidpf,
    >(fopen_file_func)));
    (*pzlib_filefunc_def).zread_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::zconf_h::uLong>,
        crate::src::qcommon::ioapi::read_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::zconf_h::voidpf,
            _: crate::zconf_h::voidpf,
            _: *mut libc::c_void,
            _: crate::zconf_h::uLong,
        ) -> crate::zconf_h::uLong,
        unsafe extern "C" fn() -> crate::zconf_h::uLong,
    >(fread_file_func)));
    (*pzlib_filefunc_def).zwrite_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::zconf_h::uLong>,
        crate::src::qcommon::ioapi::write_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::zconf_h::voidpf,
            _: crate::zconf_h::voidpf,
            _: *const libc::c_void,
            _: crate::zconf_h::uLong,
        ) -> crate::zconf_h::uLong,
        unsafe extern "C" fn() -> crate::zconf_h::uLong,
    >(fwrite_file_func)));
    (*pzlib_filefunc_def).ztell_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> isize>,
        crate::src::qcommon::ioapi::tell_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> isize,
        unsafe extern "C" fn() -> isize,
    >(ftell_file_func)));
    (*pzlib_filefunc_def).zseek_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> isize>,
        crate::src::qcommon::ioapi::seek_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::zconf_h::voidpf,
            _: crate::zconf_h::voidpf,
            _: crate::zconf_h::uLong,
            _: i32,
        ) -> isize,
        unsafe extern "C" fn() -> isize,
    >(fseek_file_func)));
    (*pzlib_filefunc_def).zclose_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> i32>,
        crate::src::qcommon::ioapi::close_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> i32,
        unsafe extern "C" fn() -> i32,
    >(fclose_file_func)));
    (*pzlib_filefunc_def).zerror_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> i32>,
        crate::src::qcommon::ioapi::testerror_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> i32,
        unsafe extern "C" fn() -> i32,
    >(ferror_file_func)));
    (*pzlib_filefunc_def).opaque = 0 as *mut libc::c_void;
}

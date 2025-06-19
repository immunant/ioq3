use ::libc;
/*
The code in this file is in the public domain. The rest of ioquake3
is licensed under the GPLv2. Do not mingle code, please!
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_LaunchAutoupdater(
    mut _argc: i32,
    mut _argv: *mut *mut libc::c_char,
) {
    /* possibly unused. Pacify compilers. */
}

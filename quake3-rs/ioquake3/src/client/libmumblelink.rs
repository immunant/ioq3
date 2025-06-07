use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::wchar_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__suseconds_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__timezone_ptr_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::gettimeofday;
pub use crate::stdlib::int32_t;

pub use crate::stdlib::mmap;
pub use crate::stdlib::mode_t;
pub use crate::stdlib::munmap;

pub use crate::stdlib::timezone;
pub use crate::stdlib::uint32_t;

pub use ::libc::shm_open;
pub use ::libc::timeval;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LinkedMem {
    pub uiVersion: crate::stdlib::uint32_t,
    pub uiTick: crate::stdlib::uint32_t,
    pub fAvatarPosition: [libc::c_float; 3],
    pub fAvatarFront: [libc::c_float; 3],
    pub fAvatarTop: [libc::c_float; 3],
    pub name: [crate::stddef_h::wchar_t; 256],
    pub fCameraPosition: [libc::c_float; 3],
    pub fCameraFront: [libc::c_float; 3],
    pub fCameraTop: [libc::c_float; 3],
    pub identity: [crate::stddef_h::wchar_t; 256],
    pub context_len: crate::stdlib::uint32_t,
    pub context: [libc::c_uchar; 256],
    pub description: [crate::stddef_h::wchar_t; 2048],
}

static mut lm: *mut LinkedMem = 0 as *const LinkedMem as *mut LinkedMem;

unsafe extern "C" fn GetTickCount() -> crate::stdlib::int32_t {
    let mut tv: ::libc::timeval = ::libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    crate::stdlib::gettimeofday(&mut tv, 0 as *mut crate::stdlib::timezone);
    return (tv.tv_usec / 1000 as libc::c_int as libc::c_long
        + tv.tv_sec * 1000 as libc::c_int as libc::c_long) as crate::stdlib::int32_t;
}
/* libmumblelink.h -- mumble link interface

  Copyright (C) 2008 Ludwig Nussel <ludwig.nussel@suse.de>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.

*/
#[no_mangle]

pub unsafe extern "C" fn mumble_link(mut name: *const libc::c_char) -> libc::c_int {
    let mut file: [libc::c_char; 256] = [0; 256];
    let mut shmfd: libc::c_int = 0;
    if !lm.is_null() {
        return 0 as libc::c_int;
    }
    crate::stdlib::snprintf(
        file.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"/MumbleLink.%d\x00" as *const u8 as *const libc::c_char,
        ::libc::getuid(),
    );
    shmfd = ::libc::shm_open(
        file.as_mut_ptr(),
        0o2 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int) as crate::stdlib::mode_t,
    );
    if shmfd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    lm = crate::stdlib::mmap(
        0 as *mut libc::c_void,
        ::std::mem::size_of::<LinkedMem>() as libc::c_ulong,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x1 as libc::c_int,
        shmfd,
        0 as libc::c_int as crate::stdlib::__off_t,
    ) as *mut LinkedMem;
    if lm == -(1 as libc::c_int) as *mut libc::c_void as *mut LinkedMem {
        lm = 0 as *mut LinkedMem;
        ::libc::close(shmfd);
        return -(1 as libc::c_int);
    }
    ::libc::close(shmfd);
    crate::stdlib::memset(
        lm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<LinkedMem>() as libc::c_ulong,
    );
    crate::stdlib::mbstowcs(
        (*lm).name.as_mut_ptr(),
        name,
        (::std::mem::size_of::<[crate::stddef_h::wchar_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<crate::stddef_h::wchar_t>() as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn mumble_update_coordinates(
    mut fPosition: *mut libc::c_float,
    mut fFront: *mut libc::c_float,
    mut fTop: *mut libc::c_float,
) {
    mumble_update_coordinates2(fPosition, fFront, fTop, fPosition, fFront, fTop);
}
/* new for mumble 1.2: also set camera position */
#[no_mangle]

pub unsafe extern "C" fn mumble_update_coordinates2(
    mut fAvatarPosition: *mut libc::c_float,
    mut fAvatarFront: *mut libc::c_float,
    mut fAvatarTop: *mut libc::c_float,
    mut fCameraPosition: *mut libc::c_float,
    mut fCameraFront: *mut libc::c_float,
    mut fCameraTop: *mut libc::c_float,
) {
    if lm.is_null() {
        return;
    }
    crate::stdlib::memcpy(
        (*lm).fAvatarPosition.as_mut_ptr() as *mut libc::c_void,
        fAvatarPosition as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        (*lm).fAvatarFront.as_mut_ptr() as *mut libc::c_void,
        fAvatarFront as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        (*lm).fAvatarTop.as_mut_ptr() as *mut libc::c_void,
        fAvatarTop as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        (*lm).fCameraPosition.as_mut_ptr() as *mut libc::c_void,
        fCameraPosition as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        (*lm).fCameraFront.as_mut_ptr() as *mut libc::c_void,
        fCameraFront as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        (*lm).fCameraTop.as_mut_ptr() as *mut libc::c_void,
        fCameraTop as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 3]>() as libc::c_ulong,
    );
    (*lm).uiVersion = 2 as libc::c_int as crate::stdlib::uint32_t;
    (*lm).uiTick = GetTickCount() as crate::stdlib::uint32_t;
}
#[no_mangle]

pub unsafe extern "C" fn mumble_set_identity(mut identity: *const libc::c_char) {
    let mut len: crate::stddef_h::size_t = 0;
    if lm.is_null() {
        return;
    }
    len = if (::std::mem::size_of::<[crate::stddef_h::wchar_t; 256]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::stddef_h::wchar_t>() as libc::c_ulong)
        < crate::stdlib::strlen(identity).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        (::std::mem::size_of::<[crate::stddef_h::wchar_t; 256]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<crate::stddef_h::wchar_t>() as libc::c_ulong)
    } else {
        crate::stdlib::strlen(identity).wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
    crate::stdlib::mbstowcs((*lm).identity.as_mut_ptr(), identity, len);
}
#[no_mangle]

pub unsafe extern "C" fn mumble_set_context(
    mut context: *const libc::c_uchar,
    mut len: crate::stddef_h::size_t,
) {
    if lm.is_null() {
        return;
    }
    len = if (::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong) < len {
        ::std::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong
    } else {
        len
    };
    (*lm).context_len = len as crate::stdlib::uint32_t;
    crate::stdlib::memcpy(
        (*lm).context.as_mut_ptr() as *mut libc::c_void,
        context as *const libc::c_void,
        len,
    );
}
#[no_mangle]

pub unsafe extern "C" fn mumble_set_description(mut description: *const libc::c_char) {
    let mut len: crate::stddef_h::size_t = 0;
    if lm.is_null() {
        return;
    }
    len = if (::std::mem::size_of::<[crate::stddef_h::wchar_t; 2048]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::stddef_h::wchar_t>() as libc::c_ulong)
        < crate::stdlib::strlen(description).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        (::std::mem::size_of::<[crate::stddef_h::wchar_t; 2048]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<crate::stddef_h::wchar_t>() as libc::c_ulong)
    } else {
        crate::stdlib::strlen(description).wrapping_add(1 as libc::c_int as libc::c_ulong)
    };
    crate::stdlib::mbstowcs((*lm).description.as_mut_ptr(), description, len);
}
#[no_mangle]

pub unsafe extern "C" fn mumble_unlink() {
    if lm.is_null() {
        return;
    }
    crate::stdlib::munmap(
        lm as *mut libc::c_void,
        ::std::mem::size_of::<LinkedMem>() as libc::c_ulong,
    );
    lm = 0 as *mut LinkedMem;
}
#[no_mangle]

pub unsafe extern "C" fn mumble_islinked() -> libc::c_int {
    return (lm != 0 as *mut libc::c_void as *mut LinkedMem) as libc::c_int;
}

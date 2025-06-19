use ::libc;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

#[no_mangle]

pub static mut allocatedmemory: i32 = 0;
#[no_mangle]

pub static mut totalmemorysize: i32 = 0;
#[no_mangle]

pub static mut numblocks: i32 = 0;
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
/* ****************************************************************************
 * name:		l_memory.h
 *
 * desc:		memory management
 *
 * $Archive: /source/code/botlib/l_memory.h $
 *
 *****************************************************************************/
//#define MEMDEBUG
//allocate a memory block of the given size
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GetMemory(mut size: libc::c_ulong) -> *mut libc::c_void
//MEMDEBUG
{
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    ptr = crate::src::botlib::be_interface::botimport
        .GetMemory
        .expect("non-null function pointer")(
        size.wrapping_add(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as i32,
    );
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    memid = ptr as *mut libc::c_ulong;
    *memid = 0x12345678 as libc::c_long as libc::c_ulong;
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as isize)
        as *mut libc::c_ulong as *mut libc::c_void;
}
//allocate a memory block of the given size and clear it
//end of the function GetMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GetClearedMemory(mut size: libc::c_ulong) -> *mut libc::c_void
//MEMDEBUG
{
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = GetMemory(size);
    //MEMDEBUG
    crate::stdlib::memset(ptr, 0 as i32, size);
    return ptr;
}
//
//allocate a memory block of the given size
//end of the function GetClearedMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GetHunkMemory(mut size: libc::c_ulong) -> *mut libc::c_void
//MEMDEBUG
{
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    ptr = crate::src::botlib::be_interface::botimport
        .HunkAlloc
        .expect("non-null function pointer")(
        size.wrapping_add(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as i32,
    );
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    memid = ptr as *mut libc::c_ulong;
    *memid = 0x87654321 as libc::c_long as libc::c_ulong;
    return (ptr as *mut libc::c_char)
        .offset(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as isize)
        as *mut libc::c_ulong as *mut libc::c_void;
}
//allocate a memory block of the given size and clear it
//end of the function GetHunkMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GetClearedHunkMemory(mut size: libc::c_ulong) -> *mut libc::c_void
//MEMDEBUG
{
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = GetHunkMemory(size);
    //MEMDEBUG
    crate::stdlib::memset(ptr, 0 as i32, size);
    return ptr;
}
//free the given memory block
//end of the function GetClearedHunkMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeMemory(mut ptr: *mut libc::c_void) {
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    memid = (ptr as *mut libc::c_char)
        .offset(-(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong as isize))
        as *mut libc::c_ulong;
    if *memid == 0x12345678 as libc::c_long as libc::c_ulong {
        crate::src::botlib::be_interface::botimport
            .FreeMemory
            .expect("non-null function pointer")(memid as *mut libc::c_void);
    };
    //end if
}
//returns the amount available memory
//end of the function FreeMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AvailableMemory() -> i32 {
    return crate::src::botlib::be_interface::botimport
        .AvailableMemory
        .expect("non-null function pointer")();
}
//prints the total used memory size
//end of the function AvailableMemory
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn PrintUsedMemorySize() {}
//print all memory blocks with label
//end of the function PrintUsedMemorySize
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn PrintMemoryLabels() {}
//end of the function PrintMemoryLabels

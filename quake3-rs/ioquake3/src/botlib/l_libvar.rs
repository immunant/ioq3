// =============== BEGIN l_libvar_h ================

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
 * name:		l_libvar.h
 *
 * desc:		botlib vars
 *
 * $Archive: /source/code/botlib/l_libvar.h $
 *
 *****************************************************************************/

//library variable
pub type libvar_t = crate::src::botlib::l_libvar::libvar_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub flags: i32,
    pub modified: crate::src::qcommon::q_shared::qboolean,
    pub value: f32,
    pub next: *mut crate::src::botlib::l_libvar::libvar_s,
}
use ::libc;

pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Q_stricmp;

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
 * name:		l_libvar.c
 *
 * desc:		bot library variables
 *
 * $Archive: /MissionPack/code/botlib/l_libvar.c $
 *
 *****************************************************************************/
//list with library variables
#[no_mangle]

pub static mut libvarlist: *mut crate::src::botlib::l_libvar::libvar_t = 0
    as *const crate::src::botlib::l_libvar::libvar_t
    as *mut crate::src::botlib::l_libvar::libvar_t;
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarStringValue(mut string: *const libc::c_char) -> f32 {
    let mut dotfound: i32 = 0 as i32; //end while
    let mut value: f32 = 0 as i32 as f32; //end if
    while *string != 0 {
        if (*string as i32) < '0' as i32 || *string as i32 > '9' as i32 {
            if dotfound != 0 || *string as i32 != '.' as i32 {
                return 0 as i32 as f32;
            } else {
                dotfound = 10 as i32; //end if
                string = string.offset(1)
            }
            //end if
        } //end else
        if dotfound != 0 {
            value = value + (*string as i32 - '0' as i32) as f32 / dotfound as f32; //end if
            dotfound *= 10 as i32
        } else {
            value = (value as f64 * 10.0f64 + (*string as i32 - '0' as i32) as f32 as f64) as f32
        }
        string = string.offset(1)
    }
    return value;
}
//end of the function LibVarStringValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarAlloc(
    mut var_name: *const libc::c_char,
) -> *mut crate::src::botlib::l_libvar::libvar_t {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t;
    v = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_libvar::libvar_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_libvar::libvar_t;
    crate::stdlib::memset(
        v as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::botlib::l_libvar::libvar_t>() as libc::c_ulong,
    );
    (*v).name = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen(var_name).wrapping_add(1 as i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    libc::strcpy((*v).name, var_name);
    //add the variable in the list
    (*v).next = libvarlist;
    libvarlist = v;
    return v;
}
//end of the function LibVarAlloc
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarDeAlloc(mut v: *mut crate::src::botlib::l_libvar::libvar_t) {
    if !(*v).string.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*v).string as *mut libc::c_void);
    }
    crate::src::botlib::l_memory::FreeMemory((*v).name as *mut libc::c_void);
    crate::src::botlib::l_memory::FreeMemory(v as *mut libc::c_void);
}
//removes all library variables
//end of the function LibVarDeAlloc
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarDeAllocAll() {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end for
    v = libvarlist;
    while !v.is_null() {
        libvarlist = (*libvarlist).next;
        LibVarDeAlloc(v);
        v = libvarlist
    }
    libvarlist = 0 as *mut crate::src::botlib::l_libvar::libvar_t;
}
//gets the library variable with the given name
//end of the function LibVarDeAllocAll
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarGet(
    mut var_name: *const libc::c_char,
) -> *mut crate::src::botlib::l_libvar::libvar_t {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end for
    v = libvarlist;
    while !v.is_null() {
        if Q_stricmp((*v).name, var_name) == 0 {
            return v;
        }
        v = (*v).next
        //end if
    }
    return 0 as *mut crate::src::botlib::l_libvar::libvar_t;
}
//gets the string of the library variable with the given name
//end of the function LibVarGet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarGetString(mut var_name: *const libc::c_char) -> *mut libc::c_char {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end if
    v = LibVarGet(var_name);
    if !v.is_null() {
        return (*v).string;
    } else {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    };
    //end else
}
//gets the value of the library variable with the given name
//end of the function LibVarGetString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarGetValue(mut var_name: *const libc::c_char) -> f32 {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end if
    v = LibVarGet(var_name);
    if !v.is_null() {
        return (*v).value;
    } else {
        return 0 as i32 as f32;
    };
    //end else
}
//creates the library variable if not existing already and returns it
//end of the function LibVarGetValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVar(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> *mut crate::src::botlib::l_libvar::libvar_t {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() {
        return v;
    }
    //create new variable
    v = LibVarAlloc(var_name);
    //variable string
    (*v).string = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen(value).wrapping_add(1 as i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    libc::strcpy((*v).string, value);
    //the value
    (*v).value = LibVarStringValue((*v).string);
    //variable is modified
    (*v).modified = qtrue;
    //
    return v;
}
//creates the library variable if not existing already and returns the value string
//end of the function LibVar
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarString(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> *mut libc::c_char {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t;
    v = LibVar(var_name, value);
    return (*v).string;
}
//creates the library variable if not existing already and returns the value
//end of the function LibVarString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarValue(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> f32 {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t;
    v = LibVar(var_name, value);
    return (*v).value;
}
//sets the library variable
//end of the function LibVarValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarSet(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end else
    v = LibVarGet(var_name); //end if
    if !v.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*v).string as *mut libc::c_void);
    } else {
        v = LibVarAlloc(var_name)
    }
    //variable string
    (*v).string = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen(value).wrapping_add(1 as i32 as libc::c_ulong),
    ) as *mut libc::c_char;
    libc::strcpy((*v).string, value);
    //the value
    (*v).value = LibVarStringValue((*v).string);
    //variable is modified
    (*v).modified = qtrue;
}
//returns true if the library variable has been modified
//end of the function LibVarSet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarChanged(mut var_name: *const libc::c_char) -> qboolean {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t; //end if
    v = LibVarGet(var_name);
    if !v.is_null() {
        return (*v).modified;
    } else {
        return qfalse;
    };
    //end else
}
//sets the library variable to unmodified
//end of the function LibVarChanged
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn LibVarSetNotModified(mut var_name: *const libc::c_char) {
    let mut v: *mut crate::src::botlib::l_libvar::libvar_t =
        0 as *mut crate::src::botlib::l_libvar::libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() {
        (*v).modified = qfalse
    };
    //end if
}
//end of the function LibVarSetNotModified

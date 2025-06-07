// =============== BEGIN l_struct_h ================
pub type fielddef_t = crate::src::botlib::l_struct::fielddef_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fielddef_s {
    pub name: *mut libc::c_char,
    pub offset: libc::c_int,
    pub type_0: libc::c_int,
    pub maxarray: libc::c_int,
    pub floatmin: libc::c_float,
    pub floatmax: libc::c_float,
    pub substruct: *mut crate::src::botlib::l_struct::structdef_s,
}

pub type structdef_t = crate::src::botlib::l_struct::structdef_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct structdef_s {
    pub size: libc::c_int,
    pub fields: *mut crate::src::botlib::l_struct::fielddef_t,
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

pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::PC_CheckTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectAnyToken;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenType;
pub use crate::src::botlib::l_precomp::PC_UnreadLastToken;
pub use crate::src::botlib::l_precomp::SourceError;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
pub use crate::src::botlib::l_script::StripSingleQuotes;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::Com_sprintf;

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
 * name:		l_struct.c
 *
 * desc:		structure reading / writing
 *
 * $Archive: /MissionPack/CODE/botlib/l_struct.c $
 *
 *****************************************************************************/
//BOTLIB
//BSPC
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FindField(
    mut defs: *mut crate::src::botlib::l_struct::fielddef_t,
    mut name: *mut libc::c_char,
) -> *mut crate::src::botlib::l_struct::fielddef_t {
    let mut i: libc::c_int = 0; //end for
    i = 0 as libc::c_int;
    while !(*defs.offset(i as isize)).name.is_null() {
        if ::libc::strcmp((*defs.offset(i as isize)).name, name) == 0 {
            return &mut *defs.offset(i as isize) as *mut crate::src::botlib::l_struct::fielddef_t;
        }
        i += 1
    }
    return 0 as *mut crate::src::botlib::l_struct::fielddef_t;
}
//end of the function FindField
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadNumber(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut negative: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    let mut intval: libc::c_long = 0;
    let mut intmin: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut intmax: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut floatval: libc::c_double = 0.;
    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
        source as *mut crate::src::botlib::l_precomp::source_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    //check for minus sign
    if token.type_0 == 5 as libc::c_int {
        //end if
        if (*fd).type_0 & 0x400 as libc::c_int != 0 {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"expected unsigned value, found %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            return crate::src::qcommon::q_shared::qfalse;
        }
        //if not a minus sign
        if ::libc::strcmp(
            token.string.as_mut_ptr(),
            b"-\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"unexpected punctuation %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            return crate::src::qcommon::q_shared::qfalse;
        }
        negative = crate::src::qcommon::q_shared::qtrue as libc::c_int;
        //read the number
        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    //check if it is a number
    if token.type_0 != 3 as libc::c_int {
        crate::src::botlib::l_precomp::SourceError(
            source as *mut crate::src::botlib::l_precomp::source_s,
            b"expected number, found %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token.string.as_mut_ptr(),
        ); //end if
        return crate::src::qcommon::q_shared::qfalse;
    }
    //check for a float value
    if token.subtype & 0x800 as libc::c_int != 0 {
        //end if
        if (*fd).type_0 & 0xff as libc::c_int != 3 as libc::c_int {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"unexpected float\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end if
            return crate::src::qcommon::q_shared::qfalse;
        } //end if
        floatval = token.floatvalue as libc::c_double;
        if negative != 0 {
            floatval = -floatval
        }
        if (*fd).type_0 & 0x200 as libc::c_int != 0 {
            if floatval < (*fd).floatmin as libc::c_double
                || floatval > (*fd).floatmax as libc::c_double
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"float out of range [%f, %f]\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*fd).floatmin as libc::c_double,
                    (*fd).floatmax as libc::c_double,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            //end if
        }
        *(p as *mut libc::c_float) = floatval as libc::c_float;
        return crate::src::qcommon::q_shared::qtrue;
    }
    //
    intval = token.intvalue as libc::c_long;
    if negative != 0 {
        intval = -intval
    }
    //check bounds
    if (*fd).type_0 & 0xff as libc::c_int == 1 as libc::c_int {
        if (*fd).type_0 & 0x400 as libc::c_int != 0 {
            intmin = 0 as libc::c_int as libc::c_long; //end if
            intmax = 255 as libc::c_int as libc::c_long
        } else {
            intmin = -(128 as libc::c_int) as libc::c_long; //end else if
            intmax = 127 as libc::c_int as libc::c_long
        }
    } //end else if
    if (*fd).type_0 & 0xff as libc::c_int == 2 as libc::c_int {
        if (*fd).type_0 & 0x400 as libc::c_int != 0 {
            intmin = 0 as libc::c_int as libc::c_long; //end if
            intmax = 65535 as libc::c_int as libc::c_long
        } else {
            intmin = -(32768 as libc::c_int) as libc::c_long;
            intmax = 32767 as libc::c_int as libc::c_long
        }
    }
    if (*fd).type_0 & 0xff as libc::c_int == 1 as libc::c_int
        || (*fd).type_0 & 0xff as libc::c_int == 2 as libc::c_int
    {
        if (*fd).type_0 & 0x200 as libc::c_int != 0 {
            intmin = if intmin as libc::c_float > (*fd).floatmin {
                intmin as libc::c_float
            } else {
                (*fd).floatmin
            } as libc::c_long;
            intmax = if (intmax as libc::c_float) < (*fd).floatmax {
                intmax as libc::c_float
            } else {
                (*fd).floatmax
            } as libc::c_long
        }
        if intval < intmin || intval > intmax {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"value %ld out of range [%ld, %ld]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                intval,
                intmin,
                intmax,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    //end if
    } else if (*fd).type_0 & 0xff as libc::c_int == 3 as libc::c_int {
        if (*fd).type_0 & 0x200 as libc::c_int != 0 {
            if (intval as libc::c_float) < (*fd).floatmin
                || intval as libc::c_float > (*fd).floatmax
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"value %ld out of range [%f, %f]\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    intval,
                    (*fd).floatmin as libc::c_double,
                    (*fd).floatmax as libc::c_double,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
            //end if
        }
        //end if
    }
    //store the value
    if (*fd).type_0 & 0xff as libc::c_int == 1 as libc::c_int {
        //end else
        if (*fd).type_0 & 0x400 as libc::c_int != 0 {
            *(p as *mut libc::c_uchar) = intval as libc::c_uchar
        } else {
            *(p as *mut libc::c_char) = intval as libc::c_char
        }
    } else if (*fd).type_0 & 0xff as libc::c_int == 2 as libc::c_int {
        //end if
        if (*fd).type_0 & 0x400 as libc::c_int != 0 {
            *(p as *mut libc::c_uint) = intval as libc::c_uint
        } else {
            *(p as *mut libc::c_int) = intval as libc::c_int
        }
    } else if (*fd).type_0 & 0xff as libc::c_int == 3 as libc::c_int {
        *(p as *mut libc::c_float) = intval as libc::c_float
    } //end else
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function ReadNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadChar(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
        source as *mut crate::src::botlib::l_precomp::source_s,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    //take literals into account
    if token.type_0 == 2 as libc::c_int {
        //end if
        crate::src::botlib::l_script::StripSingleQuotes(token.string.as_mut_ptr()); //end if
        *(p as *mut libc::c_char) = token.string[0 as libc::c_int as usize]
    } else {
        crate::src::botlib::l_precomp::PC_UnreadLastToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
        );
        if ReadNumber(source, fd, p) as u64 == 0 {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function ReadChar
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut _fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if crate::src::botlib::l_precomp::PC_ExpectTokenType(
        source as *mut crate::src::botlib::l_precomp::source_s,
        1 as libc::c_int,
        0 as libc::c_int,
        &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    //remove the double quotes
    crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
    //copy the string
    crate::stdlib::strncpy(
        p as *mut libc::c_char,
        token.string.as_mut_ptr(),
        (80 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    //make sure the string is closed with a zero
    *(p as *mut libc::c_char).offset((80 as libc::c_int - 1 as libc::c_int) as isize) =
        '\u{0}' as i32 as libc::c_char;
    //
    return 1 as libc::c_int;
}
//read a structure from a script
//end of the function ReadString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadStructure(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end while
    let mut fd: *mut crate::src::botlib::l_struct::fielddef_t =
        0 as *mut crate::src::botlib::l_struct::fielddef_t;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut num: libc::c_int = 0;
    if crate::src::botlib::l_precomp::PC_ExpectTokenString(
        source as *mut crate::src::botlib::l_precomp::source_s,
        b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
            source as *mut crate::src::botlib::l_precomp::source_s,
            &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //if end of structure
        if ::libc::strcmp(
            token.string.as_mut_ptr(),
            b"}\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            break;
        }
        //find the field with the name
        fd = FindField((*def).fields, token.string.as_mut_ptr()); //end if
        if fd.is_null() {
            crate::src::botlib::l_precomp::SourceError(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"unknown structure field %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end else
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        } //end if
        if (*fd).type_0 & 0x100 as libc::c_int != 0 {
            num = (*fd).maxarray; //end if
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source as *mut crate::src::botlib::l_precomp::source_s,
                b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        } else {
            num = 1 as libc::c_int
        } //end switch
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        loop {
            let fresh0 = num;
            num = num - 1;
            if !(fresh0 > 0 as libc::c_int) {
                break;
            }
            if (*fd).type_0 & 0x100 as libc::c_int != 0 {
                if crate::src::botlib::l_precomp::PC_CheckTokenString(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    break;
                }
            }
            match (*fd).type_0 & 0xff as libc::c_int {
                1 => {
                    if ReadChar(source, fd, p) as u64 == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_char>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                    //end case
                }
                2 => {
                    if ReadNumber(source, fd, p) as u64 == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                }
                3 => {
                    if ReadNumber(source, fd, p) as u64 == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_float>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                }
                4 => {
                    if ReadString(source, fd, p) == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char).offset(80 as libc::c_int as isize)
                        as *mut libc::c_void
                }
                6 => {
                    if (*fd).substruct.is_null() {
                        crate::src::botlib::l_precomp::SourceError(
                            source as *mut crate::src::botlib::l_precomp::source_s,
                            b"BUG: no sub structure defined\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    ReadStructure(source, (*fd).substruct, p as *mut libc::c_char);
                    p = (p as *mut libc::c_char).offset((*(*fd).substruct).size as isize)
                        as *mut libc::c_void
                }
                _ => {}
            }
            if !((*fd).type_0 & 0x100 as libc::c_int != 0) {
                continue;
            }
            if crate::src::botlib::l_precomp::PC_ExpectAnyToken(
                source as *mut crate::src::botlib::l_precomp::source_s,
                &mut token as *mut _ as *mut crate::src::botlib::l_script::token_s,
            ) == 0
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"}\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                break;
            }
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b",\x00" as *const u8 as *const libc::c_char,
            ) != 0
            {
                crate::src::botlib::l_precomp::SourceError(
                    source as *mut crate::src::botlib::l_precomp::source_s,
                    b"expected a comma, found %s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    token.string.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            //end if
            //end if
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//writes indents
//end of the function ReadStructure
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn WriteIndent(
    mut fp: *mut crate::stdlib::FILE,
    mut indent: libc::c_int,
) -> libc::c_int {
    loop {
        let fresh1 = indent; //end while
        indent = indent - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        if crate::stdlib::fprintf(fp, b"\t\x00" as *const u8 as *const libc::c_char)
            < 0 as libc::c_int
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//writes a float without traling zeros
//end of the function WriteIndent
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn WriteFloat(
    mut fp: *mut crate::stdlib::FILE,
    mut value: libc::c_float,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut l: libc::c_int = 0;
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%f\x00" as *const u8 as *const libc::c_char,
        value as libc::c_double,
    );
    l = crate::stdlib::strlen(buf.as_mut_ptr()) as libc::c_int;
    loop
    //strip any trailing zeros
    {
        let fresh2 = l; //end while
        l = l - 1; //end if
        if !(fresh2 > 1 as libc::c_int) {
            break;
        }
        if buf[l as usize] as libc::c_int != '0' as i32
            && buf[l as usize] as libc::c_int != '.' as i32
        {
            break;
        }
        if buf[l as usize] as libc::c_int == '.' as i32 {
            buf[l as usize] = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            buf[l as usize] = 0 as libc::c_int as libc::c_char
        }
    }
    //write the float to file
    if crate::stdlib::fprintf(
        fp,
        b"%s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    ) < 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
//end of the function WriteFloat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn WriteStructWithIndent(
    mut fp: *mut crate::stdlib::FILE,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
    mut indent: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0; //end for
    let mut num: libc::c_int = 0; //end else
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void; //end if
    let mut fd: *mut crate::src::botlib::l_struct::fielddef_t =
        0 as *mut crate::src::botlib::l_struct::fielddef_t;
    if WriteIndent(fp, indent) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if crate::stdlib::fprintf(fp, b"{\r\n\x00" as *const u8 as *const libc::c_char)
        < 0 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    indent += 1;
    i = 0 as libc::c_int;
    while !(*(*def).fields.offset(i as isize)).name.is_null() {
        fd =
            &mut *(*def).fields.offset(i as isize) as *mut crate::src::botlib::l_struct::fielddef_t;
        if WriteIndent(fp, indent) == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if crate::stdlib::fprintf(
            fp,
            b"%s\t\x00" as *const u8 as *const libc::c_char,
            (*fd).name,
        ) < 0 as libc::c_int
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        if (*fd).type_0 & 0x100 as libc::c_int != 0 {
            num = (*fd).maxarray;
            if crate::stdlib::fprintf(fp, b"{\x00" as *const u8 as *const libc::c_char)
                < 0 as libc::c_int
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        } else {
            num = 1 as libc::c_int
        }
        loop
        //end if
        {
            let fresh3 = num; //end while
            num = num - 1; //end switch
            if !(fresh3 > 0 as libc::c_int) {
                break;
            }
            match (*fd).type_0 & 0xff as libc::c_int {
                1 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        *(p as *mut libc::c_char) as libc::c_int,
                    ) < 0 as libc::c_int
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_char>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                    //end case
                }
                2 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        *(p as *mut libc::c_int),
                    ) < 0 as libc::c_int
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                }
                3 => {
                    if WriteFloat(fp, *(p as *mut libc::c_float)) == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_float>() as libc::c_ulong as isize)
                        as *mut libc::c_void
                }
                4 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                        p as *mut libc::c_char,
                    ) < 0 as libc::c_int
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end case
                    p = (p as *mut libc::c_char).offset(80 as libc::c_int as isize)
                        as *mut libc::c_void
                }
                6 => {
                    if WriteStructWithIndent(fp, (*fd).substruct, structure, indent) == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    } //end if
                    p = (p as *mut libc::c_char).offset((*(*fd).substruct).size as isize)
                        as *mut libc::c_void
                }
                _ => {}
            }
            if (*fd).type_0 & 0x100 as libc::c_int != 0 {
                if num > 0 as libc::c_int {
                    if crate::stdlib::fprintf(fp, b",\x00" as *const u8 as *const libc::c_char)
                        < 0 as libc::c_int
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                } else if crate::stdlib::fprintf(fp, b"}\x00" as *const u8 as *const libc::c_char)
                    < 0 as libc::c_int
                {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                //end else
            }
        }
        if crate::stdlib::fprintf(fp, b"\r\n\x00" as *const u8 as *const libc::c_char)
            < 0 as libc::c_int
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        i += 1
    }
    indent -= 1;
    if WriteIndent(fp, indent) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if crate::stdlib::fprintf(fp, b"}\r\n\x00" as *const u8 as *const libc::c_char)
        < 0 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//write a structure to a file
//end of the function WriteStructWithIndent
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn WriteStructure(
    mut fp: *mut crate::stdlib::FILE,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
) -> libc::c_int {
    return WriteStructWithIndent(fp, def, structure, 0 as libc::c_int);
}
//end of the function WriteStructure

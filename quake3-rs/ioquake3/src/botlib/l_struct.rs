// =============== BEGIN l_struct_h ================
pub type fielddef_t = crate::src::botlib::l_struct::fielddef_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fielddef_s {
    pub name: *mut libc::c_char,
    pub offset: i32,
    pub type_0: i32,
    pub maxarray: i32,
    pub floatmin: f32,
    pub floatmax: f32,
    pub substruct: *mut crate::src::botlib::l_struct::structdef_s,
}

pub type structdef_t = crate::src::botlib::l_struct::structdef_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct structdef_s {
    pub size: i32,
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
    let mut i: i32 = 0; //end for
    i = 0 as i32;
    while !(*defs.offset(i as isize)).name.is_null() {
        if libc::strcmp((*defs.offset(i as isize)).name, name) == 0 {
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
    mut source: *mut source_t,
    mut fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> qboolean {
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut token_s,
    };
    let mut negative: i32 = qfalse as i32;
    let mut intval: isize = 0;
    let mut intmin: isize = 0 as i32 as isize;
    let mut intmax: isize = 0 as i32 as isize;
    let mut floatval: f64 = 0.;
    if PC_ExpectAnyToken(
        source as *mut source_s,
        &mut token as *mut _ as *mut token_s,
    ) == 0
    {
        return qfalse;
    }
    //check for minus sign
    if token.type_0 == 5 as i32 {
        //end if
        if (*fd).type_0 & 0x400 as i32 != 0 {
            SourceError(
                source as *mut source_s,
                b"expected unsigned value, found %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            return qfalse;
        }
        //if not a minus sign
        if libc::strcmp(
            token.string.as_mut_ptr(),
            b"-\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            SourceError(
                source as *mut source_s,
                b"unexpected punctuation %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            return qfalse;
        }
        negative = qtrue as i32;
        //read the number
        if PC_ExpectAnyToken(
            source as *mut source_s,
            &mut token as *mut _ as *mut token_s,
        ) == 0
        {
            return qfalse;
        }
    }
    //check if it is a number
    if token.type_0 != 3 as i32 {
        SourceError(
            source as *mut source_s,
            b"expected number, found %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token.string.as_mut_ptr(),
        ); //end if
        return qfalse;
    }
    //check for a float value
    if token.subtype & 0x800 as i32 != 0 {
        //end if
        if (*fd).type_0 & 0xff as i32 != 3 as i32 {
            SourceError(
                source as *mut source_s,
                b"unexpected float\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ); //end if
            return qfalse;
        } //end if
        floatval = token.floatvalue as f64;
        if negative != 0 {
            floatval = -floatval
        }
        if (*fd).type_0 & 0x200 as i32 != 0 {
            if floatval < (*fd).floatmin as f64 || floatval > (*fd).floatmax as f64 {
                SourceError(
                    source as *mut source_s,
                    b"float out of range [%f, %f]\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*fd).floatmin as f64,
                    (*fd).floatmax as f64,
                );
                return qfalse;
            }
            //end if
        }
        *(p as *mut f32) = floatval as f32;
        return qtrue;
    }
    //
    intval = token.intvalue as isize;
    if negative != 0 {
        intval = -intval
    }
    //check bounds
    if (*fd).type_0 & 0xff as i32 == 1 as i32 {
        if (*fd).type_0 & 0x400 as i32 != 0 {
            intmin = 0 as i32 as isize; //end if
            intmax = 255 as i32 as isize
        } else {
            intmin = -(128 as i32) as isize; //end else if
            intmax = 127 as i32 as isize
        }
    } //end else if
    if (*fd).type_0 & 0xff as i32 == 2 as i32 {
        if (*fd).type_0 & 0x400 as i32 != 0 {
            intmin = 0 as i32 as isize; //end if
            intmax = 65535 as i32 as isize
        } else {
            intmin = -(32768 as i32) as isize;
            intmax = 32767 as i32 as isize
        }
    }
    if (*fd).type_0 & 0xff as i32 == 1 as i32 || (*fd).type_0 & 0xff as i32 == 2 as i32 {
        if (*fd).type_0 & 0x200 as i32 != 0 {
            intmin = if intmin as f32 > (*fd).floatmin {
                intmin as f32
            } else {
                (*fd).floatmin
            } as isize;
            intmax = if (intmax as f32) < (*fd).floatmax {
                intmax as f32
            } else {
                (*fd).floatmax
            } as isize
        }
        if intval < intmin || intval > intmax {
            SourceError(
                source as *mut source_s,
                b"value %ld out of range [%ld, %ld]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                intval,
                intmin,
                intmax,
            );
            return qfalse;
        }
    //end if
    } else if (*fd).type_0 & 0xff as i32 == 3 as i32 {
        if (*fd).type_0 & 0x200 as i32 != 0 {
            if (intval as f32) < (*fd).floatmin || intval as f32 > (*fd).floatmax {
                SourceError(
                    source as *mut source_s,
                    b"value %ld out of range [%f, %f]\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    intval,
                    (*fd).floatmin as f64,
                    (*fd).floatmax as f64,
                );
                return qfalse;
            }
            //end if
        }
        //end if
    }
    //store the value
    if (*fd).type_0 & 0xff as i32 == 1 as i32 {
        //end else
        if (*fd).type_0 & 0x400 as i32 != 0 {
            *(p as *mut u8) = intval as u8
        } else {
            *(p as *mut libc::c_char) = intval as libc::c_char
        }
    } else if (*fd).type_0 & 0xff as i32 == 2 as i32 {
        //end if
        if (*fd).type_0 & 0x400 as i32 != 0 {
            *(p as *mut u32) = intval as u32
        } else {
            *(p as *mut i32) = intval as i32
        }
    } else if (*fd).type_0 & 0xff as i32 == 3 as i32 {
        *(p as *mut f32) = intval as f32
    } //end else
    return qtrue;
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
    mut source: *mut source_t,
    mut fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> qboolean {
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut token_s,
    };
    if PC_ExpectAnyToken(
        source as *mut source_s,
        &mut token as *mut _ as *mut token_s,
    ) == 0
    {
        return qfalse;
    }
    //take literals into account
    if token.type_0 == 2 as i32 {
        //end if
        StripSingleQuotes(token.string.as_mut_ptr()); //end if
        *(p as *mut libc::c_char) = token.string[0 as i32 as usize]
    } else {
        PC_UnreadLastToken(source as *mut source_s);
        if ReadNumber(source, fd, p) as u64 == 0 {
            return qfalse;
        }
    }
    return qtrue;
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
    mut source: *mut source_t,
    mut _fd: *mut crate::src::botlib::l_struct::fielddef_t,
    mut p: *mut libc::c_void,
) -> i32 {
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut token_s,
    };
    if PC_ExpectTokenType(
        source as *mut source_s,
        1 as i32,
        0 as i32,
        &mut token as *mut _ as *mut token_s,
    ) == 0
    {
        return 0 as i32;
    }
    //remove the double quotes
    StripDoubleQuotes(token.string.as_mut_ptr());
    //copy the string
    crate::stdlib::strncpy(
        p as *mut libc::c_char,
        token.string.as_mut_ptr(),
        (80 as i32 - 1 as i32) as usize,
    );
    //make sure the string is closed with a zero
    *(p as *mut libc::c_char).offset((80 as i32 - 1 as i32) as isize) =
        '\u{0}' as i32 as libc::c_char;
    //
    return 1 as i32;
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
    mut source: *mut source_t,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
) -> i32 {
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut token_s,
    }; //end while
    let mut fd: *mut crate::src::botlib::l_struct::fielddef_t =
        0 as *mut crate::src::botlib::l_struct::fielddef_t;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut num: i32 = 0;
    if PC_ExpectTokenString(
        source as *mut source_s,
        b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        return 0 as i32;
    }
    loop {
        if PC_ExpectAnyToken(
            source as *mut source_s,
            &mut token as *mut _ as *mut token_s,
        ) == 0
        {
            return qfalse as i32;
        }
        //if end of structure
        if libc::strcmp(
            token.string.as_mut_ptr(),
            b"}\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            break;
        }
        //find the field with the name
        fd = FindField((*def).fields, token.string.as_mut_ptr()); //end if
        if fd.is_null() {
            SourceError(
                source as *mut source_s,
                b"unknown structure field %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end else
            return qfalse as i32;
        } //end if
        if (*fd).type_0 & 0x100 as i32 != 0 {
            num = (*fd).maxarray; //end if
            if PC_ExpectTokenString(
                source as *mut source_s,
                b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                return qfalse as i32;
            }
        } else {
            num = 1 as i32
        } //end switch
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        loop {
            let fresh0 = num;
            num = num - 1;
            if !(fresh0 > 0 as i32) {
                break;
            }
            if (*fd).type_0 & 0x100 as i32 != 0 {
                if PC_CheckTokenString(
                    source as *mut source_s,
                    b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
                {
                    break;
                }
            }
            match (*fd).type_0 & 0xff as i32 {
                1 => {
                    if ReadChar(source, fd, p) as u64 == 0 {
                        return qfalse as i32;
                    }
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_char>() as usize as isize)
                        as *mut libc::c_void
                    //end case
                }
                2 => {
                    if ReadNumber(source, fd, p) as u64 == 0 {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<i32>() as usize as isize)
                        as *mut libc::c_void
                }
                3 => {
                    if ReadNumber(source, fd, p) as u64 == 0 {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<f32>() as usize as isize)
                        as *mut libc::c_void
                }
                4 => {
                    if ReadString(source, fd, p) == 0 {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char).offset(80 as i32 as isize) as *mut libc::c_void
                }
                6 => {
                    if (*fd).substruct.is_null() {
                        SourceError(
                            source as *mut source_s,
                            b"BUG: no sub structure defined\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                        return qfalse as i32;
                    }
                    ReadStructure(source, (*fd).substruct, p as *mut libc::c_char);
                    p = (p as *mut libc::c_char).offset((*(*fd).substruct).size as isize)
                        as *mut libc::c_void
                }
                _ => {}
            }
            if !((*fd).type_0 & 0x100 as i32 != 0) {
                continue;
            }
            if PC_ExpectAnyToken(
                source as *mut source_s,
                &mut token as *mut _ as *mut token_s,
            ) == 0
            {
                return qfalse as i32;
            }
            if libc::strcmp(
                token.string.as_mut_ptr(),
                b"}\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                break;
            }
            if libc::strcmp(
                token.string.as_mut_ptr(),
                b",\x00" as *const u8 as *const libc::c_char,
            ) != 0
            {
                SourceError(
                    source as *mut source_s,
                    b"expected a comma, found %s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    token.string.as_mut_ptr(),
                );
                return qfalse as i32;
            }
            //end if
            //end if
        }
    }
    return qtrue as i32;
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

pub unsafe extern "C" fn WriteIndent(mut fp: *mut FILE, mut indent: i32) -> i32 {
    loop {
        let fresh1 = indent; //end while
        indent = indent - 1;
        if !(fresh1 > 0 as i32) {
            break;
        }
        if crate::stdlib::fprintf(fp, b"\t\x00" as *const u8 as *const libc::c_char) < 0 as i32 {
            return qfalse as i32;
        }
    }
    return qtrue as i32;
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

pub unsafe extern "C" fn WriteFloat(mut fp: *mut FILE, mut value: f32) -> i32 {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut l: i32 = 0;
    Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as usize as i32,
        b"%f\x00" as *const u8 as *const libc::c_char,
        value as f64,
    );
    l = crate::stdlib::strlen(buf.as_mut_ptr()) as i32;
    loop
    //strip any trailing zeros
    {
        let fresh2 = l; //end while
        l = l - 1; //end if
        if !(fresh2 > 1 as i32) {
            break;
        }
        if buf[l as usize] as i32 != '0' as i32 && buf[l as usize] as i32 != '.' as i32 {
            break;
        }
        if buf[l as usize] as i32 == '.' as i32 {
            buf[l as usize] = 0 as i32 as libc::c_char;
            break;
        } else {
            buf[l as usize] = 0 as i32 as libc::c_char
        }
    }
    //write the float to file
    if crate::stdlib::fprintf(
        fp,
        b"%s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    ) < 0 as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
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
    mut fp: *mut FILE,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
    mut indent: i32,
) -> i32 {
    let mut i: i32 = 0; //end for
    let mut num: i32 = 0; //end else
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void; //end if
    let mut fd: *mut crate::src::botlib::l_struct::fielddef_t =
        0 as *mut crate::src::botlib::l_struct::fielddef_t;
    if WriteIndent(fp, indent) == 0 {
        return qfalse as i32;
    }
    if crate::stdlib::fprintf(fp, b"{\r\n\x00" as *const u8 as *const libc::c_char) < 0 as i32 {
        return qfalse as i32;
    }
    indent += 1;
    i = 0 as i32;
    while !(*(*def).fields.offset(i as isize)).name.is_null() {
        fd =
            &mut *(*def).fields.offset(i as isize) as *mut crate::src::botlib::l_struct::fielddef_t;
        if WriteIndent(fp, indent) == 0 {
            return qfalse as i32;
        }
        if crate::stdlib::fprintf(
            fp,
            b"%s\t\x00" as *const u8 as *const libc::c_char,
            (*fd).name,
        ) < 0 as i32
        {
            return qfalse as i32;
        }
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        if (*fd).type_0 & 0x100 as i32 != 0 {
            num = (*fd).maxarray;
            if crate::stdlib::fprintf(fp, b"{\x00" as *const u8 as *const libc::c_char) < 0 as i32 {
                return qfalse as i32;
            }
        } else {
            num = 1 as i32
        }
        loop
        //end if
        {
            let fresh3 = num; //end while
            num = num - 1; //end switch
            if !(fresh3 > 0 as i32) {
                break;
            }
            match (*fd).type_0 & 0xff as i32 {
                1 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        *(p as *mut libc::c_char) as i32,
                    ) < 0 as i32
                    {
                        return qfalse as i32;
                    }
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<libc::c_char>() as usize as isize)
                        as *mut libc::c_void
                    //end case
                }
                2 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        *(p as *mut i32),
                    ) < 0 as i32
                    {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<i32>() as usize as isize)
                        as *mut libc::c_void
                }
                3 => {
                    if WriteFloat(fp, *(p as *mut f32)) == 0 {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char)
                        .offset(::std::mem::size_of::<f32>() as usize as isize)
                        as *mut libc::c_void
                }
                4 => {
                    if crate::stdlib::fprintf(
                        fp,
                        b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                        p as *mut libc::c_char,
                    ) < 0 as i32
                    {
                        return qfalse as i32;
                    } //end case
                    p = (p as *mut libc::c_char).offset(80 as i32 as isize) as *mut libc::c_void
                }
                6 => {
                    if WriteStructWithIndent(fp, (*fd).substruct, structure, indent) == 0 {
                        return qfalse as i32;
                    } //end if
                    p = (p as *mut libc::c_char).offset((*(*fd).substruct).size as isize)
                        as *mut libc::c_void
                }
                _ => {}
            }
            if (*fd).type_0 & 0x100 as i32 != 0 {
                if num > 0 as i32 {
                    if crate::stdlib::fprintf(fp, b",\x00" as *const u8 as *const libc::c_char)
                        < 0 as i32
                    {
                        return qfalse as i32;
                    }
                } else if crate::stdlib::fprintf(fp, b"}\x00" as *const u8 as *const libc::c_char)
                    < 0 as i32
                {
                    return qfalse as i32;
                }
                //end else
            }
        }
        if crate::stdlib::fprintf(fp, b"\r\n\x00" as *const u8 as *const libc::c_char) < 0 as i32 {
            return qfalse as i32;
        }
        i += 1
    }
    indent -= 1;
    if WriteIndent(fp, indent) == 0 {
        return qfalse as i32;
    }
    if crate::stdlib::fprintf(fp, b"}\r\n\x00" as *const u8 as *const libc::c_char) < 0 as i32 {
        return qfalse as i32;
    }
    return qtrue as i32;
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
    mut fp: *mut FILE,
    mut def: *mut crate::src::botlib::l_struct::structdef_t,
    mut structure: *mut libc::c_char,
) -> i32 {
    return WriteStructWithIndent(fp, def, structure, 0 as i32);
}
//end of the function WriteStructure

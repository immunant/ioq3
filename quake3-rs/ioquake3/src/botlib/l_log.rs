use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_interface::botimport;
pub use crate::src::botlib::be_interface::botlib_globals_s;
pub use crate::src::botlib::be_interface::botlib_globals_t;
pub use crate::src::botlib::be_interface::botlibglobals;

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
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

pub type logfile_t = logfile_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct logfile_s {
    pub filename: [libc::c_char; 1024],
    pub fp: *mut FILE,
    pub numwrites: i32,
}

static mut logfile: logfile_t = logfile_t {
    filename: [0; 1024],
    fp: 0 as *const FILE as *mut FILE,
    numwrites: 0,
};
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
 * name:		l_log.h
 *
 * desc:		log file
 *
 * $Archive: /source/code/botlib/l_log.h $
 *
 *****************************************************************************/
//open a log file
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_Open(mut filename: *mut libc::c_char) {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char; //end if
    if crate::src::botlib::l_libvar::LibVarValue(
        b"log\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        return;
    } //end if
    if filename.is_null() || crate::stdlib::strlen(filename) == 0 {
        botimport.Print.expect("non-null function pointer")(
            1 as i32,
            b"openlog <filename>\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return;
    }
    if !logfile.fp.is_null() {
        botimport.Print.expect("non-null function pointer")(
            3 as i32,
            b"log file %s is already opened\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            logfile.filename.as_mut_ptr(),
        );
        return;
    }
    ospath = crate::src::qcommon::files::FS_BuildOSPath(
        crate::src::qcommon::cvar::Cvar_VariableString(
            b"fs_homepath\x00" as *const u8 as *const libc::c_char,
        ),
        crate::src::qcommon::cvar::Cvar_VariableString(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        ),
        filename,
    );
    logfile.fp = crate::stdlib::fopen(ospath, b"wb\x00" as *const u8 as *const libc::c_char);
    if logfile.fp.is_null() {
        botimport.Print.expect("non-null function pointer")(
            3 as i32,
            b"can\'t open the log file %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    Q_strncpyz(logfile.filename.as_mut_ptr(), filename, 1024 as i32);
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"Opened log %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        logfile.filename.as_mut_ptr(),
    );
}
//close the current log file
//end of the function Log_Create
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_Close() {
    if logfile.fp.is_null() {
        return;
    } //end if
    if crate::stdlib::fclose(logfile.fp) != 0 {
        botimport.Print.expect("non-null function pointer")(
            3 as i32,
            b"can\'t close log file %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            logfile.filename.as_mut_ptr(),
        );
        return;
    }
    logfile.fp = 0 as *mut FILE;
    botimport.Print.expect("non-null function pointer")(
        1 as i32,
        b"Closed log %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        logfile.filename.as_mut_ptr(),
    );
}
//close log file if present
//end of the function Log_Close
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_Shutdown() {
    if !logfile.fp.is_null() {
        Log_Close();
    };
}
//write to the current opened log file
//end of the function Log_Shutdown
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_Write(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if logfile.fp.is_null() {
        return;
    }
    ap = args.clone();
    crate::stdlib::vfprintf(logfile.fp, fmt, ap.as_va_list());
    //fprintf(logfile.fp, "\r\n");
    crate::stdlib::fflush(logfile.fp);
}
//write to the current opened log file with a time stamp
//end of the function Log_Write
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_WriteTimeStamped(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if logfile.fp.is_null() {
        return;
    }
    crate::stdlib::fprintf(
        logfile.fp,
        b"%d   %02d:%02d:%02d:%02d   \x00" as *const u8 as *const libc::c_char,
        logfile.numwrites,
        (botlibglobals.time / 60 as i32 as f32 / 60 as i32 as f32) as i32,
        (botlibglobals.time / 60 as i32 as f32) as i32,
        botlibglobals.time as i32,
        (botlibglobals.time * 100 as i32 as f32) as i32 - botlibglobals.time as i32 * 100 as i32,
    );
    ap = args.clone();
    crate::stdlib::vfprintf(logfile.fp, fmt, ap.as_va_list());
    crate::stdlib::fprintf(logfile.fp, b"\r\n\x00" as *const u8 as *const libc::c_char);
    logfile.numwrites += 1;
    crate::stdlib::fflush(logfile.fp);
}
//returns a pointer to the log file
//end of the function Log_Write
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_FilePointer() -> *mut FILE {
    return logfile.fp;
}
//flush log file
//end of the function Log_FilePointer
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Log_Flush() {
    if !logfile.fp.is_null() {
        crate::stdlib::fflush(logfile.fp);
    };
}
//end of the function Log_Flush

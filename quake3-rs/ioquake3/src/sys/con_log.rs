use ::libc;

static mut consoleLog: [libc::c_char; 32768] = [0; 32768];

static mut writePos: u32 = 0 as i32 as u32;

static mut readPos: u32 = 0 as i32 as u32;
/*
==================
CON_LogSize
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogSize() -> u32 {
    if readPos <= writePos {
        return writePos.wrapping_sub(readPos);
    } else {
        return writePos
            .wrapping_add(32768 as i32 as u32)
            .wrapping_sub(readPos);
    };
}
/*
==================
CON_LogFree
==================
*/

unsafe extern "C" fn CON_LogFree() -> u32 {
    return (32768 as i32 as u32)
        .wrapping_sub(CON_LogSize())
        .wrapping_sub(1 as i32 as u32);
}
/*
==================
CON_LogWrite
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogWrite(mut in_0: *const libc::c_char) -> u32 {
    let mut length: u32 = crate::stdlib::strlen(in_0) as u32;
    let mut firstChunk: u32 = 0;
    let mut secondChunk: u32 = 0;
    while CON_LogFree() < length && CON_LogSize() > 0 as i32 as u32 {
        // Free enough space
        while consoleLog[readPos as usize] as i32 != '\n' as i32
            && CON_LogSize() > 1 as i32 as u32
        {
            readPos = readPos
                .wrapping_add(1 as i32 as u32)
                .wrapping_rem(32768 as i32 as u32)
        }
        // Skip past the '\n'
        readPos = readPos
            .wrapping_add(1 as i32 as u32)
            .wrapping_rem(32768 as i32 as u32)
    }
    if CON_LogFree() < length {
        return 0 as i32 as u32;
    }
    if writePos.wrapping_add(length) > 32768 as i32 as u32 {
        firstChunk = (32768 as i32 as u32).wrapping_sub(writePos);
        secondChunk = length.wrapping_sub(firstChunk)
    } else {
        firstChunk = length;
        secondChunk = 0 as i32 as u32
    }
    crate::stdlib::memcpy(
        consoleLog.as_mut_ptr().offset(writePos as isize) as *mut libc::c_void,
        in_0 as *const libc::c_void,
        firstChunk as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        consoleLog.as_mut_ptr() as *mut libc::c_void,
        in_0.offset(firstChunk as isize) as *const libc::c_void,
        secondChunk as libc::c_ulong,
    );
    writePos = writePos
        .wrapping_add(length)
        .wrapping_rem(32768 as i32 as u32);
    return length;
}
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
// Require a minimum version of SDL
// Console
/*
==================
CON_LogRead
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogRead(
    mut out: *mut libc::c_char,
    mut outSize: u32,
) -> u32 {
    let mut firstChunk: u32 = 0;
    let mut secondChunk: u32 = 0;
    if CON_LogSize() < outSize {
        outSize = CON_LogSize()
    }
    if readPos.wrapping_add(outSize) > 32768 as i32 as u32 {
        firstChunk = (32768 as i32 as u32).wrapping_sub(readPos);
        secondChunk = outSize.wrapping_sub(firstChunk)
    } else {
        firstChunk = outSize;
        secondChunk = 0 as i32 as u32
    }
    crate::stdlib::memcpy(
        out as *mut libc::c_void,
        consoleLog.as_mut_ptr().offset(readPos as isize) as *const libc::c_void,
        firstChunk as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        out.offset(firstChunk as isize) as *mut libc::c_void,
        consoleLog.as_mut_ptr() as *const libc::c_void,
        secondChunk as libc::c_ulong,
    );
    readPos = readPos
        .wrapping_add(outSize)
        .wrapping_rem(32768 as i32 as u32);
    return outSize;
}

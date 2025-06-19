use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::qfiles_h::vmHeader_t;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_Alloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::qcommon_h::vmInterpret_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::VMI_BYTECODE;
pub use crate::qcommon_h::VMI_COMPILED;
pub use crate::qcommon_h::VMI_NATIVE;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::common::com_developer;
pub use crate::src::qcommon::common::Hunk_MemoryRemaining;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::files::FS_FindVM;
pub use crate::src::qcommon::files::FS_FreeFile;
pub use crate::src::qcommon::files::FS_ReadFile;
pub use crate::src::qcommon::files::FS_ReadFileDir;
pub use crate::src::qcommon::files::FS_Which;
pub use crate::src::qcommon::vm_interpreted::VM_CallInterpreted;
pub use crate::src::qcommon::vm_interpreted::VM_PrepareInterpreter;
pub use crate::src::qcommon::vm_x86::VM_CallCompiled;
pub use crate::src::qcommon::vm_x86::VM_Compile;
pub use crate::src::sys::sys_main::Sys_LoadGameDll;
pub use crate::src::sys::sys_main::Sys_UnloadDll;

pub use crate::vm_local_h::vmSymbol_s;
pub use crate::vm_local_h::vmSymbol_t;
pub use crate::vm_local_h::vm_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_136 {
    pub c: *mut libc::c_char,
    pub v: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_137 {
    pub h: *mut vmHeader_t,
    pub v: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_138 {
    pub callnum: i32,
    pub args: [i32; 12],
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
// vm.c -- virtual machine
/*


intermix code and data
symbol table

a dll has one imported function: VM_SystemCall
and one exported function: Perform


*/
#[no_mangle]

pub static mut currentVM: *mut vm_t =
    0 as *const vm_t as *mut vm_t;
#[no_mangle]

pub static mut lastVM: *mut vm_t =
    0 as *const vm_t as *mut vm_t;
#[no_mangle]

pub static mut vm_debugLevel: i32 = 0;
// used by Com_Error to get rid of running vm's before longjmp

static mut forced_unload: i32 = 0;
#[no_mangle]

pub static mut vmTable: [vm_t; 3] = [vm_t {
    programStack: 0,
    systemCall: None,
    name: [0; 64],
    searchPath: 0 as *const libc::c_void as *mut libc::c_void,
    dllHandle: 0 as *const libc::c_void as *mut libc::c_void,
    entryPoint: None,
    destroy: None,
    currentlyInterpreting: qfalse,
    compiled: qfalse,
    codeBase: 0 as *const byte
        as *mut byte,
    entryOfs: 0,
    codeLength: 0,
    instructionPointers: 0 as *const intptr_t as *mut intptr_t,
    instructionCount: 0,
    dataBase: 0 as *const byte
        as *mut byte,
    dataMask: 0,
    dataAlloc: 0,
    stackBottom: 0,
    numSymbols: 0,
    symbols: 0 as *const vmSymbol_s as *mut vmSymbol_s,
    callLevel: 0,
    breakFunction: 0,
    breakCount: 0,
    jumpTableTargets: 0 as *const byte
        as *mut byte,
    numJumpTableTargets: 0,
}; 3];
// 64bit!
#[no_mangle]

pub unsafe extern "C" fn VM_Debug(mut level: i32) {
    vm_debugLevel = level;
}
/*
==============
VM_Init
==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Init() {
    Cvar_Get(
        b"vm_cgame\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s; // !@# SHIP WITH SET TO 2

    Cvar_Get(
        b"vm_game\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s; // !@# SHIP WITH SET TO 2

    Cvar_Get(
        b"vm_ui\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x1 as i32,
    ) as *mut cvar_s; // !@# SHIP WITH SET TO 2
    Cmd_AddCommand(
        b"vmprofile\x00" as *const u8 as *const libc::c_char,
        Some(VM_VmProfile_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"vminfo\x00" as *const u8 as *const libc::c_char,
        Some(VM_VmInfo_f as unsafe extern "C" fn() -> ()),
    );
    crate::stdlib::memset(
        vmTable.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[vm_t; 3]>() as libc::c_ulong,
    );
}
/*
===============
VM_ValueToSymbol

Assumes a program counter value
===============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_ValueToSymbol(
    mut vm: *mut vm_t,
    mut value: i32,
) -> *const libc::c_char {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    static mut text: [libc::c_char; 1024] = [0; 1024];
    sym = (*vm).symbols;
    if sym.is_null() {
        return b"NO SYMBOLS\x00" as *const u8 as *const libc::c_char;
    }
    // find the symbol
    while !(*sym).next.is_null() && (*(*sym).next).symValue <= value {
        sym = (*sym).next
    }
    if value == (*sym).symValue {
        return (*sym).symName.as_mut_ptr();
    }
    Com_sprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
        b"%s+%i\x00" as *const u8 as *const libc::c_char,
        (*sym).symName.as_mut_ptr(),
        value - (*sym).symValue,
    );
    return text.as_mut_ptr();
}
/*
===============
VM_ValueToFunctionSymbol

For profiling, find the symbol behind this value
===============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_ValueToFunctionSymbol(
    mut vm: *mut vm_t,
    mut value: i32,
) -> *mut vmSymbol_t {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    static mut nullSym: vmSymbol_t = vmSymbol_t {
        next: 0 as *const vmSymbol_s as *mut vmSymbol_s,
        symValue: 0,
        profileCount: 0,
        symName: [0; 1],
    };
    sym = (*vm).symbols;
    if sym.is_null() {
        return &mut nullSym;
    }
    while !(*sym).next.is_null() && (*(*sym).next).symValue <= value {
        sym = (*sym).next
    }
    return sym;
}
/*
===============
VM_SymbolToValue
===============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_SymbolToValue(
    mut vm: *mut vm_t,
    mut symbol: *const libc::c_char,
) -> i32 {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    sym = (*vm).symbols;
    while !sym.is_null() {
        if libc::strcmp(symbol, (*sym).symName.as_mut_ptr()) == 0 {
            return (*sym).symValue;
        }
        sym = (*sym).next
    }
    return 0 as i32;
}
/*
=====================
VM_SymbolForCompiledPointer
=====================
*/
// 64bit!
/*
===============
ParseHex
===============
*/
#[no_mangle]

pub unsafe extern "C" fn ParseHex(mut text: *const libc::c_char) -> i32 {
    let mut value: i32 = 0;
    let mut c: i32 = 0;
    value = 0 as i32;
    loop {
        let fresh0 = text;
        text = text.offset(1);
        c = *fresh0 as i32;
        if !(c != 0 as i32) {
            break;
        }
        if c >= '0' as i32 && c <= '9' as i32 {
            value = value * 16 as i32 + c - '0' as i32
        } else if c >= 'a' as i32 && c <= 'f' as i32 {
            value = value * 16 as i32 + 10 as i32 + c - 'a' as i32
        } else {
            if !(c >= 'A' as i32 && c <= 'F' as i32) {
                continue;
            }
            value = value * 16 as i32 + 10 as i32 + c - 'A' as i32
        }
    }
    return value;
}
/*
===============
VM_LoadSymbols
===============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_LoadSymbols(mut vm: *mut vm_t) {
    let mut mapfile: C2RustUnnamed_136 = C2RustUnnamed_136 {
        c: 0 as *mut libc::c_char,
    };
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut symbols: [libc::c_char; 64] = [0; 64];
    let mut prev: *mut *mut vmSymbol_t =
        0 as *mut *mut vmSymbol_t;
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut count: i32 = 0;
    let mut value: i32 = 0;
    let mut chars: i32 = 0;
    let mut segment: i32 = 0;
    let mut numInstructions: i32 = 0;
    // don't load symbols if not developer
    if (*com_developer).integer == 0 {
        return;
    }
    COM_StripExtension(
        (*vm).name.as_mut_ptr(),
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    Com_sprintf(
        symbols.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"vm/%s.map\x00" as *const u8 as *const libc::c_char,
        name.as_mut_ptr(),
    );
    FS_ReadFile(symbols.as_mut_ptr(), &mut mapfile.v);
    if mapfile.c.is_null() {
        Com_Printf(
            b"Couldn\'t load symbol file: %s\n\x00" as *const u8 as *const libc::c_char,
            symbols.as_mut_ptr(),
        );
        return;
    }
    numInstructions = (*vm).instructionCount;
    // parse the symbols
    text_p = mapfile.c;
    prev = &mut (*vm).symbols;
    count = 0 as i32;
    loop {
        token = COM_Parse(&mut text_p);
        if *token.offset(0 as i32 as isize) == 0 {
            break;
        }
        segment = ParseHex(token);
        if segment != 0 {
            COM_Parse(&mut text_p);
            COM_Parse(&mut text_p);
        // only load code segment values
        } else {
            token = COM_Parse(&mut text_p);
            if *token.offset(0 as i32 as isize) == 0 {
                Com_Printf(
                    b"WARNING: incomplete line at end of file\n\x00" as *const u8
                        as *const libc::c_char,
                );
                break;
            } else {
                value = ParseHex(token);
                token = COM_Parse(&mut text_p);
                if *token.offset(0 as i32 as isize) == 0 {
                    Com_Printf(
                        b"WARNING: incomplete line at end of file\n\x00" as *const u8
                            as *const libc::c_char,
                    );
                    break;
                } else {
                    chars = crate::stdlib::strlen(token) as i32;
                    sym = Hunk_Alloc(
                        (::std::mem::size_of::<vmSymbol_t>() as libc::c_ulong)
                            .wrapping_add(chars as libc::c_ulong) as i32,
                        h_high,
                    ) as *mut vmSymbol_t;
                    *prev = sym;
                    prev = &mut (*sym).next;
                    (*sym).next = 0 as *mut vmSymbol_s;
                    // convert value from an instruction number to a code offset
                    if value >= 0 as i32 && value < numInstructions {
                        value = *(*vm).instructionPointers.offset(value as isize) as i32
                    }
                    (*sym).symValue = value;
                    Q_strncpyz(
                        (*sym).symName.as_mut_ptr(),
                        token,
                        chars + 1 as i32,
                    );
                    count += 1
                }
            }
        }
    }
    (*vm).numSymbols = count;
    Com_Printf(
        b"%i symbols parsed from %s\n\x00" as *const u8 as *const libc::c_char,
        count,
        symbols.as_mut_ptr(),
    );
    FS_FreeFile(mapfile.v);
}
/*
============
VM_DllSyscall

Dlls will call this directly

 rcg010206 The horror; the horror.

  The syscall mechanism relies on stack manipulation to get its args.
   This is likely due to C's inability to pass "..." parameters to
   a function in one clean chunk. On PowerPC Linux, these parameters
   are not necessarily passed on the stack, so while (&arg[0] == arg)
   is true, (&arg[1] == 2nd function parameter) is not necessarily
   accurate, as arg's value might have been stored to the stack or
   other piece of scratch memory to give it a valid address, but the
   next parameter might still be sitting in a register.

  Quake's syscall system also assumes that the stack grows downward,
   and that any needed types can be squeezed, safely, into a signed int.

  This hack below copies all needed values for an argument to a
   array in memory, so that Quake can get the correct values. This can
   also be used on systems where the stack grows upwards, as the
   presumably standard and safe stdargs.h macros are used.

  As for having enough space in a signed int for your datatypes, well,
   it might be better to wait for DOOM 3 before you start porting.  :)

  The original code, while probably still inherently dangerous, seems
   to work well enough for the platforms it already works on. Rather
   than add the performance hit for those platforms, the original code
   is still in use there.

  For speed, we just grab 15 arguments, and don't worry about exactly
   how many the syscall actually needs; the extra is thrown away.

============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_DllSyscall(
    mut arg: intptr_t,
    mut args: ...
) -> intptr_t {
    // rcg010206 - see commentary above
    let mut args_0: [intptr_t; 16] = [0; 16];
    let mut i: i32 = 0;
    let mut ap: ::std::ffi::VaListImpl;
    args_0[0 as i32 as usize] = arg;
    ap = args.clone();
    i = 1 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[intptr_t; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<intptr_t>() as libc::c_ulong)
    {
        args_0[i as usize] = ap.as_va_list().arg::<intptr_t>();
        i += 1
    }
    return (*currentVM).systemCall.expect("non-null function pointer")(args_0.as_mut_ptr());
    // original id code
}
/*
=================
VM_LoadQVM

Load a .qvm file
=================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_LoadQVM(
    mut vm: *mut vm_t,
    mut alloc: qboolean,
    mut unpure: qboolean,
) -> *mut vmHeader_t {
    let mut dataLength: i32 = 0;
    let mut i: i32 = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut header: C2RustUnnamed_137 = C2RustUnnamed_137 {
        h: 0 as *mut vmHeader_t,
    };
    // load the image
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"vm/%s.qvm\x00" as *const u8 as *const libc::c_char,
        (*vm).name.as_mut_ptr(),
    );
    Com_Printf(
        b"Loading vm file %s...\n\x00" as *const u8 as *const libc::c_char,
        filename.as_mut_ptr(),
    );
    FS_ReadFileDir(
        filename.as_mut_ptr(),
        (*vm).searchPath,
        unpure,
        &mut header.v,
    );
    if header.h.is_null() {
        Com_Printf(
            b"Failed.\n\x00" as *const u8 as *const libc::c_char,
        );
        VM_Free(vm);
        Com_Printf(
            b"^3Warning: Couldn\'t open VM file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return 0 as *mut vmHeader_t;
    }
    // show where the qvm was loaded from
    FS_Which(filename.as_mut_ptr(), (*vm).searchPath);
    if (*header.h).vmMagic == 0x12721445 as i32 {
        Com_Printf(
            b"...which has vmMagic VM_MAGIC_VER2\n\x00" as *const u8 as *const libc::c_char,
        );
        // byte swap the header
        i = 0 as i32;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<vmHeader_t>() as libc::c_ulong)
                .wrapping_div(4 as i32 as libc::c_ulong)
        {
            *(header.h as *mut i32).offset(i as isize) = *(header.h as *mut i32).offset(i as isize);
            i += 1
        }
        // validate
        if (*header.h).jtrgLength < 0 as i32
            || (*header.h).bssLength < 0 as i32
            || (*header.h).dataLength < 0 as i32
            || (*header.h).litLength < 0 as i32
            || (*header.h).codeLength <= 0 as i32
        {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(
                b"^3Warning: %s has bad header\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return 0 as *mut vmHeader_t;
        }
    } else if (*header.h).vmMagic == 0x12721444 as i32 {
        // byte swap the header
        // sizeof( vmHeader_t ) - sizeof( int ) is the 1.32b vm header size
        i = 0 as i32;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<vmHeader_t>() as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<i32>() as libc::c_ulong)
                .wrapping_div(4 as i32 as libc::c_ulong)
        {
            *(header.h as *mut i32).offset(i as isize) = *(header.h as *mut i32).offset(i as isize);
            i += 1
        }
        // validate
        if (*header.h).bssLength < 0 as i32
            || (*header.h).dataLength < 0 as i32
            || (*header.h).litLength < 0 as i32
            || (*header.h).codeLength <= 0 as i32
        {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(
                b"^3Warning: %s has bad header\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return 0 as *mut vmHeader_t;
        }
    } else {
        VM_Free(vm);
        FS_FreeFile(header.v);
        Com_Printf(
            b"^3Warning: %s does not have a recognisable magic number in its header\n\x00"
                as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return 0 as *mut vmHeader_t;
    }
    // round up to next power of 2 so all data operations can
    // be mask protected
    dataLength = (*header.h).dataLength + (*header.h).litLength + (*header.h).bssLength;
    i = 0 as i32;
    while dataLength > (1 as i32) << i {
        i += 1
    }
    dataLength = (1 as i32) << i;
    if alloc as u64 != 0 {
        // allocate zero filled space for initialized and uninitialized data
        // leave some space beyond data mask so we can secure all mask operations
        (*vm).dataAlloc = dataLength + 4 as i32;
        (*vm).dataBase = Hunk_Alloc(
            (*vm).dataAlloc,
            h_high,
        ) as *mut byte;
        (*vm).dataMask = dataLength - 1 as i32
    } else {
        // clear the data, but make sure we're not clearing more than allocated
        if (*vm).dataAlloc != dataLength + 4 as i32 {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(
                b"^3Warning: Data region size of %s not matching after VM_Restart()\n\x00"
                    as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return 0 as *mut vmHeader_t;
        }
        crate::stdlib::memset(
            (*vm).dataBase as *mut libc::c_void,
            0 as i32,
            (*vm).dataAlloc as libc::c_ulong,
        );
    }
    // copy the intialized data
    crate::stdlib::memcpy(
        (*vm).dataBase as *mut libc::c_void,
        (header.h as *mut byte)
            .offset((*header.h).dataOffset as isize) as *const libc::c_void,
        ((*header.h).dataLength + (*header.h).litLength) as libc::c_ulong,
    );
    // byte swap the longs
    i = 0 as i32;
    while i < (*header.h).dataLength {
        *((*vm).dataBase.offset(i as isize) as *mut i32) =
            *((*vm).dataBase.offset(i as isize) as *mut i32);
        i += 4 as i32
    }
    if (*header.h).vmMagic == 0x12721445 as i32 {
        let mut previousNumJumpTableTargets: i32 = (*vm).numJumpTableTargets;
        (*header.h).jtrgLength &= !(0x3 as i32);
        (*vm).numJumpTableTargets = (*header.h).jtrgLength >> 2 as i32;
        Com_Printf(
            b"Loading %d jump table targets\n\x00" as *const u8 as *const libc::c_char,
            (*vm).numJumpTableTargets,
        );
        if alloc as u64 != 0 {
            (*vm).jumpTableTargets = Hunk_Alloc(
                (*header.h).jtrgLength,
                h_high,
            ) as *mut byte
        } else {
            if (*vm).numJumpTableTargets != previousNumJumpTableTargets {
                VM_Free(vm);
                FS_FreeFile(header.v);
                Com_Printf(
                    b"^3Warning: Jump table size of %s not matching after VM_Restart()\n\x00"
                        as *const u8 as *const libc::c_char,
                    filename.as_mut_ptr(),
                );
                return 0 as *mut vmHeader_t;
            }
            crate::stdlib::memset(
                (*vm).jumpTableTargets as *mut libc::c_void,
                0 as i32,
                (*header.h).jtrgLength as libc::c_ulong,
            );
        }
        crate::stdlib::memcpy(
            (*vm).jumpTableTargets as *mut libc::c_void,
            (header.h as *mut byte)
                .offset((*header.h).dataOffset as isize)
                .offset((*header.h).dataLength as isize)
                .offset((*header.h).litLength as isize) as *const libc::c_void,
            (*header.h).jtrgLength as libc::c_ulong,
        );
        // byte swap the longs
        i = 0 as i32;
        while i < (*header.h).jtrgLength {
            *((*vm).jumpTableTargets.offset(i as isize) as *mut i32) =
                *((*vm).jumpTableTargets.offset(i as isize) as *mut i32);
            i += 4 as i32
        }
    }
    return header.h;
}
/*
=================
VM_Restart

Reload the data, but leave everything else in place
This allows a server to do a map_restart without changing memory allocation

We need to make sure that servers can access unpure QVMs (not contained in any pak)
even if the client is pure, so take "unpure" as argument.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Restart(
    mut vm: *mut vm_t,
    mut unpure: qboolean,
) -> *mut vm_t {
    let mut header: *mut vmHeader_t = 0 as *mut vmHeader_t;
    // DLL's can't be restarted in place
    if !(*vm).dllHandle.is_null() {
        let mut name: [libc::c_char; 64] = [0; 64];
        let mut systemCall: Option<
            unsafe extern "C" fn(_: *mut intptr_t) -> intptr_t,
        > = None;
        systemCall = (*vm).systemCall;
        Q_strncpyz(
            name.as_mut_ptr(),
            (*vm).name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        );
        VM_Free(vm);
        vm = VM_Create(name.as_mut_ptr(), systemCall, VMI_NATIVE);
        return vm;
    }
    // load the image
    Com_Printf(
        b"VM_Restart()\n\x00" as *const u8 as *const libc::c_char,
    );
    header = VM_LoadQVM(vm, qfalse, unpure);
    if header.is_null() {
        Com_Error(
            ERR_DROP as i32,
            b"VM_Restart failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    // free the original file
    FS_FreeFile(header as *mut libc::c_void);
    return vm;
}
/*
================
VM_Create

If image ends in .qvm it will be interpreted, otherwise
it will attempt to load as a system dll
================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Create(
    mut module: *const libc::c_char,
    mut systemCalls: Option<
        unsafe extern "C" fn(_: *mut intptr_t) -> intptr_t,
    >,
    mut interpret: vmInterpret_t,
) -> *mut vm_t {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut header: *mut vmHeader_t = 0 as *mut vmHeader_t;
    let mut i: i32 = 0;
    let mut remaining: i32 = 0;
    let mut retval: i32 = 0;
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    let mut startSearch: *mut libc::c_void = 0 as *mut libc::c_void;
    if module.is_null() || *module.offset(0 as i32 as isize) == 0 || systemCalls.is_none() {
        Com_Error(
            ERR_FATAL as i32,
            b"VM_Create: bad parms\x00" as *const u8 as *const libc::c_char,
        );
    }
    remaining = Hunk_MemoryRemaining();
    // see if we already have the VM
    i = 0 as i32;
    while i < 3 as i32 {
        if Q_stricmp(vmTable[i as usize].name.as_mut_ptr(), module)
            == 0
        {
            vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
            return vm;
        }
        i += 1
    }
    // find a free vm
    i = 0 as i32;
    while i < 3 as i32 {
        if vmTable[i as usize].name[0 as i32 as usize] == 0 {
            break;
        }
        i += 1
    }
    if i == 3 as i32 {
        Com_Error(
            ERR_FATAL as i32,
            b"VM_Create: no free vm_t\x00" as *const u8 as *const libc::c_char,
        );
    }
    vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
    Q_strncpyz(
        (*vm).name.as_mut_ptr(),
        module,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    loop {
        retval = FS_FindVM(
            &mut startSearch,
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
            module,
            (interpret as u32 == VMI_NATIVE as i32 as u32) as i32,
        );
        if retval == VMI_NATIVE as i32 {
            Com_Printf(
                b"Try loading dll file %s\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            (*vm).dllHandle = Sys_LoadGameDll(
                filename.as_mut_ptr(),
                &mut (*vm).entryPoint,
                Some(
                    VM_DllSyscall
                        as unsafe extern "C" fn(
                            _: intptr_t,
                            _: ...
                        ) -> intptr_t,
                ),
            );
            if !(*vm).dllHandle.is_null() {
                (*vm).systemCall = systemCalls;
                return vm;
            }
            Com_Printf(
                b"Failed loading dll, trying next\n\x00" as *const u8 as *const libc::c_char,
            );
        } else if retval == VMI_COMPILED as i32 {
            (*vm).searchPath = startSearch;
            header = VM_LoadQVM(
                vm,
                qtrue,
                qfalse,
            );
            if !header.is_null() {
                break;
            }
            // VM_Free overwrites the name on failed load
            Q_strncpyz(
                (*vm).name.as_mut_ptr(),
                module,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
            );
        }
        if !(retval >= 0 as i32) {
            break;
        }
    }
    if retval < 0 as i32 {
        return 0 as *mut vm_t;
    }
    (*vm).systemCall = systemCalls;
    // allocate space for the jump targets, which will be filled in by the compile/prep functions
    (*vm).instructionCount = (*header).instructionCount;
    (*vm).instructionPointers = Hunk_Alloc(
        ((*vm).instructionCount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<intptr_t>() as libc::c_ulong)
            as i32,
        h_high,
    ) as *mut intptr_t;
    // copy or compile the instructions
    (*vm).codeLength = (*header).codeLength;
    (*vm).compiled = qfalse;
    if interpret as u32 != VMI_BYTECODE as i32 as u32 {
        (*vm).compiled = qtrue;
        VM_Compile(
            vm as *mut vm_s,
            header as *mut vmHeader_t,
        );
    }
    // VM_Compile may have reset vm->compiled if compilation failed
    if (*vm).compiled as u64 == 0 {
        VM_PrepareInterpreter(
            vm as *mut vm_s,
            header as *mut vmHeader_t,
        );
    }
    // free the original file
    FS_FreeFile(header as *mut libc::c_void);
    // load the map file
    VM_LoadSymbols(vm);
    // the stack is implicitly at the end of the image
    (*vm).programStack = (*vm).dataMask + 1 as i32;
    (*vm).stackBottom = (*vm).programStack - 0x10000 as i32;
    Com_Printf(
        b"%s loaded in %d bytes on the hunk\n\x00" as *const u8 as *const libc::c_char,
        module,
        remaining - Hunk_MemoryRemaining(),
    );
    return vm;
}
/*
==============
VM_Free
==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Free(mut vm: *mut vm_t) {
    if vm.is_null() {
        return;
    }
    if (*vm).callLevel != 0 {
        if forced_unload == 0 {
            Com_Error(
                ERR_FATAL as i32,
                b"VM_Free(%s) on running vm\x00" as *const u8 as *const libc::c_char,
                (*vm).name.as_mut_ptr(),
            );
        } else {
            Com_Printf(
                b"forcefully unloading %s vm\n\x00" as *const u8 as *const libc::c_char,
                (*vm).name.as_mut_ptr(),
            );
        }
    }
    if (*vm).destroy.is_some() {
        (*vm).destroy.expect("non-null function pointer")(vm);
    }
    if !(*vm).dllHandle.is_null() {
        Sys_UnloadDll((*vm).dllHandle);
        crate::stdlib::memset(
            vm as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<vm_t>() as libc::c_ulong,
        );
    }
    // now automatically freed by hunk
    crate::stdlib::memset(
        vm as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<vm_t>() as libc::c_ulong,
    );
    currentVM = 0 as *mut vm_t;
    lastVM = 0 as *mut vm_t;
}
#[no_mangle]

pub unsafe extern "C" fn VM_Clear() {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 3 as i32 {
        VM_Free(&mut *vmTable.as_mut_ptr().offset(i as isize));
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn VM_Forced_Unload_Start() {
    forced_unload = 1 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn VM_Forced_Unload_Done() {
    forced_unload = 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn VM_ArgPtr(mut intValue: intptr_t) -> *mut libc::c_void {
    if intValue == 0 {
        return 0 as *mut libc::c_void;
    }
    // currentVM is missing on reconnect
    if currentVM.is_null() {
        return 0 as *mut libc::c_void;
    }
    if (*currentVM).entryPoint.is_some() {
        return (*currentVM).dataBase.offset(intValue as isize) as *mut libc::c_void;
    } else {
        return (*currentVM)
            .dataBase
            .offset((intValue & (*currentVM).dataMask as isize) as isize)
            as *mut libc::c_void;
    };
}
#[no_mangle]

pub unsafe extern "C" fn VM_ExplicitArgPtr(
    mut vm: *mut vm_t,
    mut intValue: intptr_t,
) -> *mut libc::c_void {
    if intValue == 0 {
        return 0 as *mut libc::c_void;
    }
    // currentVM is missing on reconnect here as well?
    if currentVM.is_null() {
        return 0 as *mut libc::c_void;
    }
    //
    if (*vm).entryPoint.is_some() {
        return (*vm).dataBase.offset(intValue as isize) as *mut libc::c_void;
    } else {
        return (*vm)
            .dataBase
            .offset((intValue & (*vm).dataMask as isize) as isize)
            as *mut libc::c_void;
    };
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============
VM_Call


Upon a system call, the stack will look like:

sp+32	parm1
sp+28	parm0
sp+24	return value
sp+20	return address
sp+16	local1
sp+14	local0
sp+12	arg1
sp+8	arg0
sp+4	return stack
sp		return address

An interpreted function will immediately execute
an OP_ENTER instruction, which will subtract space for
locals from sp
==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Call(
    mut vm: *mut vm_t,
    mut callnum: i32,
    mut args: ...
) -> intptr_t {
    let mut oldVM: *mut vm_t = 0 as *mut vm_t;
    let mut r: intptr_t = 0;
    let mut i: i32 = 0;
    if vm.is_null() || (*vm).name[0 as i32 as usize] == 0 {
        Com_Error(
            ERR_FATAL as i32,
            b"VM_Call with NULL vm\x00" as *const u8 as *const libc::c_char,
        );
    }
    oldVM = currentVM;
    currentVM = vm;
    lastVM = vm;
    if vm_debugLevel != 0 {
        Com_Printf(
            b"VM_Call( %d )\n\x00" as *const u8 as *const libc::c_char,
            callnum,
        );
    }
    (*vm).callLevel += 1;
    // if we have a dll loaded, call it directly
    if (*vm).entryPoint.is_some() {
        //rcg010207 -  see dissertation at top of VM_DllSyscall() in this file.
        let mut args_0: [i32; 12] = [0; 12];
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        i = 0 as i32;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[i32; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
        {
            args_0[i as usize] = ap.as_va_list().arg::<i32>();
            i += 1
        }
        r = (*vm).entryPoint.expect("non-null function pointer")(
            callnum,
            args_0[0 as i32 as usize],
            args_0[1 as i32 as usize],
            args_0[2 as i32 as usize],
            args_0[3 as i32 as usize],
            args_0[4 as i32 as usize],
            args_0[5 as i32 as usize],
            args_0[6 as i32 as usize],
            args_0[7 as i32 as usize],
            args_0[8 as i32 as usize],
            args_0[9 as i32 as usize],
            args_0[10 as i32 as usize],
            args_0[11 as i32 as usize],
        )
    } else {
        // calling convention doesn't need conversion in some cases
        let mut a: C2RustUnnamed_138 = C2RustUnnamed_138 {
            callnum: 0,
            args: [0; 12],
        };
        let mut ap_0: ::std::ffi::VaListImpl;
        a.callnum = callnum;
        ap_0 = args.clone();
        i = 0 as i32;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[i32; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
        {
            a.args[i as usize] = ap_0.as_va_list().arg::<i32>();
            i += 1
        }
        if (*vm).compiled as u64 != 0 {
            r = VM_CallCompiled(
                vm as *mut vm_s,
                &mut a.callnum,
            ) as intptr_t
        } else {
            r = VM_CallInterpreted(
                vm as *mut vm_s,
                &mut a.callnum,
            ) as intptr_t
        }
    }
    (*vm).callLevel -= 1;
    if !oldVM.is_null() {
        currentVM = oldVM
    }
    return r;
}
//=================================================================

unsafe extern "C" fn VM_ProfileSort(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    let mut sa: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut sb: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    sa = *(a as *mut *mut vmSymbol_t);
    sb = *(b as *mut *mut vmSymbol_t);
    if (*sa).profileCount < (*sb).profileCount {
        return -(1 as i32);
    }
    if (*sa).profileCount > (*sb).profileCount {
        return 1 as i32;
    }
    return 0 as i32;
}
/*
==============
VM_VmProfile_f

==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_VmProfile_f() {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut sorted: *mut *mut vmSymbol_t =
        0 as *mut *mut vmSymbol_t;
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut i: i32 = 0;
    let mut total: f64 = 0.;
    if lastVM.is_null() {
        return;
    }
    vm = lastVM;
    if (*vm).numSymbols == 0 {
        return;
    }
    sorted =
        Z_Malloc(((*vm).numSymbols as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<*mut vmSymbol_t>() as libc::c_ulong,
        ) as i32) as *mut *mut vmSymbol_t;
    let ref mut fresh1 = *sorted.offset(0 as i32 as isize);
    *fresh1 = (*vm).symbols;
    total = (**sorted.offset(0 as i32 as isize)).profileCount as f64;
    i = 1 as i32;
    while i < (*vm).numSymbols {
        let ref mut fresh2 = *sorted.offset(i as isize);
        *fresh2 = (**sorted.offset((i - 1 as i32) as isize)).next;
        total += (**sorted.offset(i as isize)).profileCount as f64;
        i += 1
    }
    qsort(
        sorted as *mut libc::c_void,
        (*vm).numSymbols as size_t,
        ::std::mem::size_of::<*mut vmSymbol_t>() as libc::c_ulong,
        Some(
            VM_ProfileSort
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    i = 0 as i32;
    while i < (*vm).numSymbols {
        let mut perc: i32 = 0;
        sym = *sorted.offset(i as isize);
        perc = ((100 as i32 as f32 * (*sym).profileCount as f32) as f64 / total) as i32;
        Com_Printf(
            b"%2i%% %9i %s\n\x00" as *const u8 as *const libc::c_char,
            perc,
            (*sym).profileCount,
            (*sym).symName.as_mut_ptr(),
        );
        (*sym).profileCount = 0 as i32;
        i += 1
    }
    Com_Printf(
        b"    %9.0f total\n\x00" as *const u8 as *const libc::c_char,
        total,
    );
    Z_Free(sorted as *mut libc::c_void);
}
/*
==============
VM_VmInfo_f

==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_VmInfo_f() {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut i: i32 = 0;
    Com_Printf(
        b"Registered virtual machines:\n\x00" as *const u8 as *const libc::c_char,
    );
    i = 0 as i32;
    while i < 3 as i32 {
        vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
        if (*vm).name[0 as i32 as usize] == 0 {
            break;
        }
        Com_Printf(
            b"%s : \x00" as *const u8 as *const libc::c_char,
            (*vm).name.as_mut_ptr(),
        );
        if !(*vm).dllHandle.is_null() {
            Com_Printf(
                b"native\n\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            if (*vm).compiled as u64 != 0 {
                Com_Printf(
                    b"compiled on load\n\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                Com_Printf(
                    b"interpreted\n\x00" as *const u8 as *const libc::c_char,
                );
            }
            Com_Printf(
                b"    code length : %7i\n\x00" as *const u8 as *const libc::c_char,
                (*vm).codeLength,
            );
            Com_Printf(
                b"    table length: %7i\n\x00" as *const u8 as *const libc::c_char,
                (*vm).instructionCount * 4 as i32,
            );
            Com_Printf(
                b"    data length : %7i\n\x00" as *const u8 as *const libc::c_char,
                (*vm).dataMask + 1 as i32,
            );
        }
        i += 1
    }
}
/*
===============
VM_LogSyscalls

Insert calls to this while debugging the vm compiler
===============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_LogSyscalls(mut args: *mut i32) {
    static mut callnum: i32 = 0;
    static mut f: *mut FILE =
        0 as *const FILE as *mut FILE;
    if f.is_null() {
        f = crate::stdlib::fopen(
            b"syscalls.log\x00" as *const u8 as *const libc::c_char,
            b"w\x00" as *const u8 as *const libc::c_char,
        )
    }
    callnum += 1;
    crate::stdlib::fprintf(
        f,
        b"%i: %p (%i) = %i %i %i %i\n\x00" as *const u8 as *const libc::c_char,
        callnum,
        args.offset_from((*currentVM).dataBase as *mut i32) as isize as *mut libc::c_void,
        *args.offset(0 as i32 as isize),
        *args.offset(1 as i32 as isize),
        *args.offset(2 as i32 as isize),
        *args.offset(3 as i32 as isize),
        *args.offset(4 as i32 as isize),
    );
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
// Max number of arguments to pass from engine to vm's vmMain function.
// command number + 12 arguments
// Max number of arguments to pass from a vm to engine's syscall handler function for the vm.
// syscall number + 15 arguments
// don't change, this is hardcoded into x86 VMs, opStack protection relies
// on this
// don't change
// Hardcoded in q3asm a reserved at end of bss
//-------------------
//-------------------
// *(stack[top-1]) = stack[top]
//-------------------
// variable sized
// DO NOT MOVE OR CHANGE THESE WITHOUT CHANGING THE VM_OFFSET_* DEFINES
// USED BY THE ASM CODE
// the vm may be recursively entered
//------------------------------------
// hint for FS_ReadFileDir()
// for dynamic linked modules
// for interpreted modules
// actually allocated
// if programStack < stackBottom, error
// counts recursive VM_Call
// increment breakCount on function entry to this
/*
=================
VM_BlockCopy
Executes a block copy operation within currentVM data space
=================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_BlockCopy(mut dest: u32, mut src: u32, mut n: size_t) {
    let mut dataMask: u32 = (*currentVM).dataMask as u32;
    if dest & dataMask != dest
        || src & dataMask != src
        || (dest as libc::c_ulong).wrapping_add(n) & dataMask as libc::c_ulong
            != (dest as libc::c_ulong).wrapping_add(n)
        || (src as libc::c_ulong).wrapping_add(n) & dataMask as libc::c_ulong
            != (src as libc::c_ulong).wrapping_add(n)
    {
        Com_Error(
            ERR_DROP as i32,
            b"OP_BLOCK_COPY out of range!\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memcpy(
        (*currentVM).dataBase.offset(dest as isize) as *mut libc::c_void,
        (*currentVM).dataBase.offset(src as isize) as *const libc::c_void,
        n,
    );
}

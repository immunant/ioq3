use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qfiles_h::vmHeader_t;
pub use crate::src::asm::ftola::qftolsse;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_Alloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::qcommon_h::vm_t;
pub use crate::src::qcommon::vm::VM_BlockCopy;
pub use crate::src::qcommon::vm::VM_Debug;
pub use crate::src::qcommon::vm::VM_ValueToSymbol;

pub use crate::vm_local_h::vmSymbol_s;
pub use crate::vm_local_h::vm_s;
pub use crate::vm_local_h::OP_ADD;
pub use crate::vm_local_h::OP_ADDF;
pub use crate::vm_local_h::OP_ARG;
pub use crate::vm_local_h::OP_BAND;
pub use crate::vm_local_h::OP_BCOM;
pub use crate::vm_local_h::OP_BLOCK_COPY;
pub use crate::vm_local_h::OP_BOR;
pub use crate::vm_local_h::OP_BREAK;
pub use crate::vm_local_h::OP_BXOR;
pub use crate::vm_local_h::OP_CALL;
pub use crate::vm_local_h::OP_CONST;
pub use crate::vm_local_h::OP_CVFI;
pub use crate::vm_local_h::OP_CVIF;
pub use crate::vm_local_h::OP_DIVF;
pub use crate::vm_local_h::OP_DIVI;
pub use crate::vm_local_h::OP_DIVU;
pub use crate::vm_local_h::OP_ENTER;
pub use crate::vm_local_h::OP_EQ;
pub use crate::vm_local_h::OP_EQF;
pub use crate::vm_local_h::OP_GEF;
pub use crate::vm_local_h::OP_GEI;
pub use crate::vm_local_h::OP_GEU;
pub use crate::vm_local_h::OP_GTF;
pub use crate::vm_local_h::OP_GTI;
pub use crate::vm_local_h::OP_GTU;
pub use crate::vm_local_h::OP_IGNORE;
pub use crate::vm_local_h::OP_JUMP;
pub use crate::vm_local_h::OP_LEAVE;
pub use crate::vm_local_h::OP_LEF;
pub use crate::vm_local_h::OP_LEI;
pub use crate::vm_local_h::OP_LEU;
pub use crate::vm_local_h::OP_LOAD1;
pub use crate::vm_local_h::OP_LOAD2;
pub use crate::vm_local_h::OP_LOAD4;
pub use crate::vm_local_h::OP_LOCAL;
pub use crate::vm_local_h::OP_LSH;
pub use crate::vm_local_h::OP_LTF;
pub use crate::vm_local_h::OP_LTI;
pub use crate::vm_local_h::OP_LTU;
pub use crate::vm_local_h::OP_MODI;
pub use crate::vm_local_h::OP_MODU;
pub use crate::vm_local_h::OP_MULF;
pub use crate::vm_local_h::OP_MULI;
pub use crate::vm_local_h::OP_MULU;
pub use crate::vm_local_h::OP_NE;
pub use crate::vm_local_h::OP_NEF;
pub use crate::vm_local_h::OP_NEGF;
pub use crate::vm_local_h::OP_NEGI;
pub use crate::vm_local_h::OP_POP;
pub use crate::vm_local_h::OP_PUSH;
pub use crate::vm_local_h::OP_RSHI;
pub use crate::vm_local_h::OP_RSHU;
pub use crate::vm_local_h::OP_SEX16;
pub use crate::vm_local_h::OP_SEX8;
pub use crate::vm_local_h::OP_STORE1;
pub use crate::vm_local_h::OP_STORE2;
pub use crate::vm_local_h::OP_STORE4;
pub use crate::vm_local_h::OP_SUB;
pub use crate::vm_local_h::OP_SUBF;
pub use crate::vm_local_h::OP_UNDEF;
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
//#define	DEBUG_VM
#[inline]

unsafe extern "C" fn loadWord(mut addr: *mut libc::c_void) -> libc::c_int {
    let mut word: libc::c_int = 0;
    crate::stdlib::memcpy(
        &mut word as *mut libc::c_int as *mut libc::c_void,
        addr,
        4 as libc::c_int as libc::c_ulong,
    );
    return word;
}
#[no_mangle]

pub unsafe extern "C" fn VM_Indent(mut vm: *mut crate::qcommon_h::vm_t) -> *mut libc::c_char {
    static mut string: *mut libc::c_char = b"                                        \x00"
        as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if (*vm).callLevel > 20 as libc::c_int {
        return string;
    }
    return string.offset((2 as libc::c_int * (20 as libc::c_int - (*vm).callLevel)) as isize);
}
#[no_mangle]

pub unsafe extern "C" fn VM_StackTrace(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut programCounter: libc::c_int,
    mut programStack: libc::c_int,
) {
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    loop {
        crate::src::qcommon::common::Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::vm::VM_ValueToSymbol(
                vm as *mut crate::vm_local_h::vm_s,
                programCounter,
            ),
        );
        programStack = *(&mut *(*vm)
            .dataBase
            .offset((programStack + 4 as libc::c_int) as isize)
            as *mut crate::src::qcommon::q_shared::byte
            as *mut libc::c_int);
        programCounter = *(&mut *(*vm).dataBase.offset(programStack as isize)
            as *mut crate::src::qcommon::q_shared::byte
            as *mut libc::c_int);
        if !(programCounter != -(1 as libc::c_int) && {
            count += 1;
            (count) < 32 as libc::c_int
        }) {
            break;
        }
    }
}
/*
====================
VM_PrepareInterpreter
====================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_PrepareInterpreter(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut header: *mut crate::qfiles_h::vmHeader_t,
) {
    let mut op: libc::c_int = 0; // we're now int aligned
    let mut byte_pc: libc::c_int = 0;
    let mut int_pc: libc::c_int = 0;
    let mut code: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut instruction: libc::c_int = 0;
    let mut codeBase: *mut libc::c_int = 0 as *mut libc::c_int;
    (*vm).codeBase = crate::src::qcommon::common::Hunk_Alloc(
        (*vm).codeLength * 4 as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
    ) as *mut crate::src::qcommon::q_shared::byte;
    //	memcpy( vm->codeBase, (byte *)header + header->codeOffset, vm->codeLength );
    // we don't need to translate the instructions, but we still need
    // to find each instructions starting point for jumps
    byte_pc = 0 as libc::c_int;
    int_pc = byte_pc;
    instruction = 0 as libc::c_int;
    code =
        (header as *mut crate::src::qcommon::q_shared::byte).offset((*header).codeOffset as isize);
    codeBase = (*vm).codeBase as *mut libc::c_int;
    // Copy and expand instructions to words while building instruction table
    while instruction < (*header).instructionCount {
        *(*vm).instructionPointers.offset(instruction as isize) = int_pc as crate::stdlib::intptr_t;
        instruction += 1;
        op = *code.offset(byte_pc as isize) as libc::c_int;
        *codeBase.offset(int_pc as isize) = op;
        if byte_pc > (*header).codeLength {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"VM_PrepareInterpreter: pc > header->codeLength\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        byte_pc += 1;
        int_pc += 1;
        // these are the only opcodes that aren't a single byte
        match op {
            3 | 8 | 9 | 4 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24
            | 25 | 26 | 34 => {
                *codeBase.offset(int_pc as isize) = loadWord(&mut *code.offset(byte_pc as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut libc::c_void);
                byte_pc += 4 as libc::c_int;
                int_pc += 1
            }
            33 => {
                *codeBase.offset(int_pc as isize) = *code.offset(byte_pc as isize) as libc::c_int;
                byte_pc += 1;
                int_pc += 1
            }
            _ => {}
        }
    }
    int_pc = 0 as libc::c_int;
    instruction = 0 as libc::c_int;
    // Now that the code has been expanded to int-sized opcodes, we'll translate instruction index
    //into an index into codeBase[], which contains opcodes and operands.
    while instruction < (*header).instructionCount {
        op = *codeBase.offset(int_pc as isize);
        instruction += 1;
        int_pc += 1;
        match op {
            11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 => {
                // These ops need to translate addresses in jumps from instruction index to int index
                if *codeBase.offset(int_pc as isize) < 0 as libc::c_int
                    || *codeBase.offset(int_pc as isize) > (*vm).instructionCount
                {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                        b"VM_PrepareInterpreter: Jump to invalid instruction number\x00"
                            as *const u8 as *const libc::c_char,
                    );
                }
                // codeBase[pc] is the instruction index. Convert that into an offset into
                //the int-aligned codeBase[] by the lookup table.
                *codeBase.offset(int_pc as isize) = *(*vm)
                    .instructionPointers
                    .offset(*codeBase.offset(int_pc as isize) as isize)
                    as libc::c_int;
                int_pc += 1
            }
            3 | 8 | 9 | 4 | 34 | 33 => {
                // These opcodes have an operand that isn't an instruction index
                int_pc += 1
            }
            _ => {}
        }
    }
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
==============
VM_Call


Upon a system call, the stack will look like:

sp+32	parm1
sp+28	parm0
sp+24	return stack
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

pub unsafe extern "C" fn VM_CallInterpreted(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut args: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut stack: [crate::src::qcommon::q_shared::byte; 1039] = [0; 1039];
    let mut opStack: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut opStackOfs: crate::stdlib::uint8_t = 0;
    let mut programCounter: libc::c_int = 0;
    let mut programStack: libc::c_int = 0;
    let mut stackOnEntry: libc::c_int = 0;
    let mut image: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut codeImage: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut v1: libc::c_int = 0;
    let mut dataMask: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    // interpret the code
    (*vm).currentlyInterpreting = crate::src::qcommon::q_shared::qtrue;
    // we might be called recursively, so this might not be the very top
    stackOnEntry = (*vm).programStack;
    programStack = stackOnEntry;
    // set up the stack frame
    image = (*vm).dataBase; // return stack
    codeImage = (*vm).codeBase as *mut libc::c_int; // will terminate the loop on return
    dataMask = (*vm).dataMask;
    programCounter = 0 as libc::c_int;
    programStack -= 8 as libc::c_int + 4 as libc::c_int * 13 as libc::c_int;
    arg = 0 as libc::c_int;
    while arg < 13 as libc::c_int {
        *(&mut *image.offset((programStack + 8 as libc::c_int + arg * 4 as libc::c_int) as isize)
            as *mut crate::src::qcommon::q_shared::byte as *mut libc::c_int) =
            *args.offset(arg as isize);
        arg += 1
    }
    *(&mut *image.offset((programStack + 4 as libc::c_int) as isize)
        as *mut crate::src::qcommon::q_shared::byte as *mut libc::c_int) = 0 as libc::c_int;
    *(&mut *image.offset(programStack as isize) as *mut crate::src::qcommon::q_shared::byte
        as *mut libc::c_int) = -(1 as libc::c_int);
    crate::src::qcommon::vm::VM_Debug(0 as libc::c_int);
    // leave a free spot at start of stack so
    // that as long as opStack is valid, opStack-1 will
    // not corrupt anything
    opStack = (stack.as_mut_ptr() as crate::stdlib::intptr_t + 16 as libc::c_int as libc::c_long
        - 1 as libc::c_int as libc::c_long
        & !(16 as libc::c_int - 1 as libc::c_int) as libc::c_long)
        as *mut libc::c_void as *mut libc::c_int;
    *opStack = 0xdeadbeef as libc::c_uint as libc::c_int;
    opStackOfs = 0 as libc::c_int as crate::stdlib::uint8_t;
    's_105: loop
    //	vm_debugLevel=2;
    // main interpreter loop, will exit when a LEAVE instruction
    // grabs the -1 program counter
    {
        let mut opcode: libc::c_int = 0;
        let mut r0: libc::c_int = 0;
        let mut r1: libc::c_int = 0;
        'c_7647: loop
        //		unsigned int	r2;
        {
            r0 = *opStack.offset(opStackOfs as isize);
            r1 = *opStack.offset(
                (opStackOfs as libc::c_int - 1 as libc::c_int) as crate::stdlib::uint8_t as isize,
            );
            loop {
                let fresh0 = programCounter;
                programCounter = programCounter + 1;
                opcode = *codeImage.offset(fresh0 as isize);
                match opcode {
                    2 => (*vm).breakCount += 1,
                    8 => {
                        opStackOfs = opStackOfs.wrapping_add(1);
                        r1 = r0;
                        let ref mut fresh1 = *opStack.offset(opStackOfs as isize);
                        *fresh1 = *codeImage.offset(programCounter as isize);
                        r0 = *fresh1;
                        programCounter += 1 as libc::c_int
                    }
                    9 => {
                        opStackOfs = opStackOfs.wrapping_add(1);
                        r1 = r0;
                        let ref mut fresh2 = *opStack.offset(opStackOfs as isize);
                        *fresh2 = *codeImage.offset(programCounter as isize) + programStack;
                        r0 = *fresh2;
                        programCounter += 1 as libc::c_int
                    }
                    29 => {
                        let ref mut fresh3 = *opStack.offset(opStackOfs as isize);
                        *fresh3 = *(&mut *image.offset((r0 & dataMask) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_int);
                        r0 = *fresh3
                    }
                    28 => {
                        let ref mut fresh4 = *opStack.offset(opStackOfs as isize);
                        *fresh4 = *(&mut *image.offset((r0 & dataMask) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_ushort)
                            as libc::c_int;
                        r0 = *fresh4
                    }
                    27 => {
                        let ref mut fresh5 = *opStack.offset(opStackOfs as isize);
                        *fresh5 = *image.offset((r0 & dataMask) as isize) as libc::c_int;
                        r0 = *fresh5
                    }
                    32 => {
                        *(&mut *image.offset((r1 & dataMask) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_int) = r0;
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        continue 'c_7647;
                    }
                    31 => {
                        *(&mut *image.offset((r1 & dataMask) as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_short) = r0 as libc::c_short;
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        continue 'c_7647;
                    }
                    30 => {
                        *image.offset((r1 & dataMask) as isize) =
                            r0 as crate::src::qcommon::q_shared::byte;
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        continue 'c_7647;
                    }
                    33 => {
                        // single byte offset from programStack
                        *(&mut *image.offset(
                            (*codeImage.offset(programCounter as isize) + programStack & dataMask)
                                as isize,
                        ) as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_int) = r0;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        programCounter += 1 as libc::c_int;
                        continue 'c_7647;
                    }
                    34 => {
                        crate::src::qcommon::vm::VM_BlockCopy(
                            r1 as libc::c_uint,
                            r0 as libc::c_uint,
                            *codeImage.offset(programCounter as isize) as crate::stddef_h::size_t,
                        );
                        programCounter += 1 as libc::c_int;
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        continue 'c_7647;
                    }
                    5 => {
                        // save current program counter
                        *(&mut *image.offset(programStack as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_int) = programCounter;
                        // jump to the location on the stack
                        programCounter = r0;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        if programCounter < 0 as libc::c_int {
                            // system call
                            let mut r: libc::c_int = 0;
                            //				int		temp;
                            // save the stack to allow recursive VM entry
                            //				temp = vm->callLevel;
                            (*vm).programStack = programStack - 4 as libc::c_int;
                            *(&mut *image.offset((programStack + 4 as libc::c_int) as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut libc::c_int) = -(1 as libc::c_int) - programCounter;
                            //VM_LogSyscalls( (int *)&image[ programStack + 4 ] );
                            // the vm has ints on the stack, we expect
                            // pointers so we might have to convert it
                            if ::std::mem::size_of::<crate::stdlib::intptr_t>() as libc::c_ulong
                                != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                            {
                                let mut argarr: [crate::stdlib::intptr_t; 16] = [0; 16];
                                let mut imagePtr: *mut libc::c_int = &mut *image
                                    .offset(programStack as isize)
                                    as *mut crate::src::qcommon::q_shared::byte
                                    as *mut libc::c_int;
                                let mut i: libc::c_int = 0;
                                i = 0 as libc::c_int;
                                while (i as libc::c_ulong)
                                    < (::std::mem::size_of::<[crate::stdlib::intptr_t; 16]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<crate::stdlib::intptr_t>()
                                                as libc::c_ulong,
                                        )
                                {
                                    imagePtr = imagePtr.offset(1);
                                    argarr[i as usize] = *imagePtr as crate::stdlib::intptr_t;
                                    i += 1
                                }
                                r = (*vm).systemCall.expect("non-null function pointer")(
                                    argarr.as_mut_ptr(),
                                ) as libc::c_int
                            } else {
                                let mut argptr: *mut crate::stdlib::intptr_t = &mut *image
                                    .offset((programStack + 4 as libc::c_int) as isize)
                                    as *mut crate::src::qcommon::q_shared::byte
                                    as *mut crate::stdlib::intptr_t;
                                r = (*vm).systemCall.expect("non-null function pointer")(argptr)
                                    as libc::c_int
                            }
                            opStackOfs = opStackOfs.wrapping_add(1);
                            *opStack.offset(opStackOfs as isize) = r;
                            programCounter = *(&mut *image.offset(programStack as isize)
                                as *mut crate::src::qcommon::q_shared::byte
                                as *mut libc::c_int)
                        } else if programCounter as libc::c_uint
                            >= (*vm).instructionCount as libc::c_uint
                        {
                            crate::src::qcommon::common::Com_Error(
                                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                b"VM program counter out of range in OP_CALL\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            programCounter =
                                *(*vm).instructionPointers.offset(programCounter as isize)
                                    as libc::c_int
                        }
                        continue 'c_7647;
                    }
                    6 => {
                        // save return value
                        // push and pop are only needed for discarded or bad function return values
                        opStackOfs = opStackOfs.wrapping_add(1);
                        continue 'c_7647;
                    }
                    7 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        continue 'c_7647;
                    }
                    3 => {
                        // get size of stack frame
                        v1 = *codeImage.offset(programCounter as isize);
                        programCounter += 1 as libc::c_int;
                        programStack -= v1;
                        continue 'c_7647;
                    }
                    4 => {
                        // remove our stack frame
                        v1 = *codeImage.offset(programCounter as isize);
                        programStack += v1;
                        // grab the saved program counter
                        programCounter = *(&mut *image.offset(programStack as isize)
                            as *mut crate::src::qcommon::q_shared::byte
                            as *mut libc::c_int);
                        // check for leaving the VM
                        if programCounter == -(1 as libc::c_int) {
                            break 's_105;
                        }
                        if programCounter as libc::c_uint >= (*vm).codeLength as libc::c_uint {
                            crate::src::qcommon::common::Com_Error(
                                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                b"VM program counter out of range in OP_LEAVE\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        continue 'c_7647;
                    }
                    10 => {
                        /*
                        ===================================================================
                        BRANCHES
                        ===================================================================
                        */
                        if r0 as libc::c_uint >= (*vm).instructionCount as libc::c_uint {
                            crate::src::qcommon::common::Com_Error(
                                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                b"VM program counter out of range in OP_JUMP\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        programCounter =
                            *(*vm).instructionPointers.offset(r0 as isize) as libc::c_int;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        continue 'c_7647;
                    }
                    11 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 == r0 {
                            current_block = 4691324637564808323;
                            break;
                        } else {
                            current_block = 562309032768341766;
                            break;
                        }
                    }
                    12 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 != r0 {
                            current_block = 3812947724376655173;
                            break;
                        } else {
                            current_block = 3575278370434307847;
                            break;
                        }
                    }
                    13 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 < r0 {
                            current_block = 2522825242109451841;
                            break;
                        } else {
                            current_block = 8304106758420804164;
                            break;
                        }
                    }
                    14 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 <= r0 {
                            current_block = 8533724845731836612;
                            break;
                        } else {
                            current_block = 16667286137552459707;
                            break;
                        }
                    }
                    15 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 > r0 {
                            current_block = 7728257318064351663;
                            break;
                        } else {
                            current_block = 18039443766442739006;
                            break;
                        }
                    }
                    16 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 >= r0 {
                            current_block = 5590933039760577279;
                            break;
                        } else {
                            current_block = 10357520176418200368;
                            break;
                        }
                    }
                    17 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if (r1 as libc::c_uint) < r0 as libc::c_uint {
                            current_block = 12608488225262500095;
                            break;
                        } else {
                            current_block = 14114759727632161892;
                            break;
                        }
                    }
                    18 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 as libc::c_uint <= r0 as libc::c_uint {
                            current_block = 5089124893069931607;
                            break;
                        } else {
                            current_block = 7034501744547627146;
                            break;
                        }
                    }
                    19 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 as libc::c_uint > r0 as libc::c_uint {
                            current_block = 4871270227279186910;
                            break;
                        } else {
                            current_block = 5908482871227205451;
                            break;
                        }
                    }
                    20 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if r1 as libc::c_uint >= r0 as libc::c_uint {
                            current_block = 15993708482136914563;
                            break;
                        } else {
                            current_block = 6938158527927677584;
                            break;
                        }
                    }
                    21 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) == *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 6186816898867308296;
                            break;
                        } else {
                            current_block = 7173345243791314703;
                            break;
                        }
                    }
                    22 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) != *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 11226769033371074123;
                            break;
                        } else {
                            current_block = 4183419379601546972;
                            break;
                        }
                    }
                    23 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) < *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 4122836492991094814;
                            break;
                        } else {
                            current_block = 2413388577390654262;
                            break;
                        }
                    }
                    24 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) <= *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 654039154479240366;
                            break;
                        } else {
                            current_block = 6957654774345280688;
                            break;
                        }
                    }
                    25 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) > *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 2346768750020253347;
                            break;
                        } else {
                            current_block = 11814324130289762492;
                            break;
                        }
                    }
                    26 => {
                        opStackOfs = (opStackOfs as libc::c_int - 2 as libc::c_int)
                            as crate::stdlib::uint8_t;
                        if *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 1 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) >= *(opStack as *mut libc::c_float).offset(
                            (opStackOfs as libc::c_int + 2 as libc::c_int) as crate::stdlib::uint8_t
                                as isize,
                        ) {
                            current_block = 14187386403465544025;
                            break;
                        } else {
                            current_block = 8551376836414271792;
                            break;
                        }
                    }
                    37 => {
                        //===================================================================
                        *opStack.offset(opStackOfs as isize) = -r0; //vm->instructionPointers[r2];
                        continue 'c_7647; //vm->instructionPointers[r2];
                    }
                    38 => {
                        opStackOfs = opStackOfs.wrapping_sub(1); //vm->instructionPointers[r2];
                        *opStack.offset(opStackOfs as isize) = r1 + r0; //vm->instructionPointers[r2];
                        continue 'c_7647; //vm->instructionPointers[r2];
                    }
                    39 => {
                        opStackOfs = opStackOfs.wrapping_sub(1); //vm->instructionPointers[r2];
                        *opStack.offset(opStackOfs as isize) = r1 - r0; //vm->instructionPointers[r2];
                        continue 'c_7647; //vm->instructionPointers[r2];
                    }
                    40 => {
                        opStackOfs = opStackOfs.wrapping_sub(1); //vm->instructionPointers[r2];
                        *opStack.offset(opStackOfs as isize) = r1 / r0; //vm->instructionPointers[r2];
                        continue 'c_7647; //vm->instructionPointers[r2];
                    }
                    41 => {
                        opStackOfs = opStackOfs.wrapping_sub(1); //vm->instructionPointers[r2];
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint).wrapping_div(r0 as libc::c_uint) as libc::c_int; //vm->instructionPointers[r2];
                        continue 'c_7647; //vm->instructionPointers[r2];
                    }
                    42 => {
                        opStackOfs = opStackOfs.wrapping_sub(1); //vm->instructionPointers[r2];
                        *opStack.offset(opStackOfs as isize) = r1 % r0; //vm->instructionPointers[r2];
                        continue 'c_7647;
                    }
                    43 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint).wrapping_rem(r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    44 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = r1 * r0;
                        continue 'c_7647;
                    }
                    45 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint).wrapping_mul(r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    46 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint & r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    47 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint | r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    48 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint ^ r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    49 => {
                        *opStack.offset(opStackOfs as isize) = !(r0 as libc::c_uint) as libc::c_int;
                        continue 'c_7647;
                    }
                    50 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = r1 << r0;
                        continue 'c_7647;
                    }
                    51 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = r1 >> r0;
                        continue 'c_7647;
                    }
                    52 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) =
                            (r1 as libc::c_uint >> r0) as libc::c_int;
                        continue 'c_7647;
                    }
                    53 => {
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            -*(opStack as *mut libc::c_float).offset(opStackOfs as isize);
                        continue 'c_7647;
                    }
                    54 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            *(opStack as *mut libc::c_float).offset(opStackOfs as isize)
                                + *(opStack as *mut libc::c_float).offset(
                                    (opStackOfs as libc::c_int + 1 as libc::c_int)
                                        as crate::stdlib::uint8_t
                                        as isize,
                                );
                        continue 'c_7647;
                    }
                    55 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            *(opStack as *mut libc::c_float).offset(opStackOfs as isize)
                                - *(opStack as *mut libc::c_float).offset(
                                    (opStackOfs as libc::c_int + 1 as libc::c_int)
                                        as crate::stdlib::uint8_t
                                        as isize,
                                );
                        continue 'c_7647;
                    }
                    56 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            *(opStack as *mut libc::c_float).offset(opStackOfs as isize)
                                / *(opStack as *mut libc::c_float).offset(
                                    (opStackOfs as libc::c_int + 1 as libc::c_int)
                                        as crate::stdlib::uint8_t
                                        as isize,
                                );
                        continue 'c_7647;
                    }
                    57 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            *(opStack as *mut libc::c_float).offset(opStackOfs as isize)
                                * *(opStack as *mut libc::c_float).offset(
                                    (opStackOfs as libc::c_int + 1 as libc::c_int)
                                        as crate::stdlib::uint8_t
                                        as isize,
                                );
                        continue 'c_7647;
                    }
                    58 => {
                        *(opStack as *mut libc::c_float).offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as libc::c_float;
                        continue 'c_7647;
                    }
                    59 => {
                        *opStack.offset(opStackOfs as isize) = crate::src::asm::ftola::qftolsse(
                            *(opStack as *mut libc::c_float).offset(opStackOfs as isize),
                        )
                            as libc::c_int;
                        continue 'c_7647;
                    }
                    35 => {
                        *opStack.offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as libc::c_schar as libc::c_int;
                        continue 'c_7647;
                    }
                    36 => {
                        *opStack.offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as libc::c_short as libc::c_int;
                        continue 'c_7647;
                    }
                    _ => {
                        break 'c_7647;
                    }
                }
            }
            match current_block {
                4122836492991094814 => programCounter = *codeImage.offset(programCounter as isize),
                6186816898867308296 => programCounter = *codeImage.offset(programCounter as isize),
                15993708482136914563 => programCounter = *codeImage.offset(programCounter as isize),
                4871270227279186910 => programCounter = *codeImage.offset(programCounter as isize),
                5089124893069931607 => programCounter = *codeImage.offset(programCounter as isize),
                12608488225262500095 => programCounter = *codeImage.offset(programCounter as isize),
                5590933039760577279 => programCounter = *codeImage.offset(programCounter as isize),
                7728257318064351663 => programCounter = *codeImage.offset(programCounter as isize),
                4691324637564808323 => programCounter = *codeImage.offset(programCounter as isize),
                3812947724376655173 => programCounter = *codeImage.offset(programCounter as isize),
                2522825242109451841 => programCounter = *codeImage.offset(programCounter as isize),
                8533724845731836612 => programCounter = *codeImage.offset(programCounter as isize),
                562309032768341766 => programCounter += 1 as libc::c_int,
                3575278370434307847 => programCounter += 1 as libc::c_int,
                8304106758420804164 => programCounter += 1 as libc::c_int,
                16667286137552459707 => programCounter += 1 as libc::c_int,
                18039443766442739006 => programCounter += 1 as libc::c_int,
                10357520176418200368 => programCounter += 1 as libc::c_int,
                14114759727632161892 => programCounter += 1 as libc::c_int,
                7034501744547627146 => programCounter += 1 as libc::c_int,
                5908482871227205451 => programCounter += 1 as libc::c_int,
                6938158527927677584 => programCounter += 1 as libc::c_int,
                7173345243791314703 => programCounter += 1 as libc::c_int,
                4183419379601546972 => programCounter += 1 as libc::c_int,
                2413388577390654262 => programCounter += 1 as libc::c_int,
                6957654774345280688 => programCounter += 1 as libc::c_int,
                11814324130289762492 => programCounter += 1 as libc::c_int,
                8551376836414271792 => programCounter += 1 as libc::c_int,
                14187386403465544025 => programCounter = *codeImage.offset(programCounter as isize),
                2346768750020253347 => programCounter = *codeImage.offset(programCounter as isize),
                654039154479240366 => programCounter = *codeImage.offset(programCounter as isize),
                _ => programCounter = *codeImage.offset(programCounter as isize),
            }
        }
    }
    (*vm).currentlyInterpreting = crate::src::qcommon::q_shared::qfalse;
    if opStackOfs as libc::c_int != 1 as libc::c_int
        || *opStack as libc::c_uint != 0xdeadbeef as libc::c_uint
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Interpreter error: opStack[0] = %X, opStackOfs = %d\x00" as *const u8
                as *const libc::c_char,
            *opStack.offset(0 as libc::c_int as isize),
            opStackOfs as libc::c_int,
        );
    }
    (*vm).programStack = stackOnEntry;
    // return the result
    return *opStack.offset(opStackOfs as isize);
}

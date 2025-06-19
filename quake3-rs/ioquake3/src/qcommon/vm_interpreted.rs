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

unsafe extern "C" fn loadWord(mut addr: *mut libc::c_void) -> i32 {
    let mut word: i32 = 0;
    crate::stdlib::memcpy(
        &mut word as *mut i32 as *mut libc::c_void,
        addr,
        4 as i32 as libc::c_ulong,
    );
    return word;
}
#[no_mangle]

pub unsafe extern "C" fn VM_Indent(mut vm: *mut vm_t) -> *mut libc::c_char {
    static mut string: *mut libc::c_char = b"                                        \x00"
        as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    if (*vm).callLevel > 20 as i32 {
        return string;
    }
    return string.offset((2 as i32 * (20 as i32 - (*vm).callLevel)) as isize);
}
#[no_mangle]

pub unsafe extern "C" fn VM_StackTrace(
    mut vm: *mut vm_t,
    mut programCounter: i32,
    mut programStack: i32,
) {
    let mut count: i32 = 0;
    count = 0 as i32;
    loop {
        Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            VM_ValueToSymbol(
                vm as *mut vm_s,
                programCounter,
            ),
        );
        programStack = *(&mut *(*vm).dataBase.offset((programStack + 4 as i32) as isize)
            as *mut byte as *mut i32);
        programCounter = *(&mut *(*vm).dataBase.offset(programStack as isize)
            as *mut byte as *mut i32);
        if !(programCounter != -(1 as i32) && {
            count += 1;
            (count) < 32 as i32
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
    mut vm: *mut vm_t,
    mut header: *mut vmHeader_t,
) {
    let mut op: i32 = 0; // we're now int aligned
    let mut byte_pc: i32 = 0;
    let mut int_pc: i32 = 0;
    let mut code: *mut byte =
        0 as *mut byte;
    let mut instruction: i32 = 0;
    let mut codeBase: *mut i32 = 0 as *mut i32;
    (*vm).codeBase = Hunk_Alloc(
        (*vm).codeLength * 4 as i32,
        h_high,
    ) as *mut byte;
    //	memcpy( vm->codeBase, (byte *)header + header->codeOffset, vm->codeLength );
    // we don't need to translate the instructions, but we still need
    // to find each instructions starting point for jumps
    byte_pc = 0 as i32;
    int_pc = byte_pc;
    instruction = 0 as i32;
    code =
        (header as *mut byte).offset((*header).codeOffset as isize);
    codeBase = (*vm).codeBase as *mut i32;
    // Copy and expand instructions to words while building instruction table
    while instruction < (*header).instructionCount {
        *(*vm).instructionPointers.offset(instruction as isize) = int_pc as intptr_t;
        instruction += 1;
        op = *code.offset(byte_pc as isize) as i32;
        *codeBase.offset(int_pc as isize) = op;
        if byte_pc > (*header).codeLength {
            Com_Error(
                ERR_DROP as i32,
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
                    as *mut byte
                    as *mut libc::c_void);
                byte_pc += 4 as i32;
                int_pc += 1
            }
            33 => {
                *codeBase.offset(int_pc as isize) = *code.offset(byte_pc as isize) as i32;
                byte_pc += 1;
                int_pc += 1
            }
            _ => {}
        }
    }
    int_pc = 0 as i32;
    instruction = 0 as i32;
    // Now that the code has been expanded to int-sized opcodes, we'll translate instruction index
    //into an index into codeBase[], which contains opcodes and operands.
    while instruction < (*header).instructionCount {
        op = *codeBase.offset(int_pc as isize);
        instruction += 1;
        int_pc += 1;
        match op {
            11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 => {
                // These ops need to translate addresses in jumps from instruction index to int index
                if *codeBase.offset(int_pc as isize) < 0 as i32
                    || *codeBase.offset(int_pc as isize) > (*vm).instructionCount
                {
                    Com_Error(
                        ERR_DROP as i32,
                        b"VM_PrepareInterpreter: Jump to invalid instruction number\x00"
                            as *const u8 as *const libc::c_char,
                    );
                }
                // codeBase[pc] is the instruction index. Convert that into an offset into
                //the int-aligned codeBase[] by the lookup table.
                *codeBase.offset(int_pc as isize) = *(*vm)
                    .instructionPointers
                    .offset(*codeBase.offset(int_pc as isize) as isize)
                    as i32;
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
    mut vm: *mut vm_t,
    mut args: *mut i32,
) -> i32 {
    let mut current_block: u64;
    let mut stack: [byte; 1039] = [0; 1039];
    let mut opStack: *mut i32 = 0 as *mut i32;
    let mut opStackOfs: uint8_t = 0;
    let mut programCounter: i32 = 0;
    let mut programStack: i32 = 0;
    let mut stackOnEntry: i32 = 0;
    let mut image: *mut byte =
        0 as *mut byte;
    let mut codeImage: *mut i32 = 0 as *mut i32;
    let mut v1: i32 = 0;
    let mut dataMask: i32 = 0;
    let mut arg: i32 = 0;
    // interpret the code
    (*vm).currentlyInterpreting = qtrue;
    // we might be called recursively, so this might not be the very top
    stackOnEntry = (*vm).programStack;
    programStack = stackOnEntry;
    // set up the stack frame
    image = (*vm).dataBase; // return stack
    codeImage = (*vm).codeBase as *mut i32; // will terminate the loop on return
    dataMask = (*vm).dataMask;
    programCounter = 0 as i32;
    programStack -= 8 as i32 + 4 as i32 * 13 as i32;
    arg = 0 as i32;
    while arg < 13 as i32 {
        *(&mut *image.offset((programStack + 8 as i32 + arg * 4 as i32) as isize)
            as *mut byte as *mut i32) = *args.offset(arg as isize);
        arg += 1
    }
    *(&mut *image.offset((programStack + 4 as i32) as isize)
        as *mut byte as *mut i32) = 0 as i32;
    *(&mut *image.offset(programStack as isize) as *mut byte
        as *mut i32) = -(1 as i32);
    VM_Debug(0 as i32);
    // leave a free spot at start of stack so
    // that as long as opStack is valid, opStack-1 will
    // not corrupt anything
    opStack = (stack.as_mut_ptr() as intptr_t + 16 as i32 as isize
        - 1 as i32 as isize
        & !(16 as i32 - 1 as i32) as isize) as *mut libc::c_void as *mut i32;
    *opStack = 0xdeadbeef as u32 as i32;
    opStackOfs = 0 as i32 as uint8_t;
    's_105: loop
    //	vm_debugLevel=2;
    // main interpreter loop, will exit when a LEAVE instruction
    // grabs the -1 program counter
    {
        let mut opcode: i32 = 0;
        let mut r0: i32 = 0;
        let mut r1: i32 = 0;
        'c_7647: loop
        //		unsigned int	r2;
        {
            r0 = *opStack.offset(opStackOfs as isize);
            r1 = *opStack.offset((opStackOfs as i32 - 1 as i32) as uint8_t as isize);
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
                        programCounter += 1 as i32
                    }
                    9 => {
                        opStackOfs = opStackOfs.wrapping_add(1);
                        r1 = r0;
                        let ref mut fresh2 = *opStack.offset(opStackOfs as isize);
                        *fresh2 = *codeImage.offset(programCounter as isize) + programStack;
                        r0 = *fresh2;
                        programCounter += 1 as i32
                    }
                    29 => {
                        let ref mut fresh3 = *opStack.offset(opStackOfs as isize);
                        *fresh3 = *(&mut *image.offset((r0 & dataMask) as isize)
                            as *mut byte
                            as *mut i32);
                        r0 = *fresh3
                    }
                    28 => {
                        let ref mut fresh4 = *opStack.offset(opStackOfs as isize);
                        *fresh4 = *(&mut *image.offset((r0 & dataMask) as isize)
                            as *mut byte
                            as *mut u16) as i32;
                        r0 = *fresh4
                    }
                    27 => {
                        let ref mut fresh5 = *opStack.offset(opStackOfs as isize);
                        *fresh5 = *image.offset((r0 & dataMask) as isize) as i32;
                        r0 = *fresh5
                    }
                    32 => {
                        *(&mut *image.offset((r1 & dataMask) as isize)
                            as *mut byte
                            as *mut i32) = r0;
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        continue 'c_7647;
                    }
                    31 => {
                        *(&mut *image.offset((r1 & dataMask) as isize)
                            as *mut byte
                            as *mut i16) = r0 as i16;
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        continue 'c_7647;
                    }
                    30 => {
                        *image.offset((r1 & dataMask) as isize) =
                            r0 as byte;
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        continue 'c_7647;
                    }
                    33 => {
                        // single byte offset from programStack
                        *(&mut *image.offset(
                            (*codeImage.offset(programCounter as isize) + programStack & dataMask)
                                as isize,
                        ) as *mut byte
                            as *mut i32) = r0;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        programCounter += 1 as i32;
                        continue 'c_7647;
                    }
                    34 => {
                        VM_BlockCopy(
                            r1 as u32,
                            r0 as u32,
                            *codeImage.offset(programCounter as isize) as size_t,
                        );
                        programCounter += 1 as i32;
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        continue 'c_7647;
                    }
                    5 => {
                        // save current program counter
                        *(&mut *image.offset(programStack as isize)
                            as *mut byte
                            as *mut i32) = programCounter;
                        // jump to the location on the stack
                        programCounter = r0;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        if programCounter < 0 as i32 {
                            // system call
                            let mut r: i32 = 0;
                            //				int		temp;
                            // save the stack to allow recursive VM entry
                            //				temp = vm->callLevel;
                            (*vm).programStack = programStack - 4 as i32;
                            *(&mut *image.offset((programStack + 4 as i32) as isize)
                                as *mut byte
                                as *mut i32) = -(1 as i32) - programCounter;
                            //VM_LogSyscalls( (int *)&image[ programStack + 4 ] );
                            // the vm has ints on the stack, we expect
                            // pointers so we might have to convert it
                            if ::std::mem::size_of::<intptr_t>() as libc::c_ulong
                                != ::std::mem::size_of::<i32>() as libc::c_ulong
                            {
                                let mut argarr: [intptr_t; 16] = [0; 16];
                                let mut imagePtr: *mut i32 = &mut *image
                                    .offset(programStack as isize)
                                    as *mut byte
                                    as *mut i32;
                                let mut i: i32 = 0;
                                i = 0 as i32;
                                while (i as libc::c_ulong)
                                    < (::std::mem::size_of::<[intptr_t; 16]>()
                                        as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<intptr_t>()
                                                as libc::c_ulong,
                                        )
                                {
                                    imagePtr = imagePtr.offset(1);
                                    argarr[i as usize] = *imagePtr as intptr_t;
                                    i += 1
                                }
                                r = (*vm).systemCall.expect("non-null function pointer")(
                                    argarr.as_mut_ptr(),
                                ) as i32
                            } else {
                                let mut argptr: *mut intptr_t = &mut *image
                                    .offset((programStack + 4 as i32) as isize)
                                    as *mut byte
                                    as *mut intptr_t;
                                r = (*vm).systemCall.expect("non-null function pointer")(argptr)
                                    as i32
                            }
                            opStackOfs = opStackOfs.wrapping_add(1);
                            *opStack.offset(opStackOfs as isize) = r;
                            programCounter = *(&mut *image.offset(programStack as isize)
                                as *mut byte
                                as *mut i32)
                        } else if programCounter as u32 >= (*vm).instructionCount as u32 {
                            Com_Error(
                                ERR_DROP as i32,
                                b"VM program counter out of range in OP_CALL\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            programCounter =
                                *(*vm).instructionPointers.offset(programCounter as isize) as i32
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
                        programCounter += 1 as i32;
                        programStack -= v1;
                        continue 'c_7647;
                    }
                    4 => {
                        // remove our stack frame
                        v1 = *codeImage.offset(programCounter as isize);
                        programStack += v1;
                        // grab the saved program counter
                        programCounter = *(&mut *image.offset(programStack as isize)
                            as *mut byte
                            as *mut i32);
                        // check for leaving the VM
                        if programCounter == -(1 as i32) {
                            break 's_105;
                        }
                        if programCounter as u32 >= (*vm).codeLength as u32 {
                            Com_Error(
                                ERR_DROP as i32,
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
                        if r0 as u32 >= (*vm).instructionCount as u32 {
                            Com_Error(
                                ERR_DROP as i32,
                                b"VM program counter out of range in OP_JUMP\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        programCounter = *(*vm).instructionPointers.offset(r0 as isize) as i32;
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        continue 'c_7647;
                    }
                    11 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 == r0 {
                            current_block = 4691324637564808323;
                            break;
                        } else {
                            current_block = 562309032768341766;
                            break;
                        }
                    }
                    12 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 != r0 {
                            current_block = 3812947724376655173;
                            break;
                        } else {
                            current_block = 3575278370434307847;
                            break;
                        }
                    }
                    13 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 < r0 {
                            current_block = 2522825242109451841;
                            break;
                        } else {
                            current_block = 8304106758420804164;
                            break;
                        }
                    }
                    14 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 <= r0 {
                            current_block = 8533724845731836612;
                            break;
                        } else {
                            current_block = 16667286137552459707;
                            break;
                        }
                    }
                    15 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 > r0 {
                            current_block = 7728257318064351663;
                            break;
                        } else {
                            current_block = 18039443766442739006;
                            break;
                        }
                    }
                    16 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 >= r0 {
                            current_block = 5590933039760577279;
                            break;
                        } else {
                            current_block = 10357520176418200368;
                            break;
                        }
                    }
                    17 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if (r1 as u32) < r0 as u32 {
                            current_block = 12608488225262500095;
                            break;
                        } else {
                            current_block = 14114759727632161892;
                            break;
                        }
                    }
                    18 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 as u32 <= r0 as u32 {
                            current_block = 5089124893069931607;
                            break;
                        } else {
                            current_block = 7034501744547627146;
                            break;
                        }
                    }
                    19 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 as u32 > r0 as u32 {
                            current_block = 4871270227279186910;
                            break;
                        } else {
                            current_block = 5908482871227205451;
                            break;
                        }
                    }
                    20 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if r1 as u32 >= r0 as u32 {
                            current_block = 15993708482136914563;
                            break;
                        } else {
                            current_block = 6938158527927677584;
                            break;
                        }
                    }
                    21 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) == *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
                        ) {
                            current_block = 6186816898867308296;
                            break;
                        } else {
                            current_block = 7173345243791314703;
                            break;
                        }
                    }
                    22 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) != *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
                        ) {
                            current_block = 11226769033371074123;
                            break;
                        } else {
                            current_block = 4183419379601546972;
                            break;
                        }
                    }
                    23 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) < *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
                        ) {
                            current_block = 4122836492991094814;
                            break;
                        } else {
                            current_block = 2413388577390654262;
                            break;
                        }
                    }
                    24 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) <= *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
                        ) {
                            current_block = 654039154479240366;
                            break;
                        } else {
                            current_block = 6957654774345280688;
                            break;
                        }
                    }
                    25 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) > *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
                        ) {
                            current_block = 2346768750020253347;
                            break;
                        } else {
                            current_block = 11814324130289762492;
                            break;
                        }
                    }
                    26 => {
                        opStackOfs = (opStackOfs as i32 - 2 as i32) as uint8_t;
                        if *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                        ) >= *(opStack as *mut f32).offset(
                            (opStackOfs as i32 + 2 as i32) as uint8_t as isize,
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
                            (r1 as u32).wrapping_div(r0 as u32) as i32; //vm->instructionPointers[r2];
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
                            (r1 as u32).wrapping_rem(r0 as u32) as i32;
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
                            (r1 as u32).wrapping_mul(r0 as u32) as i32;
                        continue 'c_7647;
                    }
                    46 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = (r1 as u32 & r0 as u32) as i32;
                        continue 'c_7647;
                    }
                    47 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = (r1 as u32 | r0 as u32) as i32;
                        continue 'c_7647;
                    }
                    48 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *opStack.offset(opStackOfs as isize) = (r1 as u32 ^ r0 as u32) as i32;
                        continue 'c_7647;
                    }
                    49 => {
                        *opStack.offset(opStackOfs as isize) = !(r0 as u32) as i32;
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
                        *opStack.offset(opStackOfs as isize) = (r1 as u32 >> r0) as i32;
                        continue 'c_7647;
                    }
                    53 => {
                        *(opStack as *mut f32).offset(opStackOfs as isize) =
                            -*(opStack as *mut f32).offset(opStackOfs as isize);
                        continue 'c_7647;
                    }
                    54 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut f32).offset(opStackOfs as isize) = *(opStack as *mut f32)
                            .offset(opStackOfs as isize)
                            + *(opStack as *mut f32).offset(
                                (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                            );
                        continue 'c_7647;
                    }
                    55 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut f32).offset(opStackOfs as isize) = *(opStack as *mut f32)
                            .offset(opStackOfs as isize)
                            - *(opStack as *mut f32).offset(
                                (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                            );
                        continue 'c_7647;
                    }
                    56 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut f32).offset(opStackOfs as isize) = *(opStack as *mut f32)
                            .offset(opStackOfs as isize)
                            / *(opStack as *mut f32).offset(
                                (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                            );
                        continue 'c_7647;
                    }
                    57 => {
                        opStackOfs = opStackOfs.wrapping_sub(1);
                        *(opStack as *mut f32).offset(opStackOfs as isize) = *(opStack as *mut f32)
                            .offset(opStackOfs as isize)
                            * *(opStack as *mut f32).offset(
                                (opStackOfs as i32 + 1 as i32) as uint8_t as isize,
                            );
                        continue 'c_7647;
                    }
                    58 => {
                        *(opStack as *mut f32).offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as f32;
                        continue 'c_7647;
                    }
                    59 => {
                        *opStack.offset(opStackOfs as isize) = qftolsse(
                            *(opStack as *mut f32).offset(opStackOfs as isize),
                        ) as i32;
                        continue 'c_7647;
                    }
                    35 => {
                        *opStack.offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as i8 as i32;
                        continue 'c_7647;
                    }
                    36 => {
                        *opStack.offset(opStackOfs as isize) =
                            *opStack.offset(opStackOfs as isize) as i16 as i32;
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
                562309032768341766 => programCounter += 1 as i32,
                3575278370434307847 => programCounter += 1 as i32,
                8304106758420804164 => programCounter += 1 as i32,
                16667286137552459707 => programCounter += 1 as i32,
                18039443766442739006 => programCounter += 1 as i32,
                10357520176418200368 => programCounter += 1 as i32,
                14114759727632161892 => programCounter += 1 as i32,
                7034501744547627146 => programCounter += 1 as i32,
                5908482871227205451 => programCounter += 1 as i32,
                6938158527927677584 => programCounter += 1 as i32,
                7173345243791314703 => programCounter += 1 as i32,
                4183419379601546972 => programCounter += 1 as i32,
                2413388577390654262 => programCounter += 1 as i32,
                6957654774345280688 => programCounter += 1 as i32,
                11814324130289762492 => programCounter += 1 as i32,
                8551376836414271792 => programCounter += 1 as i32,
                14187386403465544025 => programCounter = *codeImage.offset(programCounter as isize),
                2346768750020253347 => programCounter = *codeImage.offset(programCounter as isize),
                654039154479240366 => programCounter = *codeImage.offset(programCounter as isize),
                _ => programCounter = *codeImage.offset(programCounter as isize),
            }
        }
    }
    (*vm).currentlyInterpreting = qfalse;
    if opStackOfs as i32 != 1 as i32 || *opStack as u32 != 0xdeadbeef as u32 {
        Com_Error(
            ERR_DROP as i32,
            b"Interpreter error: opStack[0] = %X, opStackOfs = %d\x00" as *const u8
                as *const libc::c_char,
            *opStack.offset(0 as i32 as isize),
            opStackOfs as i32,
        );
    }
    (*vm).programStack = stackOnEntry;
    // return the result
    return *opStack.offset(opStackOfs as isize);
}

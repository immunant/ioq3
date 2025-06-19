use ::libc;
/*
===========================================================================
Copyright (C) 2011 Thilo Schulz <thilo@tjps.eu>

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

static mut fpucw: libc::c_ushort = 0xc7f as libc::c_int as libc::c_ushort;
/*
 * GNU inline asm ftol conversion functions using SSE or FPU
 */
#[no_mangle]

pub unsafe extern "C" fn qftolsse(mut f: libc::c_float) -> libc::c_long {
    let mut retval: libc::c_long = 0;
    asm!("cvttss2si $1, $0\n" : "=r" (retval) : "x" (f) : : "volatile");
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn qvmftolsse() -> libc::c_int {
    let mut retval: libc::c_int = 0;
    asm!("movss (%rdi, %rbx, 4), %xmm0\ncvttss2si %xmm0, $0\n" : "=r" (retval) : :
     "xmm0" : "volatile");
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn qftolx87(mut f: libc::c_float) -> libc::c_long {
    let mut retval: libc::c_long = 0;
    let mut oldcw: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    asm!("fnstcw $2\nfldcw $3\nflds $1\nfistpl $1\nfldcw $2\nmov $1, $0\n" : "=r"
     (retval) : "*m" (&f), "*m" (&oldcw), "*m" (&fpucw) : : "volatile");
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn qvmftolx87() -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut oldcw: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    asm!("fnstcw $1\nfldcw $2\nflds (%rdi, %rbx, 4)\nfistpl (%rdi, %rbx, 4)\nfldcw $1\nmov (%rdi, %rbx, 4), $0\n"
     : "=r" (retval) : "*m" (&oldcw), "*m" (&fpucw) : : "volatile");
    return retval;
}

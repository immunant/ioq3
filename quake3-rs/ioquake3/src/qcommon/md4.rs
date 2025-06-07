use ::libc;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::stdlib::__uint32_t;

pub use crate::stdlib::uint32_t;
/*
    mdfour.c

    An implementation of MD4 designed for use in the samba SMB
    authentication protocol

    Copyright (C) 1997-1998  Andrew Tridgell

    This program is free software; you can redistribute it and/or
    modify it under the terms of the GNU General Public License
    as published by the Free Software Foundation; either version 2
    of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

    See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to:

        Free Software Foundation, Inc.
        59 Temple Place - Suite 330
        Boston, MA  02111-1307, USA

    $Id: mdfour.c,v 1.1 2002/08/23 22:03:27 abster Exp $
*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdfour {
    pub A: crate::stdlib::uint32_t,
    pub B: crate::stdlib::uint32_t,
    pub C: crate::stdlib::uint32_t,
    pub D: crate::stdlib::uint32_t,
    pub totalN: crate::stdlib::uint32_t,
}
/* NOTE: This code makes no attempt to be fast!

   It assumes that an int is at least 32 bits long
*/

static mut m: *mut mdfour = 0 as *const mdfour as *mut mdfour;
/* this applies md4 to 64 byte chunks */

unsafe extern "C" fn mdfour64(mut M: *mut crate::stdlib::uint32_t) {
    let mut j: libc::c_int = 0;
    let mut AA: crate::stdlib::uint32_t = 0;
    let mut BB: crate::stdlib::uint32_t = 0;
    let mut CC: crate::stdlib::uint32_t = 0;
    let mut DD: crate::stdlib::uint32_t = 0;
    let mut X: [crate::stdlib::uint32_t; 16] = [0; 16];
    let mut A: crate::stdlib::uint32_t = 0;
    let mut B: crate::stdlib::uint32_t = 0;
    let mut C: crate::stdlib::uint32_t = 0;
    let mut D: crate::stdlib::uint32_t = 0;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        X[j as usize] = *M.offset(j as isize);
        j += 1
    }
    A = (*m).A;
    B = (*m).B;
    C = (*m).C;
    D = (*m).D;
    AA = A;
    BB = B;
    CC = C;
    DD = D;
    A = A
        .wrapping_add(B & C | !B & D)
        .wrapping_add(X[0 as libc::c_int as usize])
        << 3 as libc::c_int
        | A.wrapping_add(B & C | !B & D)
            .wrapping_add(X[0 as libc::c_int as usize])
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | !A & C)
        .wrapping_add(X[1 as libc::c_int as usize])
        << 7 as libc::c_int
        | D.wrapping_add(A & B | !A & C)
            .wrapping_add(X[1 as libc::c_int as usize])
            >> 32 as libc::c_int - 7 as libc::c_int;
    C = C
        .wrapping_add(D & A | !D & B)
        .wrapping_add(X[2 as libc::c_int as usize])
        << 11 as libc::c_int
        | C.wrapping_add(D & A | !D & B)
            .wrapping_add(X[2 as libc::c_int as usize])
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C & D | !C & A)
        .wrapping_add(X[3 as libc::c_int as usize])
        << 19 as libc::c_int
        | B.wrapping_add(C & D | !C & A)
            .wrapping_add(X[3 as libc::c_int as usize])
            >> 32 as libc::c_int - 19 as libc::c_int;
    A = A
        .wrapping_add(B & C | !B & D)
        .wrapping_add(X[4 as libc::c_int as usize])
        << 3 as libc::c_int
        | A.wrapping_add(B & C | !B & D)
            .wrapping_add(X[4 as libc::c_int as usize])
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | !A & C)
        .wrapping_add(X[5 as libc::c_int as usize])
        << 7 as libc::c_int
        | D.wrapping_add(A & B | !A & C)
            .wrapping_add(X[5 as libc::c_int as usize])
            >> 32 as libc::c_int - 7 as libc::c_int;
    C = C
        .wrapping_add(D & A | !D & B)
        .wrapping_add(X[6 as libc::c_int as usize])
        << 11 as libc::c_int
        | C.wrapping_add(D & A | !D & B)
            .wrapping_add(X[6 as libc::c_int as usize])
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C & D | !C & A)
        .wrapping_add(X[7 as libc::c_int as usize])
        << 19 as libc::c_int
        | B.wrapping_add(C & D | !C & A)
            .wrapping_add(X[7 as libc::c_int as usize])
            >> 32 as libc::c_int - 19 as libc::c_int;
    A = A
        .wrapping_add(B & C | !B & D)
        .wrapping_add(X[8 as libc::c_int as usize])
        << 3 as libc::c_int
        | A.wrapping_add(B & C | !B & D)
            .wrapping_add(X[8 as libc::c_int as usize])
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | !A & C)
        .wrapping_add(X[9 as libc::c_int as usize])
        << 7 as libc::c_int
        | D.wrapping_add(A & B | !A & C)
            .wrapping_add(X[9 as libc::c_int as usize])
            >> 32 as libc::c_int - 7 as libc::c_int;
    C = C
        .wrapping_add(D & A | !D & B)
        .wrapping_add(X[10 as libc::c_int as usize])
        << 11 as libc::c_int
        | C.wrapping_add(D & A | !D & B)
            .wrapping_add(X[10 as libc::c_int as usize])
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C & D | !C & A)
        .wrapping_add(X[11 as libc::c_int as usize])
        << 19 as libc::c_int
        | B.wrapping_add(C & D | !C & A)
            .wrapping_add(X[11 as libc::c_int as usize])
            >> 32 as libc::c_int - 19 as libc::c_int;
    A = A
        .wrapping_add(B & C | !B & D)
        .wrapping_add(X[12 as libc::c_int as usize])
        << 3 as libc::c_int
        | A.wrapping_add(B & C | !B & D)
            .wrapping_add(X[12 as libc::c_int as usize])
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | !A & C)
        .wrapping_add(X[13 as libc::c_int as usize])
        << 7 as libc::c_int
        | D.wrapping_add(A & B | !A & C)
            .wrapping_add(X[13 as libc::c_int as usize])
            >> 32 as libc::c_int - 7 as libc::c_int;
    C = C
        .wrapping_add(D & A | !D & B)
        .wrapping_add(X[14 as libc::c_int as usize])
        << 11 as libc::c_int
        | C.wrapping_add(D & A | !D & B)
            .wrapping_add(X[14 as libc::c_int as usize])
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C & D | !C & A)
        .wrapping_add(X[15 as libc::c_int as usize])
        << 19 as libc::c_int
        | B.wrapping_add(C & D | !C & A)
            .wrapping_add(X[15 as libc::c_int as usize])
            >> 32 as libc::c_int - 19 as libc::c_int;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[0 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[0 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[4 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 5 as libc::c_int
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[4 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 5 as libc::c_int;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[8 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[8 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[12 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 13 as libc::c_int
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[12 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 13 as libc::c_int;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[1 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[1 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[5 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 5 as libc::c_int
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[5 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 5 as libc::c_int;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[9 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[9 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[13 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 13 as libc::c_int
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[13 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 13 as libc::c_int;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[2 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[2 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[6 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 5 as libc::c_int
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[6 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 5 as libc::c_int;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[10 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[10 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[14 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 13 as libc::c_int
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[14 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 13 as libc::c_int;
    A = A
        .wrapping_add(B & C | B & D | C & D)
        .wrapping_add(X[3 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B & C | B & D | C & D)
            .wrapping_add(X[3 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A & B | A & C | B & C)
        .wrapping_add(X[7 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 5 as libc::c_int
        | D.wrapping_add(A & B | A & C | B & C)
            .wrapping_add(X[7 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 5 as libc::c_int;
    C = C
        .wrapping_add(D & A | D & B | A & B)
        .wrapping_add(X[11 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | C.wrapping_add(D & A | D & B | A & B)
            .wrapping_add(X[11 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    B = B
        .wrapping_add(C & D | C & A | D & A)
        .wrapping_add(X[15 as libc::c_int as usize])
        .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
        << 13 as libc::c_int
        | B.wrapping_add(C & D | C & A | D & A)
            .wrapping_add(X[15 as libc::c_int as usize])
            .wrapping_add(0x5a827999 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 13 as libc::c_int;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[0 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[0 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[8 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[8 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[4 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 11 as libc::c_int
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[4 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[12 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 15 as libc::c_int
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[12 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 15 as libc::c_int;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[2 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[2 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[10 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[10 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[6 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 11 as libc::c_int
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[6 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[14 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 15 as libc::c_int
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[14 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 15 as libc::c_int;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[1 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[1 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[9 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[9 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[5 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 11 as libc::c_int
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[5 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[13 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 15 as libc::c_int
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[13 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 15 as libc::c_int;
    A = A
        .wrapping_add(B ^ C ^ D)
        .wrapping_add(X[3 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 3 as libc::c_int
        | A.wrapping_add(B ^ C ^ D)
            .wrapping_add(X[3 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 3 as libc::c_int;
    D = D
        .wrapping_add(A ^ B ^ C)
        .wrapping_add(X[11 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 9 as libc::c_int
        | D.wrapping_add(A ^ B ^ C)
            .wrapping_add(X[11 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 9 as libc::c_int;
    C = C
        .wrapping_add(D ^ A ^ B)
        .wrapping_add(X[7 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 11 as libc::c_int
        | C.wrapping_add(D ^ A ^ B)
            .wrapping_add(X[7 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 11 as libc::c_int;
    B = B
        .wrapping_add(C ^ D ^ A)
        .wrapping_add(X[15 as libc::c_int as usize])
        .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
        << 15 as libc::c_int
        | B.wrapping_add(C ^ D ^ A)
            .wrapping_add(X[15 as libc::c_int as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as libc::c_uint)
            >> 32 as libc::c_int - 15 as libc::c_int;
    A = (A as libc::c_uint).wrapping_add(AA) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    B = (B as libc::c_uint).wrapping_add(BB) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    C = (C as libc::c_uint).wrapping_add(CC) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    D = (D as libc::c_uint).wrapping_add(DD) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        X[j as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
        j += 1
    }
    (*m).A = A;
    (*m).B = B;
    (*m).C = C;
    (*m).D = D;
}

unsafe extern "C" fn copy64(
    mut M: *mut crate::stdlib::uint32_t,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        *M.offset(i as isize) = (*in_0.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize)
            as crate::stdlib::uint32_t)
            << 24 as libc::c_int
            | (*in_0.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
                as crate::stdlib::uint32_t)
                << 16 as libc::c_int
            | (*in_0.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize)
                as crate::stdlib::uint32_t)
                << 8 as libc::c_int
            | (*in_0.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
                as crate::stdlib::uint32_t)
                << 0 as libc::c_int;
        i += 1
    }
}

unsafe extern "C" fn copy4(
    mut out: *mut crate::src::qcommon::q_shared::byte,
    mut x: crate::stdlib::uint32_t,
) {
    *out.offset(0 as libc::c_int as isize) =
        (x & 0xff as libc::c_int as libc::c_uint) as crate::src::qcommon::q_shared::byte;
    *out.offset(1 as libc::c_int as isize) = (x >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint)
        as crate::src::qcommon::q_shared::byte;
    *out.offset(2 as libc::c_int as isize) = (x >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint)
        as crate::src::qcommon::q_shared::byte;
    *out.offset(3 as libc::c_int as isize) = (x >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint)
        as crate::src::qcommon::q_shared::byte;
}
#[no_mangle]

pub unsafe extern "C" fn mdfour_begin(mut md: *mut mdfour) {
    (*md).A = 0x67452301 as libc::c_int as crate::stdlib::uint32_t;
    (*md).B = 0xefcdab89 as libc::c_uint;
    (*md).C = 0x98badcfe as libc::c_uint;
    (*md).D = 0x10325476 as libc::c_int as crate::stdlib::uint32_t;
    (*md).totalN = 0 as libc::c_int as crate::stdlib::uint32_t;
}

unsafe extern "C" fn mdfour_tail(
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut n: libc::c_int,
) {
    let mut buf: [crate::src::qcommon::q_shared::byte; 128] = [0; 128];
    let mut M: [crate::stdlib::uint32_t; 16] = [0; 16];
    let mut b: crate::stdlib::uint32_t = 0;
    (*m).totalN = ((*m).totalN as libc::c_uint).wrapping_add(n as libc::c_uint)
        as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (*m).totalN.wrapping_mul(8 as libc::c_int as libc::c_uint);
    crate::stdlib::memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        128 as libc::c_int as libc::c_ulong,
    );
    if n != 0 {
        crate::stdlib::memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            in_0 as *const libc::c_void,
            n as libc::c_ulong,
        );
    }
    buf[n as usize] = 0x80 as libc::c_int as crate::src::qcommon::q_shared::byte;
    if n <= 55 as libc::c_int {
        copy4(buf.as_mut_ptr().offset(56 as libc::c_int as isize), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
    } else {
        copy4(buf.as_mut_ptr().offset(120 as libc::c_int as isize), b);
        copy64(M.as_mut_ptr(), buf.as_mut_ptr());
        mdfour64(M.as_mut_ptr());
        copy64(
            M.as_mut_ptr(),
            buf.as_mut_ptr().offset(64 as libc::c_int as isize),
        );
        mdfour64(M.as_mut_ptr());
    };
}

unsafe extern "C" fn mdfour_update(
    mut md: *mut mdfour,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut n: libc::c_int,
) {
    let mut M: [crate::stdlib::uint32_t; 16] = [0; 16];
    m = md;
    if n == 0 as libc::c_int {
        mdfour_tail(in_0, n);
    }
    while n >= 64 as libc::c_int {
        copy64(M.as_mut_ptr(), in_0);
        mdfour64(M.as_mut_ptr());
        in_0 = in_0.offset(64 as libc::c_int as isize);
        n -= 64 as libc::c_int;
        (*m).totalN = ((*m).totalN as libc::c_uint).wrapping_add(64 as libc::c_int as libc::c_uint)
            as crate::stdlib::uint32_t as crate::stdlib::uint32_t
    }
    mdfour_tail(in_0, n);
}

unsafe extern "C" fn mdfour_result(
    mut md: *mut mdfour,
    mut out: *mut crate::src::qcommon::q_shared::byte,
) {
    copy4(out, (*md).A);
    copy4(out.offset(4 as libc::c_int as isize), (*md).B);
    copy4(out.offset(8 as libc::c_int as isize), (*md).C);
    copy4(out.offset(12 as libc::c_int as isize), (*md).D);
}

unsafe extern "C" fn mdfour(
    mut out: *mut crate::src::qcommon::q_shared::byte,
    mut in_0: *mut crate::src::qcommon::q_shared::byte,
    mut n: libc::c_int,
) {
    let mut md: mdfour = mdfour {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        totalN: 0,
    };
    mdfour_begin(&mut md);
    mdfour_update(&mut md, in_0, n);
    mdfour_result(&mut md, out);
}
// will be journaled properly
//===================================================================
#[no_mangle]

pub unsafe extern "C" fn Com_BlockChecksum(
    mut buffer: *const libc::c_void,
    mut length: libc::c_int,
) -> libc::c_uint {
    let mut digest: [libc::c_int; 4] = [0; 4];
    let mut val: libc::c_uint = 0;
    mdfour(
        digest.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte,
        buffer as *mut crate::src::qcommon::q_shared::byte,
        length,
    );
    val = (digest[0 as libc::c_int as usize]
        ^ digest[1 as libc::c_int as usize]
        ^ digest[2 as libc::c_int as usize]
        ^ digest[3 as libc::c_int as usize]) as libc::c_uint;
    return val;
}

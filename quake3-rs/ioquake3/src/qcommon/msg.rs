use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::huff_t;
pub use crate::qcommon_h::huffman_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::node_t;
pub use crate::qcommon_h::nodetype;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::huffman::Huff_Init;
pub use crate::src::qcommon::huffman::Huff_addRef;
pub use crate::src::qcommon::huffman::Huff_getBit;
pub use crate::src::qcommon::huffman::Huff_getBloc;
pub use crate::src::qcommon::huffman::Huff_offsetReceive;
pub use crate::src::qcommon::huffman::Huff_offsetTransmit;
pub use crate::src::qcommon::huffman::Huff_putBit;
pub use crate::src::qcommon::huffman::Huff_setBloc;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

extern "C" {
    #[no_mangle]
    pub static mut cl_shownet: *mut crate::src::qcommon::q_shared::cvar_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netField_t {
    pub name: *mut libc::c_char,
    pub offset: libc::c_int,
    pub bits: libc::c_int,
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

static mut msgHuff: crate::qcommon_h::huffman_t = crate::qcommon_h::huffman_t {
    compressor: crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        lhead: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        ltail: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        loc: [0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *const *mut crate::qcommon_h::node_t as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            right: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            parent: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            next: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            prev: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            head: 0 as *const *mut crate::qcommon_h::nodetype
                as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t; 768],
    },
    decompressor: crate::qcommon_h::huff_t {
        blocNode: 0,
        blocPtrs: 0,
        tree: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        lhead: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        ltail: 0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t,
        loc: [0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t; 257],
        freelist: 0 as *const *mut crate::qcommon_h::node_t as *mut *mut crate::qcommon_h::node_t,
        nodeList: [crate::qcommon_h::node_t {
            left: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            right: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            parent: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            next: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            prev: 0 as *const crate::qcommon_h::nodetype as *mut crate::qcommon_h::nodetype,
            head: 0 as *const *mut crate::qcommon_h::nodetype
                as *mut *mut crate::qcommon_h::nodetype,
            weight: 0,
            symbol: 0,
        }; 768],
        nodePtrs: [0 as *const crate::qcommon_h::node_t as *mut crate::qcommon_h::node_t; 768],
    },
};

static mut msgInit: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
#[no_mangle]

pub static mut pcount: [libc::c_int; 256] = [0; 256];
/*
==============================================================================

            MESSAGE IO FUNCTIONS

Handles byte ordering and avoids alignment errors
==============================================================================
*/
#[no_mangle]

pub static mut oldsize: libc::c_int = 0 as libc::c_int;
#[no_mangle]

pub unsafe extern "C" fn MSG_Init(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: libc::c_int,
) {
    if msgInit as u64 == 0 {
        MSG_initHuffman();
    }
    crate::stdlib::memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>() as libc::c_ulong,
    );
    (*buf).data = data;
    (*buf).maxsize = length;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_InitOOB(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: libc::c_int,
) {
    if msgInit as u64 == 0 {
        MSG_initHuffman();
    }
    crate::stdlib::memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>() as libc::c_ulong,
    );
    (*buf).data = data;
    (*buf).maxsize = length;
    (*buf).oob = crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_Clear(mut buf: *mut crate::qcommon_h::msg_t) {
    (*buf).cursize = 0 as libc::c_int;
    (*buf).overflowed = crate::src::qcommon::q_shared::qfalse;
    (*buf).bit = 0 as libc::c_int;
    //<- in bits
}
#[no_mangle]

pub unsafe extern "C" fn MSG_Bitstream(mut buf: *mut crate::qcommon_h::msg_t) {
    (*buf).oob = crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_BeginReading(mut msg: *mut crate::qcommon_h::msg_t) {
    (*msg).readcount = 0 as libc::c_int;
    (*msg).bit = 0 as libc::c_int;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_BeginReadingOOB(mut msg: *mut crate::qcommon_h::msg_t) {
    (*msg).readcount = 0 as libc::c_int;
    (*msg).bit = 0 as libc::c_int;
    (*msg).oob = crate::src::qcommon::q_shared::qtrue;
}
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
#[no_mangle]

pub unsafe extern "C" fn MSG_Copy(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *mut crate::src::qcommon::q_shared::byte,
    mut length: libc::c_int,
    mut src: *mut crate::qcommon_h::msg_t,
) {
    if length < (*src).cursize {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MSG_Copy: can\'t copy into a smaller msg_t buffer\x00" as *const u8
                as *const libc::c_char,
        );
    }
    crate::stdlib::memcpy(
        buf as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<crate::qcommon_h::msg_t>() as libc::c_ulong,
    );
    (*buf).data = data;
    crate::stdlib::memcpy(
        (*buf).data as *mut libc::c_void,
        (*src).data as *const libc::c_void,
        (*src).cursize as libc::c_ulong,
    );
}
/*
=============================================================================

bit functions

=============================================================================
*/
// negative bit values include signs
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteBits(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut value: libc::c_int,
    mut bits: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    oldsize += bits;
    if (*msg).overflowed as u64 != 0 {
        return;
    }
    if bits == 0 as libc::c_int || bits < -(31 as libc::c_int) || bits > 32 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MSG_WriteBits: bad bits %i\x00" as *const u8 as *const libc::c_char,
            bits,
        );
    }
    if bits < 0 as libc::c_int {
        bits = -bits
    }
    if (*msg).oob as u64 != 0 {
        if (*msg).cursize + (bits >> 3 as libc::c_int) > (*msg).maxsize {
            (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
            return;
        }
        if bits == 8 as libc::c_int {
            *(*msg).data.offset((*msg).cursize as isize) =
                value as crate::src::qcommon::q_shared::byte;
            (*msg).cursize += 1 as libc::c_int;
            (*msg).bit += 8 as libc::c_int
        } else if bits == 16 as libc::c_int {
            let mut temp: libc::c_short = value as libc::c_short;
            crate::stdlib::memcpy(
                &mut *(*msg).data.offset((*msg).cursize as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut libc::c_void,
                &mut temp as *mut libc::c_short as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            (*msg).cursize += 2 as libc::c_int;
            (*msg).bit += 16 as libc::c_int
        } else if bits == 32 as libc::c_int {
            crate::stdlib::memcpy(
                &mut *(*msg).data.offset((*msg).cursize as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *mut libc::c_void,
                &mut value as *mut libc::c_int as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
            (*msg).cursize += 4 as libc::c_int;
            (*msg).bit += 32 as libc::c_int
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"can\'t write %d bits\x00" as *const u8 as *const libc::c_char,
                bits,
            );
        }
    } else {
        value = (value as libc::c_uint & 0xffffffff as libc::c_uint >> 32 as libc::c_int - bits)
            as libc::c_int;
        if bits & 7 as libc::c_int != 0 {
            let mut nbits: libc::c_int = 0;
            nbits = bits & 7 as libc::c_int;
            if (*msg).bit + nbits > (*msg).maxsize << 3 as libc::c_int {
                (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
                return;
            }
            i = 0 as libc::c_int;
            while i < nbits {
                crate::src::qcommon::huffman::Huff_putBit(
                    value & 1 as libc::c_int,
                    (*msg).data,
                    &mut (*msg).bit,
                );
                value = value >> 1 as libc::c_int;
                i += 1
            }
            bits = bits - nbits
        }
        if bits != 0 {
            i = 0 as libc::c_int;
            while i < bits {
                crate::src::qcommon::huffman::Huff_offsetTransmit(
                    &mut msgHuff.compressor as *mut _ as *mut crate::qcommon_h::huff_t,
                    value & 0xff as libc::c_int,
                    (*msg).data,
                    &mut (*msg).bit,
                    (*msg).maxsize << 3 as libc::c_int,
                );
                value = value >> 8 as libc::c_int;
                if (*msg).bit > (*msg).maxsize << 3 as libc::c_int {
                    (*msg).overflowed = crate::src::qcommon::q_shared::qtrue;
                    return;
                }
                i += 8 as libc::c_int
            }
        }
        (*msg).cursize = ((*msg).bit >> 3 as libc::c_int) + 1 as libc::c_int
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadBits(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut bits: libc::c_int,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut get: libc::c_int = 0;
    let mut sgn: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut i: libc::c_int = 0;
    let mut nbits: libc::c_int = 0;
    //	FILE*	fp;
    if (*msg).readcount > (*msg).cursize {
        return 0 as libc::c_int;
    }
    value = 0 as libc::c_int;
    if bits < 0 as libc::c_int {
        bits = -bits;
        sgn = crate::src::qcommon::q_shared::qtrue
    } else {
        sgn = crate::src::qcommon::q_shared::qfalse
    }
    if (*msg).oob as u64 != 0 {
        if (*msg).readcount + (bits >> 3 as libc::c_int) > (*msg).cursize {
            (*msg).readcount = (*msg).cursize + 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        if bits == 8 as libc::c_int {
            value = *(*msg).data.offset((*msg).readcount as isize) as libc::c_int;
            (*msg).readcount += 1 as libc::c_int;
            (*msg).bit += 8 as libc::c_int
        } else if bits == 16 as libc::c_int {
            let mut temp: libc::c_short = 0;
            crate::stdlib::memcpy(
                &mut temp as *mut libc::c_short as *mut libc::c_void,
                &mut *(*msg).data.offset((*msg).readcount as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
            value = temp as libc::c_int;
            (*msg).readcount += 2 as libc::c_int;
            (*msg).bit += 16 as libc::c_int
        } else if bits == 32 as libc::c_int {
            crate::stdlib::memcpy(
                &mut value as *mut libc::c_int as *mut libc::c_void,
                &mut *(*msg).data.offset((*msg).readcount as isize)
                    as *mut crate::src::qcommon::q_shared::byte
                    as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
            (*msg).readcount += 4 as libc::c_int;
            (*msg).bit += 32 as libc::c_int
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"can\'t read %d bits\x00" as *const u8 as *const libc::c_char,
                bits,
            );
        }
    } else {
        nbits = 0 as libc::c_int;
        if bits & 7 as libc::c_int != 0 {
            nbits = bits & 7 as libc::c_int;
            if (*msg).bit + nbits > (*msg).cursize << 3 as libc::c_int {
                (*msg).readcount = (*msg).cursize + 1 as libc::c_int;
                return 0 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < nbits {
                value |=
                    crate::src::qcommon::huffman::Huff_getBit((*msg).data, &mut (*msg).bit) << i;
                i += 1
            }
            bits = bits - nbits
        }
        if bits != 0 {
            //			fp = fopen("c:\\netchan.bin", "a");
            i = 0 as libc::c_int;
            while i < bits {
                crate::src::qcommon::huffman::Huff_offsetReceive(
                    msgHuff.decompressor.tree as *mut crate::qcommon_h::nodetype,
                    &mut get,
                    (*msg).data,
                    &mut (*msg).bit,
                    (*msg).cursize << 3 as libc::c_int,
                );
                //				fwrite(&get, 1, 1, fp);
                value = (value as libc::c_uint | (get as libc::c_uint) << i + nbits) as libc::c_int;
                if (*msg).bit > (*msg).cursize << 3 as libc::c_int {
                    (*msg).readcount = (*msg).cursize + 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
                i += 8 as libc::c_int
            }
            //			fclose(fp);
        }
        (*msg).readcount = ((*msg).bit >> 3 as libc::c_int) + 1 as libc::c_int
    }
    if sgn as libc::c_uint != 0 && bits > 0 as libc::c_int && bits < 32 as libc::c_int {
        if value & (1 as libc::c_int) << bits - 1 as libc::c_int != 0 {
            value |= -(1 as libc::c_int) ^ ((1 as libc::c_int) << bits) - 1 as libc::c_int
        }
    }
    return value;
}
//================================================================================
//
// writing functions
//
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteChar(mut sb: *mut crate::qcommon_h::msg_t, mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 8 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteByte(mut sb: *mut crate::qcommon_h::msg_t, mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 8 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteData(
    mut buf: *mut crate::qcommon_h::msg_t,
    mut data: *const libc::c_void,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        MSG_WriteByte(
            buf,
            *(data as *mut crate::src::qcommon::q_shared::byte).offset(i as isize) as libc::c_int,
        );
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteShort(mut sb: *mut crate::qcommon_h::msg_t, mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 16 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteLong(mut sb: *mut crate::qcommon_h::msg_t, mut c: libc::c_int) {
    MSG_WriteBits(sb, c, 32 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteFloat(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut f: libc::c_float,
) {
    let mut dat: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    dat.f = f;
    MSG_WriteBits(sb, dat.i, 32 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteString(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut s: *const libc::c_char,
) {
    if s.is_null() {
        MSG_WriteData(
            sb,
            b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    } else {
        let mut l: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut string: [libc::c_char; 1024] = [0; 1024];
        l = crate::stdlib::strlen(s) as libc::c_int;
        if l >= 1024 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"MSG_WriteString: MAX_STRING_CHARS\x00" as *const u8 as *const libc::c_char,
            );
            MSG_WriteData(
                sb,
                b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int,
            );
            return;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            string.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        // get rid of 0x80+ and '%' chars, because old clients don't like them
        i = 0 as libc::c_int;
        while i < l {
            if *(string.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte).offset(i as isize)
                as libc::c_int
                > 127 as libc::c_int
                || string[i as usize] as libc::c_int == '%' as i32
            {
                string[i as usize] = '.' as i32 as libc::c_char
            }
            i += 1
        }
        MSG_WriteData(
            sb,
            string.as_mut_ptr() as *const libc::c_void,
            l + 1 as libc::c_int,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteBigString(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut s: *const libc::c_char,
) {
    if s.is_null() {
        MSG_WriteData(
            sb,
            b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    } else {
        let mut l: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut string: [libc::c_char; 8192] = [0; 8192];
        l = crate::stdlib::strlen(s) as libc::c_int;
        if l >= 8192 as libc::c_int {
            crate::src::qcommon::common::Com_Printf(
                b"MSG_WriteString: BIG_INFO_STRING\x00" as *const u8 as *const libc::c_char,
            );
            MSG_WriteData(
                sb,
                b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int,
            );
            return;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            string.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as libc::c_int,
        );
        // get rid of 0x80+ and '%' chars, because old clients don't like them
        i = 0 as libc::c_int;
        while i < l {
            if *(string.as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte).offset(i as isize)
                as libc::c_int
                > 127 as libc::c_int
                || string[i as usize] as libc::c_int == '%' as i32
            {
                string[i as usize] = '.' as i32 as libc::c_char
            }
            i += 1
        }
        MSG_WriteData(
            sb,
            string.as_mut_ptr() as *const libc::c_void,
            l + 1 as libc::c_int,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteAngle(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut f: libc::c_float,
) {
    MSG_WriteByte(
        sb,
        (f * 256 as libc::c_int as libc::c_float / 360 as libc::c_int as libc::c_float)
            as libc::c_int
            & 255 as libc::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteAngle16(
    mut sb: *mut crate::qcommon_h::msg_t,
    mut f: libc::c_float,
) {
    MSG_WriteShort(
        sb,
        (f * 65536 as libc::c_int as libc::c_float / 360 as libc::c_int as libc::c_float)
            as libc::c_int
            & 65535 as libc::c_int,
    );
}
//============================================================
//
// reading functions
//
// returns -1 if no more characters are available
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadChar(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0; // use ReadByte so -1 is out of bounds
    c = MSG_ReadBits(msg, 8 as libc::c_int) as libc::c_schar as libc::c_int;
    if (*msg).readcount > (*msg).cursize {
        c = -(1 as libc::c_int)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadByte(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 8 as libc::c_int) as libc::c_uchar as libc::c_int;
    if (*msg).readcount > (*msg).cursize {
        c = -(1 as libc::c_int)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_LookaheadByte(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_int {
    let bloc: libc::c_int = crate::src::qcommon::huffman::Huff_getBloc();
    let readcount: libc::c_int = (*msg).readcount;
    let bit: libc::c_int = (*msg).bit;
    let mut c: libc::c_int = MSG_ReadByte(msg);
    crate::src::qcommon::huffman::Huff_setBloc(bloc);
    (*msg).readcount = readcount;
    (*msg).bit = bit;
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadShort(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 16 as libc::c_int) as libc::c_short as libc::c_int;
    if (*msg).readcount > (*msg).cursize {
        c = -(1 as libc::c_int)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadLong(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = MSG_ReadBits(msg, 32 as libc::c_int);
    if (*msg).readcount > (*msg).cursize {
        c = -(1 as libc::c_int)
    }
    return c;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadFloat(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_float {
    let mut dat: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    dat.i = MSG_ReadBits(msg, 32 as libc::c_int);
    if (*msg).readcount > (*msg).cursize {
        dat.f = -(1 as libc::c_int) as libc::c_float
    }
    return dat.f;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadString(
    mut msg: *mut crate::qcommon_h::msg_t,
) -> *mut libc::c_char {
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0 as libc::c_int;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1 as libc::c_int) || c == 0 as libc::c_int {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 as libc::c_int {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            break; // use ReadByte so -1 is out of bounds
        }
        let fresh0 = l;
        l = l + 1;
        string[fresh0 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadBigString(
    mut msg: *mut crate::qcommon_h::msg_t,
) -> *mut libc::c_char {
    static mut string: [libc::c_char; 8192] = [0; 8192];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0 as libc::c_int;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1 as libc::c_int) || c == 0 as libc::c_int {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 as libc::c_int {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            break; // use ReadByte so -1 is out of bounds
        }
        let fresh1 = l;
        l = l + 1;
        string[fresh1 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadStringLine(
    mut msg: *mut crate::qcommon_h::msg_t,
) -> *mut libc::c_char {
    static mut string: [libc::c_char; 1024] = [0; 1024];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0 as libc::c_int;
    loop {
        c = MSG_ReadByte(msg);
        if c == -(1 as libc::c_int) || c == 0 as libc::c_int || c == '\n' as i32 {
            break;
        }
        // translate all fmt spec to avoid crash bugs
        if c == '%' as i32 {
            c = '.' as i32
        }
        // don't allow higher ascii values
        if c > 127 as libc::c_int {
            c = '.' as i32
        }
        // break only after reading all expected data from bitstream
        if l as libc::c_ulong
            >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        let fresh2 = l;
        l = l + 1;
        string[fresh2 as usize] = c as libc::c_char
    }
    string[l as usize] = '\u{0}' as i32 as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadAngle16(mut msg: *mut crate::qcommon_h::msg_t) -> libc::c_float {
    return (MSG_ReadShort(msg) as libc::c_double
        * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadData(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut data: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *(data as *mut crate::src::qcommon::q_shared::byte).offset(i as isize) =
            MSG_ReadByte(msg) as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
// a string hasher which gives the same hash value even if the
// string is later modified via the legacy MSG read/write code
#[no_mangle]

pub unsafe extern "C" fn MSG_HashKey(
    mut string: *const libc::c_char,
    mut maxlen: libc::c_int,
) -> libc::c_int {
    let mut hash: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    hash = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < maxlen && *string.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        if *string.offset(i as isize) as libc::c_int & 0x80 as libc::c_int != 0
            || *string.offset(i as isize) as libc::c_int == '%' as i32
        {
            hash += '.' as i32 * (119 as libc::c_int + i)
        } else {
            hash += *string.offset(i as isize) as libc::c_int * (119 as libc::c_int + i)
        }
        i += 1
    }
    hash = hash ^ hash >> 10 as libc::c_int ^ hash >> 20 as libc::c_int;
    return hash;
}
/*
=============================================================================

delta functions with keys

=============================================================================
*/
#[no_mangle]

pub static mut kbitmask: [libc::c_int; 32] = [
    0x1 as libc::c_int,
    0x3 as libc::c_int,
    0x7 as libc::c_int,
    0xf as libc::c_int,
    0x1f as libc::c_int,
    0x3f as libc::c_int,
    0x7f as libc::c_int,
    0xff as libc::c_int,
    0x1ff as libc::c_int,
    0x3ff as libc::c_int,
    0x7ff as libc::c_int,
    0xfff as libc::c_int,
    0x1fff as libc::c_int,
    0x3fff as libc::c_int,
    0x7fff as libc::c_int,
    0xffff as libc::c_int,
    0x1ffff as libc::c_int,
    0x3ffff as libc::c_int,
    0x7ffff as libc::c_int,
    0xfffff as libc::c_int,
    0x1fffff as libc::c_int,
    0x3fffff as libc::c_int,
    0x7fffff as libc::c_int,
    0xffffff as libc::c_int,
    0x1ffffff as libc::c_int,
    0x3ffffff as libc::c_int,
    0x7ffffff as libc::c_int,
    0xfffffff as libc::c_int,
    0x1fffffff as libc::c_int,
    0x3fffffff as libc::c_int,
    0x7fffffff as libc::c_int,
    0xffffffff as libc::c_uint as libc::c_int,
];
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut oldV: libc::c_int,
    mut newV: libc::c_int,
    mut bits: libc::c_int,
) {
    if oldV == newV {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        return;
    }
    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
    MSG_WriteBits(msg, newV ^ key, bits);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut oldV: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
        return MSG_ReadBits(msg, bits) ^ key & kbitmask[(bits - 1 as libc::c_int) as usize];
    }
    return oldV;
}
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaKeyFloat(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut oldV: libc::c_float,
    mut newV: libc::c_float,
) {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    if oldV == newV {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        return;
    }
    fi.f = newV;
    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
    MSG_WriteBits(msg, fi.i ^ key, 32 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaKeyFloat(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut oldV: libc::c_float,
) -> libc::c_float {
    if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
        let mut fi: crate::src::qcommon::q_shared::floatint_t =
            crate::src::qcommon::q_shared::floatint_t { f: 0. };
        fi.i = MSG_ReadBits(msg, 32 as libc::c_int) ^ key;
        return fi.f;
    }
    return oldV;
}
/*
============================================================================

usercmd_t communication

============================================================================
*/
/*
=====================
MSG_WriteDeltaUsercmdKey
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaUsercmdKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut from: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut to: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    if (*to).serverTime - (*from).serverTime < 256 as libc::c_int {
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int); // no change
        MSG_WriteBits(msg, (*to).serverTime - (*from).serverTime, 8 as libc::c_int);
    } else {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        MSG_WriteBits(msg, (*to).serverTime, 32 as libc::c_int);
    }
    if (*from).angles[0 as libc::c_int as usize] == (*to).angles[0 as libc::c_int as usize]
        && (*from).angles[1 as libc::c_int as usize] == (*to).angles[1 as libc::c_int as usize]
        && (*from).angles[2 as libc::c_int as usize] == (*to).angles[2 as libc::c_int as usize]
        && (*from).forwardmove as libc::c_int == (*to).forwardmove as libc::c_int
        && (*from).rightmove as libc::c_int == (*to).rightmove as libc::c_int
        && (*from).upmove as libc::c_int == (*to).upmove as libc::c_int
        && (*from).buttons == (*to).buttons
        && (*from).weapon as libc::c_int == (*to).weapon as libc::c_int
    {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        oldsize += 7 as libc::c_int;
        return;
    }
    key ^= (*to).serverTime;
    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).angles[0 as libc::c_int as usize],
        (*to).angles[0 as libc::c_int as usize],
        16 as libc::c_int,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).angles[1 as libc::c_int as usize],
        (*to).angles[1 as libc::c_int as usize],
        16 as libc::c_int,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).angles[2 as libc::c_int as usize],
        (*to).angles[2 as libc::c_int as usize],
        16 as libc::c_int,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).forwardmove as libc::c_int,
        (*to).forwardmove as libc::c_int,
        8 as libc::c_int,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).rightmove as libc::c_int,
        (*to).rightmove as libc::c_int,
        8 as libc::c_int,
    );
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).upmove as libc::c_int,
        (*to).upmove as libc::c_int,
        8 as libc::c_int,
    );
    MSG_WriteDeltaKey(msg, key, (*from).buttons, (*to).buttons, 16 as libc::c_int);
    MSG_WriteDeltaKey(
        msg,
        key,
        (*from).weapon as libc::c_int,
        (*to).weapon as libc::c_int,
        8 as libc::c_int,
    );
}
/*
=====================
MSG_ReadDeltaUsercmdKey
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaUsercmdKey(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut key: libc::c_int,
    mut from: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut to: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
        (*to).serverTime = (*from).serverTime + MSG_ReadBits(msg, 8 as libc::c_int)
    } else {
        (*to).serverTime = MSG_ReadBits(msg, 32 as libc::c_int)
    }
    if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
        key ^= (*to).serverTime;
        (*to).angles[0 as libc::c_int as usize] = MSG_ReadDeltaKey(
            msg,
            key,
            (*from).angles[0 as libc::c_int as usize],
            16 as libc::c_int,
        );
        (*to).angles[1 as libc::c_int as usize] = MSG_ReadDeltaKey(
            msg,
            key,
            (*from).angles[1 as libc::c_int as usize],
            16 as libc::c_int,
        );
        (*to).angles[2 as libc::c_int as usize] = MSG_ReadDeltaKey(
            msg,
            key,
            (*from).angles[2 as libc::c_int as usize],
            16 as libc::c_int,
        );
        (*to).forwardmove = MSG_ReadDeltaKey(
            msg,
            key,
            (*from).forwardmove as libc::c_int,
            8 as libc::c_int,
        ) as libc::c_schar;
        if (*to).forwardmove as libc::c_int == -(128 as libc::c_int) {
            (*to).forwardmove = -(127 as libc::c_int) as libc::c_schar
        }
        (*to).rightmove =
            MSG_ReadDeltaKey(msg, key, (*from).rightmove as libc::c_int, 8 as libc::c_int)
                as libc::c_schar;
        if (*to).rightmove as libc::c_int == -(128 as libc::c_int) {
            (*to).rightmove = -(127 as libc::c_int) as libc::c_schar
        }
        (*to).upmove = MSG_ReadDeltaKey(msg, key, (*from).upmove as libc::c_int, 8 as libc::c_int)
            as libc::c_schar;
        if (*to).upmove as libc::c_int == -(128 as libc::c_int) {
            (*to).upmove = -(127 as libc::c_int) as libc::c_schar
        }
        (*to).buttons = MSG_ReadDeltaKey(msg, key, (*from).buttons, 16 as libc::c_int);
        (*to).weapon = MSG_ReadDeltaKey(msg, key, (*from).weapon as libc::c_int, 8 as libc::c_int)
            as crate::src::qcommon::q_shared::byte
    } else {
        (*to).angles[0 as libc::c_int as usize] = (*from).angles[0 as libc::c_int as usize];
        (*to).angles[1 as libc::c_int as usize] = (*from).angles[1 as libc::c_int as usize];
        (*to).angles[2 as libc::c_int as usize] = (*from).angles[2 as libc::c_int as usize];
        (*to).forwardmove = (*from).forwardmove;
        (*to).rightmove = (*from).rightmove;
        (*to).upmove = (*from).upmove;
        (*to).buttons = (*from).buttons;
        (*to).weapon = (*from).weapon
    };
}
/*
=============================================================================

entityState_t communication

=============================================================================
*/
/*
=================
MSG_ReportChangeVectors_f

Prints out a table from the current statistics for copying to code
=================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReportChangeVectors_f() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if pcount[i as usize] != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%d used %d\n\x00" as *const u8 as *const libc::c_char,
                i,
                pcount[i as usize],
            );
        }
        i += 1
    }
}
// using the stringizing operator to save typing...
// Initialized in run_static_initializers
#[no_mangle]

pub static mut entityStateFields: [netField_t; 51] = [netField_t {
    name: 0 as *mut libc::c_char,
    offset: 0,
    bits: 0,
}; 51];
/*
==================
MSG_WriteDeltaEntity

Writes part of a packetentities message, including the entity number.
Can delta from either a baseline or a previous packet_entity
If to is NULL, a remove entity update will be sent
If force is not set, then nothing at all will be generated if the entity is
identical, under the assumption that the in-order delta code will catch it.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaEntity(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::entityState_s,
    mut to: *mut crate::src::qcommon::q_shared::entityState_s,
    mut force: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut trunc: libc::c_int = 0;
    let mut fullFloat: libc::c_float = 0.;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    numFields = (::std::mem::size_of::<[netField_t; 51]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<netField_t>() as libc::c_ulong)
        as libc::c_int;
    // all fields should be 32 bits to avoid any compiler packing issues
    // the "number" field is not part of the field list
    // if this assert fails, someone added a field to the entityState_t
    // struct without updating the message fields
    // a NULL to is a delta remove message
    if to.is_null() {
        if from.is_null() {
            return;
        }
        MSG_WriteBits(msg, (*from).number, 10 as libc::c_int);
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
        return;
    }
    if (*to).number < 0 as libc::c_int || (*to).number >= (1 as libc::c_int) << 10 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"MSG_WriteDeltaEntity: Bad entity number: %i\x00" as *const u8 as *const libc::c_char,
            (*to).number,
        );
    }
    lc = 0 as libc::c_int;
    // build the change vector as bytes so it is endien independent
    i = 0 as libc::c_int;
    field = entityStateFields.as_mut_ptr();
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if *fromF != *toF {
            lc = i + 1 as libc::c_int
        }
        i += 1;
        field = field.offset(1)
    }
    if lc == 0 as libc::c_int {
        // nothing at all changed
        if force as u64 == 0 {
            return;
            // nothing at all
        }
        // write two bits for no change
        MSG_WriteBits(msg, (*to).number, 10 as libc::c_int); // not removed
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int); // no delta
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int); // not removed
        return;
    } // we have a delta
    MSG_WriteBits(msg, (*to).number, 10 as libc::c_int); // # of changes
    MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int); // no change
    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int); // changed
    MSG_WriteByte(msg, lc);
    oldsize += numFields;
    i = 0 as libc::c_int;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        } else {
            MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
            if (*field).bits == 0 as libc::c_int {
                // float
                fullFloat = *(toF as *mut libc::c_float);
                trunc = fullFloat as libc::c_int;
                if fullFloat == 0.0f32 {
                    MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
                    oldsize += 13 as libc::c_int
                } else {
                    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
                    if trunc as libc::c_float == fullFloat
                        && trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int)
                            >= 0 as libc::c_int
                        && (trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int))
                            < (1 as libc::c_int) << 13 as libc::c_int
                    {
                        // send as small integer
                        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
                        MSG_WriteBits(
                            msg,
                            trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int),
                            13 as libc::c_int,
                        );
                    } else {
                        // send as full floating point value
                        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
                        MSG_WriteBits(msg, *toF, 32 as libc::c_int);
                    }
                }
            } else if *toF == 0 as libc::c_int {
                MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
            } else {
                MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
                // integer
                MSG_WriteBits(msg, *toF, (*field).bits);
            }
        }
        i += 1;
        field = field.offset(1)
    }
}
/*
==================
MSG_ReadDeltaEntity

The entity number has already been read from the message, which
is how the from state is identified.

If the delta removes the entity, entityState_t->number will be set to MAX_GENTITIES-1

Can go from either a baseline or a previous packet_entity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaEntity(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::entityState_t,
    mut to: *mut crate::src::qcommon::q_shared::entityState_t,
    mut number: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut print: libc::c_int = 0;
    let mut trunc: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut endBit: libc::c_int = 0;
    if number < 0 as libc::c_int || number >= (1 as libc::c_int) << 10 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Bad delta entity number: %i\x00" as *const u8 as *const libc::c_char,
            number,
        );
    }
    if (*msg).bit == 0 as libc::c_int {
        startBit = (*msg).readcount * 8 as libc::c_int - 10 as libc::c_int
    } else {
        startBit = ((*msg).readcount - 1 as libc::c_int) * 8 as libc::c_int + (*msg).bit
            - 10 as libc::c_int
    }
    // check for a remove
    if MSG_ReadBits(msg, 1 as libc::c_int) == 1 as libc::c_int {
        crate::stdlib::memset(
            to as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong,
        );
        (*to).number = ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int;
        if !cl_shownet.is_null()
            && ((*cl_shownet).integer >= 2 as libc::c_int
                || (*cl_shownet).integer == -(1 as libc::c_int))
        {
            crate::src::qcommon::common::Com_Printf(
                b"%3i: #%-3i remove\n\x00" as *const u8 as *const libc::c_char,
                (*msg).readcount,
                number,
            );
        }
        return;
    }
    // check for no delta
    if MSG_ReadBits(msg, 1 as libc::c_int) == 0 as libc::c_int {
        *to = *from;
        (*to).number = number;
        return;
    }
    numFields = (::std::mem::size_of::<[netField_t; 51]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<netField_t>() as libc::c_ulong)
        as libc::c_int;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"invalid entityState field count\x00" as *const u8 as *const libc::c_char,
        );
    }
    // shownet 2/3 will interleave with other printed info, -1 will
    // just print the delta records`
    if !cl_shownet.is_null()
        && ((*cl_shownet).integer >= 2 as libc::c_int
            || (*cl_shownet).integer == -(1 as libc::c_int))
    {
        print = 1 as libc::c_int;
        crate::src::qcommon::common::Com_Printf(
            b"%3i: #%-3i \x00" as *const u8 as *const libc::c_char,
            (*msg).readcount,
            (*to).number,
        );
    } else {
        print = 0 as libc::c_int
    }
    (*to).number = number;
    i = 0 as libc::c_int;
    field = entityStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if MSG_ReadBits(msg, 1 as libc::c_int) == 0 {
            // no change
            *toF = *fromF
        } else if (*field).bits == 0 as libc::c_int {
            // float
            if MSG_ReadBits(msg, 1 as libc::c_int) == 0 as libc::c_int {
                *(toF as *mut libc::c_float) = 0.0f32
            } else if MSG_ReadBits(msg, 1 as libc::c_int) == 0 as libc::c_int {
                // integral float
                trunc = MSG_ReadBits(msg, 13 as libc::c_int);
                // bias to allow equal parts positive and negative
                trunc -= (1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int;
                *(toF as *mut libc::c_float) = trunc as libc::c_float;
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%i \x00" as *const u8 as *const libc::c_char,
                        (*field).name,
                        trunc,
                    );
                }
            } else {
                // full floating point value
                *toF = MSG_ReadBits(msg, 32 as libc::c_int);
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%f \x00" as *const u8 as *const libc::c_char,
                        (*field).name,
                        *(toF as *mut libc::c_float) as libc::c_double,
                    );
                }
            }
        } else if MSG_ReadBits(msg, 1 as libc::c_int) == 0 as libc::c_int {
            *toF = 0 as libc::c_int
        } else {
            // integer
            *toF = MSG_ReadBits(msg, (*field).bits);
            if print != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:%i \x00" as *const u8 as *const libc::c_char,
                    (*field).name,
                    *toF,
                );
            }
        }
        i += 1;
        field = field.offset(1)
    }
    i = lc;
    field = &mut *entityStateFields.as_mut_ptr().offset(lc as isize) as *mut netField_t;
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        //			pcount[i]++;
        // no change
        *toF = *fromF;
        i += 1;
        field = field.offset(1)
    }
    if print != 0 {
        if (*msg).bit == 0 as libc::c_int {
            endBit = (*msg).readcount * 8 as libc::c_int - 10 as libc::c_int
        } else {
            endBit = ((*msg).readcount - 1 as libc::c_int) * 8 as libc::c_int + (*msg).bit
                - 10 as libc::c_int
        }
        crate::src::qcommon::common::Com_Printf(
            b" (%i bits)\n\x00" as *const u8 as *const libc::c_char,
            endBit - startBit,
        );
    };
}
/*
============================================================================

plyer_state_t communication

============================================================================
*/
// using the stringizing operator to save typing...
// Initialized in run_static_initializers
#[no_mangle]

pub static mut playerStateFields: [netField_t; 48] = [netField_t {
    name: 0 as *mut libc::c_char,
    offset: 0,
    bits: 0,
}; 48];
/*
=============
MSG_WriteDeltaPlayerstate

=============
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_WriteDeltaPlayerstate(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::playerState_s,
    mut to: *mut crate::src::qcommon::q_shared::playerState_s,
) {
    let mut i: libc::c_int = 0; // # of changes
    let mut dummy: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        }; // no change
    let mut statsbits: libc::c_int = 0; // changed
    let mut persistantbits: libc::c_int = 0;
    let mut ammobits: libc::c_int = 0;
    let mut powerupbits: libc::c_int = 0;
    let mut numFields: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut fullFloat: libc::c_float = 0.;
    let mut trunc: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    if from.is_null() {
        from = &mut dummy;
        crate::stdlib::memset(
            &mut dummy as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>() as libc::c_ulong,
        );
    }
    numFields = (::std::mem::size_of::<[netField_t; 48]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<netField_t>() as libc::c_ulong)
        as libc::c_int;
    lc = 0 as libc::c_int;
    i = 0 as libc::c_int;
    field = playerStateFields.as_mut_ptr();
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if *fromF != *toF {
            lc = i + 1 as libc::c_int
        }
        i += 1;
        field = field.offset(1)
    }
    MSG_WriteByte(msg, lc);
    oldsize += numFields - lc;
    i = 0 as libc::c_int;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if *fromF == *toF {
            MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        } else {
            MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
            //		pcount[i]++;
            if (*field).bits == 0 as libc::c_int {
                // float
                fullFloat = *(toF as *mut libc::c_float);
                trunc = fullFloat as libc::c_int;
                if trunc as libc::c_float == fullFloat
                    && trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int)
                        >= 0 as libc::c_int
                    && (trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int))
                        < (1 as libc::c_int) << 13 as libc::c_int
                {
                    // send as small integer
                    MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
                    MSG_WriteBits(
                        msg,
                        trunc + ((1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int),
                        13 as libc::c_int,
                    );
                } else {
                    // send as full floating point value
                    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
                    MSG_WriteBits(msg, *toF, 32 as libc::c_int);
                }
            } else {
                // integer
                MSG_WriteBits(msg, *toF, (*field).bits);
            }
        }
        i += 1;
        field = field.offset(1)
    }
    //
    // send the arrays
    //
    statsbits = 0 as libc::c_int; // no change
    i = 0 as libc::c_int; // changed
    while i < 16 as libc::c_int {
        if (*to).stats[i as usize] != (*from).stats[i as usize] {
            statsbits |= (1 as libc::c_int) << i
        } // changed
        i += 1
    }
    persistantbits = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if (*to).persistant[i as usize] != (*from).persistant[i as usize] {
            persistantbits |= (1 as libc::c_int) << i
        }
        i += 1
    }
    ammobits = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if (*to).ammo[i as usize] != (*from).ammo[i as usize] {
            ammobits |= (1 as libc::c_int) << i
        }
        i += 1
    }
    powerupbits = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if (*to).powerups[i as usize] != (*from).powerups[i as usize] {
            powerupbits |= (1 as libc::c_int) << i
        }
        i += 1
    }
    if statsbits == 0 && persistantbits == 0 && ammobits == 0 && powerupbits == 0 {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        oldsize += 4 as libc::c_int;
        return;
    }
    MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
    if statsbits != 0 {
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
        MSG_WriteBits(msg, statsbits, 16 as libc::c_int);
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if statsbits & (1 as libc::c_int) << i != 0 {
                MSG_WriteShort(msg, (*to).stats[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        // no change
    } // changed
    if persistantbits != 0 {
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
        MSG_WriteBits(msg, persistantbits, 16 as libc::c_int);
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if persistantbits & (1 as libc::c_int) << i != 0 {
                MSG_WriteShort(msg, (*to).persistant[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        // no change
    } // changed
    if ammobits != 0 {
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
        MSG_WriteBits(msg, ammobits, 16 as libc::c_int);
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if ammobits & (1 as libc::c_int) << i != 0 {
                MSG_WriteShort(msg, (*to).ammo[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        // no change
    } // changed
    if powerupbits != 0 {
        MSG_WriteBits(msg, 1 as libc::c_int, 1 as libc::c_int);
        MSG_WriteBits(msg, powerupbits, 16 as libc::c_int);
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if powerupbits & (1 as libc::c_int) << i != 0 {
                MSG_WriteLong(msg, (*to).powerups[i as usize]);
            }
            i += 1
        }
    } else {
        MSG_WriteBits(msg, 0 as libc::c_int, 1 as libc::c_int);
        // no change
    };
}
/*
===================
MSG_ReadDeltaPlayerstate
===================
*/
#[no_mangle]

pub unsafe extern "C" fn MSG_ReadDeltaPlayerstate(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut from: *mut crate::src::qcommon::q_shared::playerState_t,
    mut to: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut i: libc::c_int = 0;
    let mut lc: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut field: *mut netField_t = 0 as *mut netField_t;
    let mut numFields: libc::c_int = 0;
    let mut startBit: libc::c_int = 0;
    let mut endBit: libc::c_int = 0;
    let mut print: libc::c_int = 0;
    let mut fromF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut toF: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut trunc: libc::c_int = 0;
    let mut dummy: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        };
    if from.is_null() {
        from = &mut dummy;
        crate::stdlib::memset(
            &mut dummy as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>() as libc::c_ulong,
        );
    }
    *to = *from;
    if (*msg).bit == 0 as libc::c_int {
        startBit = (*msg).readcount * 8 as libc::c_int - 10 as libc::c_int
    } else {
        startBit = ((*msg).readcount - 1 as libc::c_int) * 8 as libc::c_int + (*msg).bit
            - 10 as libc::c_int
    }
    // shownet 2/3 will interleave with other printed info, -2 will
    // just print the delta records
    if !cl_shownet.is_null()
        && ((*cl_shownet).integer >= 2 as libc::c_int
            || (*cl_shownet).integer == -(2 as libc::c_int))
    {
        print = 1 as libc::c_int;
        crate::src::qcommon::common::Com_Printf(
            b"%3i: playerstate \x00" as *const u8 as *const libc::c_char,
            (*msg).readcount,
        );
    } else {
        print = 0 as libc::c_int
    }
    numFields = (::std::mem::size_of::<[netField_t; 48]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<netField_t>() as libc::c_ulong)
        as libc::c_int;
    lc = MSG_ReadByte(msg);
    if lc > numFields || lc < 0 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"invalid playerState field count\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    field = playerStateFields.as_mut_ptr();
    while i < lc {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        if MSG_ReadBits(msg, 1 as libc::c_int) == 0 {
            // no change
            *toF = *fromF
        } else if (*field).bits == 0 as libc::c_int {
            // float
            if MSG_ReadBits(msg, 1 as libc::c_int) == 0 as libc::c_int {
                // integral float
                trunc = MSG_ReadBits(msg, 13 as libc::c_int);
                // bias to allow equal parts positive and negative
                trunc -= (1 as libc::c_int) << 13 as libc::c_int - 1 as libc::c_int;
                *(toF as *mut libc::c_float) = trunc as libc::c_float;
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%i \x00" as *const u8 as *const libc::c_char,
                        (*field).name,
                        trunc,
                    );
                }
            } else {
                // full floating point value
                *toF = MSG_ReadBits(msg, 32 as libc::c_int);
                if print != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"%s:%f \x00" as *const u8 as *const libc::c_char,
                        (*field).name,
                        *(toF as *mut libc::c_float) as libc::c_double,
                    );
                }
            }
        } else {
            // integer
            *toF = MSG_ReadBits(msg, (*field).bits);
            if print != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:%i \x00" as *const u8 as *const libc::c_char,
                    (*field).name,
                    *toF,
                );
            }
        }
        i += 1;
        field = field.offset(1)
    }
    i = lc;
    field = &mut *playerStateFields.as_mut_ptr().offset(lc as isize) as *mut netField_t;
    while i < numFields {
        fromF = (from as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        toF = (to as *mut crate::src::qcommon::q_shared::byte).offset((*field).offset as isize)
            as *mut libc::c_int;
        // no change
        *toF = *fromF;
        i += 1;
        field = field.offset(1)
    }
    // read the arrays
    if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
        // parse stats
        if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 as libc::c_int {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const libc::c_char,
                    b"PS_STATS\x00" as *const u8 as *const libc::c_char,
                );
            }
            bits = MSG_ReadBits(msg, 16 as libc::c_int);
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                if bits & (1 as libc::c_int) << i != 0 {
                    (*to).stats[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse persistant stats
        if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 as libc::c_int {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const libc::c_char,
                    b"PS_PERSISTANT\x00" as *const u8 as *const libc::c_char,
                );
            }
            bits = MSG_ReadBits(msg, 16 as libc::c_int);
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                if bits & (1 as libc::c_int) << i != 0 {
                    (*to).persistant[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse ammo
        if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 as libc::c_int {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const libc::c_char,
                    b"PS_AMMO\x00" as *const u8 as *const libc::c_char,
                );
            }
            bits = MSG_ReadBits(msg, 16 as libc::c_int);
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                if bits & (1 as libc::c_int) << i != 0 {
                    (*to).ammo[i as usize] = MSG_ReadShort(msg)
                }
                i += 1
            }
        }
        // parse powerups
        if MSG_ReadBits(msg, 1 as libc::c_int) != 0 {
            if !cl_shownet.is_null() && (*cl_shownet).integer == 4 as libc::c_int {
                crate::src::qcommon::common::Com_Printf(
                    b"%s \x00" as *const u8 as *const libc::c_char,
                    b"PS_POWERUPS\x00" as *const u8 as *const libc::c_char,
                ); // Do update
            }
            bits = MSG_ReadBits(msg, 16 as libc::c_int);
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                if bits & (1 as libc::c_int) << i != 0 {
                    (*to).powerups[i as usize] = MSG_ReadLong(msg)
                }
                i += 1
            }
        }
    }
    if print != 0 {
        if (*msg).bit == 0 as libc::c_int {
            endBit = (*msg).readcount * 8 as libc::c_int - 10 as libc::c_int
        } else {
            endBit = ((*msg).readcount - 1 as libc::c_int) * 8 as libc::c_int + (*msg).bit
                - 10 as libc::c_int
        }
        crate::src::qcommon::common::Com_Printf(
            b" (%i bits)\n\x00" as *const u8 as *const libc::c_char,
            endBit - startBit,
        );
    };
}
#[no_mangle]

pub static mut msg_hData: [libc::c_int; 256] = [
    250315 as libc::c_int,
    41193 as libc::c_int,
    6292 as libc::c_int,
    7106 as libc::c_int,
    3730 as libc::c_int,
    3750 as libc::c_int,
    6110 as libc::c_int,
    23283 as libc::c_int,
    33317 as libc::c_int,
    6950 as libc::c_int,
    7838 as libc::c_int,
    9714 as libc::c_int,
    9257 as libc::c_int,
    17259 as libc::c_int,
    3949 as libc::c_int,
    1778 as libc::c_int,
    8288 as libc::c_int,
    1604 as libc::c_int,
    1590 as libc::c_int,
    1663 as libc::c_int,
    1100 as libc::c_int,
    1213 as libc::c_int,
    1238 as libc::c_int,
    1134 as libc::c_int,
    1749 as libc::c_int,
    1059 as libc::c_int,
    1246 as libc::c_int,
    1149 as libc::c_int,
    1273 as libc::c_int,
    4486 as libc::c_int,
    2805 as libc::c_int,
    3472 as libc::c_int,
    21819 as libc::c_int,
    1159 as libc::c_int,
    1670 as libc::c_int,
    1066 as libc::c_int,
    1043 as libc::c_int,
    1012 as libc::c_int,
    1053 as libc::c_int,
    1070 as libc::c_int,
    1726 as libc::c_int,
    888 as libc::c_int,
    1180 as libc::c_int,
    850 as libc::c_int,
    960 as libc::c_int,
    780 as libc::c_int,
    1752 as libc::c_int,
    3296 as libc::c_int,
    10630 as libc::c_int,
    4514 as libc::c_int,
    5881 as libc::c_int,
    2685 as libc::c_int,
    4650 as libc::c_int,
    3837 as libc::c_int,
    2093 as libc::c_int,
    1867 as libc::c_int,
    2584 as libc::c_int,
    1949 as libc::c_int,
    1972 as libc::c_int,
    940 as libc::c_int,
    1134 as libc::c_int,
    1788 as libc::c_int,
    1670 as libc::c_int,
    1206 as libc::c_int,
    5719 as libc::c_int,
    6128 as libc::c_int,
    7222 as libc::c_int,
    6654 as libc::c_int,
    3710 as libc::c_int,
    3795 as libc::c_int,
    1492 as libc::c_int,
    1524 as libc::c_int,
    2215 as libc::c_int,
    1140 as libc::c_int,
    1355 as libc::c_int,
    971 as libc::c_int,
    2180 as libc::c_int,
    1248 as libc::c_int,
    1328 as libc::c_int,
    1195 as libc::c_int,
    1770 as libc::c_int,
    1078 as libc::c_int,
    1264 as libc::c_int,
    1266 as libc::c_int,
    1168 as libc::c_int,
    965 as libc::c_int,
    1155 as libc::c_int,
    1186 as libc::c_int,
    1347 as libc::c_int,
    1228 as libc::c_int,
    1529 as libc::c_int,
    1600 as libc::c_int,
    2617 as libc::c_int,
    2048 as libc::c_int,
    2546 as libc::c_int,
    3275 as libc::c_int,
    2410 as libc::c_int,
    3585 as libc::c_int,
    2504 as libc::c_int,
    2800 as libc::c_int,
    2675 as libc::c_int,
    6146 as libc::c_int,
    3663 as libc::c_int,
    2840 as libc::c_int,
    14253 as libc::c_int,
    3164 as libc::c_int,
    2221 as libc::c_int,
    1687 as libc::c_int,
    3208 as libc::c_int,
    2739 as libc::c_int,
    3512 as libc::c_int,
    4796 as libc::c_int,
    4091 as libc::c_int,
    3515 as libc::c_int,
    5288 as libc::c_int,
    4016 as libc::c_int,
    7937 as libc::c_int,
    6031 as libc::c_int,
    5360 as libc::c_int,
    3924 as libc::c_int,
    4892 as libc::c_int,
    3743 as libc::c_int,
    4566 as libc::c_int,
    4807 as libc::c_int,
    5852 as libc::c_int,
    6400 as libc::c_int,
    6225 as libc::c_int,
    8291 as libc::c_int,
    23243 as libc::c_int,
    7838 as libc::c_int,
    7073 as libc::c_int,
    8935 as libc::c_int,
    5437 as libc::c_int,
    4483 as libc::c_int,
    3641 as libc::c_int,
    5256 as libc::c_int,
    5312 as libc::c_int,
    5328 as libc::c_int,
    5370 as libc::c_int,
    3492 as libc::c_int,
    2458 as libc::c_int,
    1694 as libc::c_int,
    1821 as libc::c_int,
    2121 as libc::c_int,
    1916 as libc::c_int,
    1149 as libc::c_int,
    1516 as libc::c_int,
    1367 as libc::c_int,
    1236 as libc::c_int,
    1029 as libc::c_int,
    1258 as libc::c_int,
    1104 as libc::c_int,
    1245 as libc::c_int,
    1006 as libc::c_int,
    1149 as libc::c_int,
    1025 as libc::c_int,
    1241 as libc::c_int,
    952 as libc::c_int,
    1287 as libc::c_int,
    997 as libc::c_int,
    1713 as libc::c_int,
    1009 as libc::c_int,
    1187 as libc::c_int,
    879 as libc::c_int,
    1099 as libc::c_int,
    929 as libc::c_int,
    1078 as libc::c_int,
    951 as libc::c_int,
    1656 as libc::c_int,
    930 as libc::c_int,
    1153 as libc::c_int,
    1030 as libc::c_int,
    1262 as libc::c_int,
    1062 as libc::c_int,
    1214 as libc::c_int,
    1060 as libc::c_int,
    1621 as libc::c_int,
    930 as libc::c_int,
    1106 as libc::c_int,
    912 as libc::c_int,
    1034 as libc::c_int,
    892 as libc::c_int,
    1158 as libc::c_int,
    990 as libc::c_int,
    1175 as libc::c_int,
    850 as libc::c_int,
    1121 as libc::c_int,
    903 as libc::c_int,
    1087 as libc::c_int,
    920 as libc::c_int,
    1144 as libc::c_int,
    1056 as libc::c_int,
    3462 as libc::c_int,
    2240 as libc::c_int,
    4397 as libc::c_int,
    12136 as libc::c_int,
    7758 as libc::c_int,
    1345 as libc::c_int,
    1307 as libc::c_int,
    3278 as libc::c_int,
    1950 as libc::c_int,
    886 as libc::c_int,
    1023 as libc::c_int,
    1112 as libc::c_int,
    1077 as libc::c_int,
    1042 as libc::c_int,
    1061 as libc::c_int,
    1071 as libc::c_int,
    1484 as libc::c_int,
    1001 as libc::c_int,
    1096 as libc::c_int,
    915 as libc::c_int,
    1052 as libc::c_int,
    995 as libc::c_int,
    1070 as libc::c_int,
    876 as libc::c_int,
    1111 as libc::c_int,
    851 as libc::c_int,
    1059 as libc::c_int,
    805 as libc::c_int,
    1112 as libc::c_int,
    923 as libc::c_int,
    1103 as libc::c_int,
    817 as libc::c_int,
    1899 as libc::c_int,
    1872 as libc::c_int,
    976 as libc::c_int,
    841 as libc::c_int,
    1127 as libc::c_int,
    956 as libc::c_int,
    1159 as libc::c_int,
    950 as libc::c_int,
    7791 as libc::c_int,
    954 as libc::c_int,
    1289 as libc::c_int,
    933 as libc::c_int,
    1127 as libc::c_int,
    3207 as libc::c_int,
    1020 as libc::c_int,
    927 as libc::c_int,
    1355 as libc::c_int,
    768 as libc::c_int,
    1040 as libc::c_int,
    745 as libc::c_int,
    952 as libc::c_int,
    805 as libc::c_int,
    1073 as libc::c_int,
    740 as libc::c_int,
    1013 as libc::c_int,
    805 as libc::c_int,
    1008 as libc::c_int,
    796 as libc::c_int,
    996 as libc::c_int,
    1057 as libc::c_int,
    11457 as libc::c_int,
    13504 as libc::c_int,
];
#[no_mangle]

pub unsafe extern "C" fn MSG_initHuffman() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    msgInit = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::huffman::Huff_Init(
        &mut msgHuff as *mut _ as *mut crate::qcommon_h::huffman_t,
    );
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        j = 0 as libc::c_int;
        while j < msg_hData[i as usize] {
            crate::src::qcommon::huffman::Huff_addRef(
                &mut msgHuff.compressor as *mut _ as *mut crate::qcommon_h::huff_t,
                i as crate::src::qcommon::q_shared::byte,
            );
            crate::src::qcommon::huffman::Huff_addRef(
                &mut msgHuff.decompressor as *mut _ as *mut crate::qcommon_h::huff_t,
                i as crate::src::qcommon::q_shared::byte,
            );
            j += 1
            // Do update
        }
        i += 1
    }
}
unsafe extern "C" fn run_static_initializers() {
    entityStateFields = [
        {
            let mut init = netField_t {
                name: b"pos.trTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trTime as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[0]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[1]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trBase[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trBase
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[1]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDelta[2]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDelta
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[0]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"event\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).event
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eType\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eType
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoAnim\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).torsoAnim
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eventParm
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsAnim\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).legsAnim
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"groundEntityNum\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .groundEntityNum as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trType\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trType as *mut crate::src::qcommon::q_shared::trType_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eFlags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).eFlags
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 19 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"otherEntityNum\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .otherEntityNum as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).weapon
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"clientNum\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).clientNum
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pos.trDuration\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .pos
                    .trDuration as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trType\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trType as *mut crate::src::qcommon::q_shared::trType_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"solid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).solid
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 24 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"powerups\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).powerups
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"modelindex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).modelindex
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"otherEntityNum2\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .otherEntityNum2 as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"loopSound\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).loopSound
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"generic1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).generic1
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin2[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .origin2
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"modelindex2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).modelindex2
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"time\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).time
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trTime as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDuration\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDuration as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trBase[2]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trBase
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[0]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[1]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"apos.trDelta[2]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .apos
                    .trDelta
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"time2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).time2
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"angles2[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .angles2
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"constantLight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t))
                    .constantLight as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"frame\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::entityState_t)).frame
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
    ];
    playerStateFields = [
        {
            let mut init = netField_t {
                name: b"commandTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).commandTime
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 32 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"bobCycle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).bobCycle
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weaponTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weaponTime
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: -(16 as libc::c_int),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"origin[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .origin
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"velocity[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .velocity
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsTimer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).legsTimer
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_time\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_time
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: -(16 as libc::c_int),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventSequence\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventSequence as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoAnim\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).torsoAnim
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"movementDir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).movementDir
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"events[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .events
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"legsAnim\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).legsAnim
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"events[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .events
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_flags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_flags
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"groundEntityNum\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .groundEntityNum as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weaponstate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weaponstate
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eFlags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).eFlags
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"externalEvent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .externalEvent as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"gravity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).gravity
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).speed
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[1]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"externalEventParm\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .externalEventParm as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewheight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).viewheight
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: -(8 as libc::c_int),
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageEvent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageEvent
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageYaw\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageYaw
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damagePitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damagePitch
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"damageCount\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).damageCount
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"generic1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).generic1
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"pm_type\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).pm_type
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[0]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"delta_angles[2]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .delta_angles
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"torsoTimer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).torsoTimer
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 12 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParms[0]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventParms
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"eventParms[1]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .eventParms
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_int
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"clientNum\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).clientNum
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 8 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).weapon
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 5 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"viewangles[2]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .viewangles
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[0]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[1]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"grapplePoint[2]\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                offset: &mut *(*(0 as *mut crate::src::qcommon::q_shared::playerState_t))
                    .grapplePoint
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    as *mut crate::src::qcommon::q_shared::vec_t
                    as crate::stddef_h::size_t as libc::c_int,
                bits: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"jumppad_ent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).jumppad_ent
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 10 as libc::c_int,
            };
            init
        },
        {
            let mut init = netField_t {
                name: b"loopSound\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                offset: &mut (*(0 as *mut crate::src::qcommon::q_shared::playerState_t)).loopSound
                    as *mut libc::c_int as crate::stddef_h::size_t
                    as libc::c_int,
                bits: 16 as libc::c_int,
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
//===========================================================================
/*
void MSG_NUinitHuffman() {
    byte	*data;
    int		size, i, ch;
    int		array[256];

    msgInit = qtrue;

    Huff_Init(&msgHuff);
    // load it in
    size = FS_ReadFile( "netchan/netchan.bin", (void **)&data );

    for(i=0;i<256;i++) {
        array[i] = 0;
    }
    for(i=0;i<size;i++) {
        ch = data[i];
        Huff_addRef(&msgHuff.compressor,	ch);			// Do update
        Huff_addRef(&msgHuff.decompressor,	ch);			// Do update
        array[ch]++;
    }
    Com_Printf("msg_hData {\n");
    for(i=0;i<256;i++) {
        if (array[i] == 0) {
            Huff_addRef(&msgHuff.compressor,	i);			// Do update
            Huff_addRef(&msgHuff.decompressor,	i);			// Do update
        }
        Com_Printf("%d,			// %d\n", array[i], i);
    }
    Com_Printf("};\n");
    FS_FreeFile( data );
    Cbuf_AddText( "condump dump.txt\n" );
}
*/

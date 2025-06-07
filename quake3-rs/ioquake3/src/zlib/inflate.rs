// =============== BEGIN inflate_h ================
pub type inflate_mode = libc::c_uint;

pub const HEAD: crate::src::zlib::inflate::inflate_mode = 0;

pub const FLAGS: crate::src::zlib::inflate::inflate_mode = 1;

pub const TIME: crate::src::zlib::inflate::inflate_mode = 2;

pub const OS: crate::src::zlib::inflate::inflate_mode = 3;

pub const EXLEN: crate::src::zlib::inflate::inflate_mode = 4;

pub const EXTRA: crate::src::zlib::inflate::inflate_mode = 5;

pub const NAME: crate::src::zlib::inflate::inflate_mode = 6;

pub const COMMENT: crate::src::zlib::inflate::inflate_mode = 7;

pub const HCRC: crate::src::zlib::inflate::inflate_mode = 8;

pub const DICTID: crate::src::zlib::inflate::inflate_mode = 9;

pub const DICT: crate::src::zlib::inflate::inflate_mode = 10;

pub const TYPE: crate::src::zlib::inflate::inflate_mode = 11;

pub const TYPEDO: crate::src::zlib::inflate::inflate_mode = 12;

pub const STORED: crate::src::zlib::inflate::inflate_mode = 13;

pub const COPY: crate::src::zlib::inflate::inflate_mode = 14;

pub const TABLE: crate::src::zlib::inflate::inflate_mode = 15;

pub const LENLENS: crate::src::zlib::inflate::inflate_mode = 16;

pub const CODELENS: crate::src::zlib::inflate::inflate_mode = 17;

pub const LEN: crate::src::zlib::inflate::inflate_mode = 18;

pub const LENEXT: crate::src::zlib::inflate::inflate_mode = 19;

pub const DIST: crate::src::zlib::inflate::inflate_mode = 20;

pub const DISTEXT: crate::src::zlib::inflate::inflate_mode = 21;

pub const MATCH: crate::src::zlib::inflate::inflate_mode = 22;

pub const LIT: crate::src::zlib::inflate::inflate_mode = 23;

pub const CHECK: crate::src::zlib::inflate::inflate_mode = 24;

pub const LENGTH: crate::src::zlib::inflate::inflate_mode = 25;

pub const DONE: crate::src::zlib::inflate::inflate_mode = 26;

pub const BAD: crate::src::zlib::inflate::inflate_mode = 27;

pub const MEM: crate::src::zlib::inflate::inflate_mode = 28;

pub const SYNC: crate::src::zlib::inflate::inflate_mode = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inflate_state {
    pub mode: crate::src::zlib::inflate::inflate_mode,
    pub last: libc::c_int,
    pub wrap: libc::c_int,
    pub havedict: libc::c_int,
    pub flags: libc::c_int,
    pub dmax: libc::c_uint,
    pub check: libc::c_ulong,
    pub total: libc::c_ulong,
    pub head: crate::zlib_h::gz_headerp,
    pub wbits: libc::c_uint,
    pub wsize: libc::c_uint,
    pub whave: libc::c_uint,
    pub write: libc::c_uint,
    pub window: *mut libc::c_uchar,
    pub hold: libc::c_ulong,
    pub bits: libc::c_uint,
    pub length: libc::c_uint,
    pub offset: libc::c_uint,
    pub extra: libc::c_uint,
    pub lencode: *const crate::src::zlib::inftrees::code,
    pub distcode: *const crate::src::zlib::inftrees::code,
    pub lenbits: libc::c_uint,
    pub distbits: libc::c_uint,
    pub ncode: libc::c_uint,
    pub nlen: libc::c_uint,
    pub ndist: libc::c_uint,
    pub have: libc::c_uint,
    pub next: *mut crate::src::zlib::inftrees::code,
    pub lens: [libc::c_ushort; 320],
    pub work: [libc::c_ushort; 288],
    pub codes: [crate::src::zlib::inftrees::code; 2048],
}
use ::libc;

pub use crate::src::zlib::adler32::adler32;

pub use crate::src::zlib::inftrees::code;
pub use crate::src::zlib::inftrees::codetype;
pub use crate::src::zlib::inftrees::inflate_table;
pub use crate::src::zlib::inftrees::CODES;
pub use crate::src::zlib::inftrees::DISTS;
pub use crate::src::zlib::inftrees::LENS;

pub use crate::zconf_h::uInt;
pub use crate::zconf_h::uLong;
pub use crate::zconf_h::voidpf;
pub use crate::zconf_h::Byte;
pub use crate::zconf_h::Bytef;
pub use crate::zlib_h::alloc_func;
pub use crate::zlib_h::free_func;
pub use crate::zlib_h::gz_header;
pub use crate::zlib_h::gz_header_s;
pub use crate::zlib_h::gz_headerp;
pub use crate::zlib_h::internal_state;
pub use crate::zlib_h::z_stream;
pub use crate::zlib_h::z_stream_s;
pub use crate::zlib_h::z_streamp;
#[no_mangle]

pub unsafe extern "C" fn inflateReset(mut strm: crate::zlib_h::z_streamp) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state; /* to support ill-conceived Java test suite */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    } /* in case we return an error */
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    (*state).total = 0 as libc::c_int as libc::c_ulong;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = 0 as *mut libc::c_char;
    (*strm).adler = 1 as libc::c_int as crate::zconf_h::uLong;
    (*state).mode = crate::src::zlib::inflate::HEAD;
    (*state).last = 0 as libc::c_int;
    (*state).havedict = 0 as libc::c_int;
    (*state).dmax = 32768 as libc::c_uint;
    (*state).head = 0 as crate::zlib_h::gz_headerp;
    (*state).wsize = 0 as libc::c_int as libc::c_uint;
    (*state).whave = 0 as libc::c_int as libc::c_uint;
    (*state).write = 0 as libc::c_int as libc::c_uint;
    (*state).hold = 0 as libc::c_int as libc::c_ulong;
    (*state).bits = 0 as libc::c_int as libc::c_uint;
    (*state).next = (*state).codes.as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn inflatePrime(
    mut strm: crate::zlib_h::z_streamp,
    mut bits: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if bits > 16 as libc::c_int
        || (*state).bits.wrapping_add(bits as libc::c_uint) > 32 as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    value = (value as libc::c_long
        & ((1 as libc::c_long) << bits) - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    (*state).hold = (*state)
        .hold
        .wrapping_add((value << (*state).bits) as libc::c_ulong);
    (*state).bits = (*state).bits.wrapping_add(bits as libc::c_uint);
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn inflateInit2_(
    mut strm: crate::zlib_h::z_streamp,
    mut windowBits: libc::c_int,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    if version.is_null()
        || *version.offset(0 as libc::c_int as isize) as libc::c_int
            != (*::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"1.2.3\x00"))
                [0 as libc::c_int as usize] as libc::c_int
        || stream_size
            != ::std::mem::size_of::<crate::zlib_h::z_stream>() as libc::c_ulong as libc::c_int
    {
        return -(6 as libc::c_int);
    }
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    (*strm).msg = 0 as *mut libc::c_char;
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            crate::src::zlib::zutil::zcalloc
                as unsafe extern "C" fn(
                    _: crate::zconf_h::voidpf,
                    _: libc::c_uint,
                    _: libc::c_uint,
                ) -> crate::zconf_h::voidpf,
        );
        (*strm).opaque = 0 as crate::zconf_h::voidpf
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(
            crate::src::zlib::zutil::zcfree
                as unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> (),
        )
    }
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as libc::c_int as crate::zconf_h::uInt,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as libc::c_ulong
            as crate::zconf_h::uInt,
    ) as *mut crate::src::zlib::inflate::inflate_state;
    if state.is_null() {
        return -(4 as libc::c_int);
    }
    (*strm).state = state as *mut crate::zlib_h::internal_state;
    if windowBits < 0 as libc::c_int {
        (*state).wrap = 0 as libc::c_int;
        windowBits = -windowBits
    } else {
        (*state).wrap = (windowBits >> 4 as libc::c_int) + 1 as libc::c_int
    }
    if windowBits < 8 as libc::c_int || windowBits > 15 as libc::c_int {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as crate::zconf_h::voidpf,
        );
        (*strm).state = 0 as *mut crate::zlib_h::internal_state;
        return -(2 as libc::c_int);
    }
    (*state).wbits = windowBits as libc::c_uint;
    (*state).window = 0 as *mut libc::c_uchar;
    return inflateReset(strm);
}
#[no_mangle]

pub unsafe extern "C" fn inflateInit_(
    mut strm: crate::zlib_h::z_streamp,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    return inflateInit2_(strm, 15 as libc::c_int, version, stream_size);
}
/* inflate.c -- zlib decompression
 * Copyright (C) 1995-2005 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
 * Change history:
 *
 * 1.2.beta0    24 Nov 2002
 * - First version -- complete rewrite of inflate to simplify code, avoid
 *   creation of window when not needed, minimize use of window when it is
 *   needed, make inffast.c even faster, implement gzip decoding, and to
 *   improve code readability and style over the previous zlib inflate code
 *
 * 1.2.beta1    25 Nov 2002
 * - Use pointers for available input and output checking in inffast.c
 * - Remove input and output counters in inffast.c
 * - Change inffast.c entry and loop from avail_in >= 7 to >= 6
 * - Remove unnecessary second byte pull from length extra in inffast.c
 * - Unroll direct copy to three copies per loop in inffast.c
 *
 * 1.2.beta2    4 Dec 2002
 * - Change external routine names to reduce potential conflicts
 * - Correct filename to inffixed.h for fixed tables in inflate.c
 * - Make hbuf[] unsigned char to match parameter type in inflate.c
 * - Change strm->next_out[-state->offset] to *(strm->next_out - state->offset)
 *   to avoid negation problem on Alphas (64 bit) in inflate.c
 *
 * 1.2.beta3    22 Dec 2002
 * - Add comments on state->bits assertion in inffast.c
 * - Add comments on op field in inftrees.h
 * - Fix bug in reuse of allocated window after inflateReset()
 * - Remove bit fields--back to byte structure for speed
 * - Remove distance extra == 0 check in inflate_fast()--only helps for lengths
 * - Change post-increments to pre-increments in inflate_fast(), PPC biased?
 * - Add compile time option, POSTINC, to use post-increments instead (Intel?)
 * - Make MATCH copy in inflate() much faster for when inflate_fast() not used
 * - Use local copies of stream next and avail values, as well as local bit
 *   buffer and bit count in inflate()--for speed when inflate_fast() not used
 *
 * 1.2.beta4    1 Jan 2003
 * - Split ptr - 257 statements in inflate_table() to avoid compiler warnings
 * - Move a comment on output buffer sizes from inffast.c to inflate.c
 * - Add comments in inffast.c to introduce the inflate_fast() routine
 * - Rearrange window copies in inflate_fast() for speed and simplification
 * - Unroll last copy for window match in inflate_fast()
 * - Use local copies of window variables in inflate_fast() for speed
 * - Pull out common write == 0 case for speed in inflate_fast()
 * - Make op and len in inflate_fast() unsigned for consistency
 * - Add FAR to lcode and dcode declarations in inflate_fast()
 * - Simplified bad distance check in inflate_fast()
 * - Added inflateBackInit(), inflateBack(), and inflateBackEnd() in new
 *   source file infback.c to provide a call-back interface to inflate for
 *   programs like gzip and unzip -- uses window as output buffer to avoid
 *   window copying
 *
 * 1.2.beta5    1 Jan 2003
 * - Improved inflateBack() interface to allow the caller to provide initial
 *   input in strm.
 * - Fixed stored blocks bug in inflateBack()
 *
 * 1.2.beta6    4 Jan 2003
 * - Added comments in inffast.c on effectiveness of POSTINC
 * - Typecasting all around to reduce compiler warnings
 * - Changed loops from while (1) or do {} while (1) to for (;;), again to
 *   make compilers happy
 * - Changed type of window in inflateBackInit() to unsigned char *
 *
 * 1.2.beta7    27 Jan 2003
 * - Changed many types to unsigned or unsigned short to avoid warnings
 * - Added inflateCopy() function
 *
 * 1.2.0        9 Mar 2003
 * - Changed inflateBack() interface to provide separate opaque descriptors
 *   for the in() and out() functions
 * - Changed inflateBack() argument and in_func typedef to swap the length
 *   and buffer address return values for the input function
 * - Check next_in and next_out for Z_NULL on entry to inflate()
 *
 * The history for versions after 1.2.0 are in ChangeLog in zlib distribution.
 */
/* function prototypes */
/*
  Return state with length and distance decoding tables and index sizes set to
  fixed code decoding.  Normally this returns fixed tables from inffixed.h.
  If BUILDFIXED is defined, then instead this routine builds the tables the
  first time it's called, and returns those tables the first time and
  thereafter.  This reduces the size of the code by about 2K bytes, in
  exchange for a little execution time.  However, BUILDFIXED should not be
  used for threaded applications, since the rewriting of the tables and virgin
  may not be thread-safe.
*/

unsafe extern "C" fn fixedtables(mut state: *mut crate::src::zlib::inflate::inflate_state) {
    /* inffixed.h -- table for decoding fixed codes
     * Generated automatically by makefixed().
     */
    /* WARNING: this file should *not* be used by applications. It
      is part of the implementation of the compression library and
      is subject to change. Applications should only use zlib.h.
    */
    static mut lenfix: [crate::src::zlib::inftrees::code; 512] = [
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 192 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 160 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 224 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 144 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 208 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 176 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 240 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 200 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 168 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 232 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 152 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 216 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 184 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 248 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 196 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 164 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 228 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 148 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 212 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 180 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 244 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 204 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 172 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 236 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 156 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 220 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 188 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 252 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 194 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 162 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 226 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 146 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 210 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 178 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 242 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 202 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 170 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 234 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 154 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 218 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 186 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 250 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 198 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 166 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 230 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 150 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 214 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 182 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 246 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 206 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 174 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 238 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 158 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 222 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 190 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 254 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 161 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 225 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 209 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 177 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 241 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 201 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 169 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 233 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 153 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 217 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 185 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 249 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 197 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 165 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 229 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 149 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 213 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 181 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 245 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 205 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 173 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 237 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 157 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 221 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 189 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 253 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 147 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 211 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 179 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 243 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 203 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 171 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 235 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 155 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 219 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 187 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 251 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 199 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 167 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 231 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 151 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 215 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 183 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 247 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 207 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 175 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 239 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 159 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 223 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 191 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 255 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    static mut distfix: [crate::src::zlib::inftrees::code; 32] = [
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 257 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4097 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1025 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 16385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 513 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 8193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2049 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 6145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1537 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 24577 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 769 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 12289 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3073 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = crate::src::zlib::inftrees::code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    /* !BUILDFIXED */
    /* BUILDFIXED */
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as libc::c_int as libc::c_uint;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as libc::c_int as libc::c_uint;
}
/* MAKEFIXED */
/*
  Update the window with the last wsize (normally 32K) bytes written before
  returning.  If window does not exist yet, create it.  This is only called
  when a window is already in use, or when output has been written during this
  inflate call, but the end of the deflate stream has not been reached yet.
  It is also called to create a window for dictionary data when a dictionary
  is loaded.

  Providing output buffers larger than 32K to inflate() should provide a speed
  advantage, since only the last 32K of output is copied to the sliding window
  upon return from inflate(), and since all distances after the first 32K of
  output will fall in the output data, making match copies simpler and faster.
  The advantage may be dependent on the size of the processor's data caches.
*/

unsafe extern "C" fn updatewindow(
    mut strm: crate::zlib_h::z_streamp,
    mut out: libc::c_uint,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    let mut copy: libc::c_uint = 0;
    let mut dist: libc::c_uint = 0;
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    /* if it hasn't been done already, allocate space for the window */
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as crate::zconf_h::uInt,
        ) as *mut libc::c_uchar;
        if (*state).window.is_null() {
            return 1 as libc::c_int;
        }
    }
    /* if window not in use yet, initialize */
    if (*state).wsize == 0 as libc::c_int as libc::c_uint {
        (*state).wsize = (1 as libc::c_uint) << (*state).wbits;
        (*state).write = 0 as libc::c_int as libc::c_uint;
        (*state).whave = 0 as libc::c_int as libc::c_uint
    }
    /* copy state->wsize or less output bytes into the circular window */
    copy = out.wrapping_sub((*strm).avail_out);
    if copy >= (*state).wsize {
        crate::stdlib::memcpy(
            (*state).window as *mut libc::c_void,
            (*strm).next_out.offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as libc::c_ulong,
        );
        (*state).write = 0 as libc::c_int as libc::c_uint;
        (*state).whave = (*state).wsize
    } else {
        dist = (*state).wsize.wrapping_sub((*state).write);
        if dist > copy {
            dist = copy
        }
        crate::stdlib::memcpy(
            (*state).window.offset((*state).write as isize) as *mut libc::c_void,
            (*strm).next_out.offset(-(copy as isize)) as *const libc::c_void,
            dist as libc::c_ulong,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            crate::stdlib::memcpy(
                (*state).window as *mut libc::c_void,
                (*strm).next_out.offset(-(copy as isize)) as *const libc::c_void,
                copy as libc::c_ulong,
            );
            (*state).write = copy;
            (*state).whave = (*state).wsize
        } else {
            (*state).write = (*state).write.wrapping_add(dist);
            if (*state).write == (*state).wsize {
                (*state).write = 0 as libc::c_int as libc::c_uint
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).whave.wrapping_add(dist)
            }
        }
    }
    return 0 as libc::c_int;
}
/* Macros for inflate(): */
/* check function to use adler32() for zlib or crc32() for gzip */
/* check macros for header crc */
/* Load registers with state in inflate() for speed */
/* Restore state from registers in inflate() */
/* Clear the input bit accumulator */
/* Get a byte of input into the bit accumulator, or return from inflate()
if there is no input available. */
/* Assure that there are at least n bits in the bit accumulator.  If there is
not enough available input to do that, then return from inflate(). */
/* Return the low n bits of the bit accumulator (n < 16) */
/* Remove n bits from the bit accumulator */
/* Remove zero to seven bits as needed to go to a byte boundary */
/* Reverse the bytes in a 32-bit value */
/*
  inflate() uses a state machine to process as much input data and generate as
  much output data as possible before returning.  The state machine is
  structured roughly as follows:

   for (;;) switch (state) {
   ...
   case STATEn:
       if (not enough input data or output space to make progress)
           return;
       ... make progress ...
       state = STATEm;
       break;
   ...
   }

  so when inflate() is called again, the same case is attempted again, and
  if the appropriate resources are provided, the machine proceeds to the
  next state.  The NEEDBITS() macro is usually the way the state evaluates
  whether it can proceed or should return.  NEEDBITS() does the return if
  the requested bits are not available.  The typical use of the BITS macros
  is:

       NEEDBITS(n);
       ... do something with BITS(n) ...
       DROPBITS(n);

  where NEEDBITS(n) either returns from inflate() if there isn't enough
  input left to load n bits into the accumulator, or it continues.  BITS(n)
  gives the low n bits in the accumulator.  When done, DROPBITS(n) drops
  the low n bits off the accumulator.  INITBITS() clears the accumulator
  and sets the number of available bits to zero.  BYTEBITS() discards just
  enough bits to put the accumulator on a byte boundary.  After BYTEBITS()
  and a NEEDBITS(8), then BITS(8) would return the next byte in the stream.

  NEEDBITS(n) uses PULLBYTE() to get an available byte of input, or to return
  if there is no input available.  The decoding of variable length codes uses
  PULLBYTE() directly in order to pull just enough bytes to decode the next
  code, and no more.

  Some states loop until they get enough input, making sure that enough
  state information is maintained to continue the loop where it left off
  if NEEDBITS() returns in the loop.  For example, want, need, and keep
  would all have to actually be part of the saved state in case NEEDBITS()
  returns:

   case STATEw:
       while (want < need) {
           NEEDBITS(n);
           keep[want++] = BITS(n);
           DROPBITS(n);
       }
       state = STATEx;
   case STATEx:

  As shown above, if the next state is also the next case, then the break
  is omitted.

  A state may also return if there is not enough output space available to
  complete that state.  Those states are copying stored data, writing a
  literal byte, and copying a matching string.

  When returning, a "goto inf_leave" is used to update the total counters,
  update the check value, and determine whether any progress has been made
  during that inflate() call in order to return the proper return code.
  Progress is defined as a change in either strm->avail_in or strm->avail_out.
  When there is a window, goto inf_leave will update the window with the last
  output written.  If a goto inf_leave occurs in the middle of decompression
  and there is no window currently, goto inf_leave will create one and copy
  output to the window for the next call of inflate().

  In this implementation, the flush parameter of inflate() only affects the
  return code (per zlib.h).  inflate() always writes as much as possible to
  strm->next_out, given the space available and the provided input--the effect
  documented in zlib.h of Z_SYNC_FLUSH.  Furthermore, inflate() always defers
  the allocation of and copying into a sliding window until necessary, which
  provides the effect documented in zlib.h for Z_FINISH when the entire input
  stream available.  So the only thing the flush parameter actually does is:
  when flush is set to Z_FINISH, inflate() cannot return Z_OK.  Instead it
  will return Z_BUF_ERROR if it has not reached the end of the stream.
*/
#[no_mangle]

pub unsafe extern "C" fn inflate(
    mut strm: crate::zlib_h::z_streamp,
    mut flush: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64; /* next input */
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state; /* next output */
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* available input and output */
    let mut put: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* bit buffer */
    let mut have: libc::c_uint = 0; /* bits in bit buffer */
    let mut left: libc::c_uint = 0; /* save starting available input and output */
    let mut hold: libc::c_ulong = 0; /* number of stored or match bytes to copy */
    let mut bits: libc::c_uint = 0; /* where to copy match bytes from */
    let mut in_0: libc::c_uint = 0; /* current decoding table entry */
    let mut out: libc::c_uint = 0; /* parent table entry */
    let mut copy: libc::c_uint = 0; /* length to copy for repeats, bits to drop */
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* return code */
    let mut this: crate::src::zlib::inftrees::code = crate::src::zlib::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* skip check */
    let mut last: crate::src::zlib::inftrees::code = crate::src::zlib::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* go to byte boundary */
    let mut len: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    static mut order: [libc::c_ushort; 19] = [
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        8 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        6 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        11 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        12 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        14 as libc::c_int as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        15 as libc::c_int as libc::c_ushort,
    ];
    if strm.is_null()
        || (*strm).state.is_null()
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).mode as libc::c_uint
        == crate::src::zlib::inflate::TYPE as libc::c_int as libc::c_uint
    {
        (*state).mode = crate::src::zlib::inflate::TYPEDO
    }
    put = (*strm).next_out;
    left = (*strm).avail_out;
    next = (*strm).next_in;
    have = (*strm).avail_in;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = 0 as libc::c_int;
    's_114: loop {
        match (*state).mode as libc::c_uint {
            0 => {
                if (*state).wrap == 0 as libc::c_int {
                    (*state).mode = crate::src::zlib::inflate::TYPEDO;
                    continue;
                } else {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if (((hold as libc::c_uint
                        & ((1 as libc::c_uint) << 8 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        << 8 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(hold >> 8 as libc::c_int)
                        .wrapping_rem(31 as libc::c_int as libc::c_ulong)
                        != 0
                    {
                        (*strm).msg = b"incorrect header check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else if hold as libc::c_uint
                        & ((1 as libc::c_uint) << 4 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        != 8 as libc::c_int as libc::c_uint
                    {
                        (*strm).msg = b"unknown compression method\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        hold >>= 4 as libc::c_int;
                        bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                        len = (hold as libc::c_uint
                            & ((1 as libc::c_uint) << 4 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        .wrapping_add(8 as libc::c_int as libc::c_uint);
                        if len > (*state).wbits {
                            (*strm).msg = b"invalid window size\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            continue;
                        } else {
                            (*state).dmax = (1 as libc::c_uint) << len;
                            (*state).check = crate::src::zlib::adler32::adler32(
                                0 as libc::c_long as crate::zconf_h::uLong,
                                0 as *const crate::zconf_h::Bytef,
                                0 as libc::c_int as crate::zconf_h::uInt,
                            );
                            (*strm).adler = (*state).check;
                            (*state).mode = if hold & 0x200 as libc::c_int as libc::c_ulong != 0 {
                                crate::src::zlib::inflate::DICTID as libc::c_int
                            } else {
                                crate::src::zlib::inflate::TYPE as libc::c_int
                            }
                                as crate::src::zlib::inflate::inflate_mode;
                            hold = 0 as libc::c_int as libc::c_ulong;
                            bits = 0 as libc::c_int as libc::c_uint;
                            continue;
                        }
                    }
                }
            }
            9 => {
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                (*state).check = (hold >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                    .wrapping_add(hold >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (hold & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (hold & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int,
                    );
                (*strm).adler = (*state).check;
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = crate::src::zlib::inflate::DICT;
                current_block = 210528378685203046;
            }
            10 => {
                current_block = 210528378685203046;
            }
            11 => {
                current_block = 10674880093440332853;
            }
            12 => {
                current_block = 14847832218395804385;
            }
            13 => {
                hold >>= bits & 7 as libc::c_int as libc::c_uint;
                bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                if hold & 0xffff as libc::c_int as libc::c_ulong
                    != hold >> 16 as libc::c_int ^ 0xffff as libc::c_int as libc::c_ulong
                {
                    (*strm).msg = b"invalid stored block lengths\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).length = hold as libc::c_uint & 0xffff as libc::c_int as libc::c_uint;
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = crate::src::zlib::inflate::COPY
                }
                current_block = 18432964712698998993;
            }
            14 => {
                current_block = 18432964712698998993;
            }
            15 => {
                while bits < 14 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh4 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh4 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                (*state).nlen = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(257 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state).ndist = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(1 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state).ncode = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 4 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                .wrapping_add(4 as libc::c_int as libc::c_uint);
                hold >>= 4 as libc::c_int;
                bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                if (*state).nlen > 286 as libc::c_int as libc::c_uint
                    || (*state).ndist > 30 as libc::c_int as libc::c_uint
                {
                    (*strm).msg = b"too many length or distance symbols\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = crate::src::zlib::inflate::LENLENS
                }
                current_block = 11322929247169729670;
            }
            16 => {
                current_block = 11322929247169729670;
            }
            17 => {
                current_block = 6177865312519592116;
            }
            18 => {
                current_block = 11341304196878840394;
            }
            19 => {
                current_block = 10540587995463041380;
            }
            20 => {
                current_block = 583050819838508811;
            }
            21 => {
                current_block = 6144666487834620188;
            }
            22 => {
                current_block = 12040508763604396018;
            }
            23 => {
                if left == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                let fresh24 = put;
                put = put.offset(1);
                *fresh24 = (*state).length as libc::c_uchar;
                left = left.wrapping_sub(1);
                (*state).mode = crate::src::zlib::inflate::LEN;
                continue;
            }
            24 => {
                if (*state).wrap != 0 {
                    while bits < 32 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh25 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh25 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out = ((*strm).total_out as libc::c_ulong)
                        .wrapping_add(out as libc::c_ulong)
                        as crate::zconf_h::uLong
                        as crate::zconf_h::uLong;
                    (*state).total = (*state).total.wrapping_add(out as libc::c_ulong);
                    if out != 0 {
                        (*state).check = crate::src::zlib::adler32::adler32(
                            (*state).check,
                            put.offset(-(out as isize)),
                            out,
                        );
                        (*strm).adler = (*state).check
                    }
                    out = left;
                    if (hold >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            hold >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_ulong,
                        )
                        .wrapping_add(
                            (hold & 0xff00 as libc::c_int as libc::c_ulong) << 8 as libc::c_int,
                        )
                        .wrapping_add(
                            (hold & 0xff as libc::c_int as libc::c_ulong) << 24 as libc::c_int,
                        )
                        != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint
                    }
                }
                (*state).mode = crate::src::zlib::inflate::DONE;
                current_block = 1550432445778694857;
            }
            26 => {
                current_block = 1550432445778694857;
            }
            27 => {
                ret = -(3 as libc::c_int);
                break;
            }
            28 => return -(4 as libc::c_int),
            29 | _ => return -(2 as libc::c_int),
        }
        match current_block {
            11322929247169729670 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh5 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh5 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    let fresh6 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh6 as usize] as usize] = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 3 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_ushort;
                    hold >>= 3 as libc::c_int;
                    bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint)
                }
                while (*state).have < 19 as libc::c_int as libc::c_uint {
                    let fresh7 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh7 as usize] as usize] =
                        0 as libc::c_int as libc::c_ushort
                }
                (*state).next = (*state).codes.as_mut_ptr();
                (*state).lencode = (*state).next as *const crate::src::zlib::inftrees::code;
                (*state).lenbits = 7 as libc::c_int as libc::c_uint;
                ret = crate::src::zlib::inftrees::inflate_table(
                    crate::src::zlib::inftrees::CODES,
                    (*state).lens.as_mut_ptr(),
                    19 as libc::c_int as libc::c_uint,
                    &mut (*state).next as *mut _ as *mut *mut crate::src::zlib::inftrees::code,
                    &mut (*state).lenbits,
                    (*state).work.as_mut_ptr(),
                );
                if ret != 0 {
                    (*strm).msg = b"invalid code lengths set\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = crate::src::zlib::inflate::CODELENS
                }
                current_block = 6177865312519592116;
            }
            18432964712698998993 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have
                    }
                    if copy > left {
                        copy = left
                    }
                    if copy == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    crate::stdlib::memcpy(
                        put as *mut libc::c_void,
                        next as *const libc::c_void,
                        copy as libc::c_ulong,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = (*state).length.wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = crate::src::zlib::inflate::TYPE;
                    continue;
                }
            }
            210528378685203046 => {
                if (*state).havedict == 0 as libc::c_int {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return 2 as libc::c_int;
                }
                (*state).check = crate::src::zlib::adler32::adler32(
                    0 as libc::c_long as crate::zconf_h::uLong,
                    0 as *const crate::zconf_h::Bytef,
                    0 as libc::c_int as crate::zconf_h::uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = crate::src::zlib::inflate::TYPE;
                current_block = 10674880093440332853;
            }
            1550432445778694857 => {
                ret = 1 as libc::c_int;
                break;
            }
            _ => {}
        }
        match current_block {
            6177865312519592116 => {
                while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                    loop {
                        this = *(*state).lencode.offset(
                            (hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                as isize,
                        );
                        if this.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh8 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh8 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if (this.val as libc::c_int) < 16 as libc::c_int {
                        while bits < this.bits as libc::c_uint {
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh9 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh9 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                        }
                        hold >>= this.bits as libc::c_int;
                        bits = bits.wrapping_sub(this.bits as libc::c_uint);
                        let fresh10 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[fresh10 as usize] = this.val
                    } else {
                        if this.val as libc::c_int == 16 as libc::c_int {
                            while bits
                                < (this.bits as libc::c_int + 2 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh11 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh11 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= this.bits as libc::c_int;
                            bits = bits.wrapping_sub(this.bits as libc::c_uint);
                            if (*state).have == 0 as libc::c_int as libc::c_uint {
                                (*strm).msg = b"invalid bit length repeat\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                                (*state).mode = crate::src::zlib::inflate::BAD;
                                break;
                            } else {
                                len = (*state).lens[(*state)
                                    .have
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as usize] as libc::c_uint;
                                copy = (3 as libc::c_int as libc::c_uint).wrapping_add(
                                    hold as libc::c_uint
                                        & ((1 as libc::c_uint) << 2 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                                hold >>= 2 as libc::c_int;
                                bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint)
                            }
                        } else if this.val as libc::c_int == 17 as libc::c_int {
                            while bits
                                < (this.bits as libc::c_int + 3 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh12 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh12 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= this.bits as libc::c_int;
                            bits = bits.wrapping_sub(this.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (3 as libc::c_int as libc::c_uint).wrapping_add(
                                hold as libc::c_uint
                                    & ((1 as libc::c_uint) << 3 as libc::c_int)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
                            hold >>= 3 as libc::c_int;
                            bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint)
                        } else {
                            while bits
                                < (this.bits as libc::c_int + 7 as libc::c_int) as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh13 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh13 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= this.bits as libc::c_int;
                            bits = bits.wrapping_sub(this.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (11 as libc::c_int as libc::c_uint).wrapping_add(
                                hold as libc::c_uint
                                    & ((1 as libc::c_uint) << 7 as libc::c_int)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
                            hold >>= 7 as libc::c_int;
                            bits = bits.wrapping_sub(7 as libc::c_int as libc::c_uint)
                        }
                        if (*state).have.wrapping_add(copy)
                            > (*state).nlen.wrapping_add((*state).ndist)
                        {
                            (*strm).msg = b"invalid bit length repeat\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            break;
                        } else {
                            loop {
                                let fresh14 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(fresh14 != 0) {
                                    break;
                                }
                                let fresh15 = (*state).have;
                                (*state).have = (*state).have.wrapping_add(1);
                                (*state).lens[fresh15 as usize] = len as libc::c_ushort
                            }
                        }
                    }
                }
                /* handle error breaks in while */
                if (*state).mode as libc::c_uint
                    == crate::src::zlib::inflate::BAD as libc::c_int as libc::c_uint
                {
                    continue;
                }
                /* build code tables */
                (*state).next = (*state).codes.as_mut_ptr();
                (*state).lencode = (*state).next as *const crate::src::zlib::inftrees::code;
                (*state).lenbits = 9 as libc::c_int as libc::c_uint;
                ret = crate::src::zlib::inftrees::inflate_table(
                    crate::src::zlib::inftrees::LENS,
                    (*state).lens.as_mut_ptr(),
                    (*state).nlen,
                    &mut (*state).next as *mut _ as *mut *mut crate::src::zlib::inftrees::code,
                    &mut (*state).lenbits,
                    (*state).work.as_mut_ptr(),
                );
                if ret != 0 {
                    (*strm).msg = b"invalid literal/lengths set\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).distcode = (*state).next as *const crate::src::zlib::inftrees::code;
                    (*state).distbits = 6 as libc::c_int as libc::c_uint;
                    ret = crate::src::zlib::inftrees::inflate_table(
                        crate::src::zlib::inftrees::DISTS,
                        (*state).lens.as_mut_ptr().offset((*state).nlen as isize),
                        (*state).ndist,
                        &mut (*state).next as *mut _ as *mut *mut crate::src::zlib::inftrees::code,
                        &mut (*state).distbits,
                        (*state).work.as_mut_ptr(),
                    );
                    if ret != 0 {
                        (*strm).msg = b"invalid distances set\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        (*state).mode = crate::src::zlib::inflate::LEN
                    }
                }
                current_block = 11341304196878840394;
            }
            10674880093440332853 => {
                if flush == 5 as libc::c_int {
                    break;
                }
                current_block = 14847832218395804385;
            }
            _ => {}
        }
        match current_block {
            11341304196878840394 => {
                if have >= 6 as libc::c_int as libc::c_uint
                    && left >= 258 as libc::c_int as libc::c_uint
                {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    crate::src::zlib::inffast::inflate_fast(
                        strm as *mut crate::zlib_h::z_stream_s,
                        out,
                    );
                    put = (*strm).next_out;
                    left = (*strm).avail_out;
                    next = (*strm).next_in;
                    have = (*strm).avail_in;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    continue;
                } else {
                    loop {
                        this = *(*state).lencode.offset(
                            (hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).lenbits)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                as isize,
                        );
                        if this.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh16 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh16 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    if this.op as libc::c_int != 0
                        && this.op as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int
                    {
                        last = this;
                        loop {
                            this = *(*state).lencode.offset(
                                (last.val as libc::c_uint).wrapping_add(
                                    (hold as libc::c_uint
                                        & ((1 as libc::c_uint)
                                            << last.bits as libc::c_int + last.op as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                        >> last.bits as libc::c_int,
                                ) as isize,
                            );
                            if (last.bits as libc::c_int + this.bits as libc::c_int) as libc::c_uint
                                <= bits
                            {
                                break;
                            }
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh17 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh17 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                        }
                        hold >>= last.bits as libc::c_int;
                        bits = bits.wrapping_sub(last.bits as libc::c_uint)
                    }
                    hold >>= this.bits as libc::c_int;
                    bits = bits.wrapping_sub(this.bits as libc::c_uint);
                    (*state).length = this.val as libc::c_uint;
                    if this.op as libc::c_int == 0 as libc::c_int {
                        (*state).mode = crate::src::zlib::inflate::LIT;
                        continue;
                    } else if this.op as libc::c_int & 32 as libc::c_int != 0 {
                        (*state).mode = crate::src::zlib::inflate::TYPE;
                        continue;
                    } else if this.op as libc::c_int & 64 as libc::c_int != 0 {
                        (*strm).msg = b"invalid literal/length code\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        (*state).extra =
                            this.op as libc::c_uint & 15 as libc::c_int as libc::c_uint;
                        (*state).mode = crate::src::zlib::inflate::LENEXT
                    }
                }
                current_block = 10540587995463041380;
            }
            14847832218395804385 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as libc::c_int as libc::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                    (*state).mode = crate::src::zlib::inflate::CHECK;
                    continue;
                } else {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh2 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh2 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).last = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 1 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_int;
                    hold >>= 1 as libc::c_int;
                    bits = bits.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    match hold as libc::c_uint
                        & ((1 as libc::c_uint) << 2 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    {
                        0 => {
                            /* stored block */
                            (*state).mode = crate::src::zlib::inflate::STORED
                        }
                        1 => {
                            /* fixed block */
                            fixedtables(state); /* decode codes */
                            (*state).mode = crate::src::zlib::inflate::LEN
                        }
                        2 => {
                            /* dynamic block */
                            (*state).mode = crate::src::zlib::inflate::TABLE
                        }
                        3 => {
                            (*strm).msg = b"invalid block type\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD
                        }
                        _ => {}
                    }
                    hold >>= 2 as libc::c_int;
                    bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                    continue;
                }
            }
            _ => {}
        }
        match current_block {
            10540587995463041380 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh18 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as libc::c_uint
                            & ((1 as libc::c_uint) << (*state).extra)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra)
                }
                (*state).mode = crate::src::zlib::inflate::DIST;
                current_block = 583050819838508811;
            }
            _ => {}
        }
        loop {
            match current_block {
                583050819838508811 => {
                    this = *(*state).distcode.offset(
                        (hold as libc::c_uint
                            & ((1 as libc::c_uint) << (*state).distbits)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                            as isize,
                    );
                    if this.bits as libc::c_uint <= bits {
                        if this.op as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int {
                            last = this;
                            loop {
                                this = *(*state).distcode.offset(
                                    (last.val as libc::c_uint).wrapping_add(
                                        (hold as libc::c_uint
                                            & ((1 as libc::c_uint)
                                                << last.bits as libc::c_int
                                                    + last.op as libc::c_int)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                            >> last.bits as libc::c_int,
                                    ) as isize,
                                );
                                if (last.bits as libc::c_int + this.bits as libc::c_int)
                                    as libc::c_uint
                                    <= bits
                                {
                                    break;
                                }
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh20 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh20 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                            hold >>= last.bits as libc::c_int;
                            bits = bits.wrapping_sub(last.bits as libc::c_uint)
                        }
                        hold >>= this.bits as libc::c_int;
                        bits = bits.wrapping_sub(this.bits as libc::c_uint);
                        if this.op as libc::c_int & 64 as libc::c_int != 0 {
                            (*strm).msg = b"invalid distance code\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            break;
                        } else {
                            (*state).offset = this.val as libc::c_uint;
                            (*state).extra =
                                this.op as libc::c_uint & 15 as libc::c_int as libc::c_uint;
                            (*state).mode = crate::src::zlib::inflate::DISTEXT;
                            current_block = 6144666487834620188;
                        }
                    } else {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh19 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh19 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                        current_block = 583050819838508811;
                    }
                }
                6144666487834620188 => {
                    if (*state).extra != 0 {
                        while bits < (*state).extra {
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh21 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh21 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).extra)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra)
                    }
                    if (*state).offset > (*state).whave.wrapping_add(out).wrapping_sub(left) {
                        (*strm).msg = b"invalid distance too far back\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        break;
                    } else {
                        (*state).mode = crate::src::zlib::inflate::MATCH;
                        current_block = 12040508763604396018;
                    }
                }
                _ => {
                    if left == 0 as libc::c_int as libc::c_uint {
                        break 's_114;
                    }
                    copy = out.wrapping_sub(left);
                    if (*state).offset > copy {
                        /* copy from window */
                        copy = (*state).offset.wrapping_sub(copy);
                        if copy > (*state).write {
                            copy = copy.wrapping_sub((*state).write);
                            from = (*state)
                                .window
                                .offset((*state).wsize.wrapping_sub(copy) as isize)
                        } else {
                            from = (*state)
                                .window
                                .offset((*state).write.wrapping_sub(copy) as isize)
                        }
                        if copy > (*state).length {
                            copy = (*state).length
                        }
                    } else {
                        /* copy from output */
                        from = put.offset(-((*state).offset as isize));
                        copy = (*state).length
                    }
                    if copy > left {
                        copy = left
                    }
                    left = left.wrapping_sub(copy);
                    (*state).length = (*state).length.wrapping_sub(copy);
                    loop {
                        let fresh22 = from;
                        from = from.offset(1);
                        let fresh23 = put;
                        put = put.offset(1);
                        *fresh23 = *fresh22;
                        copy = copy.wrapping_sub(1);
                        if !(copy != 0) {
                            break;
                        }
                    }
                    if (*state).length == 0 as libc::c_int as libc::c_uint {
                        (*state).mode = crate::src::zlib::inflate::LEN
                    }
                    break;
                }
            }
        }
    }
    /*
      Return from inflate(), updating the total counts and the check value.
      If there was no progress during the inflate() call, return a buffer
      error.  Call updatewindow() to create and/or update the window state.
      Note: a memory error from inflate() is non-recoverable.
    */
    (*strm).next_out = put;
    (*strm).avail_out = left;
    (*strm).next_in = next;
    (*strm).avail_in = have;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || ((*state).mode as libc::c_uint)
            < crate::src::zlib::inflate::CHECK as libc::c_int as libc::c_uint
            && out != (*strm).avail_out
    {
        if updatewindow(strm, out) != 0 {
            (*state).mode = crate::src::zlib::inflate::MEM;
            return -(4 as libc::c_int);
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm).total_in = ((*strm).total_in as libc::c_ulong).wrapping_add(in_0 as libc::c_ulong)
        as crate::zconf_h::uLong as crate::zconf_h::uLong;
    (*strm).total_out = ((*strm).total_out as libc::c_ulong).wrapping_add(out as libc::c_ulong)
        as crate::zconf_h::uLong as crate::zconf_h::uLong;
    (*state).total = (*state).total.wrapping_add(out as libc::c_ulong);
    if (*state).wrap != 0 && out != 0 {
        (*state).check = crate::src::zlib::adler32::adler32(
            (*state).check,
            (*strm).next_out.offset(-(out as isize)),
            out,
        );
        (*strm).adler = (*state).check
    }
    (*strm).data_type = (*state)
        .bits
        .wrapping_add(
            (if (*state).last != 0 {
                64 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        )
        .wrapping_add(
            (if (*state).mode as libc::c_uint
                == crate::src::zlib::inflate::TYPE as libc::c_int as libc::c_uint
            {
                128 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        ) as libc::c_int;
    if (in_0 == 0 as libc::c_int as libc::c_uint && out == 0 as libc::c_int as libc::c_uint
        || flush == 4 as libc::c_int)
        && ret == 0 as libc::c_int
    {
        ret = -(5 as libc::c_int)
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn inflateEnd(mut strm: crate::zlib_h::z_streamp) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as crate::zconf_h::voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as crate::zconf_h::voidpf,
    );
    (*strm).state = 0 as *mut crate::zlib_h::internal_state;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: crate::zlib_h::z_streamp,
    mut dictionary: *const crate::zconf_h::Bytef,
    mut dictLength: crate::zconf_h::uInt,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    let mut id: libc::c_ulong = 0;
    /* check state */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).wrap != 0 as libc::c_int
        && (*state).mode as libc::c_uint
            != crate::src::zlib::inflate::DICT as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    /* check for correct dictionary id */
    if (*state).mode as libc::c_uint
        == crate::src::zlib::inflate::DICT as libc::c_int as libc::c_uint
    {
        id = crate::src::zlib::adler32::adler32(
            0 as libc::c_long as crate::zconf_h::uLong,
            0 as *const crate::zconf_h::Bytef,
            0 as libc::c_int as crate::zconf_h::uInt,
        );
        id = crate::src::zlib::adler32::adler32(id, dictionary, dictLength);
        if id != (*state).check {
            return -(3 as libc::c_int);
        }
    }
    /* copy dictionary to window */
    if updatewindow(strm, (*strm).avail_out) != 0 {
        (*state).mode = crate::src::zlib::inflate::MEM;
        return -(4 as libc::c_int);
    }
    if dictLength > (*state).wsize {
        crate::stdlib::memcpy(
            (*state).window as *mut libc::c_void,
            dictionary
                .offset(dictLength as isize)
                .offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as libc::c_ulong,
        );
        (*state).whave = (*state).wsize
    } else {
        crate::stdlib::memcpy(
            (*state)
                .window
                .offset((*state).wsize as isize)
                .offset(-(dictLength as isize)) as *mut libc::c_void,
            dictionary as *const libc::c_void,
            dictLength as libc::c_ulong,
        );
        (*state).whave = dictLength
    }
    (*state).havedict = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn inflateGetHeader(
    mut strm: crate::zlib_h::z_streamp,
    mut head: crate::zlib_h::gz_headerp,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    /* check state */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).wrap & 2 as libc::c_int == 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    /* save header structure */
    (*state).head = head;
    (*head).done = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
  Search buf[0..len-1] for the pattern: 0, 0, 0xff, 0xff.  Return when found
  or when out of input.  When called, *have is the number of pattern bytes
  found in order so far, in 0..3.  On return *have is updated to the new
  state.  If on return *have equals four, then the pattern was found and the
  return value is how many bytes were read including the last byte of the
  pattern.  If *have is less than four, then the pattern has not been found
  yet and the return value is len.  In the latter case, syncsearch() can be
  called again with more data and the *have state.  *have is initialized to
  zero for the first call.
*/

unsafe extern "C" fn syncsearch(
    mut have: *mut libc::c_uint,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_uint {
    let mut got: libc::c_uint = 0; /* number of bytes to look at or looked at */
    let mut next: libc::c_uint = 0; /* temporary to save total_in and total_out */
    got = *have; /* to restore bit buffer to byte string */
    next = 0 as libc::c_int as libc::c_uint;
    while next < len && got < 4 as libc::c_int as libc::c_uint {
        if *buf.offset(next as isize) as libc::c_int
            == (if got < 2 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                0xff as libc::c_int
            })
        {
            got = got.wrapping_add(1)
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as libc::c_int as libc::c_uint
        } else {
            got = (4 as libc::c_int as libc::c_uint).wrapping_sub(got)
        }
        next = next.wrapping_add(1)
    }
    *have = got;
    return next;
}
#[no_mangle]

pub unsafe extern "C" fn inflateSync(mut strm: crate::zlib_h::z_streamp) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut in_0: libc::c_ulong = 0;
    let mut out: libc::c_ulong = 0;
    let mut buf: [libc::c_uchar; 4] = [0; 4];
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    /* check parameters */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && (*state).bits < 8 as libc::c_int as libc::c_uint
    {
        return -(5 as libc::c_int);
    }
    /* if first time, start search in bit buffer */
    if (*state).mode as libc::c_uint
        != crate::src::zlib::inflate::SYNC as libc::c_int as libc::c_uint
    {
        (*state).mode = crate::src::zlib::inflate::SYNC;
        (*state).hold <<= (*state).bits & 7 as libc::c_int as libc::c_uint;
        (*state).bits = (*state)
            .bits
            .wrapping_sub((*state).bits & 7 as libc::c_int as libc::c_uint);
        len = 0 as libc::c_int as libc::c_uint;
        while (*state).bits >= 8 as libc::c_int as libc::c_uint {
            let fresh26 = len;
            len = len.wrapping_add(1);
            buf[fresh26 as usize] = (*state).hold as libc::c_uchar;
            (*state).hold >>= 8 as libc::c_int;
            (*state).bits = (*state).bits.wrapping_sub(8 as libc::c_int as libc::c_uint)
        }
        (*state).have = 0 as libc::c_int as libc::c_uint;
        syncsearch(&mut (*state).have, buf.as_mut_ptr(), len);
    }
    /* search available input */
    len = syncsearch(&mut (*state).have, (*strm).next_in, (*strm).avail_in);
    (*strm).avail_in = ((*strm).avail_in as libc::c_uint).wrapping_sub(len) as crate::zconf_h::uInt
        as crate::zconf_h::uInt;
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = ((*strm).total_in as libc::c_ulong).wrapping_add(len as libc::c_ulong)
        as crate::zconf_h::uLong as crate::zconf_h::uLong;
    /* return no joy or set up to restart inflate() on a new block */
    if (*state).have != 4 as libc::c_int as libc::c_uint {
        return -(3 as libc::c_int);
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = crate::src::zlib::inflate::TYPE;
    return 0 as libc::c_int;
}
/*
     Sets the destination stream as a complete copy of the source stream.

     This function can be useful when randomly accessing a large stream.  The
   first pass through the stream can periodically record the inflate state,
   allowing restarting inflate at those points when randomly accessing the
   stream.

     inflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
   (such as zalloc being NULL). msg is left unchanged in both source and
   destination.
*/
/*
     This function is equivalent to inflateEnd followed by inflateInit,
   but does not free and reallocate all the internal decompression state.
   The stream will keep attributes that may have been set by inflateInit2.

      inflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being NULL).
*/
/*
     This function inserts bits in the inflate input stream.  The intent is
  that this function is used to start inflating at a bit position in the
  middle of a byte.  The provided bits will be used before any bytes are used
  from next_in.  This function should only be used with raw inflate, and
  should be used before the first inflate() call after inflateInit2() or
  inflateReset().  bits must be less than or equal to 16, and that many of the
  least significant bits of value will be inserted in the input.

      inflatePrime returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
      inflateGetHeader() requests that gzip header information be stored in the
   provided gz_header structure.  inflateGetHeader() may be called after
   inflateInit2() or inflateReset(), and before the first call of inflate().
   As inflate() processes the gzip stream, head->done is zero until the header
   is completed, at which time head->done is set to one.  If a zlib stream is
   being decoded, then head->done is set to -1 to indicate that there will be
   no gzip header information forthcoming.  Note that Z_BLOCK can be used to
   force inflate() to return immediately after header processing is complete
   and before any actual data is decompressed.

      The text, time, xflags, and os fields are filled in with the gzip header
   contents.  hcrc is set to true if there is a header CRC.  (The header CRC
   was valid if done is set to one.)  If extra is not Z_NULL, then extra_max
   contains the maximum number of bytes to write to extra.  Once done is true,
   extra_len contains the actual extra field length, and extra contains the
   extra field, or that field truncated if extra_max is less than extra_len.
   If name is not Z_NULL, then up to name_max characters are written there,
   terminated with a zero unless the length is greater than name_max.  If
   comment is not Z_NULL, then up to comm_max characters are written there,
   terminated with a zero unless the length is greater than comm_max.  When
   any of extra, name, or comment are not Z_NULL and the respective field is
   not present in the header, then that field is set to Z_NULL to signal its
   absence.  This allows the use of deflateSetHeader() with the returned
   structure to duplicate the header.  However if those fields are set to
   allocated memory, then the application will need to save those pointers
   elsewhere so that they can be eventually freed.

      If inflateGetHeader is not used, then the header information is simply
   discarded.  The header is always checked for validity, including the header
   CRC if present.  inflateReset() will reset the process to discard the header
   information.  The application would need to call inflateGetHeader() again to
   retrieve the header from the next gzip stream.

      inflateGetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateBackInit OF((z_streamp strm, int windowBits,
                                        unsigned char FAR *window));

     Initialize the internal stream state for decompression using inflateBack()
   calls.  The fields zalloc, zfree and opaque in strm must be initialized
   before the call.  If zalloc and zfree are Z_NULL, then the default library-
   derived memory allocation routines are used.  windowBits is the base two
   logarithm of the window size, in the range 8..15.  window is a caller
   supplied buffer of that size.  Except for special applications where it is
   assured that deflate was used with small window sizes, windowBits must be 15
   and a 32K byte window must be supplied to be able to decompress general
   deflate streams.

     See inflateBack() for the usage of these routines.

     inflateBackInit will return Z_OK on success, Z_STREAM_ERROR if any of
   the paramaters are invalid, Z_MEM_ERROR if the internal state could not
   be allocated, or Z_VERSION_ERROR if the version of the library does not
   match the version of the header file.
*/
/*
     inflateBack() does a raw inflate with a single call using a call-back
   interface for input and output.  This is more efficient than inflate() for
   file i/o applications in that it avoids copying between the output and the
   sliding window by simply making the window itself the output buffer.  This
   function trusts the application to not change the output buffer passed by
   the output function, at least until inflateBack() returns.

     inflateBackInit() must be called first to allocate the internal state
   and to initialize the state with the user-provided window buffer.
   inflateBack() may then be used multiple times to inflate a complete, raw
   deflate stream with each call.  inflateBackEnd() is then called to free
   the allocated state.

     A raw deflate stream is one with no zlib or gzip header or trailer.
   This routine would normally be used in a utility that reads zip or gzip
   files and writes out uncompressed files.  The utility would decode the
   header and process the trailer on its own, hence this routine expects
   only the raw deflate stream to decompress.  This is different from the
   normal behavior of inflate(), which expects either a zlib or gzip header and
   trailer around the deflate stream.

     inflateBack() uses two subroutines supplied by the caller that are then
   called by inflateBack() for input and output.  inflateBack() calls those
   routines until it reads a complete deflate stream and writes out all of the
   uncompressed data, or until it encounters an error.  The function's
   parameters and return types are defined above in the in_func and out_func
   typedefs.  inflateBack() will call in(in_desc, &buf) which should return the
   number of bytes of provided input, and a pointer to that input in buf.  If
   there is no input available, in() must return zero--buf is ignored in that
   case--and inflateBack() will return a buffer error.  inflateBack() will call
   out(out_desc, buf, len) to write the uncompressed data buf[0..len-1].  out()
   should return zero on success, or non-zero on failure.  If out() returns
   non-zero, inflateBack() will return with an error.  Neither in() nor out()
   are permitted to change the contents of the window provided to
   inflateBackInit(), which is also the buffer that out() uses to write from.
   The length written by out() will be at most the window size.  Any non-zero
   amount of input may be provided by in().

     For convenience, inflateBack() can be provided input on the first call by
   setting strm->next_in and strm->avail_in.  If that input is exhausted, then
   in() will be called.  Therefore strm->next_in must be initialized before
   calling inflateBack().  If strm->next_in is Z_NULL, then in() will be called
   immediately for input.  If strm->next_in is not Z_NULL, then strm->avail_in
   must also be initialized, and then if strm->avail_in is not zero, input will
   initially be taken from strm->next_in[0 .. strm->avail_in - 1].

     The in_desc and out_desc parameters of inflateBack() is passed as the
   first parameter of in() and out() respectively when they are called.  These
   descriptors can be optionally used to pass any information that the caller-
   supplied in() and out() functions need to do their job.

     On return, inflateBack() will set strm->next_in and strm->avail_in to
   pass back any unused input that was provided by the last in() call.  The
   return values of inflateBack() can be Z_STREAM_END on success, Z_BUF_ERROR
   if in() or out() returned an error, Z_DATA_ERROR if there was a format
   error in the deflate stream (in which case strm->msg is set to indicate the
   nature of the error), or Z_STREAM_ERROR if the stream was not properly
   initialized.  In the case of Z_BUF_ERROR, an input or output error can be
   distinguished using strm->next_in which will be Z_NULL only if in() returned
   an error.  If strm->next is not Z_NULL, then the Z_BUF_ERROR was due to
   out() returning non-zero.  (in() will always be called before out(), so
   strm->next_in is assured to be defined if out() returns non-zero.)  Note
   that inflateBack() cannot return Z_OK.
*/
/*
     All memory allocated by inflateBackInit() is freed.

     inflateBackEnd() returns Z_OK on success, or Z_STREAM_ERROR if the stream
   state was inconsistent.
*/
/* Return flags indicating compile-time options.

   Type sizes, two bits each, 00 = 16 bits, 01 = 32, 10 = 64, 11 = other:
    1.0: size of uInt
    3.2: size of uLong
    5.4: size of voidpf (pointer)
    7.6: size of z_off_t

   Compiler, assembler, and debug options:
    8: DEBUG
    9: ASMV or ASMINF -- use ASM code
    10: ZLIB_WINAPI -- exported functions use the WINAPI calling convention
    11: 0 (reserved)

   One-time table building (smaller code, but not thread-safe if true):
    12: BUILDFIXED -- build static block decoding tables when needed
    13: DYNAMIC_CRC_TABLE -- build CRC calculation tables when needed
    14,15: 0 (reserved)

   Library content (indicates missing functionality):
    16: NO_GZCOMPRESS -- gz* functions cannot compress (to avoid linking
                         deflate code when not needed)
    17: NO_GZIP -- deflate can't write gzip streams, and inflate can't detect
                   and decode gzip streams (to avoid linking crc code)
    18-19: 0 (reserved)

   Operation variations (changes in library functionality):
    20: PKZIP_BUG_WORKAROUND -- slightly more permissive inflate
    21: FASTEST -- deflate algorithm with only one, lowest compression level
    22,23: 0 (reserved)

   The sprintf variant used by gzprintf (zero is best):
    24: 0 = vs*, 1 = s* -- 1 means limited to 20 arguments after the format
    25: 0 = *nprintf, 1 = *printf -- 1 means gzprintf() not secure!
    26: 0 = returns value, 1 = void -- 1 means inferred string length returned

   Remainder:
    27-31: 0 (reserved)
*/
/* utility functions */
/*
     The following utility functions are implemented on top of the
   basic stream-oriented functions. To simplify the interface, some
   default options are assumed (compression level and memory usage,
   standard memory allocation functions). The source code of these
   utility functions can easily be modified if you need special options.
*/
/*
     Compresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer. Upon entry, destLen is the total
   size of the destination buffer, which must be at least the value returned
   by compressBound(sourceLen). Upon exit, destLen is the actual size of the
   compressed buffer.
     This function can be used to compress a whole file at once if the
   input file is mmap'ed.
     compress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer.
*/
/*
     Compresses the source buffer into the destination buffer. The level
   parameter has the same meaning as in deflateInit.  sourceLen is the byte
   length of the source buffer. Upon entry, destLen is the total size of the
   destination buffer, which must be at least the value returned by
   compressBound(sourceLen). Upon exit, destLen is the actual size of the
   compressed buffer.

     compress2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_BUF_ERROR if there was not enough room in the output buffer,
   Z_STREAM_ERROR if the level parameter is invalid.
*/
/*
     compressBound() returns an upper bound on the compressed size after
   compress() or compress2() on sourceLen bytes.  It would be used before
   a compress() or compress2() call to allocate the destination buffer.
*/
/*
     Decompresses the source buffer into the destination buffer.  sourceLen is
   the byte length of the source buffer. Upon entry, destLen is the total
   size of the destination buffer, which must be large enough to hold the
   entire uncompressed data. (The size of the uncompressed data must have
   been saved previously by the compressor and transmitted to the decompressor
   by some mechanism outside the scope of this compression library.)
   Upon exit, destLen is the actual size of the compressed buffer.
     This function can be used to decompress a whole file at once if the
   input file is mmap'ed.

     uncompress returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_BUF_ERROR if there was not enough room in the output
   buffer, or Z_DATA_ERROR if the input data was corrupted or incomplete.
*/
/*
  Opens a gzip (.gz) file for reading or writing. The mode parameter
is as in fopen ("rb" or "wb") but can also include a compression level
("wb9") or a strategy: 'f' for filtered data as in "wb6f", 'h' for
Huffman only compression as in "wb1h", or 'R' for run-length encoding
as in "wb1R". (See the description of deflateInit2 for more information
about the strategy parameter.)

  gzopen can be used to read a file which is not in gzip format; in this
case gzread will directly read from the file without decompression.

  gzopen returns NULL if the file could not be opened or if there was
insufficient memory to allocate the (de)compression state; errno
can be checked to distinguish the two cases (if errno is zero, the
zlib error is Z_MEM_ERROR).  */
/*
     gzdopen() associates a gzFile with the file descriptor fd.  File
   descriptors are obtained from calls like open, dup, creat, pipe or
   fileno (in the file has been previously opened with fopen).
   The mode parameter is as in gzopen.
     The next call of gzclose on the returned gzFile will also close the
   file descriptor fd, just like fclose(fdopen(fd), mode) closes the file
   descriptor fd. If you want to keep fd open, use gzdopen(dup(fd), mode).
     gzdopen returns NULL if there was insufficient memory to allocate
   the (de)compression state.
*/
/*
     Dynamically update the compression level or strategy. See the description
   of deflateInit2 for the meaning of these parameters.
     gzsetparams returns Z_OK if success, or Z_STREAM_ERROR if the file was not
   opened for writing.
*/
/*
  Reads the given number of uncompressed bytes from the compressed file.
If the input file was not in gzip format, gzread copies the given number
of bytes into the buffer.
  gzread returns the number of uncompressed bytes actually read (0 for
end of file, -1 for error). */
/*
     Writes the given number of uncompressed bytes into the compressed file.
   gzwrite returns the number of uncompressed bytes actually written
   (0 in case of error).
*/
/*
     Converts, formats, and writes the args to the compressed file under
   control of the format string, as in fprintf. gzprintf returns the number of
   uncompressed bytes actually written (0 in case of error).  The number of
   uncompressed bytes written is limited to 4095. The caller should assure that
   this limit is not exceeded. If it is exceeded, then gzprintf() will return
   return an error (0) with nothing written. In this case, there may also be a
   buffer overflow with unpredictable consequences, which is possible only if
   zlib was compiled with the insecure functions sprintf() or vsprintf()
   because the secure snprintf() or vsnprintf() functions were not available.
*/
/*
      Writes the given null-terminated string to the compressed file, excluding
   the terminating null character.
      gzputs returns the number of characters written, or -1 in case of error.
*/
/*
      Reads bytes from the compressed file until len-1 characters are read, or
   a newline character is read and transferred to buf, or an end-of-file
   condition is encountered.  The string is then terminated with a null
   character.
      gzgets returns buf, or Z_NULL in case of error.
*/
/*
      Writes c, converted to an unsigned char, into the compressed file.
   gzputc returns the value that was written, or -1 in case of error.
*/
/*
      Reads one byte from the compressed file. gzgetc returns this byte
   or -1 in case of end of file or error.
*/
/*
      Push one character back onto the stream to be read again later.
   Only one character of push-back is allowed.  gzungetc() returns the
   character pushed, or -1 on failure.  gzungetc() will fail if a
   character has been pushed but not read yet, or if c is -1. The pushed
   character will be discarded if the stream is repositioned with gzseek()
   or gzrewind().
*/
/*
     Flushes all pending output into the compressed file. The parameter
   flush is as in the deflate() function. The return value is the zlib
   error number (see function gzerror below). gzflush returns Z_OK if
   the flush parameter is Z_FINISH and all output could be flushed.
     gzflush should be called only when strictly necessary because it can
   degrade compression.
*/
/*
      Sets the starting position for the next gzread or gzwrite on the
   given compressed file. The offset represents a number of bytes in the
   uncompressed data stream. The whence parameter is defined as in lseek(2);
   the value SEEK_END is not supported.
     If the file is opened for reading, this function is emulated but can be
   extremely slow. If the file is opened for writing, only forward seeks are
   supported; gzseek then compresses a sequence of zeroes up to the new
   starting position.

      gzseek returns the resulting offset location as measured in bytes from
   the beginning of the uncompressed stream, or -1 in case of error, in
   particular if the file is opened for writing and the new starting position
   would be before the current position.
*/
/*
     Rewinds the given file. This function is supported only for reading.

   gzrewind(file) is equivalent to (int)gzseek(file, 0L, SEEK_SET)
*/
/*
     Returns the starting position for the next gzread or gzwrite on the
   given compressed file. This position represents a number of bytes in the
   uncompressed data stream.

   gztell(file) is equivalent to gzseek(file, 0L, SEEK_CUR)
*/
/*
     Returns 1 when EOF has previously been detected reading the given
   input stream, otherwise zero.
*/
/*
     Returns 1 if file is being read directly without decompression, otherwise
   zero.
*/
/*
     Flushes all pending output if necessary, closes the compressed file
   and deallocates all the (de)compression state. The return value is the zlib
   error number (see function gzerror below).
*/
/*
     Returns the error message for the last error which occurred on the
   given compressed file. errnum is set to zlib error number. If an
   error occurred in the file system and not in the compression library,
   errnum is set to Z_ERRNO and the application may consult errno
   to get the exact error code.
*/
/*
     Clears the error and end-of-file flags for file. This is analogous to the
   clearerr() function in stdio. This is useful for continuing to read a gzip
   file that is being written concurrently.
*/
/* checksum functions */
/*
     These functions are not related to compression but are exported
   anyway because they might be useful in applications using the
   compression library.
*/
/*
     Update a running Adler-32 checksum with the bytes buf[0..len-1] and
   return the updated checksum. If buf is NULL, this function returns
   the required initial value for the checksum.
   An Adler-32 checksum is almost as reliable as a CRC32 but can be computed
   much faster. Usage example:

     uLong adler = adler32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       adler = adler32(adler, buffer, length);
     }
     if (adler != original_adler) error();
*/
/*
     Combine two Adler-32 checksums into one.  For two sequences of bytes, seq1
   and seq2 with lengths len1 and len2, Adler-32 checksums were calculated for
   each, adler1 and adler2.  adler32_combine() returns the Adler-32 checksum of
   seq1 and seq2 concatenated, requiring only adler1, adler2, and len2.
*/
/*
     Update a running CRC-32 with the bytes buf[0..len-1] and return the
   updated CRC-32. If buf is NULL, this function returns the required initial
   value for the for the crc. Pre- and post-conditioning (one's complement) is
   performed within this function so it shouldn't be done by the application.
   Usage example:

     uLong crc = crc32(0L, Z_NULL, 0);

     while (read_buffer(buffer, length) != EOF) {
       crc = crc32(crc, buffer, length);
     }
     if (crc != original_crc) error();
*/
/*
     Combine two CRC-32 check values into one.  For two sequences of bytes,
   seq1 and seq2 with lengths len1 and len2, CRC-32 check values were
   calculated for each, crc1 and crc2.  crc32_combine() returns the CRC-32
   check value of seq1 and seq2 concatenated, requiring only crc1, crc2, and
   len2.
*/
/* various hacks, don't look :) */
/* deflateInit and inflateInit are macros to allow checking the zlib version
 * and the compiler's view of z_stream:
 */
/*
  Returns true if inflate is currently at the end of a block generated by
  Z_SYNC_FLUSH or Z_FULL_FLUSH. This function is used by one PPP
  implementation to provide an additional safety check. PPP uses
  Z_SYNC_FLUSH but removes the length bytes of the resulting empty stored
  block. When decompressing, PPP checks that at the end of input packet,
  inflate is waiting for these length bytes.
*/
#[no_mangle]

pub unsafe extern "C" fn inflateSyncPoint(mut strm: crate::zlib_h::z_streamp) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    return ((*state).mode as libc::c_uint
        == crate::src::zlib::inflate::STORED as libc::c_int as libc::c_uint
        && (*state).bits == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
/*
     Initializes the compression dictionary from the given byte sequence
   without producing any compressed output. This function must be called
   immediately after deflateInit, deflateInit2 or deflateReset, before any
   call of deflate. The compressor and decompressor must use exactly the same
   dictionary (see inflateSetDictionary).

     The dictionary should consist of strings (byte sequences) that are likely
   to be encountered later in the data to be compressed, with the most commonly
   used strings preferably put towards the end of the dictionary. Using a
   dictionary is most useful when the data to be compressed is short and can be
   predicted with good accuracy; the data can then be compressed better than
   with the default empty dictionary.

     Depending on the size of the compression data structures selected by
   deflateInit or deflateInit2, a part of the dictionary may in effect be
   discarded, for example if the dictionary is larger than the window size in
   deflate or deflate2. Thus the strings most likely to be useful should be
   put at the end of the dictionary, not at the front. In addition, the
   current implementation of deflate will use at most the window size minus
   262 bytes of the provided dictionary.

     Upon return of this function, strm->adler is set to the adler32 value
   of the dictionary; the decompressor may later use this value to determine
   which dictionary has been used by the compressor. (The adler32 value
   applies to the whole dictionary even if only a subset of the dictionary is
   actually used by the compressor.) If a raw deflate was requested, then the
   adler32 value is not computed and strm->adler is not set.

     deflateSetDictionary returns Z_OK if success, or Z_STREAM_ERROR if a
   parameter is invalid (such as NULL dictionary) or the stream state is
   inconsistent (for example if deflate has already been called for this stream
   or if the compression method is bsort). deflateSetDictionary does not
   perform any compression: this will be done by deflate().
*/
/*
     Sets the destination stream as a complete copy of the source stream.

     This function can be useful when several compression strategies will be
   tried, for example when there are several ways of pre-processing the input
   data with a filter. The streams that will be discarded should then be freed
   by calling deflateEnd.  Note that deflateCopy duplicates the internal
   compression state which can be quite large, so this strategy is slow and
   can consume lots of memory.

     deflateCopy returns Z_OK if success, Z_MEM_ERROR if there was not
   enough memory, Z_STREAM_ERROR if the source stream state was inconsistent
   (such as zalloc being NULL). msg is left unchanged in both source and
   destination.
*/
/*
     This function is equivalent to deflateEnd followed by deflateInit,
   but does not free and reallocate all the internal compression state.
   The stream will keep the same compression level and any other attributes
   that may have been set by deflateInit2.

      deflateReset returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent (such as zalloc or state being NULL).
*/
/*
     Dynamically update the compression level and compression strategy.  The
   interpretation of level and strategy is as in deflateInit2.  This can be
   used to switch between compression and straight copy of the input data, or
   to switch to a different kind of input data requiring a different
   strategy. If the compression level is changed, the input available so far
   is compressed with the old level (and may be flushed); the new level will
   take effect only at the next call of deflate().

     Before the call of deflateParams, the stream state must be set as for
   a call of deflate(), since the currently available input may have to
   be compressed and flushed. In particular, strm->avail_out must be non-zero.

     deflateParams returns Z_OK if success, Z_STREAM_ERROR if the source
   stream state was inconsistent or if a parameter was invalid, Z_BUF_ERROR
   if strm->avail_out was zero.
*/
/*
    Fine tune deflate's internal compression parameters.  This should only be
  used by someone who understands the algorithm used by zlib's deflate for
  searching for the best matching string, and even then only by the most
  fanatic optimizer trying to squeeze out the last compressed bit for their
  specific input data.  Read the deflate.c source code for the meaning of the
  max_lazy, good_length, nice_length, and max_chain parameters.

    deflateTune() can be called after deflateInit() or deflateInit2(), and
  returns Z_OK on success, or Z_STREAM_ERROR for an invalid deflate stream.
*/
/*
     deflateBound() returns an upper bound on the compressed size after
   deflation of sourceLen bytes.  It must be called after deflateInit()
   or deflateInit2().  This would be used to allocate an output buffer
   for deflation in a single pass, and so would be called before deflate().
*/
/*
     deflatePrime() inserts bits in the deflate output stream.  The intent
  is that this function is used to start off the deflate output with the
  bits leftover from a previous deflate stream when appending to it.  As such,
  this function can only be used for raw deflate, and must be used before the
  first deflate() call after a deflateInit2() or deflateReset().  bits must be
  less than or equal to 16, and that many of the least significant bits of
  value will be inserted in the output.

      deflatePrime returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
      deflateSetHeader() provides gzip header information for when a gzip
   stream is requested by deflateInit2().  deflateSetHeader() may be called
   after deflateInit2() or deflateReset() and before the first call of
   deflate().  The text, time, os, extra field, name, and comment information
   in the provided gz_header structure are written to the gzip header (xflag is
   ignored -- the extra flags are set according to the compression level).  The
   caller must assure that, if not Z_NULL, name and comment are terminated with
   a zero byte, and that if extra is not Z_NULL, that extra_len bytes are
   available there.  If hcrc is true, a gzip header crc is included.  Note that
   the current versions of the command-line version of gzip (up through version
   1.3.x) do not support header crc's, and will report that it is a "multi-part
   gzip file" and give up.

      If deflateSetHeader is not used, the default gzip header has text false,
   the time set to zero, and os set to 255, with no extra, name, or comment
   fields.  The gzip header is returned to the default state by deflateReset().

      deflateSetHeader returns Z_OK if success, or Z_STREAM_ERROR if the source
   stream state was inconsistent.
*/
/*
ZEXTERN int ZEXPORT inflateInit2 OF((z_streamp strm,
                                     int  windowBits));

     This is another version of inflateInit with an extra parameter. The
   fields next_in, avail_in, zalloc, zfree and opaque must be initialized
   before by the caller.

     The windowBits parameter is the base two logarithm of the maximum window
   size (the size of the history buffer).  It should be in the range 8..15 for
   this version of the library. The default value is 15 if inflateInit is used
   instead. windowBits must be greater than or equal to the windowBits value
   provided to deflateInit2() while compressing, or it must be equal to 15 if
   deflateInit2() was not used. If a compressed stream with a larger window
   size is given as input, inflate() will return with the error code
   Z_DATA_ERROR instead of trying to allocate a larger window.

     windowBits can also be -8..-15 for raw inflate. In this case, -windowBits
   determines the window size. inflate() will then process raw deflate data,
   not looking for a zlib or gzip header, not generating a check value, and not
   looking for any check values for comparison at the end of the stream. This
   is for use with other formats that use the deflate compressed data format
   such as zip.  Those formats provide their own check values. If a custom
   format is developed using the raw deflate format for compressed data, it is
   recommended that a check value such as an adler32 or a crc32 be applied to
   the uncompressed data as is done in the zlib, gzip, and zip formats.  For
   most applications, the zlib format should be used as is. Note that comments
   above on the use in deflateInit2() applies to the magnitude of windowBits.

     windowBits can also be greater than 15 for optional gzip decoding. Add
   32 to windowBits to enable zlib and gzip decoding with automatic header
   detection, or add 16 to decode only the gzip format (the zlib format will
   return a Z_DATA_ERROR).  If a gzip stream is being decoded, strm->adler is
   a crc32 instead of an adler32.

     inflateInit2 returns Z_OK if success, Z_MEM_ERROR if there was not enough
   memory, Z_STREAM_ERROR if a parameter is invalid (such as a null strm). msg
   is set to null if there is no error message.  inflateInit2 does not perform
   any decompression apart from reading the zlib header if present: this will
   be done by inflate(). (So next_in and avail_in may be modified, but next_out
   and avail_out are unchanged.)
*/
/*
     Initializes the decompression dictionary from the given uncompressed byte
   sequence. This function must be called immediately after a call of inflate,
   if that call returned Z_NEED_DICT. The dictionary chosen by the compressor
   can be determined from the adler32 value returned by that call of inflate.
   The compressor and decompressor must use exactly the same dictionary (see
   deflateSetDictionary).  For raw inflate, this function can be called
   immediately after inflateInit2() or inflateReset() and before any call of
   inflate() to set the dictionary.  The application must insure that the
   dictionary that was used for compression is provided.

     inflateSetDictionary returns Z_OK if success, Z_STREAM_ERROR if a
   parameter is invalid (such as NULL dictionary) or the stream state is
   inconsistent, Z_DATA_ERROR if the given dictionary doesn't match the
   expected one (incorrect adler32 value). inflateSetDictionary does not
   perform any decompression: this will be done by subsequent calls of
   inflate().
*/
/*
    Skips invalid compressed data until a full flush point (see above the
  description of deflate with Z_FULL_FLUSH) can be found, or until all
  available input is skipped. No output is provided.

    inflateSync returns Z_OK if a full flush point has been found, Z_BUF_ERROR
  if no more input was provided, Z_DATA_ERROR if no flush point has been found,
  or Z_STREAM_ERROR if the stream structure was inconsistent. In the success
  case, the application may save the current current value of total_in which
  indicates where valid compressed data was found. In the error case, the
  application may repeatedly call inflateSync, providing more input each time,
  until success or end of the input data.
*/
#[no_mangle]

pub unsafe extern "C" fn inflateCopy(
    mut dest: crate::zlib_h::z_streamp,
    mut source: crate::zlib_h::z_streamp,
) -> libc::c_int {
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    let mut copy: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state;
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wsize: libc::c_uint = 0;
    /* check input */
    if dest.is_null()
        || source.is_null()
        || (*source).state.is_null()
        || (*source).zalloc.is_none()
        || (*source).zfree.is_none()
    {
        return -(2 as libc::c_int);
    }
    state = (*source).state as *mut crate::src::zlib::inflate::inflate_state;
    /* allocate space */
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as libc::c_int as crate::zconf_h::uInt,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as libc::c_ulong
            as crate::zconf_h::uInt,
    ) as *mut crate::src::zlib::inflate::inflate_state;
    if copy.is_null() {
        return -(4 as libc::c_int);
    }
    window = 0 as *mut libc::c_uchar;
    if !(*state).window.is_null() {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as crate::zconf_h::uInt,
        ) as *mut libc::c_uchar;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                (*source).opaque,
                copy as crate::zconf_h::voidpf,
            );
            return -(4 as libc::c_int);
        }
    }
    /* copy state */
    crate::stdlib::memcpy(
        dest as *mut libc::c_void,
        source as *const libc::c_void,
        ::std::mem::size_of::<crate::zlib_h::z_stream>() as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        copy as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as libc::c_ulong,
    );
    if (*state).lencode >= (*state).codes.as_mut_ptr() as *const crate::src::zlib::inftrees::code
        && (*state).lencode
            <= (*state)
                .codes
                .as_mut_ptr()
                .offset(2048 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
                as *const crate::src::zlib::inftrees::code
    {
        (*copy).lencode = (*copy).codes.as_mut_ptr().offset(
            (*state)
                .lencode
                .wrapping_offset_from((*state).codes.as_mut_ptr()) as libc::c_long
                as isize,
        );
        (*copy).distcode = (*copy).codes.as_mut_ptr().offset(
            (*state)
                .distcode
                .wrapping_offset_from((*state).codes.as_mut_ptr()) as libc::c_long
                as isize,
        )
    }
    (*copy).next = (*copy).codes.as_mut_ptr().offset(
        (*state)
            .next
            .wrapping_offset_from((*state).codes.as_mut_ptr()) as libc::c_long as isize,
    );
    if !window.is_null() {
        wsize = (1 as libc::c_uint) << (*state).wbits;
        crate::stdlib::memcpy(
            window as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            wsize as libc::c_ulong,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut crate::zlib_h::internal_state;
    return 0 as libc::c_int;
}

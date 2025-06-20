// =============== BEGIN inflate_h ================
pub type inflate_mode = u32;

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
    pub last: i32,
    pub wrap: i32,
    pub havedict: i32,
    pub flags: i32,
    pub dmax: u32,
    pub check: usize,
    pub total: usize,
    pub head: crate::zlib_h::gz_headerp,
    pub wbits: u32,
    pub wsize: u32,
    pub whave: u32,
    pub write: u32,
    pub window: *mut u8,
    pub hold: usize,
    pub bits: u32,
    pub length: u32,
    pub offset: u32,
    pub extra: u32,
    pub lencode: *const crate::src::zlib::inftrees::code,
    pub distcode: *const crate::src::zlib::inftrees::code,
    pub lenbits: u32,
    pub distbits: u32,
    pub ncode: u32,
    pub nlen: u32,
    pub ndist: u32,
    pub have: u32,
    pub next: *mut crate::src::zlib::inftrees::code,
    pub lens: [u16; 320],
    pub work: [u16; 288],
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

pub unsafe extern "C" fn inflateReset(mut strm: z_streamp) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut(); /* to support ill-conceived Java test suite */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    } /* in case we return an error */
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    (*state).total = 0 as i32 as usize;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = std::ptr::null_mut();
    (*strm).adler = 1 as i32 as uLong;
    (*state).mode = crate::src::zlib::inflate::HEAD;
    (*state).last = 0 as i32;
    (*state).havedict = 0 as i32;
    (*state).dmax = 32768 as u32;
    (*state).head = 0 as gz_headerp;
    (*state).wsize = 0 as i32 as u32;
    (*state).whave = 0 as i32 as u32;
    (*state).write = 0 as i32 as u32;
    (*state).hold = 0 as i32 as usize;
    (*state).bits = 0 as i32 as u32;
    (*state).next = (*state).codes.as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn inflatePrime(mut strm: z_streamp, mut bits: i32, mut value: i32) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if bits > 16 as i32 || (*state).bits.wrapping_add(bits as u32) > 32 as i32 as u32 {
        return -(2 as i32);
    }
    value = (value as isize & ((1 as isize) << bits) - 1 as i32 as isize) as i32;
    (*state).hold = (*state)
        .hold
        .wrapping_add((value << (*state).bits) as usize);
    (*state).bits = (*state).bits.wrapping_add(bits as u32);
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn inflateInit2_(
    mut strm: z_streamp,
    mut windowBits: i32,
    mut version: *const libc::c_char,
    mut stream_size: i32,
) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    if version.is_null()
        || *version.offset(0 as i32 as isize) as i32
            != (*::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"1.2.3\x00"))
                [0 as i32 as usize] as i32
        || stream_size != ::std::mem::size_of::<z_stream>() as usize as i32
    {
        return -(6 as i32);
    }
    if strm.is_null() {
        return -(2 as i32);
    }
    (*strm).msg = std::ptr::null_mut();
    if (*strm).zalloc.is_none() {
        (*strm).zalloc = Some(
            crate::src::zlib::zutil::zcalloc
                as unsafe extern "C" fn(_: voidpf, _: u32, _: u32) -> voidpf,
        );
        (*strm).opaque = 0 as voidpf
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree = Some(
            crate::src::zlib::zutil::zcfree as unsafe extern "C" fn(_: voidpf, _: voidpf) -> (),
        )
    }
    state = Some((*strm).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*strm).opaque,
        1 as i32 as uInt,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as usize as uInt,
    ) as *mut crate::src::zlib::inflate::inflate_state;
    if state.is_null() {
        return -(4 as i32);
    }
    (*strm).state = state as *mut internal_state;
    if windowBits < 0 as i32 {
        (*state).wrap = 0 as i32;
        windowBits = -windowBits
    } else {
        (*state).wrap = (windowBits >> 4 as i32) + 1 as i32
    }
    if windowBits < 8 as i32 || windowBits > 15 as i32 {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            state as voidpf,
        );
        (*strm).state = std::ptr::null_mut();
        return -(2 as i32);
    }
    (*state).wbits = windowBits as u32;
    (*state).window = std::ptr::null_mut();
    return inflateReset(strm);
}
#[no_mangle]

pub unsafe extern "C" fn inflateInit_(
    mut strm: z_streamp,
    mut version: *const libc::c_char,
    mut stream_size: i32,
) -> i32 {
    return inflateInit2_(strm, 15 as i32, version, stream_size);
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
    static mut lenfix: [code; 512] = [
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 80 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 16 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 112 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 48 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 192 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 96 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 32 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 160 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 128 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 64 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 224 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 88 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 24 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 144 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 120 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 56 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 208 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 104 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 40 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 176 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 136 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 72 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 240 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 84 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 20 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 227 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 116 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 52 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 200 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 100 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 36 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 168 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 132 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 68 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 232 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 92 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 28 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 152 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 124 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 60 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 216 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 108 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 44 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 184 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 12 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 140 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 76 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 248 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 82 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 18 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 163 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 114 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 50 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 196 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 98 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 34 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 164 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 2 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 130 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 66 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 228 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 90 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 26 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 148 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 122 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 58 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 212 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 106 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 42 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 180 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 138 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 74 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 244 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 86 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 22 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 118 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 54 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 204 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 102 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 38 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 172 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 134 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 70 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 236 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 94 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 30 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 156 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 126 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 62 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 220 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 110 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 46 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 188 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 14 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 142 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 78 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 252 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 81 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 113 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 49 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 194 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 97 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 33 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 162 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 1 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 129 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 65 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 226 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 89 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 25 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 146 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 121 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 57 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 210 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 105 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 41 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 178 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 137 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 73 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 242 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 85 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 21 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 258 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 117 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 53 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 202 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 101 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 37 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 170 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 133 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 69 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 234 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 93 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 29 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 154 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 125 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 61 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 218 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 109 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 45 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 186 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 141 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 77 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 250 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 195 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 198 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 166 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 230 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 91 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 150 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 123 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 214 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 107 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 182 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 139 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 75 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 246 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 87 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 119 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 55 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 206 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 103 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 39 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 174 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 135 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 71 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 238 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 95 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 158 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 127 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 63 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 222 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 111 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 47 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 190 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 143 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 79 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 254 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 80 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 16 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 112 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 48 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 193 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 96 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 32 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 161 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 128 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 64 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 225 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 88 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 24 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 145 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 120 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 56 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 209 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 104 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 40 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 177 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 136 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 72 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 241 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 84 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 20 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 227 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 116 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 52 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 201 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 100 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 36 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 169 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 132 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 68 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 233 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 92 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 28 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 153 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 124 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 60 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 217 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 108 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 44 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 185 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 12 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 140 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 76 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 249 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 82 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 18 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 163 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 114 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 50 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 197 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 98 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 34 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 165 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 2 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 130 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 66 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 229 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 90 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 26 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 149 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 122 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 58 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 213 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 106 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 42 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 181 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 138 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 74 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 245 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 86 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 22 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 118 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 54 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 205 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 102 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 38 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 173 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 134 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 70 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 237 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 94 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 30 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 157 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 126 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 62 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 221 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 110 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 46 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 189 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 14 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 142 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 78 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 253 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 81 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 113 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 49 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 195 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 10 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 97 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 33 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 163 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 1 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 129 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 65 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 227 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 6 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 89 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 25 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 147 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 121 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 57 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 211 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 105 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 41 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 179 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 137 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 73 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 243 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 85 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 21 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 258 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 117 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 53 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 203 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 101 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 37 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 171 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 133 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 69 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 235 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 8 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 93 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 29 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 155 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 125 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 61 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 219 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 109 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 45 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 187 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 141 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 77 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 251 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 83 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 195 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 115 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 199 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 35 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 167 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 131 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 231 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 91 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 151 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 67 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 123 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 59 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 215 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 19 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 107 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 43 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 183 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 11 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 139 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 75 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 247 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 87 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 23 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 51 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 119 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 55 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 207 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 103 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 39 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 175 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 135 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 71 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 239 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 95 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 31 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 159 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 99 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 127 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 63 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 223 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 7 as i32 as u8,
                val: 27 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 111 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 47 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 191 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 15 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 143 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 8 as i32 as u8,
                val: 79 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as i32 as u8,
                bits: 9 as i32 as u8,
                val: 255 as i32 as u16,
            };
            init
        },
    ];
    static mut distfix: [code; 32] = [
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 257 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 17 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 4097 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 5 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1025 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 65 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 16385 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 3 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 513 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 33 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 8193 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 9 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 2049 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 129 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 2 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 385 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 25 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 6145 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 7 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 1537 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 97 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 24577 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 4 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 769 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 49 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 12289 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 13 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 3073 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 193 as i32 as u16,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as i32 as u8,
                bits: 5 as i32 as u8,
                val: 0 as i32 as u16,
            };
            init
        },
    ];
    /* !BUILDFIXED */
    /* BUILDFIXED */
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as i32 as u32;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as i32 as u32;
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

unsafe extern "C" fn updatewindow(mut strm: z_streamp, mut out: u32) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    let mut copy: u32 = 0;
    let mut dist: u32 = 0;
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    /* if it hasn't been done already, allocate space for the window */
    if (*state).window.is_null() {
        (*state).window = Some((*strm).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*strm).opaque,
            (1 as u32) << (*state).wbits,
            ::std::mem::size_of::<u8>() as usize as uInt,
        ) as *mut u8;
        if (*state).window.is_null() {
            return 1 as i32;
        }
    }
    /* if window not in use yet, initialize */
    if (*state).wsize == 0 as i32 as u32 {
        (*state).wsize = (1 as u32) << (*state).wbits;
        (*state).write = 0 as i32 as u32;
        (*state).whave = 0 as i32 as u32
    }
    /* copy state->wsize or less output bytes into the circular window */
    copy = out.wrapping_sub((*strm).avail_out);
    if copy >= (*state).wsize {
        crate::stdlib::memcpy(
            (*state).window as *mut libc::c_void,
            (*strm).next_out.offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as usize,
        );
        (*state).write = 0 as i32 as u32;
        (*state).whave = (*state).wsize
    } else {
        dist = (*state).wsize.wrapping_sub((*state).write);
        if dist > copy {
            dist = copy
        }
        crate::stdlib::memcpy(
            (*state).window.offset((*state).write as isize) as *mut libc::c_void,
            (*strm).next_out.offset(-(copy as isize)) as *const libc::c_void,
            dist as usize,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            crate::stdlib::memcpy(
                (*state).window as *mut libc::c_void,
                (*strm).next_out.offset(-(copy as isize)) as *const libc::c_void,
                copy as usize,
            );
            (*state).write = copy;
            (*state).whave = (*state).wsize
        } else {
            (*state).write = (*state).write.wrapping_add(dist);
            if (*state).write == (*state).wsize {
                (*state).write = 0 as i32 as u32
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = (*state).whave.wrapping_add(dist)
            }
        }
    }
    return 0 as i32;
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

pub unsafe extern "C" fn inflate(mut strm: z_streamp, mut flush: i32) -> i32 {
    let mut current_block: u64; /* next input */
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut(); /* next output */
    let mut next: *mut u8 = std::ptr::null_mut(); /* available input and output */
    let mut put: *mut u8 = std::ptr::null_mut(); /* bit buffer */
    let mut have: u32 = 0; /* bits in bit buffer */
    let mut left: u32 = 0; /* save starting available input and output */
    let mut hold: usize = 0; /* number of stored or match bytes to copy */
    let mut bits: u32 = 0; /* where to copy match bytes from */
    let mut in_0: u32 = 0; /* current decoding table entry */
    let mut out: u32 = 0; /* parent table entry */
    let mut copy: u32 = 0; /* length to copy for repeats, bits to drop */
    let mut from: *mut u8 = std::ptr::null_mut(); /* return code */
    let mut this: code = code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* skip check */
    let mut last: code = code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* go to byte boundary */
    let mut len: u32 = 0;
    let mut ret: i32 = 0;
    static mut order: [u16; 19] = [
        16 as i32 as u16,
        17 as i32 as u16,
        18 as i32 as u16,
        0 as i32 as u16,
        8 as i32 as u16,
        7 as i32 as u16,
        9 as i32 as u16,
        6 as i32 as u16,
        10 as i32 as u16,
        5 as i32 as u16,
        11 as i32 as u16,
        4 as i32 as u16,
        12 as i32 as u16,
        3 as i32 as u16,
        13 as i32 as u16,
        2 as i32 as u16,
        14 as i32 as u16,
        1 as i32 as u16,
        15 as i32 as u16,
    ];
    if strm.is_null()
        || (*strm).state.is_null()
        || (*strm).next_out.is_null()
        || (*strm).next_in.is_null() && (*strm).avail_in != 0 as i32 as u32
    {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).mode as u32 == crate::src::zlib::inflate::TYPE as i32 as u32 {
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
    ret = 0 as i32;
    's_114: loop {
        match (*state).mode as u32 {
            0 => {
                if (*state).wrap == 0 as i32 {
                    (*state).mode = crate::src::zlib::inflate::TYPEDO;
                    continue;
                } else {
                    while bits < 16 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    if (((hold as u32 & ((1 as u32) << 8 as i32).wrapping_sub(1 as i32 as u32))
                        << 8 as i32) as usize)
                        .wrapping_add(hold >> 8 as i32)
                        .wrapping_rem(31 as i32 as usize)
                        != 0
                    {
                        (*strm).msg = b"incorrect header check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else if hold as u32 & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32)
                        != 8 as i32 as u32
                    {
                        (*strm).msg = b"unknown compression method\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        hold >>= 4 as i32;
                        bits = bits.wrapping_sub(4 as i32 as u32);
                        len = (hold as u32
                            & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32))
                        .wrapping_add(8 as i32 as u32);
                        if len > (*state).wbits {
                            (*strm).msg = b"invalid window size\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            continue;
                        } else {
                            (*state).dmax = (1 as u32) << len;
                            (*state).check =
                                adler32(0 as isize as uLong, std::ptr::null(), 0 as i32 as uInt);
                            (*strm).adler = (*state).check;
                            (*state).mode = if hold & 0x200 as i32 as usize != 0 {
                                crate::src::zlib::inflate::DICTID as i32
                            } else {
                                crate::src::zlib::inflate::TYPE as i32
                            }
                                as crate::src::zlib::inflate::inflate_mode;
                            hold = 0 as i32 as usize;
                            bits = 0 as i32 as u32;
                            continue;
                        }
                    }
                }
            }
            9 => {
                while bits < 32 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as usize) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32)
                }
                (*state).check = (hold >> 24 as i32 & 0xff as i32 as usize)
                    .wrapping_add(hold >> 8 as i32 & 0xff00 as i32 as usize)
                    .wrapping_add((hold & 0xff00 as i32 as usize) << 8 as i32)
                    .wrapping_add((hold & 0xff as i32 as usize) << 24 as i32);
                (*strm).adler = (*state).check;
                hold = 0 as i32 as usize;
                bits = 0 as i32 as u32;
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
                hold >>= bits & 7 as i32 as u32;
                bits = bits.wrapping_sub(bits & 7 as i32 as u32);
                while bits < 32 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as usize) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32)
                }
                if hold & 0xffff as i32 as usize != hold >> 16 as i32 ^ 0xffff as i32 as usize {
                    (*strm).msg = b"invalid stored block lengths\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).length = hold as u32 & 0xffff as i32 as u32;
                    hold = 0 as i32 as usize;
                    bits = 0 as i32 as u32;
                    (*state).mode = crate::src::zlib::inflate::COPY
                }
                current_block = 18432964712698998993;
            }
            14 => {
                current_block = 18432964712698998993;
            }
            15 => {
                while bits < 14 as i32 as u32 {
                    if have == 0 as i32 as u32 {
                        break 's_114;
                    }
                    have = have.wrapping_sub(1);
                    let fresh4 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh4 as usize) << bits);
                    bits = bits.wrapping_add(8 as i32 as u32)
                }
                (*state).nlen = (hold as u32
                    & ((1 as u32) << 5 as i32).wrapping_sub(1 as i32 as u32))
                .wrapping_add(257 as i32 as u32);
                hold >>= 5 as i32;
                bits = bits.wrapping_sub(5 as i32 as u32);
                (*state).ndist = (hold as u32
                    & ((1 as u32) << 5 as i32).wrapping_sub(1 as i32 as u32))
                .wrapping_add(1 as i32 as u32);
                hold >>= 5 as i32;
                bits = bits.wrapping_sub(5 as i32 as u32);
                (*state).ncode = (hold as u32
                    & ((1 as u32) << 4 as i32).wrapping_sub(1 as i32 as u32))
                .wrapping_add(4 as i32 as u32);
                hold >>= 4 as i32;
                bits = bits.wrapping_sub(4 as i32 as u32);
                if (*state).nlen > 286 as i32 as u32 || (*state).ndist > 30 as i32 as u32 {
                    (*strm).msg = b"too many length or distance symbols\x00" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                    (*state).mode = crate::src::zlib::inflate::BAD;
                    continue;
                } else {
                    (*state).have = 0 as i32 as u32;
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
                if left == 0 as i32 as u32 {
                    break;
                }
                let fresh24 = put;
                put = put.offset(1);
                *fresh24 = (*state).length as u8;
                left = left.wrapping_sub(1);
                (*state).mode = crate::src::zlib::inflate::LEN;
                continue;
            }
            24 => {
                if (*state).wrap != 0 {
                    while bits < 32 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh25 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh25 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    out = out.wrapping_sub(left);
                    (*strm).total_out =
                        ((*strm).total_out as usize).wrapping_add(out as usize) as uLong;
                    (*state).total = (*state).total.wrapping_add(out as usize);
                    if out != 0 {
                        (*state).check = adler32((*state).check, put.offset(-(out as isize)), out);
                        (*strm).adler = (*state).check
                    }
                    out = left;
                    if (hold >> 24 as i32 & 0xff as i32 as usize)
                        .wrapping_add(hold >> 8 as i32 & 0xff00 as i32 as usize)
                        .wrapping_add((hold & 0xff00 as i32 as usize) << 8 as i32)
                        .wrapping_add((hold & 0xff as i32 as usize) << 24 as i32)
                        != (*state).check
                    {
                        (*strm).msg = b"incorrect data check\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        hold = 0 as i32 as usize;
                        bits = 0 as i32 as u32
                    }
                }
                (*state).mode = crate::src::zlib::inflate::DONE;
                current_block = 1550432445778694857;
            }
            26 => {
                current_block = 1550432445778694857;
            }
            27 => {
                ret = -(3 as i32);
                break;
            }
            28 => return -(4 as i32),
            29 | _ => return -(2 as i32),
        }
        match current_block {
            11322929247169729670 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh5 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh5 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    let fresh6 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh6 as usize] as usize] = (hold as u32
                        & ((1 as u32) << 3 as i32).wrapping_sub(1 as i32 as u32))
                        as u16;
                    hold >>= 3 as i32;
                    bits = bits.wrapping_sub(3 as i32 as u32)
                }
                while (*state).have < 19 as i32 as u32 {
                    let fresh7 = (*state).have;
                    (*state).have = (*state).have.wrapping_add(1);
                    (*state).lens[order[fresh7 as usize] as usize] = 0 as i32 as u16
                }
                (*state).next = (*state).codes.as_mut_ptr();
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 7 as i32 as u32;
                ret = inflate_table(
                    CODES,
                    (*state).lens.as_mut_ptr(),
                    19 as i32 as u32,
                    &mut (*state).next as *mut _ as *mut *mut code,
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
                    (*state).have = 0 as i32 as u32;
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
                    if copy == 0 as i32 as u32 {
                        break;
                    }
                    crate::stdlib::memcpy(
                        put as *mut libc::c_void,
                        next as *const libc::c_void,
                        copy as usize,
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
                if (*state).havedict == 0 as i32 {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return 2 as i32;
                }
                (*state).check = adler32(0 as isize as uLong, std::ptr::null(), 0 as i32 as uInt);
                (*strm).adler = (*state).check;
                (*state).mode = crate::src::zlib::inflate::TYPE;
                current_block = 10674880093440332853;
            }
            1550432445778694857 => {
                ret = 1 as i32;
                break;
            }
            _ => {}
        }
        match current_block {
            6177865312519592116 => {
                while (*state).have < (*state).nlen.wrapping_add((*state).ndist) {
                    loop {
                        this = *(*state).lencode.offset(
                            (hold as u32
                                & ((1 as u32) << (*state).lenbits).wrapping_sub(1 as i32 as u32))
                                as isize,
                        );
                        if this.bits as u32 <= bits {
                            break;
                        }
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh8 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh8 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    if (this.val as i32) < 16 as i32 {
                        while bits < this.bits as u32 {
                            if have == 0 as i32 as u32 {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh9 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh9 as usize) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32)
                        }
                        hold >>= this.bits as i32;
                        bits = bits.wrapping_sub(this.bits as u32);
                        let fresh10 = (*state).have;
                        (*state).have = (*state).have.wrapping_add(1);
                        (*state).lens[fresh10 as usize] = this.val
                    } else {
                        if this.val as i32 == 16 as i32 {
                            while bits < (this.bits as i32 + 2 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh11 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh11 as usize) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32)
                            }
                            hold >>= this.bits as i32;
                            bits = bits.wrapping_sub(this.bits as u32);
                            if (*state).have == 0 as i32 as u32 {
                                (*strm).msg = b"invalid bit length repeat\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                                (*state).mode = crate::src::zlib::inflate::BAD;
                                break;
                            } else {
                                len = (*state).lens
                                    [(*state).have.wrapping_sub(1 as i32 as u32) as usize]
                                    as u32;
                                copy = (3 as i32 as u32).wrapping_add(
                                    hold as u32
                                        & ((1 as u32) << 2 as i32).wrapping_sub(1 as i32 as u32),
                                );
                                hold >>= 2 as i32;
                                bits = bits.wrapping_sub(2 as i32 as u32)
                            }
                        } else if this.val as i32 == 17 as i32 {
                            while bits < (this.bits as i32 + 3 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh12 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh12 as usize) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32)
                            }
                            hold >>= this.bits as i32;
                            bits = bits.wrapping_sub(this.bits as u32);
                            len = 0 as i32 as u32;
                            copy = (3 as i32 as u32).wrapping_add(
                                hold as u32
                                    & ((1 as u32) << 3 as i32).wrapping_sub(1 as i32 as u32),
                            );
                            hold >>= 3 as i32;
                            bits = bits.wrapping_sub(3 as i32 as u32)
                        } else {
                            while bits < (this.bits as i32 + 7 as i32) as u32 {
                                if have == 0 as i32 as u32 {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh13 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh13 as usize) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32)
                            }
                            hold >>= this.bits as i32;
                            bits = bits.wrapping_sub(this.bits as u32);
                            len = 0 as i32 as u32;
                            copy = (11 as i32 as u32).wrapping_add(
                                hold as u32
                                    & ((1 as u32) << 7 as i32).wrapping_sub(1 as i32 as u32),
                            );
                            hold >>= 7 as i32;
                            bits = bits.wrapping_sub(7 as i32 as u32)
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
                                (*state).lens[fresh15 as usize] = len as u16
                            }
                        }
                    }
                }
                /* handle error breaks in while */
                if (*state).mode as u32 == crate::src::zlib::inflate::BAD as i32 as u32 {
                    continue;
                }
                /* build code tables */
                (*state).next = (*state).codes.as_mut_ptr();
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 9 as i32 as u32;
                ret = inflate_table(
                    LENS,
                    (*state).lens.as_mut_ptr(),
                    (*state).nlen,
                    &mut (*state).next as *mut _ as *mut *mut code,
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
                    (*state).distcode = (*state).next as *const code;
                    (*state).distbits = 6 as i32 as u32;
                    ret = inflate_table(
                        DISTS,
                        (*state).lens.as_mut_ptr().offset((*state).nlen as isize),
                        (*state).ndist,
                        &mut (*state).next as *mut _ as *mut *mut code,
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
                if flush == 5 as i32 {
                    break;
                }
                current_block = 14847832218395804385;
            }
            _ => {}
        }
        match current_block {
            11341304196878840394 => {
                if have >= 6 as i32 as u32 && left >= 258 as i32 as u32 {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    crate::src::zlib::inffast::inflate_fast(strm as *mut z_stream_s, out);
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
                            (hold as u32
                                & ((1 as u32) << (*state).lenbits).wrapping_sub(1 as i32 as u32))
                                as isize,
                        );
                        if this.bits as u32 <= bits {
                            break;
                        }
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh16 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh16 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    if this.op as i32 != 0 && this.op as i32 & 0xf0 as i32 == 0 as i32 {
                        last = this;
                        loop {
                            this = *(*state).lencode.offset(
                                (last.val as u32).wrapping_add(
                                    (hold as u32
                                        & ((1 as u32) << last.bits as i32 + last.op as i32)
                                            .wrapping_sub(1 as i32 as u32))
                                        >> last.bits as i32,
                                ) as isize,
                            );
                            if (last.bits as i32 + this.bits as i32) as u32 <= bits {
                                break;
                            }
                            if have == 0 as i32 as u32 {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh17 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh17 as usize) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32)
                        }
                        hold >>= last.bits as i32;
                        bits = bits.wrapping_sub(last.bits as u32)
                    }
                    hold >>= this.bits as i32;
                    bits = bits.wrapping_sub(this.bits as u32);
                    (*state).length = this.val as u32;
                    if this.op as i32 == 0 as i32 {
                        (*state).mode = crate::src::zlib::inflate::LIT;
                        continue;
                    } else if this.op as i32 & 32 as i32 != 0 {
                        (*state).mode = crate::src::zlib::inflate::TYPE;
                        continue;
                    } else if this.op as i32 & 64 as i32 != 0 {
                        (*strm).msg = b"invalid literal/length code\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        continue;
                    } else {
                        (*state).extra = this.op as u32 & 15 as i32 as u32;
                        (*state).mode = crate::src::zlib::inflate::LENEXT
                    }
                }
                current_block = 10540587995463041380;
            }
            14847832218395804385 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as i32 as u32;
                    bits = bits.wrapping_sub(bits & 7 as i32 as u32);
                    (*state).mode = crate::src::zlib::inflate::CHECK;
                    continue;
                } else {
                    while bits < 3 as i32 as u32 {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh2 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh2 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    (*state).last = (hold as u32
                        & ((1 as u32) << 1 as i32).wrapping_sub(1 as i32 as u32))
                        as i32;
                    hold >>= 1 as i32;
                    bits = bits.wrapping_sub(1 as i32 as u32);
                    match hold as u32 & ((1 as u32) << 2 as i32).wrapping_sub(1 as i32 as u32) {
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
                    hold >>= 2 as i32;
                    bits = bits.wrapping_sub(2 as i32 as u32);
                    continue;
                }
            }
            _ => {}
        }
        match current_block {
            10540587995463041380 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh18 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh18 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32)
                    }
                    (*state).length = (*state).length.wrapping_add(
                        hold as u32 & ((1 as u32) << (*state).extra).wrapping_sub(1 as i32 as u32),
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
                        (hold as u32
                            & ((1 as u32) << (*state).distbits).wrapping_sub(1 as i32 as u32))
                            as isize,
                    );
                    if this.bits as u32 <= bits {
                        if this.op as i32 & 0xf0 as i32 == 0 as i32 {
                            last = this;
                            loop {
                                this = *(*state).distcode.offset(
                                    (last.val as u32).wrapping_add(
                                        (hold as u32
                                            & ((1 as u32) << last.bits as i32 + last.op as i32)
                                                .wrapping_sub(1 as i32 as u32))
                                            >> last.bits as i32,
                                    ) as isize,
                                );
                                if (last.bits as i32 + this.bits as i32) as u32 <= bits {
                                    break;
                                }
                                if have == 0 as i32 as u32 {
                                    break 's_114;
                                }
                                have = have.wrapping_sub(1);
                                let fresh20 = next;
                                next = next.offset(1);
                                hold = hold.wrapping_add((*fresh20 as usize) << bits);
                                bits = bits.wrapping_add(8 as i32 as u32)
                            }
                            hold >>= last.bits as i32;
                            bits = bits.wrapping_sub(last.bits as u32)
                        }
                        hold >>= this.bits as i32;
                        bits = bits.wrapping_sub(this.bits as u32);
                        if this.op as i32 & 64 as i32 != 0 {
                            (*strm).msg = b"invalid distance code\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            break;
                        } else {
                            (*state).offset = this.val as u32;
                            (*state).extra = this.op as u32 & 15 as i32 as u32;
                            (*state).mode = crate::src::zlib::inflate::DISTEXT;
                            current_block = 6144666487834620188;
                        }
                    } else {
                        if have == 0 as i32 as u32 {
                            break 's_114;
                        }
                        have = have.wrapping_sub(1);
                        let fresh19 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh19 as usize) << bits);
                        bits = bits.wrapping_add(8 as i32 as u32);
                        current_block = 583050819838508811;
                    }
                }
                6144666487834620188 => {
                    if (*state).extra != 0 {
                        while bits < (*state).extra {
                            if have == 0 as i32 as u32 {
                                break 's_114;
                            }
                            have = have.wrapping_sub(1);
                            let fresh21 = next;
                            next = next.offset(1);
                            hold = hold.wrapping_add((*fresh21 as usize) << bits);
                            bits = bits.wrapping_add(8 as i32 as u32)
                        }
                        (*state).offset = (*state).offset.wrapping_add(
                            hold as u32
                                & ((1 as u32) << (*state).extra).wrapping_sub(1 as i32 as u32),
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
                    if left == 0 as i32 as u32 {
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
                    if (*state).length == 0 as i32 as u32 {
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
        || ((*state).mode as u32) < crate::src::zlib::inflate::CHECK as i32 as u32
            && out != (*strm).avail_out
    {
        if updatewindow(strm, out) != 0 {
            (*state).mode = crate::src::zlib::inflate::MEM;
            return -(4 as i32);
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm).total_in = ((*strm).total_in as usize).wrapping_add(in_0 as usize) as uLong;
    (*strm).total_out = ((*strm).total_out as usize).wrapping_add(out as usize) as uLong;
    (*state).total = (*state).total.wrapping_add(out as usize);
    if (*state).wrap != 0 && out != 0 {
        (*state).check = adler32(
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
                64 as i32
            } else {
                0 as i32
            }) as u32,
        )
        .wrapping_add(
            (if (*state).mode as u32 == crate::src::zlib::inflate::TYPE as i32 as u32 {
                128 as i32
            } else {
                0 as i32
            }) as u32,
        ) as i32;
    if (in_0 == 0 as i32 as u32 && out == 0 as i32 as u32 || flush == 4 as i32) && ret == 0 as i32 {
        ret = -(5 as i32)
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn inflateEnd(mut strm: z_streamp) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if !(*state).window.is_null() {
        Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
            (*strm).opaque,
            (*state).window as voidpf,
        );
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")(
        (*strm).opaque,
        (*strm).state as voidpf,
    );
    (*strm).state = std::ptr::null_mut();
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn inflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    let mut id: usize = 0;
    /* check state */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).wrap != 0 as i32
        && (*state).mode as u32 != crate::src::zlib::inflate::DICT as i32 as u32
    {
        return -(2 as i32);
    }
    /* check for correct dictionary id */
    if (*state).mode as u32 == crate::src::zlib::inflate::DICT as i32 as u32 {
        id = adler32(0 as isize as uLong, std::ptr::null(), 0 as i32 as uInt);
        id = adler32(id, dictionary, dictLength);
        if id != (*state).check {
            return -(3 as i32);
        }
    }
    /* copy dictionary to window */
    if updatewindow(strm, (*strm).avail_out) != 0 {
        (*state).mode = crate::src::zlib::inflate::MEM;
        return -(4 as i32);
    }
    if dictLength > (*state).wsize {
        crate::stdlib::memcpy(
            (*state).window as *mut libc::c_void,
            dictionary
                .offset(dictLength as isize)
                .offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as usize,
        );
        (*state).whave = (*state).wsize
    } else {
        crate::stdlib::memcpy(
            (*state)
                .window
                .offset((*state).wsize as isize)
                .offset(-(dictLength as isize)) as *mut libc::c_void,
            dictionary as *const libc::c_void,
            dictLength as usize,
        );
        (*state).whave = dictLength
    }
    (*state).havedict = 1 as i32;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn inflateGetHeader(mut strm: z_streamp, mut head: gz_headerp) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    /* check state */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*state).wrap & 2 as i32 == 0 as i32 {
        return -(2 as i32);
    }
    /* save header structure */
    (*state).head = head;
    (*head).done = 0 as i32;
    return 0 as i32;
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

unsafe extern "C" fn syncsearch(mut have: *mut u32, mut buf: *mut u8, mut len: u32) -> u32 {
    let mut got: u32 = 0; /* number of bytes to look at or looked at */
    let mut next: u32 = 0; /* temporary to save total_in and total_out */
    got = *have; /* to restore bit buffer to byte string */
    next = 0 as i32 as u32;
    while next < len && got < 4 as i32 as u32 {
        if *buf.offset(next as isize) as i32
            == (if got < 2 as i32 as u32 {
                0 as i32
            } else {
                0xff as i32
            })
        {
            got = got.wrapping_add(1)
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as i32 as u32
        } else {
            got = (4 as i32 as u32).wrapping_sub(got)
        }
        next = next.wrapping_add(1)
    }
    *have = got;
    return next;
}
#[no_mangle]

pub unsafe extern "C" fn inflateSync(mut strm: z_streamp) -> i32 {
    let mut len: u32 = 0;
    let mut in_0: usize = 0;
    let mut out: usize = 0;
    let mut buf: [u8; 4] = [0; 4];
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    /* check parameters */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    if (*strm).avail_in == 0 as i32 as u32 && (*state).bits < 8 as i32 as u32 {
        return -(5 as i32);
    }
    /* if first time, start search in bit buffer */
    if (*state).mode as u32 != crate::src::zlib::inflate::SYNC as i32 as u32 {
        (*state).mode = crate::src::zlib::inflate::SYNC;
        (*state).hold <<= (*state).bits & 7 as i32 as u32;
        (*state).bits = (*state).bits.wrapping_sub((*state).bits & 7 as i32 as u32);
        len = 0 as i32 as u32;
        while (*state).bits >= 8 as i32 as u32 {
            let fresh26 = len;
            len = len.wrapping_add(1);
            buf[fresh26 as usize] = (*state).hold as u8;
            (*state).hold >>= 8 as i32;
            (*state).bits = (*state).bits.wrapping_sub(8 as i32 as u32)
        }
        (*state).have = 0 as i32 as u32;
        syncsearch(&mut (*state).have, buf.as_mut_ptr(), len);
    }
    /* search available input */
    len = syncsearch(&mut (*state).have, (*strm).next_in, (*strm).avail_in);
    (*strm).avail_in = ((*strm).avail_in as u32).wrapping_sub(len) as uInt;
    (*strm).next_in = (*strm).next_in.offset(len as isize);
    (*strm).total_in = ((*strm).total_in as usize).wrapping_add(len as usize) as uLong;
    /* return no joy or set up to restart inflate() on a new block */
    if (*state).have != 4 as i32 as u32 {
        return -(3 as i32);
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = crate::src::zlib::inflate::TYPE;
    return 0 as i32;
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

pub unsafe extern "C" fn inflateSyncPoint(mut strm: z_streamp) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as i32);
    }
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    return ((*state).mode as u32 == crate::src::zlib::inflate::STORED as i32 as u32
        && (*state).bits == 0 as i32 as u32) as i32;
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

pub unsafe extern "C" fn inflateCopy(mut dest: z_streamp, mut source: z_streamp) -> i32 {
    let mut state: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    let mut copy: *mut crate::src::zlib::inflate::inflate_state = std::ptr::null_mut();
    let mut window: *mut u8 = std::ptr::null_mut();
    let mut wsize: u32 = 0;
    /* check input */
    if dest.is_null()
        || source.is_null()
        || (*source).state.is_null()
        || (*source).zalloc.is_none()
        || (*source).zfree.is_none()
    {
        return -(2 as i32);
    }
    state = (*source).state as *mut crate::src::zlib::inflate::inflate_state;
    /* allocate space */
    copy = Some((*source).zalloc.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        (*source).opaque,
        1 as i32 as uInt,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as usize as uInt,
    ) as *mut crate::src::zlib::inflate::inflate_state;
    if copy.is_null() {
        return -(4 as i32);
    }
    window = std::ptr::null_mut();
    if !(*state).window.is_null() {
        window = Some((*source).zalloc.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*source).opaque,
            (1 as u32) << (*state).wbits,
            ::std::mem::size_of::<u8>() as usize as uInt,
        ) as *mut u8;
        if window.is_null() {
            Some((*source).zfree.expect("non-null function pointer"))
                .expect("non-null function pointer")((*source).opaque, copy as voidpf);
            return -(4 as i32);
        }
    }
    /* copy state */
    crate::stdlib::memcpy(
        dest as *mut libc::c_void,
        source as *const libc::c_void,
        ::std::mem::size_of::<z_stream>() as usize,
    );
    crate::stdlib::memcpy(
        copy as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<crate::src::zlib::inflate::inflate_state>() as usize,
    );
    if (*state).lencode >= (*state).codes.as_mut_ptr() as *const code
        && (*state).lencode
            <= (*state)
                .codes
                .as_mut_ptr()
                .offset(2048 as i32 as isize)
                .offset(-(1 as i32 as isize)) as *const code
    {
        (*copy).lencode = (*copy)
            .codes
            .as_mut_ptr()
            .offset((*state).lencode.offset_from((*state).codes.as_mut_ptr()) as isize as isize);
        (*copy).distcode = (*copy)
            .codes
            .as_mut_ptr()
            .offset((*state).distcode.offset_from((*state).codes.as_mut_ptr()) as isize as isize)
    }
    (*copy).next = (*copy)
        .codes
        .as_mut_ptr()
        .offset((*state).next.offset_from((*state).codes.as_mut_ptr()) as isize as isize);
    if !window.is_null() {
        wsize = (1 as u32) << (*state).wbits;
        crate::stdlib::memcpy(
            window as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            wsize as usize,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return 0 as i32;
}

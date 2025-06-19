use ::libc;

pub use crate::src::zlib::inflate::inflate_mode;
pub use crate::src::zlib::inflate::inflate_state;
pub use crate::src::zlib::inflate::BAD;
pub use crate::src::zlib::inflate::CHECK;
pub use crate::src::zlib::inflate::CODELENS;
pub use crate::src::zlib::inflate::COMMENT;
pub use crate::src::zlib::inflate::COPY;
pub use crate::src::zlib::inflate::DICT;
pub use crate::src::zlib::inflate::DICTID;
pub use crate::src::zlib::inflate::DIST;
pub use crate::src::zlib::inflate::DISTEXT;
pub use crate::src::zlib::inflate::DONE;
pub use crate::src::zlib::inflate::EXLEN;
pub use crate::src::zlib::inflate::EXTRA;
pub use crate::src::zlib::inflate::FLAGS;
pub use crate::src::zlib::inflate::HCRC;
pub use crate::src::zlib::inflate::HEAD;
pub use crate::src::zlib::inflate::LEN;
pub use crate::src::zlib::inflate::LENEXT;
pub use crate::src::zlib::inflate::LENGTH;
pub use crate::src::zlib::inflate::LENLENS;
pub use crate::src::zlib::inflate::LIT;
pub use crate::src::zlib::inflate::MATCH;
pub use crate::src::zlib::inflate::MEM;
pub use crate::src::zlib::inflate::NAME;
pub use crate::src::zlib::inflate::OS;
pub use crate::src::zlib::inflate::STORED;
pub use crate::src::zlib::inflate::SYNC;
pub use crate::src::zlib::inflate::TABLE;
pub use crate::src::zlib::inflate::TIME;
pub use crate::src::zlib::inflate::TYPE;
pub use crate::src::zlib::inflate::TYPEDO;
pub use crate::src::zlib::inftrees::code;
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
/* inffast.h -- header to use inffast.c
 * Copyright (C) 1995-2003 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* WARNING: this file should *not* be used by applications. It is
  part of the implementation of the compression library and is
  subject to change. Applications should only use zlib.h.
*/
/*
  Decode literal, length, and distance codes and write out the resulting
  literal and match bytes until either not enough input or output is
  available, an end-of-block is encountered, or a data error is encountered.
  When large enough input and output buffers are supplied to inflate(), for
  example, a 16K input buffer and a 64K output buffer, more than 95% of the
  inflate execution time is spent in this routine.

  Entry assumptions:

       state->mode == LEN
       strm->avail_in >= 6
       strm->avail_out >= 258
       start >= strm->avail_out
       state->bits < 8

  On return, state->mode is one of:

       LEN -- ran out of enough output space or enough available input
       TYPE -- reached end of block code, inflate() to interpret next block
       BAD -- error in block data

  Notes:

   - The maximum input bits used by a length/distance pair is 15 bits for the
     length code, 5 bits for the length extra, 15 bits for the distance code,
     and 13 bits for the distance extra.  This totals 48 bits, or six bytes.
     Therefore if strm->avail_in >= 6, then there is enough input to avoid
     checking for available input while decoding.

   - The maximum bytes that a single length/distance pair can output is 258
     bytes, which is the maximum length that can be coded.  inflate_fast()
     requires strm->avail_out >= 258 for each loop to avoid checking for
     output space.
*/
#[no_mangle]

pub unsafe extern "C" fn inflate_fast(mut strm: crate::zlib_h::z_streamp, mut start: libc::c_uint)
/* inflate()'s starting value for strm->avail_out */
{
    let mut state: *mut crate::src::zlib::inflate::inflate_state =
        0 as *mut crate::src::zlib::inflate::inflate_state; /* local strm->next_in */
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* while in < last, enough input available */
    let mut last: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* local strm->next_out */
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* inflate()'s initial strm->next_out */
    let mut beg: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* while out < end, enough space available */
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* window size or zero if not using window */
    let mut wsize: libc::c_uint = 0; /* valid bytes in the window */
    let mut whave: libc::c_uint = 0; /* window write index */
    let mut write: libc::c_uint = 0; /* allocated sliding window, if wsize != 0 */
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* local strm->hold */
    let mut hold: libc::c_ulong = 0; /* local strm->bits */
    let mut bits: libc::c_uint = 0; /* local strm->lencode */
    let mut lcode: *const crate::src::zlib::inftrees::code =
        0 as *const crate::src::zlib::inftrees::code; /* local strm->distcode */
    let mut dcode: *const crate::src::zlib::inftrees::code =
        0 as *const crate::src::zlib::inftrees::code; /* mask for first level of length codes */
    let mut lmask: libc::c_uint = 0; /* mask for first level of distance codes */
    let mut dmask: libc::c_uint = 0; /* retrieved table entry */
    let mut this: crate::src::zlib::inftrees::code = crate::src::zlib::inftrees::code {
        op: 0,
        bits: 0,
        val: 0,
    }; /* code bits, operation, extra bits, or */
    /*  window position, window bytes to copy */
    let mut op: libc::c_uint = 0; /* match length, unused bytes */
    let mut len: libc::c_uint = 0; /* match distance */
    let mut dist: libc::c_uint = 0; /* where to copy match from */
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* copy state to local variables */
    state = (*strm).state as *mut crate::src::zlib::inflate::inflate_state;
    in_0 = (*strm).next_in.offset(-(1 as libc::c_int as isize));
    last = in_0.offset(
        (*strm)
            .avail_in
            .wrapping_sub(5 as libc::c_int as libc::c_uint) as isize,
    );
    out = (*strm).next_out.offset(-(1 as libc::c_int as isize));
    beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
    end = out.offset(
        (*strm)
            .avail_out
            .wrapping_sub(257 as libc::c_int as libc::c_uint) as isize,
    );
    wsize = (*state).wsize;
    whave = (*state).whave;
    write = (*state).write;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask =
        ((1 as libc::c_uint) << (*state).lenbits).wrapping_sub(1 as libc::c_int as libc::c_uint);
    dmask =
        ((1 as libc::c_uint) << (*state).distbits).wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut current_block_141: u64;
    's_132: loop
    /* decode literals and length/distances until end-of-block or not enough
    input data or output space */
    {
        if bits < 15 as libc::c_int as libc::c_uint {
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
        }
        this = *lcode.offset((hold & lmask as libc::c_ulong) as isize);
        loop {
            op = this.bits as libc::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = this.op as libc::c_uint;
            if op == 0 as libc::c_int as libc::c_uint {
                /* literal */
                out = out.offset(1);
                *out = this.val as libc::c_uchar;
                current_block_141 = 18386322304582297246;
                break;
            } else if op & 16 as libc::c_int as libc::c_uint != 0 {
                /* length base */
                len = this.val as libc::c_uint; /* number of extra bits */
                op &= 15 as libc::c_int as libc::c_uint;
                if op != 0 {
                    if bits < op {
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                    }
                    len = len.wrapping_add(
                        hold as libc::c_uint
                            & ((1 as libc::c_uint) << op)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    hold >>= op;
                    bits = bits.wrapping_sub(op)
                }
                if bits < 15 as libc::c_int as libc::c_uint {
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                }
                this = *dcode.offset((hold & dmask as libc::c_ulong) as isize);
                current_block_141 = 719419377338824450;
                break;
            } else if op & 64 as libc::c_int as libc::c_uint == 0 as libc::c_int as libc::c_uint {
                /* 2nd level length code */
                this = *lcode.offset((this.val as libc::c_ulong).wrapping_add(
                    hold
                        & ((1 as libc::c_uint) << op).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                ) as isize)
            } else if op & 32 as libc::c_int as libc::c_uint != 0 {
                current_block_141 = 5250576585193495047;
                break;
            } else {
                current_block_141 = 4976922244085895320;
                break;
            }
        }
        match current_block_141 {
            719419377338824450 => {
                loop {
                    op = this.bits as libc::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = this.op as libc::c_uint;
                    if op & 16 as libc::c_int as libc::c_uint != 0 {
                        /* distance base */
                        dist = this.val as libc::c_uint; /* number of extra bits */
                        op &= 15 as libc::c_int as libc::c_uint; /* max distance in output */
                        if bits < op {
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            if bits < op {
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint)
                            }
                        }
                        dist = dist.wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << op)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as libc::c_long as libc::c_uint;
                        if dist > op {
                            current_block_141 = 5873035170358615968;
                            break;
                        } else {
                            current_block_141 = 17239133558811367971;
                            break;
                        }
                    } else if op & 64 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        /* 2nd level distance code */
                        this = *dcode.offset(
                            (this.val as libc::c_ulong).wrapping_add(
                                hold & ((1 as libc::c_uint) << op)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong,
                            ) as isize,
                        )
                    } else {
                        (*strm).msg = b"invalid distance code\x00" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char; /* copy direct from output */
                        (*state).mode = crate::src::zlib::inflate::BAD;
                        break 's_132;
                    }
                }
                match current_block_141 {
                    17239133558811367971 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            /* minimum length is three */
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            len = len.wrapping_sub(3 as libc::c_int as libc::c_uint);
                            if !(len > 2 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                        if len != 0 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            if len > 1 as libc::c_int as libc::c_uint {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from
                            }
                        }
                    }
                    _ => {
                        /* see if copy from window */
                        op = dist.wrapping_sub(op); /* distance back in window */
                        if op > whave {
                            (*strm).msg = b"invalid distance too far back\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                            (*state).mode = crate::src::zlib::inflate::BAD;
                            break;
                        } else {
                            from = window.offset(-(1 as libc::c_int as isize));
                            if write == 0 as libc::c_int as libc::c_uint {
                                /* very common case */
                                from = from.offset(wsize.wrapping_sub(op) as isize);
                                if op < len {
                                    /* some from window */
                                    len = len.wrapping_sub(op);
                                    loop {
                                        from = from.offset(1);
                                        out = out.offset(1);
                                        *out = *from;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize))
                                    /* rest from output */
                                }
                            } else if write < op {
                                /* wrap around window */
                                from = from
                                    .offset(wsize.wrapping_add(write).wrapping_sub(op) as isize);
                                op = op.wrapping_sub(write);
                                if op < len {
                                    /* some from end of window */
                                    len = len.wrapping_sub(op);
                                    loop {
                                        from = from.offset(1);
                                        out = out.offset(1);
                                        *out = *from;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = window.offset(-(1 as libc::c_int as isize));
                                    if write < len {
                                        /* some from start of window */
                                        op = write;
                                        len = len.wrapping_sub(op);
                                        loop {
                                            from = from.offset(1);
                                            out = out.offset(1);
                                            *out = *from;
                                            op = op.wrapping_sub(1);
                                            if !(op != 0) {
                                                break;
                                            }
                                        }
                                        from = out.offset(-(dist as isize))
                                        /* rest from output */
                                    }
                                }
                            } else {
                                /* contiguous in window */
                                from = from.offset(write.wrapping_sub(op) as isize);
                                if op < len {
                                    /* some from window */
                                    len = len.wrapping_sub(op);
                                    loop {
                                        from = from.offset(1);
                                        out = out.offset(1);
                                        *out = *from;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize))
                                    /* rest from output */
                                }
                            }
                            while len > 2 as libc::c_int as libc::c_uint {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                                len = len.wrapping_sub(3 as libc::c_int as libc::c_uint)
                            }
                            if len != 0 {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                                if len > 1 as libc::c_int as libc::c_uint {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from
                                }
                            }
                        }
                    }
                }
            }
            4976922244085895320 => {
                (*strm).msg = b"invalid literal/length code\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*state).mode = crate::src::zlib::inflate::BAD;
                break;
            }
            5250576585193495047 =>
            /* end-of-block */
            {
                (*state).mode = crate::src::zlib::inflate::TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    /* return unused bytes (on entry, bits < 8, so in won't go too far back) */
    len = bits >> 3 as libc::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as libc::c_int);
    hold &= ((1 as libc::c_uint) << bits).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_ulong;
    /* update state and return */
    (*strm).next_in = in_0.offset(1 as libc::c_int as isize);
    (*strm).next_out = out.offset(1 as libc::c_int as isize);
    (*strm).avail_in = if in_0 < last {
        (5 as libc::c_int as libc::c_long) + last.offset_from(in_0) as libc::c_long
    } else {
        (5 as libc::c_int as libc::c_long) - in_0.offset_from(last) as libc::c_long
    } as libc::c_uint;
    (*strm).avail_out = if out < end {
        (257 as libc::c_int as libc::c_long) + end.offset_from(out) as libc::c_long
    } else {
        (257 as libc::c_int as libc::c_long) - out.offset_from(end) as libc::c_long
    } as libc::c_uint;
    (*state).hold = hold;
    (*state).bits = bits;
}
/* !ASMINF */
/*
  inflate_fast() speedups that turned out slower (on a PowerPC G3 750CXe):
  - Using bit fields for code structure
  - Different op definition to avoid & for extra bits (do & for table bits)
  - Three separate decoding do-loops for direct, window, and write == 0
  - Special case for distance > 1 copies to do overlapped load and store copy
  - Explicit branch predictions (based on measured branch probabilities)
  - Deferring match copy and interspersed it with decoding subsequent codes
  - Swapping literal/length else
  - Swapping window/direct else
  - Larger unrolled copy loops (three is about right)
  - Moving len -= 3 statement into middle of loop
*/

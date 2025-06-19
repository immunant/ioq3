use ::libc;

pub use crate::stdlib::__jmp_buf;
pub use crate::stdlib::__sigset_t;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::_setjmp;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;
/* number of fixed literal/length codes */
/* input and output state */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct state {
    pub out: *mut crate::stdlib::uint8_t,
    pub outlen: crate::stdlib::uint32_t,
    pub outcnt: crate::stdlib::uint32_t,
    pub in_0: *mut crate::stdlib::uint8_t,
    pub inlen: crate::stdlib::uint32_t,
    pub incnt: crate::stdlib::uint32_t,
    pub bitbuf: crate::stdlib::int32_t,
    pub bitcnt: crate::stdlib::int32_t,
    pub env: crate::stdlib::jmp_buf,
}
/*
 * Huffman code decoding tables.  count[1..MAXBITS] is the number of symbols of
 * each length, which for a canonical code are stepped through in order.
 * symbol[] are the symbol values in canonical order, where the number of
 * entries is the sum of the counts in count[].  The decoding process can be
 * seen in the function decode() below.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huffman {
    pub count: *mut crate::stdlib::int16_t,
    pub symbol: *mut crate::stdlib::int16_t,
}
/*
 * Return need bits from the input stream.  This always leaves less than
 * eight bits in the buffer.  bits() works properly for need == 0.
 *
 * Format notes:
 *
 * - Bits are stored in bytes from the least significant bit to the most
 *   significant bit.  Therefore bits are dropped from the bottom of the bit
 *   buffer, using shift right, and new bytes are appended to the top of the
 *   bit buffer, using shift left.
 */

unsafe extern "C" fn bits(
    mut s: *mut state,
    mut need: crate::stdlib::int32_t,
) -> crate::stdlib::int32_t {
    let mut val: crate::stdlib::int32_t = 0; /* bit accumulator (can use up to 20 bits) */
    /* load at least need bits into val */
    val = (*s).bitbuf; /* out of input */
    while (*s).bitcnt < need {
        if (*s).incnt == (*s).inlen {
            crate::stdlib::longjmp((*s).env.as_mut_ptr(), 1 as i32); /* load eight bits */
        }
        let fresh0 = (*s).incnt;
        (*s).incnt = (*s).incnt.wrapping_add(1);
        val |= (*(*s).in_0.offset(fresh0 as isize) as crate::stdlib::int32_t) << (*s).bitcnt;
        (*s).bitcnt += 8 as i32
    }
    /* drop need bits and update buffer, always zero to seven bits left */
    (*s).bitbuf = val >> need;
    (*s).bitcnt -= need;
    /* return need bits, zeroing the bits above that */
    return (val as libc::c_long & ((1 as libc::c_long) << need) - 1 as i32 as libc::c_long)
        as crate::stdlib::int32_t;
}
/*
 * Process a stored block.
 *
 * Format notes:
 *
 * - After the two-bit stored block type (00), the stored block length and
 *   stored bytes are byte-aligned for fast copying.  Therefore any leftover
 *   bits in the byte that has the last bit of the type, as many as seven, are
 *   discarded.  The value of the discarded bits are not defined and should not
 *   be checked against any expectation.
 *
 * - The second inverted copy of the stored block length does not have to be
 *   checked, but it's probably a good idea to do so anyway.
 *
 * - A stored block can have zero length.  This is sometimes used to byte-align
 *   subsets of the compressed data for random access or partial recovery.
 */

unsafe extern "C" fn stored(mut s: *mut state) -> crate::stdlib::int32_t {
    let mut len: crate::stdlib::uint32_t = 0; /* length of stored block */
    /* discard leftover bits from current byte (assumes s->bitcnt < 8) */
    (*s).bitbuf = 0 as i32;
    (*s).bitcnt = 0 as i32;
    /* get length and check against its one's complement */
    if (*s).incnt.wrapping_add(4 as i32 as u32) > (*s).inlen {
        return 2 as i32;
    } /* not enough input */
    let fresh1 = (*s).incnt; /* didn't match complement! */
    (*s).incnt = (*s).incnt.wrapping_add(1);
    len = *(*s).in_0.offset(fresh1 as isize) as crate::stdlib::uint32_t;
    let fresh2 = (*s).incnt;
    (*s).incnt = (*s).incnt.wrapping_add(1);
    len |=
        ((*(*s).in_0.offset(fresh2 as isize) as i32) << 8 as i32) as u32;
    let fresh3 = (*s).incnt;
    (*s).incnt = (*s).incnt.wrapping_add(1);
    if *(*s).in_0.offset(fresh3 as isize) as u32
        != !len & 0xff as i32 as u32
        || {
            let fresh4 = (*s).incnt;
            (*s).incnt = (*s).incnt.wrapping_add(1);
            (*(*s).in_0.offset(fresh4 as isize) as u32)
                != !len >> 8 as i32 & 0xff as i32 as u32
        }
    {
        return -(2 as i32);
    }
    /* copy len bytes from in to out */
    if (*s).incnt.wrapping_add(len) > (*s).inlen {
        return 2 as i32;
    } /* not enough input */
    if !(*s).out.is_null() {
        if (*s).outcnt.wrapping_add(len) > (*s).outlen {
            return 1 as i32;
        } /* not enough output space */
        loop {
            let fresh5 = len;
            len = len.wrapping_sub(1);
            if !(fresh5 != 0) {
                break;
            }
            let fresh6 = (*s).incnt;
            (*s).incnt = (*s).incnt.wrapping_add(1);
            let fresh7 = (*s).outcnt;
            (*s).outcnt = (*s).outcnt.wrapping_add(1);
            *(*s).out.offset(fresh7 as isize) = *(*s).in_0.offset(fresh6 as isize)
        }
    } else {
        /* just scanning */
        (*s).outcnt = ((*s).outcnt as u32).wrapping_add(len) as crate::stdlib::uint32_t
            as crate::stdlib::uint32_t;
        (*s).incnt = ((*s).incnt as u32).wrapping_add(len) as crate::stdlib::uint32_t
            as crate::stdlib::uint32_t
    }
    /* done with a valid stored block */
    return 0 as i32;
}
/* canonically ordered symbols */
/*
 * Decode a code from the stream s using huffman table h.  Return the symbol or
 * a negative value if there is an error.  If all of the lengths are zero, i.e.
 * an empty code, or if the code is incomplete and an invalid code is received,
 * then -9 is returned after reading MAXBITS bits.
 *
 * Format notes:
 *
 * - The codes as stored in the compressed data are bit-reversed relative to
 *   a simple integer ordering of codes of the same lengths.  Hence below the
 *   bits are pulled from the compressed data one at a time and used to
 *   build the code value reversed from what is in the stream in order to
 *   permit simple integer comparisons for decoding.  A table-based decoding
 *   scheme (as used in zlib) does not need to do this reversal.
 *
 * - The first code for the shortest length is all zeros.  Subsequent codes of
 *   the same length are simply integer increments of the previous code.  When
 *   moving up a length, a zero bit is appended to the code.  For a complete
 *   code, the last code of the longest length will be all ones.
 *
 * - Incomplete codes are handled by this decoder, since they are permitted
 *   in the deflate format.  See the format notes for fixed() and dynamic().
 */

unsafe extern "C" fn decode(mut s: *mut state, mut h: *mut huffman) -> crate::stdlib::int32_t {
    let mut len: crate::stdlib::int32_t = 0; /* current number of bits in code */
    let mut code: crate::stdlib::int32_t = 0; /* len bits being decoded */
    let mut first: crate::stdlib::int32_t = 0; /* first code of length len */
    let mut count: crate::stdlib::int32_t = 0; /* number of codes of length len */
    let mut index: crate::stdlib::int32_t = 0; /* index of first code of length len in symbol table */
    let mut bitbuf: crate::stdlib::int32_t = 0; /* bits from stream */
    let mut left: crate::stdlib::int32_t = 0; /* bits left in next or left to process */
    let mut next: *mut crate::stdlib::int16_t = 0 as *mut crate::stdlib::int16_t; /* next number of codes */
    bitbuf = (*s).bitbuf;
    left = (*s).bitcnt;
    index = 0 as i32;
    first = index;
    code = first;
    len = 1 as i32;
    next = (*h).count.offset(1 as i32 as isize);
    loop {
        loop {
            let fresh8 = left;
            left = left - 1;
            if !(fresh8 != 0) {
                break;
            }
            code |= bitbuf & 1 as i32;
            bitbuf >>= 1 as i32;
            let fresh9 = next;
            next = next.offset(1);
            count = *fresh9 as crate::stdlib::int32_t;
            if code < first + count {
                /* if length len, return symbol */
                (*s).bitbuf = bitbuf; /* else update for next length */
                (*s).bitcnt = (*s).bitcnt - len & 7 as i32; /* out of input */
                return *(*h).symbol.offset((index + (code - first)) as isize)
                    as crate::stdlib::int32_t;
            }
            index += count;
            first += count;
            first <<= 1 as i32;
            code <<= 1 as i32;
            len += 1
        }
        left = 15 as i32 + 1 as i32 - len;
        if left == 0 as i32 {
            break;
        }
        if (*s).incnt == (*s).inlen {
            crate::stdlib::longjmp((*s).env.as_mut_ptr(), 1 as i32);
        }
        let fresh10 = (*s).incnt;
        (*s).incnt = (*s).incnt.wrapping_add(1);
        bitbuf = *(*s).in_0.offset(fresh10 as isize) as crate::stdlib::int32_t;
        if left > 8 as i32 {
            left = 8 as i32
        }
    }
    return -(9 as i32);
    /* ran out of codes */
}
/*
 * Given the list of code lengths length[0..n-1] representing a canonical
 * Huffman code for n symbols, construct the tables required to decode those
 * codes.  Those tables are the number of codes of each length, and the symbols
 * sorted by length, retaining their original order within each length.  The
 * return value is zero for a complete code set, negative for an over-
 * subscribed code set, and positive for an incomplete code set.  The tables
 * can be used if the return value is zero or positive, but they cannot be used
 * if the return value is negative.  If the return value is zero, it is not
 * possible for decode() using that table to return an error--any stream of
 * enough bits will resolve to a symbol.  If the return value is positive, then
 * it is possible for decode() using that table to return an error for received
 * codes past the end of the incomplete lengths.
 *
 * Not used by decode(), but used for error checking, h->count[0] is the number
 * of the n symbols not in the code.  So n - h->count[0] is the number of
 * codes.  This is useful for checking for incomplete codes that have more than
 * one symbol, which is an error in a dynamic block.
 *
 * Assumption: for all i in 0..n-1, 0 <= length[i] <= MAXBITS
 * This is assured by the construction of the length arrays in dynamic() and
 * fixed() and is not verified by construct().
 *
 * Format notes:
 *
 * - Permitted and expected examples of incomplete codes are one of the fixed
 *   codes and any code with a single symbol which in deflate is coded as one
 *   bit instead of zero bits.  See the format notes for fixed() and dynamic().
 *
 * - Within a given code length, the symbols are kept in ascending order for
 *   the code bits definition.
 */

unsafe extern "C" fn construct(
    mut h: *mut huffman,
    mut length: *mut crate::stdlib::int16_t,
    mut n: crate::stdlib::int32_t,
) -> crate::stdlib::int32_t {
    let mut symbol: crate::stdlib::int32_t = 0; /* current symbol when stepping through length[] */
    let mut len: crate::stdlib::int32_t = 0; /* current length when stepping through h->count[] */
    let mut left: crate::stdlib::int32_t = 0; /* number of possible codes left of current length */
    let mut offs: [crate::stdlib::int16_t; 16] = [0; 16]; /* offsets in symbol table for each length */
    /* count number of codes of each length */
    len = 0 as i32; /* assumes lengths are within bounds */
    while len <= 15 as i32 {
        *(*h).count.offset(len as isize) = 0 as i32 as crate::stdlib::int16_t; /* complete, but decode() will fail */
        len += 1
    }
    symbol = 0 as i32;
    while symbol < n {
        let ref mut fresh11 = *(*h).count.offset(*length.offset(symbol as isize) as isize);
        *fresh11 += 1;
        symbol += 1
    }
    if *(*h).count.offset(0 as i32 as isize) as i32 == n {
        /* no codes! */
        return 0 as i32;
    }
    /* check for an over-subscribed or incomplete set of lengths */
    left = 1 as i32; /* one possible code of zero length */
    len = 1 as i32; /* left > 0 means incomplete */
    while len <= 15 as i32 {
        left <<= 1 as i32; /* one more bit, double codes left */
        /* over-subscribed--return negative */
        left -= *(*h).count.offset(len as isize) as i32; /* deduct count from possible codes */
        if left < 0 as i32 {
            return left;
        }
        len += 1
    }
    /* generate offsets into symbol table for each length for sorting */
    offs[1 as i32 as usize] = 0 as i32 as crate::stdlib::int16_t;
    len = 1 as i32;
    while len < 15 as i32 {
        offs[(len + 1 as i32) as usize] = (offs[len as usize] as i32
            + *(*h).count.offset(len as isize) as i32)
            as crate::stdlib::int16_t;
        len += 1
    }
    /*
     * put symbols in table sorted by length, by symbol order within each
     * length
     */
    symbol = 0 as i32;
    while symbol < n {
        if *length.offset(symbol as isize) as i32 != 0 as i32 {
            let fresh12 = offs[*length.offset(symbol as isize) as usize];
            offs[*length.offset(symbol as isize) as usize] =
                offs[*length.offset(symbol as isize) as usize] + 1;
            *(*h).symbol.offset(fresh12 as isize) = symbol as crate::stdlib::int16_t
        }
        symbol += 1
    }
    /* return zero for complete set, positive for incomplete set */
    return left;
}
/*
 * Decode literal/length and distance codes until an end-of-block code.
 *
 * Format notes:
 *
 * - Compressed data that is after the block type if fixed or after the code
 *   description if dynamic is a combination of literals and length/distance
 *   pairs terminated by and end-of-block code.  Literals are simply Huffman
 *   coded bytes.  A length/distance pair is a coded length followed by a
 *   coded distance to represent a string that occurs earlier in the
 *   uncompressed data that occurs again at the current location.
 *
 * - Literals, lengths, and the end-of-block code are combined into a single
 *   code of up to 286 symbols.  They are 256 literals (0..255), 29 length
 *   symbols (257..285), and the end-of-block symbol (256).
 *
 * - There are 256 possible lengths (3..258), and so 29 symbols are not enough
 *   to represent all of those.  Lengths 3..10 and 258 are in fact represented
 *   by just a length symbol.  Lengths 11..257 are represented as a symbol and
 *   some number of extra bits that are added as an integer to the base length
 *   of the length symbol.  The number of extra bits is determined by the base
 *   length symbol.  These are in the static arrays below, lens[] for the base
 *   lengths and lext[] for the corresponding number of extra bits.
 *
 * - The reason that 258 gets its own symbol is that the longest length is used
 *   often in highly redundant files.  Note that 258 can also be coded as the
 *   base value 227 plus the maximum extra value of 31.  While a good deflate
 *   should never do this, it is not an error, and should be decoded properly.
 *
 * - If a length is decoded, including its extra bits if any, then it is
 *   followed a distance code.  There are up to 30 distance symbols.  Again
 *   there are many more possible distances (1..32768), so extra bits are added
 *   to a base value represented by the symbol.  The distances 1..4 get their
 *   own symbol, but the rest require extra bits.  The base distances and
 *   corresponding number of extra bits are below in the static arrays dist[]
 *   and dext[].
 *
 * - Literal bytes are simply written to the output.  A length/distance pair is
 *   an instruction to copy previously uncompressed bytes to the output.  The
 *   copy is from distance bytes back in the output stream, copying for length
 *   bytes.
 *
 * - Distances pointing before the beginning of the output data are not
 *   permitted.
 *
 * - Overlapped copies, where the length is greater than the distance, are
 *   allowed and common.  For example, a distance of one and a length of 258
 *   simply copies the last byte 258 times.  A distance of four and a length of
 *   twelve copies the last four bytes three times.  A simple forward copy
 *   ignoring whether the length is greater than the distance or not implements
 *   this correctly.  You should not use memcpy() since its behavior is not
 *   defined for overlapped arrays.  You should not use memmove() or bcopy()
 *   since though their behavior -is- defined for overlapping arrays, it is
 *   defined to do the wrong thing in this case.
 */

unsafe extern "C" fn codes(
    mut s: *mut state,
    mut lencode: *mut huffman,
    mut distcode: *mut huffman,
) -> crate::stdlib::int32_t {
    let mut symbol: crate::stdlib::int32_t = 0; /* decoded symbol */
    let mut len: crate::stdlib::int32_t = 0; /* length for copy */
    let mut dist: crate::stdlib::uint32_t = 0; /* distance for copy */
    static mut lens: [crate::stdlib::int16_t; 29] = [
        3 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        6 as i32 as crate::stdlib::int16_t,
        7 as i32 as crate::stdlib::int16_t,
        8 as i32 as crate::stdlib::int16_t,
        9 as i32 as crate::stdlib::int16_t,
        10 as i32 as crate::stdlib::int16_t,
        11 as i32 as crate::stdlib::int16_t,
        13 as i32 as crate::stdlib::int16_t,
        15 as i32 as crate::stdlib::int16_t,
        17 as i32 as crate::stdlib::int16_t,
        19 as i32 as crate::stdlib::int16_t,
        23 as i32 as crate::stdlib::int16_t,
        27 as i32 as crate::stdlib::int16_t,
        31 as i32 as crate::stdlib::int16_t,
        35 as i32 as crate::stdlib::int16_t,
        43 as i32 as crate::stdlib::int16_t,
        51 as i32 as crate::stdlib::int16_t,
        59 as i32 as crate::stdlib::int16_t,
        67 as i32 as crate::stdlib::int16_t,
        83 as i32 as crate::stdlib::int16_t,
        99 as i32 as crate::stdlib::int16_t,
        115 as i32 as crate::stdlib::int16_t,
        131 as i32 as crate::stdlib::int16_t,
        163 as i32 as crate::stdlib::int16_t,
        195 as i32 as crate::stdlib::int16_t,
        227 as i32 as crate::stdlib::int16_t,
        258 as i32 as crate::stdlib::int16_t,
    ];
    static mut lext: [crate::stdlib::int16_t; 29] = [
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
    ];
    static mut dists: [crate::stdlib::int16_t; 30] = [
        1 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        7 as i32 as crate::stdlib::int16_t,
        9 as i32 as crate::stdlib::int16_t,
        13 as i32 as crate::stdlib::int16_t,
        17 as i32 as crate::stdlib::int16_t,
        25 as i32 as crate::stdlib::int16_t,
        33 as i32 as crate::stdlib::int16_t,
        49 as i32 as crate::stdlib::int16_t,
        65 as i32 as crate::stdlib::int16_t,
        97 as i32 as crate::stdlib::int16_t,
        129 as i32 as crate::stdlib::int16_t,
        193 as i32 as crate::stdlib::int16_t,
        257 as i32 as crate::stdlib::int16_t,
        385 as i32 as crate::stdlib::int16_t,
        513 as i32 as crate::stdlib::int16_t,
        769 as i32 as crate::stdlib::int16_t,
        1025 as i32 as crate::stdlib::int16_t,
        1537 as i32 as crate::stdlib::int16_t,
        2049 as i32 as crate::stdlib::int16_t,
        3073 as i32 as crate::stdlib::int16_t,
        4097 as i32 as crate::stdlib::int16_t,
        6145 as i32 as crate::stdlib::int16_t,
        8193 as i32 as crate::stdlib::int16_t,
        12289 as i32 as crate::stdlib::int16_t,
        16385 as i32 as crate::stdlib::int16_t,
        24577 as i32 as crate::stdlib::int16_t,
    ];
    static mut dext: [crate::stdlib::int16_t; 30] = [
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        6 as i32 as crate::stdlib::int16_t,
        6 as i32 as crate::stdlib::int16_t,
        7 as i32 as crate::stdlib::int16_t,
        7 as i32 as crate::stdlib::int16_t,
        8 as i32 as crate::stdlib::int16_t,
        8 as i32 as crate::stdlib::int16_t,
        9 as i32 as crate::stdlib::int16_t,
        9 as i32 as crate::stdlib::int16_t,
        10 as i32 as crate::stdlib::int16_t,
        10 as i32 as crate::stdlib::int16_t,
        11 as i32 as crate::stdlib::int16_t,
        11 as i32 as crate::stdlib::int16_t,
        12 as i32 as crate::stdlib::int16_t,
        12 as i32 as crate::stdlib::int16_t,
        13 as i32 as crate::stdlib::int16_t,
        13 as i32 as crate::stdlib::int16_t,
    ];
    loop
    /* decode literals and length/distance pairs */
    {
        symbol = decode(s, lencode); /* end of block symbol */
        if symbol < 0 as i32 {
            return symbol;
        } /* invalid symbol */
        if symbol < 256 as i32 {
            /* literal: symbol is the byte */
            /* write out the literal */
            if !(*s).out.is_null() {
                if (*s).outcnt == (*s).outlen {
                    return 1 as i32;
                }
                *(*s).out.offset((*s).outcnt as isize) = symbol as crate::stdlib::uint8_t
            }
            (*s).outcnt = (*s).outcnt.wrapping_add(1)
        } else if symbol > 256 as i32 {
            /* length */
            /* get and compute length */
            symbol -= 257 as i32; /* invalid fixed code */
            if symbol >= 29 as i32 {
                return -(9 as i32);
            }
            len = lens[symbol as usize] as i32
                + bits(s, lext[symbol as usize] as crate::stdlib::int32_t);
            /* get and check distance */
            symbol = decode(s, distcode); /* invalid symbol */
            if symbol < 0 as i32 {
                return symbol;
            } /* distance too far back */
            dist = (dists[symbol as usize] as i32
                + bits(s, dext[symbol as usize] as crate::stdlib::int32_t))
                as crate::stdlib::uint32_t;
            if dist > (*s).outcnt {
                return -(10 as i32);
            }
            /* copy length bytes from distance bytes back */
            if !(*s).out.is_null() {
                if (*s).outcnt.wrapping_add(len as u32) > (*s).outlen {
                    return 1 as i32;
                }
                loop {
                    let fresh13 = len;
                    len = len - 1;
                    if !(fresh13 != 0) {
                        break;
                    }
                    *(*s).out.offset((*s).outcnt as isize) =
                        *(*s).out.offset((*s).outcnt.wrapping_sub(dist) as isize);
                    (*s).outcnt = (*s).outcnt.wrapping_add(1)
                }
            } else {
                (*s).outcnt = ((*s).outcnt as u32).wrapping_add(len as u32)
                    as crate::stdlib::uint32_t
                    as crate::stdlib::uint32_t
            }
        }
        if !(symbol != 256 as i32) {
            break;
        }
    }
    /* done with a valid fixed or dynamic block */
    return 0 as i32;
}
/*
 * Process a fixed codes block.
 *
 * Format notes:
 *
 * - This block type can be useful for compressing small amounts of data for
 *   which the size of the code descriptions in a dynamic block exceeds the
 *   benefit of custom codes for that block.  For fixed codes, no bits are
 *   spent on code descriptions.  Instead the code lengths for literal/length
 *   codes and distance codes are fixed.  The specific lengths for each symbol
 *   can be seen in the "for" loops below.
 *
 * - The literal/length code is complete, but has two symbols that are invalid
 *   and should result in an error if received.  This cannot be implemented
 *   simply as an incomplete code since those two symbols are in the "middle"
 *   of the code.  They are eight bits long and the longest literal/length\
 *   code is nine bits.  Therefore the code must be constructed with those
 *   symbols, and the invalid symbols must be detected after decoding.
 *
 * - The fixed distance codes also have two invalid symbols that should result
 *   in an error if received.  Since all of the distance codes are the same
 *   length, this can be implemented as an incomplete code.  Then the invalid
 *   codes are detected while decoding.
 */

unsafe extern "C" fn fixed(mut s: *mut state) -> crate::stdlib::int32_t {
    static mut virgin: crate::stdlib::int32_t = 1 as i32;
    static mut lencnt: [crate::stdlib::int16_t; 16] = [0; 16];
    static mut lensym: [crate::stdlib::int16_t; 288] = [0; 288];
    static mut distcnt: [crate::stdlib::int16_t; 16] = [0; 16];
    static mut distsym: [crate::stdlib::int16_t; 30] = [0; 30];
    static mut lencode: huffman = unsafe {
        {
            let mut init = huffman {
                count: lencnt.as_ptr() as *mut _,
                symbol: lensym.as_ptr() as *mut _,
            };
            init
        }
    };
    static mut distcode: huffman = unsafe {
        {
            let mut init = huffman {
                count: distcnt.as_ptr() as *mut _,
                symbol: distsym.as_ptr() as *mut _,
            };
            init
        }
    };
    /* build fixed huffman tables if first call (may not be thread safe) */
    if virgin != 0 {
        let mut symbol: crate::stdlib::int32_t = 0;
        let mut lengths: [crate::stdlib::int16_t; 288] = [0; 288];
        /* literal/length table */
        symbol = 0 as i32;
        while symbol < 144 as i32 {
            lengths[symbol as usize] = 8 as i32 as crate::stdlib::int16_t;
            symbol += 1
        }
        while symbol < 256 as i32 {
            lengths[symbol as usize] = 9 as i32 as crate::stdlib::int16_t;
            symbol += 1
        }
        while symbol < 280 as i32 {
            lengths[symbol as usize] = 7 as i32 as crate::stdlib::int16_t;
            symbol += 1
        }
        while symbol < 288 as i32 {
            lengths[symbol as usize] = 8 as i32 as crate::stdlib::int16_t;
            symbol += 1
        }
        construct(&mut lencode, lengths.as_mut_ptr(), 288 as i32);
        /* distance table */
        symbol = 0 as i32;
        while symbol < 30 as i32 {
            lengths[symbol as usize] = 5 as i32 as crate::stdlib::int16_t;
            symbol += 1
        }
        construct(&mut distcode, lengths.as_mut_ptr(), 30 as i32);
        /* do this just once */
        virgin = 0 as i32
    }
    /* decode data until end-of-block code */
    return codes(s, &mut lencode, &mut distcode);
}
/*
 * Process a dynamic codes block.
 *
 * Format notes:
 *
 * - A dynamic block starts with a description of the literal/length and
 *   distance codes for that block.  New dynamic blocks allow the compressor to
 *   rapidly adapt to changing data with new codes optimized for that data.
 *
 * - The codes used by the deflate format are "canonical", which means that
 *   the actual bits of the codes are generated in an unambiguous way simply
 *   from the number of bits in each code.  Therefore the code descriptions
 *   are simply a list of code lengths for each symbol.
 *
 * - The code lengths are stored in order for the symbols, so lengths are
 *   provided for each of the literal/length symbols, and for each of the
 *   distance symbols.
 *
 * - If a symbol is not used in the block, this is represented by a zero as
 *   as the code length.  This does not mean a zero-length code, but rather
 *   that no code should be created for this symbol.  There is no way in the
 *   deflate format to represent a zero-length code.
 *
 * - The maximum number of bits in a code is 15, so the possible lengths for
 *   any code are 1..15.
 *
 * - The fact that a length of zero is not permitted for a code has an
 *   interesting consequence.  Normally if only one symbol is used for a given
 *   code, then in fact that code could be represented with zero bits.  However
 *   in deflate, that code has to be at least one bit.  So for example, if
 *   only a single distance base symbol appears in a block, then it will be
 *   represented by a single code of length one, in particular one 0 bit.  This
 *   is an incomplete code, since if a 1 bit is received, it has no meaning,
 *   and should result in an error.  So incomplete distance codes of one symbol
 *   should be permitted, and the receipt of invalid codes should be handled.
 *
 * - It is also possible to have a single literal/length code, but that code
 *   must be the end-of-block code, since every dynamic block has one.  This
 *   is not the most efficient way to create an empty block (an empty fixed
 *   block is fewer bits), but it is allowed by the format.  So incomplete
 *   literal/length codes of one symbol should also be permitted.
 *
 * - If there are only literal codes and no lengths, then there are no distance
 *   codes.  This is represented by one distance code with zero bits.
 *
 * - The list of up to 286 length/literal lengths and up to 30 distance lengths
 *   are themselves compressed using Huffman codes and run-length encoding.  In
 *   the list of code lengths, a 0 symbol means no code, a 1..15 symbol means
 *   that length, and the symbols 16, 17, and 18 are run-length instructions.
 *   Each of 16, 17, and 18 are follwed by extra bits to define the length of
 *   the run.  16 copies the last length 3 to 6 times.  17 represents 3 to 10
 *   zero lengths, and 18 represents 11 to 138 zero lengths.  Unused symbols
 *   are common, hence the special coding for zero lengths.
 *
 * - The symbols for 0..18 are Huffman coded, and so that code must be
 *   described first.  This is simply a sequence of up to 19 three-bit values
 *   representing no code (0) or the code length for that symbol (1..7).
 *
 * - A dynamic block starts with three fixed-size counts from which is computed
 *   the number of literal/length code lengths, the number of distance code
 *   lengths, and the number of code length code lengths (ok, you come up with
 *   a better name!) in the code descriptions.  For the literal/length and
 *   distance codes, lengths after those provided are considered zero, i.e. no
 *   code.  The code length code lengths are received in a permuted order (see
 *   the order[] array below) to make a short code length code length list more
 *   likely.  As it turns out, very short and very long codes are less likely
 *   to be seen in a dynamic code description, hence what may appear initially
 *   to be a peculiar ordering.
 *
 * - Given the number of literal/length code lengths (nlen) and distance code
 *   lengths (ndist), then they are treated as one long list of nlen + ndist
 *   code lengths.  Therefore run-length coding can and often does cross the
 *   boundary between the two sets of lengths.
 *
 * - So to summarize, the code description at the start of a dynamic block is
 *   three counts for the number of code lengths for the literal/length codes,
 *   the distance codes, and the code length codes.  This is followed by the
 *   code length code lengths, three bits each.  This is used to construct the
 *   code length code which is used to read the remainder of the lengths.  Then
 *   the literal/length code lengths and distance lengths are read as a single
 *   set of lengths using the code length codes.  Codes are constructed from
 *   the resulting two sets of lengths, and then finally you can start
 *   decoding actual compressed data in the block.
 *
 * - For reference, a "typical" size for the code description in a dynamic
 *   block is around 80 bytes.
 */

unsafe extern "C" fn dynamic(mut s: *mut state) -> crate::stdlib::int32_t {
    let mut nlen: crate::stdlib::int32_t = 0; /* number of lengths in descriptor */
    let mut ndist: crate::stdlib::int32_t = 0; /* index of lengths[] */
    let mut ncode: crate::stdlib::int32_t = 0; /* construct() return value */
    let mut index: crate::stdlib::int32_t = 0; /* descriptor code lengths */
    let mut err: crate::stdlib::int32_t = 0; /* lencode memory */
    let mut lengths: [crate::stdlib::int16_t; 316] = [0; 316]; /* distcode memory */
    let mut lencnt: [crate::stdlib::int16_t; 16] = [0; 16]; /* length code */
    let mut lensym: [crate::stdlib::int16_t; 286] = [0; 286]; /* distance code */
    let mut distcnt: [crate::stdlib::int16_t; 16] = [0; 16];
    let mut distsym: [crate::stdlib::int16_t; 30] = [0; 30];
    let mut lencode: huffman = {
        let mut init = huffman {
            count: lencnt.as_mut_ptr(),
            symbol: lensym.as_mut_ptr(),
        };
        init
    };
    let mut distcode: huffman = {
        let mut init = huffman {
            count: distcnt.as_mut_ptr(),
            symbol: distsym.as_mut_ptr(),
        };
        init
    };
    static mut order: [crate::stdlib::int16_t; 19] = [
        16 as i32 as crate::stdlib::int16_t,
        17 as i32 as crate::stdlib::int16_t,
        18 as i32 as crate::stdlib::int16_t,
        0 as i32 as crate::stdlib::int16_t,
        8 as i32 as crate::stdlib::int16_t,
        7 as i32 as crate::stdlib::int16_t,
        9 as i32 as crate::stdlib::int16_t,
        6 as i32 as crate::stdlib::int16_t,
        10 as i32 as crate::stdlib::int16_t,
        5 as i32 as crate::stdlib::int16_t,
        11 as i32 as crate::stdlib::int16_t,
        4 as i32 as crate::stdlib::int16_t,
        12 as i32 as crate::stdlib::int16_t,
        3 as i32 as crate::stdlib::int16_t,
        13 as i32 as crate::stdlib::int16_t,
        2 as i32 as crate::stdlib::int16_t,
        14 as i32 as crate::stdlib::int16_t,
        1 as i32 as crate::stdlib::int16_t,
        15 as i32 as crate::stdlib::int16_t,
    ];
    /* get number of lengths in each table, check lengths */
    nlen = bits(s, 5 as i32) + 257 as i32; /* bad counts */
    ndist = bits(s, 5 as i32) + 1 as i32;
    ncode = bits(s, 4 as i32) + 4 as i32;
    if nlen > 286 as i32 || ndist > 30 as i32 {
        return -(3 as i32);
    }
    /* read code length code lengths (really), missing lengths are zero */
    index = 0 as i32;
    while index < ncode {
        lengths[order[index as usize] as usize] =
            bits(s, 3 as i32) as crate::stdlib::int16_t;
        index += 1
    }
    while index < 19 as i32 {
        lengths[order[index as usize] as usize] = 0 as i32 as crate::stdlib::int16_t;
        index += 1
    }
    /* build huffman table for code lengths codes (use lencode temporarily) */
    err = construct(&mut lencode, lengths.as_mut_ptr(), 19 as i32); /* require complete code set here */
    if err != 0 as i32 {
        return -(4 as i32);
    }
    /* read length/literal and distance code length tables */
    index = 0 as i32; /* decoded value */
    while index < nlen + ndist {
        let mut symbol: crate::stdlib::int32_t = 0; /* last length to repeat */
        let mut len: crate::stdlib::int32_t = 0;
        symbol = decode(s, &mut lencode);
        if symbol < 16 as i32 {
            /* length in 0..15 */
            let fresh14 = index;
            index = index + 1;
            lengths[fresh14 as usize] = symbol as crate::stdlib::int16_t
        } else {
            /* repeat instruction */
            len = 0 as i32; /* assume repeating zeros */
            if symbol == 16 as i32 {
                /* repeat last length 3..6 times */
                if index == 0 as i32 {
                    return -(5 as i32);
                } /* no last length! */
                len = lengths[(index - 1 as i32) as usize] as crate::stdlib::int32_t; /* last length */
                symbol = 3 as i32 + bits(s, 2 as i32)
            } else if symbol == 17 as i32 {
                /* repeat zero 3..10 times */
                symbol = 3 as i32 + bits(s, 3 as i32)
            } else {
                /* == 18, repeat zero 11..138 times */
                symbol = 11 as i32 + bits(s, 7 as i32)
            } /* too many lengths! */
            if index + symbol > nlen + ndist {
                return -(6 as i32);
            }
            loop {
                let fresh15 = symbol;
                symbol = symbol - 1;
                if !(fresh15 != 0) {
                    break;
                }
                /* repeat last or zero symbol times */
                let fresh16 = index;
                index = index + 1;
                lengths[fresh16 as usize] = len as crate::stdlib::int16_t
            }
        }
    }
    /* build huffman table for literal/length codes */
    err = construct(&mut lencode, lengths.as_mut_ptr(), nlen); /* only allow incomplete codes if just one code */
    if err < 0 as i32
        || err > 0 as i32
            && nlen - *lencode.count.offset(0 as i32 as isize) as i32
                != 1 as i32
    {
        return -(7 as i32);
    }
    /* build huffman table for distance codes */
    err = construct(
        &mut distcode,
        lengths.as_mut_ptr().offset(nlen as isize),
        ndist,
    ); /* only allow incomplete codes if just one code */
    if err < 0 as i32
        || err > 0 as i32
            && ndist - *distcode.count.offset(0 as i32 as isize) as i32
                != 1 as i32
    {
        return -(8 as i32);
    }
    /* decode data until end-of-block code */
    return codes(s, &mut lencode, &mut distcode);
}
/*
 *  This is a modified version of Mark Adlers work,
 *  see below for the original copyright.
 *  2006 - Joerg Dietrich <dietrich_joerg@gmx.de>
 */
/* puff.h
 Copyright (C) 2002, 2003 Mark Adler, all rights reserved
 version 1.7, 3 Mar 2002

 This software is provided 'as-is', without any express or implied
 warranty.  In no event will the author be held liable for any damages
 arising from the use of this software.

 Permission is granted to anyone to use this software for any purpose,
 including commercial applications, and to alter it and redistribute it
 freely, subject to the following restrictions:

 1. The origin of this software must not be misrepresented; you must not
    claim that you wrote the original software. If you use this software
    in a product, an acknowledgment in the product documentation would be
    appreciated but is not required.
 2. Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.
 3. This notice may not be removed or altered from any source distribution.

 Mark Adler    madler@alumni.caltech.edu
*/
/*
 * See puff.c for purpose and usage.
 */
/*
 * Inflate source to dest.  On return, destlen and sourcelen are updated to the
 * size of the uncompressed data and the size of the deflate data respectively.
 * On success, the return value of puff() is zero.  If there is an error in the
 * source data, i.e. it is not in the deflate format, then a negative value is
 * returned.  If there is not enough input available or there is not enough
 * output space, then a positive error is returned.  In that case, destlen and
 * sourcelen are not updated to facilitate retrying from the beginning with the
 * provision of more input data or more output space.  In the case of invalid
 * inflate data (a negative error), the dest and source pointers are updated to
 * facilitate the debugging of deflators.
 *
 * puff() also has a mode to determine the size of the uncompressed output with
 * no output written.  For this dest must be (uint8_t *)0.  In this case,
 * the input value of *destlen is ignored, and on return *destlen is set to the
 * size of the uncompressed output.
 *
 * The return codes are:
 *
 *   2:  available inflate data did not terminate
 *   1:  output space exhausted before completing inflate
 *   0:  successful inflate
 *  -1:  invalid block type (type == 3)
 *  -2:  stored block length did not match one's complement
 *  -3:  dynamic block code description: too many length or distance codes
 *  -4:  dynamic block code description: code lengths codes incomplete
 *  -5:  dynamic block code description: repeat lengths with no first length
 *  -6:  dynamic block code description: repeat more than specified lengths
 *  -7:  dynamic block code description: invalid literal/length code lengths
 *  -8:  dynamic block code description: invalid distance code lengths
 *  -9:  invalid literal/length or distance code in fixed or dynamic block
 * -10:  distance is too far back in fixed or dynamic block
 *
 * Format notes:
 *
 * - Three bits are read for each block to determine the kind of block and
 *   whether or not it is the last block.  Then the block is decoded and the
 *   process repeated if it was not the last block.
 *
 * - The leftover bits in the last byte of the deflate data after the last
 *   block (if it was a fixed or dynamic block) are undefined and have no
 *   expected values to check.
 */
#[no_mangle]

pub unsafe extern "C" fn puff(
    mut dest: *mut crate::stdlib::uint8_t,
    mut destlen: *mut crate::stdlib::uint32_t,
    mut source: *mut crate::stdlib::uint8_t,
    mut sourcelen: *mut crate::stdlib::uint32_t,
) -> crate::stdlib::int32_t
/* amount of input available */ {
    let mut s: state = state {
        out: 0 as *mut crate::stdlib::uint8_t,
        outlen: 0,
        outcnt: 0,
        in_0: 0 as *mut crate::stdlib::uint8_t,
        inlen: 0,
        incnt: 0,
        bitbuf: 0,
        bitcnt: 0,
        env: [crate::stdlib::__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: crate::stdlib::__sigset_t { __val: [0; 16] },
        }; 1],
    }; /* input/output state */
    let mut last: crate::stdlib::int32_t = 0; /* block information */
    let mut type_0: crate::stdlib::int32_t = 0; /* return value */
    let mut err: crate::stdlib::int32_t = 0;
    /* initialize output state */
    s.out = dest; /* ignored if dest is NULL */
    s.outlen = *destlen;
    s.outcnt = 0 as i32 as crate::stdlib::uint32_t;
    /* initialize input state */
    s.in_0 = source;
    s.inlen = *sourcelen;
    s.incnt = 0 as i32 as crate::stdlib::uint32_t;
    s.bitbuf = 0 as i32;
    s.bitcnt = 0 as i32;
    /* return if bits() or decode() tries to read past available input */
    if crate::stdlib::_setjmp(s.env.as_mut_ptr()) != 0 as i32 {
        /* if came back here via longjmp() */
        err = 2 as i32
    } else {
        loop
        /* then skip do-loop, return error */
        /* process blocks until last block or error */
        {
            last = bits(&mut s, 1 as i32); /* one if last block */
            type_0 = bits(&mut s, 2 as i32); /* block type 0..3 */
            err = if type_0 == 0 as i32 {
                stored(&mut s)
            } else if type_0 == 1 as i32 {
                fixed(&mut s)
            } else if type_0 == 2 as i32 {
                dynamic(&mut s)
            } else {
                -(1 as i32)
            }; /* type == 3, invalid */
            if err != 0 as i32 {
                break;
            }
            if !(last == 0) {
                break;
            }
            /* return with error */
        }
    }
    /* update the lengths and return */
    if err <= 0 as i32 {
        *destlen = s.outcnt;
        *sourcelen = s.incnt
    }
    return err;
}

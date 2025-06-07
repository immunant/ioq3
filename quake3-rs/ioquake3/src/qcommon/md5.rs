use ::libc;

pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

/*
 * This code implements the MD5 message-digest algorithm.
 * The algorithm is due to Ron Rivest.  This code was
 * written by Colin Plumb in 1993, no copyright is claimed.
 * This code is in the public domain; do with it what you wish.
 *
 * Equivalent code is available from RSA Data Security, Inc.
 * This code has been tested against that, and is equivalent,
 * except that you don't need to include two pages of legalese
 * with every copy.
 *
 * To compute the message digest of a chunk of bytes, declare an
 * MD5Context structure, pass it to MD5Init, call MD5Update as
 * needed on buffers full of bytes, and then call MD5Final, which
 * will fill a supplied 16-byte array with the digest.
 */

pub type MD5_CTX = MD5Context;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MD5Context {
    pub buf: [crate::stdlib::uint32_t; 4],
    pub bits: [crate::stdlib::uint32_t; 2],
    pub in_0: [libc::c_uchar; 64],
}
/* Nothing */
// Q3_BIG_ENDIAN
/*
 * Start MD5 accumulation.  Set bit count to 0 and buffer to mysterious
 * initialization constants.
 */

unsafe extern "C" fn MD5Init(mut ctx: *mut MD5Context) {
    (*ctx).buf[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as crate::stdlib::uint32_t;
    (*ctx).buf[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    (*ctx).buf[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    (*ctx).buf[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as crate::stdlib::uint32_t;
    (*ctx).bits[0 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
    (*ctx).bits[1 as libc::c_int as usize] = 0 as libc::c_int as crate::stdlib::uint32_t;
}
/* The four core functions - F1 is optimized somewhat */
/* #define F1(x, y, z) (x & y | ~x & z) */
/* This is the central step in the MD5 algorithm. */
/*
 * The core of the MD5 algorithm, this alters an existing MD5 hash to
 * reflect the addition of 16 longwords of new data.  MD5Update blocks
 * the data and converts bytes into longwords for this routine.
 */

unsafe extern "C" fn MD5Transform(
    mut buf: *mut crate::stdlib::uint32_t,
    mut in_0: *const crate::stdlib::uint32_t,
) {
    let mut a: crate::stdlib::uint32_t = 0;
    let mut b: crate::stdlib::uint32_t = 0;
    let mut c: crate::stdlib::uint32_t = 0;
    let mut d: crate::stdlib::uint32_t = 0;
    a = *buf.offset(0 as libc::c_int as isize);
    b = *buf.offset(1 as libc::c_int as isize);
    c = *buf.offset(2 as libc::c_int as isize);
    d = *buf.offset(3 as libc::c_int as isize);
    a = (a as libc::c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
            .wrapping_add(0xd76aa478 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
            .wrapping_add(0xe8c7b756 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
            .wrapping_add(0x242070db as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
            .wrapping_add(0xc1bdceee as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
            .wrapping_add(0xf57c0faf as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
            .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
            .wrapping_add(0xa8304613 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
            .wrapping_add(0xfd469501 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
            .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
            .wrapping_add(0x8b44f7af as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
            .wrapping_add(0xffff5bb1 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
            .wrapping_add(0x895cd7be as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (d ^ b & (c ^ d))
            .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
            .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (c ^ a & (b ^ c))
            .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
            .wrapping_add(0xfd987193 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (b ^ d & (a ^ b))
            .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
            .wrapping_add(0xa679438e as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (a ^ c & (d ^ a))
            .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
            .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
            .wrapping_add(0xf61e2562 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
            .wrapping_add(0xc040b340 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
            .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
            .wrapping_add(0xe9b6c7aa as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
            .wrapping_add(0xd62f105d as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
            .wrapping_add(0x2441453 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
            .wrapping_add(0xd8a1e681 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
            .wrapping_add(0xe7d3fbc8 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
            .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
            .wrapping_add(0xc33707d6 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
            .wrapping_add(0xf4d50d87 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
            .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ d & (b ^ c))
            .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
            .wrapping_add(0xa9e3e905 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ c & (a ^ b))
            .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
            .wrapping_add(0xfcefa3f8 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ b & (d ^ a))
            .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
            .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ a & (c ^ d))
            .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
            .wrapping_add(0x8d2a4c8a as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
            .wrapping_add(0xfffa3942 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
            .wrapping_add(0x8771f681 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
            .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
            .wrapping_add(0xfde5380c as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
            .wrapping_add(0xa4beea44 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
            .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
            .wrapping_add(0xf6bb4b60 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
            .wrapping_add(0xbebfbc70 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
            .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
            .wrapping_add(0xeaa127fa as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
            .wrapping_add(0xd4ef3085 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
            .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (b ^ c ^ d)
            .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
            .wrapping_add(0xd9d4d039 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (a ^ b ^ c)
            .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
            .wrapping_add(0xe6db99e5 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (d ^ a ^ b)
            .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
            .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (c ^ d ^ a)
            .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
            .wrapping_add(0xc4ac5665 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(0 as libc::c_int as isize))
            .wrapping_add(0xf4292244 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(7 as libc::c_int as isize))
            .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(14 as libc::c_int as isize))
            .wrapping_add(0xab9423a7 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(5 as libc::c_int as isize))
            .wrapping_add(0xfc93a039 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(12 as libc::c_int as isize))
            .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(3 as libc::c_int as isize))
            .wrapping_add(0x8f0ccc92 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(10 as libc::c_int as isize))
            .wrapping_add(0xffeff47d as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(1 as libc::c_int as isize))
            .wrapping_add(0x85845dd1 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(8 as libc::c_int as isize))
            .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(15 as libc::c_int as isize))
            .wrapping_add(0xfe2ce6e0 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(6 as libc::c_int as isize))
            .wrapping_add(0xa3014314 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(13 as libc::c_int as isize))
            .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = (a as libc::c_uint).wrapping_add(
        (c ^ (b | !d))
            .wrapping_add(*in_0.offset(4 as libc::c_int as isize))
            .wrapping_add(0xf7537e82 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = (d as libc::c_uint).wrapping_add(
        (b ^ (a | !c))
            .wrapping_add(*in_0.offset(11 as libc::c_int as isize))
            .wrapping_add(0xbd3af235 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = (c as libc::c_uint).wrapping_add(
        (a ^ (d | !b))
            .wrapping_add(*in_0.offset(2 as libc::c_int as isize))
            .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = (b as libc::c_uint).wrapping_add(
        (d ^ (c | !a))
            .wrapping_add(*in_0.offset(9 as libc::c_int as isize))
            .wrapping_add(0xeb86d391 as libc::c_uint),
    ) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t as crate::stdlib::uint32_t;
    let ref mut fresh0 = *buf.offset(0 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_uint).wrapping_add(a) as crate::stdlib::uint32_t
        as crate::stdlib::uint32_t;
    let ref mut fresh1 = *buf.offset(1 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(b) as crate::stdlib::uint32_t
        as crate::stdlib::uint32_t;
    let ref mut fresh2 = *buf.offset(2 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(c) as crate::stdlib::uint32_t
        as crate::stdlib::uint32_t;
    let ref mut fresh3 = *buf.offset(3 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(d) as crate::stdlib::uint32_t
        as crate::stdlib::uint32_t;
}
/*
 * Update context to reflect the concatenation of another buffer full
 * of bytes.
 */

unsafe extern "C" fn MD5Update(
    mut ctx: *mut MD5Context,
    mut buf: *const libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut t: crate::stdlib::uint32_t = 0;
    /* Update bitcount */
    t = (*ctx).bits[0 as libc::c_int as usize]; /* Carry from low to high */
    (*ctx).bits[0 as libc::c_int as usize] = t.wrapping_add(len << 3 as libc::c_int); /* Bytes already in shsInfo->data */
    if (*ctx).bits[0 as libc::c_int as usize] < t {
        (*ctx).bits[1 as libc::c_int as usize] =
            (*ctx).bits[1 as libc::c_int as usize].wrapping_add(1)
    }
    (*ctx).bits[1 as libc::c_int as usize] =
        ((*ctx).bits[1 as libc::c_int as usize] as libc::c_uint)
            .wrapping_add(len >> 29 as libc::c_int) as crate::stdlib::uint32_t
            as crate::stdlib::uint32_t;
    t = t >> 3 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
    /* Handle any leading odd-sized chunks */
    if t != 0 {
        let mut p: *mut libc::c_uchar = (*ctx).in_0.as_mut_ptr().offset(t as isize);
        t = (64 as libc::c_int as libc::c_uint).wrapping_sub(t);
        if len < t {
            crate::stdlib::memcpy(
                p as *mut libc::c_void,
                buf as *const libc::c_void,
                len as libc::c_ulong,
            );
            return;
        }
        crate::stdlib::memcpy(
            p as *mut libc::c_void,
            buf as *const libc::c_void,
            t as libc::c_ulong,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t
                as *const crate::stdlib::uint32_t,
        );
        buf = buf.offset(t as isize);
        len = len.wrapping_sub(t)
    }
    /* Process data in 64-byte chunks */
    while len >= 64 as libc::c_int as libc::c_uint {
        crate::stdlib::memcpy(
            (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
            buf as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t
                as *const crate::stdlib::uint32_t,
        );
        buf = buf.offset(64 as libc::c_int as isize);
        len = len.wrapping_sub(64 as libc::c_int as libc::c_uint)
    }
    /* Handle any remaining bytes of data. */
    crate::stdlib::memcpy(
        (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
        buf as *const libc::c_void,
        len as libc::c_ulong,
    );
}
/*
 * Final wrapup - pad to 64-byte boundary with the bit pattern
 * 1 0* (64-bit count of bits processed, MSB-first)
 */

unsafe extern "C" fn MD5Final(mut ctx: *mut MD5Context, mut digest: *mut libc::c_uchar) {
    let mut count: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    /* Compute number of bytes mod 64 */
    count = (*ctx).bits[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_uint;
    /* Set the first char of padding to 0x80.  This is safe since there is
    always at least one byte free */
    p = (*ctx).in_0.as_mut_ptr().offset(count as isize);
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = 0x80 as libc::c_int as libc::c_uchar;
    /* Bytes of padding needed to make 64 bytes */
    count = ((64 as libc::c_int - 1 as libc::c_int) as libc::c_uint).wrapping_sub(count);
    /* Pad out to 56 mod 64 */
    if count < 8 as libc::c_int as libc::c_uint {
        /* Two lots of padding:  Pad the first block to 64 bytes */
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0 as libc::c_int,
            count as libc::c_ulong,
        );
        MD5Transform(
            (*ctx).buf.as_mut_ptr(),
            (*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t
                as *const crate::stdlib::uint32_t,
        );
        /* Now fill the next block with 56 bytes */
        crate::stdlib::memset(
            (*ctx).in_0.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            56 as libc::c_int as libc::c_ulong,
        );
    } else {
        /* Pad block to 56 bytes */
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0 as libc::c_int,
            count.wrapping_sub(8 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
    }
    /* Append length in bits and transform */
    *((*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t)
        .offset(14 as libc::c_int as isize) = (*ctx).bits[0 as libc::c_int as usize];
    *((*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t)
        .offset(15 as libc::c_int as isize) = (*ctx).bits[1 as libc::c_int as usize];
    MD5Transform(
        (*ctx).buf.as_mut_ptr(),
        (*ctx).in_0.as_mut_ptr() as *mut crate::stdlib::uint32_t as *const crate::stdlib::uint32_t,
    );
    if !digest.is_null() {
        crate::stdlib::memcpy(
            digest as *mut libc::c_void,
            (*ctx).buf.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
    }
    crate::stdlib::memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<MD5Context>() as libc::c_ulong,
    );
    /* In case it's sensitive */
}
#[no_mangle]

pub unsafe extern "C" fn Com_MD5File(
    mut fn_0: *const libc::c_char,
    mut length: libc::c_int,
    mut prefix: *const libc::c_char,
    mut prefix_len: libc::c_int,
) -> *mut libc::c_char {
    static mut final_0: [libc::c_char; 33] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    let mut digest: [libc::c_uchar; 16] =
        *::std::mem::transmute::<&[u8; 16], &mut [libc::c_uchar; 16]>(
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        );
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut md5: MD5_CTX = MD5_CTX {
        buf: [0; 4],
        bits: [0; 2],
        in_0: [0; 64],
    };
    let mut buffer: [crate::src::qcommon::q_shared::byte; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut filelen: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut total: libc::c_int = 0 as libc::c_int;
    crate::src::qcommon::q_shared::Q_strncpyz(
        final_0.as_mut_ptr(),
        b"\x00" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as libc::c_int,
    );
    filelen = crate::src::qcommon::files::FS_SV_FOpenFileRead(fn_0, &mut f) as libc::c_int;
    if f == 0 {
        return final_0.as_mut_ptr();
    }
    if filelen < 1 as libc::c_int {
        crate::src::qcommon::files::FS_FCloseFile(f);
        return final_0.as_mut_ptr();
    }
    if filelen < length || length == 0 {
        length = filelen
    }
    MD5Init(&mut md5);
    if prefix_len != 0 && *prefix as libc::c_int != 0 {
        MD5Update(
            &mut md5,
            prefix as *mut libc::c_uchar,
            prefix_len as libc::c_uint,
        );
    }
    loop {
        r = crate::src::qcommon::files::FS_Read(
            buffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 2048]>() as libc::c_ulong
                as libc::c_int,
            f,
        );
        if r < 1 as libc::c_int {
            break;
        }
        if r + total > length {
            r = length - total
        }
        total += r;
        MD5Update(&mut md5, buffer.as_mut_ptr(), r as libc::c_uint);
        if (r as libc::c_ulong)
            < ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 2048]>() as libc::c_ulong
            || total >= length
        {
            break;
        }
    }
    crate::src::qcommon::files::FS_FCloseFile(f);
    MD5Final(&mut md5, digest.as_mut_ptr());
    final_0[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        crate::src::qcommon::q_shared::Q_strcat(
            final_0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::q_shared::va(
                b"%02X\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                digest[i as usize] as libc::c_int,
            ),
        );
        i += 1
    }
    return final_0.as_mut_ptr();
}

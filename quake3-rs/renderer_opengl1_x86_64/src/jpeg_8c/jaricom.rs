use ::libc;

/* JPEG_INTERNAL_OPTIONS */

/* FAST_FLOAT should be either float or double, whichever is done faster
 * by your compiler.  (Note that this type is only used in the floating point
 * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
 * Typically, float is faster in ANSI C compilers, while double is faster in
 * pre-ANSI compilers (because they insist on converting to double anyway).
 * The code below therefore chooses float if we have ANSI-style prototypes.
 */

/* type for fastest integer multiply */

/* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
 * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
 * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
 */

/* If your compiler supports inline functions, define INLINE
 * as the inline keyword; otherwise define it as empty.
 */

/* Definitions for speed-related optimizations. */

/* JSAMPLEs per RGB scanline element */

/* Offset of Blue */

/* Offset of Green */

/* Offset of Red in an RGB scanline element */

/*
 * Ordering of RGB data in scanlines passed to or from the application.
 * If your application wants to deal with data in the order B,G,R, just
 * change these macros.  You can also deal with formats such as R,G,B,X
 * (one extra byte per pixel) by changing RGB_PIXELSIZE.  Note that changing
 * the offsets will also change the order in which colormap data is organized.
 * RESTRICTIONS:
 * 1. The sample applications cjpeg,djpeg do NOT support modified RGB formats.
 * 2. These macros only affect RGB<=>YCbCr color conversion, so they are not
 *    useful if you are using JPEG color spaces other than YCbCr or grayscale.
 * 3. The color quantizer modules will not behave desirably if RGB_PIXELSIZE
 *    is not 3 (they don't understand about dummy color components!).  So you
 *    can't use color quantization if you change that value.
 */

/* more capability options later, no doubt */

/* 2-pass color quantization? */

/* 1-pass color quantization? */

/* Fast path for sloppy upsampling? */

/* Output rescaling at upsample stage? */

/* Block smoothing? (Progressive only) */

/* jpeg_save_markers() needed? */

/* Output rescaling via IDCT? */

/* Progressive JPEG? (Requires MULTISCAN)*/

/* Multiple-scan JPEG files? */

/* Arithmetic coding back end? */

/* Decoder capability options: */

/* Input image smoothing option? */

/* Note: if you selected 12-bit data precision, it is dangerous to turn off
 * ENTROPY_OPT_SUPPORTED.  The standard Huffman tables are only good for 8-bit
 * precision, so jchuff.c normally uses entropy optimization to compute
 * usable tables for higher precision.  If you don't want to do optimization,
 * you'll have to supply different default Huffman tables.
 * The exact same statements apply for progressive JPEG: the default tables
 * don't work for progressive mode.  (This may get fixed, however.)
 */

/* Optimization of entropy coding parms? */

/* Input rescaling via DCT? (Requires DCT_ISLOW)*/

/* Progressive JPEG? (Requires MULTISCAN)*/

/* Multiple-scan JPEG files? */

/* Arithmetic coding back end? */

/* Encoder capability options: */

/* floating-point: accurate, fast on fast HW */

/* faster, less accurate integer method */

/* slow but accurate integer algorithm */

/* Capability options common to encoder and decoder: */

/*
 * These defines indicate whether to include various optional functions.
 * Undefining some of these symbols will produce a smaller but less capable
 * library.  Note that you can leave certain source files out of the
 * compilation/linking process if you've #undef'd the corresponding symbols.
 * (You may HAVE to do that if your compiler doesn't like null source files.)
 */

/*
 * The remaining options affect code selection within the JPEG library,
 * but they don't need to be visible to most applications using the library.
 * To minimize application namespace pollution, the symbols won't be
 * defined unless JPEG_INTERNALS or JPEG_INTERNAL_OPTIONS has been defined.
 */

/* values of boolean */

/* in case these macros already exist */
pub use crate::jmorecfg_h::INT32;
/*
 * jaricom.c
 *
 * Developed 1997-2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains probability estimation tables for common use in
 * arithmetic entropy encoding and decoding routines.
 *
 * This data represents Table D.2 in the JPEG spec (ISO/IEC IS 10918-1
 * and CCITT Recommendation ITU-T T.81) and Table 24 in the JBIG spec
 * (ISO/IEC IS 11544 and CCITT Recommendation ITU-T T.82).
 */
/* The following #define specifies the packing of the four components
 * into the compact INT32 representation.
 * Note that this formula must match the actual arithmetic encoder
 * and decoder implementation.  The implementation has to be changed
 * if this formula is changed.
 * The current organization is leaned on Markus Kuhn's JBIG
 * implementation (jbig_tab.c).
 */
#[no_mangle]

pub static mut jpeg_aritab: [crate::jmorecfg_h::INT32; 114] = [
    (0x5a1d as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 1 as i32 as isize,
    (0x2586 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (2 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 14 as i32 as isize,
    (0x1114 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (3 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 16 as i32 as isize,
    (0x80b as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (4 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 18 as i32 as isize,
    (0x3d8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (5 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 20 as i32 as isize,
    (0x1da as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (6 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 23 as i32 as isize,
    (0xe5 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (7 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 25 as i32 as isize,
    (0x6f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (8 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 28 as i32 as isize,
    (0x36 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (9 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 30 as i32 as isize,
    (0x1a as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (10 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 33 as i32 as isize,
    (0xd as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (11 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 35 as i32 as isize,
    (0x6 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (12 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 9 as i32 as isize,
    (0x3 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (13 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 10 as i32 as isize,
    (0x1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (13 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 12 as i32 as isize,
    (0x5a7f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (15 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 15 as i32 as isize,
    (0x3f25 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (16 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 36 as i32 as isize,
    (0x2cf2 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (17 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 38 as i32 as isize,
    (0x207c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (18 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 39 as i32 as isize,
    (0x17b9 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (19 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 40 as i32 as isize,
    (0x1182 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (20 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 42 as i32 as isize,
    (0xcef as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (21 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 43 as i32 as isize,
    (0x9a1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (22 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 45 as i32 as isize,
    (0x72f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (23 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 46 as i32 as isize,
    (0x55c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (24 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 48 as i32 as isize,
    (0x406 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (25 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 49 as i32 as isize,
    (0x303 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (26 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 51 as i32 as isize,
    (0x240 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (27 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 52 as i32 as isize,
    (0x1b1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (28 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 54 as i32 as isize,
    (0x144 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (29 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 56 as i32 as isize,
    (0xf5 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (30 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 57 as i32 as isize,
    (0xb7 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (31 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 59 as i32 as isize,
    (0x8a as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (32 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 60 as i32 as isize,
    (0x68 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (33 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 62 as i32 as isize,
    (0x4e as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (34 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 63 as i32 as isize,
    (0x3b as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (35 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 32 as i32 as isize,
    (0x2c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (9 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 33 as i32 as isize,
    (0x5ae1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (37 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 37 as i32 as isize,
    (0x484c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (38 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 64 as i32 as isize,
    (0x3a0d as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (39 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 65 as i32 as isize,
    (0x2ef1 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (40 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 67 as i32 as isize,
    (0x261f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (41 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 68 as i32 as isize,
    (0x1f33 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (42 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 69 as i32 as isize,
    (0x19a8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (43 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 70 as i32 as isize,
    (0x1518 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (44 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 72 as i32 as isize,
    (0x1177 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (45 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 73 as i32 as isize,
    (0xe74 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (46 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 74 as i32 as isize,
    (0xbfb as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (47 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 75 as i32 as isize,
    (0x9f8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (48 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 77 as i32 as isize,
    (0x861 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (49 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 78 as i32 as isize,
    (0x706 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (50 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 79 as i32 as isize,
    (0x5cd as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (51 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 48 as i32 as isize,
    (0x4de as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (52 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 50 as i32 as isize,
    (0x40f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (53 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 50 as i32 as isize,
    (0x363 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (54 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 51 as i32 as isize,
    (0x2d4 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (55 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 52 as i32 as isize,
    (0x25c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (56 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 53 as i32 as isize,
    (0x1f8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (57 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 54 as i32 as isize,
    (0x1a4 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (58 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 55 as i32 as isize,
    (0x160 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (59 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 56 as i32 as isize,
    (0x125 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (60 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 57 as i32 as isize,
    (0xf6 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (61 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 58 as i32 as isize,
    (0xcb as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (62 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 59 as i32 as isize,
    (0xab as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (63 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 61 as i32 as isize,
    (0x8f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (32 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 61 as i32 as isize,
    (0x5b12 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (65 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 65 as i32 as isize,
    (0x4d04 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (66 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 80 as i32 as isize,
    (0x412c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (67 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 81 as i32 as isize,
    (0x37d8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (68 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 82 as i32 as isize,
    (0x2fe8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (69 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 83 as i32 as isize,
    (0x293c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (70 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 84 as i32 as isize,
    (0x2379 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (71 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 86 as i32 as isize,
    (0x1edf as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (72 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 87 as i32 as isize,
    (0x1aa9 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (73 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 87 as i32 as isize,
    (0x174e as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (74 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 72 as i32 as isize,
    (0x1424 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (75 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 72 as i32 as isize,
    (0x119c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (76 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 74 as i32 as isize,
    (0xf6b as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (77 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 74 as i32 as isize,
    (0xd51 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (78 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 75 as i32 as isize,
    (0xbb6 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (79 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 77 as i32 as isize,
    (0xa40 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (48 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 77 as i32 as isize,
    (0x5832 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (81 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 80 as i32 as isize,
    (0x4d1c as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (82 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 88 as i32 as isize,
    (0x438e as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (83 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 89 as i32 as isize,
    (0x3bdd as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (84 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 90 as i32 as isize,
    (0x34ee as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (85 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 91 as i32 as isize,
    (0x2eae as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (86 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 92 as i32 as isize,
    (0x299a as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (87 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 93 as i32 as isize,
    (0x2516 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (71 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 86 as i32 as isize,
    (0x5570 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (89 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 88 as i32 as isize,
    (0x4ca9 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (90 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 95 as i32 as isize,
    (0x44d9 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (91 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 96 as i32 as isize,
    (0x3e22 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (92 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 97 as i32 as isize,
    (0x3824 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (93 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 99 as i32 as isize,
    (0x32b4 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (94 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 99 as i32 as isize,
    (0x2e17 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (86 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 93 as i32 as isize,
    (0x56a8 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (96 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 95 as i32 as isize,
    (0x4f46 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (97 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 101 as i32 as isize,
    (0x47e5 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (98 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 102 as i32 as isize,
    (0x41cf as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (99 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 103 as i32 as isize,
    (0x3c3d as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (100 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 104 as i32 as isize,
    (0x375e as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (93 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 99 as i32 as isize,
    (0x5231 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (102 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 105 as i32 as isize,
    (0x4c0f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (103 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 106 as i32 as isize,
    (0x4639 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (104 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 107 as i32 as isize,
    (0x415e as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (99 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 103 as i32 as isize,
    (0x5627 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (106 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 105 as i32 as isize,
    (0x50e7 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (107 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 108 as i32 as isize,
    (0x4b85 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (103 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 109 as i32 as isize,
    (0x5597 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (109 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 110 as i32 as isize,
    (0x504f as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (107 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 111 as i32 as isize,
    (0x5a10 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (111 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 110 as i32 as isize,
    (0x5522 as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (109 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 112 as i32 as isize,
    (0x59eb as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (111 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (1 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 112 as i32 as isize,
    (0x5a1d as i32 as crate::jmorecfg_h::INT32) << 16 as i32
        | (113 as i32 as crate::jmorecfg_h::INT32) << 8 as i32
        | (0 as i32 as crate::jmorecfg_h::INT32) << 7 as i32
        | 113 as i32 as isize,
];

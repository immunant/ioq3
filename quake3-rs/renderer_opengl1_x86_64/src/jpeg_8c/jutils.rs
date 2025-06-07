use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;

/*
 * jutils.c
 *
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * Modified 2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains tables and miscellaneous utility routines needed
 * for both compression and decompression.
 * Note we prefix all global names with "j" to minimize conflicts with
 * a surrounding application.
 */
/*
 * jpeg_zigzag_order[i] is the zigzag-order position of the i'th element
 * of a DCT block read in natural order (left to right, top to bottom).
 */
/* This table is not actually needed in v6a */
/*
 * jpeg_natural_order[i] is the natural-order position of the i'th element
 * of zigzag order.
 *
 * When reading corrupted data, the Huffman decoders could attempt
 * to reference an entry beyond the end of this array (if the decoded
 * zero run length reaches past the end of the block).  To prevent
 * wild stores without adding an inner-loop test, we put some extra
 * "63"s after the real entries.  This will cause the extra coefficient
 * to be stored in location 63 of the block, not somewhere random.
 * The worst case would be a run-length of 15, which means we need 16
 * fake entries.
 */
#[no_mangle]

pub static mut jpeg_natural_order: [libc::c_int; 80] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    24 as libc::c_int,
    32 as libc::c_int,
    25 as libc::c_int,
    18 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
    33 as libc::c_int,
    40 as libc::c_int,
    48 as libc::c_int,
    41 as libc::c_int,
    34 as libc::c_int,
    27 as libc::c_int,
    20 as libc::c_int,
    13 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    14 as libc::c_int,
    21 as libc::c_int,
    28 as libc::c_int,
    35 as libc::c_int,
    42 as libc::c_int,
    49 as libc::c_int,
    56 as libc::c_int,
    57 as libc::c_int,
    50 as libc::c_int,
    43 as libc::c_int,
    36 as libc::c_int,
    29 as libc::c_int,
    22 as libc::c_int,
    15 as libc::c_int,
    23 as libc::c_int,
    30 as libc::c_int,
    37 as libc::c_int,
    44 as libc::c_int,
    51 as libc::c_int,
    58 as libc::c_int,
    59 as libc::c_int,
    52 as libc::c_int,
    45 as libc::c_int,
    38 as libc::c_int,
    31 as libc::c_int,
    39 as libc::c_int,
    46 as libc::c_int,
    53 as libc::c_int,
    60 as libc::c_int,
    61 as libc::c_int,
    54 as libc::c_int,
    47 as libc::c_int,
    55 as libc::c_int,
    62 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order7: [libc::c_int; 65] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    24 as libc::c_int,
    32 as libc::c_int,
    25 as libc::c_int,
    18 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
    33 as libc::c_int,
    40 as libc::c_int,
    48 as libc::c_int,
    41 as libc::c_int,
    34 as libc::c_int,
    27 as libc::c_int,
    20 as libc::c_int,
    13 as libc::c_int,
    6 as libc::c_int,
    14 as libc::c_int,
    21 as libc::c_int,
    28 as libc::c_int,
    35 as libc::c_int,
    42 as libc::c_int,
    49 as libc::c_int,
    50 as libc::c_int,
    43 as libc::c_int,
    36 as libc::c_int,
    29 as libc::c_int,
    22 as libc::c_int,
    30 as libc::c_int,
    37 as libc::c_int,
    44 as libc::c_int,
    51 as libc::c_int,
    52 as libc::c_int,
    45 as libc::c_int,
    38 as libc::c_int,
    46 as libc::c_int,
    53 as libc::c_int,
    54 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order6: [libc::c_int; 52] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    24 as libc::c_int,
    32 as libc::c_int,
    25 as libc::c_int,
    18 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
    33 as libc::c_int,
    40 as libc::c_int,
    41 as libc::c_int,
    34 as libc::c_int,
    27 as libc::c_int,
    20 as libc::c_int,
    13 as libc::c_int,
    21 as libc::c_int,
    28 as libc::c_int,
    35 as libc::c_int,
    42 as libc::c_int,
    43 as libc::c_int,
    36 as libc::c_int,
    29 as libc::c_int,
    37 as libc::c_int,
    44 as libc::c_int,
    45 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order5: [libc::c_int; 41] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    24 as libc::c_int,
    32 as libc::c_int,
    25 as libc::c_int,
    18 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    12 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
    33 as libc::c_int,
    34 as libc::c_int,
    27 as libc::c_int,
    20 as libc::c_int,
    28 as libc::c_int,
    35 as libc::c_int,
    36 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order4: [libc::c_int; 32] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    24 as libc::c_int,
    25 as libc::c_int,
    18 as libc::c_int,
    11 as libc::c_int,
    19 as libc::c_int,
    26 as libc::c_int,
    27 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order3: [libc::c_int; 25] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    16 as libc::c_int,
    9 as libc::c_int,
    2 as libc::c_int,
    10 as libc::c_int,
    17 as libc::c_int,
    18 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
#[no_mangle]

pub static mut jpeg_natural_order2: [libc::c_int; 20] = [
    0 as libc::c_int,
    1 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
    63 as libc::c_int,
];
/* Utility routines in jutils.c */
/*
 * Arithmetic utilities
 */
#[no_mangle]

pub unsafe extern "C" fn jdiv_round_up(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long
/* Compute a/b rounded up to next integer, ie, ceil(a/b) */
/* Assumes a >= 0, b > 0 */ {
    return (a + b - 1 as libc::c_long) / b;
}
#[no_mangle]

pub unsafe extern "C" fn jround_up(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long
/* Compute a rounded up to next multiple of b, ie, ceil(a/b)*b */
/* Assumes a >= 0, b > 0 */ {
    a += b - 1 as libc::c_long;
    return a - a % b;
}
/* On normal machines we can apply MEMCOPY() and MEMZERO() to sample arrays
 * and coefficient-block arrays.  This won't work on 80x86 because the arrays
 * are FAR and we're assuming a small-pointer memory model.  However, some
 * DOS compilers provide far-pointer versions of memcpy() and memset() even
 * in the small-model libraries.  These will be used if USE_FMEM is defined.
 * Otherwise, the routines below do it the hard way.  (The performance cost
 * is not all that great, because these routines aren't very heavily used.)
 */
/* normal case, same as regular macros */
/* 80x86 case, define if we can */
#[no_mangle]

pub unsafe extern "C" fn jcopy_sample_rows(
    mut input_array: crate::jpeglib_h::JSAMPARRAY,
    mut source_row: libc::c_int,
    mut output_array: crate::jpeglib_h::JSAMPARRAY,
    mut dest_row: libc::c_int,
    mut num_rows: libc::c_int,
    mut num_cols: crate::jmorecfg_h::JDIMENSION,
)
/* Copy some rows of samples from one place to another.
 * num_rows rows are copied from input_array[source_row++]
 * to output_array[dest_row++]; these areas may overlap for duplication.
 * The source and destination arrays must be at least as wide as num_cols.
 */
{
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut count: crate::stddef_h::size_t = (num_cols as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong);
    let mut row: libc::c_int = 0;
    input_array = input_array.offset(source_row as isize);
    output_array = output_array.offset(dest_row as isize);
    row = num_rows;
    while row > 0 as libc::c_int {
        let fresh0 = input_array;
        input_array = input_array.offset(1);
        inptr = *fresh0;
        let fresh1 = output_array;
        output_array = output_array.offset(1);
        outptr = *fresh1;
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            count,
        );
        row -= 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn jcopy_block_row(
    mut input_row: crate::jpeglib_h::JBLOCKROW,
    mut output_row: crate::jpeglib_h::JBLOCKROW,
    mut num_blocks: crate::jmorecfg_h::JDIMENSION,
)
/* Copy a row of coefficient blocks from one place to another. */
{
    crate::stdlib::memcpy(
        output_row as *mut libc::c_void,
        input_row as *const libc::c_void,
        (num_blocks as libc::c_ulong).wrapping_mul(
            (64 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JCOEF>() as libc::c_ulong),
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn jzero_far(
    mut target: *mut libc::c_void,
    mut bytestozero: crate::stddef_h::size_t,
)
/* Zero out a chunk of FAR memory. */
/* This might be sample-array data, block-array data, or alloc_large data. */
{
    crate::stdlib::memset(target, 0 as libc::c_int, bytestozero);
}

pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
/*
 * jfdctflt.c
 *
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2003-2009 by Guido Vollbeding.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains a floating-point implementation of the
 * forward DCT (Discrete Cosine Transform).
 *
 * This implementation should be more accurate than either of the integer
 * DCT implementations.  However, it may not give the same results on all
 * machines because of differences in roundoff behavior.  Speed will depend
 * on the hardware's floating point capacity.
 *
 * A 2-D DCT can be done by 1-D DCT on each row followed by 1-D DCT
 * on each column.  Direct algorithms are also available, but they are
 * much more complex and seem not to be any faster when reduced to code.
 *
 * This implementation is based on Arai, Agui, and Nakajima's algorithm for
 * scaled DCT.  Their original paper (Trans. IEICE E-71(11):1095) is in
 * Japanese, but the algorithm is described in the Pennebaker & Mitchell
 * JPEG textbook (see REFERENCES section in file README).  The following code
 * is based directly on figure 4-8 in P&M.
 * While an 8-point DCT cannot be done in less than 11 multiplies, it is
 * possible to arrange the computation so that many of the multiplies are
 * simple scalings of the final outputs.  These multiplies can then be
 * folded into the multiplications or divisions by the JPEG quantization
 * table entries.  The AA&N method leaves only 5 multiplies and 29 adds
 * to be done in the DCT itself.
 * The primary disadvantage of this method is that with a fixed-point
 * implementation, accuracy is lost due to imprecise representation of the
 * scaled quantization values.  However, that problem does not arise if
 * we use floating point arithmetic.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/*
 * Perform the forward DCT on one block of samples.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_float(
    mut data: *mut f32,
    mut sample_data: JSAMPARRAY,
    mut start_col: JDIMENSION,
) {
    let mut tmp0: f32 = 0.;
    let mut tmp1: f32 = 0.;
    let mut tmp2: f32 = 0.;
    let mut tmp3: f32 = 0.;
    let mut tmp4: f32 = 0.;
    let mut tmp5: f32 = 0.;
    let mut tmp6: f32 = 0.;
    let mut tmp7: f32 = 0.;
    let mut tmp10: f32 = 0.;
    let mut tmp11: f32 = 0.;
    let mut tmp12: f32 = 0.;
    let mut tmp13: f32 = 0.;
    let mut z1: f32 = 0.;
    let mut z2: f32 = 0.;
    let mut z3: f32 = 0.;
    let mut z4: f32 = 0.;
    let mut z5: f32 = 0.;
    let mut z11: f32 = 0.;
    let mut z13: f32 = 0.;
    let mut dataptr: *mut f32 = 0 as *mut f32;
    let mut elemptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 8 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as f32;
        tmp7 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as f32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as f32;
        tmp6 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as f32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as f32;
        tmp5 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as f32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as f32;
        tmp4 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as f32;
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset(0 as i32 as isize) = tmp10 + tmp11 - (8 as i32 * 128 as i32) as f32;
        *dataptr.offset(4 as i32 as isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as f32;
        *dataptr.offset(2 as i32 as isize) = tmp13 + z1;
        *dataptr.offset(6 as i32 as isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as f32;
        z2 = 0.541196100f64 as f32 * tmp10 + z5;
        z4 = 1.306562965f64 as f32 * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as f32;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset(5 as i32 as isize) = z13 + z2;
        *dataptr.offset(3 as i32 as isize) = z13 - z2;
        *dataptr.offset(1 as i32 as isize) = z11 + z4;
        *dataptr.offset(7 as i32 as isize) = z11 - z4;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Load data into workspace */
    /* Even part */
    /* phase 2 */
    /* Apply unsigned->signed conversion */
    /* phase 3 */
    /* c4 */
    /* phase 5 */
    /* Odd part */
    /* phase 2 */
    /* The rotator is modified from fig 4-8 to avoid extra negations. */
    /* c6 */
    /* c2-c6 */
    /* c2+c6 */
    /* c4 */
    /* phase 5 */
    /* phase 6 */
    /* Pass 2: process columns. */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        tmp0 = *dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize);
        tmp7 = *dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize);
        tmp1 = *dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize);
        tmp6 = *dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize);
        tmp2 = *dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize);
        tmp5 = *dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize);
        tmp3 = *dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize);
        tmp4 = *dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize);
        /* advance pointer to next column */
        tmp10 = tmp0 + tmp3;
        tmp13 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp1 - tmp2;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = tmp10 + tmp11;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = tmp10 - tmp11;
        z1 = (tmp12 + tmp13) * 0.707106781f64 as f32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = tmp13 + z1;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = tmp13 - z1;
        tmp10 = tmp4 + tmp5;
        tmp11 = tmp5 + tmp6;
        tmp12 = tmp6 + tmp7;
        z5 = (tmp10 - tmp12) * 0.382683433f64 as f32;
        z2 = 0.541196100f64 as f32 * tmp10 + z5;
        z4 = 1.306562965f64 as f32 * tmp12 + z5;
        z3 = tmp11 * 0.707106781f64 as f32;
        z11 = tmp7 + z3;
        z13 = tmp7 - z3;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = z13 + z2;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = z13 - z2;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = z11 + z4;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = z11 - z4;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Even part */
/* phase 2 */
/* phase 3 */
/* c4 */
/* phase 5 */
/* Odd part */
/* phase 2 */
/* The rotator is modified from fig 4-8 to avoid extra negations. */
/* c6 */
/* c2-c6 */
/* c2+c6 */
/* c4 */
/* phase 5 */
/* phase 6 */
/* DCT_FLOAT_SUPPORTED */

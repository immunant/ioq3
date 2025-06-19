pub use crate::stddef_h::size_t;

pub use crate::jdct_h::DCTELEM;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT32;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_0;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
/*
 * jidctfst.c
 *
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * This file is part of the Independent JPEG Group's software.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains a fast, not so accurate integer implementation of the
 * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
 * must also perform dequantization of the input coefficients.
 *
 * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
 * on each row (or vice versa, but it's more convenient to emit a row at
 * a time).  Direct algorithms are also available, but they are much more
 * complex and seem not to be any faster when reduced to code.
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
 * The primary disadvantage of this method is that with fixed-point math,
 * accuracy is lost due to imprecise representation of the scaled
 * quantization values.  The smaller the quantization table entry, the less
 * precise the scaled value, so this implementation does worse with high-
 * quality-setting files than with low-quality ones.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/* Scaling decisions are generally the same as in the LL&M algorithm;
 * see jidctint.c for more details.  However, we choose to descale
 * (right shift) multiplication products as soon as they are formed,
 * rather than carrying additional fractional bits into subsequent additions.
 * This compromises accuracy slightly, but it lets us save a few shifts.
 * More importantly, 16-bit arithmetic is then adequate (for 8-bit samples)
 * everywhere except in the multiplications proper; this saves a good deal
 * of work on 16-bit-int machines.
 *
 * The dequantized coefficients are not integers because the AA&N scaling
 * factors have been incorporated.  We represent them scaled up by PASS1_BITS,
 * so that the first and second IDCT rounds have the same input scaling.
 * For 8-bit JSAMPLEs, we choose IFAST_SCALE_BITS = PASS1_BITS so as to
 * avoid a descaling shift; this compromises accuracy rather drastically
 * for small quantization table entries, but it saves a lot of shifts.
 * For 12-bit JSAMPLEs, there's no hope of using 16x16 multiplies anyway,
 * so we use a much larger scaling factor to preserve accuracy.
 *
 * A final compromise is to represent the multiplicative constants to only
 * 8 fractional bits, rather than 13.  This saves some shifting work on some
 * machines, and may also reduce the cost of multiplication (since there
 * are fewer one-bits in the constants).
 */
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(1.082392200) */
/* FIX(1.414213562) */
/* FIX(1.847759065) */
/* FIX(2.613125930) */
/* We can gain a little more speed, with a further compromise in accuracy,
 * by omitting the addition in a descaling shift.  This yields an incorrectly
 * rounded result half the time...
 */
/* Multiply a DCTELEM variable by an INT32 constant, and immediately
 * descale to yield a DCTELEM result.
 */
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce a DCTELEM result.  For 8-bit data a 16x16->16
 * multiplication will do.  For 12-bit data, the multiplier table is
 * declared INT32, so a 32-bit multiply will be used.
 */
/* Like DESCALE, but applies to a DCTELEM and produces an int.
 * We assume that int right shift is unsigned if INT32 right shift is.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_idct_ifast(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: DCTELEM = 0; /* buffers data between passes */
    let mut tmp1: DCTELEM = 0;
    let mut tmp2: DCTELEM = 0;
    let mut tmp3: DCTELEM = 0;
    let mut tmp4: DCTELEM = 0;
    let mut tmp5: DCTELEM = 0;
    let mut tmp6: DCTELEM = 0;
    let mut tmp7: DCTELEM = 0;
    let mut tmp10: DCTELEM = 0;
    let mut tmp11: DCTELEM = 0;
    let mut tmp12: DCTELEM = 0;
    let mut tmp13: DCTELEM = 0;
    let mut z5: DCTELEM = 0;
    let mut z10: DCTELEM = 0;
    let mut z11: DCTELEM = 0;
    let mut z12: DCTELEM = 0;
    let mut z13: DCTELEM = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut IFAST_MULT_TYPE =
        0 as *mut IFAST_MULT_TYPE;
    let mut wsptr: *mut i32 = 0 as *mut i32;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE =
        (*cinfo).sample_range_limit.offset(128 as i32 as isize);
    let mut ctr: i32 = 0;
    let mut workspace: [i32; 64] = [0; 64];
    /* for DESCALE */
    /* for IDESCALE */
    /* Pass 1: process columns from input, store into work array. */
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut IFAST_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32;
    while ctr > 0 as i32 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((8 as i32 * 1 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 2 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 3 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 4 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 5 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 6 as i32) as isize) as i32 == 0 as i32
            && *inptr.offset((8 as i32 * 7 as i32) as isize) as i32 == 0 as i32
        {
            /* AC terms all zero */
            let mut dcval: i32 = *inptr.offset((8 as i32 * 0 as i32) as isize)
                as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 0 as i32) as isize); /* advance pointers to next column */
            *wsptr.offset((8 as i32 * 0 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 1 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 2 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 3 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 4 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 5 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 6 as i32) as isize) = dcval;
            *wsptr.offset((8 as i32 * 7 as i32) as isize) = dcval;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        } else {
            /* Even part */
            tmp0 = *inptr.offset((8 as i32 * 0 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 0 as i32) as isize); /* phase 3 */
            tmp1 = *inptr.offset((8 as i32 * 2 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 2 as i32) as isize); /* phases 5-3 */
            tmp2 = *inptr.offset((8 as i32 * 4 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 4 as i32) as isize); /* 2*c4 */
            tmp3 = *inptr.offset((8 as i32 * 6 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 6 as i32) as isize); /* phase 2 */
            tmp10 = tmp0 + tmp2;
            tmp11 = tmp0 - tmp2;
            tmp13 = tmp1 + tmp3;
            tmp12 = ((tmp1 - tmp3) as isize * 362 as i32 as INT32 >> 8 as i32)
                as DCTELEM
                - tmp13;
            tmp0 = tmp10 + tmp13;
            tmp3 = tmp10 - tmp13;
            tmp1 = tmp11 + tmp12;
            tmp2 = tmp11 - tmp12;
            /* Odd part */
            tmp4 = *inptr.offset((8 as i32 * 1 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 1 as i32) as isize); /* phase 6 */
            tmp5 = *inptr.offset((8 as i32 * 3 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 3 as i32) as isize); /* phase 5 */
            tmp6 = *inptr.offset((8 as i32 * 5 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 5 as i32) as isize); /* 2*c4 */
            tmp7 = *inptr.offset((8 as i32 * 7 as i32) as isize) as IFAST_MULT_TYPE
                * *quantptr.offset((8 as i32 * 7 as i32) as isize); /* 2*c2 */
            z13 = tmp6 + tmp5; /* 2*(c2-c6) */
            z10 = tmp6 - tmp5; /* -2*(c2+c6) */
            z11 = tmp4 + tmp7; /* phase 2 */
            z12 = tmp4 - tmp7; /* advance pointers to next column */
            tmp7 = z11 + z13;
            tmp11 = ((z11 - z13) as isize * 362 as i32 as INT32 >> 8 as i32)
                as DCTELEM;
            z5 = ((z10 + z12) as isize * 473 as i32 as INT32 >> 8 as i32)
                as DCTELEM;
            tmp10 = (z12 as isize * 277 as i32 as INT32 >> 8 as i32)
                as DCTELEM
                - z5;
            tmp12 = (z10 as isize * -(669 as i32 as INT32) >> 8 as i32)
                as DCTELEM
                + z5;
            tmp6 = tmp12 - tmp7;
            tmp5 = tmp11 - tmp6;
            tmp4 = tmp10 + tmp5;
            *wsptr.offset((8 as i32 * 0 as i32) as isize) = tmp0 + tmp7;
            *wsptr.offset((8 as i32 * 7 as i32) as isize) = tmp0 - tmp7;
            *wsptr.offset((8 as i32 * 1 as i32) as isize) = tmp1 + tmp6;
            *wsptr.offset((8 as i32 * 6 as i32) as isize) = tmp1 - tmp6;
            *wsptr.offset((8 as i32 * 2 as i32) as isize) = tmp2 + tmp5;
            *wsptr.offset((8 as i32 * 5 as i32) as isize) = tmp2 - tmp5;
            *wsptr.offset((8 as i32 * 4 as i32) as isize) = tmp3 + tmp4;
            *wsptr.offset((8 as i32 * 3 as i32) as isize) = tmp3 - tmp4;
            inptr = inptr.offset(1);
            quantptr = quantptr.offset(1);
            wsptr = wsptr.offset(1)
        }
        ctr -= 1
    }
    /* Pass 2: process rows from work array, store into output array. */
    /* Note that we must descale the results by a factor of 8 == 2**3, */
    /* and also undo the PASS1_BITS scaling. */
    wsptr = workspace.as_mut_ptr();
    ctr = 0 as i32;
    while ctr < 8 as i32 {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* advance pointer to next row */
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */
        if *wsptr.offset(1 as i32 as isize) == 0 as i32
            && *wsptr.offset(2 as i32 as isize) == 0 as i32
            && *wsptr.offset(3 as i32 as isize) == 0 as i32
            && *wsptr.offset(4 as i32 as isize) == 0 as i32
            && *wsptr.offset(5 as i32 as isize) == 0 as i32
            && *wsptr.offset(6 as i32 as isize) == 0 as i32
            && *wsptr.offset(7 as i32 as isize) == 0 as i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE = *range_limit.offset(
                (*wsptr.offset(0 as i32 as isize) >> 2 as i32 + 3 as i32
                    & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            ); /* advance pointer to next row */
            *outptr.offset(0 as i32 as isize) = dcval_0;
            *outptr.offset(1 as i32 as isize) = dcval_0;
            *outptr.offset(2 as i32 as isize) = dcval_0;
            *outptr.offset(3 as i32 as isize) = dcval_0;
            *outptr.offset(4 as i32 as isize) = dcval_0;
            *outptr.offset(5 as i32 as isize) = dcval_0;
            *outptr.offset(6 as i32 as isize) = dcval_0;
            *outptr.offset(7 as i32 as isize) = dcval_0;
            wsptr = wsptr.offset(8 as i32 as isize)
        } else {
            /* Even part */
            tmp10 = *wsptr.offset(0 as i32 as isize) + *wsptr.offset(4 as i32 as isize);
            tmp11 = *wsptr.offset(0 as i32 as isize) - *wsptr.offset(4 as i32 as isize);
            tmp13 = *wsptr.offset(2 as i32 as isize) + *wsptr.offset(6 as i32 as isize);
            tmp12 = ((*wsptr.offset(2 as i32 as isize) - *wsptr.offset(6 as i32 as isize)) as isize
                * 362 as i32 as INT32
                >> 8 as i32) as DCTELEM
                - tmp13;
            tmp0 = tmp10 + tmp13;
            tmp3 = tmp10 - tmp13;
            tmp1 = tmp11 + tmp12;
            tmp2 = tmp11 - tmp12;
            /* Odd part */
            z13 = *wsptr.offset(5 as i32 as isize) + *wsptr.offset(3 as i32 as isize); /* phase 5 */
            z10 = *wsptr.offset(5 as i32 as isize) - *wsptr.offset(3 as i32 as isize); /* 2*c4 */
            z11 = *wsptr.offset(1 as i32 as isize) + *wsptr.offset(7 as i32 as isize); /* 2*c2 */
            z12 = *wsptr.offset(1 as i32 as isize) - *wsptr.offset(7 as i32 as isize); /* 2*(c2-c6) */
            tmp7 = z11 + z13; /* -2*(c2+c6) */
            tmp11 = ((z11 - z13) as isize * 362 as i32 as INT32 >> 8 as i32)
                as DCTELEM; /* phase 2 */
            z5 = ((z10 + z12) as isize * 473 as i32 as INT32 >> 8 as i32)
                as DCTELEM;
            tmp10 = (z12 as isize * 277 as i32 as INT32 >> 8 as i32)
                as DCTELEM
                - z5;
            tmp12 = (z10 as isize * -(669 as i32 as INT32) >> 8 as i32)
                as DCTELEM
                + z5;
            tmp6 = tmp12 - tmp7;
            tmp5 = tmp11 - tmp6;
            tmp4 = tmp10 + tmp5;
            /* Final output stage: scale down by a factor of 8 and range-limit */
            *outptr.offset(0 as i32 as isize) = *range_limit.offset(
                (tmp0 + tmp7 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(7 as i32 as isize) = *range_limit.offset(
                (tmp0 - tmp7 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(1 as i32 as isize) = *range_limit.offset(
                (tmp1 + tmp6 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(6 as i32 as isize) = *range_limit.offset(
                (tmp1 - tmp6 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(2 as i32 as isize) = *range_limit.offset(
                (tmp2 + tmp5 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(5 as i32 as isize) = *range_limit.offset(
                (tmp2 - tmp5 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(4 as i32 as isize) = *range_limit.offset(
                (tmp3 + tmp4 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            *outptr.offset(3 as i32 as isize) = *range_limit.offset(
                (tmp3 - tmp4 >> 2 as i32 + 3 as i32 & 255 as i32 * 4 as i32 + 3 as i32) as isize,
            );
            wsptr = wsptr.offset(8 as i32 as isize)
        }
        ctr += 1
    }
}
/* DCT_IFAST_SUPPORTED */

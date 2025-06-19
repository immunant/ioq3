use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::jdct_h::DCTELEM;
pub use crate::jmorecfg_h::INT32;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;

/* preferred floating type */
/*
 * Each IDCT routine is responsible for range-limiting its results and
 * converting them to unsigned form (0..MAXJSAMPLE).  The raw outputs could
 * be quite far out of range if the input data is corrupt, so a bulletproof
 * range-limiting step is required.  We use a mask-and-table-lookup method
 * to do the combined operations quickly.  See the comments with
 * prepare_range_limit_table (in jdmaster.c) for more info.
 */
/* 2 bits wider than legal samples */
/* Short forms of external names for systems with brain-damaged linkers. */
/* NEED_SHORT_EXTERNAL_NAMES */
/* Extern declarations for the forward and inverse DCT routines. */
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(0.298631336) */
/* FIX(0.390180644) */
/* FIX(0.541196100) */
/* FIX(0.765366865) */
/* FIX(0.899976223) */
/* FIX(1.175875602) */
/* FIX(1.501321110) */
/* FIX(1.847759065) */
/* FIX(1.961570560) */
/* FIX(2.053119869) */
/* FIX(2.562915447) */
/* FIX(3.072711026) */
/* Multiply an INT32 variable by an INT32 constant to yield an INT32 result.
 * For 8-bit samples with the recommended scaling, all the variable
 * and constant values involved are no more than 16 bits wide, so a
 * 16x16->32 bit multiply can be used instead of a full 32x32 multiply.
 * For 12-bit samples, a full 32-bit multiplication will be needed.
 */
/*
 * Perform the forward DCT on one block of samples.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_islow(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 8 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp3;
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11
            - (8 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = (tmp10 - tmp11 << 2 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32;
        *dataptr.offset(2 as i32 as isize) = (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp1 + tmp3;
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32;
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32;
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32;
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32;
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32);
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + tmp10 + tmp12 >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp1 + tmp11 + tmp13 >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp2 + tmp11 + tmp12 >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp3 + tmp10 + tmp13 >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part per LL&M figure 1 --- note that published figure is faulty;
     * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
     */
    /* Apply unsigned->signed conversion */
    /* Add fudge factor here for final descale. */
    /* Odd part per figure 8 --- note paper omits factor of sqrt(2).
     * cK represents sqrt(2) * cos(K*pi/16).
     * i0..i3 in the paper are tmp0..tmp3 here.
     */
    /*  c3 */
    /* Add fudge factor here for final descale. */
    /*  c1+c3-c5-c7 */
    /*  c1+c3+c5-c7 */
    /*  c1+c3-c5+c7 */
    /* -c1+c3+c5-c7 */
    /*  c7-c3 */
    /* -c1-c3 */
    /*  c5-c3 */
    /* -c3-c5 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part per LL&M figure 1 --- note that published figure is faulty;
         * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
         */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        /* advance pointer to next column */
        tmp10 = tmp0 + tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 - 1 as i32);
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            (tmp10 + tmp11 >> 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) =
            (tmp10 - tmp11 >> 2 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp1 + tmp3;
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32;
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32;
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32;
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32);
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + tmp10 + tmp12 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + tmp11 + tmp13 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + tmp11 + tmp12 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp3 + tmp10 + tmp13 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Add fudge factor here for final descale. */
/* Add fudge factor here for final descale. */
/* Odd part per figure 8 --- note paper omits factor of sqrt(2).
 * cK represents sqrt(2) * cos(K*pi/16).
 * i0..i3 in the paper are tmp0..tmp3 here.
 */
/*  c3 */
/* Add fudge factor here for final descale. */
/*  c1+c3-c5-c7 */
/*  c1+c3+c5-c7 */
/*  c1+c3-c5+c7 */
/* -c1+c3+c5-c7 */
/*  c7-c3 */
/* -c1-c3 */
/*  c5-c3 */
/* -c3-c5 */
/*
 * Perform the forward DCT on a 7x7 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_7x7(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* cK represents sqrt(2) * cos(K*pi/14). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 7 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = *elemptr.offset(3 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2;
        *dataptr.offset(0 as i32 as isize) = ((z1 + tmp1 + tmp3
            - (7 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        tmp3 += tmp3;
        z1 -= tmp3;
        z1 -= tmp3;
        z1 = z1
            * (0.353553391f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp0 - tmp2)
            * (0.920609002f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z3 = (tmp1 - tmp2)
            * (0.314692123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) =
            (z1 + z2
                + z3
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        z1 -= z2;
        z2 = (tmp0 - tmp1)
            * (0.881747734f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(4 as i32 as isize) = (z2 + z3
            - (tmp1 - tmp3)
                * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) =
            (z1 + z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        tmp1 = (tmp10 + tmp11)
            * (0.935414347f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 = (tmp10 - tmp11)
            * (0.170262339f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (tmp11 + tmp12)
            * -((1.378756276f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp1 += tmp2;
        tmp3 = (tmp10 + tmp12)
            * (0.613604268f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 += tmp3;
        tmp2 += tmp3
            + tmp12
                * (1.870828693f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* (c2+c6-c4)/2 */
    /* (c2+c4-c6)/2 */
    /* c6 */
    /* c4 */
    /* Odd part */
    /* (c3+c1-c5)/2 */
    /* (c3+c5-c1)/2 */
    /* -c1 */
    /* c5 */
    /* c3+c1-c5 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/7)**2 = 64/49, which we fold
     * into the constant multipliers:
     * cK now represents sqrt(2) * cos(K*pi/14) * 64/49.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 7 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = *dataptr.offset((8 as i32 * 3 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((z1 + tmp1 + tmp3)
            * (1.306122449f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp3 += tmp3;
        z1 -= tmp3;
        z1 -= tmp3;
        /* advance pointer to next column */
        z1 = z1
            * (0.461784020f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c6-c4)/2 */
        z2 = (tmp0 - tmp2)
            * (1.202428084f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c4-c6)/2 */
        z3 = (tmp1 - tmp2)
            * (0.411026446f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + z2
                + z3
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM; /* c4 */
        z1 -= z2;
        z2 = (tmp0 - tmp1)
            * (1.151670509f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (z2 + z3
            - (tmp1 - tmp3)
                * (0.923568041f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 + z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        tmp1 = (tmp10 + tmp11)
            * (1.221765677f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 = (tmp10 - tmp11)
            * (0.222383464f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (tmp11 + tmp12)
            * -((1.800824523f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp1 += tmp2;
        tmp3 = (tmp10 + tmp12)
            * (0.801442310f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 += tmp3;
        tmp2 += tmp3
            + tmp12
                * (2.443531355f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* (c3+c1-c5)/2 */
/* (c3+c5-c1)/2 */
/* -c1 */
/* c5 */
/* c3+c1-c5 */
/*
 * Perform the forward DCT on a 6x6 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_6x6(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* cK represents sqrt(2) * cos(K*pi/12). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11
            - (6 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = (tmp12
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp11 - tmp11)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp0 + tmp2)
            * (0.366025404f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + (tmp0 + tmp1 << 2 as i32)) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp1 - tmp2 << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp10 + (tmp2 - tmp1 << 2 as i32)) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/6)**2 = 16/9, which we fold
     * into the constant multipliers:
     * cK now represents sqrt(2) * cos(K*pi/12) * 16/9.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp12
            * (2.177324216f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp11 - tmp11)
            * (1.257078722f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp0 + tmp2)
            * (0.650711829f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + (tmp0 + tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = ((tmp0 - tmp1 - tmp2)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp10
            + (tmp2 - tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c5 */
/*
 * Perform the forward DCT on a 5x5 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_5x5(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* cK represents sqrt(2) * cos(K*pi/10). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 5 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = *elemptr.offset(2 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp10 + tmp2 - (5 as i32 * 128 as i32) as isize) << 2 as i32 + 1 as i32)
                as crate::jdct_h::DCTELEM;
        tmp11 = tmp11
            * (0.790569415f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 -= tmp2 << 2 as i32;
        tmp10 = tmp10
            * (0.353553391f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp11
            + tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = (tmp11 - tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp0 + tmp1)
            * (0.831253876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(1 as i32 as isize) = (tmp10
            + tmp0
                * (0.513743148f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) = (tmp10
            - tmp1
                * (2.176250899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* (c2+c4)/2 */
    /* (c2-c4)/2 */
    /* Odd part */
    /* c3 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/5)**2 = 64/25, which we partially
     * fold into the constant multipliers (other part was done in pass 1):
     * cK now represents sqrt(2) * cos(K*pi/10) * 32/25.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 5 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = *dataptr.offset((8 as i32 * 2 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp2)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp11 = tmp11
            * (1.011928851f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c4)/2 */
        tmp10 -= tmp2 << 2 as i32; /* (c2-c4)/2 */
        tmp10 = tmp10
            * (0.452548340f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp11
            + tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (tmp11 - tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp0 + tmp1)
            * (1.064004961f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + tmp0
                * (0.657591230f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp10
            - tmp1
                * (2.785601151f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c3 */
/*
 * Perform the forward DCT on a 4x4 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_4x4(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We must also scale the output by (8/4)**2 = 2**2, which we add here. */
    /* cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point FDCT]. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(2 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp0 + tmp1 - (4 as i32 * 128 as i32) as isize) << 2 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) =
            (tmp0 - tmp1 << 2 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 3 as i32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* c6 */
    /* Add fudge factor here for final descale. */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        /* Even part */
        /* Add fudge factor here for final descale. */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize)) as isize
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 - 1 as i32);
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            (tmp0 + tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (tmp0 - tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c6 */
/* Add fudge factor here for final descale. */
/*
 * Perform the forward DCT on a 3x3 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_3x3(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We scale the results further by 2**2 as part of output adaption */
    /* scaling for different DCT size. */
    /* cK represents sqrt(2) * cos(K*pi/6). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 3 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = *elemptr.offset(1 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp0 + tmp1 - (3 as i32 * 128 as i32) as isize) << 2 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = ((tmp0 - tmp1 - tmp1)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(1 as i32 as isize) = (tmp2
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/3)**2 = 64/9, which we partially
     * fold into the constant multipliers (other part was done in pass 1):
     * cK now represents sqrt(2) * cos(K*pi/6) * 16/9.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 3 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = *dataptr.offset((8 as i32 * 1 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp0 + tmp1)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = ((tmp0 - tmp1 - tmp1)
            * (1.257078722f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp2
            * (2.177324216f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/*
 * Perform the forward DCT on a 2x2 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_2x2(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* Row 0 */
    elemptr = (*sample_data.offset(0 as i32 as isize)).offset(start_col as isize);
    tmp0 = (*elemptr.offset(0 as i32 as isize) as i32 + *elemptr.offset(1 as i32 as isize) as i32)
        as crate::jmorecfg_h::INT32;
    tmp1 = (*elemptr.offset(0 as i32 as isize) as i32 - *elemptr.offset(1 as i32 as isize) as i32)
        as crate::jmorecfg_h::INT32;
    /* Row 1 */
    elemptr = (*sample_data.offset(1 as i32 as isize)).offset(start_col as isize);
    tmp2 = (*elemptr.offset(0 as i32 as isize) as i32 + *elemptr.offset(1 as i32 as isize) as i32)
        as crate::jmorecfg_h::INT32;
    tmp3 = (*elemptr.offset(0 as i32 as isize) as i32 - *elemptr.offset(1 as i32 as isize) as i32)
        as crate::jmorecfg_h::INT32;
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/2)**2 = 2**4.
     */
    /* Column 0 */
    /* Apply unsigned->signed conversion */
    *data.offset((8 as i32 * 0 as i32) as isize) = ((tmp0 + tmp2
        - (4 as i32 * 128 as i32) as isize)
        << 4 as i32) as crate::jdct_h::DCTELEM;
    *data.offset((8 as i32 * 1 as i32) as isize) =
        (tmp0 - tmp2 << 4 as i32) as crate::jdct_h::DCTELEM;
    /* Column 1 */
    *data.offset((8 as i32 * 0 as i32 + 1 as i32) as isize) =
        (tmp1 + tmp3 << 4 as i32) as crate::jdct_h::DCTELEM;
    *data.offset((8 as i32 * 1 as i32 + 1 as i32) as isize) =
        (tmp1 - tmp3 << 4 as i32) as crate::jdct_h::DCTELEM;
}
/*
 * Perform the forward DCT on a 1x1 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_1x1(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* We leave the result scaled up by an overall factor of 8. */
    /* We must also scale the output by (8/1)**2 = 2**6. */
    /* Apply unsigned->signed conversion */
    *data.offset(0 as i32 as isize) =
        (*(*sample_data.offset(0 as i32 as isize)).offset(start_col as isize) as i32 - 128 as i32)
            << 6 as i32;
}
/*
 * Perform the forward DCT on a 9x9 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_9x9(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 8] = [0; 8];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* we scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* cK represents sqrt(2) * cos(K*pi/18). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = *elemptr.offset(4 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2 + tmp3;
        z2 = tmp1 + tmp4;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((z1 + z2 - (9 as i32 * 128 as i32) as isize)
            << 1 as i32) as crate::jdct_h::DCTELEM; /* c2 */
        *dataptr.offset(6 as i32 as isize) = ((z1 - z2 - z2)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM; /* c6 */
        z1 = (tmp0 - tmp2)
            * (1.328926049f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp1 - tmp4 - tmp4)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = ((tmp2 - tmp3)
            * (1.083350441f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + z1
            + z2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp3 - tmp0)
            * (0.245575608f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + z1
            - z2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        *dataptr.offset(3 as i32 as isize) = ((tmp10 - tmp12 - tmp13)
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM; /* c3 */
        tmp11 = tmp11
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp0 = (tmp10 + tmp12)
            * (0.909038955f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c7 */
        tmp1 = (tmp10 + tmp13)
            * (0.483689525f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c1 */
        *dataptr.offset(1 as i32 as isize) = (tmp11
            + tmp0
            + tmp1
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM; /* Done. */
        tmp2 = (tmp12 - tmp13)
            * (1.392728481f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(5 as i32 as isize) = (tmp0 - tmp11 - tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp1 - tmp11
            + tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 9 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/9)**2 = 64/81, which we partially
     * fold into the constant multipliers and final/initial shifting:
     * cK now represents sqrt(2) * cos(K*pi/18) * 128/81.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = *dataptr.offset((8 as i32 * 4 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2 + tmp3;
        z2 = tmp1 + tmp4;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((z1 + z2)
            * (1.580246914f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = ((z1 - z2 - z2)
            * (1.117403309f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        z1 = (tmp0 - tmp2)
            * (2.100031287f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c2 */
        z2 = (tmp1 - tmp4 - tmp4)
            * (1.117403309f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = ((tmp2 - tmp3)
            * (1.711961190f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + z1
            + z2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp3 - tmp0)
            * (0.388070096f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + z1
            - z2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = ((tmp10 - tmp12 - tmp13)
            * (1.935399303f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp11 = tmp11
            * (1.935399303f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = (tmp10 + tmp12)
            * (1.436506004f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp1 = (tmp10 + tmp13)
            * (0.764348879f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp11
            + tmp0
            + tmp1
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp2 = (tmp12 - tmp13)
            * (2.200854883f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp0 - tmp11 - tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp1 - tmp11
            + tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c1 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 10x10 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_10x10(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 16] = [0; 16];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* we scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* cK represents sqrt(2) * cos(K*pi/20). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp4;
        tmp13 = tmp0 - tmp4;
        tmp11 = tmp1 + tmp3;
        tmp14 = tmp1 - tmp3;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12
            - (10 as i32 * 128 as i32) as isize)
            << 1 as i32) as crate::jdct_h::DCTELEM; /* c6 */
        tmp12 += tmp12;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp12)
            * (1.144122806f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 - tmp12)
                * (0.437016024f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp13 + tmp14)
            * (0.831253876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp13
                * (0.513743148f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp14
                * (2.176250899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp10 = tmp0 + tmp4; /* (c1-c9)/2 */
        tmp11 = tmp1 - tmp3; /* Done. */
        *dataptr.offset(5 as i32 as isize) =
            (tmp10 - tmp11 - tmp2 << 1 as i32) as crate::jdct_h::DCTELEM;
        tmp2 <<= 13 as i32;
        *dataptr.offset(1 as i32 as isize) = (tmp0
            * (1.396802247f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (1.260073511f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
            + tmp3
                * (0.642039522f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.221231742f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 - tmp4)
            * (0.951056516f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 + tmp3)
                * (0.587785252f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp10 + tmp11)
            * (0.309016994f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 << 13 as i32 - 1 as i32)
            - tmp2;
        *dataptr.offset(3 as i32 as isize) = (tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp12 - tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 10 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/10)**2 = 16/25, which we partially
     * fold into the constant multipliers and final/initial shifting:
     * cK now represents sqrt(2) * cos(K*pi/20) * 32/25.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp4;
        tmp13 = tmp0 - tmp4;
        tmp11 = tmp1 + tmp3;
        tmp14 = tmp1 - tmp3;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 += tmp12;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp12)
            * (1.464477191f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 - tmp12)
                * (0.559380511f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp13 + tmp14)
            * (1.064004961f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp13
                * (0.657591230f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp14
                * (2.785601151f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp4;
        tmp11 = tmp1 - tmp3;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = ((tmp10 - tmp11 - tmp2)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp2 = tmp2
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp0
            * (1.787906876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (1.612894094f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
            + tmp3
                * (0.821810588f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.283176630f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 - tmp4)
            * (1.217352341f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 + tmp3)
                * (0.752365123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp10 + tmp11)
            * (0.395541753f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp11
                * (0.64f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                    as crate::jmorecfg_h::INT32
            - tmp2;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp12 - tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* 32/25 */
/* (c1-c9)/2 */
/* 16/25 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on an 11x11 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_11x11(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 24] = [0; 24];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* we scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* cK represents sqrt(2) * cos(K*pi/22). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = *elemptr.offset(5 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp14 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp0 + tmp1 + tmp2 + tmp3 + tmp4 + tmp5
            - (11 as i32 * 128 as i32) as isize)
            << 1 as i32) as crate::jdct_h::DCTELEM; /* c10 */
        tmp5 += tmp5; /* c6 */
        tmp0 -= tmp5; /* c4 */
        tmp1 -= tmp5;
        tmp2 -= tmp5;
        tmp3 -= tmp5;
        tmp4 -= tmp5;
        z1 = (tmp0 + tmp3)
            * (1.356927976f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp2 + tmp4)
                * (0.201263574f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp1 - tmp3)
            * (0.926112931f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z3 = (tmp0 - tmp1)
            * (1.189712156f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (z1 + z2
            - tmp3
                * (1.018300590f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp4
                * (1.390975730f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = (z2
            + z3
            + tmp1
                * (0.062335650f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp2
                * (1.356927976f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.587485545f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (z1 + z3
            - tmp0
                * (1.620527200f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp2
                * (0.788749120f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp1 = (tmp10 + tmp11)
            * (1.286413905f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c3 */
        tmp2 = (tmp10 + tmp12)
            * (1.068791298f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp3 = (tmp10 + tmp13)
            * (0.764581576f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c7 */
        tmp0 = tmp1 + tmp2 + tmp3
            - tmp10
                * (1.719967871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (0.398430003f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c9 */
        tmp4 = (tmp11 + tmp12)
            * -((0.764581576f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* -c7 */
        tmp5 = (tmp11 + tmp13)
            * -((1.399818907f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* -c1 */
        tmp1 += tmp4
            + tmp5
            + tmp11
                * (1.276416582f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (1.068791298f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp10 = (tmp12 + tmp13)
            * (0.398430003f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c9 */
        tmp2 += tmp4 + tmp10
            - tmp12
                * (1.989053629f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (1.399818907f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c1 */
        tmp3 += tmp5
            + tmp10
            + tmp13
                * (1.305598626f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (1.286413905f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c3 */
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
                >> 13 as i32 - 1 as i32) as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
                >> 13 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
                >> 13 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32 - 1 as i32)
                >> 13 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 11 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/11)**2 = 64/121, which we partially
     * fold into the constant multipliers and final/initial shifting:
     * cK now represents sqrt(2) * cos(K*pi/22) * 128/121.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = *dataptr.offset((8 as i32 * 5 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp14 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            ((tmp0 + tmp1 + tmp2 + tmp3 + tmp4 + tmp5)
                * (1.057851240f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        tmp5 += tmp5;
        tmp0 -= tmp5;
        tmp1 -= tmp5;
        tmp2 -= tmp5;
        tmp3 -= tmp5;
        tmp4 -= tmp5;
        /* advance pointer to next column */
        z1 = (tmp0 + tmp3)
            * (1.435427942f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp2 + tmp4)
                * (0.212906922f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c10 */
        z2 = (tmp1 - tmp3)
            * (0.979689713f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        z3 = (tmp0 - tmp1)
            * (1.258538479f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c4 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (z1 + z2
            - tmp3
                * (1.077210542f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp4
                * (1.471445400f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (z2
            + z3
            + tmp1
                * (0.065941844f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp2
                * (1.435427942f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.621472312f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (z1 + z3
            - tmp0
                * (1.714276708f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp2
                * (0.834379234f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp1 = (tmp10 + tmp11)
            * (1.360834544f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 = (tmp10 + tmp12)
            * (1.130622199f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp3 = (tmp10 + tmp13)
            * (0.808813568f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + tmp2 + tmp3
            - tmp10
                * (1.819470145f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (0.421479672f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp4 = (tmp11 + tmp12)
            * -((0.808813568f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp5 = (tmp11 + tmp13)
            * -((1.480800167f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp1 += tmp4
            + tmp5
            + tmp11
                * (1.350258864f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (1.130622199f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = (tmp12 + tmp13)
            * (0.421479672f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 += tmp4 + tmp10
            - tmp12
                * (2.104122847f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (1.480800167f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp3 += tmp5
            + tmp10
            + tmp13
                * (1.381129125f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (1.360834544f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c3 */
/* c5 */
/* c7 */
/* c9 */
/* -c7 */
/* -c1 */
/* c5 */
/* c9 */
/* c1 */
/* c3 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 12x12 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_12x12(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 32] = [0; 32];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* cK represents sqrt(2) * cos(K*pi/24). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp5;
        tmp13 = tmp0 - tmp5;
        tmp11 = tmp1 + tmp4;
        tmp14 = tmp1 - tmp4;
        tmp12 = tmp2 + tmp3;
        tmp15 = tmp2 - tmp3;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = (tmp10 + tmp11 + tmp12
            - (12 as i32 * 128 as i32) as isize)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp13 - tmp14 - tmp15) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp12)
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = (tmp14 - tmp15
            + (tmp13 + tmp15)
                * (1.366025404f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp10 = (tmp1 + tmp4) * 4433 as i32 as crate::jmorecfg_h::INT32; /* c9 */
        tmp14 = tmp10 + tmp1 * 6270 as i32 as crate::jmorecfg_h::INT32; /* c3-c9 */
        tmp15 = tmp10 - tmp4 * 15137 as i32 as crate::jmorecfg_h::INT32; /* c3+c9 */
        tmp12 = (tmp0 + tmp2)
            * (1.121971054f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp13 = (tmp0 + tmp3)
            * (0.860918669f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c7 */
        tmp10 = tmp12 + tmp13 + tmp14
            - tmp0
                * (0.580774953f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.184591911f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c11 */
        tmp11 = (tmp2 + tmp3)
            * -((0.184591911f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* -c11 */
        tmp12 += tmp11
            - tmp15
            - tmp2
                * (2.339493912f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.860918669f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c7 */
        tmp13 += tmp11 - tmp14
            + tmp3
                * (0.725788011f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (1.121971054f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp11 = tmp15
            + (tmp0 - tmp3)
                * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp2 + tmp5) * 4433 as i32 as crate::jmorecfg_h::INT32; /* c9 */
        *dataptr.offset(1 as i32 as isize) = (tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) = (tmp11
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) = (tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 12 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/12)**2 = 4/9, which we partially
     * fold into the constant multipliers and final shifting:
     * cK now represents sqrt(2) * cos(K*pi/24) * 8/9.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp5;
        tmp13 = tmp0 - tmp5;
        tmp11 = tmp1 + tmp4;
        tmp14 = tmp1 - tmp4;
        tmp12 = tmp2 + tmp3;
        tmp15 = tmp2 - tmp3;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = ((tmp13 - tmp14 - tmp15)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp12)
            * (1.088662108f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = ((tmp14 - tmp15)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp13 + tmp15)
                * (1.214244803f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp1 + tmp4)
            * (0.481063200f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp14 = tmp10
            + tmp1
                * (0.680326102f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp15 = tmp10
            - tmp4
                * (1.642452502f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (0.997307603f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (0.765261039f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12 + tmp13 + tmp14
            - tmp0
                * (0.516244403f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.164081699f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = (tmp2 + tmp3)
            * -((0.164081699f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp12 += tmp11
            - tmp15
            - tmp2
                * (2.079550144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.765261039f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp11 - tmp14
            + tmp3
                * (0.645144899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (0.997307603f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = tmp15
            + (tmp0 - tmp3)
                * (1.161389302f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp2 + tmp5)
                * (0.481063200f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp10 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp11 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp12 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp13 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* c5 */
/* c7 */
/* c11 */
/* -c11 */
/* c7 */
/* c5 */
/* c9 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 13x13 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_13x13(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 40] = [0; 40];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* cK represents sqrt(2) * cos(K*pi/26). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = *elemptr.offset(6 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp14 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp15 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = (tmp0 + tmp1 + tmp2 + tmp3 + tmp4 + tmp5 + tmp6
            - (13 as i32 * 128 as i32) as isize)
            as crate::jdct_h::DCTELEM; /* (c8-c12)/2 */
        tmp6 += tmp6; /* (c8+c12)/2 */
        tmp0 -= tmp6;
        tmp1 -= tmp6;
        tmp2 -= tmp6;
        tmp3 -= tmp6;
        tmp4 -= tmp6;
        tmp5 -= tmp6;
        *dataptr.offset(2 as i32 as isize) = (tmp0
            * (1.373119086f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (1.058554052f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
                * (0.501487041f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp3
                * (0.170464608f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp4
                * (0.803364869f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (1.252223920f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp0 - tmp2)
            * (1.155388986f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp3 - tmp4)
                * (0.435816023f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 - tmp5)
                * (0.316450131f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp0 + tmp2)
            * (0.096834934f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp3 + tmp4)
                * (0.937303064f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp1 + tmp5)
                * (0.486914739f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(4 as i32 as isize) =
            (z1 + z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
                >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (z1 - z2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp1 = (tmp10 + tmp11)
            * (1.322312651f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c3 */
        tmp2 = (tmp10 + tmp12)
            * (1.163874945f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp3 = (tmp10 + tmp13)
            * (0.937797057f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 + tmp15)
                * (0.338443458f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c11 */
        tmp0 = tmp1 + tmp2 + tmp3
            - tmp10
                * (2.020082300f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (0.318774355f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c9-c11 */
        tmp4 = (tmp14 - tmp15)
            * (0.937797057f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 + tmp12)
                * (0.338443458f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c11 */
        tmp5 = (tmp11 + tmp13)
            * -((1.163874945f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* -c5 */
        tmp1 += tmp4
            + tmp5
            + tmp11
                * (0.837223564f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (2.341699410f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c1+c7 */
        tmp6 = (tmp12 + tmp13)
            * -((0.657217813f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* -c9 */
        tmp2 += tmp4 + tmp6
            - tmp12
                * (1.572116027f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp15
                * (2.260109708f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c3+c7 */
        tmp3 += tmp5
            + tmp6
            + tmp13
                * (2.205608352f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp15
                * (1.742345811f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c1+c11 */
        *dataptr.offset(1 as i32 as isize) = (tmp0
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) = (tmp1
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) = (tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp3
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 13 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/13)**2 = 64/169, which we partially
     * fold into the constant multipliers and final shifting:
     * cK now represents sqrt(2) * cos(K*pi/26) * 128/169.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = *dataptr.offset((8 as i32 * 6 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp14 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp15 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            ((tmp0 + tmp1 + tmp2 + tmp3 + tmp4 + tmp5 + tmp6)
                * (0.757396450f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        tmp6 += tmp6;
        tmp0 -= tmp6;
        tmp1 -= tmp6;
        tmp2 -= tmp6;
        tmp3 -= tmp6;
        tmp4 -= tmp6;
        tmp5 -= tmp6;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp0
            * (1.039995521f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (0.801745081f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
                * (0.379824504f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp3
                * (0.129109289f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp4
                * (0.608465700f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (0.948429952f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        z1 = (tmp0 - tmp2)
            * (0.875087516f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp3 - tmp4)
                * (0.330085509f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 - tmp5)
                * (0.239678205f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* (c8-c12)/2 */
        z2 = (tmp0 + tmp2)
            * (0.073342435f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp3 + tmp4)
                * (0.709910013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp1 + tmp5)
                * (0.368787494f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* (c8+c12)/2 */
        *dataptr.offset((8 as i32 * 4 as i32) as isize) =
            (z1 + z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 - z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        tmp1 = (tmp10 + tmp11)
            * (1.001514908f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 = (tmp10 + tmp12)
            * (0.881514751f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp3 = (tmp10 + tmp13)
            * (0.710284161f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 + tmp15)
                * (0.256335874f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 + tmp2 + tmp3
            - tmp10
                * (1.530003162f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp14
                * (0.241438564f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp4 = (tmp14 - tmp15)
            * (0.710284161f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 + tmp12)
                * (0.256335874f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp5 = (tmp11 + tmp13)
            * -((0.881514751f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp1 += tmp4
            + tmp5
            + tmp11
                * (0.634110155f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (1.773594819f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp6 = (tmp12 + tmp13)
            * -((0.497774438f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp2 += tmp4 + tmp6
            - tmp12
                * (1.190715098f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp15
                * (1.711799069f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp3 += tmp5
            + tmp6
            + tmp13
                * (1.670519935f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp15
                * (1.319646532f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c3 */
/* c5 */
/* c11 */
/* c9-c11 */
/* c11 */
/* -c5 */
/* c1+c7 */
/* -c9 */
/* c3+c7 */
/* c1+c11 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 14x14 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_14x14(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 48] = [0; 48];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* cK represents sqrt(2) * cos(K*pi/28). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp6;
        tmp14 = tmp0 - tmp6;
        tmp11 = tmp1 + tmp5;
        tmp15 = tmp1 - tmp5;
        tmp12 = tmp2 + tmp4;
        tmp16 = tmp2 - tmp4;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = (tmp10 + tmp11 + tmp12 + tmp13
            - (14 as i32 * 128 as i32) as isize)
            as crate::jdct_h::DCTELEM; /* c6 */
        tmp13 += tmp13;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp13)
            * (1.274162392f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13)
                * (0.314692123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp12 - tmp13)
                * (0.881747734f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        tmp10 = (tmp14 + tmp15)
            * (1.105676686f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp14
                * (0.273079590f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (0.613604268f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp15
                * (1.719280954f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp16
                * (1.378756276f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp10 = tmp1 + tmp2; /* -c13 */
        tmp11 = tmp5 - tmp4; /* c1 */
        *dataptr.offset(7 as i32 as isize) =
            (tmp0 - tmp10 + tmp3 - tmp11 - tmp6) as crate::jdct_h::DCTELEM; /* c9 */
        tmp3 <<= 13 as i32; /* c11 */
        tmp10 = tmp10
            * -((0.158341681f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* Done. */
        tmp11 = tmp11
            * (1.405321284f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 += tmp11 - tmp3;
        tmp11 = (tmp0 + tmp2)
            * (1.197448846f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 + tmp6)
                * (0.752406978f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(5 as i32 as isize) = (tmp10 + tmp11
            - tmp2
                * (2.373959773f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (1.119999435f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 + tmp1)
            * (1.334852607f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 - tmp6)
                * (0.467085129f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(3 as i32 as isize) = (tmp10 + tmp12
            - tmp1
                * (0.424103948f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (3.069855259f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(1 as i32 as isize) = (tmp11 + tmp12 + tmp3 + tmp6
            - (tmp0 + tmp6)
                * (1.126980169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 14 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/14)**2 = 16/49, which we partially
     * fold into the constant multipliers and final shifting:
     * cK now represents sqrt(2) * cos(K*pi/28) * 32/49.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp6;
        tmp14 = tmp0 - tmp6;
        tmp11 = tmp1 + tmp5;
        tmp15 = tmp1 - tmp5;
        tmp12 = tmp2 + tmp4;
        tmp16 = tmp2 - tmp4;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12 + tmp13)
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp13 += tmp13;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp13)
            * (0.832106052f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13)
                * (0.205513223f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp12 - tmp13)
                * (0.575835255f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp14 + tmp15)
            * (0.722074570f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp14
                * (0.178337691f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (0.400721155f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp15
                * (1.122795725f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp16
                * (0.900412262f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp1 + tmp2;
        tmp11 = tmp5 - tmp4;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = ((tmp0 - tmp10 + tmp3 - tmp11 - tmp6)
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp3 = tmp3
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp10
            * -((0.103406812f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp11 = tmp11
            * (0.917760839f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 += tmp11 - tmp3;
        tmp11 = (tmp0 + tmp2)
            * (0.782007410f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 + tmp6)
                * (0.491367823f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp10 + tmp11
            - tmp2
                * (1.550341076f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.731428202f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 + tmp1)
            * (0.871740478f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 - tmp6)
                * (0.305035186f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp10 + tmp12
            - tmp1
                * (0.276965844f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (2.004803435f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp11 + tmp12 + tmp3
            - tmp0
                * (0.735987049f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (0.082925825f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* 32/49 */
/* -c13 */
/* c1 */
/* c9 */
/* c11 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 15x15 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_15x15(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp7: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 56] = [0; 56];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* cK represents sqrt(2) * cos(K*pi/30). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp7 = *elemptr.offset(7 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp14 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp15 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp16 = (*elemptr.offset(6 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp4 + tmp5;
        z2 = tmp1 + tmp3 + tmp6;
        z3 = tmp2 + tmp7;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) =
            (z1 + z2 + z3 - (15 as i32 * 128 as i32) as isize) as crate::jdct_h::DCTELEM; /* c4+c8 */
        z3 += z3; /* c2-c4 */
        *dataptr.offset(6 as i32 as isize) = ((z1 - z3)
            * (1.144122806f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (z2 - z3)
                * (0.437016024f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM; /* (c6+c12)/2 */
        tmp2 += (tmp1 + tmp4 >> 1 as i32) - tmp7 - tmp7;
        z1 = (tmp3 - tmp2)
            * (1.531135173f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp6 - tmp2)
                * (2.238241955f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp5 - tmp2)
            * (0.798468008f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp0 - tmp2)
                * (0.091361227f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        z3 = (tmp0 - tmp3)
            * (1.383309603f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (0.946293579f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp1 - tmp4)
                * (0.790569415f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) =
            (z1 + z3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
                >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) =
            (z2 + z3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
                >> 13 as i32) as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp2 = (tmp10 - tmp12 - tmp13 + tmp15 + tmp16)
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        /* c5 */
        tmp1 = (tmp10 - tmp14 - tmp15)
            * (1.344997024f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13 - tmp16)
                * (0.831253876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c9 */
        tmp12 = tmp12
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c5 */
        tmp4 = (tmp10 - tmp16)
            * (1.406466353f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 + tmp14)
                * (1.344997024f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp13 + tmp15)
                * (0.575212477f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c11 */
        tmp0 = tmp13
            * (0.475753014f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (0.513743148f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (1.700497885f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
            + tmp12; /* c1+c13 */
        tmp3 = tmp10
            * -((0.355500862f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            - tmp11
                * (2.176250899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp15
                * (0.869244010f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
            - tmp12; /* c11+c13 */
        *dataptr.offset(1 as i32 as isize) = (tmp0
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) = (tmp1
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) = (tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp3
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32)
            >> 13 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 15 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/15)**2 = 64/225, which we partially
     * fold into the constant multipliers and final shifting:
     * cK now represents sqrt(2) * cos(K*pi/30) * 256/225.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp7 = *dataptr.offset((8 as i32 * 7 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp14 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp15 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp16 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp4 + tmp5;
        z2 = tmp1 + tmp3 + tmp6;
        z3 = tmp2 + tmp7;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((z1 + z2 + z3)
            * (1.137777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        z3 += z3;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = ((z1 - z3)
            * (1.301757503f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (z2 - z3)
                * (0.497227121f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp2 += (tmp1 + tmp4 >> 1 as i32) - tmp7 - tmp7;
        /* advance pointer to next column */
        z1 = (tmp3 - tmp2)
            * (1.742091575f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp6 - tmp2)
                * (2.546621957f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c4+c8 */
        z2 = (tmp5 - tmp2)
            * (0.908479156f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp0 - tmp2)
                * (0.103948774f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c2-c4 */
        z3 = (tmp0 - tmp3)
            * (1.573898926f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (1.076671805f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp1 - tmp4)
                * (0.899492312f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* (c6+c12)/2 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + z3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) =
            (z2 + z3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        tmp2 = (tmp10 - tmp12 - tmp13 + tmp15 + tmp16)
            * (1.393487498f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp1 = (tmp10 - tmp14 - tmp15)
            * (1.530307725f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13 - tmp16)
                * (0.945782187f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = tmp12
            * (1.393487498f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp4 = (tmp10 - tmp16)
            * (1.600246161f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 + tmp14)
                * (1.530307725f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp13 + tmp15)
                * (0.654463974f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp13
            * (0.541301207f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp14
                * (0.584525538f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (1.934788705f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
            + tmp12;
        tmp3 = tmp10
            * -((0.404480980f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            - tmp11
                * (2.476089912f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp15
                * (0.989006518f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
            - tmp12;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c5 */
/* c9 */
/* c5 */
/* c11 */
/* c1+c13 */
/* c11+c13 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 16x16 sample block.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_16x16(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp7: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp17: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 64] = [0; 64];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* cK represents sqrt(2) * cos(K*pi/32). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(15 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp7 = (*elemptr.offset(7 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp7;
        tmp14 = tmp0 - tmp7;
        tmp11 = tmp1 + tmp6;
        tmp15 = tmp1 - tmp6;
        tmp12 = tmp2 + tmp5;
        tmp16 = tmp2 - tmp5;
        tmp13 = tmp3 + tmp4;
        tmp17 = tmp3 - tmp4;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(15 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp7 = (*elemptr.offset(7 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12 + tmp13
            - (16 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM; /* c2[16] = c1[8] */
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp13)
            * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp12) * 4433 as i32 as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp17 - tmp15)
            * (0.275899379f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 - tmp16)
                * (1.387039845f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp15
                * (1.451774982f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (2.172734804f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp14
                * (0.211164243f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp17
                * (1.061594338f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp11 = (tmp0 + tmp1)
            * (1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp7)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c13 */
        tmp12 = (tmp0 + tmp2)
            * (1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 + tmp7)
                * (0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c11 */
        tmp13 = (tmp0 + tmp3)
            * (1.093201867f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 - tmp7)
                * (0.897167586f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c9 */
        tmp14 = (tmp1 + tmp2)
            * (0.138617169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (1.407403738f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c1 */
        tmp15 = (tmp1 + tmp3)
            * -((0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp4 + tmp6)
                * -((1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32); /* -c5 */
        tmp16 = (tmp2 + tmp3)
            * -((1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp5 - tmp4)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c13 */
        tmp10 = tmp11 + tmp12 + tmp13
            - tmp0
                * (2.286341144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp7
                * (0.779653625f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c15+c13-c11+c9 */
        tmp11 += tmp14
            + tmp15
            + tmp1
                * (0.071888074f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (1.663905119f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c7+c13+c1-c5 */
        tmp12 += tmp14 + tmp16
            - tmp2
                * (1.125726048f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (1.227391138f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c9-c11+c1-c13 */
        tmp13 += tmp15
            + tmp16
            + tmp3
                * (1.065388962f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (2.167985692f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c1+c13+c5-c9 */
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) =
            (tmp11 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp12 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp13 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 8 as i32 * 2 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/16)**2 = 1/2**2.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp7 = (*dataptr.offset((8 as i32 * 7 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp7;
        tmp14 = tmp0 - tmp7;
        tmp11 = tmp1 + tmp6;
        tmp15 = tmp1 - tmp6;
        tmp12 = tmp2 + tmp5;
        tmp16 = tmp2 - tmp5;
        tmp13 = tmp3 + tmp4;
        tmp17 = tmp3 - tmp4;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp7 = (*dataptr.offset((8 as i32 * 7 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = (tmp10
            + tmp11
            + tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 + 2 as i32 - 1 as i32)
            >> 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp13)
            * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp12) * 4433 as i32 as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp17 - tmp15)
            * (0.275899379f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 - tmp16)
                * (1.387039845f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c2[16] = c1[8] */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp15
                * (1.451774982f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (2.172734804f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp14
                * (0.211164243f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp17
                * (1.061594338f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp11 = (tmp0 + tmp1)
            * (1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp7)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 + tmp7)
                * (0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (1.093201867f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 - tmp7)
                * (0.897167586f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp14 = (tmp1 + tmp2)
            * (0.138617169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (1.407403738f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp15 = (tmp1 + tmp3)
            * -((0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp4 + tmp6)
                * -((1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp16 = (tmp2 + tmp3)
            * -((1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp5 - tmp4)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + tmp13
            - tmp0
                * (2.286341144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp7
                * (0.779653625f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 += tmp14
            + tmp15
            + tmp1
                * (0.071888074f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (1.663905119f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 += tmp14 + tmp16
            - tmp2
                * (1.125726048f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (1.227391138f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp15
            + tmp16
            + tmp3
                * (1.065388962f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (2.167985692f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp11
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c13 */
/* c11 */
/* c9 */
/* c1 */
/* -c5 */
/* c13 */
/* c15+c13-c11+c9 */
/* c7+c13+c1-c5 */
/* c9-c11+c1-c13 */
/* c1+c13+c5-c9 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 16x8 sample block.
 *
 * 16-point FDCT in pass 1 (rows), 8-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_16x8(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp7: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp17: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 16-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/32). */
    dataptr = data;
    ctr = 0 as i32;
    ctr = 0 as i32;
    while ctr < 8 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(15 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp7 = (*elemptr.offset(7 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp7;
        tmp14 = tmp0 - tmp7;
        tmp11 = tmp1 + tmp6;
        tmp15 = tmp1 - tmp6;
        tmp12 = tmp2 + tmp5;
        tmp16 = tmp2 - tmp5;
        tmp13 = tmp3 + tmp4;
        tmp17 = tmp3 - tmp4;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(15 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(14 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp7 = (*elemptr.offset(7 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12 + tmp13
            - (16 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp13)
            * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp12) * 4433 as i32 as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp17 - tmp15)
            * (0.275899379f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 - tmp16)
                * (1.387039845f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp15
                * (1.451774982f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (2.172734804f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp14
                * (0.211164243f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp17
                * (1.061594338f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp11 = (tmp0 + tmp1)
            * (1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp7)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 + tmp7)
                * (0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (1.093201867f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 - tmp7)
                * (0.897167586f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp14 = (tmp1 + tmp2)
            * (0.138617169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (1.407403738f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp15 = (tmp1 + tmp3)
            * -((0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp4 + tmp6)
                * -((1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp16 = (tmp2 + tmp3)
            * -((1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp5 - tmp4)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + tmp13
            - tmp0
                * (2.286341144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp7
                * (0.779653625f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 += tmp14
            + tmp15
            + tmp1
                * (0.071888074f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (1.663905119f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 += tmp14 + tmp16
            - tmp2
                * (1.125726048f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (1.227391138f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp15
            + tmp16
            + tmp3
                * (1.065388962f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (2.167985692f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp11 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp12 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp13 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* c2[16] = c1[8] */
    /* Odd part */
    /* c13 */
    /* c11 */
    /* c9 */
    /* c1 */
    /* -c5 */
    /* c13 */
    /* c15+c13-c11+c9 */
    /* c7+c13+c1-c5 */
    /* c9-c11+c1-c13 */
    /* c1+c13+c5-c9 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by 8/16 = 1/2.
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part per LL&M figure 1 --- note that published figure is faulty;
         * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
         */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp3;
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = (tmp10
            + tmp11
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 + 1 as i32 - 1 as i32)
            >> 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (tmp10 - tmp11
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 + 1 as i32 - 1 as i32)
            >> 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32
                + ((1 as i32 as crate::jmorecfg_h::INT32)
                    << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32
                + ((1 as i32 as crate::jmorecfg_h::INT32)
                    << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = tmp0 + tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp1 + tmp3;
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32;
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32;
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32;
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32;
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32);
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp0
            + tmp10
            + tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp1
            + tmp11
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp2
            + tmp11
            + tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp3
            + tmp10
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Odd part per figure 8 --- note paper omits factor of sqrt(2).
 * 8-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
 * i0..i3 in the paper are tmp0..tmp3 here.
 */
/*  c3 */
/*  c1+c3-c5-c7 */
/*  c1+c3+c5-c7 */
/*  c1+c3-c5+c7 */
/* -c1+c3+c5-c7 */
/*  c7-c3 */
/* -c1-c3 */
/*  c5-c3 */
/* -c3-c5 */
/*
 * Perform the forward DCT on a 14x7 sample block.
 *
 * 14-point FDCT in pass 1 (rows), 7-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_14x7(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Zero bottom row of output coefficient block. */
    crate::stdlib::memset(
        &mut *data.offset((8 as i32 * 7 as i32) as isize) as *mut crate::jdct_h::DCTELEM
            as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 14-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/28). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 7 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp13 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp6;
        tmp14 = tmp0 - tmp6;
        tmp11 = tmp1 + tmp5;
        tmp15 = tmp1 - tmp5;
        tmp12 = tmp2 + tmp4;
        tmp16 = tmp2 - tmp4;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(13 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(12 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp6 = (*elemptr.offset(6 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12 + tmp13
            - (14 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        tmp13 += tmp13;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp13)
            * (1.274162392f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13)
                * (0.314692123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp12 - tmp13)
                * (0.881747734f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp14 + tmp15)
            * (1.105676686f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp14
                * (0.273079590f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (0.613604268f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp15
                * (1.719280954f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp16
                * (1.378756276f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp1 + tmp2;
        tmp11 = tmp5 - tmp4;
        *dataptr.offset(7 as i32 as isize) =
            (tmp0 - tmp10 + tmp3 - tmp11 - tmp6 << 2 as i32) as crate::jdct_h::DCTELEM;
        tmp3 <<= 13 as i32;
        tmp10 = tmp10
            * -((0.158341681f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp11 = tmp11
            * (1.405321284f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 += tmp11 - tmp3;
        tmp11 = (tmp0 + tmp2)
            * (1.197448846f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 + tmp6)
                * (0.752406978f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(5 as i32 as isize) = (tmp10 + tmp11
            - tmp2
                * (2.373959773f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (1.119999435f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 + tmp1)
            * (1.334852607f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 - tmp6)
                * (0.467085129f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(3 as i32 as isize) = (tmp10 + tmp12
            - tmp1
                * (0.424103948f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (3.069855259f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(1 as i32 as isize) = (tmp11 + tmp12 + tmp3 + tmp6
            - (tmp0 + tmp6)
                * (1.126980169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* c6 */
    /* Odd part */
    /* -c13 */
    /* c1 */
    /* c9 */
    /* c11 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/14)*(8/7) = 32/49, which we
     * partially fold into the constant multipliers and final shifting:
     * 7-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/14) * 64/49.
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = *dataptr.offset((8 as i32 * 3 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((z1 + tmp1 + tmp3)
            * (1.306122449f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp3 += tmp3;
        z1 -= tmp3;
        z1 -= tmp3;
        /* advance pointer to next column */
        z1 = z1
            * (0.461784020f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c6-c4)/2 */
        z2 = (tmp0 - tmp2)
            * (1.202428084f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c4-c6)/2 */
        z3 = (tmp1 - tmp2)
            * (0.411026446f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + z2
                + z3
                + ((1 as i32 as crate::jmorecfg_h::INT32)
                    << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM; /* c4 */
        z1 -= z2;
        z2 = (tmp0 - tmp1)
            * (1.151670509f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (z2 + z3
            - (tmp1 - tmp3)
                * (0.923568041f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 + z2
                + ((1 as i32 as crate::jmorecfg_h::INT32)
                    << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        tmp1 = (tmp10 + tmp11)
            * (1.221765677f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp2 = (tmp10 - tmp11)
            * (0.222383464f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 = tmp1 - tmp2;
        tmp1 += tmp2;
        tmp2 = (tmp11 + tmp12)
            * -((1.800824523f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp1 += tmp2;
        tmp3 = (tmp10 + tmp12)
            * (0.801442310f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 += tmp3;
        tmp2 += tmp3
            + tmp12
                * (2.443531355f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp0
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp1
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp2
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* (c3+c1-c5)/2 */
/* (c3+c5-c1)/2 */
/* -c1 */
/* c5 */
/* c3+c1-c5 */
/*
 * Perform the forward DCT on a 12x6 sample block.
 *
 * 12-point FDCT in pass 1 (rows), 6-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_12x6(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Zero 2 bottom rows of output coefficient block. */
    crate::stdlib::memset(
        &mut *data.offset((8 as i32 * 6 as i32) as isize) as *mut crate::jdct_h::DCTELEM
            as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong)
            .wrapping_mul(2 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 12-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/24). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp5;
        tmp13 = tmp0 - tmp5;
        tmp11 = tmp1 + tmp4;
        tmp14 = tmp1 - tmp4;
        tmp12 = tmp2 + tmp3;
        tmp15 = tmp2 - tmp3;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(11 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(10 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp5 = (*elemptr.offset(5 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12
            - (12 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) =
            (tmp13 - tmp14 - tmp15 << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp12)
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = (tmp14 - tmp15
            + (tmp13 + tmp15)
                * (1.366025404f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp1 + tmp4) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp14 = tmp10 + tmp1 * 6270 as i32 as crate::jmorecfg_h::INT32;
        tmp15 = tmp10 - tmp4 * 15137 as i32 as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (1.121971054f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (0.860918669f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12 + tmp13 + tmp14
            - tmp0
                * (0.580774953f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.184591911f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = (tmp2 + tmp3)
            * -((0.184591911f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp12 += tmp11
            - tmp15
            - tmp2
                * (2.339493912f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.860918669f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp11 - tmp14
            + tmp3
                * (0.725788011f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (1.121971054f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = tmp15
            + (tmp0 - tmp3)
                * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp2 + tmp5) * 4433 as i32 as crate::jmorecfg_h::INT32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp11 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp12 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp13 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* c9 */
    /* c3-c9 */
    /* c3+c9 */
    /* c5 */
    /* c7 */
    /* c11 */
    /* -c11 */
    /* c7 */
    /* c5 */
    /* c9 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/12)*(8/6) = 8/9, which we
     * partially fold into the constant multipliers and final shifting:
     * 6-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/12) * 16/9.
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp12
            * (2.177324216f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp11 - tmp11)
            * (1.257078722f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp0 + tmp2)
            * (0.650711829f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + (tmp0 + tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = ((tmp0 - tmp1 - tmp2)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp10
            + (tmp2 - tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c5 */
/*
 * Perform the forward DCT on a 10x5 sample block.
 *
 * 10-point FDCT in pass 1 (rows), 5-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_10x5(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Zero 3 bottom rows of output coefficient block. */
    crate::stdlib::memset(
        &mut *data.offset((8 as i32 * 5 as i32) as isize) as *mut crate::jdct_h::DCTELEM
            as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong)
            .wrapping_mul(3 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 10-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/20). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 5 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp4;
        tmp13 = tmp0 - tmp4;
        tmp11 = tmp1 + tmp3;
        tmp14 = tmp1 - tmp3;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(9 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(8 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp4 = (*elemptr.offset(4 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11 + tmp12
            - (10 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        tmp12 += tmp12;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp12)
            * (1.144122806f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 - tmp12)
                * (0.437016024f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp13 + tmp14)
            * (0.831253876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp10
            + tmp13
                * (0.513743148f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (tmp10
            - tmp14
                * (2.176250899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp4;
        tmp11 = tmp1 - tmp3;
        *dataptr.offset(5 as i32 as isize) =
            (tmp10 - tmp11 - tmp2 << 2 as i32) as crate::jdct_h::DCTELEM;
        tmp2 <<= 13 as i32;
        *dataptr.offset(1 as i32 as isize) = (tmp0
            * (1.396802247f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (1.260073511f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
            + tmp3
                * (0.642039522f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.221231742f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 - tmp4)
            * (0.951056516f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 + tmp3)
                * (0.587785252f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp10 + tmp11)
            * (0.309016994f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 << 13 as i32 - 1 as i32)
            - tmp2;
        *dataptr.offset(3 as i32 as isize) = (tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp12 - tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* c6 */
    /* Odd part */
    /* (c1-c9)/2 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/10)*(8/5) = 32/25, which we
     * fold into the constant multipliers:
     * 5-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/10) * 32/25.
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = *dataptr.offset((8 as i32 * 2 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp2)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp11 = tmp11
            * (1.011928851f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2+c4)/2 */
        tmp10 -= tmp2 << 2 as i32; /* (c2-c4)/2 */
        tmp10 = tmp10
            * (0.452548340f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp11
            + tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = (tmp11 - tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp0 + tmp1)
            * (1.064004961f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + tmp0
                * (0.657591230f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp10
            - tmp1
                * (2.785601151f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c3 */
/*
 * Perform the forward DCT on an 8x4 sample block.
 *
 * 8-point FDCT in pass 1 (rows), 4-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_8x4(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Zero 4 bottom rows of output coefficient block. */
    crate::stdlib::memset(
        &mut *data.offset((8 as i32 * 4 as i32) as isize) as *mut crate::jdct_h::DCTELEM
            as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(8 as i32 as libc::c_ulong)
            .wrapping_mul(4 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We must also scale the output by 8/4 = 2, which we add here. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp3;
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp10 + tmp11 - (8 as i32 * 128 as i32) as isize) << 2 as i32 + 1 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) =
            (tmp10 - tmp11 << 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 2 as i32;
        *dataptr.offset(2 as i32 as isize) = (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp1 + tmp3;
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 2 as i32;
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32;
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32;
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32;
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32);
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + tmp10 + tmp12 >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp1 + tmp11 + tmp13 >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp2 + tmp11 + tmp12 >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) =
            (tmp3 + tmp10 + tmp13 >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part per LL&M figure 1 --- note that published figure is faulty;
     * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
     */
    /* Apply unsigned->signed conversion */
    /* Add fudge factor here for final descale. */
    /* Odd part per figure 8 --- note paper omits factor of sqrt(2).
     * 8-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
     * i0..i3 in the paper are tmp0..tmp3 here.
     */
    /*  c3 */
    /* Add fudge factor here for final descale. */
    /*  c1+c3-c5-c7 */
    /*  c1+c3+c5-c7 */
    /*  c1+c3-c5+c7 */
    /* -c1+c3+c5-c7 */
    /*  c7-c3 */
    /* -c1-c3 */
    /*  c5-c3 */
    /* -c3-c5 */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * 4-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
     */
    dataptr = data;
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        /* Add fudge factor here for final descale. */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize)) as isize
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 - 1 as i32);
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            (tmp0 + tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (tmp0 - tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c6 */
/* Add fudge factor here for final descale. */
/*
 * Perform the forward DCT on a 6x3 sample block.
 *
 * 6-point FDCT in pass 1 (rows), 3-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_6x3(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* 6-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/12). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 3 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp10 + tmp11 - (6 as i32 * 128 as i32) as isize) << 2 as i32 + 1 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = (tmp12
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp11 - tmp11)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = (tmp0 + tmp2)
            * (0.366025404f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + (tmp0 + tmp1 << 2 as i32 + 1 as i32)) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp1 - tmp2 << 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp10 + (tmp2 - tmp1 << 2 as i32 + 1 as i32)) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/6)*(8/3) = 32/9, which we partially
     * fold into the constant multipliers (other part was done in pass 1):
     * 3-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/6) * 16/9.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = *dataptr.offset((8 as i32 * 1 as i32) as isize) as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp0 + tmp1)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = ((tmp0 - tmp1 - tmp1)
            * (1.257078722f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp2
            * (2.177324216f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/*
 * Perform the forward DCT on a 4x2 sample block.
 *
 * 4-point FDCT in pass 1 (rows), 2-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_4x2(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We must also scale the output by (8/4)*(8/2) = 2**3, which we add here. */
    /* 4-point FDCT kernel, */
    /* cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point FDCT]. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 2 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(2 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp0 + tmp1 - (4 as i32 * 128 as i32) as isize) << 2 as i32 + 3 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) =
            (tmp0 - tmp1 << 2 as i32 + 3 as i32) as crate::jdct_h::DCTELEM;
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 4 as i32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 3 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 3 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* c6 */
    /* Add fudge factor here for final descale. */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        /* Even part */
        /* Add fudge factor here for final descale. */
        tmp0 = *dataptr.offset((8 as i32 * 0 as i32) as isize) as isize
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 - 1 as i32);
        tmp1 = *dataptr.offset((8 as i32 * 1 as i32) as isize) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            (tmp0 + tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 - tmp1 >> 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/*
 * Perform the forward DCT on a 2x1 sample block.
 *
 * 2-point FDCT in pass 1 (rows), 1-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_2x1(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    elemptr = (*sample_data.offset(0 as i32 as isize)).offset(start_col as isize);
    tmp0 = *elemptr.offset(0 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
    tmp1 = *elemptr.offset(1 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
    /* We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/2)*(8/1) = 2**5.
     */
    /* Even part */
    /* Apply unsigned->signed conversion */
    *data.offset(0 as i32 as isize) = ((tmp0 + tmp1 - (2 as i32 * 128 as i32) as isize)
        << 5 as i32) as crate::jdct_h::DCTELEM;
    /* Odd part */
    *data.offset(1 as i32 as isize) = (tmp0 - tmp1 << 5 as i32) as crate::jdct_h::DCTELEM;
}
/*
 * Perform the forward DCT on an 8x16 sample block.
 *
 * 8-point FDCT in pass 1 (rows), 16-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_8x16(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp7: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut tmp17: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 64] = [0; 64];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part per LL&M figure 1 --- note that published figure is faulty;
         * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
         */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp3;
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(7 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = (*elemptr.offset(3 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11
            - (8 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = (tmp10 - tmp11 << 2 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) =
            (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) = (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part per figure 8 --- note paper omits factor of sqrt(2).
         * 8-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
         * i0..i3 in the paper are tmp0..tmp3 here.
         */
        tmp10 = tmp0 + tmp3; /*  c3 */
        tmp11 = tmp1 + tmp2; /*  c1+c3-c5-c7 */
        tmp12 = tmp0 + tmp2; /*  c1+c3+c5-c7 */
        tmp13 = tmp1 + tmp3; /*  c1+c3-c5+c7 */
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32; /* -c1+c3+c5-c7 */
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32; /*  c7-c3 */
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32; /* -c1-c3 */
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32; /*  c5-c3 */
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32; /* -c3-c5 */
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32); /* Done. */
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset(1 as i32 as isize) = (tmp0
            + tmp10
            + tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) = (tmp1
            + tmp11
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) = (tmp2
            + tmp11
            + tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(7 as i32 as isize) = (tmp3
            + tmp10
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 8 as i32 * 2 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by 8/16 = 1/2.
     * 16-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/32).
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 8 as i32 - 1 as i32;
    while ctr >= 0 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp7 = (*dataptr.offset((8 as i32 * 7 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp7;
        tmp14 = tmp0 - tmp7;
        tmp11 = tmp1 + tmp6;
        tmp15 = tmp1 - tmp6;
        tmp12 = tmp2 + tmp5;
        tmp16 = tmp2 - tmp5;
        tmp13 = tmp3 + tmp4;
        tmp17 = tmp3 - tmp4;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp7 = (*dataptr.offset((8 as i32 * 7 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = (tmp10
            + tmp11
            + tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 + 1 as i32 - 1 as i32)
            >> 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp13)
            * (1.306562965f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp12) * 4433 as i32 as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp17 - tmp15)
            * (0.275899379f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp14 - tmp16)
                * (1.387039845f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32; /* c2[16] = c1[8] */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp15
                * (1.451774982f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (2.172734804f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp14
                * (0.211164243f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp17
                * (1.061594338f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        tmp11 = (tmp0 + tmp1)
            * (1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp7)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 + tmp7)
                * (0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (1.093201867f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 - tmp7)
                * (0.897167586f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp14 = (tmp1 + tmp2)
            * (0.138617169f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp6 - tmp5)
                * (1.407403738f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp15 = (tmp1 + tmp3)
            * -((0.666655658f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp4 + tmp6)
                * -((1.247225013f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp16 = (tmp2 + tmp3)
            * -((1.353318001f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32)
            + (tmp5 - tmp4)
                * (0.410524528f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp11 + tmp12 + tmp13
            - tmp0
                * (2.286341144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp7
                * (0.779653625f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 += tmp14
            + tmp15
            + tmp1
                * (0.071888074f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (1.663905119f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 += tmp14 + tmp16
            - tmp2
                * (1.125726048f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (1.227391138f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp15
            + tmp16
            + tmp3
                * (1.065388962f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (2.167985692f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp11
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp12
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 + 2 as i32 + 1 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32 + 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr -= 1
    }
}
/* Odd part */
/* c13 */
/* c11 */
/* c9 */
/* c1 */
/* -c5 */
/* c13 */
/* c15+c13-c11+c9 */
/* c7+c13+c1-c5 */
/* c9-c11+c1-c13 */
/* c1+c13+c5-c9 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 7x14 sample block.
 *
 * 7-point FDCT in pass 1 (rows), 14-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_7x14(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp6: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut tmp16: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut z2: crate::jmorecfg_h::INT32 = 0;
    let mut z3: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 48] = [0; 48];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 7-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/14). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(6 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp3 = *elemptr.offset(3 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(6 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp12 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        z1 = tmp0 + tmp2;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((z1 + tmp1 + tmp3
            - (7 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM; /* (c2+c6-c4)/2 */
        tmp3 += tmp3; /* (c2+c4-c6)/2 */
        z1 -= tmp3; /* c6 */
        z1 -= tmp3; /* c4 */
        z1 = z1
            * (0.353553391f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z2 = (tmp0 - tmp2)
            * (0.920609002f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        z3 = (tmp1 - tmp2)
            * (0.314692123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) =
            (z1 + z2
                + z3
                + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        z1 -= z2;
        z2 = (tmp0 - tmp1)
            * (0.881747734f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(4 as i32 as isize) = (z2 + z3
            - (tmp1 - tmp3)
                * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(6 as i32 as isize) =
            (z1 + z2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp1 = (tmp10 + tmp11)
            * (0.935414347f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c3+c1-c5)/2 */
        tmp2 = (tmp10 - tmp11)
            * (0.170262339f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c3+c5-c1)/2 */
        tmp0 = tmp1 - tmp2; /* -c1 */
        tmp1 += tmp2; /* c5 */
        tmp2 = (tmp11 + tmp12)
            * -((1.378756276f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32); /* c3+c1-c5 */
        tmp1 += tmp2; /* Done. */
        tmp3 = (tmp10 + tmp12)
            * (0.613604268f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp0 += tmp3;
        tmp2 += tmp3
            + tmp12
                * (1.870828693f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp1 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp2 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
                >> 13 as i32 - 2 as i32) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 14 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/7)*(8/14) = 32/49, which we
     * fold into the constant multipliers:
     * 14-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/28) * 32/49.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 0 as i32;
    while ctr < 7 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp13 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp6;
        tmp14 = tmp0 - tmp6;
        tmp11 = tmp1 + tmp5;
        tmp15 = tmp1 - tmp5;
        tmp12 = tmp2 + tmp4;
        tmp16 = tmp2 - tmp4;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp6 = (*dataptr.offset((8 as i32 * 6 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12 + tmp13)
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp13 += tmp13;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp13)
            * (0.832106052f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp11 - tmp13)
                * (0.205513223f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp12 - tmp13)
                * (0.575835255f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp14 + tmp15)
            * (0.722074570f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp14
                * (0.178337691f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp16
                * (0.400721155f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp15
                * (1.122795725f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp16
                * (0.900412262f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp1 + tmp2;
        tmp11 = tmp5 - tmp4;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = ((tmp0 - tmp10 + tmp3 - tmp11 - tmp6)
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp3 = tmp3
            * (0.653061224f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp10
            * -((0.103406812f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp11 = tmp11
            * (0.917760839f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 += tmp11 - tmp3;
        tmp11 = (tmp0 + tmp2)
            * (0.782007410f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp4 + tmp6)
                * (0.491367823f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp10 + tmp11
            - tmp2
                * (1.550341076f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.731428202f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 + tmp1)
            * (0.871740478f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp5 - tmp6)
                * (0.305035186f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp10 + tmp12
            - tmp1
                * (0.276965844f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (2.004803435f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp11 + tmp12 + tmp3
            - tmp0
                * (0.735987049f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp6
                * (0.082925825f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* 32/49 */
/* -c13 */
/* c1 */
/* c9 */
/* c11 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 6x12 sample block.
 *
 * 6-point FDCT in pass 1 (rows), 12-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_6x12(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp5: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut tmp15: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 32] = [0; 32];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 6-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/12). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(5 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(2 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp11
            - (6 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = (tmp12
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = ((tmp10 - tmp11 - tmp11)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp10 = (tmp0 + tmp2)
            * (0.366025404f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32; /* Done. */
        *dataptr.offset(1 as i32 as isize) =
            (tmp10 + (tmp0 + tmp1 << 2 as i32)) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp1 - tmp2 << 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(5 as i32 as isize) =
            (tmp10 + (tmp2 - tmp1 << 2 as i32)) as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 12 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/6)*(8/12) = 8/9, which we
     * fold into the constant multipliers:
     * 12-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/24) * 8/9.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp5;
        tmp13 = tmp0 - tmp5;
        tmp11 = tmp1 + tmp4;
        tmp14 = tmp1 - tmp4;
        tmp12 = tmp2 + tmp3;
        tmp15 = tmp2 - tmp3;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp5 = (*dataptr.offset((8 as i32 * 5 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = ((tmp13 - tmp14 - tmp15)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp12)
            * (1.088662108f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = ((tmp14 - tmp15)
            * (0.888888889f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + (tmp13 + tmp15)
                * (1.214244803f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp1 + tmp4)
            * (0.481063200f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp14 = tmp10
            + tmp1
                * (0.680326102f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp15 = tmp10
            - tmp4
                * (1.642452502f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp12 = (tmp0 + tmp2)
            * (0.997307603f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp0 + tmp3)
            * (0.765261039f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp10 = tmp12 + tmp13 + tmp14
            - tmp0
                * (0.516244403f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.164081699f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = (tmp2 + tmp3)
            * -((0.164081699f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32);
        tmp12 += tmp11
            - tmp15
            - tmp2
                * (2.079550144f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp5
                * (0.765261039f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 += tmp11 - tmp14
            + tmp3
                * (0.645144899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - tmp5
                * (0.997307603f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp11 = tmp15
            + (tmp0 - tmp3)
                * (1.161389302f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp2 + tmp5)
                * (0.481063200f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp10 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp11 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp12 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp13 + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
                >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c9 */
/* c3-c9 */
/* c3+c9 */
/* c5 */
/* c7 */
/* c11 */
/* -c11 */
/* c7 */
/* c5 */
/* c9 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 5x10 sample block.
 *
 * 5-point FDCT in pass 1 (rows), 10-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_5x10(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp4: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut tmp14: crate::jmorecfg_h::INT32 = 0;
    let mut workspace: [crate::jdct_h::DCTELEM; 16] = [0; 16];
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut wsptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* 5-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/10). */
    dataptr = data;
    ctr = 0 as i32;
    loop {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* switch pointer to extended workspace */
        /* Even part */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp2 = *elemptr.offset(2 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp1;
        tmp11 = tmp0 - tmp1;
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(4 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        /* Apply unsigned->signed conversion */
        *dataptr.offset(0 as i32 as isize) = ((tmp10 + tmp2
            - (5 as i32 * 128 as i32) as isize)
            << 2 as i32) as crate::jdct_h::DCTELEM; /* (c2+c4)/2 */
        tmp11 = tmp11
            * (0.790569415f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* (c2-c4)/2 */
        tmp10 -= tmp2 << 2 as i32;
        tmp10 = tmp10
            * (0.353553391f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset(2 as i32 as isize) = (tmp11
            + tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(4 as i32 as isize) = (tmp11 - tmp10
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* Odd part */
        tmp10 = (tmp0 + tmp1)
            * (0.831253876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c3 */
        *dataptr.offset(1 as i32 as isize) = (tmp10
            + tmp0
                * (0.513743148f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM; /* Done. */
        *dataptr.offset(3 as i32 as isize) = (tmp10
            - tmp1
                * (2.176250899f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32)
            as crate::jdct_h::DCTELEM;
        ctr += 1;
        if ctr != 8 as i32 {
            if ctr == 10 as i32 {
                break;
            }
            dataptr = dataptr.offset(8 as i32 as isize)
        /* advance pointer to next row */
        } else {
            dataptr = workspace.as_mut_ptr()
        }
    }
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/5)*(8/10) = 32/25, which we
     * fold into the constant multipliers:
     * 10-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/20) * 32/25.
     */
    dataptr = data;
    wsptr = workspace.as_mut_ptr();
    ctr = 0 as i32;
    while ctr < 5 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp12 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp4;
        tmp13 = tmp0 - tmp4;
        tmp11 = tmp1 + tmp3;
        tmp14 = tmp1 - tmp3;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *wsptr.offset((8 as i32 * 1 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *wsptr.offset((8 as i32 * 0 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp4 = (*dataptr.offset((8 as i32 * 4 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11 + tmp12)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 += tmp12;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp12)
            * (1.464477191f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp11 - tmp12)
                * (0.559380511f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp13 + tmp14)
            * (1.064004961f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32; /* c6 */
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp10
            + tmp13
                * (0.657591230f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) = (tmp10
            - tmp14
                * (2.785601151f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp4;
        tmp11 = tmp1 - tmp3;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = ((tmp10 - tmp11 - tmp2)
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp2 = tmp2
            * (1.28f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp0
            * (1.787906876f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp1
                * (1.612894094f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp2
            + tmp3
                * (0.821810588f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp4
                * (0.283176630f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        tmp12 = (tmp0 - tmp4)
            * (1.217352341f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            - (tmp1 + tmp3)
                * (0.752365123f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32;
        tmp13 = (tmp10 + tmp11)
            * (0.395541753f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + tmp11
                * (0.64f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64 + 0.5f64)
                    as crate::jmorecfg_h::INT32
            - tmp2;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = (tmp12
            + tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) = (tmp12 - tmp13
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        wsptr = wsptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* 32/25 */
/* (c1-c9)/2 */
/* 16/25 */
/* advance pointer to next column */
/*
 * Perform the forward DCT on a 4x8 sample block.
 *
 * 4-point FDCT in pass 1 (rows), 8-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_4x8(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp3: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut tmp13: crate::jmorecfg_h::INT32 = 0;
    let mut z1: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We must also scale the output by 8/4 = 2, which we add here. */
    /* 4-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 8 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(3 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = (*elemptr.offset(1 as i32 as isize) as i32
            + *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp10 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(3 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        tmp11 = (*elemptr.offset(1 as i32 as isize) as i32
            - *elemptr.offset(2 as i32 as isize) as i32)
            as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp0 + tmp1 - (4 as i32 * 128 as i32) as isize) << 2 as i32 + 1 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) =
            (tmp0 - tmp1 << 2 as i32 + 1 as i32) as crate::jdct_h::DCTELEM;
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 2 as i32 - 2 as i32;
        *dataptr.offset(1 as i32 as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(3 as i32 as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32
                >> 13 as i32 - 2 as i32 - 1 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* c6 */
    /* Add fudge factor here for final descale. */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        /* Even part per LL&M figure 1 --- note that published figure is faulty;
         * rotator "sqrt(2)*c1" should be "sqrt(2)*c6".
         */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        /* advance pointer to next column */
        tmp10 = tmp0 + tmp3 + ((1 as i32 as crate::jmorecfg_h::INT32) << 2 as i32 - 1 as i32);
        tmp12 = tmp0 - tmp3;
        tmp11 = tmp1 + tmp2;
        tmp13 = tmp1 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 7 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 6 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp3 = (*dataptr.offset((8 as i32 * 3 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) =
            (tmp10 + tmp11 >> 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) =
            (tmp10 - tmp11 >> 2 as i32) as crate::jdct_h::DCTELEM;
        z1 = (tmp12 + tmp13) * 4433 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) =
            (z1 + tmp12 * 6270 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 6 as i32) as isize) =
            (z1 - tmp13 * 15137 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32 + 2 as i32)
                as crate::jdct_h::DCTELEM;
        tmp10 = tmp0 + tmp3;
        tmp11 = tmp1 + tmp2;
        tmp12 = tmp0 + tmp2;
        tmp13 = tmp1 + tmp3;
        z1 = (tmp12 + tmp13) * 9633 as i32 as crate::jmorecfg_h::INT32;
        z1 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32;
        tmp0 = tmp0 * 12299 as i32 as crate::jmorecfg_h::INT32;
        tmp1 = tmp1 * 25172 as i32 as crate::jmorecfg_h::INT32;
        tmp2 = tmp2 * 16819 as i32 as crate::jmorecfg_h::INT32;
        tmp3 = tmp3 * 2446 as i32 as crate::jmorecfg_h::INT32;
        tmp10 = tmp10 * -(7373 as i32 as crate::jmorecfg_h::INT32);
        tmp11 = tmp11 * -(20995 as i32 as crate::jmorecfg_h::INT32);
        tmp12 = tmp12 * -(3196 as i32 as crate::jmorecfg_h::INT32);
        tmp13 = tmp13 * -(16069 as i32 as crate::jmorecfg_h::INT32);
        tmp12 += z1;
        tmp13 += z1;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + tmp10 + tmp12 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp1 + tmp11 + tmp13 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) =
            (tmp2 + tmp11 + tmp12 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 7 as i32) as isize) =
            (tmp3 + tmp10 + tmp13 >> 13 as i32 + 2 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Add fudge factor here for final descale. */
/* Add fudge factor here for final descale. */
/* Odd part per figure 8 --- note paper omits factor of sqrt(2).
 * 8-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/16).
 * i0..i3 in the paper are tmp0..tmp3 here.
 */
/*  c3 */
/* Add fudge factor here for final descale. */
/*  c1+c3-c5-c7 */
/*  c1+c3+c5-c7 */
/*  c1+c3-c5+c7 */
/* -c1+c3+c5-c7 */
/*  c7-c3 */
/* -c1-c3 */
/*  c5-c3 */
/* -c3-c5 */
/*
 * Perform the forward DCT on a 3x6 sample block.
 *
 * 3-point FDCT in pass 1 (rows), 6-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_3x6(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp2: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut tmp12: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT; */
    /* furthermore, we scale the results by 2**PASS1_BITS. */
    /* We scale the results further by 2 as part of output adaption */
    /* scaling for different DCT size. */
    /* 3-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/6). */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 6 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = (*elemptr.offset(0 as i32 as isize) as i32
            + *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        tmp1 = *elemptr.offset(1 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp2 = (*elemptr.offset(0 as i32 as isize) as i32
            - *elemptr.offset(2 as i32 as isize) as i32) as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) =
            ((tmp0 + tmp1 - (3 as i32 * 128 as i32) as isize) << 2 as i32 + 1 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset(2 as i32 as isize) = ((tmp0 - tmp1 - tmp1)
            * (0.707106781f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset(1 as i32 as isize) = (tmp2
            * (1.224744871f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32)
                << 13 as i32 - 2 as i32 - 1 as i32 - 1 as i32)
            >> 13 as i32 - 2 as i32 - 1 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* Pass 2: process columns.
     * We remove the PASS1_BITS scaling, but leave the results scaled up
     * by an overall factor of 8.
     * We must also scale the output by (8/6)*(8/3) = 32/9, which we partially
     * fold into the constant multipliers (other part was done in pass 1):
     * 6-point FDCT kernel, cK represents sqrt(2) * cos(K*pi/12) * 16/9.
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 3 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = tmp0 + tmp2;
        tmp12 = tmp0 - tmp2;
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 5 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 4 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp2 = (*dataptr.offset((8 as i32 * 2 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = ((tmp10 + tmp11)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp12
            * (2.177324216f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 4 as i32) as isize) = ((tmp10 - tmp11 - tmp11)
            * (1.257078722f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp10 = (tmp0 + tmp2)
            * (0.650711829f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) = (tmp10
            + (tmp0 + tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) = ((tmp0 - tmp1 - tmp2)
            * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 5 as i32) as isize) = (tmp10
            + (tmp2 - tmp1)
                * (1.777777778f64 * ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32) as f64
                    + 0.5f64) as crate::jmorecfg_h::INT32
            + ((1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 + 2 as i32 - 1 as i32)
            >> 13 as i32 + 2 as i32)
            as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c5 */
/*
 * Perform the forward DCT on a 2x4 sample block.
 *
 * 2-point FDCT in pass 1 (rows), 4-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_2x4(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    let mut tmp10: crate::jmorecfg_h::INT32 = 0;
    let mut tmp11: crate::jmorecfg_h::INT32 = 0;
    let mut dataptr: *mut crate::jdct_h::DCTELEM = 0 as *mut crate::jdct_h::DCTELEM;
    let mut elemptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ctr: i32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    /* Pass 1: process rows. */
    /* Note results are scaled up by sqrt(8) compared to a true DCT. */
    /* We must also scale the output by (8/2)*(8/4) = 2**3, which we add here. */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 4 as i32 {
        elemptr = (*sample_data.offset(ctr as isize)).offset(start_col as isize);
        /* advance pointer to next row */
        tmp0 = *elemptr.offset(0 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        tmp1 = *elemptr.offset(1 as i32 as isize) as i32 as crate::jmorecfg_h::INT32;
        *dataptr.offset(0 as i32 as isize) = ((tmp0 + tmp1
            - (2 as i32 * 128 as i32) as isize)
            << 3 as i32) as crate::jdct_h::DCTELEM;
        *dataptr.offset(1 as i32 as isize) = (tmp0 - tmp1 << 3 as i32) as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(8 as i32 as isize);
        ctr += 1
    }
    /* Even part */
    /* Apply unsigned->signed conversion */
    /* Odd part */
    /* Pass 2: process columns.
     * We leave the results scaled up by an overall factor of 8.
     * 4-point FDCT kernel,
     * cK represents sqrt(2) * cos(K*pi/16) [refers to 8-point FDCT].
     */
    dataptr = data;
    ctr = 0 as i32;
    while ctr < 2 as i32 {
        /* Even part */
        tmp0 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            + *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp1 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            + *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp10 = (*dataptr.offset((8 as i32 * 0 as i32) as isize)
            - *dataptr.offset((8 as i32 * 3 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        tmp11 = (*dataptr.offset((8 as i32 * 1 as i32) as isize)
            - *dataptr.offset((8 as i32 * 2 as i32) as isize))
            as crate::jmorecfg_h::INT32;
        *dataptr.offset((8 as i32 * 0 as i32) as isize) = (tmp0 + tmp1) as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 2 as i32) as isize) = (tmp0 - tmp1) as crate::jdct_h::DCTELEM;
        /* advance pointer to next column */
        tmp0 = (tmp10 + tmp11) * 4433 as i32 as crate::jmorecfg_h::INT32;
        tmp0 += (1 as i32 as crate::jmorecfg_h::INT32) << 13 as i32 - 1 as i32;
        *dataptr.offset((8 as i32 * 1 as i32) as isize) =
            (tmp0 + tmp10 * 6270 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32)
                as crate::jdct_h::DCTELEM;
        *dataptr.offset((8 as i32 * 3 as i32) as isize) =
            (tmp0 - tmp11 * 15137 as i32 as crate::jmorecfg_h::INT32 >> 13 as i32)
                as crate::jdct_h::DCTELEM;
        dataptr = dataptr.offset(1);
        ctr += 1
    }
}
/* Odd part */
/* c6 */
/* Add fudge factor here for final descale. */
/*
 * Perform the forward DCT on a 1x2 sample block.
 *
 * 1-point FDCT in pass 1 (rows), 2-point in pass 2 (columns).
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_fdct_1x2(
    mut data: *mut crate::jdct_h::DCTELEM,
    mut sample_data: crate::jpeglib_h::JSAMPARRAY,
    mut start_col: crate::jmorecfg_h::JDIMENSION,
) {
    let mut tmp0: crate::jmorecfg_h::INT32 = 0;
    let mut tmp1: crate::jmorecfg_h::INT32 = 0;
    /* Pre-zero output coefficient block. */
    crate::stdlib::memset(
        data as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<crate::jdct_h::DCTELEM>() as libc::c_ulong)
            .wrapping_mul(64 as i32 as libc::c_ulong),
    );
    tmp0 = *(*sample_data.offset(0 as i32 as isize)).offset(start_col as isize) as i32
        as crate::jmorecfg_h::INT32;
    tmp1 = *(*sample_data.offset(1 as i32 as isize)).offset(start_col as isize) as i32
        as crate::jmorecfg_h::INT32;
    /* We leave the results scaled up by an overall factor of 8.
     * We must also scale the output by (8/1)*(8/2) = 2**5.
     */
    /* Even part */
    /* Apply unsigned->signed conversion */
    *data.offset((8 as i32 * 0 as i32) as isize) = ((tmp0 + tmp1
        - (2 as i32 * 128 as i32) as isize)
        << 5 as i32) as crate::jdct_h::DCTELEM;
    /* Odd part */
    *data.offset((8 as i32 * 1 as i32) as isize) =
        (tmp0 - tmp1 << 5 as i32) as crate::jdct_h::DCTELEM;
}
/* DCT_ISLOW_SUPPORTED */
/* DCT_SCALING_SUPPORTED */

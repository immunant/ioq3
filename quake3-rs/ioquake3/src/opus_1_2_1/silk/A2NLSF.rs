pub mod SigProc_FIX_h {
    /* compute whitening filter coefficients from normalized line spectral frequencies */
    /* O    monic whitening filter coefficients in Q12,  [ d ]          */
    /* I    normalized line spectral frequencies in Q15, [ d ]          */
    /* I    filter order (should be even)                               */
    /* I    Run-time architecture                                       */
    /* Convert int32 coefficients to int16 coefs and make sure there's no wrap-around */
    /* O    Output signal                                               */
    /* I/O  Input signal                                                */
    /* I    Input Q domain                                              */
    /* I    Input Q domain                                              */
    /* I    Filter order                                                */
    /* I/O   Unsorted / Sorted vector                                   */
    /* O     Index vector for the sorted elements                       */
    /* I     Vector length                                              */
    /* I     Number of correctly sorted positions                       */
    /* I/O   Unsorted / Sorted vector                                   */
    /* O     Index vector for the sorted elements                       */
    /* I     Vector length                                              */
    /* I     Number of correctly sorted positions                       */
    /* I/O   Unsorted / Sorted vector                                   */
    /* I     Vector length                                              */
    /* NLSF stabilizer, for a single input data vector */
    /* I/O   Unstable/stabilized normalized LSF vector in Q15 [L]       */
    /* I     Min distance vector, NDeltaMin_Q15[L] must be >= 1 [L+1]   */
    /* I     Number of NLSF parameters in the input vector              */
    /* Laroia low complexity NLSF weights */
    /* O     Pointer to input vector weights [D]                        */
    /* I     Pointer to input vector         [D]                        */
    /* I     Input vector dimension (even)                              */
    /* Compute reflection coefficients from input signal */
    /* O    Residual energy                                             */
    /* O    Residual energy Q value                                     */
    /* O    Prediction coefficients (length order)                      */
    /* I    Input signal, length: nb_subfr * ( D + subfr_length )       */
    /* I    Inverse of max prediction gain                              */
    /* I    Input signal subframe length (incl. D preceding samples)    */
    /* I    Number of subframes stacked in x                            */
    /* I    Order                                                       */
    /* I    Run-time architecture                                       */
    /* Copy and multiply a vector by a constant */
    /* I    Gain in Q16                                                 */
    /* I    Length                                                      */
    /* Some for the LTP related function requires Q26 to work.*/
    /* I/O  Q0/Q18                                                      */
    /* I    Q26                                                         */
    /* I    length                                                      */
    /* *******************************************************************/
    /*                        INLINE ARM MATH                           */
    /* *******************************************************************/
    /*    return sum( inVec1[i] * inVec2[i] ) */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I vector lengths                                              */
    /*    I Run-time architecture                                       */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I number of bits to shift                                     */
    /*    I vector lengths                                              */
    /*    I input vector 1                                              */
    /*    I input vector 2                                              */
    /*    I vector lengths                                              */
    /* *******************************************************************/
    /*                                MACROS                            */
    /* *******************************************************************/
    /* Rotate a32 right by 'rot' bits. Negative rot values result in rotating
    left. Output is 32bit int.
    Note: contemporary compilers recognize the C expression below and
    compile it into a 'ror' instruction if available. No need for OPUS_INLINE ASM! */
    /* Allocate opus_int16 aligned to 4-byte memory address */
    /* Useful Macros that can be adjusted to other platforms */
    /* Fixed point macros */
    /* (a32 * b32) output have to be 32bit int */
    /* (a32 * b32) output have to be 32bit uint */
    /* a32 + (b32 * c32) output have to be 32bit int */
    /* a32 + (b32 * c32) output have to be 32bit uint */
    /* ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* a32 + ((a32 >> 16)  * (b32 >> 16)) output have to be 32bit int */
    /* (a32 * b32) */
    /*(opus_int64)*/
    /* Adds two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Subtractss two signed 32-bit values in a way that can overflow, while not relying on undefined behaviour
    (just standard two's complement implementation-specific behaviour) */
    /* Multiply-accumulate macros that allow overflow in the addition (ie, no asserts in debug mode) */
    /* These macros enables checking for overflow in silk_API_Debug.h*/
    /* Saturation for positive input values */
    /* Add with saturation for positive input values */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 8  */
    /* shift >= 0, shift < 16 */
    /* shift >= 0, shift < 32 */
    /* shift >= 0, shift < 64 */
    /* shift >= 0, shift < 32 */
    /* saturates before shifting */
    /* shift >= 0, allowed to overflow */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* shift >= 0 */
    /* Requires that shift > 0 */
    /* Number of rightshift required to fit the multiplication */
    /* Macro to convert floating-point constants to fixed-point */
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_min_32(
        mut a: crate::opus_types_h::opus_int32,
        mut b: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return if a < b { a } else { b };
    }

    /* SILK_SIGPROC_FIX_H */
    /*    silk_SMMUL: Signed top word multiply.
    ARMv6        2 instruction cycles.
    ARMv3M+      3 instruction cycles. use SMULL and ignore LSB registers.(except xM)*/
    /*#define silk_SMMUL(a32, b32)                (opus_int32)silk_RSHIFT(silk_SMLAL(silk_SMULWB((a32), (b32)), (a32), silk_RSHIFT_ROUND((b32), 16)), 16)*/
    /* the following seems faster on x86 */
    /*  Add some multiplication functions that can be easily mapped to ARM. */
    /* PSEUDO-RANDOM GENERATOR                                                          */
    /* Make sure to store the result as the seed for the next call (also in between     */
    /* frames), otherwise result won't be random at all. When only using some of the    */
    /* bits, take the most significant bits by right-shifting.                          */
    /* Be careful, silk_abs returns wrong when input equals to silk_intXX_MIN */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::silk::bwexpander_32::silk_bwexpander_32;

pub use crate::src::opus_1_2_1::silk::A2NLSF::SigProc_FIX_h::silk_min_32;
/* Helper function for A2NLSF(..)                    */
/* Transforms polynomials from cos(n*f) to cos(f)^n  */
#[inline]

unsafe extern "C" fn silk_A2NLSF_trans_poly(mut p: *mut opus_int32, dd: i32)
/* I      Polynomial order (= filter order / 2 )    */
{
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    k = 2 as i32;
    while k <= dd {
        n = dd;
        while n > k {
            let ref mut fresh0 = *p.offset((n - 2 as i32) as isize);
            *fresh0 -= *p.offset(n as isize);
            n -= 1
        }
        let ref mut fresh1 = *p.offset((k - 2 as i32) as isize);
        *fresh1 -= ((*p.offset(k as isize) as opus_uint32) << 1 as i32)
            as opus_int32;
        k += 1
    }
}
/* Helper function for A2NLSF(..) */
/* Polynomial evaluation          */
#[inline]

unsafe extern "C" fn silk_A2NLSF_eval_poly(
    mut p: *mut opus_int32,
    x: opus_int32,
    dd: i32,
) -> opus_int32
/* I    Order                                   */ {
    let mut n: i32 = 0; /* Q16 */
    let mut x_Q16: opus_int32 = 0;
    let mut y32: opus_int32 = 0;
    y32 = *p.offset(dd as isize);
    x_Q16 =
        ((x as opus_uint32) << 4 as i32) as opus_int32;
    if (8 as i32 == dd) as i32 as isize != 0 {
        y32 = (*p.offset(7 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(6 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(5 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(4 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(3 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(2 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(1 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32;
        y32 = (*p.offset(0 as i32 as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
            as opus_int32
    } else {
        n = dd - 1 as i32;
        while n >= 0 as i32 {
            y32 = (*p.offset(n as isize) as i64 + (y32 as i64 * x_Q16 as i64 >> 16 as i32))
                as opus_int32;
            n -= 1
            /* Q16 */
        }
    }
    return y32;
}
#[inline]

unsafe extern "C" fn silk_A2NLSF_init(
    mut a_Q16: *const opus_int32,
    mut P: *mut opus_int32,
    mut Q: *mut opus_int32,
    dd: i32,
) {
    let mut k: i32 = 0;
    /* Convert filter coefs to even and odd polynomials */
    *P.offset(dd as isize) = ((1 as i32 as opus_uint32) << 16 as i32)
        as opus_int32; /* Q16 */
    *Q.offset(dd as isize) = ((1 as i32 as opus_uint32) << 16 as i32)
        as opus_int32;
    k = 0 as i32;
    while k < dd {
        *P.offset(k as isize) =
            -*a_Q16.offset((dd - k - 1 as i32) as isize) - *a_Q16.offset((dd + k) as isize);
        *Q.offset(k as isize) =
            -*a_Q16.offset((dd - k - 1 as i32) as isize) + *a_Q16.offset((dd + k) as isize);
        k += 1
        /* Q16 */
    }
    /* Divide out zeros as we have that for even filter orders, */
    /* z =  1 is always a root in Q, and                        */
    /* z = -1 is always a root in P                             */
    k = dd;
    while k > 0 as i32 {
        let ref mut fresh2 = *P.offset((k - 1 as i32) as isize);
        *fresh2 -= *P.offset(k as isize);
        let ref mut fresh3 = *Q.offset((k - 1 as i32) as isize);
        *fresh3 += *Q.offset(k as isize);
        k -= 1
    }
    /* Transform polynomials from cos(n*f) to cos(f)^n */
    silk_A2NLSF_trans_poly(P, dd);
    silk_A2NLSF_trans_poly(Q, dd);
}
/* Compute Normalized Line Spectral Frequencies (NLSFs) from whitening filter coefficients      */
/* If not all roots are found, the a_Q16 coefficients are bandwidth expanded until convergence. */
/* Compute Normalized Line Spectral Frequencies (NLSFs) from whitening filter coefficients      */
/* If not all roots are found, the a_Q16 coefficients are bandwidth expanded until convergence. */
#[no_mangle]

pub unsafe extern "C" fn silk_A2NLSF(
    mut NLSF: *mut opus_int16,
    mut a_Q16: *mut opus_int32,
    d: i32,
)
/* I    Filter order (must be even)                                 */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut dd: i32 = 0;
    let mut root_ix: i32 = 0;
    let mut ffrac: i32 = 0;
    let mut xlo: opus_int32 = 0;
    let mut xhi: opus_int32 = 0;
    let mut xmid: opus_int32 = 0;
    let mut ylo: opus_int32 = 0;
    let mut yhi: opus_int32 = 0;
    let mut ymid: opus_int32 = 0;
    let mut thr: opus_int32 = 0;
    let mut nom: opus_int32 = 0;
    let mut den: opus_int32 = 0;
    let mut P: [opus_int32; 13] = [0; 13];
    let mut Q: [opus_int32; 13] = [0; 13];
    let mut PQ: [*mut opus_int32; 2] =
        [0 as *mut opus_int32; 2];
    let mut p: *mut opus_int32 = 0 as *mut opus_int32;
    /* Store pointers to array */
    PQ[0 as i32 as usize] = P.as_mut_ptr();
    PQ[1 as i32 as usize] = Q.as_mut_ptr();
    dd = d >> 1 as i32;
    silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
    /* Find roots, alternating between P and Q */
    p = P.as_mut_ptr(); /* Pointer to polynomial */
    xlo = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12[0 as i32 as usize]
        as opus_int32; /* Q12*/
    ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
    if ylo < 0 as i32 {
        /* Set the first NLSF to zero and move on to the next */
        *NLSF.offset(0 as i32 as isize) = 0 as i32 as opus_int16;
        /* Index of current root */
        p = Q.as_mut_ptr(); /* Pointer to polynomial */
        ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
        root_ix = 1 as i32
    } else {
        root_ix = 0 as i32
        /* Index of current root */
    } /* Loop counter */
    k = 1 as i32; /* Counter for bandwidth expansions applied */
    i = 0 as i32;
    thr = 0 as i32;
    loop {
        /* Evaluate polynomial */
        xhi = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12[k as usize]
            as opus_int32; /* Q12 */
        yhi = silk_A2NLSF_eval_poly(p, xhi, dd);
        /* Detect zero crossing */
        if ylo <= 0 as i32 && yhi >= thr || ylo >= 0 as i32 && yhi <= -thr {
            if yhi == 0 as i32 {
                /* If the root lies exactly at the end of the current       */
                /* interval, look for the next root in the next interval    */
                thr = 1 as i32
            } else {
                thr = 0 as i32
            }
            /* Binary division */
            ffrac = -(256 as i32);
            m = 0 as i32;
            while m < 3 as i32 {
                /* Evaluate polynomial */
                xmid = if 1 as i32 == 1 as i32 {
                    (xlo + xhi >> 1 as i32) + (xlo + xhi & 1 as i32)
                } else {
                    ((xlo + xhi >> 1 as i32 - 1 as i32) + 1 as i32) >> 1 as i32
                };
                ymid = silk_A2NLSF_eval_poly(p, xmid, dd);
                /* Detect zero crossing */
                if ylo <= 0 as i32 && ymid >= 0 as i32 || ylo >= 0 as i32 && ymid <= 0 as i32 {
                    /* Reduce frequency */
                    xhi = xmid;
                    yhi = ymid
                } else {
                    /* Increase frequency */
                    xlo = xmid;
                    ylo = ymid;
                    ffrac = ffrac + (128 as i32 >> m)
                }
                m += 1
            }
            /* Interpolate */
            if (if ylo > 0 as i32 { ylo } else { -ylo }) < 65536 as i32 {
                /* Avoid dividing by zero */
                den = ylo - yhi;
                nom = ((ylo as opus_uint32) << 8 as i32 - 3 as i32)
                    as opus_int32
                    + (den >> 1 as i32);
                if den != 0 as i32 {
                    ffrac += nom / den
                }
            } else {
                /* No risk of dividing by zero because abs(ylo - yhi) >= abs(ylo) >= 65536 */
                ffrac += ylo / (ylo - yhi >> 8 as i32 - 3 as i32)
            } /* Next root */
            *NLSF.offset(root_ix as isize) = silk_min_32(
                ((k as opus_uint32) << 8 as i32)
                    as opus_int32
                    + ffrac,
                0x7fff as i32,
            ) as opus_int16;
            root_ix += 1;
            if root_ix >= d {
                break;
            }
            /* Alternate pointer to polynomial */
            p = PQ[(root_ix & 1 as i32) as usize];
            /* Evaluate polynomial */
            xlo = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12
                [(k - 1 as i32) as usize] as opus_int32; /* Q12*/
            ylo = (((1 as i32 - (root_ix & 2 as i32)) as opus_uint32)
                << 12 as i32) as opus_int32
        } else {
            /* Increment loop counter */
            k += 1;
            xlo = xhi;
            ylo = yhi;
            thr = 0 as i32;
            if k > 128 as i32 {
                i += 1;
                if i > 16 as i32 {
                    /* Reset loop counter */
                    /* Set NLSFs to white spectrum and exit */
                    *NLSF.offset(0 as i32 as isize) = (((1 as i32) << 15 as i32) / (d + 1 as i32))
                        as opus_int16;
                    k = 1 as i32;
                    while k < d {
                        *NLSF.offset(k as isize) = (*NLSF.offset((k - 1 as i32) as isize) as i32
                            + *NLSF.offset(0 as i32 as isize) as i32)
                            as opus_int16;
                        k += 1
                    }
                    return;
                }
                silk_bwexpander_32(
                    a_Q16,
                    d,
                    65536 as i32
                        - ((1 as i32 as opus_uint32) << i)
                            as opus_int32,
                );
                silk_A2NLSF_init(a_Q16, P.as_mut_ptr(), Q.as_mut_ptr(), dd);
                p = P.as_mut_ptr();
                xlo = crate::src::opus_1_2_1::silk::table_LSF_cos::silk_LSFCosTab_FIX_Q12
                    [0 as i32 as usize] as opus_int32;
                ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
                if ylo < 0 as i32 {
                    /* Error: Apply progressively more bandwidth expansion and run again */
                    /* Pointer to polynomial */
                    /* Q12*/
                    /* Set the first NLSF to zero and move on to the next */
                    *NLSF.offset(0 as i32 as isize) = 0 as i32 as opus_int16;
                    /* Index of current root */
                    p = Q.as_mut_ptr(); /* Pointer to polynomial */
                    ylo = silk_A2NLSF_eval_poly(p, xlo, dd);
                    root_ix = 1 as i32
                } else {
                    root_ix = 0 as i32
                    /* Index of current root */
                }
                k = 1 as i32
            }
        }
    }
}

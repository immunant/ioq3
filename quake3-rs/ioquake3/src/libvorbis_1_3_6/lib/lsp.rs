use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::stdlib::__compar_fn_t;

pub use crate::stdlib::qsort;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function: LSP (also called LSF) conversion routines

 The LSP generation code is taken (with minimal modification and a
 few bugfixes) from "On the Computation of the LSP Frequencies" by
 Joseph Rothweiler (see http://www.rothweiler.us for contact info).
 The paper is available at:

 http://www.myown1.com/joe/lsf

********************************************************************/
/* Note that the lpc-lsp conversion finds the roots of polynomial with
an iterative root polisher (CACM algorithm 283).  It *is* possible
to confuse this algorithm into not converging; that should only
happen with absurdly closely spaced roots (very sharp peaks in the
LPC f response) which in turn should be impossible in our use of
the code.  If this *does* happen anyway, it's a bug in the floor
finder; find the cause of the confusion (probably a single bin
spike or accidental near-float-limit resolution problems) and
correct it. */
/* three possible LSP to f curve functions; the exact computation
(float), a lookup based float implementation, and an integer
implementation.  The float lookup is likely the optimal choice on
any machine with an FPU.  The integer implementation is *not* fixed
point (due to the need for a large dynamic range and thus a
separately tracked exponent) and thus much more complex than the
relatively simple float implementations. It's mostly for future
work on a fully fixed point implementation for processors like the
ARM family. */
/* define either of these (preferably FLOAT_LOOKUP) to have faster
but less precise implementation. */
/* old, nonoptimized but simple version for any poor sap who needs to
figure out what the hell this code does, or wants the other
fraction of a dB precision */
/* side effect: changes *lsp to cosines of lsp */
#[no_mangle]

pub unsafe extern "C" fn vorbis_lsp_to_curve(
    mut curve: *mut libc::c_float,
    mut map: *mut libc::c_int,
    mut n: libc::c_int,
    mut ln: libc::c_int,
    mut lsp: *mut libc::c_float,
    mut m: libc::c_int,
    mut amp: libc::c_float,
    mut ampoffset: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut wdel: libc::c_float =
        (3.14159265358979323846f64 / ln as libc::c_double) as libc::c_float;
    i = 0 as libc::c_int;
    while i < m {
        *lsp.offset(i as isize) = (2.0f32 as libc::c_double
            * crate::stdlib::cos(*lsp.offset(i as isize) as libc::c_double))
            as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = *map.offset(i as isize);
        let mut p: libc::c_float = 0.5f32;
        let mut q: libc::c_float = 0.5f32;
        let mut w: libc::c_float = (2.0f32 as libc::c_double
            * crate::stdlib::cos((wdel * k as libc::c_float) as libc::c_double))
            as libc::c_float;
        j = 1 as libc::c_int;
        while j < m {
            q *= w - *lsp.offset((j - 1 as libc::c_int) as isize);
            p *= w - *lsp.offset(j as isize);
            j += 2 as libc::c_int
        }
        if j == m {
            /* odd order filter; slightly assymetric */
            /* the last coefficient */
            q *= w - *lsp.offset((j - 1 as libc::c_int) as isize);
            p *= p * (4.0f32 - w * w);
            q *= q
        } else {
            /* even order filter; still symmetric */
            p *= p * (2.0f32 - w);
            q *= q * (2.0f32 + w)
        }
        q = crate::stdlib::exp(
            (amp as libc::c_double / crate::stdlib::sqrt((p + q) as libc::c_double)
                - ampoffset as libc::c_double)
                * 0.11512925f32 as libc::c_double,
        ) as libc::c_float;
        *curve.offset(i as isize) *= q;
        loop {
            i += 1;
            if !(*map.offset(i as isize) == k) {
                break;
            }
            *curve.offset(i as isize) *= q
        }
    }
}

unsafe extern "C" fn cheby(mut g: *mut libc::c_float, mut ord: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    *g.offset(0 as libc::c_int as isize) *= 0.5f32;
    i = 2 as libc::c_int;
    while i <= ord {
        j = ord;
        while j >= i {
            *g.offset((j - 2 as libc::c_int) as isize) -= *g.offset(j as isize);
            *g.offset(j as isize) += *g.offset(j as isize);
            j -= 1
        }
        i += 1
    }
}

unsafe extern "C" fn comp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> libc::c_int {
    return ((*(a as *mut libc::c_float)) < *(b as *mut libc::c_float)) as libc::c_int
        - (*(a as *mut libc::c_float) > *(b as *mut libc::c_float)) as libc::c_int;
}

unsafe extern "C" fn Laguerre_With_Deflation(
    mut a: *mut libc::c_float,
    mut ord: libc::c_int,
    mut r: *mut libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((ord + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut defl: *mut libc::c_double = fresh0.as_mut_ptr() as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i <= ord {
        *defl.offset(i as isize) = *a.offset(i as isize) as libc::c_double;
        i += 1
    }
    m = ord;
    while m > 0 as libc::c_int {
        let mut new: libc::c_double = 0.0f32 as libc::c_double;
        let mut delta: libc::c_double = 0.;
        loop
        /* iterate a root */
        {
            let mut p: libc::c_double = *defl.offset(m as isize);
            let mut pp: libc::c_double = 0.0f32 as libc::c_double;
            let mut ppp: libc::c_double = 0.0f32 as libc::c_double;
            let mut denom: libc::c_double = 0.;
            /* eval the polynomial and its first two derivatives */
            i = m;
            while i > 0 as libc::c_int {
                ppp = new * ppp + pp;
                pp = new * pp + p;
                p = new * p + *defl.offset((i - 1 as libc::c_int) as isize);
                i -= 1
            }
            /* Laguerre's method */
            denom = (m - 1 as libc::c_int) as libc::c_double
                * ((m - 1 as libc::c_int) as libc::c_double * pp * pp
                    - m as libc::c_double * p * ppp); /* complex root!  The LPC generator handed us a bad filter */
            if denom < 0 as libc::c_int as libc::c_double {
                return -(1 as libc::c_int);
            }
            if pp > 0 as libc::c_int as libc::c_double {
                denom = pp + crate::stdlib::sqrt(denom);
                if denom < 10e-7f64 {
                    denom = 10e-7f64
                }
            } else {
                denom = pp - crate::stdlib::sqrt(denom);
                if denom > -10e-7f64 {
                    denom = -10e-7f64
                }
            }
            delta = m as libc::c_double * p / denom;
            new -= delta;
            if delta < 0.0f32 as libc::c_double {
                delta *= -(1 as libc::c_int) as libc::c_double
            }
            if crate::stdlib::fabs(delta / new) < 10e-12f64 {
                break;
            }
        }
        *r.offset((m - 1 as libc::c_int) as isize) = new as libc::c_float;
        /* forward deflation */
        i = m;
        while i > 0 as libc::c_int {
            *defl.offset((i - 1 as libc::c_int) as isize) += new * *defl.offset(i as isize);
            i -= 1
        }
        defl = defl.offset(1);
        m -= 1
    }
    return 0 as libc::c_int;
}
/* for spit-and-polish only */

unsafe extern "C" fn Newton_Raphson(
    mut a: *mut libc::c_float,
    mut ord: libc::c_int,
    mut r: *mut libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut error: libc::c_double = 1.0f32 as libc::c_double;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (ord as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            as usize,
    );
    let mut root: *mut libc::c_double = fresh1.as_mut_ptr() as *mut libc::c_double;
    i = 0 as libc::c_int;
    while i < ord {
        *root.offset(i as isize) = *r.offset(i as isize) as libc::c_double;
        i += 1
    }
    while error > 1e-20f64 {
        error = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < ord {
            /* Update each point. */
            let mut pp: libc::c_double = 0.0f64;
            let mut delta: libc::c_double = 0.;
            let mut rooti: libc::c_double = *root.offset(i as isize);
            let mut p: libc::c_double = *a.offset(ord as isize) as libc::c_double;
            k = ord - 1 as libc::c_int;
            while k >= 0 as libc::c_int {
                pp = pp * rooti + p;
                p = p * rooti + *a.offset(k as isize) as libc::c_double;
                k -= 1
            }
            delta = p / pp;
            *root.offset(i as isize) -= delta;
            error += delta * delta;
            i += 1
        }
        if count > 40 as libc::c_int {
            return -(1 as libc::c_int);
        }
        count += 1
    }
    /* Replaced the original bubble sort with a real sort.  With your
    help, we can eliminate the bubble sort in our lifetime. --Monty */
    i = 0 as libc::c_int;
    while i < ord {
        *r.offset(i as isize) = *root.offset(i as isize) as libc::c_float;
        i += 1
    }
    return 0 as libc::c_int;
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function: LSP (also called LSF) conversion routines

********************************************************************/
/* Convert lpc coefficients to lsp coefficients */
#[no_mangle]

pub unsafe extern "C" fn vorbis_lpc_to_lsp(
    mut lpc: *mut libc::c_float,
    mut lsp: *mut libc::c_float,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut order2: libc::c_int = m + 1 as libc::c_int >> 1 as libc::c_int;
    let mut g1_order: libc::c_int = 0;
    let mut g2_order: libc::c_int = 0;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut g1: *mut libc::c_float = fresh2.as_mut_ptr() as *mut libc::c_float;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut g2: *mut libc::c_float = fresh3.as_mut_ptr() as *mut libc::c_float;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut g1r: *mut libc::c_float = fresh4.as_mut_ptr() as *mut libc::c_float;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut g2r: *mut libc::c_float = fresh5.as_mut_ptr() as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    /* even and odd are slightly different base cases */
    g1_order = m + 1 as libc::c_int >> 1 as libc::c_int;
    g2_order = m >> 1 as libc::c_int;
    /* Compute the lengths of the x polynomials. */
    /* Compute the first half of K & R F1 & F2 polynomials. */
    /* Compute half of the symmetric and antisymmetric polynomials. */
    /* Remove the roots at +1 and -1. */
    *g1.offset(g1_order as isize) = 1.0f32;
    i = 1 as libc::c_int;
    while i <= g1_order {
        *g1.offset((g1_order - i) as isize) =
            *lpc.offset((i - 1 as libc::c_int) as isize) + *lpc.offset((m - i) as isize);
        i += 1
    }
    *g2.offset(g2_order as isize) = 1.0f32;
    i = 1 as libc::c_int;
    while i <= g2_order {
        *g2.offset((g2_order - i) as isize) =
            *lpc.offset((i - 1 as libc::c_int) as isize) - *lpc.offset((m - i) as isize);
        i += 1
    }
    if g1_order > g2_order {
        i = 2 as libc::c_int;
        while i <= g2_order {
            *g2.offset((g2_order - i) as isize) +=
                *g2.offset((g2_order - i + 2 as libc::c_int) as isize);
            i += 1
        }
    } else {
        i = 1 as libc::c_int;
        while i <= g1_order {
            *g1.offset((g1_order - i) as isize) -=
                *g1.offset((g1_order - i + 1 as libc::c_int) as isize);
            i += 1
        }
        i = 1 as libc::c_int;
        while i <= g2_order {
            *g2.offset((g2_order - i) as isize) +=
                *g2.offset((g2_order - i + 1 as libc::c_int) as isize);
            i += 1
        }
    }
    /* Convert into polynomials in cos(alpha) */
    cheby(g1, g1_order);
    cheby(g2, g2_order);
    /* Find the roots of the 2 even polynomials.*/
    if Laguerre_With_Deflation(g1, g1_order, g1r) != 0
        || Laguerre_With_Deflation(g2, g2_order, g2r) != 0
    {
        return -(1 as libc::c_int);
    } /* if it fails, it leaves g1r alone */
    Newton_Raphson(g1, g1_order, g1r); /* if it fails, it leaves g2r alone */
    Newton_Raphson(g2, g2_order, g2r);
    crate::stdlib::qsort(
        g1r as *mut libc::c_void,
        g1_order as crate::stddef_h::size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        Some(
            comp as unsafe extern "C" fn(
                _: *const libc::c_void,
                _: *const libc::c_void,
            ) -> libc::c_int,
        ),
    );
    crate::stdlib::qsort(
        g2r as *mut libc::c_void,
        g2_order as crate::stddef_h::size_t,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
        Some(
            comp as unsafe extern "C" fn(
                _: *const libc::c_void,
                _: *const libc::c_void,
            ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < g1_order {
        *lsp.offset((i * 2 as libc::c_int) as isize) =
            crate::stdlib::acos(*g1r.offset(i as isize) as libc::c_double) as libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < g2_order {
        *lsp.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize) =
            crate::stdlib::acos(*g2r.offset(i as isize) as libc::c_double) as libc::c_float;
        i += 1
    }
    return 0 as libc::c_int;
}

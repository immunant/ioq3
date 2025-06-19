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
    mut curve: *mut f32,
    mut map: *mut i32,
    mut n: i32,
    mut ln: i32,
    mut lsp: *mut f32,
    mut m: i32,
    mut amp: f32,
    mut ampoffset: f32,
) {
    let mut i: i32 = 0;
    let mut wdel: f32 =
        (3.14159265358979323846f64 / ln as f64) as f32;
    i = 0 as i32;
    while i < m {
        *lsp.offset(i as isize) = (2.0f32 as f64
            * crate::stdlib::cos(*lsp.offset(i as isize) as f64))
            as f32;
        i += 1
    }
    i = 0 as i32;
    while i < n {
        let mut j: i32 = 0;
        let mut k: i32 = *map.offset(i as isize);
        let mut p: f32 = 0.5f32;
        let mut q: f32 = 0.5f32;
        let mut w: f32 = (2.0f32 as f64
            * crate::stdlib::cos((wdel * k as f32) as f64))
            as f32;
        j = 1 as i32;
        while j < m {
            q *= w - *lsp.offset((j - 1 as i32) as isize);
            p *= w - *lsp.offset(j as isize);
            j += 2 as i32
        }
        if j == m {
            /* odd order filter; slightly assymetric */
            /* the last coefficient */
            q *= w - *lsp.offset((j - 1 as i32) as isize);
            p *= p * (4.0f32 - w * w);
            q *= q
        } else {
            /* even order filter; still symmetric */
            p *= p * (2.0f32 - w);
            q *= q * (2.0f32 + w)
        }
        q = crate::stdlib::exp(
            (amp as f64 / crate::stdlib::sqrt((p + q) as f64)
                - ampoffset as f64)
                * 0.11512925f32 as f64,
        ) as f32;
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

unsafe extern "C" fn cheby(mut g: *mut f32, mut ord: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    *g.offset(0 as i32 as isize) *= 0.5f32;
    i = 2 as i32;
    while i <= ord {
        j = ord;
        while j >= i {
            *g.offset((j - 2 as i32) as isize) -= *g.offset(j as isize);
            *g.offset(j as isize) += *g.offset(j as isize);
            j -= 1
        }
        i += 1
    }
}

unsafe extern "C" fn comp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    return ((*(a as *mut f32)) < *(b as *mut f32)) as i32
        - (*(a as *mut f32) > *(b as *mut f32)) as i32;
}

unsafe extern "C" fn Laguerre_With_Deflation(
    mut a: *mut f32,
    mut ord: i32,
    mut r: *mut f32,
) -> i32 {
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f64>() as libc::c_ulong)
            .wrapping_mul((ord + 1 as i32) as libc::c_ulong) as usize,
    );
    let mut defl: *mut f64 = fresh0.as_mut_ptr() as *mut f64;
    i = 0 as i32;
    while i <= ord {
        *defl.offset(i as isize) = *a.offset(i as isize) as f64;
        i += 1
    }
    m = ord;
    while m > 0 as i32 {
        let mut new: f64 = 0.0f32 as f64;
        let mut delta: f64 = 0.;
        loop
        /* iterate a root */
        {
            let mut p: f64 = *defl.offset(m as isize);
            let mut pp: f64 = 0.0f32 as f64;
            let mut ppp: f64 = 0.0f32 as f64;
            let mut denom: f64 = 0.;
            /* eval the polynomial and its first two derivatives */
            i = m;
            while i > 0 as i32 {
                ppp = new * ppp + pp;
                pp = new * pp + p;
                p = new * p + *defl.offset((i - 1 as i32) as isize);
                i -= 1
            }
            /* Laguerre's method */
            denom = (m - 1 as i32) as f64
                * ((m - 1 as i32) as f64 * pp * pp
                    - m as f64 * p * ppp); /* complex root!  The LPC generator handed us a bad filter */
            if denom < 0 as i32 as f64 {
                return -(1 as i32);
            }
            if pp > 0 as i32 as f64 {
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
            delta = m as f64 * p / denom;
            new -= delta;
            if delta < 0.0f32 as f64 {
                delta *= -(1 as i32) as f64
            }
            if crate::stdlib::fabs(delta / new) < 10e-12f64 {
                break;
            }
        }
        *r.offset((m - 1 as i32) as isize) = new as f32;
        /* forward deflation */
        i = m;
        while i > 0 as i32 {
            *defl.offset((i - 1 as i32) as isize) += new * *defl.offset(i as isize);
            i -= 1
        }
        defl = defl.offset(1);
        m -= 1
    }
    return 0 as i32;
}
/* for spit-and-polish only */

unsafe extern "C" fn Newton_Raphson(
    mut a: *mut f32,
    mut ord: i32,
    mut r: *mut f32,
) -> i32 {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut error: f64 = 1.0f32 as f64;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (ord as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f64>() as libc::c_ulong)
            as usize,
    );
    let mut root: *mut f64 = fresh1.as_mut_ptr() as *mut f64;
    i = 0 as i32;
    while i < ord {
        *root.offset(i as isize) = *r.offset(i as isize) as f64;
        i += 1
    }
    while error > 1e-20f64 {
        error = 0 as i32 as f64;
        i = 0 as i32;
        while i < ord {
            /* Update each point. */
            let mut pp: f64 = 0.0f64;
            let mut delta: f64 = 0.;
            let mut rooti: f64 = *root.offset(i as isize);
            let mut p: f64 = *a.offset(ord as isize) as f64;
            k = ord - 1 as i32;
            while k >= 0 as i32 {
                pp = pp * rooti + p;
                p = p * rooti + *a.offset(k as isize) as f64;
                k -= 1
            }
            delta = p / pp;
            *root.offset(i as isize) -= delta;
            error += delta * delta;
            i += 1
        }
        if count > 40 as i32 {
            return -(1 as i32);
        }
        count += 1
    }
    /* Replaced the original bubble sort with a real sort.  With your
    help, we can eliminate the bubble sort in our lifetime. --Monty */
    i = 0 as i32;
    while i < ord {
        *r.offset(i as isize) = *root.offset(i as isize) as f32;
        i += 1
    }
    return 0 as i32;
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
    mut lpc: *mut f32,
    mut lsp: *mut f32,
    mut m: i32,
) -> i32 {
    let mut order2: i32 = m + 1 as i32 >> 1 as i32;
    let mut g1_order: i32 = 0;
    let mut g2_order: i32 = 0;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as i32) as libc::c_ulong) as usize,
    );
    let mut g1: *mut f32 = fresh2.as_mut_ptr() as *mut f32;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as i32) as libc::c_ulong) as usize,
    );
    let mut g2: *mut f32 = fresh3.as_mut_ptr() as *mut f32;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as i32) as libc::c_ulong) as usize,
    );
    let mut g1r: *mut f32 = fresh4.as_mut_ptr() as *mut f32;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((order2 + 1 as i32) as libc::c_ulong) as usize,
    );
    let mut g2r: *mut f32 = fresh5.as_mut_ptr() as *mut f32;
    let mut i: i32 = 0;
    /* even and odd are slightly different base cases */
    g1_order = m + 1 as i32 >> 1 as i32;
    g2_order = m >> 1 as i32;
    /* Compute the lengths of the x polynomials. */
    /* Compute the first half of K & R F1 & F2 polynomials. */
    /* Compute half of the symmetric and antisymmetric polynomials. */
    /* Remove the roots at +1 and -1. */
    *g1.offset(g1_order as isize) = 1.0f32;
    i = 1 as i32;
    while i <= g1_order {
        *g1.offset((g1_order - i) as isize) =
            *lpc.offset((i - 1 as i32) as isize) + *lpc.offset((m - i) as isize);
        i += 1
    }
    *g2.offset(g2_order as isize) = 1.0f32;
    i = 1 as i32;
    while i <= g2_order {
        *g2.offset((g2_order - i) as isize) =
            *lpc.offset((i - 1 as i32) as isize) - *lpc.offset((m - i) as isize);
        i += 1
    }
    if g1_order > g2_order {
        i = 2 as i32;
        while i <= g2_order {
            *g2.offset((g2_order - i) as isize) +=
                *g2.offset((g2_order - i + 2 as i32) as isize);
            i += 1
        }
    } else {
        i = 1 as i32;
        while i <= g1_order {
            *g1.offset((g1_order - i) as isize) -=
                *g1.offset((g1_order - i + 1 as i32) as isize);
            i += 1
        }
        i = 1 as i32;
        while i <= g2_order {
            *g2.offset((g2_order - i) as isize) +=
                *g2.offset((g2_order - i + 1 as i32) as isize);
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
        return -(1 as i32);
    } /* if it fails, it leaves g1r alone */
    Newton_Raphson(g1, g1_order, g1r); /* if it fails, it leaves g2r alone */
    Newton_Raphson(g2, g2_order, g2r);
    crate::stdlib::qsort(
        g1r as *mut libc::c_void,
        g1_order as crate::stddef_h::size_t,
        ::std::mem::size_of::<f32>() as libc::c_ulong,
        Some(
            comp as unsafe extern "C" fn(
                _: *const libc::c_void,
                _: *const libc::c_void,
            ) -> i32,
        ),
    );
    crate::stdlib::qsort(
        g2r as *mut libc::c_void,
        g2_order as crate::stddef_h::size_t,
        ::std::mem::size_of::<f32>() as libc::c_ulong,
        Some(
            comp as unsafe extern "C" fn(
                _: *const libc::c_void,
                _: *const libc::c_void,
            ) -> i32,
        ),
    );
    i = 0 as i32;
    while i < g1_order {
        *lsp.offset((i * 2 as i32) as isize) =
            crate::stdlib::acos(*g1r.offset(i as isize) as f64) as f32;
        i += 1
    }
    i = 0 as i32;
    while i < g2_order {
        *lsp.offset((i * 2 as i32 + 1 as i32) as isize) =
            crate::stdlib::acos(*g2r.offset(i as isize) as f64) as f32;
        i += 1
    }
    return 0 as i32;
}

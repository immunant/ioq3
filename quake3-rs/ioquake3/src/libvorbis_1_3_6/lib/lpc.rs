use ::libc;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2007             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

 function: LPC low level routines

********************************************************************/
/* simple linear scale LPC code */
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

 function: LPC low level routines

********************************************************************/
/* Some of these routines (autocorrelator, LPC coefficient estimator)
are derived from code written by Jutta Degener and Carsten Bormann;
thus we include their copyright below.  The entirety of this file
is freely redistributable on the condition that both of these
copyright notices are preserved without modification.  */
/* Preserved Copyright: *********************************************/
/* Copyright 1992, 1993, 1994 by Jutta Degener and Carsten Bormann,
Technische Universita"t Berlin

Any use of this software is permitted provided that this notice is not
removed and that neither the authors nor the Technische Universita"t
Berlin are deemed to have made any representations as to the
suitability of this software for any purpose nor are held responsible
for any defects of this software. THERE IS ABSOLUTELY NO WARRANTY FOR
THIS SOFTWARE.

As a matter of courtesy, the authors request to be informed about uses
this software has found, about bugs in this software, and about any
improvements that may be of general interest.

Berlin, 28.11.1994
Jutta Degener
Carsten Bormann

*********************************************************************/
/* Autocorrelation LPC coeff generation algorithm invented by
N. Levinson in 1947, modified by J. Durbin in 1959. */
/* Input : n elements of time doamin data
Output: m lpc coefficients, excitation energy */
#[no_mangle]

pub unsafe extern "C" fn vorbis_lpc_from_data(
    mut data: *mut libc::c_float,
    mut lpci: *mut libc::c_float,
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_float {
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((m + 1 as libc::c_int) as libc::c_ulong) as usize,
    );
    let mut aut: *mut libc::c_double = fresh0.as_mut_ptr() as *mut libc::c_double;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong).wrapping_mul(m as libc::c_ulong)
            as usize,
    );
    let mut lpc: *mut libc::c_double = fresh1.as_mut_ptr() as *mut libc::c_double;
    let mut error: libc::c_double = 0.;
    let mut epsilon: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    /* autocorrelation, p+1 lag coefficients */
    j = m + 1 as libc::c_int; /* double needed for accumulator depth */
    loop {
        let fresh2 = j;
        j = j - 1;
        if !(fresh2 != 0) {
            break;
        }
        let mut d: libc::c_double = 0 as libc::c_int as libc::c_double;
        i = j;
        while i < n {
            d += *data.offset(i as isize) as libc::c_double
                * *data.offset((i - j) as isize) as libc::c_double;
            i += 1
        }
        *aut.offset(j as isize) = d
    }
    /* Generate lpc coefficients from autocorr values */
    /* set our noise floor to about -100dB */
    error = *aut.offset(0 as libc::c_int as isize) * (1.0f64 + 1e-10f64);
    epsilon = 1e-9f64 * *aut.offset(0 as libc::c_int as isize) + 1e-10f64;
    i = 0 as libc::c_int;
    while i < m {
        let mut r: libc::c_double = -*aut.offset((i + 1 as libc::c_int) as isize);
        if error < epsilon {
            crate::stdlib::memset(
                lpc.offset(i as isize) as *mut libc::c_void,
                0 as libc::c_int,
                ((m - i) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            );
            break;
        } else {
            /* Sum up this iteration's reflection coefficient; note that in
            Vorbis we don't save it.  If anyone wants to recycle this code
            and needs reflection coefficients, save the results of 'r' from
            each iteration. */
            j = 0 as libc::c_int;
            while j < i {
                r -= *lpc.offset(j as isize) * *aut.offset((i - j) as isize);
                j += 1
            }
            r /= error;
            /* Update LPC coefficients and total error */
            *lpc.offset(i as isize) = r;
            j = 0 as libc::c_int;
            while j < i / 2 as libc::c_int {
                let mut tmp: libc::c_double = *lpc.offset(j as isize);
                *lpc.offset(j as isize) += r * *lpc.offset((i - 1 as libc::c_int - j) as isize);
                *lpc.offset((i - 1 as libc::c_int - j) as isize) += r * tmp;
                j += 1
            }
            if i & 1 as libc::c_int != 0 {
                *lpc.offset(j as isize) += *lpc.offset(j as isize) * r
            }
            error *= 1.0f64 - r * r;
            i += 1
        }
    }
    /* slightly damp the filter */
    let mut g: libc::c_double = 0.99f64;
    let mut damp: libc::c_double = g;
    j = 0 as libc::c_int;
    while j < m {
        *lpc.offset(j as isize) *= damp;
        damp *= g;
        j += 1
    }
    j = 0 as libc::c_int;
    while j < m {
        *lpci.offset(j as isize) = *lpc.offset(j as isize) as libc::c_float;
        j += 1
    }
    /* we need the error value to know how big an impulse to hit the
    filter with later */
    return error as libc::c_float;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_lpc_predict(
    mut coeff: *mut libc::c_float,
    mut prime: *mut libc::c_float,
    mut m: libc::c_int,
    mut data: *mut libc::c_float,
    mut n: libc::c_long,
) {
    /* in: coeff[0...m-1] LPC coefficients
         prime[0...m-1] initial values (allocated size of n+m-1)
    out: data[0...n-1] data samples */
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut o: libc::c_long = 0;
    let mut p: libc::c_long = 0;
    let mut y: libc::c_float = 0.;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((m as libc::c_long + n) as libc::c_ulong) as usize,
    );
    let mut work: *mut libc::c_float = fresh3.as_mut_ptr() as *mut libc::c_float;
    if prime.is_null() {
        i = 0 as libc::c_int as libc::c_long;
        while i < m as libc::c_long {
            *work.offset(i as isize) = 0.0f32;
            i += 1
        }
    } else {
        i = 0 as libc::c_int as libc::c_long;
        while i < m as libc::c_long {
            *work.offset(i as isize) = *prime.offset(i as isize);
            i += 1
        }
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < n {
        y = 0 as libc::c_int as libc::c_float;
        o = i;
        p = m as libc::c_long;
        j = 0 as libc::c_int as libc::c_long;
        while j < m as libc::c_long {
            let fresh4 = o;
            o = o + 1;
            p -= 1;
            y -= *work.offset(fresh4 as isize) * *coeff.offset(p as isize);
            j += 1
        }
        let ref mut fresh5 = *work.offset(o as isize);
        *fresh5 = y;
        *data.offset(i as isize) = *fresh5;
        i += 1
    }
}

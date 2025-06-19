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
    mut data: *mut f32,
    mut lpci: *mut f32,
    mut n: i32,
    mut m: i32,
) -> f32 {
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f64>() as usize)
            .wrapping_mul((m + 1 as i32) as usize) as usize,
    );
    let mut aut: *mut f64 = fresh0.as_mut_ptr() as *mut f64;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f64>() as usize).wrapping_mul(m as usize) as usize,
    );
    let mut lpc: *mut f64 = fresh1.as_mut_ptr() as *mut f64;
    let mut error: f64 = 0.;
    let mut epsilon: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    /* autocorrelation, p+1 lag coefficients */
    j = m + 1 as i32; /* double needed for accumulator depth */
    loop {
        let fresh2 = j;
        j = j - 1;
        if !(fresh2 != 0) {
            break;
        }
        let mut d: f64 = 0 as i32 as f64;
        i = j;
        while i < n {
            d += *data.offset(i as isize) as f64 * *data.offset((i - j) as isize) as f64;
            i += 1
        }
        *aut.offset(j as isize) = d
    }
    /* Generate lpc coefficients from autocorr values */
    /* set our noise floor to about -100dB */
    error = *aut.offset(0 as i32 as isize) * (1.0f64 + 1e-10f64);
    epsilon = 1e-9f64 * *aut.offset(0 as i32 as isize) + 1e-10f64;
    i = 0 as i32;
    while i < m {
        let mut r: f64 = -*aut.offset((i + 1 as i32) as isize);
        if error < epsilon {
            crate::stdlib::memset(
                lpc.offset(i as isize) as *mut libc::c_void,
                0 as i32,
                ((m - i) as usize)
                    .wrapping_mul(::std::mem::size_of::<f64>() as usize),
            );
            break;
        } else {
            /* Sum up this iteration's reflection coefficient; note that in
            Vorbis we don't save it.  If anyone wants to recycle this code
            and needs reflection coefficients, save the results of 'r' from
            each iteration. */
            j = 0 as i32;
            while j < i {
                r -= *lpc.offset(j as isize) * *aut.offset((i - j) as isize);
                j += 1
            }
            r /= error;
            /* Update LPC coefficients and total error */
            *lpc.offset(i as isize) = r;
            j = 0 as i32;
            while j < i / 2 as i32 {
                let mut tmp: f64 = *lpc.offset(j as isize);
                *lpc.offset(j as isize) += r * *lpc.offset((i - 1 as i32 - j) as isize);
                *lpc.offset((i - 1 as i32 - j) as isize) += r * tmp;
                j += 1
            }
            if i & 1 as i32 != 0 {
                *lpc.offset(j as isize) += *lpc.offset(j as isize) * r
            }
            error *= 1.0f64 - r * r;
            i += 1
        }
    }
    /* slightly damp the filter */
    let mut g: f64 = 0.99f64;
    let mut damp: f64 = g;
    j = 0 as i32;
    while j < m {
        *lpc.offset(j as isize) *= damp;
        damp *= g;
        j += 1
    }
    j = 0 as i32;
    while j < m {
        *lpci.offset(j as isize) = *lpc.offset(j as isize) as f32;
        j += 1
    }
    /* we need the error value to know how big an impulse to hit the
    filter with later */
    return error as f32;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_lpc_predict(
    mut coeff: *mut f32,
    mut prime: *mut f32,
    mut m: i32,
    mut data: *mut f32,
    mut n: isize,
) {
    /* in: coeff[0...m-1] LPC coefficients
         prime[0...m-1] initial values (allocated size of n+m-1)
    out: data[0...n-1] data samples */
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut o: isize = 0;
    let mut p: isize = 0;
    let mut y: f32 = 0.;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as usize)
            .wrapping_mul((m as isize + n) as usize) as usize,
    );
    let mut work: *mut f32 = fresh3.as_mut_ptr() as *mut f32;
    if prime.is_null() {
        i = 0 as i32 as isize;
        while i < m as isize {
            *work.offset(i as isize) = 0.0f32;
            i += 1
        }
    } else {
        i = 0 as i32 as isize;
        while i < m as isize {
            *work.offset(i as isize) = *prime.offset(i as isize);
            i += 1
        }
    }
    i = 0 as i32 as isize;
    while i < n {
        y = 0 as i32 as f32;
        o = i;
        p = m as isize;
        j = 0 as i32 as isize;
        while j < m as isize {
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

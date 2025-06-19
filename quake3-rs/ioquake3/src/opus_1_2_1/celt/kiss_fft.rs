// =============== BEGIN kiss_fft_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kiss_fft_cpx {
    pub r: f32,
    pub i: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kiss_twiddle_cpx {
    pub r: f32,
    pub i: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_fft_state {
    pub is_supported: i32,
    pub priv_0: *mut libc::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kiss_fft_state {
    pub nfft: i32,
    pub scale: crate::arch_h::opus_val16,
    pub shift: i32,
    pub factors: [crate::opus_types_h::opus_int16; 16],
    pub bitrev: *const crate::opus_types_h::opus_int16,
    pub twiddles: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx,
    pub arch_fft: *mut crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state,
}
use ::libc;

pub use crate::arch_h::opus_val16;
pub use crate::opus_types_h::opus_int16;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::int16_t;

/*Copyright (c) 2003-2004, Mark Borgerding
Lots of modifications by Jean-Marc Valin
Copyright (c) 2005-2007, Xiph.Org Foundation
Copyright (c) 2008,      Xiph.Org Foundation, CSIRO

All rights reserved.

Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are met:

  * Redistributions of source code must retain the above copyright notice,
     this list of conditions and the following disclaimer.
  * Redistributions in binary form must reproduce the above copyright notice,
     this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.*/
/* This code is originally from Mark Borgerding's KISS-FFT but has been
heavily modified to better suit Opus */
/* The guts header contains all the multiplication and addition macros that are defined for
   complex numbers.  It also delares the kf_ internal functions.
*/

unsafe extern "C" fn kf_bfly2(
    mut Fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    mut _m: i32,
    mut N: i32,
) {
    let mut Fout2: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut i: i32 = 0;
    let mut tw: crate::arch_h::opus_val16 = 0.;
    tw = 0.7071067812f32;
    /* We know that m==4 here because the radix-2 is just after a radix-4 */
    i = 0 as i32;
    while i < N {
        let mut t: crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
            crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. };
        Fout2 = Fout.offset(4 as i32 as isize);
        t = *Fout2.offset(0 as i32 as isize);
        (*Fout2.offset(0 as i32 as isize)).r = (*Fout.offset(0 as i32 as isize)).r - t.r;
        (*Fout2.offset(0 as i32 as isize)).i = (*Fout.offset(0 as i32 as isize)).i - t.i;
        (*Fout.offset(0 as i32 as isize)).r += t.r;
        (*Fout.offset(0 as i32 as isize)).i += t.i;
        t.r = ((*Fout2.offset(1 as i32 as isize)).r + (*Fout2.offset(1 as i32 as isize)).i) * tw;
        t.i = ((*Fout2.offset(1 as i32 as isize)).i - (*Fout2.offset(1 as i32 as isize)).r) * tw;
        (*Fout2.offset(1 as i32 as isize)).r = (*Fout.offset(1 as i32 as isize)).r - t.r;
        (*Fout2.offset(1 as i32 as isize)).i = (*Fout.offset(1 as i32 as isize)).i - t.i;
        (*Fout.offset(1 as i32 as isize)).r += t.r;
        (*Fout.offset(1 as i32 as isize)).i += t.i;
        t.r = (*Fout2.offset(2 as i32 as isize)).i;
        t.i = -(*Fout2.offset(2 as i32 as isize)).r;
        (*Fout2.offset(2 as i32 as isize)).r = (*Fout.offset(2 as i32 as isize)).r - t.r;
        (*Fout2.offset(2 as i32 as isize)).i = (*Fout.offset(2 as i32 as isize)).i - t.i;
        (*Fout.offset(2 as i32 as isize)).r += t.r;
        (*Fout.offset(2 as i32 as isize)).i += t.i;
        t.r = ((*Fout2.offset(3 as i32 as isize)).i - (*Fout2.offset(3 as i32 as isize)).r) * tw;
        t.i = -((*Fout2.offset(3 as i32 as isize)).i + (*Fout2.offset(3 as i32 as isize)).r) * tw;
        (*Fout2.offset(3 as i32 as isize)).r = (*Fout.offset(3 as i32 as isize)).r - t.r;
        (*Fout2.offset(3 as i32 as isize)).i = (*Fout.offset(3 as i32 as isize)).i - t.i;
        (*Fout.offset(3 as i32 as isize)).r += t.r;
        (*Fout.offset(3 as i32 as isize)).i += t.i;
        Fout = Fout.offset(8 as i32 as isize);
        i += 1
    }
}

unsafe extern "C" fn kf_bfly4(
    mut Fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    fstride: crate::stddef_h::size_t,
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut m: i32,
    mut N: i32,
    mut mm: i32,
) {
    let mut i: i32 = 0;
    if m == 1 as i32 {
        /* Degenerate case where all the twiddles are 1. */
        i = 0 as i32;
        while i < N {
            let mut scratch0: crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
                crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. };
            let mut scratch1: crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
                crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. };
            scratch0.r = (*Fout).r - (*Fout.offset(2 as i32 as isize)).r;
            scratch0.i = (*Fout).i - (*Fout.offset(2 as i32 as isize)).i;
            (*Fout).r += (*Fout.offset(2 as i32 as isize)).r;
            (*Fout).i += (*Fout.offset(2 as i32 as isize)).i;
            scratch1.r = (*Fout.offset(1 as i32 as isize)).r + (*Fout.offset(3 as i32 as isize)).r;
            scratch1.i = (*Fout.offset(1 as i32 as isize)).i + (*Fout.offset(3 as i32 as isize)).i;
            (*Fout.offset(2 as i32 as isize)).r = (*Fout).r - scratch1.r;
            (*Fout.offset(2 as i32 as isize)).i = (*Fout).i - scratch1.i;
            (*Fout).r += scratch1.r;
            (*Fout).i += scratch1.i;
            scratch1.r = (*Fout.offset(1 as i32 as isize)).r - (*Fout.offset(3 as i32 as isize)).r;
            scratch1.i = (*Fout.offset(1 as i32 as isize)).i - (*Fout.offset(3 as i32 as isize)).i;
            (*Fout.offset(1 as i32 as isize)).r = scratch0.r + scratch1.i;
            (*Fout.offset(1 as i32 as isize)).i = scratch0.i - scratch1.r;
            (*Fout.offset(3 as i32 as isize)).r = scratch0.r - scratch1.i;
            (*Fout.offset(3 as i32 as isize)).i = scratch0.i + scratch1.r;
            Fout = Fout.offset(4 as i32 as isize);
            i += 1
        }
    } else {
        let mut j: i32 = 0;
        let mut scratch: [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx; 6] =
            [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. }; 6];
        let mut tw1: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
            0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
        let mut tw2: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
            0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
        let mut tw3: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
            0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
        let m2: i32 = 2 as i32 * m;
        let m3: i32 = 3 as i32 * m;
        let mut Fout_beg: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx = Fout;
        i = 0 as i32;
        while i < N {
            Fout = Fout_beg.offset((i * mm) as isize);
            tw1 = (*st).twiddles;
            tw2 = tw1;
            tw3 = tw2;
            /* m is guaranteed to be a multiple of 4. */
            j = 0 as i32;
            while j < m {
                scratch[0 as i32 as usize].r = (*Fout.offset(m as isize)).r * (*tw1).r
                    - (*Fout.offset(m as isize)).i * (*tw1).i;
                scratch[0 as i32 as usize].i = (*Fout.offset(m as isize)).r * (*tw1).i
                    + (*Fout.offset(m as isize)).i * (*tw1).r;
                scratch[1 as i32 as usize].r = (*Fout.offset(m2 as isize)).r * (*tw2).r
                    - (*Fout.offset(m2 as isize)).i * (*tw2).i;
                scratch[1 as i32 as usize].i = (*Fout.offset(m2 as isize)).r * (*tw2).i
                    + (*Fout.offset(m2 as isize)).i * (*tw2).r;
                scratch[2 as i32 as usize].r = (*Fout.offset(m3 as isize)).r * (*tw3).r
                    - (*Fout.offset(m3 as isize)).i * (*tw3).i;
                scratch[2 as i32 as usize].i = (*Fout.offset(m3 as isize)).r * (*tw3).i
                    + (*Fout.offset(m3 as isize)).i * (*tw3).r;
                scratch[5 as i32 as usize].r = (*Fout).r - scratch[1 as i32 as usize].r;
                scratch[5 as i32 as usize].i = (*Fout).i - scratch[1 as i32 as usize].i;
                (*Fout).r += scratch[1 as i32 as usize].r;
                (*Fout).i += scratch[1 as i32 as usize].i;
                scratch[3 as i32 as usize].r =
                    scratch[0 as i32 as usize].r + scratch[2 as i32 as usize].r;
                scratch[3 as i32 as usize].i =
                    scratch[0 as i32 as usize].i + scratch[2 as i32 as usize].i;
                scratch[4 as i32 as usize].r =
                    scratch[0 as i32 as usize].r - scratch[2 as i32 as usize].r;
                scratch[4 as i32 as usize].i =
                    scratch[0 as i32 as usize].i - scratch[2 as i32 as usize].i;
                (*Fout.offset(m2 as isize)).r = (*Fout).r - scratch[3 as i32 as usize].r;
                (*Fout.offset(m2 as isize)).i = (*Fout).i - scratch[3 as i32 as usize].i;
                tw1 = tw1.offset(fstride as isize);
                tw2 = tw2.offset(fstride.wrapping_mul(2 as i32 as libc::c_ulong) as isize);
                tw3 = tw3.offset(fstride.wrapping_mul(3 as i32 as libc::c_ulong) as isize);
                (*Fout).r += scratch[3 as i32 as usize].r;
                (*Fout).i += scratch[3 as i32 as usize].i;
                (*Fout.offset(m as isize)).r =
                    scratch[5 as i32 as usize].r + scratch[4 as i32 as usize].i;
                (*Fout.offset(m as isize)).i =
                    scratch[5 as i32 as usize].i - scratch[4 as i32 as usize].r;
                (*Fout.offset(m3 as isize)).r =
                    scratch[5 as i32 as usize].r - scratch[4 as i32 as usize].i;
                (*Fout.offset(m3 as isize)).i =
                    scratch[5 as i32 as usize].i + scratch[4 as i32 as usize].r;
                Fout = Fout.offset(1);
                j += 1
            }
            i += 1
        }
    };
}

unsafe extern "C" fn kf_bfly3(
    mut Fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    fstride: crate::stddef_h::size_t,
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut m: i32,
    mut N: i32,
    mut mm: i32,
) {
    let mut i: i32 = 0;
    let mut k: crate::stddef_h::size_t = 0;
    let m2: crate::stddef_h::size_t = (2 as i32 * m) as crate::stddef_h::size_t;
    let mut tw1: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
    let mut tw2: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
    let mut scratch: [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx; 5] =
        [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. }; 5];
    let mut epi3: crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx { r: 0., i: 0. };
    let mut Fout_beg: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx = Fout;
    epi3 = *(*st)
        .twiddles
        .offset(fstride.wrapping_mul(m as libc::c_ulong) as isize);
    i = 0 as i32;
    while i < N {
        Fout = Fout_beg.offset((i * mm) as isize);
        tw2 = (*st).twiddles;
        tw1 = tw2;
        /* For non-custom modes, m is guaranteed to be a multiple of 4. */
        k = m as crate::stddef_h::size_t;
        loop {
            scratch[1 as i32 as usize].r =
                (*Fout.offset(m as isize)).r * (*tw1).r - (*Fout.offset(m as isize)).i * (*tw1).i;
            scratch[1 as i32 as usize].i =
                (*Fout.offset(m as isize)).r * (*tw1).i + (*Fout.offset(m as isize)).i * (*tw1).r;
            scratch[2 as i32 as usize].r =
                (*Fout.offset(m2 as isize)).r * (*tw2).r - (*Fout.offset(m2 as isize)).i * (*tw2).i;
            scratch[2 as i32 as usize].i =
                (*Fout.offset(m2 as isize)).r * (*tw2).i + (*Fout.offset(m2 as isize)).i * (*tw2).r;
            scratch[3 as i32 as usize].r =
                scratch[1 as i32 as usize].r + scratch[2 as i32 as usize].r;
            scratch[3 as i32 as usize].i =
                scratch[1 as i32 as usize].i + scratch[2 as i32 as usize].i;
            scratch[0 as i32 as usize].r =
                scratch[1 as i32 as usize].r - scratch[2 as i32 as usize].r;
            scratch[0 as i32 as usize].i =
                scratch[1 as i32 as usize].i - scratch[2 as i32 as usize].i;
            tw1 = tw1.offset(fstride as isize);
            tw2 = tw2.offset(fstride.wrapping_mul(2 as i32 as libc::c_ulong) as isize);
            (*Fout.offset(m as isize)).r = (*Fout).r - scratch[3 as i32 as usize].r * 0.5f32;
            (*Fout.offset(m as isize)).i = (*Fout).i - scratch[3 as i32 as usize].i * 0.5f32;
            scratch[0 as i32 as usize].r *= epi3.i;
            scratch[0 as i32 as usize].i *= epi3.i;
            (*Fout).r += scratch[3 as i32 as usize].r;
            (*Fout).i += scratch[3 as i32 as usize].i;
            (*Fout.offset(m2 as isize)).r =
                (*Fout.offset(m as isize)).r + scratch[0 as i32 as usize].i;
            (*Fout.offset(m2 as isize)).i =
                (*Fout.offset(m as isize)).i - scratch[0 as i32 as usize].r;
            (*Fout.offset(m as isize)).r =
                (*Fout.offset(m as isize)).r - scratch[0 as i32 as usize].i;
            (*Fout.offset(m as isize)).i =
                (*Fout.offset(m as isize)).i + scratch[0 as i32 as usize].r;
            Fout = Fout.offset(1);
            k = k.wrapping_sub(1);
            if !(k != 0) {
                break;
            }
        }
        i += 1
    }
}

unsafe extern "C" fn kf_bfly5(
    mut Fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    fstride: crate::stddef_h::size_t,
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut m: i32,
    mut N: i32,
    mut mm: i32,
) {
    let mut Fout0: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut Fout1: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut Fout2: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut Fout3: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut Fout4: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx =
        0 as *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
    let mut i: i32 = 0;
    let mut u: i32 = 0;
    let mut scratch: [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx; 13] =
        [crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx { r: 0., i: 0. }; 13];
    let mut tw: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        0 as *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
    let mut ya: crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx { r: 0., i: 0. };
    let mut yb: crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx =
        crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx { r: 0., i: 0. };
    let mut Fout_beg: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx = Fout;
    ya = *(*st)
        .twiddles
        .offset(fstride.wrapping_mul(m as libc::c_ulong) as isize);
    yb = *(*st).twiddles.offset(
        fstride
            .wrapping_mul(2 as i32 as libc::c_ulong)
            .wrapping_mul(m as libc::c_ulong) as isize,
    );
    tw = (*st).twiddles;
    i = 0 as i32;
    while i < N {
        Fout = Fout_beg.offset((i * mm) as isize);
        Fout0 = Fout;
        Fout1 = Fout0.offset(m as isize);
        Fout2 = Fout0.offset((2 as i32 * m) as isize);
        Fout3 = Fout0.offset((3 as i32 * m) as isize);
        Fout4 = Fout0.offset((4 as i32 * m) as isize);
        /* For non-custom modes, m is guaranteed to be a multiple of 4. */
        u = 0 as i32;
        while u < m {
            scratch[0 as i32 as usize] = *Fout0;
            scratch[1 as i32 as usize].r = (*Fout1).r
                * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).r
                - (*Fout1).i * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).i;
            scratch[1 as i32 as usize].i = (*Fout1).r
                * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).i
                + (*Fout1).i * (*tw.offset((u as libc::c_ulong).wrapping_mul(fstride) as isize)).r;
            scratch[2 as i32 as usize].r = (*Fout2).r
                * (*tw.offset(((2 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).r
                - (*Fout2).i
                    * (*tw
                        .offset(((2 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .i;
            scratch[2 as i32 as usize].i = (*Fout2).r
                * (*tw.offset(((2 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).i
                + (*Fout2).i
                    * (*tw
                        .offset(((2 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .r;
            scratch[3 as i32 as usize].r = (*Fout3).r
                * (*tw.offset(((3 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).r
                - (*Fout3).i
                    * (*tw
                        .offset(((3 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .i;
            scratch[3 as i32 as usize].i = (*Fout3).r
                * (*tw.offset(((3 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).i
                + (*Fout3).i
                    * (*tw
                        .offset(((3 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .r;
            scratch[4 as i32 as usize].r = (*Fout4).r
                * (*tw.offset(((4 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).r
                - (*Fout4).i
                    * (*tw
                        .offset(((4 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .i;
            scratch[4 as i32 as usize].i = (*Fout4).r
                * (*tw.offset(((4 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize)).i
                + (*Fout4).i
                    * (*tw
                        .offset(((4 as i32 * u) as libc::c_ulong).wrapping_mul(fstride) as isize))
                    .r;
            scratch[7 as i32 as usize].r =
                scratch[1 as i32 as usize].r + scratch[4 as i32 as usize].r;
            scratch[7 as i32 as usize].i =
                scratch[1 as i32 as usize].i + scratch[4 as i32 as usize].i;
            scratch[10 as i32 as usize].r =
                scratch[1 as i32 as usize].r - scratch[4 as i32 as usize].r;
            scratch[10 as i32 as usize].i =
                scratch[1 as i32 as usize].i - scratch[4 as i32 as usize].i;
            scratch[8 as i32 as usize].r =
                scratch[2 as i32 as usize].r + scratch[3 as i32 as usize].r;
            scratch[8 as i32 as usize].i =
                scratch[2 as i32 as usize].i + scratch[3 as i32 as usize].i;
            scratch[9 as i32 as usize].r =
                scratch[2 as i32 as usize].r - scratch[3 as i32 as usize].r;
            scratch[9 as i32 as usize].i =
                scratch[2 as i32 as usize].i - scratch[3 as i32 as usize].i;
            (*Fout0).r = (*Fout0).r + (scratch[7 as i32 as usize].r + scratch[8 as i32 as usize].r);
            (*Fout0).i = (*Fout0).i + (scratch[7 as i32 as usize].i + scratch[8 as i32 as usize].i);
            scratch[5 as i32 as usize].r = scratch[0 as i32 as usize].r
                + (scratch[7 as i32 as usize].r * ya.r + scratch[8 as i32 as usize].r * yb.r);
            scratch[5 as i32 as usize].i = scratch[0 as i32 as usize].i
                + (scratch[7 as i32 as usize].i * ya.r + scratch[8 as i32 as usize].i * yb.r);
            scratch[6 as i32 as usize].r =
                scratch[10 as i32 as usize].i * ya.i + scratch[9 as i32 as usize].i * yb.i;
            scratch[6 as i32 as usize].i =
                -(scratch[10 as i32 as usize].r * ya.i + scratch[9 as i32 as usize].r * yb.i);
            (*Fout1).r = scratch[5 as i32 as usize].r - scratch[6 as i32 as usize].r;
            (*Fout1).i = scratch[5 as i32 as usize].i - scratch[6 as i32 as usize].i;
            (*Fout4).r = scratch[5 as i32 as usize].r + scratch[6 as i32 as usize].r;
            (*Fout4).i = scratch[5 as i32 as usize].i + scratch[6 as i32 as usize].i;
            scratch[11 as i32 as usize].r = scratch[0 as i32 as usize].r
                + (scratch[7 as i32 as usize].r * yb.r + scratch[8 as i32 as usize].r * ya.r);
            scratch[11 as i32 as usize].i = scratch[0 as i32 as usize].i
                + (scratch[7 as i32 as usize].i * yb.r + scratch[8 as i32 as usize].i * ya.r);
            scratch[12 as i32 as usize].r =
                scratch[9 as i32 as usize].i * ya.i - scratch[10 as i32 as usize].i * yb.i;
            scratch[12 as i32 as usize].i =
                scratch[10 as i32 as usize].r * yb.i - scratch[9 as i32 as usize].r * ya.i;
            (*Fout2).r = scratch[11 as i32 as usize].r + scratch[12 as i32 as usize].r;
            (*Fout2).i = scratch[11 as i32 as usize].i + scratch[12 as i32 as usize].i;
            (*Fout3).r = scratch[11 as i32 as usize].r - scratch[12 as i32 as usize].r;
            (*Fout3).i = scratch[11 as i32 as usize].i - scratch[12 as i32 as usize].i;
            Fout0 = Fout0.offset(1);
            Fout1 = Fout1.offset(1);
            Fout2 = Fout2.offset(1);
            Fout3 = Fout3.offset(1);
            Fout4 = Fout4.offset(1);
            u += 1
        }
        i += 1
    }
}
/* OVERRIDE_kf_bfly5 */
/* CUSTOM_MODES */
#[no_mangle]

pub unsafe extern "C" fn opus_fft_impl(
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
) {
    let mut m2: i32 = 0;
    let mut m: i32 = 0;
    let mut p: i32 = 0;
    let mut L: i32 = 0;
    let mut fstride: [i32; 8] = [0; 8];
    let mut i: i32 = 0;
    let mut shift: i32 = 0;
    /* st->shift can be -1 */
    shift = if (*st).shift > 0 as i32 {
        (*st).shift
    } else {
        0 as i32
    };
    fstride[0 as i32 as usize] = 1 as i32;
    L = 0 as i32;
    loop {
        p = (*st).factors[(2 as i32 * L) as usize] as i32;
        m = (*st).factors[(2 as i32 * L + 1 as i32) as usize] as i32;
        fstride[(L + 1 as i32) as usize] = fstride[L as usize] * p;
        L += 1;
        if !(m != 1 as i32) {
            break;
        }
    }
    m = (*st).factors[(2 as i32 * L - 1 as i32) as usize] as i32;
    i = L - 1 as i32;
    while i >= 0 as i32 {
        if i != 0 as i32 {
            m2 = (*st).factors[(2 as i32 * i - 1 as i32) as usize] as i32
        } else {
            m2 = 1 as i32
        }
        match (*st).factors[(2 as i32 * i) as usize] as i32 {
            2 => {
                kf_bfly2(fout, m, fstride[i as usize]);
            }
            4 => {
                kf_bfly4(
                    fout,
                    (fstride[i as usize] << shift) as crate::stddef_h::size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            3 => {
                kf_bfly3(
                    fout,
                    (fstride[i as usize] << shift) as crate::stddef_h::size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            5 => {
                kf_bfly5(
                    fout,
                    (fstride[i as usize] << shift) as crate::stddef_h::size_t,
                    st,
                    m,
                    fstride[i as usize],
                    m2,
                );
            }
            _ => {}
        }
        m = m2;
        i -= 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn opus_fft_c(
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut fin: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    mut fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
) {
    let mut i: i32 = 0;
    let mut scale: crate::arch_h::opus_val16 = 0.;
    scale = (*st).scale;
    /* Bit-reverse the input */
    i = 0 as i32;
    while i < (*st).nfft {
        let mut x: crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx = *fin.offset(i as isize);
        (*fout.offset(*(*st).bitrev.offset(i as isize) as isize)).r = scale * x.r;
        (*fout.offset(*(*st).bitrev.offset(i as isize) as isize)).i = scale * x.i;
        i += 1
    }
    opus_fft_impl(st, fout);
}
/*Copyright (c) 2003-2004, Mark Borgerding
Lots of modifications by Jean-Marc Valin
Copyright (c) 2005-2007, Xiph.Org Foundation
Copyright (c) 2008,      Xiph.Org Foundation, CSIRO

All rights reserved.

Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are met:

  * Redistributions of source code must retain the above copyright notice,
     this list of conditions and the following disclaimer.
  * Redistributions in binary form must reproduce the above copyright notice,
     this list of conditions and the following disclaimer in the
     documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.*/
/*  default is float */
/* e.g. an fft of length 128 has 4 factors
as far as kissfft is concerned
4*4*4*2
*/
/*typedef struct kiss_fft_state* kiss_fft_cfg;*/
/* *
 *  opus_fft_alloc
 *
 *  Initialize a FFT (or IFFT) algorithm's cfg/state buffer.
 *
 *  typical usage:      kiss_fft_cfg mycfg=opus_fft_alloc(1024,0,NULL,NULL);
 *
 *  The return value from fft_alloc is a cfg buffer used internally
 *  by the fft routine or NULL.
 *
 *  If lenmem is NULL, then opus_fft_alloc will allocate a cfg buffer using malloc.
 *  The returned value should be free()d when done to avoid memory leaks.
 *
 *  The state can be placed in a user supplied buffer 'mem':
 *  If lenmem is not NULL and mem is not NULL and *lenmem is large enough,
 *      then the function places the cfg in mem and the size used in *lenmem
 *      and returns mem.
 *
 *  If lenmem is not NULL and ( mem is NULL or *lenmem is not large enough),
 *      then the function returns NULL and places the minimum cfg
 *      buffer size in *lenmem.
 * */
/* *
* opus_fft(cfg,in_out_buf)
*
* Perform an FFT on a complex input buffer.
* for a forward FFT,
* fin should be  f[0] , f[1] , ... ,f[nfft-1]
* fout will be   F[0] , F[1] , ... ,F[nfft-1]
* Note that each element is complex and can be accessed like
   f[k].r and f[k].i
* */
#[no_mangle]

pub unsafe extern "C" fn opus_ifft_c(
    mut st: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state,
    mut fin: *const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
    mut fout: *mut crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx,
) {
    let mut i: i32 = 0;
    /* Bit-reverse the input */
    i = 0 as i32;
    while i < (*st).nfft {
        *fout.offset(*(*st).bitrev.offset(i as isize) as isize) = *fin.offset(i as isize);
        i += 1
    }
    i = 0 as i32;
    while i < (*st).nfft {
        (*fout.offset(i as isize)).i = -(*fout.offset(i as isize)).i;
        i += 1
    }
    opus_fft_impl(st, fout);
    i = 0 as i32;
    while i < (*st).nfft {
        (*fout.offset(i as isize)).i = -(*fout.offset(i as isize)).i;
        i += 1
    }
}

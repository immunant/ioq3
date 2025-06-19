// =============== BEGIN mdct_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdct_lookup {
    pub n: i32,
    pub maxshift: i32,
    pub kfft: [*const crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state; 4],
    pub trig: *const f32,
}
use ::libc;

pub use crate::arch_h::opus_val16;
pub use crate::opus_types_h::opus_int16;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::int16_t;

pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::kiss_fft::opus_fft_impl;
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2008 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* This is a simple MDCT implementation that uses a N/4 complex FFT
   to do most of the work. It should be relatively straightforward to
   plug in pretty much and FFT here.

   This replaces the Vorbis FFT (and uses the exact same API), which
   was a bit too messy and that was ending up duplicating code
   (might as well use the same FFT everywhere).

   The algorithm is similar to (and inspired from) Fabrice Bellard's
   MDCT implementation in FFMPEG, but has differences in signs, ordering
   and scaling in many places.
*/
/* CUSTOM_MODES */
/* Forward MDCT trashes the input array */
#[no_mangle]

pub unsafe extern "C" fn clt_mdct_forward_c(
    mut l: *const crate::src::opus_1_2_1::celt::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
    mut window: *const opus_val16,
    mut overlap: i32,
    mut shift: i32,
    mut stride: i32,
    mut _arch: i32,
) {
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut N2: i32 = 0;
    let mut N4: i32 = 0;
    let mut f: *mut f32 = 0 as *mut f32;
    let mut f2: *mut kiss_fft_cpx = 0 as *mut kiss_fft_cpx;
    let mut st: *const kiss_fft_state = (*l).kfft[shift as usize];
    let mut trig: *const f32 = 0 as *const f32;
    let mut scale: opus_val16 = 0.;
    scale = (*st).scale;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as i32;
    while i < shift {
        N >>= 1 as i32;
        trig = trig.offset(N as isize);
        i += 1
    }
    N2 = N >> 1 as i32;
    N4 = N >> 2 as i32;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong).wrapping_mul(N2 as libc::c_ulong) as usize,
    );
    f = fresh0.as_mut_ptr() as *mut f32;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<kiss_fft_cpx>() as libc::c_ulong).wrapping_mul(N4 as libc::c_ulong)
            as usize,
    );
    f2 = fresh1.as_mut_ptr() as *mut kiss_fft_cpx;
    /* Consider the input to be composed of four blocks: [a, b, c, d] */
    /* Window, shuffle, fold */
    /* Temp pointers to make it really clear to the compiler what we're doing */
    let mut xp1: *const f32 = in_0.offset((overlap >> 1 as i32) as isize);
    let mut xp2: *const f32 = in_0
        .offset(N2 as isize)
        .offset(-(1 as i32 as isize))
        .offset((overlap >> 1 as i32) as isize);
    let mut yp: *mut f32 = f;
    let mut wp1: *const opus_val16 = window.offset((overlap >> 1 as i32) as isize);
    let mut wp2: *const opus_val16 = window
        .offset((overlap >> 1 as i32) as isize)
        .offset(-(1 as i32 as isize));
    i = 0 as i32;
    while i < overlap + 3 as i32 >> 2 as i32 {
        /* Real part arranged as -d-cR, Imag part arranged as -b+aR*/
        let fresh2 = yp;
        yp = yp.offset(1);
        *fresh2 = *wp2 * *xp1.offset(N2 as isize) + *wp1 * *xp2;
        let fresh3 = yp;
        yp = yp.offset(1);
        *fresh3 = *wp1 * *xp1 - *wp2 * *xp2.offset(-N2 as isize);
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        wp1 = wp1.offset(2 as i32 as isize);
        wp2 = wp2.offset(-(2 as i32 as isize));
        i += 1
    }
    wp1 = window;
    wp2 = window.offset(overlap as isize).offset(-(1 as i32 as isize));
    while i < N4 - (overlap + 3 as i32 >> 2 as i32) {
        /* Real part arranged as a-bR, Imag part arranged as -c-dR */
        let fresh4 = yp;
        yp = yp.offset(1);
        *fresh4 = *xp2;
        let fresh5 = yp;
        yp = yp.offset(1);
        *fresh5 = *xp1;
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        i += 1
    }
    while i < N4 {
        /* Real part arranged as a-bR, Imag part arranged as -c-dR */
        let fresh6 = yp;
        yp = yp.offset(1);
        *fresh6 = -(*wp1 * *xp1.offset(-N2 as isize)) + *wp2 * *xp2;
        let fresh7 = yp;
        yp = yp.offset(1);
        *fresh7 = *wp2 * *xp1 + *wp1 * *xp2.offset(N2 as isize);
        xp1 = xp1.offset(2 as i32 as isize);
        xp2 = xp2.offset(-(2 as i32 as isize));
        wp1 = wp1.offset(2 as i32 as isize);
        wp2 = wp2.offset(-(2 as i32 as isize));
        i += 1
    }
    /* Pre-rotation */
    let mut yp_0: *mut f32 = f;
    let mut t: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    i = 0 as i32;
    while i < N4 {
        let mut yc: kiss_fft_cpx = kiss_fft_cpx { r: 0., i: 0. };
        let mut t0: f32 = 0.;
        let mut t1: f32 = 0.;
        let mut re: f32 = 0.;
        let mut im: f32 = 0.;
        let mut yr: f32 = 0.;
        let mut yi: f32 = 0.;
        t0 = *t.offset(i as isize);
        t1 = *t.offset((N4 + i) as isize);
        let fresh8 = yp_0;
        yp_0 = yp_0.offset(1);
        re = *fresh8;
        let fresh9 = yp_0;
        yp_0 = yp_0.offset(1);
        im = *fresh9;
        yr = re * t0 - im * t1;
        yi = im * t0 + re * t1;
        yc.r = yr;
        yc.i = yi;
        yc.r = scale * yc.r;
        yc.i = scale * yc.i;
        *f2.offset(*(*st).bitrev.offset(i as isize) as isize) = yc;
        i += 1
    }
    /* N/4 complex FFT, does not downscale anymore */
    opus_fft_impl(st as *const kiss_fft_state, f2 as *mut kiss_fft_cpx);
    /* Post-rotate */
    /* Temp pointers to make it really clear to the compiler what we're doing */
    let mut fp: *const kiss_fft_cpx = f2;
    let mut yp1: *mut f32 = out;
    let mut yp2: *mut f32 = out.offset((stride * (N2 - 1 as i32)) as isize);
    let mut t_0: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    i = 0 as i32;
    while i < N4 {
        let mut yr_0: f32 = 0.;
        let mut yi_0: f32 = 0.;
        yr_0 = (*fp).i * *t_0.offset((N4 + i) as isize) - (*fp).r * *t_0.offset(i as isize);
        yi_0 = (*fp).r * *t_0.offset((N4 + i) as isize) + (*fp).i * *t_0.offset(i as isize);
        *yp1 = yr_0;
        *yp2 = yi_0;
        fp = fp.offset(1);
        yp1 = yp1.offset((2 as i32 * stride) as isize);
        yp2 = yp2.offset(-((2 as i32 * stride) as isize));
        i += 1
    }
}
/* Temp pointers to make it really clear to the compiler what we're doing */
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2008 Xiph.Org Foundation
Written by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* This is a simple MDCT implementation that uses a N/4 complex FFT
   to do most of the work. It should be relatively straightforward to
   plug in pretty much and FFT here.

   This replaces the Vorbis FFT (and uses the exact same API), which
   was a bit too messy and that was ending up duplicating code
   (might as well use the same FFT everywhere).

   The algorithm is similar to (and inspired from) Fabrice Bellard's
   MDCT implementation in FFMPEG, but has differences in signs, ordering
   and scaling in many places.
*/
/* * Compute a forward MDCT and scale by 4/N, trashes the input array */
/* * Compute a backward MDCT (no scaling) and performs weighted overlap-add
(scales implicitly by 1/2) */
/* OVERRIDE_clt_mdct_forward */
#[no_mangle]

pub unsafe extern "C" fn clt_mdct_backward_c(
    mut l: *const crate::src::opus_1_2_1::celt::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
    mut window: *const opus_val16,
    mut overlap: i32,
    mut shift: i32,
    mut stride: i32,
    mut _arch: i32,
) {
    let mut i: i32 = 0;
    let mut N: i32 = 0;
    let mut N2: i32 = 0;
    let mut N4: i32 = 0;
    let mut trig: *const f32 = 0 as *const f32;
    N = (*l).n;
    trig = (*l).trig;
    i = 0 as i32;
    while i < shift {
        N >>= 1 as i32;
        trig = trig.offset(N as isize);
        i += 1
    }
    N2 = N >> 1 as i32;
    N4 = N >> 2 as i32;
    /* Pre-rotate */
    /* Temp pointers to make it really clear to the compiler what we're doing */
    let mut xp1: *const f32 = in_0;
    let mut xp2: *const f32 = in_0.offset((stride * (N2 - 1 as i32)) as isize);
    let mut yp: *mut f32 = out.offset((overlap >> 1 as i32) as isize);
    let mut t: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    let mut bitrev: *const opus_int16 = (*(*l).kfft[shift as usize]).bitrev;
    i = 0 as i32;
    while i < N4 {
        let mut rev: i32 = 0;
        let mut yr: f32 = 0.;
        let mut yi: f32 = 0.;
        let fresh10 = bitrev;
        bitrev = bitrev.offset(1);
        rev = *fresh10 as i32;
        yr = *xp2 * *t.offset(i as isize) + *xp1 * *t.offset((N4 + i) as isize);
        yi = *xp1 * *t.offset(i as isize) - *xp2 * *t.offset((N4 + i) as isize);
        /* We swap real and imag because we use an FFT instead of an IFFT. */
        *yp.offset((2 as i32 * rev + 1 as i32) as isize) = yr;
        *yp.offset((2 as i32 * rev) as isize) = yi;
        /* Storing the pre-rotation directly in the bitrev order. */
        xp1 = xp1.offset((2 as i32 * stride) as isize);
        xp2 = xp2.offset(-((2 as i32 * stride) as isize));
        i += 1
    }
    opus_fft_impl(
        (*l).kfft[shift as usize] as *const kiss_fft_state,
        out.offset((overlap >> 1 as i32) as isize) as *mut kiss_fft_cpx as *mut kiss_fft_cpx,
    );
    /* Post-rotate and de-shuffle from both ends of the buffer at once to make
    it in-place. */
    let mut yp0: *mut f32 = out.offset((overlap >> 1 as i32) as isize);
    let mut yp1: *mut f32 = out
        .offset((overlap >> 1 as i32) as isize)
        .offset(N2 as isize)
        .offset(-(2 as i32 as isize));
    let mut t_0: *const f32 = &*trig.offset(0 as i32 as isize) as *const f32;
    /* Loop to (N4+1)>>1 to handle odd N4. When N4 is odd, the
    middle pair will be computed twice. */
    i = 0 as i32;
    while i < N4 + 1 as i32 >> 1 as i32 {
        let mut re: f32 = 0.;
        let mut im: f32 = 0.;
        let mut yr_0: f32 = 0.;
        let mut yi_0: f32 = 0.;
        let mut t0: f32 = 0.;
        let mut t1: f32 = 0.;
        /* We swap real and imag because we're using an FFT instead of an IFFT. */
        re = *yp0.offset(1 as i32 as isize);
        im = *yp0.offset(0 as i32 as isize);
        t0 = *t_0.offset(i as isize);
        t1 = *t_0.offset((N4 + i) as isize);
        /* We'd scale up by 2 here, but instead it's done when mixing the windows */
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        /* We swap real and imag because we're using an FFT instead of an IFFT. */
        re = *yp1.offset(1 as i32 as isize);
        im = *yp1.offset(0 as i32 as isize);
        *yp0.offset(0 as i32 as isize) = yr_0;
        *yp1.offset(1 as i32 as isize) = yi_0;
        t0 = *t_0.offset((N4 - i - 1 as i32) as isize);
        t1 = *t_0.offset((N2 - i - 1 as i32) as isize);
        /* We'd scale up by 2 here, but instead it's done when mixing the windows */
        yr_0 = re * t0 + im * t1;
        yi_0 = re * t1 - im * t0;
        *yp1.offset(0 as i32 as isize) = yr_0;
        *yp0.offset(1 as i32 as isize) = yi_0;
        yp0 = yp0.offset(2 as i32 as isize);
        yp1 = yp1.offset(-(2 as i32 as isize));
        i += 1
    }
    /* Mirror on both sides for TDAC */
    let mut xp1_0: *mut f32 = out.offset(overlap as isize).offset(-(1 as i32 as isize));
    let mut yp1_0: *mut f32 = out;
    let mut wp1: *const opus_val16 = window;
    let mut wp2: *const opus_val16 = window.offset(overlap as isize).offset(-(1 as i32 as isize));
    i = 0 as i32;
    while i < overlap / 2 as i32 {
        let mut x1: f32 = 0.;
        let mut x2: f32 = 0.;
        x1 = *xp1_0;
        x2 = *yp1_0;
        let fresh11 = yp1_0;
        yp1_0 = yp1_0.offset(1);
        *fresh11 = *wp2 * x2 - *wp1 * x1;
        let fresh12 = xp1_0;
        xp1_0 = xp1_0.offset(-1);
        *fresh12 = *wp1 * x2 + *wp2 * x1;
        wp1 = wp1.offset(1);
        wp2 = wp2.offset(-1);
        i += 1
    }
}
/* OVERRIDE_clt_mdct_backward */

// =============== BEGIN mdct_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdct_lookup {
    pub n: i32,
    pub log2n: i32,
    pub trig: *mut f32,
    pub bitrev: *mut i32,
    pub scale: f32,
}
use ::libc;

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

function: normalized modified discrete cosine transform
          power of two length transform only [64 <= n ]

Original algorithm adapted long ago from _The use of multirate filter
banks for coding of high quality digital audio_, by T. Sporer,
K. Brandenburg and B. Edler, collection of the European Signal
Processing Conference (EUSIPCO), Amsterdam, June 1992, Vol.1, pp
211-214

The below code implements an algorithm that no longer looks much like
that presented in the paper, but the basic structure remains if you
dig deep enough to see it.

This module DOES NOT INCLUDE code to generate/apply the window
function.  Everybody has their own weird favorite including me... I
happen to like the properties of y=sin(.5PI*sin^2(x)), but others may
vehemently disagree.

********************************************************************/
/* this can also be run as an integer transform by uncommenting a
define in mdct.h; the integerization is a first pass and although
it's likely stable for Vorbis, the dynamic range is constrained and
roundoff isn't done (so it's noisy).  Consider it functional, but
only a starting point.  There's no point on a machine with an FPU */
/* build lookups for trig functions; also pre-figure scaling and
some window function algebra. */
#[no_mangle]

pub unsafe extern "C" fn mdct_init(
    mut lookup: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut n: i32,
) {
    let mut bitrev: *mut i32 = crate::stdlib::malloc(
        (::std::mem::size_of::<i32>() as usize)
            .wrapping_mul((n / 4 as i32) as usize),
    ) as *mut i32;
    let mut T: *mut f32 = crate::stdlib::malloc(
        (::std::mem::size_of::<f32>() as usize)
            .wrapping_mul((n + n / 4 as i32) as usize),
    ) as *mut f32;
    let mut i: i32 = 0;
    let mut n2: i32 = n >> 1 as i32;
    (*lookup).log2n = crate::stdlib::rint(
        crate::stdlib::log(n as f32 as f64) / crate::stdlib::log(2.0f32 as f64),
    ) as i32;
    let mut log2n: i32 = (*lookup).log2n;
    (*lookup).n = n;
    (*lookup).trig = T;
    (*lookup).bitrev = bitrev;
    /* trig lookups... */
    i = 0 as i32;
    while i < n / 4 as i32 {
        *T.offset((i * 2 as i32) as isize) =
            crate::stdlib::cos(3.14159265358979323846f64 / n as f64 * (4 as i32 * i) as f64) as f32;
        *T.offset((i * 2 as i32 + 1 as i32) as isize) =
            -crate::stdlib::sin(3.14159265358979323846f64 / n as f64 * (4 as i32 * i) as f64)
                as f32;
        *T.offset((n2 + i * 2 as i32) as isize) = crate::stdlib::cos(
            3.14159265358979323846f64 / (2 as i32 * n) as f64 * (2 as i32 * i + 1 as i32) as f64,
        ) as f32;
        *T.offset((n2 + i * 2 as i32 + 1 as i32) as isize) = crate::stdlib::sin(
            3.14159265358979323846f64 / (2 as i32 * n) as f64 * (2 as i32 * i + 1 as i32) as f64,
        ) as f32;
        i += 1
    }
    i = 0 as i32;
    while i < n / 8 as i32 {
        *T.offset((n + i * 2 as i32) as isize) = (crate::stdlib::cos(
            3.14159265358979323846f64 / n as f64 * (4 as i32 * i + 2 as i32) as f64,
        ) * 0.5f64) as f32;
        *T.offset((n + i * 2 as i32 + 1 as i32) as isize) = (-crate::stdlib::sin(
            3.14159265358979323846f64 / n as f64 * (4 as i32 * i + 2 as i32) as f64,
        ) * 0.5f64) as f32;
        i += 1
    }
    /* bitreverse lookup... */
    let mut mask: i32 = ((1 as i32) << log2n - 1 as i32) - 1 as i32;
    let mut i_0: i32 = 0;
    let mut j: i32 = 0;
    let mut msb: i32 = (1 as i32) << log2n - 2 as i32;
    i_0 = 0 as i32;
    while i_0 < n / 8 as i32 {
        let mut acc: i32 = 0 as i32;
        j = 0 as i32;
        while msb >> j != 0 {
            if msb >> j & i_0 != 0 {
                acc |= (1 as i32) << j
            }
            j += 1
        }
        *bitrev.offset((i_0 * 2 as i32) as isize) = (!acc & mask) - 1 as i32;
        *bitrev.offset((i_0 * 2 as i32 + 1 as i32) as isize) = acc;
        i_0 += 1
    }
    (*lookup).scale = 4.0f32 / n as f32;
}
/* 8 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_8(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(6 as i32 as isize) + *x.offset(2 as i32 as isize);
    let mut r1: f32 = *x.offset(6 as i32 as isize) - *x.offset(2 as i32 as isize);
    let mut r2: f32 = *x.offset(4 as i32 as isize) + *x.offset(0 as i32 as isize);
    let mut r3: f32 = *x.offset(4 as i32 as isize) - *x.offset(0 as i32 as isize);
    *x.offset(6 as i32 as isize) = r0 + r2;
    *x.offset(4 as i32 as isize) = r0 - r2;
    r0 = *x.offset(5 as i32 as isize) - *x.offset(1 as i32 as isize);
    r2 = *x.offset(7 as i32 as isize) - *x.offset(3 as i32 as isize);
    *x.offset(0 as i32 as isize) = r1 + r0;
    *x.offset(2 as i32 as isize) = r1 - r0;
    r0 = *x.offset(5 as i32 as isize) + *x.offset(1 as i32 as isize);
    r1 = *x.offset(7 as i32 as isize) + *x.offset(3 as i32 as isize);
    *x.offset(3 as i32 as isize) = r2 + r3;
    *x.offset(1 as i32 as isize) = r2 - r3;
    *x.offset(7 as i32 as isize) = r1 + r0;
    *x.offset(5 as i32 as isize) = r1 - r0;
}
/* 16 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_16(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(1 as i32 as isize) - *x.offset(9 as i32 as isize);
    let mut r1: f32 = *x.offset(0 as i32 as isize) - *x.offset(8 as i32 as isize);
    *x.offset(8 as i32 as isize) += *x.offset(0 as i32 as isize);
    *x.offset(9 as i32 as isize) += *x.offset(1 as i32 as isize);
    *x.offset(0 as i32 as isize) = (r0 + r1) * 0.70710678118654752441f32;
    *x.offset(1 as i32 as isize) = (r0 - r1) * 0.70710678118654752441f32;
    r0 = *x.offset(3 as i32 as isize) - *x.offset(11 as i32 as isize);
    r1 = *x.offset(10 as i32 as isize) - *x.offset(2 as i32 as isize);
    *x.offset(10 as i32 as isize) += *x.offset(2 as i32 as isize);
    *x.offset(11 as i32 as isize) += *x.offset(3 as i32 as isize);
    *x.offset(2 as i32 as isize) = r0;
    *x.offset(3 as i32 as isize) = r1;
    r0 = *x.offset(12 as i32 as isize) - *x.offset(4 as i32 as isize);
    r1 = *x.offset(13 as i32 as isize) - *x.offset(5 as i32 as isize);
    *x.offset(12 as i32 as isize) += *x.offset(4 as i32 as isize);
    *x.offset(13 as i32 as isize) += *x.offset(5 as i32 as isize);
    *x.offset(4 as i32 as isize) = (r0 - r1) * 0.70710678118654752441f32;
    *x.offset(5 as i32 as isize) = (r0 + r1) * 0.70710678118654752441f32;
    r0 = *x.offset(14 as i32 as isize) - *x.offset(6 as i32 as isize);
    r1 = *x.offset(15 as i32 as isize) - *x.offset(7 as i32 as isize);
    *x.offset(14 as i32 as isize) += *x.offset(6 as i32 as isize);
    *x.offset(15 as i32 as isize) += *x.offset(7 as i32 as isize);
    *x.offset(6 as i32 as isize) = r0;
    *x.offset(7 as i32 as isize) = r1;
    mdct_butterfly_8(x);
    mdct_butterfly_8(x.offset(8 as i32 as isize));
}
/* 32 point butterfly (in place, 4 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_32(mut x: *mut f32) {
    let mut r0: f32 = *x.offset(30 as i32 as isize) - *x.offset(14 as i32 as isize);
    let mut r1: f32 = *x.offset(31 as i32 as isize) - *x.offset(15 as i32 as isize);
    *x.offset(30 as i32 as isize) += *x.offset(14 as i32 as isize);
    *x.offset(31 as i32 as isize) += *x.offset(15 as i32 as isize);
    *x.offset(14 as i32 as isize) = r0;
    *x.offset(15 as i32 as isize) = r1;
    r0 = *x.offset(28 as i32 as isize) - *x.offset(12 as i32 as isize);
    r1 = *x.offset(29 as i32 as isize) - *x.offset(13 as i32 as isize);
    *x.offset(28 as i32 as isize) += *x.offset(12 as i32 as isize);
    *x.offset(29 as i32 as isize) += *x.offset(13 as i32 as isize);
    *x.offset(12 as i32 as isize) = r0 * 0.92387953251128675613f32 - r1 * 0.38268343236508977175f32;
    *x.offset(13 as i32 as isize) = r0 * 0.38268343236508977175f32 + r1 * 0.92387953251128675613f32;
    r0 = *x.offset(26 as i32 as isize) - *x.offset(10 as i32 as isize);
    r1 = *x.offset(27 as i32 as isize) - *x.offset(11 as i32 as isize);
    *x.offset(26 as i32 as isize) += *x.offset(10 as i32 as isize);
    *x.offset(27 as i32 as isize) += *x.offset(11 as i32 as isize);
    *x.offset(10 as i32 as isize) = (r0 - r1) * 0.70710678118654752441f32;
    *x.offset(11 as i32 as isize) = (r0 + r1) * 0.70710678118654752441f32;
    r0 = *x.offset(24 as i32 as isize) - *x.offset(8 as i32 as isize);
    r1 = *x.offset(25 as i32 as isize) - *x.offset(9 as i32 as isize);
    *x.offset(24 as i32 as isize) += *x.offset(8 as i32 as isize);
    *x.offset(25 as i32 as isize) += *x.offset(9 as i32 as isize);
    *x.offset(8 as i32 as isize) = r0 * 0.38268343236508977175f32 - r1 * 0.92387953251128675613f32;
    *x.offset(9 as i32 as isize) = r1 * 0.38268343236508977175f32 + r0 * 0.92387953251128675613f32;
    r0 = *x.offset(22 as i32 as isize) - *x.offset(6 as i32 as isize);
    r1 = *x.offset(7 as i32 as isize) - *x.offset(23 as i32 as isize);
    *x.offset(22 as i32 as isize) += *x.offset(6 as i32 as isize);
    *x.offset(23 as i32 as isize) += *x.offset(7 as i32 as isize);
    *x.offset(6 as i32 as isize) = r1;
    *x.offset(7 as i32 as isize) = r0;
    r0 = *x.offset(4 as i32 as isize) - *x.offset(20 as i32 as isize);
    r1 = *x.offset(5 as i32 as isize) - *x.offset(21 as i32 as isize);
    *x.offset(20 as i32 as isize) += *x.offset(4 as i32 as isize);
    *x.offset(21 as i32 as isize) += *x.offset(5 as i32 as isize);
    *x.offset(4 as i32 as isize) = r1 * 0.92387953251128675613f32 + r0 * 0.38268343236508977175f32;
    *x.offset(5 as i32 as isize) = r1 * 0.38268343236508977175f32 - r0 * 0.92387953251128675613f32;
    r0 = *x.offset(2 as i32 as isize) - *x.offset(18 as i32 as isize);
    r1 = *x.offset(3 as i32 as isize) - *x.offset(19 as i32 as isize);
    *x.offset(18 as i32 as isize) += *x.offset(2 as i32 as isize);
    *x.offset(19 as i32 as isize) += *x.offset(3 as i32 as isize);
    *x.offset(2 as i32 as isize) = (r1 + r0) * 0.70710678118654752441f32;
    *x.offset(3 as i32 as isize) = (r1 - r0) * 0.70710678118654752441f32;
    r0 = *x.offset(0 as i32 as isize) - *x.offset(16 as i32 as isize);
    r1 = *x.offset(1 as i32 as isize) - *x.offset(17 as i32 as isize);
    *x.offset(16 as i32 as isize) += *x.offset(0 as i32 as isize);
    *x.offset(17 as i32 as isize) += *x.offset(1 as i32 as isize);
    *x.offset(0 as i32 as isize) = r1 * 0.38268343236508977175f32 + r0 * 0.92387953251128675613f32;
    *x.offset(1 as i32 as isize) = r1 * 0.92387953251128675613f32 - r0 * 0.38268343236508977175f32;
    mdct_butterfly_16(x);
    mdct_butterfly_16(x.offset(16 as i32 as isize));
}
/* N point first stage butterfly (in place, 2 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_first(mut T: *mut f32, mut x: *mut f32, mut points: i32) {
    let mut x1: *mut f32 = x.offset(points as isize).offset(-(8 as i32 as isize));
    let mut x2: *mut f32 = x
        .offset((points >> 1 as i32) as isize)
        .offset(-(8 as i32 as isize));
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    loop {
        r0 = *x1.offset(6 as i32 as isize) - *x2.offset(6 as i32 as isize);
        r1 = *x1.offset(7 as i32 as isize) - *x2.offset(7 as i32 as isize);
        *x1.offset(6 as i32 as isize) += *x2.offset(6 as i32 as isize);
        *x1.offset(7 as i32 as isize) += *x2.offset(7 as i32 as isize);
        *x2.offset(6 as i32 as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *x2.offset(7 as i32 as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        r0 = *x1.offset(4 as i32 as isize) - *x2.offset(4 as i32 as isize);
        r1 = *x1.offset(5 as i32 as isize) - *x2.offset(5 as i32 as isize);
        *x1.offset(4 as i32 as isize) += *x2.offset(4 as i32 as isize);
        *x1.offset(5 as i32 as isize) += *x2.offset(5 as i32 as isize);
        *x2.offset(4 as i32 as isize) =
            r1 * *T.offset(5 as i32 as isize) + r0 * *T.offset(4 as i32 as isize);
        *x2.offset(5 as i32 as isize) =
            r1 * *T.offset(4 as i32 as isize) - r0 * *T.offset(5 as i32 as isize);
        r0 = *x1.offset(2 as i32 as isize) - *x2.offset(2 as i32 as isize);
        r1 = *x1.offset(3 as i32 as isize) - *x2.offset(3 as i32 as isize);
        *x1.offset(2 as i32 as isize) += *x2.offset(2 as i32 as isize);
        *x1.offset(3 as i32 as isize) += *x2.offset(3 as i32 as isize);
        *x2.offset(2 as i32 as isize) =
            r1 * *T.offset(9 as i32 as isize) + r0 * *T.offset(8 as i32 as isize);
        *x2.offset(3 as i32 as isize) =
            r1 * *T.offset(8 as i32 as isize) - r0 * *T.offset(9 as i32 as isize);
        r0 = *x1.offset(0 as i32 as isize) - *x2.offset(0 as i32 as isize);
        r1 = *x1.offset(1 as i32 as isize) - *x2.offset(1 as i32 as isize);
        *x1.offset(0 as i32 as isize) += *x2.offset(0 as i32 as isize);
        *x1.offset(1 as i32 as isize) += *x2.offset(1 as i32 as isize);
        *x2.offset(0 as i32 as isize) =
            r1 * *T.offset(13 as i32 as isize) + r0 * *T.offset(12 as i32 as isize);
        *x2.offset(1 as i32 as isize) =
            r1 * *T.offset(12 as i32 as isize) - r0 * *T.offset(13 as i32 as isize);
        x1 = x1.offset(-(8 as i32 as isize));
        x2 = x2.offset(-(8 as i32 as isize));
        T = T.offset(16 as i32 as isize);
        if !(x2 >= x) {
            break;
        }
    }
}
/* N/stage point generic N stage butterfly (in place, 2 register) */
#[inline]

unsafe extern "C" fn mdct_butterfly_generic(
    mut T: *mut f32,
    mut x: *mut f32,
    mut points: i32,
    mut trigint: i32,
) {
    let mut x1: *mut f32 = x.offset(points as isize).offset(-(8 as i32 as isize));
    let mut x2: *mut f32 = x
        .offset((points >> 1 as i32) as isize)
        .offset(-(8 as i32 as isize));
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    loop {
        r0 = *x1.offset(6 as i32 as isize) - *x2.offset(6 as i32 as isize);
        r1 = *x1.offset(7 as i32 as isize) - *x2.offset(7 as i32 as isize);
        *x1.offset(6 as i32 as isize) += *x2.offset(6 as i32 as isize);
        *x1.offset(7 as i32 as isize) += *x2.offset(7 as i32 as isize);
        *x2.offset(6 as i32 as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *x2.offset(7 as i32 as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(4 as i32 as isize) - *x2.offset(4 as i32 as isize);
        r1 = *x1.offset(5 as i32 as isize) - *x2.offset(5 as i32 as isize);
        *x1.offset(4 as i32 as isize) += *x2.offset(4 as i32 as isize);
        *x1.offset(5 as i32 as isize) += *x2.offset(5 as i32 as isize);
        *x2.offset(4 as i32 as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *x2.offset(5 as i32 as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(2 as i32 as isize) - *x2.offset(2 as i32 as isize);
        r1 = *x1.offset(3 as i32 as isize) - *x2.offset(3 as i32 as isize);
        *x1.offset(2 as i32 as isize) += *x2.offset(2 as i32 as isize);
        *x1.offset(3 as i32 as isize) += *x2.offset(3 as i32 as isize);
        *x2.offset(2 as i32 as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *x2.offset(3 as i32 as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        T = T.offset(trigint as isize);
        r0 = *x1.offset(0 as i32 as isize) - *x2.offset(0 as i32 as isize);
        r1 = *x1.offset(1 as i32 as isize) - *x2.offset(1 as i32 as isize);
        *x1.offset(0 as i32 as isize) += *x2.offset(0 as i32 as isize);
        *x1.offset(1 as i32 as isize) += *x2.offset(1 as i32 as isize);
        *x2.offset(0 as i32 as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *x2.offset(1 as i32 as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        T = T.offset(trigint as isize);
        x1 = x1.offset(-(8 as i32 as isize));
        x2 = x2.offset(-(8 as i32 as isize));
        if !(x2 >= x) {
            break;
        }
    }
}
#[inline]

unsafe extern "C" fn mdct_butterflies(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut x: *mut f32,
    mut points: i32,
) {
    let mut T: *mut f32 = (*init).trig;
    let mut stages: i32 = (*init).log2n - 5 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    stages -= 1;
    if stages > 0 as i32 {
        mdct_butterfly_first(T, x, points);
    }
    i = 1 as i32;
    loop {
        stages -= 1;
        if !(stages > 0 as i32) {
            break;
        }
        j = 0 as i32;
        while j < (1 as i32) << i {
            mdct_butterfly_generic(
                T,
                x.offset(((points >> i) * j) as isize),
                points >> i,
                (4 as i32) << i,
            );
            j += 1
        }
        i += 1
    }
    j = 0 as i32;
    while j < points {
        mdct_butterfly_32(x.offset(j as isize));
        j += 32 as i32
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_clear(
    mut l: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
) {
    if !l.is_null() {
        if !(*l).trig.is_null() {
            libc::free((*l).trig as *mut libc::c_void);
        }
        if !(*l).bitrev.is_null() {
            libc::free((*l).bitrev as *mut libc::c_void);
        }
        crate::stdlib::memset(
            l as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup>()
                as usize,
        );
    };
}
#[inline]

unsafe extern "C" fn mdct_bitreverse(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut x: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut bit: *mut i32 = (*init).bitrev;
    let mut w0: *mut f32 = x;
    x = w0.offset((n >> 1 as i32) as isize);
    let mut w1: *mut f32 = x;
    let mut T: *mut f32 = (*init).trig.offset(n as isize);
    loop {
        let mut x0: *mut f32 = x.offset(*bit.offset(0 as i32 as isize) as isize);
        let mut x1: *mut f32 = x.offset(*bit.offset(1 as i32 as isize) as isize);
        let mut r0: f32 = *x0.offset(1 as i32 as isize) - *x1.offset(1 as i32 as isize);
        let mut r1: f32 = *x0.offset(0 as i32 as isize) + *x1.offset(0 as i32 as isize);
        let mut r2: f32 = r1 * *T.offset(0 as i32 as isize) + r0 * *T.offset(1 as i32 as isize);
        let mut r3: f32 = r1 * *T.offset(1 as i32 as isize) - r0 * *T.offset(0 as i32 as isize);
        w1 = w1.offset(-(4 as i32 as isize));
        r0 = (*x0.offset(1 as i32 as isize) + *x1.offset(1 as i32 as isize)) * 0.5f32;
        r1 = (*x0.offset(0 as i32 as isize) - *x1.offset(0 as i32 as isize)) * 0.5f32;
        *w0.offset(0 as i32 as isize) = r0 + r2;
        *w1.offset(2 as i32 as isize) = r0 - r2;
        *w0.offset(1 as i32 as isize) = r1 + r3;
        *w1.offset(3 as i32 as isize) = r3 - r1;
        x0 = x.offset(*bit.offset(2 as i32 as isize) as isize);
        x1 = x.offset(*bit.offset(3 as i32 as isize) as isize);
        r0 = *x0.offset(1 as i32 as isize) - *x1.offset(1 as i32 as isize);
        r1 = *x0.offset(0 as i32 as isize) + *x1.offset(0 as i32 as isize);
        r2 = r1 * *T.offset(2 as i32 as isize) + r0 * *T.offset(3 as i32 as isize);
        r3 = r1 * *T.offset(3 as i32 as isize) - r0 * *T.offset(2 as i32 as isize);
        r0 = (*x0.offset(1 as i32 as isize) + *x1.offset(1 as i32 as isize)) * 0.5f32;
        r1 = (*x0.offset(0 as i32 as isize) - *x1.offset(0 as i32 as isize)) * 0.5f32;
        *w0.offset(2 as i32 as isize) = r0 + r2;
        *w1.offset(0 as i32 as isize) = r0 - r2;
        *w0.offset(3 as i32 as isize) = r1 + r3;
        *w1.offset(1 as i32 as isize) = r3 - r1;
        T = T.offset(4 as i32 as isize);
        bit = bit.offset(4 as i32 as isize);
        w0 = w0.offset(4 as i32 as isize);
        if !(w0 < w1) {
            break;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_backward(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut n2: i32 = n >> 1 as i32;
    let mut n4: i32 = n >> 2 as i32;
    /* rotate */
    let mut iX: *mut f32 = in_0.offset(n2 as isize).offset(-(7 as i32 as isize));
    let mut oX: *mut f32 = out.offset(n2 as isize).offset(n4 as isize);
    let mut T: *mut f32 = (*init).trig.offset(n4 as isize);
    loop {
        oX = oX.offset(-(4 as i32 as isize));
        *oX.offset(0 as i32 as isize) = -*iX.offset(2 as i32 as isize)
            * *T.offset(3 as i32 as isize)
            - *iX.offset(0 as i32 as isize) * *T.offset(2 as i32 as isize);
        *oX.offset(1 as i32 as isize) = *iX.offset(0 as i32 as isize)
            * *T.offset(3 as i32 as isize)
            - *iX.offset(2 as i32 as isize) * *T.offset(2 as i32 as isize);
        *oX.offset(2 as i32 as isize) = -*iX.offset(6 as i32 as isize)
            * *T.offset(1 as i32 as isize)
            - *iX.offset(4 as i32 as isize) * *T.offset(0 as i32 as isize);
        *oX.offset(3 as i32 as isize) = *iX.offset(4 as i32 as isize)
            * *T.offset(1 as i32 as isize)
            - *iX.offset(6 as i32 as isize) * *T.offset(0 as i32 as isize);
        iX = iX.offset(-(8 as i32 as isize));
        T = T.offset(4 as i32 as isize);
        if !(iX >= in_0) {
            break;
        }
    }
    iX = in_0.offset(n2 as isize).offset(-(8 as i32 as isize));
    oX = out.offset(n2 as isize).offset(n4 as isize);
    T = (*init).trig.offset(n4 as isize);
    loop {
        T = T.offset(-(4 as i32 as isize));
        *oX.offset(0 as i32 as isize) = *iX.offset(4 as i32 as isize)
            * *T.offset(3 as i32 as isize)
            + *iX.offset(6 as i32 as isize) * *T.offset(2 as i32 as isize);
        *oX.offset(1 as i32 as isize) = *iX.offset(4 as i32 as isize)
            * *T.offset(2 as i32 as isize)
            - *iX.offset(6 as i32 as isize) * *T.offset(3 as i32 as isize);
        *oX.offset(2 as i32 as isize) = *iX.offset(0 as i32 as isize)
            * *T.offset(1 as i32 as isize)
            + *iX.offset(2 as i32 as isize) * *T.offset(0 as i32 as isize);
        *oX.offset(3 as i32 as isize) = *iX.offset(0 as i32 as isize)
            * *T.offset(0 as i32 as isize)
            - *iX.offset(2 as i32 as isize) * *T.offset(1 as i32 as isize);
        iX = iX.offset(-(8 as i32 as isize));
        oX = oX.offset(4 as i32 as isize);
        if !(iX >= in_0) {
            break;
        }
    }
    mdct_butterflies(init, out.offset(n2 as isize), n2);
    mdct_bitreverse(init, out);
    /* roatate + window */
    let mut oX1: *mut f32 = out.offset(n2 as isize).offset(n4 as isize); /* forward needs working space */
    let mut oX2: *mut f32 = out.offset(n2 as isize).offset(n4 as isize);
    let mut iX_0: *mut f32 = out;
    T = (*init).trig.offset(n2 as isize);
    loop {
        oX1 = oX1.offset(-(4 as i32 as isize));
        *oX1.offset(3 as i32 as isize) = *iX_0.offset(0 as i32 as isize)
            * *T.offset(1 as i32 as isize)
            - *iX_0.offset(1 as i32 as isize) * *T.offset(0 as i32 as isize);
        *oX2.offset(0 as i32 as isize) = -(*iX_0.offset(0 as i32 as isize)
            * *T.offset(0 as i32 as isize)
            + *iX_0.offset(1 as i32 as isize) * *T.offset(1 as i32 as isize));
        *oX1.offset(2 as i32 as isize) = *iX_0.offset(2 as i32 as isize)
            * *T.offset(3 as i32 as isize)
            - *iX_0.offset(3 as i32 as isize) * *T.offset(2 as i32 as isize);
        *oX2.offset(1 as i32 as isize) = -(*iX_0.offset(2 as i32 as isize)
            * *T.offset(2 as i32 as isize)
            + *iX_0.offset(3 as i32 as isize) * *T.offset(3 as i32 as isize));
        *oX1.offset(1 as i32 as isize) = *iX_0.offset(4 as i32 as isize)
            * *T.offset(5 as i32 as isize)
            - *iX_0.offset(5 as i32 as isize) * *T.offset(4 as i32 as isize);
        *oX2.offset(2 as i32 as isize) = -(*iX_0.offset(4 as i32 as isize)
            * *T.offset(4 as i32 as isize)
            + *iX_0.offset(5 as i32 as isize) * *T.offset(5 as i32 as isize));
        *oX1.offset(0 as i32 as isize) = *iX_0.offset(6 as i32 as isize)
            * *T.offset(7 as i32 as isize)
            - *iX_0.offset(7 as i32 as isize) * *T.offset(6 as i32 as isize);
        *oX2.offset(3 as i32 as isize) = -(*iX_0.offset(6 as i32 as isize)
            * *T.offset(6 as i32 as isize)
            + *iX_0.offset(7 as i32 as isize) * *T.offset(7 as i32 as isize));
        oX2 = oX2.offset(4 as i32 as isize);
        iX_0 = iX_0.offset(8 as i32 as isize);
        T = T.offset(8 as i32 as isize);
        if !(iX_0 < oX1) {
            break;
        }
    }
    iX_0 = out.offset(n2 as isize).offset(n4 as isize);
    oX1 = out.offset(n4 as isize);
    oX2 = oX1;
    loop {
        oX1 = oX1.offset(-(4 as i32 as isize));
        iX_0 = iX_0.offset(-(4 as i32 as isize));
        let ref mut fresh0 = *oX1.offset(3 as i32 as isize);
        *fresh0 = *iX_0.offset(3 as i32 as isize);
        *oX2.offset(0 as i32 as isize) = -*fresh0;
        let ref mut fresh1 = *oX1.offset(2 as i32 as isize);
        *fresh1 = *iX_0.offset(2 as i32 as isize);
        *oX2.offset(1 as i32 as isize) = -*fresh1;
        let ref mut fresh2 = *oX1.offset(1 as i32 as isize);
        *fresh2 = *iX_0.offset(1 as i32 as isize);
        *oX2.offset(2 as i32 as isize) = -*fresh2;
        let ref mut fresh3 = *oX1.offset(0 as i32 as isize);
        *fresh3 = *iX_0.offset(0 as i32 as isize);
        *oX2.offset(3 as i32 as isize) = -*fresh3;
        oX2 = oX2.offset(4 as i32 as isize);
        if !(oX2 < iX_0) {
            break;
        }
    }
    iX_0 = out.offset(n2 as isize).offset(n4 as isize);
    oX1 = out.offset(n2 as isize).offset(n4 as isize);
    oX2 = out.offset(n2 as isize);
    loop {
        oX1 = oX1.offset(-(4 as i32 as isize));
        *oX1.offset(0 as i32 as isize) = *iX_0.offset(3 as i32 as isize);
        *oX1.offset(1 as i32 as isize) = *iX_0.offset(2 as i32 as isize);
        *oX1.offset(2 as i32 as isize) = *iX_0.offset(1 as i32 as isize);
        *oX1.offset(3 as i32 as isize) = *iX_0.offset(0 as i32 as isize);
        iX_0 = iX_0.offset(4 as i32 as isize);
        if !(oX1 > oX2) {
            break;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn mdct_forward(
    mut init: *mut crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup,
    mut in_0: *mut f32,
    mut out: *mut f32,
) {
    let mut n: i32 = (*init).n;
    let mut n2: i32 = n >> 1 as i32;
    let mut n4: i32 = n >> 2 as i32;
    let mut n8: i32 = n >> 3 as i32;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (n as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize) as usize,
    );
    let mut w: *mut f32 = fresh4.as_mut_ptr() as *mut f32;
    let mut w2: *mut f32 = w.offset(n2 as isize);
    /* rotate */
    /* window + rotate + step 1 */
    let mut r0: f32 = 0.;
    let mut r1: f32 = 0.;
    let mut x0: *mut f32 = in_0.offset(n2 as isize).offset(n4 as isize);
    let mut x1: *mut f32 = x0.offset(1 as i32 as isize);
    let mut T: *mut f32 = (*init).trig.offset(n2 as isize);
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    while i < n8 {
        x0 = x0.offset(-(4 as i32 as isize));
        T = T.offset(-(2 as i32 as isize));
        r0 = *x0.offset(2 as i32 as isize) + *x1.offset(0 as i32 as isize);
        r1 = *x0.offset(0 as i32 as isize) + *x1.offset(2 as i32 as isize);
        *w2.offset(i as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *w2.offset((i + 1 as i32) as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        x1 = x1.offset(4 as i32 as isize);
        i += 2 as i32
    }
    x1 = in_0.offset(1 as i32 as isize);
    while i < n2 - n8 {
        T = T.offset(-(2 as i32 as isize));
        x0 = x0.offset(-(4 as i32 as isize));
        r0 = *x0.offset(2 as i32 as isize) - *x1.offset(0 as i32 as isize);
        r1 = *x0.offset(0 as i32 as isize) - *x1.offset(2 as i32 as isize);
        *w2.offset(i as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *w2.offset((i + 1 as i32) as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        x1 = x1.offset(4 as i32 as isize);
        i += 2 as i32
    }
    x0 = in_0.offset(n as isize);
    while i < n2 {
        T = T.offset(-(2 as i32 as isize));
        x0 = x0.offset(-(4 as i32 as isize));
        r0 = -*x0.offset(2 as i32 as isize) - *x1.offset(0 as i32 as isize);
        r1 = -*x0.offset(0 as i32 as isize) - *x1.offset(2 as i32 as isize);
        *w2.offset(i as isize) =
            r1 * *T.offset(1 as i32 as isize) + r0 * *T.offset(0 as i32 as isize);
        *w2.offset((i + 1 as i32) as isize) =
            r1 * *T.offset(0 as i32 as isize) - r0 * *T.offset(1 as i32 as isize);
        x1 = x1.offset(4 as i32 as isize);
        i += 2 as i32
    }
    mdct_butterflies(init, w.offset(n2 as isize), n2);
    mdct_bitreverse(init, w);
    /* roatate + window */
    T = (*init).trig.offset(n2 as isize);
    x0 = out.offset(n2 as isize);
    i = 0 as i32;
    while i < n4 {
        x0 = x0.offset(-1);
        *out.offset(i as isize) = (*w.offset(0 as i32 as isize) * *T.offset(0 as i32 as isize)
            + *w.offset(1 as i32 as isize) * *T.offset(1 as i32 as isize))
            * (*init).scale;
        *x0.offset(0 as i32 as isize) = (*w.offset(0 as i32 as isize)
            * *T.offset(1 as i32 as isize)
            - *w.offset(1 as i32 as isize) * *T.offset(0 as i32 as isize))
            * (*init).scale;
        w = w.offset(2 as i32 as isize);
        T = T.offset(2 as i32 as isize);
        i += 1
    }
}

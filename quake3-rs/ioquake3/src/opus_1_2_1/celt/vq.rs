use ::libc;

pub mod entcode_h {
    /*OPT: ec_window must be at least 32 bits, but if you have fast arithmetic on a
    larger type, you can speed up the decoder by using it here.*/

    /*The number of bits to use for the range-coded part of unsigned integers.*/
    /*The resolution of fractional-precision bit usage measurements, i.e.,
    3 => 1/8th bits.*/
    /*The entropy encoder/decoder context.
    We use the same structure for both, so that common functions like ec_tell()
     can be used on either one.*/

    /* Tested exhaustively for all n and for 1<=d<=256 */
    #[inline]

    pub unsafe extern "C" fn celt_udiv(
        mut n: crate::opus_types_h::opus_uint32,
        mut d: crate::opus_types_h::opus_uint32,
    ) -> crate::opus_types_h::opus_uint32 {
        return n.wrapping_div(d);
    }
}

pub mod mathops_h {
    #[inline]

    pub unsafe extern "C" fn fast_atan2f(mut y: f32, mut x: f32) -> f32 {
        let mut x2: f32 = 0.;
        let mut y2: f32 = 0.;
        x2 = x * x;
        y2 = y * y;
        if x2 + y2 < 1e-18f32 {
            return 0 as i32 as f32;
        }
        if x2 < y2 {
            let mut den: f32 = (y2 + 0.67848403f32 * x2) * (y2 + 0.08595542f32 * x2);
            return -x * y * (y2 + 0.43157974f32 * x2) / den
                + (if y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                });
        } else {
            let mut den_0: f32 = (x2 + 0.67848403f32 * y2) * (x2 + 0.08595542f32 * y2);
            return x * y * (x2 + 0.43157974f32 * y2) / den_0
                + (if y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                })
                - (if x * y < 0 as i32 as f32 {
                    -(3.141592653f32 / 2 as i32 as f32)
                } else {
                    (3.141592653f32) / 2 as i32 as f32
                });
        };
    }
    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod pitch_h {
    /*We make sure a C version is always available for cases where the overhead of
    vectorization and passing around an arch flag aren't worth it.*/
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: i32,
    ) -> crate::arch_h::opus_val32 {
        let mut i: i32 = 0;
        let mut xy: crate::arch_h::opus_val32 = 0 as i32 as crate::arch_h::opus_val32;
        i = 0 as i32;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1
        }
        return xy;
    }
}
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_norm;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;

pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;
pub use crate::src::opus_1_2_1::celt::vq::entcode_h::celt_udiv;
pub use crate::src::opus_1_2_1::celt::vq::mathops_h::fast_atan2f;
pub use crate::src::opus_1_2_1::celt::vq::pitch_h::celt_inner_prod_c;

/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
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

unsafe extern "C" fn exp_rotation1(
    mut X: *mut crate::arch_h::celt_norm,
    mut len: i32,
    mut stride: i32,
    mut c: crate::arch_h::opus_val16,
    mut s: crate::arch_h::opus_val16,
) {
    let mut i: i32 = 0;
    let mut ms: crate::arch_h::opus_val16 = 0.;
    let mut Xptr: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    Xptr = X;
    ms = -s;
    i = 0 as i32;
    while i < len - stride {
        let mut x1: crate::arch_h::celt_norm = 0.;
        let mut x2: crate::arch_h::celt_norm = 0.;
        x1 = *Xptr.offset(0 as i32 as isize);
        x2 = *Xptr.offset(stride as isize);
        *Xptr.offset(stride as isize) = c * x2 + s * x1;
        let fresh0 = Xptr;
        Xptr = Xptr.offset(1);
        *fresh0 = c * x1 + ms * x2;
        i += 1
    }
    Xptr = &mut *X.offset((len - 2 as i32 * stride - 1 as i32) as isize)
        as *mut crate::arch_h::celt_norm;
    i = len - 2 as i32 * stride - 1 as i32;
    while i >= 0 as i32 {
        let mut x1_0: crate::arch_h::celt_norm = 0.;
        let mut x2_0: crate::arch_h::celt_norm = 0.;
        x1_0 = *Xptr.offset(0 as i32 as isize);
        x2_0 = *Xptr.offset(stride as isize);
        *Xptr.offset(stride as isize) = c * x2_0 + s * x1_0;
        let fresh1 = Xptr;
        Xptr = Xptr.offset(-1);
        *fresh1 = c * x1_0 + ms * x2_0;
        i -= 1
    }
}
/* OVERRIDE_vq_exp_rotation1 */
#[no_mangle]

pub unsafe extern "C" fn exp_rotation(
    mut X: *mut crate::arch_h::celt_norm,
    mut len: i32,
    mut dir: i32,
    mut stride: i32,
    mut K: i32,
    mut spread: i32,
) {
    static mut SPREAD_FACTOR: [i32; 3] = [15 as i32, 10 as i32, 5 as i32]; /*  sin(theta) */
    let mut i: i32 = 0;
    let mut c: crate::arch_h::opus_val16 = 0.;
    let mut s: crate::arch_h::opus_val16 = 0.;
    let mut gain: crate::arch_h::opus_val16 = 0.;
    let mut theta: crate::arch_h::opus_val16 = 0.;
    let mut stride2: i32 = 0 as i32;
    let mut factor: i32 = 0;
    if 2 as i32 * K >= len || spread == 0 as i32 {
        return;
    }
    factor = SPREAD_FACTOR[(spread - 1 as i32) as usize];
    gain =
        1.0f32 * len as crate::arch_h::opus_val32 / (len + factor * K) as crate::arch_h::opus_val32;
    theta = 0.5f32 * (gain * gain);
    c = crate::stdlib::cos((0.5f32 * 3.141592653f32 * theta) as f64) as f32;
    s = crate::stdlib::cos((0.5f32 * 3.141592653f32 * (1.0f32 - theta)) as f64) as f32;
    if len >= 8 as i32 * stride {
        stride2 = 1 as i32;
        /* This is just a simple (equivalent) way of computing sqrt(len/stride) with rounding.
        It's basically incrementing long as (stride2+0.5)^2 < len/stride. */
        while ((stride2 * stride2 + stride2) * stride + (stride >> 2 as i32)) < len {
            stride2 += 1
        }
    }
    /*NOTE: As a minor optimization, we could be passing around log2(B), not B, for both this and for
    extract_collapse_mask().*/
    len = celt_udiv(
        len as crate::opus_types_h::opus_uint32,
        stride as crate::opus_types_h::opus_uint32,
    ) as i32;
    i = 0 as i32;
    while i < stride {
        if dir < 0 as i32 {
            if stride2 != 0 {
                exp_rotation1(X.offset((i * len) as isize), len, stride2, s, c);
            }
            exp_rotation1(X.offset((i * len) as isize), len, 1 as i32, c, s);
        } else {
            exp_rotation1(X.offset((i * len) as isize), len, 1 as i32, c, -s);
            if stride2 != 0 {
                exp_rotation1(X.offset((i * len) as isize), len, stride2, s, -c);
            }
        }
        i += 1
    }
}
/* * Takes the pitch vector and the decoded residual vector, computes the gain
that will give ||p+g*y||=1 and mixes the residual with the pitch. */

unsafe extern "C" fn normalise_residual(
    mut iy: *mut i32,
    mut X: *mut crate::arch_h::celt_norm,
    mut N: i32,
    mut Ryy: crate::arch_h::opus_val32,
    mut gain: crate::arch_h::opus_val16,
) {
    let mut i: i32 = 0;
    let mut t: crate::arch_h::opus_val32 = 0.;
    let mut g: crate::arch_h::opus_val16 = 0.;
    t = Ryy;
    g = 1.0f32 / crate::stdlib::sqrt(t as f64) as f32 * gain;
    i = 0 as i32;
    loop {
        *X.offset(i as isize) = g * *iy.offset(i as isize) as crate::arch_h::opus_val32;
        i += 1;
        if !(i < N) {
            break;
        }
    }
}

unsafe extern "C" fn extract_collapse_mask(mut iy: *mut i32, mut N: i32, mut B: i32) -> u32 {
    let mut collapse_mask: u32 = 0;
    let mut N0: i32 = 0;
    let mut i: i32 = 0;
    if B <= 1 as i32 {
        return 1 as i32 as u32;
    }
    /*NOTE: As a minor optimization, we could be passing around log2(B), not B, for both this and for
    exp_rotation().*/
    N0 = celt_udiv(
        N as crate::opus_types_h::opus_uint32,
        B as crate::opus_types_h::opus_uint32,
    ) as i32;
    collapse_mask = 0 as i32 as u32;
    i = 0 as i32;
    loop {
        let mut j: i32 = 0;
        let mut tmp: u32 = 0 as i32 as u32;
        j = 0 as i32;
        loop {
            tmp |= *iy.offset((i * N0 + j) as isize) as u32;
            j += 1;
            if !(j < N0) {
                break;
            }
        }
        collapse_mask |= (((tmp != 0 as i32 as u32) as i32) << i) as u32;
        i += 1;
        if !(i < B) {
            break;
        }
    }
    return collapse_mask;
}
#[no_mangle]

pub unsafe extern "C" fn op_pvq_search_c(
    mut X: *mut crate::arch_h::celt_norm,
    mut iy: *mut i32,
    mut K: i32,
    mut N: i32,
    mut _arch: i32,
) -> crate::arch_h::opus_val16 {
    let mut y: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut signx: *mut i32 = 0 as *mut i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pulsesLeft: i32 = 0;
    let mut sum: crate::arch_h::opus_val32 = 0.;
    let mut xy: crate::arch_h::opus_val32 = 0.;
    let mut yy: crate::arch_h::opus_val16 = 0.;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong) as usize,
    );
    y = fresh2.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(N as libc::c_ulong) as usize,
    );
    signx = fresh3.as_mut_ptr() as *mut i32;
    /* Get rid of the sign */
    sum = 0 as i32 as crate::arch_h::opus_val32;
    j = 0 as i32;
    loop {
        *signx.offset(j as isize) = (*X.offset(j as isize) < 0 as i32 as f32) as i32;
        /* OPT: Make sure the compiler doesn't use a branch on ABS16(). */
        *X.offset(j as isize) = crate::stdlib::fabs(*X.offset(j as isize) as f64) as f32;
        *iy.offset(j as isize) = 0 as i32;
        *y.offset(j as isize) = 0 as i32 as crate::arch_h::celt_norm;
        j += 1;
        if !(j < N) {
            break;
        }
    }
    yy = 0 as i32 as crate::arch_h::opus_val16;
    xy = yy;
    pulsesLeft = K;
    /* Do a pre-search by projecting on the pyramid */
    if K > N >> 1 as i32 {
        let mut rcp: crate::arch_h::opus_val16 = 0.;
        j = 0 as i32;
        loop {
            sum += *X.offset(j as isize);
            j += 1;
            if !(j < N) {
                break;
            }
        }
        /* If X is too small, just replace it with a pulse at 0 */
        /* Prevents infinities and NaNs from causing too many pulses
        to be allocated. 64 is an approximation of infinity here. */
        if !(sum > 1e-15f32 && sum < 64 as i32 as f32) {
            *X.offset(0 as i32 as isize) = 1.0f32;
            j = 1 as i32;
            loop {
                *X.offset(j as isize) = 0 as i32 as crate::arch_h::celt_norm;
                j += 1;
                if !(j < N) {
                    break;
                }
            }
            sum = 1.0f32
        }
        /* Using K+e with e < 1 guarantees we cannot get more than K pulses. */
        rcp = (K as f32 + 0.8f32) * (1.0f32 / sum);
        j = 0 as i32;
        loop {
            *iy.offset(j as isize) =
                crate::stdlib::floor((rcp * *X.offset(j as isize)) as f64) as i32;
            *y.offset(j as isize) = *iy.offset(j as isize) as crate::arch_h::celt_norm;
            yy = yy + *y.offset(j as isize) * *y.offset(j as isize);
            xy = xy + *X.offset(j as isize) * *y.offset(j as isize);
            let ref mut fresh4 = *y.offset(j as isize);
            *fresh4 *= 2 as i32 as f32;
            pulsesLeft -= *iy.offset(j as isize);
            j += 1;
            if !(j < N) {
                break;
            }
        }
    }
    /* This should never happen, but just in case it does (e.g. on silence)
    we fill the first bin with pulses. */
    if pulsesLeft > N + 3 as i32 {
        let mut tmp: crate::arch_h::opus_val16 = pulsesLeft as crate::arch_h::opus_val16;
        yy = yy + tmp * tmp;
        yy = yy + tmp * *y.offset(0 as i32 as isize);
        *iy.offset(0 as i32 as isize) += pulsesLeft;
        pulsesLeft = 0 as i32
    }
    i = 0 as i32;
    while i < pulsesLeft {
        let mut Rxy: crate::arch_h::opus_val16 = 0.;
        let mut Ryy: crate::arch_h::opus_val16 = 0.;
        let mut best_id: i32 = 0;
        let mut best_num: crate::arch_h::opus_val32 = 0.;
        let mut best_den: crate::arch_h::opus_val16 = 0.;
        best_id = 0 as i32;
        /* The squared magnitude term gets added anyway, so we might as well
        add it outside the loop */
        yy = yy + 1 as i32 as f32;
        /* Calculations for position 0 are out of the loop, in part to reduce
        mispredicted branches (since the if condition is usually false)
        in the loop. */
        /* Temporary sums of the new pulse(s) */
        Rxy = xy + *X.offset(0 as i32 as isize);
        /* We're multiplying y[j] by two so we don't have to do it here */
        Ryy = yy + *y.offset(0 as i32 as isize);
        /* Approximate score: we maximise Rxy/sqrt(Ryy) (we're guaranteed that
        Rxy is positive because the sign is pre-computed) */
        Rxy = Rxy * Rxy;
        best_den = Ryy;
        best_num = Rxy;
        j = 1 as i32;
        loop {
            /* Temporary sums of the new pulse(s) */
            Rxy = xy + *X.offset(j as isize);
            /* We're multiplying y[j] by two so we don't have to do it here */
            Ryy = yy + *y.offset(j as isize);
            /* Approximate score: we maximise Rxy/sqrt(Ryy) (we're guaranteed that
            Rxy is positive because the sign is pre-computed) */
            Rxy = Rxy * Rxy;
            /* The idea is to check for num/den >= best_num/best_den, but that way
            we can do it without any division */
            /* OPT: It's not clear whether a cmov is faster than a branch here
            since the condition is more often false than true and using
            a cmov introduces data dependencies across iterations. The optimal
            choice may be architecture-dependent. */
            if (best_den * Rxy > Ryy * best_num) as i32 as isize != 0 {
                best_den = Ryy;
                best_num = Rxy;
                best_id = j
            }
            j += 1;
            if !(j < N) {
                break;
            }
        }
        /* Updating the sums of the new pulse(s) */
        xy = xy + *X.offset(best_id as isize);
        /* We're multiplying y[j] by two so we don't have to do it here */
        yy = yy + *y.offset(best_id as isize);
        /* Only now that we've made the final choice, update y/iy */
        /* Multiplying y[j] by 2 so we don't have to do it everywhere else */
        let ref mut fresh5 = *y.offset(best_id as isize);
        *fresh5 += 2 as i32 as f32;
        let ref mut fresh6 = *iy.offset(best_id as isize);
        *fresh6 += 1;
        i += 1
    }
    /* Put the original sign back */
    j = 0 as i32;
    loop {
        /*iy[j] = signx[j] ? -iy[j] : iy[j];*/
        /* OPT: The is more likely to be compiled without a branch than the code above
        but has the same performance otherwise. */
        *iy.offset(j as isize) =
            (*iy.offset(j as isize) ^ -*signx.offset(j as isize)) + *signx.offset(j as isize);
        j += 1;
        if !(j < N) {
            break;
        }
    }
    return yy;
}
#[no_mangle]

pub unsafe extern "C" fn alg_quant(
    mut X: *mut crate::arch_h::celt_norm,
    mut N: i32,
    mut K: i32,
    mut spread: i32,
    mut B: i32,
    mut enc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut gain: crate::arch_h::opus_val16,
    mut resynth: i32,
    mut arch: i32,
) -> u32 {
    let mut iy: *mut i32 = 0 as *mut i32;
    let mut yy: crate::arch_h::opus_val16 = 0.;
    let mut collapse_mask: u32 = 0;
    /* Covers vectorization by up to 4. */
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul((N + 3 as i32) as libc::c_ulong) as usize,
    );
    iy = fresh7.as_mut_ptr() as *mut i32;
    exp_rotation(X, N, 1 as i32, B, K, spread);
    yy = op_pvq_search_c(X, iy, K, N, arch);
    crate::src::opus_1_2_1::celt::cwrs::encode_pulses(
        iy,
        N,
        K,
        enc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    );
    if resynth != 0 {
        normalise_residual(iy, X, N, yy, gain);
        exp_rotation(X, N, -(1 as i32), B, K, spread);
    }
    collapse_mask = extract_collapse_mask(iy, N, B);
    return collapse_mask;
}
/* * Decode pulse vector and combine the result with the pitch vector to produce
the final normalised signal in the current band. */
#[no_mangle]

pub unsafe extern "C" fn alg_unquant(
    mut X: *mut crate::arch_h::celt_norm,
    mut N: i32,
    mut K: i32,
    mut spread: i32,
    mut B: i32,
    mut dec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut gain: crate::arch_h::opus_val16,
) -> u32 {
    let mut Ryy: crate::arch_h::opus_val32 = 0.;
    let mut collapse_mask: u32 = 0;
    let mut iy: *mut i32 = 0 as *mut i32;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(N as libc::c_ulong) as usize,
    );
    iy = fresh8.as_mut_ptr() as *mut i32;
    Ryy = crate::src::opus_1_2_1::celt::cwrs::decode_pulses(
        iy,
        N,
        K,
        dec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    );
    normalise_residual(iy, X, N, Ryy, gain);
    exp_rotation(X, N, -(1 as i32), B, K, spread);
    collapse_mask = extract_collapse_mask(iy, N, B);
    return collapse_mask;
}
#[no_mangle]

pub unsafe extern "C" fn renormalise_vector(
    mut X: *mut crate::arch_h::celt_norm,
    mut N: i32,
    mut gain: crate::arch_h::opus_val16,
    mut _arch: i32,
) {
    let mut i: i32 = 0;
    let mut E: crate::arch_h::opus_val32 = 0.;
    let mut g: crate::arch_h::opus_val16 = 0.;
    let mut t: crate::arch_h::opus_val32 = 0.;
    let mut xptr: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    E = 1e-15f32 + celt_inner_prod_c(X, X, N);
    t = E;
    g = 1.0f32 / crate::stdlib::sqrt(t as f64) as f32 * gain;
    xptr = X;
    i = 0 as i32;
    while i < N {
        *xptr = g * *xptr;
        xptr = xptr.offset(1);
        i += 1
    }
    /*return celt_sqrt(E);*/
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Written by Jean-Marc Valin */
/* *
  @file vq.h
  @brief Vector quantisation of the residual
*/
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
/* * Algebraic pulse-vector quantiser. The signal x is replaced by the sum of
 * the pitch and a combination of pulses such that its norm is still equal
 * to 1. This is the function that will typically require the most CPU.
 * @param X Residual signal to quantise/encode (returns quantised version)
 * @param N Number of samples to encode
 * @param K Number of pulses to use
 * @param enc Entropy encoder state
 * @ret A mask indicating which blocks in the band received pulses
*/
/* * Algebraic pulse decoder
 * @param X Decoded normalised spectrum (returned)
 * @param N Number of samples to decode
 * @param K Number of pulses to use
 * @param dec Entropy decoder state
 * @ret A mask indicating which blocks in the band received pulses
 */
/* OVERRIDE_renormalise_vector */
#[no_mangle]

pub unsafe extern "C" fn stereo_itheta(
    mut X: *const crate::arch_h::celt_norm,
    mut Y: *const crate::arch_h::celt_norm,
    mut stereo: i32,
    mut N: i32,
    mut _arch: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut itheta: i32 = 0;
    let mut mid: crate::arch_h::opus_val16 = 0.;
    let mut side: crate::arch_h::opus_val16 = 0.;
    let mut Emid: crate::arch_h::opus_val32 = 0.;
    let mut Eside: crate::arch_h::opus_val32 = 0.;
    Eside = 1e-15f32;
    Emid = Eside;
    if stereo != 0 {
        i = 0 as i32;
        while i < N {
            let mut m: crate::arch_h::celt_norm = 0.;
            let mut s: crate::arch_h::celt_norm = 0.;
            m = *X.offset(i as isize) + *Y.offset(i as isize);
            s = *X.offset(i as isize) - *Y.offset(i as isize);
            Emid = Emid + m * m;
            Eside = Eside + s * s;
            i += 1
        }
    } else {
        Emid += celt_inner_prod_c(X, X, N);
        Eside += celt_inner_prod_c(Y, Y, N)
    }
    mid = crate::stdlib::sqrt(Emid as f64) as f32;
    side = crate::stdlib::sqrt(Eside as f64) as f32;
    itheta = crate::stdlib::floor(
        (0.5f32 + 16384 as i32 as f32 * 0.63662f32 * fast_atan2f(side, mid)) as f64,
    ) as i32;
    return itheta;
}

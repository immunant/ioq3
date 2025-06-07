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
    #[inline]

    pub unsafe extern "C" fn celt_sudiv(
        mut n: crate::opus_types_h::opus_int32,
        mut d: crate::opus_types_h::opus_int32,
    ) -> crate::opus_types_h::opus_int32 {
        return n / d;
    }
}

pub mod mathops_h {

    /* * Base-2 exponential approximation (2^x). */
    #[inline]

    pub unsafe extern "C" fn celt_exp2(mut x: libc::c_float) -> libc::c_float {
        let mut integer: libc::c_int = 0;
        let mut frac: libc::c_float = 0.;
        let mut res: crate::mathops_h::C2RustUnnamed_61 =
            crate::mathops_h::C2RustUnnamed_61 { f: 0. };
        integer = crate::stdlib::floor(x as libc::c_double) as libc::c_int;
        if integer < -(50 as libc::c_int) {
            return 0 as libc::c_int as libc::c_float;
        }
        frac = x - integer as libc::c_float;
        /* K0 = 1, K1 = log(2), K2 = 3-4*log(2), K3 = 3*log(2) - 2 */
        res.f =
            0.99992522f32 + frac * (0.69583354f32 + frac * (0.22606716f32 + 0.078024523f32 * frac));
        res.i = res
            .i
            .wrapping_add((integer << 23 as libc::c_int) as libc::c_uint)
            & 0x7fffffff as libc::c_int as libc::c_uint;
        return res.f;
    }

    /* MATHOPS_H */
    /* FIXED_POINT */
}

pub mod rate_h {
    #[inline]

    pub unsafe extern "C" fn get_pulses(mut i: libc::c_int) -> libc::c_int {
        return if i < 8 as libc::c_int {
            i
        } else {
            (8 as libc::c_int + (i & 7 as libc::c_int))
                << (i >> 3 as libc::c_int) - 1 as libc::c_int
        };
    }
    #[inline]

    pub unsafe extern "C" fn bits2pulses(
        mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        mut band: libc::c_int,
        mut LM: libc::c_int,
        mut bits: libc::c_int,
    ) -> libc::c_int {
        let mut i: libc::c_int = 0;
        let mut lo: libc::c_int = 0;
        let mut hi: libc::c_int = 0;
        let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
        LM += 1;
        cache = (*m).cache.bits.offset(
            *(*m)
                .cache
                .index
                .offset((LM * (*m).nbEBands + band) as isize) as libc::c_int as isize,
        );
        lo = 0 as libc::c_int;
        hi = *cache.offset(0 as libc::c_int as isize) as libc::c_int;
        bits -= 1;
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            let mut mid: libc::c_int = lo + hi + 1 as libc::c_int >> 1 as libc::c_int;
            /* OPT: Make sure this is implemented with a conditional move */
            if *cache.offset(mid as isize) as libc::c_int >= bits {
                hi = mid
            } else {
                lo = mid
            }
            i += 1
        }
        if bits
            - (if lo == 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                *cache.offset(lo as isize) as libc::c_int
            })
            <= *cache.offset(hi as isize) as libc::c_int - bits
        {
            return lo;
        } else {
            return hi;
        };
    }
    #[inline]

    pub unsafe extern "C" fn pulses2bits(
        mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        mut band: libc::c_int,
        mut LM: libc::c_int,
        mut pulses: libc::c_int,
    ) -> libc::c_int {
        let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
        LM += 1;
        cache = (*m).cache.bits.offset(
            *(*m)
                .cache
                .index
                .offset((LM * (*m).nbEBands + band) as isize) as libc::c_int as isize,
        );
        return if pulses == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (*cache.offset(pulses as isize) as libc::c_int) + 1 as libc::c_int
        };
    }
}

pub mod pitch_h {
    /* OVERRIDE_XCORR_KERNEL */
    #[inline]

    pub unsafe extern "C" fn dual_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y01: *const crate::arch_h::opus_val16,
        mut y02: *const crate::arch_h::opus_val16,
        mut N: libc::c_int,
        mut xy1: *mut crate::arch_h::opus_val32,
        mut xy2: *mut crate::arch_h::opus_val32,
    ) {
        let mut i: libc::c_int = 0;
        let mut xy01: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        let mut xy02: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy01 = xy01 + *x.offset(i as isize) * *y01.offset(i as isize);
            xy02 = xy02 + *x.offset(i as isize) * *y02.offset(i as isize);
            i += 1
        }
        *xy1 = xy01;
        *xy2 = xy02;
    }
    /*We make sure a C version is always available for cases where the overhead of
    vectorization and passing around an arch flag aren't worth it.*/
    #[inline]

    pub unsafe extern "C" fn celt_inner_prod_c(
        mut x: *const crate::arch_h::opus_val16,
        mut y: *const crate::arch_h::opus_val16,
        mut N: libc::c_int,
    ) -> crate::arch_h::opus_val32 {
        let mut i: libc::c_int = 0;
        let mut xy: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
        i = 0 as libc::c_int;
        while i < N {
            xy = xy + *x.offset(i as isize) * *y.offset(i as isize);
            i += 1
        }
        return xy;
    }
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::celt_ener;
pub use crate::arch_h::celt_norm;
pub use crate::arch_h::celt_sig;
pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;

pub use crate::mathops_h::C2RustUnnamed_61;
pub use crate::src::opus_1_2_1::celt::bands::entcode_h::celt_sudiv;
pub use crate::src::opus_1_2_1::celt::bands::entcode_h::celt_udiv;
pub use crate::src::opus_1_2_1::celt::bands::mathops_h::celt_exp2;
pub use crate::src::opus_1_2_1::celt::bands::pitch_h::celt_inner_prod_c;
pub use crate::src::opus_1_2_1::celt::bands::pitch_h::dual_inner_prod_c;
pub use crate::src::opus_1_2_1::celt::bands::rate_h::bits2pulses;
pub use crate::src::opus_1_2_1::celt::bands::rate_h::get_pulses;
pub use crate::src::opus_1_2_1::celt::bands::rate_h::pulses2bits;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_tell_frac;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mathops::isqrt32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct band_ctx {
    pub encode: libc::c_int,
    pub resynth: libc::c_int,
    pub m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    pub i: libc::c_int,
    pub intensity: libc::c_int,
    pub spread: libc::c_int,
    pub tf_change: libc::c_int,
    pub ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    pub remaining_bits: crate::opus_types_h::opus_int32,
    pub bandE: *const crate::arch_h::celt_ener,
    pub seed: crate::opus_types_h::opus_uint32,
    pub arch: libc::c_int,
    pub theta_round: libc::c_int,
    pub disable_inv: libc::c_int,
    pub avoid_split_noise: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct split_ctx {
    pub inv: libc::c_int,
    pub imid: libc::c_int,
    pub iside: libc::c_int,
    pub delta: libc::c_int,
    pub itheta: libc::c_int,
    pub qalloc: libc::c_int,
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Copyright (c) 2008-2009 Gregory Maxwell
Written by Jean-Marc Valin and Gregory Maxwell */
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
#[no_mangle]

pub unsafe extern "C" fn hysteresis_decision(
    mut val: crate::arch_h::opus_val16,
    mut thresholds: *const crate::arch_h::opus_val16,
    mut hysteresis: *const crate::arch_h::opus_val16,
    mut N: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < N {
        if val < *thresholds.offset(i as isize) {
            break;
        }
        i += 1
    }
    if i > prev && val < *thresholds.offset(prev as isize) + *hysteresis.offset(prev as isize) {
        i = prev
    }
    if i < prev
        && val
            > *thresholds.offset((prev - 1 as libc::c_int) as isize)
                - *hysteresis.offset((prev - 1 as libc::c_int) as isize)
    {
        i = prev
    }
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn celt_lcg_rand(
    mut seed: crate::opus_types_h::opus_uint32,
) -> crate::opus_types_h::opus_uint32 {
    return (1664525 as libc::c_int as libc::c_uint)
        .wrapping_mul(seed)
        .wrapping_add(1013904223 as libc::c_int as libc::c_uint);
}
/* This is a cos() approximation designed to be bit-exact on any platform. Bit exactness
with this approximation is important because it has an impact on the bit allocation */
#[no_mangle]

pub unsafe extern "C" fn bitexact_cos(
    mut x: crate::opus_types_h::opus_int16,
) -> crate::opus_types_h::opus_int16 {
    let mut tmp: crate::opus_types_h::opus_int32 = 0;
    let mut x2: crate::opus_types_h::opus_int16 = 0;
    tmp = 4096 as libc::c_int + x as crate::opus_types_h::opus_int32 * x as libc::c_int
        >> 13 as libc::c_int;
    x2 = tmp as crate::opus_types_h::opus_int16;
    x2 = (32767 as libc::c_int - x2 as libc::c_int
        + (16384 as libc::c_int
            + x2 as crate::opus_types_h::opus_int32
                * (-(7651 as libc::c_int)
                    + (16384 as libc::c_int
                        + x2 as crate::opus_types_h::opus_int32
                            * (8277 as libc::c_int
                                + (16384 as libc::c_int
                                    + -(626 as libc::c_int) as crate::opus_types_h::opus_int16
                                        as crate::opus_types_h::opus_int32
                                        * x2 as libc::c_int
                                    >> 15 as libc::c_int))
                                as crate::opus_types_h::opus_int16
                                as libc::c_int
                        >> 15 as libc::c_int)) as crate::opus_types_h::opus_int16
                    as libc::c_int
            >> 15 as libc::c_int)) as crate::opus_types_h::opus_int16;
    return (1 as libc::c_int + x2 as libc::c_int) as crate::opus_types_h::opus_int16;
}
#[no_mangle]

pub unsafe extern "C" fn bitexact_log2tan(
    mut isin: libc::c_int,
    mut icos: libc::c_int,
) -> libc::c_int {
    let mut lc: libc::c_int = 0;
    let mut ls: libc::c_int = 0;
    lc = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - (icos as libc::c_uint).leading_zeros() as i32;
    ls = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * 8 as libc::c_int
        - (isin as libc::c_uint).leading_zeros() as i32;
    icos <<= 15 as libc::c_int - lc;
    isin <<= 15 as libc::c_int - ls;
    return (ls - lc) * ((1 as libc::c_int) << 11 as libc::c_int)
        + (16384 as libc::c_int
            + isin as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                * ((16384 as libc::c_int
                    + isin as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                        * -(2597 as libc::c_int) as crate::opus_types_h::opus_int16 as libc::c_int
                    >> 15 as libc::c_int)
                    + 7932 as libc::c_int) as crate::opus_types_h::opus_int16
                    as libc::c_int
            >> 15 as libc::c_int)
        - (16384 as libc::c_int
            + icos as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                * ((16384 as libc::c_int
                    + icos as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                        * -(2597 as libc::c_int) as crate::opus_types_h::opus_int16 as libc::c_int
                    >> 15 as libc::c_int)
                    + 7932 as libc::c_int) as crate::opus_types_h::opus_int16
                    as libc::c_int
            >> 15 as libc::c_int);
}
/* FIXED_POINT */
/* Compute the amplitude (sqrt energy) in each of the bands */
#[no_mangle]

pub unsafe extern "C" fn compute_band_energies(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_sig,
    mut bandE: *mut crate::arch_h::celt_ener,
    mut end: libc::c_int,
    mut C: libc::c_int,
    mut LM: libc::c_int,
    mut _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    N = (*m).shortMdctSize << LM;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut sum: crate::arch_h::opus_val32 = 0.;
            sum = 1e-27f32
                + celt_inner_prod_c(
                    &*X.offset(
                        (c * N + ((*eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                    ),
                    &*X.offset(
                        (c * N + ((*eBands.offset(i as isize) as libc::c_int) << LM)) as isize,
                    ),
                    (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        - *eBands.offset(i as isize) as libc::c_int)
                        << LM,
                );
            *bandE.offset((i + c * (*m).nbEBands) as isize) =
                crate::stdlib::sqrt(sum as libc::c_double) as libc::c_float;
            i += 1
            /*printf ("%f ", bandE[i+c*m->nbEBands]);*/
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    /*printf ("\n");*/
}
/* Normalise each band such that the energy is one. */
#[no_mangle]

pub unsafe extern "C" fn normalise_bands(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut freq: *const crate::arch_h::celt_sig,
    mut X: *mut crate::arch_h::celt_norm,
    mut bandE: *const crate::arch_h::celt_ener,
    mut end: libc::c_int,
    mut C: libc::c_int,
    mut M: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    N = M * (*m).shortMdctSize;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut j: libc::c_int = 0;
            let mut g: crate::arch_h::opus_val16 =
                1.0f32 / (1e-27f32 + *bandE.offset((i + c * (*m).nbEBands) as isize));
            j = M * *eBands.offset(i as isize) as libc::c_int;
            while j < M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int {
                *X.offset((j + c * N) as isize) = *freq.offset((j + c * N) as isize) * g;
                j += 1
            }
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
/* FIXED_POINT */
/* De-normalise the energy to produce the synthesis from the unit-energy bands */
#[no_mangle]

pub unsafe extern "C" fn denormalise_bands(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut freq: *mut crate::arch_h::celt_sig,
    mut bandLogE: *const crate::arch_h::opus_val16,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut M: libc::c_int,
    mut downsample: libc::c_int,
    mut silence: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut N: libc::c_int = 0;
    let mut bound: libc::c_int = 0;
    let mut f: *mut crate::arch_h::celt_sig = 0 as *mut crate::arch_h::celt_sig;
    let mut x: *const crate::arch_h::celt_norm = 0 as *const crate::arch_h::celt_norm;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    N = M * (*m).shortMdctSize;
    bound = M * *eBands.offset(end as isize) as libc::c_int;
    if downsample != 1 as libc::c_int {
        bound = if bound < N / downsample {
            bound
        } else {
            (N) / downsample
        }
    }
    if silence != 0 {
        bound = 0 as libc::c_int;
        end = 0 as libc::c_int;
        start = end
    }
    f = freq;
    x = X.offset((M * *eBands.offset(start as isize) as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < M * *eBands.offset(start as isize) as libc::c_int {
        let fresh0 = f;
        f = f.offset(1);
        *fresh0 = 0 as libc::c_int as crate::arch_h::celt_sig;
        i += 1
    }
    i = start;
    while i < end {
        let mut j: libc::c_int = 0;
        let mut band_end: libc::c_int = 0;
        let mut g: crate::arch_h::opus_val16 = 0.;
        let mut lg: crate::arch_h::opus_val16 = 0.;
        j = M * *eBands.offset(i as isize) as libc::c_int;
        band_end = M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
        lg = *bandLogE.offset(i as isize)
            + crate::src::opus_1_2_1::celt::quant_bands::eMeans[i as usize];
        g = celt_exp2(if 32.0f32 < lg { 32.0f32 } else { lg });
        loop
        /* Be careful of the fixed-point "else" just above when changing this code */
        {
            let fresh1 = x;
            x = x.offset(1);
            let fresh2 = f;
            f = f.offset(1);
            *fresh2 = *fresh1 * g;
            j += 1;
            if !(j < band_end) {
                break;
            }
        }
        i += 1
    }
    crate::stdlib::memset(
        &mut *freq.offset(bound as isize) as *mut crate::arch_h::celt_sig as *mut libc::c_void,
        0 as libc::c_int,
        ((N - bound) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_sig>() as libc::c_ulong),
    );
}
/* This prevents energy collapse for transients with multiple short MDCTs */
#[no_mangle]

pub unsafe extern "C" fn anti_collapse(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X_: *mut crate::arch_h::celt_norm,
    mut collapse_masks: *mut libc::c_uchar,
    mut LM: libc::c_int,
    mut C: libc::c_int,
    mut size: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut logE: *const crate::arch_h::opus_val16,
    mut prev1logE: *const crate::arch_h::opus_val16,
    mut prev2logE: *const crate::arch_h::opus_val16,
    mut pulses: *const libc::c_int,
    mut seed: crate::opus_types_h::opus_uint32,
    mut arch: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = start;
    while i < end {
        let mut N0: libc::c_int = 0;
        let mut thresh: crate::arch_h::opus_val16 = 0.;
        let mut sqrt_1: crate::arch_h::opus_val16 = 0.;
        let mut depth: libc::c_int = 0;
        N0 = *(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(i as isize) as libc::c_int;
        /* depth in 1/8 bits */
        depth = (celt_udiv(
            (1 as libc::c_int + *pulses.offset(i as isize)) as crate::opus_types_h::opus_uint32,
            (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                - *(*m).eBands.offset(i as isize) as libc::c_int)
                as crate::opus_types_h::opus_uint32,
        ) >> LM) as libc::c_int;
        thresh = 0.5f32 * celt_exp2(-0.125f32 * depth as libc::c_float);
        sqrt_1 = 1.0f32 / crate::stdlib::sqrt((N0 << LM) as libc::c_double) as libc::c_float;
        c = 0 as libc::c_int;
        loop {
            let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
            let mut prev1: crate::arch_h::opus_val16 = 0.;
            let mut prev2: crate::arch_h::opus_val16 = 0.;
            let mut Ediff: crate::arch_h::opus_val32 = 0.;
            let mut r: crate::arch_h::opus_val16 = 0.;
            let mut renormalize: libc::c_int = 0 as libc::c_int;
            prev1 = *prev1logE.offset((c * (*m).nbEBands + i) as isize);
            prev2 = *prev2logE.offset((c * (*m).nbEBands + i) as isize);
            if C == 1 as libc::c_int {
                prev1 = if prev1 > *prev1logE.offset(((*m).nbEBands + i) as isize) {
                    prev1
                } else {
                    *prev1logE.offset(((*m).nbEBands + i) as isize)
                };
                prev2 = if prev2 > *prev2logE.offset(((*m).nbEBands + i) as isize) {
                    prev2
                } else {
                    *prev2logE.offset(((*m).nbEBands + i) as isize)
                }
            }
            Ediff = *logE.offset((c * (*m).nbEBands + i) as isize)
                - (if prev1 < prev2 { prev1 } else { prev2 });
            Ediff = if 0 as libc::c_int as libc::c_float > Ediff {
                0 as libc::c_int as libc::c_float
            } else {
                Ediff
            };
            /* r needs to be multiplied by 2 or 2*sqrt(2) depending on LM because
            short blocks don't have the same energy as long */
            r = 2.0f32 * celt_exp2(-Ediff);
            if LM == 3 as libc::c_int {
                r *= 1.41421356f32
            }
            r = if thresh < r { thresh } else { r };
            r = r * sqrt_1;
            X = X_
                .offset((c * size) as isize)
                .offset(((*(*m).eBands.offset(i as isize) as libc::c_int) << LM) as isize);
            k = 0 as libc::c_int;
            while k < (1 as libc::c_int) << LM {
                /* Detect collapse */
                if *collapse_masks.offset((i * C + c) as isize) as libc::c_int
                    & (1 as libc::c_int) << k
                    == 0
                {
                    /* Fill with noise */
                    j = 0 as libc::c_int;
                    while j < N0 {
                        seed = celt_lcg_rand(seed);
                        *X.offset(((j << LM) + k) as isize) =
                            if seed & 0x8000 as libc::c_int as libc::c_uint != 0 {
                                r
                            } else {
                                -r
                            };
                        j += 1
                    }
                    renormalize = 1 as libc::c_int
                }
                k += 1
            }
            /* We just added some energy, so we need to renormalise */
            if renormalize != 0 {
                crate::src::opus_1_2_1::celt::vq::renormalise_vector(X, N0 << LM, 1.0f32, arch);
            }
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1
    }
}
/* Compute the weights to use for optimizing normalized distortion across
channels. We use the amplitude to weight square distortion, which means
that we use the square root of the value we would have been using if we
wanted to minimize the MSE in the non-normalized domain. This roughly
corresponds to some quick-and-dirty perceptual experiments I ran to
measure inter-aural masking (there doesn't seem to be any published data
on the topic). */

unsafe extern "C" fn compute_channel_weights(
    mut Ex: crate::arch_h::celt_ener,
    mut Ey: crate::arch_h::celt_ener,
    mut w: *mut crate::arch_h::opus_val16,
) {
    let mut minE: crate::arch_h::celt_ener = 0.;
    minE = if Ex < Ey { Ex } else { Ey };
    /* Adjustment to make the weights a bit more conservative. */
    Ex = Ex + minE / 3 as libc::c_int as libc::c_float;
    Ey = Ey + minE / 3 as libc::c_int as libc::c_float;
    *w.offset(0 as libc::c_int as isize) = Ex;
    *w.offset(1 as libc::c_int as isize) = Ey;
}

unsafe extern "C" fn intensity_stereo(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *const crate::arch_h::celt_norm,
    mut bandE: *const crate::arch_h::celt_ener,
    mut bandID: libc::c_int,
    mut N: libc::c_int,
) {
    let mut i: libc::c_int = bandID;
    let mut j: libc::c_int = 0;
    let mut a1: crate::arch_h::opus_val16 = 0.;
    let mut a2: crate::arch_h::opus_val16 = 0.;
    let mut left: crate::arch_h::opus_val16 = 0.;
    let mut right: crate::arch_h::opus_val16 = 0.;
    let mut norm: crate::arch_h::opus_val16 = 0.;
    left = *bandE.offset(i as isize);
    right = *bandE.offset((i + (*m).nbEBands) as isize);
    norm = 1e-15f32
        + crate::stdlib::sqrt((1e-15f32 + left * left + right * right) as libc::c_double)
            as libc::c_float;
    a1 = left / norm;
    a2 = right / norm;
    j = 0 as libc::c_int;
    while j < N {
        let mut r: crate::arch_h::celt_norm = 0.;
        let mut l: crate::arch_h::celt_norm = 0.;
        l = *X.offset(j as isize);
        r = *Y.offset(j as isize);
        *X.offset(j as isize) = a1 * l + a2 * r;
        j += 1
        /* Side is not encoded, no need to calculate */
    }
}

unsafe extern "C" fn stereo_split(
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *mut crate::arch_h::celt_norm,
    mut N: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < N {
        let mut r: crate::arch_h::opus_val32 = 0.;
        let mut l: crate::arch_h::opus_val32 = 0.;
        l = 0.70710678f32 * *X.offset(j as isize);
        r = 0.70710678f32 * *Y.offset(j as isize);
        *X.offset(j as isize) = l + r;
        *Y.offset(j as isize) = r - l;
        j += 1
    }
}

unsafe extern "C" fn stereo_merge(
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *mut crate::arch_h::celt_norm,
    mut mid: crate::arch_h::opus_val16,
    mut N: libc::c_int,
    mut _arch: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut xp: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    let mut side: crate::arch_h::opus_val32 = 0 as libc::c_int as crate::arch_h::opus_val32;
    let mut El: crate::arch_h::opus_val32 = 0.;
    let mut Er: crate::arch_h::opus_val32 = 0.;
    let mut mid2: crate::arch_h::opus_val16 = 0.;
    let mut t: crate::arch_h::opus_val32 = 0.;
    let mut lgain: crate::arch_h::opus_val32 = 0.;
    let mut rgain: crate::arch_h::opus_val32 = 0.;
    /* Compute the norm of X+Y and X-Y as |X|^2 + |Y|^2 +/- sum(xy) */
    dual_inner_prod_c(Y, X, Y, N, &mut xp, &mut side);
    /* Compensating for the mid normalization */
    xp = mid * xp;
    /* mid and side are in Q15, not Q14 like X and Y */
    mid2 = mid;
    El = mid2 * mid2 + side - 2 as libc::c_int as libc::c_float * xp;
    Er = mid2 * mid2 + side + 2 as libc::c_int as libc::c_float * xp;
    if Er < 6e-4f32 || El < 6e-4f32 {
        crate::stdlib::memcpy(
            Y as *mut libc::c_void,
            X as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long * Y.wrapping_offset_from(X) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        return;
    }
    t = El;
    lgain = 1.0f32 / crate::stdlib::sqrt(t as libc::c_double) as libc::c_float;
    t = Er;
    rgain = 1.0f32 / crate::stdlib::sqrt(t as libc::c_double) as libc::c_float;
    j = 0 as libc::c_int;
    while j < N {
        let mut r: crate::arch_h::celt_norm = 0.;
        let mut l: crate::arch_h::celt_norm = 0.;
        /* Apply mid scaling (side is already scaled) */
        l = mid * *X.offset(j as isize);
        r = *Y.offset(j as isize);
        *X.offset(j as isize) = lgain * (l - r);
        *Y.offset(j as isize) = rgain * (l + r);
        j += 1
    }
}
/* Decide whether we should spread the pulses in the current frame */
#[no_mangle]

pub unsafe extern "C" fn spreading_decision(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut X: *const crate::arch_h::celt_norm,
    mut average: *mut libc::c_int,
    mut last_decision: libc::c_int,
    mut hf_average: *mut libc::c_int,
    mut tapset_decision: *mut libc::c_int,
    mut update_hf: libc::c_int,
    mut end: libc::c_int,
    mut C: libc::c_int,
    mut M: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut N0: libc::c_int = 0;
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut nbBands: libc::c_int = 0 as libc::c_int;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    let mut decision: libc::c_int = 0;
    let mut hf_sum: libc::c_int = 0 as libc::c_int;
    N0 = M * (*m).shortMdctSize;
    if M * (*eBands.offset(end as isize) as libc::c_int
        - *eBands.offset((end - 1 as libc::c_int) as isize) as libc::c_int)
        <= 8 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < end {
            let mut j: libc::c_int = 0;
            let mut N: libc::c_int = 0;
            let mut tmp: libc::c_int = 0 as libc::c_int;
            let mut tcount: [libc::c_int; 3] =
                [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
            let mut x: *const crate::arch_h::celt_norm = X
                .offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                .offset((c * N0) as isize);
            N = M
                * (*eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                    - *eBands.offset(i as isize) as libc::c_int);
            if !(N <= 8 as libc::c_int) {
                /* Compute rough CDF of |x[j]| */
                j = 0 as libc::c_int; /* Q13 */
                while j < N {
                    let mut x2N: crate::arch_h::opus_val32 = 0.;
                    x2N = *x.offset(j as isize)
                        * *x.offset(j as isize)
                        * N as crate::arch_h::opus_val32;
                    if x2N < 0.25f32 {
                        tcount[0 as libc::c_int as usize] += 1
                    }
                    if x2N < 0.0625f32 {
                        tcount[1 as libc::c_int as usize] += 1
                    }
                    if x2N < 0.015625f32 {
                        tcount[2 as libc::c_int as usize] += 1
                    }
                    j += 1
                }
                /* Only include four last bands (8 kHz and up) */
                if i > (*m).nbEBands - 4 as libc::c_int {
                    hf_sum = (hf_sum as libc::c_uint).wrapping_add(celt_udiv(
                        (32 as libc::c_int
                            * (tcount[1 as libc::c_int as usize]
                                + tcount[0 as libc::c_int as usize]))
                            as crate::opus_types_h::opus_uint32,
                        N as crate::opus_types_h::opus_uint32,
                    )) as libc::c_int as libc::c_int
                }
                tmp = (2 as libc::c_int * tcount[2 as libc::c_int as usize] >= N) as libc::c_int
                    + (2 as libc::c_int * tcount[1 as libc::c_int as usize] >= N) as libc::c_int
                    + (2 as libc::c_int * tcount[0 as libc::c_int as usize] >= N) as libc::c_int;
                sum += tmp * 256 as libc::c_int;
                nbBands += 1
            }
            i += 1
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    if update_hf != 0 {
        if hf_sum != 0 {
            hf_sum = celt_udiv(
                hf_sum as crate::opus_types_h::opus_uint32,
                (C * (4 as libc::c_int - (*m).nbEBands + end)) as crate::opus_types_h::opus_uint32,
            ) as libc::c_int
        }
        *hf_average = *hf_average + hf_sum >> 1 as libc::c_int;
        hf_sum = *hf_average;
        if *tapset_decision == 2 as libc::c_int {
            hf_sum += 4 as libc::c_int
        } else if *tapset_decision == 0 as libc::c_int {
            hf_sum -= 4 as libc::c_int
        }
        if hf_sum > 22 as libc::c_int {
            *tapset_decision = 2 as libc::c_int
        } else if hf_sum > 18 as libc::c_int {
            *tapset_decision = 1 as libc::c_int
        } else {
            *tapset_decision = 0 as libc::c_int
        }
    }
    /*printf("%d %d %d\n", hf_sum, *hf_average, *tapset_decision);*/
    /* end has to be non-zero */
    sum = celt_udiv(
        sum as crate::opus_types_h::opus_uint32,
        nbBands as crate::opus_types_h::opus_uint32,
    ) as libc::c_int;
    /* Recursive averaging */
    sum = sum + *average >> 1 as libc::c_int;
    *average = sum;
    /* Hysteresis */
    sum = 3 as libc::c_int * sum
        + ((3 as libc::c_int - last_decision << 7 as libc::c_int) + 64 as libc::c_int)
        + 2 as libc::c_int
        >> 2 as libc::c_int;
    if sum < 80 as libc::c_int {
        decision = 3 as libc::c_int
    } else if sum < 256 as libc::c_int {
        decision = 2 as libc::c_int
    } else if sum < 384 as libc::c_int {
        decision = 1 as libc::c_int
    } else {
        decision = 0 as libc::c_int
    }
    return decision;
}
/* Indexing table for converting from natural Hadamard to ordery Hadamard
This is essentially a bit-reversed Gray, on top of which we've added
an inversion of the order because we want the DC at the end rather than
the beginning. The lines are for N=2, 4, 8, 16 */

static mut ordery_table: [libc::c_int; 30] = [
    1 as libc::c_int,
    0 as libc::c_int,
    3 as libc::c_int,
    0 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    7 as libc::c_int,
    0 as libc::c_int,
    4 as libc::c_int,
    3 as libc::c_int,
    6 as libc::c_int,
    1 as libc::c_int,
    5 as libc::c_int,
    2 as libc::c_int,
    15 as libc::c_int,
    0 as libc::c_int,
    8 as libc::c_int,
    7 as libc::c_int,
    12 as libc::c_int,
    3 as libc::c_int,
    11 as libc::c_int,
    4 as libc::c_int,
    14 as libc::c_int,
    1 as libc::c_int,
    9 as libc::c_int,
    6 as libc::c_int,
    13 as libc::c_int,
    2 as libc::c_int,
    10 as libc::c_int,
    5 as libc::c_int,
];

unsafe extern "C" fn deinterleave_hadamard(
    mut X: *mut crate::arch_h::celt_norm,
    mut N0: libc::c_int,
    mut stride: libc::c_int,
    mut hadamard: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut N: libc::c_int = 0;
    N = N0 * stride;
    let mut fresh3 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong) as usize,
    );
    tmp = fresh3.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    if hadamard != 0 {
        let mut ordery: *const libc::c_int = ordery_table
            .as_ptr()
            .offset(stride as isize)
            .offset(-(2 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.offset((*ordery.offset(i as isize) * N0 + j) as isize) =
                    *X.offset((j * stride + i) as isize);
                j += 1
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.offset((i * N0 + j) as isize) = *X.offset((j * stride + i) as isize);
                j += 1
            }
            i += 1
        }
    }
    crate::stdlib::memcpy(
        X as *mut libc::c_void,
        tmp as *const libc::c_void,
        (N as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long * X.wrapping_offset_from(tmp) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
}

unsafe extern "C" fn interleave_hadamard(
    mut X: *mut crate::arch_h::celt_norm,
    mut N0: libc::c_int,
    mut stride: libc::c_int,
    mut hadamard: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut N: libc::c_int = 0;
    N = N0 * stride;
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(N as libc::c_ulong) as usize,
    );
    tmp = fresh4.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    if hadamard != 0 {
        let mut ordery: *const libc::c_int = ordery_table
            .as_ptr()
            .offset(stride as isize)
            .offset(-(2 as libc::c_int as isize));
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.offset((j * stride + i) as isize) =
                    *X.offset((*ordery.offset(i as isize) * N0 + j) as isize);
                j += 1
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < stride {
            j = 0 as libc::c_int;
            while j < N0 {
                *tmp.offset((j * stride + i) as isize) = *X.offset((i * N0 + j) as isize);
                j += 1
            }
            i += 1
        }
    }
    crate::stdlib::memcpy(
        X as *mut libc::c_void,
        tmp as *const libc::c_void,
        (N as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long * X.wrapping_offset_from(tmp) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn haar1(
    mut X: *mut crate::arch_h::celt_norm,
    mut N0: libc::c_int,
    mut stride: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    N0 >>= 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < stride {
        j = 0 as libc::c_int;
        while j < N0 {
            let mut tmp1: crate::arch_h::opus_val32 = 0.;
            let mut tmp2: crate::arch_h::opus_val32 = 0.;
            tmp1 = 0.70710678f32 * *X.offset((stride * 2 as libc::c_int * j + i) as isize);
            tmp2 = 0.70710678f32
                * *X.offset((stride * (2 as libc::c_int * j + 1 as libc::c_int) + i) as isize);
            *X.offset((stride * 2 as libc::c_int * j + i) as isize) = tmp1 + tmp2;
            *X.offset((stride * (2 as libc::c_int * j + 1 as libc::c_int) + i) as isize) =
                tmp1 - tmp2;
            j += 1
        }
        i += 1
    }
}

unsafe extern "C" fn compute_qn(
    mut N: libc::c_int,
    mut b: libc::c_int,
    mut offset: libc::c_int,
    mut pulse_cap: libc::c_int,
    mut stereo: libc::c_int,
) -> libc::c_int {
    static mut exp2_table8: [crate::opus_types_h::opus_int16; 8] = [
        16384 as libc::c_int as crate::opus_types_h::opus_int16,
        17866 as libc::c_int as crate::opus_types_h::opus_int16,
        19483 as libc::c_int as crate::opus_types_h::opus_int16,
        21247 as libc::c_int as crate::opus_types_h::opus_int16,
        23170 as libc::c_int as crate::opus_types_h::opus_int16,
        25267 as libc::c_int as crate::opus_types_h::opus_int16,
        27554 as libc::c_int as crate::opus_types_h::opus_int16,
        30048 as libc::c_int as crate::opus_types_h::opus_int16,
    ];
    let mut qn: libc::c_int = 0;
    let mut qb: libc::c_int = 0;
    let mut N2: libc::c_int = 2 as libc::c_int * N - 1 as libc::c_int;
    if stereo != 0 && N == 2 as libc::c_int {
        N2 -= 1
    }
    /* The upper limit ensures that in a stereo split with itheta==16384, we'll
    always have enough bits left over to code at least one pulse in the
    side; otherwise it would collapse, since it doesn't get folded. */
    qb = celt_sudiv(b + N2 * offset, N2);
    qb = if (b - pulse_cap - ((4 as libc::c_int) << 3 as libc::c_int)) < qb {
        (b - pulse_cap) - ((4 as libc::c_int) << 3 as libc::c_int)
    } else {
        qb
    };
    qb = if ((8 as libc::c_int) << 3 as libc::c_int) < qb {
        (8 as libc::c_int) << 3 as libc::c_int
    } else {
        qb
    };
    if qb < (1 as libc::c_int) << 3 as libc::c_int >> 1 as libc::c_int {
        qn = 1 as libc::c_int
    } else {
        qn = exp2_table8[(qb & 0x7 as libc::c_int) as usize] as libc::c_int
            >> 14 as libc::c_int - (qb >> 3 as libc::c_int);
        qn = (qn + 1 as libc::c_int >> 1 as libc::c_int) << 1 as libc::c_int
    }
    return qn;
}

unsafe extern "C" fn compute_theta(
    mut ctx: *mut band_ctx,
    mut sctx: *mut split_ctx,
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *mut crate::arch_h::celt_norm,
    mut N: libc::c_int,
    mut b: *mut libc::c_int,
    mut B: libc::c_int,
    mut B0: libc::c_int,
    mut LM: libc::c_int,
    mut stereo: libc::c_int,
    mut fill: *mut libc::c_int,
) {
    let mut qn: libc::c_int = 0;
    let mut itheta: libc::c_int = 0 as libc::c_int;
    let mut delta: libc::c_int = 0;
    let mut imid: libc::c_int = 0;
    let mut iside: libc::c_int = 0;
    let mut qalloc: libc::c_int = 0;
    let mut pulse_cap: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut tell: crate::opus_types_h::opus_int32 = 0;
    let mut inv: libc::c_int = 0 as libc::c_int;
    let mut encode: libc::c_int = 0;
    let mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut i: libc::c_int = 0;
    let mut intensity: libc::c_int = 0;
    let mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx =
        0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx;
    let mut bandE: *const crate::arch_h::celt_ener = 0 as *const crate::arch_h::celt_ener;
    encode = (*ctx).encode;
    m = (*ctx).m;
    i = (*ctx).i;
    intensity = (*ctx).intensity;
    ec = (*ctx).ec;
    bandE = (*ctx).bandE;
    /* Decide on the resolution to give to the split parameter theta */
    pulse_cap = *(*m).logN.offset(i as isize) as libc::c_int
        + LM * ((1 as libc::c_int) << 3 as libc::c_int);
    offset = (pulse_cap >> 1 as libc::c_int)
        - (if stereo != 0 && N == 2 as libc::c_int {
            16 as libc::c_int
        } else {
            4 as libc::c_int
        });
    qn = compute_qn(N, *b, offset, pulse_cap, stereo);
    if stereo != 0 && i >= intensity {
        qn = 1 as libc::c_int
    }
    if encode != 0 {
        /* theta is the atan() of the ratio between the (normalized)
        side and mid. With just that parameter, we can re-scale both
        mid and side because we know that 1) they have unit norm and
        2) they are orthogonal. */
        itheta = crate::src::opus_1_2_1::celt::vq::stereo_itheta(X, Y, stereo, N, (*ctx).arch)
    }
    tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
        ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) as crate::opus_types_h::opus_int32;
    if qn != 1 as libc::c_int {
        if encode != 0 {
            if stereo == 0 || (*ctx).theta_round == 0 as libc::c_int {
                itheta = itheta * qn + 8192 as libc::c_int >> 14 as libc::c_int;
                if stereo == 0
                    && (*ctx).avoid_split_noise != 0
                    && itheta > 0 as libc::c_int
                    && itheta < qn
                {
                    /* Check if the selected value of theta will cause the bit allocation
                    to inject noise on one side. If so, make sure the energy of that side
                    is zero. */
                    let mut unquantized: libc::c_int = celt_udiv(
                        (itheta * 16384 as libc::c_int) as crate::opus_types_h::opus_uint32,
                        qn as crate::opus_types_h::opus_uint32,
                    ) as libc::c_int;
                    imid =
                        bitexact_cos(unquantized as crate::opus_types_h::opus_int16) as libc::c_int;
                    iside = bitexact_cos(
                        (16384 as libc::c_int - unquantized) as crate::opus_types_h::opus_int16,
                    ) as libc::c_int;
                    delta = 16384 as libc::c_int
                        + ((N - 1 as libc::c_int) << 7 as libc::c_int)
                            as crate::opus_types_h::opus_int16
                            as crate::opus_types_h::opus_int32
                            * bitexact_log2tan(iside, imid) as crate::opus_types_h::opus_int16
                                as libc::c_int
                        >> 15 as libc::c_int;
                    if delta > *b {
                        itheta = qn
                    } else if delta < -*b {
                        itheta = 0 as libc::c_int
                    }
                }
            } else {
                let mut down: libc::c_int = 0;
                /* Bias quantization towards itheta=0 and itheta=16384. */
                let mut bias: libc::c_int = if itheta > 8192 as libc::c_int {
                    (32767 as libc::c_int) / qn
                } else {
                    (-(32767 as libc::c_int)) / qn
                };
                down = if (qn - 1 as libc::c_int)
                    < (if 0 as libc::c_int > itheta * qn + bias >> 14 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        (itheta * qn + bias) >> 14 as libc::c_int
                    }) {
                    (qn) - 1 as libc::c_int
                } else if 0 as libc::c_int > itheta * qn + bias >> 14 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (itheta * qn + bias) >> 14 as libc::c_int
                };
                if (*ctx).theta_round < 0 as libc::c_int {
                    itheta = down
                } else {
                    itheta = down + 1 as libc::c_int
                }
            }
        }
        /* NOTE: Renormalising X and Y *may* help fixed-point a bit at very high rate.
        Let's do that at higher complexity */
        if stereo != 0 && N > 2 as libc::c_int {
            let mut p0: libc::c_int = 3 as libc::c_int;
            let mut x: libc::c_int = itheta;
            let mut x0: libc::c_int = qn / 2 as libc::c_int;
            let mut ft: libc::c_int = p0 * (x0 + 1 as libc::c_int) + x0;
            /* Entropy coding of the angle. We use a uniform pdf for the
            time split, a step for stereo, and a triangular one for the rest. */
            /* Use a probability of p0 up to itheta=8192 and then use 1 after */
            if encode != 0 {
                crate::src::opus_1_2_1::celt::entenc::ec_encode(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    if x <= x0 {
                        (p0) * x
                    } else {
                        (x - 1 as libc::c_int - x0) + (x0 + 1 as libc::c_int) * p0
                    } as libc::c_uint,
                    if x <= x0 {
                        (p0) * (x + 1 as libc::c_int)
                    } else {
                        (x - x0) + (x0 + 1 as libc::c_int) * p0
                    } as libc::c_uint,
                    ft as libc::c_uint,
                );
            } else {
                let mut fs: libc::c_int = 0;
                fs = crate::src::opus_1_2_1::celt::entdec::ec_decode(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    ft as libc::c_uint,
                ) as libc::c_int;
                if fs < (x0 + 1 as libc::c_int) * p0 {
                    x = fs / p0
                } else {
                    x = x0 + 1 as libc::c_int + (fs - (x0 + 1 as libc::c_int) * p0)
                }
                crate::src::opus_1_2_1::celt::entdec::ec_dec_update(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    if x <= x0 {
                        (p0) * x
                    } else {
                        (x - 1 as libc::c_int - x0) + (x0 + 1 as libc::c_int) * p0
                    } as libc::c_uint,
                    if x <= x0 {
                        (p0) * (x + 1 as libc::c_int)
                    } else {
                        (x - x0) + (x0 + 1 as libc::c_int) * p0
                    } as libc::c_uint,
                    ft as libc::c_uint,
                );
                itheta = x
            }
        } else if B0 > 1 as libc::c_int || stereo != 0 {
            /* Uniform pdf */
            if encode != 0 {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_uint(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    itheta as crate::opus_types_h::opus_uint32,
                    (qn + 1 as libc::c_int) as crate::opus_types_h::opus_uint32,
                );
            } else {
                itheta = crate::src::opus_1_2_1::celt::entdec::ec_dec_uint(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (qn + 1 as libc::c_int) as crate::opus_types_h::opus_uint32,
                ) as libc::c_int
            }
        } else {
            let mut fs_0: libc::c_int = 1 as libc::c_int;
            let mut ft_0: libc::c_int = 0;
            ft_0 = ((qn >> 1 as libc::c_int) + 1 as libc::c_int)
                * ((qn >> 1 as libc::c_int) + 1 as libc::c_int);
            if encode != 0 {
                let mut fl: libc::c_int = 0;
                fs_0 = if itheta <= qn >> 1 as libc::c_int {
                    (itheta) + 1 as libc::c_int
                } else {
                    (qn + 1 as libc::c_int) - itheta
                };
                fl = if itheta <= qn >> 1 as libc::c_int {
                    (itheta * (itheta + 1 as libc::c_int)) >> 1 as libc::c_int
                } else {
                    (ft_0)
                        - ((qn + 1 as libc::c_int - itheta) * (qn + 2 as libc::c_int - itheta)
                            >> 1 as libc::c_int)
                };
                crate::src::opus_1_2_1::celt::entenc::ec_encode(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    fl as libc::c_uint,
                    (fl + fs_0) as libc::c_uint,
                    ft_0 as libc::c_uint,
                );
            } else {
                /* Triangular pdf */
                let mut fl_0: libc::c_int = 0 as libc::c_int;
                let mut fm: libc::c_int = 0;
                fm = crate::src::opus_1_2_1::celt::entdec::ec_decode(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    ft_0 as libc::c_uint,
                ) as libc::c_int;
                if fm
                    < (qn >> 1 as libc::c_int) * ((qn >> 1 as libc::c_int) + 1 as libc::c_int)
                        >> 1 as libc::c_int
                {
                    itheta = (crate::src::opus_1_2_1::celt::mathops::isqrt32(
                        (8 as libc::c_int as libc::c_uint)
                            .wrapping_mul(fm as crate::opus_types_h::opus_uint32)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        >> 1 as libc::c_int) as libc::c_int;
                    fs_0 = itheta + 1 as libc::c_int;
                    fl_0 = itheta * (itheta + 1 as libc::c_int) >> 1 as libc::c_int
                } else {
                    itheta = (((2 as libc::c_int * (qn + 1 as libc::c_int)) as libc::c_uint)
                        .wrapping_sub(crate::src::opus_1_2_1::celt::mathops::isqrt32(
                            (8 as libc::c_int as libc::c_uint)
                                .wrapping_mul(
                                    (ft_0 - fm - 1 as libc::c_int)
                                        as crate::opus_types_h::opus_uint32,
                                )
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ))
                        >> 1 as libc::c_int) as libc::c_int;
                    fs_0 = qn + 1 as libc::c_int - itheta;
                    fl_0 = ft_0
                        - ((qn + 1 as libc::c_int - itheta) * (qn + 2 as libc::c_int - itheta)
                            >> 1 as libc::c_int)
                }
                crate::src::opus_1_2_1::celt::entdec::ec_dec_update(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    fl_0 as libc::c_uint,
                    (fl_0 + fs_0) as libc::c_uint,
                    ft_0 as libc::c_uint,
                );
            }
        }
        itheta = celt_udiv(
            (itheta * 16384 as libc::c_int) as crate::opus_types_h::opus_uint32,
            qn as crate::opus_types_h::opus_uint32,
        ) as libc::c_int;
        if encode != 0 && stereo != 0 {
            if itheta == 0 as libc::c_int {
                intensity_stereo(m, X, Y, bandE, i, N);
            } else {
                stereo_split(X, Y, N);
            }
        }
    } else if stereo != 0 {
        if encode != 0 {
            inv = (itheta > 8192 as libc::c_int && (*ctx).disable_inv == 0) as libc::c_int;
            if inv != 0 {
                let mut j: libc::c_int = 0;
                j = 0 as libc::c_int;
                while j < N {
                    *Y.offset(j as isize) = -*Y.offset(j as isize);
                    j += 1
                }
            }
            intensity_stereo(m, X, Y, bandE, i, N);
        }
        if *b > (2 as libc::c_int) << 3 as libc::c_int
            && (*ctx).remaining_bits > (2 as libc::c_int) << 3 as libc::c_int
        {
            if encode != 0 {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bit_logp(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    inv,
                    2 as libc::c_int as libc::c_uint,
                );
            } else {
                inv = crate::src::opus_1_2_1::celt::entdec::ec_dec_bit_logp(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    2 as libc::c_int as libc::c_uint,
                )
            }
        } else {
            inv = 0 as libc::c_int
        }
        /* inv flag override to avoid problems with downmixing. */
        if (*ctx).disable_inv != 0 {
            inv = 0 as libc::c_int
        }
        itheta = 0 as libc::c_int
    }
    qalloc = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
        ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    )
    .wrapping_sub(tell as libc::c_uint) as libc::c_int;
    *b -= qalloc;
    if itheta == 0 as libc::c_int {
        imid = 32767 as libc::c_int;
        iside = 0 as libc::c_int;
        *fill &= ((1 as libc::c_int) << B) - 1 as libc::c_int;
        delta = -(16384 as libc::c_int)
    } else if itheta == 16384 as libc::c_int {
        imid = 0 as libc::c_int;
        iside = 32767 as libc::c_int;
        *fill &= (((1 as libc::c_int) << B) - 1 as libc::c_int) << B;
        delta = 16384 as libc::c_int
    } else {
        imid = bitexact_cos(itheta as crate::opus_types_h::opus_int16) as libc::c_int;
        iside = bitexact_cos((16384 as libc::c_int - itheta) as crate::opus_types_h::opus_int16)
            as libc::c_int;
        /* This is the mid vs side allocation that minimizes squared error
        in that band. */
        delta = 16384 as libc::c_int
            + ((N - 1 as libc::c_int) << 7 as libc::c_int) as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * bitexact_log2tan(iside, imid) as crate::opus_types_h::opus_int16 as libc::c_int
            >> 15 as libc::c_int
    }
    (*sctx).inv = inv;
    (*sctx).imid = imid;
    (*sctx).iside = iside;
    (*sctx).delta = delta;
    (*sctx).itheta = itheta;
    (*sctx).qalloc = qalloc;
}

unsafe extern "C" fn quant_band_n1(
    mut ctx: *mut band_ctx,
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *mut crate::arch_h::celt_norm,
    mut b: libc::c_int,
    mut lowband_out: *mut crate::arch_h::celt_norm,
) -> libc::c_uint {
    let mut c: libc::c_int = 0;
    let mut stereo: libc::c_int = 0;
    let mut x: *mut crate::arch_h::celt_norm = X;
    let mut encode: libc::c_int = 0;
    let mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx =
        0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx;
    encode = (*ctx).encode;
    ec = (*ctx).ec;
    stereo = (Y != 0 as *mut libc::c_void as *mut crate::arch_h::celt_norm) as libc::c_int;
    c = 0 as libc::c_int;
    loop {
        let mut sign: libc::c_int = 0 as libc::c_int;
        if (*ctx).remaining_bits >= (1 as libc::c_int) << 3 as libc::c_int {
            if encode != 0 {
                sign = (*x.offset(0 as libc::c_int as isize) < 0 as libc::c_int as libc::c_float)
                    as libc::c_int;
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    sign as crate::opus_types_h::opus_uint32,
                    1 as libc::c_int as libc::c_uint,
                );
            } else {
                sign = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    1 as libc::c_int as libc::c_uint,
                ) as libc::c_int
            }
            (*ctx).remaining_bits -= (1 as libc::c_int) << 3 as libc::c_int;
            b -= (1 as libc::c_int) << 3 as libc::c_int
        }
        if (*ctx).resynth != 0 {
            *x.offset(0 as libc::c_int as isize) = if sign != 0 { -1.0f32 } else { 1.0f32 }
        }
        x = Y;
        c += 1;
        if !(c < 1 as libc::c_int + stereo) {
            break;
        }
    }
    if !lowband_out.is_null() {
        *lowband_out.offset(0 as libc::c_int as isize) = *X.offset(0 as libc::c_int as isize)
    }
    return 1 as libc::c_int as libc::c_uint;
}
/* This function is responsible for encoding and decoding a mono partition.
It can split the band in two and transmit the energy difference with
the two half-bands. It can be called recursively so bands can end up being
split in 8 parts. */

unsafe extern "C" fn quant_partition(
    mut ctx: *mut band_ctx,
    mut X: *mut crate::arch_h::celt_norm,
    mut N: libc::c_int,
    mut b: libc::c_int,
    mut B: libc::c_int,
    mut lowband: *mut crate::arch_h::celt_norm,
    mut LM: libc::c_int,
    mut gain: crate::arch_h::opus_val16,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let mut cache: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut q: libc::c_int = 0;
    let mut curr_bits: libc::c_int = 0;
    let mut imid: libc::c_int = 0 as libc::c_int;
    let mut iside: libc::c_int = 0 as libc::c_int;
    let mut B0: libc::c_int = B;
    let mut mid: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut side: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut Y: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut encode: libc::c_int = 0;
    let mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode =
        0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
    let mut i: libc::c_int = 0;
    let mut spread: libc::c_int = 0;
    let mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx =
        0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx;
    encode = (*ctx).encode;
    m = (*ctx).m;
    i = (*ctx).i;
    spread = (*ctx).spread;
    ec = (*ctx).ec;
    /* If we need 1.5 more bit than we can produce, split the band in two. */
    cache = (*m).cache.bits.offset(
        *(*m)
            .cache
            .index
            .offset(((LM + 1 as libc::c_int) * (*m).nbEBands + i) as isize) as libc::c_int
            as isize,
    );
    if LM != -(1 as libc::c_int)
        && b > *cache.offset(*cache.offset(0 as libc::c_int as isize) as isize) as libc::c_int
            + 12 as libc::c_int
        && N > 2 as libc::c_int
    {
        let mut mbits: libc::c_int = 0;
        let mut sbits: libc::c_int = 0;
        let mut delta: libc::c_int = 0;
        let mut itheta: libc::c_int = 0;
        let mut qalloc: libc::c_int = 0;
        let mut sctx: split_ctx = split_ctx {
            inv: 0,
            imid: 0,
            iside: 0,
            delta: 0,
            itheta: 0,
            qalloc: 0,
        };
        let mut next_lowband2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
        let mut rebalance: crate::opus_types_h::opus_int32 = 0;
        N >>= 1 as libc::c_int;
        Y = X.offset(N as isize);
        LM -= 1 as libc::c_int;
        if B == 1 as libc::c_int {
            fill = fill & 1 as libc::c_int | fill << 1 as libc::c_int
        }
        B = B + 1 as libc::c_int >> 1 as libc::c_int;
        compute_theta(
            ctx,
            &mut sctx,
            X,
            Y,
            N,
            &mut b,
            B,
            B0,
            LM,
            0 as libc::c_int,
            &mut fill,
        );
        imid = sctx.imid;
        iside = sctx.iside;
        delta = sctx.delta;
        itheta = sctx.itheta;
        qalloc = sctx.qalloc;
        mid = 1.0f32 / 32768 as libc::c_int as libc::c_float * imid as libc::c_float;
        side = 1.0f32 / 32768 as libc::c_int as libc::c_float * iside as libc::c_float;
        /* Give more bits to low-energy MDCTs than they would otherwise deserve */
        if B0 > 1 as libc::c_int && itheta & 0x3fff as libc::c_int != 0 {
            if itheta > 8192 as libc::c_int {
                /* Rough approximation for pre-echo masking */
                delta -= delta >> 4 as libc::c_int - LM
            } else {
                /* Corresponds to a forward-masking slope of 1.5 dB per 10 ms */
                delta = if (0 as libc::c_int)
                    < delta + (N << 3 as libc::c_int >> 5 as libc::c_int - LM)
                {
                    0 as libc::c_int
                } else {
                    (delta) + (N << 3 as libc::c_int >> 5 as libc::c_int - LM)
                }
            }
        } /* >32-bit split case */
        mbits = if 0 as libc::c_int
            > (if b < (b - delta) / 2 as libc::c_int {
                b
            } else {
                (b - delta) / 2 as libc::c_int
            }) {
            0 as libc::c_int
        } else if b < (b - delta) / 2 as libc::c_int {
            b
        } else {
            (b - delta) / 2 as libc::c_int
        };
        sbits = b - mbits;
        (*ctx).remaining_bits -= qalloc;
        if !lowband.is_null() {
            next_lowband2 = lowband.offset(N as isize)
        }
        rebalance = (*ctx).remaining_bits;
        if mbits >= sbits {
            cm = quant_partition(ctx, X, N, mbits, B, lowband, LM, gain * mid, fill);
            rebalance = mbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << 3 as libc::c_int && itheta != 0 as libc::c_int {
                sbits += rebalance - ((3 as libc::c_int) << 3 as libc::c_int)
            }
            cm |= quant_partition(
                ctx,
                Y,
                N,
                sbits,
                B,
                next_lowband2,
                LM,
                gain * side,
                fill >> B,
            ) << (B0 >> 1 as libc::c_int)
        } else {
            cm = quant_partition(
                ctx,
                Y,
                N,
                sbits,
                B,
                next_lowband2,
                LM,
                gain * side,
                fill >> B,
            ) << (B0 >> 1 as libc::c_int);
            rebalance = sbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << 3 as libc::c_int && itheta != 16384 as libc::c_int
            {
                mbits += rebalance - ((3 as libc::c_int) << 3 as libc::c_int)
            }
            cm |= quant_partition(ctx, X, N, mbits, B, lowband, LM, gain * mid, fill)
        }
    } else {
        /* This is the basic no-split case */
        q = bits2pulses(m, i, LM, b);
        curr_bits = pulses2bits(m, i, LM, q);
        (*ctx).remaining_bits -= curr_bits;
        /* Ensures we can never bust the budget */
        while (*ctx).remaining_bits < 0 as libc::c_int && q > 0 as libc::c_int {
            (*ctx).remaining_bits += curr_bits;
            q -= 1;
            curr_bits = pulses2bits(m, i, LM, q);
            (*ctx).remaining_bits -= curr_bits
        }
        if q != 0 as libc::c_int {
            let mut K: libc::c_int = get_pulses(q);
            /* Finally do the actual quantization */
            if encode != 0 {
                cm = crate::src::opus_1_2_1::celt::vq::alg_quant(
                    X,
                    N,
                    K,
                    spread,
                    B,
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    gain,
                    (*ctx).resynth,
                    (*ctx).arch,
                )
            } else {
                cm = crate::src::opus_1_2_1::celt::vq::alg_unquant(
                    X,
                    N,
                    K,
                    spread,
                    B,
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    gain,
                )
            }
        } else {
            /* If there's no pulse, fill the band anyway */
            let mut j: libc::c_int = 0;
            if (*ctx).resynth != 0 {
                let mut cm_mask: libc::c_uint = 0;
                /* B can be as large as 16, so this shift might overflow an int on a
                16-bit platform; use a long to get defined behavior.*/
                cm_mask = (((1 as libc::c_ulong) << B) as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
                fill = (fill as libc::c_uint & cm_mask) as libc::c_int;
                if fill == 0 {
                    crate::stdlib::memset(
                        X as *mut libc::c_void,
                        0 as libc::c_int,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            ),
                    );
                } else {
                    if lowband.is_null() {
                        /* Noise */
                        j = 0 as libc::c_int;
                        while j < N {
                            (*ctx).seed = celt_lcg_rand((*ctx).seed);
                            *X.offset(j as isize) = ((*ctx).seed as crate::opus_types_h::opus_int32
                                >> 20 as libc::c_int)
                                as crate::arch_h::celt_norm;
                            j += 1
                        }
                        cm = cm_mask
                    } else {
                        /* Folded spectrum */
                        j = 0 as libc::c_int;
                        while j < N {
                            let mut tmp: crate::arch_h::opus_val16 = 0.;
                            (*ctx).seed = celt_lcg_rand((*ctx).seed);
                            /* About 48 dB below the "normal" folding level */
                            tmp = 1.0f32 / 256 as libc::c_int as libc::c_float;
                            tmp = if (*ctx).seed & 0x8000 as libc::c_int as libc::c_uint != 0 {
                                tmp
                            } else {
                                -tmp
                            };
                            *X.offset(j as isize) = *lowband.offset(j as isize) + tmp;
                            j += 1
                        }
                        cm = fill as libc::c_uint
                    }
                    crate::src::opus_1_2_1::celt::vq::renormalise_vector(X, N, gain, (*ctx).arch);
                }
            }
        }
    }
    return cm;
}
/* This function is responsible for encoding and decoding a band for the mono case. */

unsafe extern "C" fn quant_band(
    mut ctx: *mut band_ctx,
    mut X: *mut crate::arch_h::celt_norm,
    mut N: libc::c_int,
    mut b: libc::c_int,
    mut B: libc::c_int,
    mut lowband: *mut crate::arch_h::celt_norm,
    mut LM: libc::c_int,
    mut lowband_out: *mut crate::arch_h::celt_norm,
    mut gain: crate::arch_h::opus_val16,
    mut lowband_scratch: *mut crate::arch_h::celt_norm,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let mut N0: libc::c_int = N;
    let mut N_B: libc::c_int = N;
    let mut N_B0: libc::c_int = 0;
    let mut B0: libc::c_int = B;
    let mut time_divide: libc::c_int = 0 as libc::c_int;
    let mut recombine: libc::c_int = 0 as libc::c_int;
    let mut longBlocks: libc::c_int = 0;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut k: libc::c_int = 0;
    let mut encode: libc::c_int = 0;
    let mut tf_change: libc::c_int = 0;
    encode = (*ctx).encode;
    tf_change = (*ctx).tf_change;
    longBlocks = (B0 == 1 as libc::c_int) as libc::c_int;
    N_B = celt_udiv(
        N_B as crate::opus_types_h::opus_uint32,
        B as crate::opus_types_h::opus_uint32,
    ) as libc::c_int;
    /* Special case for one sample */
    if N == 1 as libc::c_int {
        return quant_band_n1(ctx, X, 0 as *mut crate::arch_h::celt_norm, b, lowband_out);
    }
    if tf_change > 0 as libc::c_int {
        recombine = tf_change
    }
    /* Band recombining to increase frequency resolution */
    if !lowband_scratch.is_null()
        && !lowband.is_null()
        && (recombine != 0
            || N_B & 1 as libc::c_int == 0 as libc::c_int && tf_change < 0 as libc::c_int
            || B0 > 1 as libc::c_int)
    {
        crate::stdlib::memcpy(
            lowband_scratch as *mut libc::c_void,
            lowband as *const libc::c_void,
            (N as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * lowband_scratch.wrapping_offset_from(lowband) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        lowband = lowband_scratch
    }
    k = 0 as libc::c_int;
    while k < recombine {
        static mut bit_interleave_table: [libc::c_uchar; 16] = [
            0 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            1 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            2 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
            3 as libc::c_int as libc::c_uchar,
        ];
        if encode != 0 {
            haar1(X, N >> k, (1 as libc::c_int) << k);
        }
        if !lowband.is_null() {
            haar1(lowband, N >> k, (1 as libc::c_int) << k);
        }
        fill = bit_interleave_table[(fill & 0xf as libc::c_int) as usize] as libc::c_int
            | (bit_interleave_table[(fill >> 4 as libc::c_int) as usize] as libc::c_int)
                << 2 as libc::c_int;
        k += 1
    }
    B >>= recombine;
    N_B <<= recombine;
    /* Increasing the time resolution */
    while N_B & 1 as libc::c_int == 0 as libc::c_int && tf_change < 0 as libc::c_int {
        if encode != 0 {
            haar1(X, N_B, B);
        }
        if !lowband.is_null() {
            haar1(lowband, N_B, B);
        }
        fill |= fill << B;
        B <<= 1 as libc::c_int;
        N_B >>= 1 as libc::c_int;
        time_divide += 1;
        tf_change += 1
    }
    B0 = B;
    N_B0 = N_B;
    /* Reorganize the samples in time order instead of frequency order */
    if B0 > 1 as libc::c_int {
        if encode != 0 {
            deinterleave_hadamard(X, N_B >> recombine, B0 << recombine, longBlocks);
        }
        if !lowband.is_null() {
            deinterleave_hadamard(lowband, N_B >> recombine, B0 << recombine, longBlocks);
        }
    }
    cm = quant_partition(ctx, X, N, b, B, lowband, LM, gain, fill);
    /* This code is used by the decoder and by the resynthesis-enabled encoder */
    if (*ctx).resynth != 0 {
        /* Undo the sample reorganization going from time order to frequency order */
        if B0 > 1 as libc::c_int {
            interleave_hadamard(X, N_B >> recombine, B0 << recombine, longBlocks);
        }
        /* Undo time-freq changes that we did earlier */
        N_B = N_B0;
        B = B0;
        k = 0 as libc::c_int;
        while k < time_divide {
            B >>= 1 as libc::c_int;
            N_B <<= 1 as libc::c_int;
            cm |= cm >> B;
            haar1(X, N_B, B);
            k += 1
        }
        k = 0 as libc::c_int;
        while k < recombine {
            static mut bit_deinterleave_table: [libc::c_uchar; 16] = [
                0 as libc::c_int as libc::c_uchar,
                0x3 as libc::c_int as libc::c_uchar,
                0xc as libc::c_int as libc::c_uchar,
                0xf as libc::c_int as libc::c_uchar,
                0x30 as libc::c_int as libc::c_uchar,
                0x33 as libc::c_int as libc::c_uchar,
                0x3c as libc::c_int as libc::c_uchar,
                0x3f as libc::c_int as libc::c_uchar,
                0xc0 as libc::c_int as libc::c_uchar,
                0xc3 as libc::c_int as libc::c_uchar,
                0xcc as libc::c_int as libc::c_uchar,
                0xcf as libc::c_int as libc::c_uchar,
                0xf0 as libc::c_int as libc::c_uchar,
                0xf3 as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ];
            cm = bit_deinterleave_table[cm as usize] as libc::c_uint;
            haar1(X, N0 >> k, (1 as libc::c_int) << k);
            k += 1
        }
        B <<= recombine;
        /* Scale output for later folding */
        if !lowband_out.is_null() {
            let mut j: libc::c_int = 0;
            let mut n: crate::arch_h::opus_val16 = 0.;
            n = crate::stdlib::sqrt(N0 as libc::c_double) as libc::c_float;
            j = 0 as libc::c_int;
            while j < N0 {
                *lowband_out.offset(j as isize) = n * *X.offset(j as isize);
                j += 1
            }
        }
        cm &= (((1 as libc::c_int) << B) - 1 as libc::c_int) as libc::c_uint
    }
    return cm;
}
/* This function is responsible for encoding and decoding a band for the stereo case. */

unsafe extern "C" fn quant_band_stereo(
    mut ctx: *mut band_ctx,
    mut X: *mut crate::arch_h::celt_norm,
    mut Y: *mut crate::arch_h::celt_norm,
    mut N: libc::c_int,
    mut b: libc::c_int,
    mut B: libc::c_int,
    mut lowband: *mut crate::arch_h::celt_norm,
    mut LM: libc::c_int,
    mut lowband_out: *mut crate::arch_h::celt_norm,
    mut lowband_scratch: *mut crate::arch_h::celt_norm,
    mut fill: libc::c_int,
) -> libc::c_uint {
    let mut imid: libc::c_int = 0 as libc::c_int;
    let mut iside: libc::c_int = 0 as libc::c_int;
    let mut inv: libc::c_int = 0 as libc::c_int;
    let mut mid: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut side: crate::arch_h::opus_val16 = 0 as libc::c_int as crate::arch_h::opus_val16;
    let mut cm: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mbits: libc::c_int = 0;
    let mut sbits: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut itheta: libc::c_int = 0;
    let mut qalloc: libc::c_int = 0;
    let mut sctx: split_ctx = split_ctx {
        inv: 0,
        imid: 0,
        iside: 0,
        delta: 0,
        itheta: 0,
        qalloc: 0,
    };
    let mut orig_fill: libc::c_int = 0;
    let mut encode: libc::c_int = 0;
    let mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx =
        0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx;
    encode = (*ctx).encode;
    ec = (*ctx).ec;
    /* Special case for one sample */
    if N == 1 as libc::c_int {
        return quant_band_n1(ctx, X, Y, b, lowband_out);
    }
    orig_fill = fill;
    compute_theta(
        ctx,
        &mut sctx,
        X,
        Y,
        N,
        &mut b,
        B,
        B,
        LM,
        1 as libc::c_int,
        &mut fill,
    );
    inv = sctx.inv;
    imid = sctx.imid;
    iside = sctx.iside;
    delta = sctx.delta;
    itheta = sctx.itheta;
    qalloc = sctx.qalloc;
    mid = 1.0f32 / 32768 as libc::c_int as libc::c_float * imid as libc::c_float;
    side = 1.0f32 / 32768 as libc::c_int as libc::c_float * iside as libc::c_float;
    /* This is a special case for N=2 that only works for stereo and takes
    advantage of the fact that mid and side are orthogonal to encode
    the side with just one bit. */
    if N == 2 as libc::c_int {
        let mut c: libc::c_int = 0;
        let mut sign: libc::c_int = 0 as libc::c_int;
        let mut x2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
        let mut y2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
        mbits = b;
        sbits = 0 as libc::c_int;
        /* Only need one bit for the side. */
        if itheta != 0 as libc::c_int && itheta != 16384 as libc::c_int {
            sbits = (1 as libc::c_int) << 3 as libc::c_int
        }
        mbits -= sbits;
        c = (itheta > 8192 as libc::c_int) as libc::c_int;
        (*ctx).remaining_bits -= qalloc + sbits;
        x2 = if c != 0 { Y } else { X };
        y2 = if c != 0 { X } else { Y };
        if sbits != 0 {
            if encode != 0 {
                /* Here we only need to encode a sign for the side. */
                sign = (*x2.offset(0 as libc::c_int as isize)
                    * *y2.offset(1 as libc::c_int as isize)
                    - *x2.offset(1 as libc::c_int as isize) * *y2.offset(0 as libc::c_int as isize)
                    < 0 as libc::c_int as libc::c_float) as libc::c_int;
                crate::src::opus_1_2_1::celt::entenc::ec_enc_bits(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    sign as crate::opus_types_h::opus_uint32,
                    1 as libc::c_int as libc::c_uint,
                );
            } else {
                sign = crate::src::opus_1_2_1::celt::entdec::ec_dec_bits(
                    ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    1 as libc::c_int as libc::c_uint,
                ) as libc::c_int
            }
        }
        sign = 1 as libc::c_int - 2 as libc::c_int * sign;
        /* We use orig_fill here because we want to fold the side, but if
        itheta==16384, we'll have cleared the low bits of fill. */
        cm = quant_band(
            ctx,
            x2,
            N,
            mbits,
            B,
            lowband,
            LM,
            lowband_out,
            1.0f32,
            lowband_scratch,
            orig_fill,
        );
        /* We don't split N=2 bands, so cm is either 1 or 0 (for a fold-collapse),
        and there's no need to worry about mixing with the other channel. */
        *y2.offset(0 as libc::c_int as isize) =
            -sign as libc::c_float * *x2.offset(1 as libc::c_int as isize);
        *y2.offset(1 as libc::c_int as isize) =
            sign as libc::c_float * *x2.offset(0 as libc::c_int as isize);
        if (*ctx).resynth != 0 {
            let mut tmp: crate::arch_h::celt_norm = 0.;
            *X.offset(0 as libc::c_int as isize) = mid * *X.offset(0 as libc::c_int as isize);
            *X.offset(1 as libc::c_int as isize) = mid * *X.offset(1 as libc::c_int as isize);
            *Y.offset(0 as libc::c_int as isize) = side * *Y.offset(0 as libc::c_int as isize);
            *Y.offset(1 as libc::c_int as isize) = side * *Y.offset(1 as libc::c_int as isize);
            tmp = *X.offset(0 as libc::c_int as isize);
            *X.offset(0 as libc::c_int as isize) = tmp - *Y.offset(0 as libc::c_int as isize);
            *Y.offset(0 as libc::c_int as isize) = tmp + *Y.offset(0 as libc::c_int as isize);
            tmp = *X.offset(1 as libc::c_int as isize);
            *X.offset(1 as libc::c_int as isize) = tmp - *Y.offset(1 as libc::c_int as isize);
            *Y.offset(1 as libc::c_int as isize) = tmp + *Y.offset(1 as libc::c_int as isize)
        }
    } else {
        /* "Normal" split code */
        let mut rebalance: crate::opus_types_h::opus_int32 = 0;
        mbits = if 0 as libc::c_int
            > (if b < (b - delta) / 2 as libc::c_int {
                b
            } else {
                (b - delta) / 2 as libc::c_int
            }) {
            0 as libc::c_int
        } else if b < (b - delta) / 2 as libc::c_int {
            b
        } else {
            (b - delta) / 2 as libc::c_int
        };
        sbits = b - mbits;
        (*ctx).remaining_bits -= qalloc;
        rebalance = (*ctx).remaining_bits;
        if mbits >= sbits {
            /* In stereo mode, we do not apply a scaling to the mid because we need the normalized
            mid for folding later. */
            cm = quant_band(
                ctx,
                X,
                N,
                mbits,
                B,
                lowband,
                LM,
                lowband_out,
                1.0f32,
                lowband_scratch,
                fill,
            );
            rebalance = mbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << 3 as libc::c_int && itheta != 0 as libc::c_int {
                sbits += rebalance - ((3 as libc::c_int) << 3 as libc::c_int)
            }
            /* For a stereo split, the high bits of fill are always zero, so no
            folding will be done to the side. */
            cm |= quant_band(
                ctx,
                Y,
                N,
                sbits,
                B,
                0 as *mut crate::arch_h::celt_norm,
                LM,
                0 as *mut crate::arch_h::celt_norm,
                side,
                0 as *mut crate::arch_h::celt_norm,
                fill >> B,
            )
        } else {
            /* For a stereo split, the high bits of fill are always zero, so no
            folding will be done to the side. */
            cm = quant_band(
                ctx,
                Y,
                N,
                sbits,
                B,
                0 as *mut crate::arch_h::celt_norm,
                LM,
                0 as *mut crate::arch_h::celt_norm,
                side,
                0 as *mut crate::arch_h::celt_norm,
                fill >> B,
            );
            rebalance = sbits - (rebalance - (*ctx).remaining_bits);
            if rebalance > (3 as libc::c_int) << 3 as libc::c_int && itheta != 16384 as libc::c_int
            {
                mbits += rebalance - ((3 as libc::c_int) << 3 as libc::c_int)
            }
            /* In stereo mode, we do not apply a scaling to the mid because we need the normalized
            mid for folding later. */
            cm |= quant_band(
                ctx,
                X,
                N,
                mbits,
                B,
                lowband,
                LM,
                lowband_out,
                1.0f32,
                lowband_scratch,
                fill,
            )
        }
    }
    /* This code is used by the decoder and by the resynthesis-enabled encoder */
    if (*ctx).resynth != 0 {
        if N != 2 as libc::c_int {
            stereo_merge(X, Y, mid, N, (*ctx).arch);
        }
        if inv != 0 {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < N {
                *Y.offset(j as isize) = -*Y.offset(j as isize);
                j += 1
            }
        }
    }
    return cm;
}

unsafe extern "C" fn special_hybrid_folding(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut norm: *mut crate::arch_h::celt_norm,
    mut norm2: *mut crate::arch_h::celt_norm,
    mut start: libc::c_int,
    mut M: libc::c_int,
    mut dual_stereo: libc::c_int,
) {
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    n1 = M
        * (*eBands.offset((start + 1 as libc::c_int) as isize) as libc::c_int
            - *eBands.offset(start as isize) as libc::c_int);
    n2 = M
        * (*eBands.offset((start + 2 as libc::c_int) as isize) as libc::c_int
            - *eBands.offset((start + 1 as libc::c_int) as isize) as libc::c_int);
    /* Duplicate enough of the first band folding data to be able to fold the second band.
    Copies no data for CELT-only mode. */
    crate::stdlib::memcpy(
        &mut *norm.offset(n1 as isize) as *mut crate::arch_h::celt_norm as *mut libc::c_void,
        &mut *norm.offset((2 as libc::c_int * n1 - n2) as isize) as *mut crate::arch_h::celt_norm
            as *const libc::c_void,
        ((n2 - n1) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * (&mut *norm.offset(n1 as isize) as *mut crate::arch_h::celt_norm)
                        .wrapping_offset_from(
                            &mut *norm.offset((2 as libc::c_int * n1 - n2) as isize),
                        ) as libc::c_long) as libc::c_ulong,
            ),
    );
    if dual_stereo != 0 {
        crate::stdlib::memcpy(
            &mut *norm2.offset(n1 as isize) as *mut crate::arch_h::celt_norm as *mut libc::c_void,
            &mut *norm2.offset((2 as libc::c_int * n1 - n2) as isize)
                as *mut crate::arch_h::celt_norm as *const libc::c_void,
            ((n2 - n1) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * (&mut *norm2.offset(n1 as isize) as *mut crate::arch_h::celt_norm)
                            .wrapping_offset_from(
                                &mut *norm2.offset((2 as libc::c_int * n1 - n2) as isize),
                            ) as libc::c_long) as libc::c_ulong,
                ),
        );
    };
}
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2009 Xiph.Org Foundation
Copyright (c) 2008-2009 Gregory Maxwell
Written by Jean-Marc Valin and Gregory Maxwell */
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
/* * Compute the amplitude (sqrt energy) in each of the bands
 * @param m Mode data
 * @param X Spectrum
 * @param bandE Square root of the energy for each band (returned)
 */
/*void compute_noise_energies(const CELTMode *m, const celt_sig *X, const opus_val16 *tonality, celt_ener *bandE);*/
/* * Normalise each band of X such that the energy in each band is
   equal to 1
* @param m Mode data
* @param X Spectrum (returned normalised)
* @param bandE Square root of the energy for each band
*/
/* * Denormalise each band of X to restore full amplitude
 * @param m Mode data
 * @param X Spectrum (returned de-normalised)
 * @param bandE Square root of the energy for each band
 */
/* * Quantisation/encoding of the residual spectrum
 * @param encode flag that indicates whether we're encoding (1) or decoding (0)
 * @param m Mode data
 * @param start First band to process
 * @param end Last band to process + 1
 * @param X Residual (normalised)
 * @param Y Residual (normalised) for second channel (or NULL for mono)
 * @param collapse_masks Anti-collapse tracking mask
 * @param bandE Square root of the energy for each band
 * @param pulses Bit allocation (per band) for PVQ
 * @param shortBlocks Zero for long blocks, non-zero for short blocks
 * @param spread Amount of spreading to use
 * @param dual_stereo Zero for MS stereo, non-zero for dual stereo
 * @param intensity First band to use intensity stereo
 * @param tf_res Time-frequency resolution change
 * @param total_bits Total number of bits that can be used for the frame (including the ones already spent)
 * @param balance Number of unallocated bits
 * @param en Entropy coder state
 * @param LM log2() of the number of 2.5 subframes in the frame
 * @param codedBands Last band to receive bits + 1
 * @param seed Random generator seed
 * @param arch Run-time architecture (see opus_select_arch())
 */
#[no_mangle]

pub unsafe extern "C" fn quant_all_bands(
    mut encode: libc::c_int,
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut X_: *mut crate::arch_h::celt_norm,
    mut Y_: *mut crate::arch_h::celt_norm,
    mut collapse_masks: *mut libc::c_uchar,
    mut bandE: *const crate::arch_h::celt_ener,
    mut pulses: *mut libc::c_int,
    mut shortBlocks: libc::c_int,
    mut spread: libc::c_int,
    mut dual_stereo: libc::c_int,
    mut intensity: libc::c_int,
    mut tf_res: *mut libc::c_int,
    mut total_bits: crate::opus_types_h::opus_int32,
    mut balance: crate::opus_types_h::opus_int32,
    mut ec: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    mut LM: libc::c_int,
    mut codedBands: libc::c_int,
    mut seed: *mut crate::opus_types_h::opus_uint32,
    mut complexity: libc::c_int,
    mut arch: libc::c_int,
    mut disable_inv: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut remaining_bits: crate::opus_types_h::opus_int32 = 0;
    let mut eBands: *const crate::opus_types_h::opus_int16 = (*m).eBands;
    let mut norm: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut norm2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut _norm: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut _lowband_scratch: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut X_save: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut Y_save: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut X_save2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut Y_save2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut norm_save2: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut resynth_alloc: libc::c_int = 0;
    let mut lowband_scratch: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
    let mut B: libc::c_int = 0;
    let mut M: libc::c_int = 0;
    let mut lowband_offset: libc::c_int = 0;
    let mut update_lowband: libc::c_int = 1 as libc::c_int;
    let mut C: libc::c_int = if !Y_.is_null() {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut norm_offset: libc::c_int = 0;
    let mut theta_rdo: libc::c_int =
        (encode != 0 && !Y_.is_null() && dual_stereo == 0 && complexity >= 8 as libc::c_int)
            as libc::c_int;
    let mut resynth: libc::c_int = (encode == 0 || theta_rdo != 0) as libc::c_int;
    let mut ctx: band_ctx = band_ctx {
        encode: 0,
        resynth: 0,
        m: 0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
        i: 0,
        intensity: 0,
        spread: 0,
        tf_change: 0,
        ec: 0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        remaining_bits: 0,
        bandE: 0 as *const crate::arch_h::celt_ener,
        seed: 0,
        arch: 0,
        theta_round: 0,
        disable_inv: 0,
        avoid_split_noise: 0,
    };
    M = (1 as libc::c_int) << LM;
    B = if shortBlocks != 0 {
        M
    } else {
        1 as libc::c_int
    };
    norm_offset = M * *eBands.offset(start as isize) as libc::c_int;
    /* No need to allocate norm for the last band because we don't need an
    output in that band. */
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong).wrapping_mul(
            (C * (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int
                - norm_offset)) as libc::c_ulong,
        ) as usize,
    );
    _norm = fresh5.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    norm = _norm;
    norm2 = norm
        .offset(
            (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int)
                as isize,
        )
        .offset(-(norm_offset as isize));
    /* For decoding, we can use the last band as scratch space because we don't need that
    scratch space for the last band and we don't care about the data there until we're
    decoding the last band. */
    if encode != 0 && resynth != 0 {
        resynth_alloc = M
            * (*eBands.offset((*m).nbEBands as isize) as libc::c_int
                - *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int)
    } else {
        resynth_alloc = 0 as libc::c_int
    }
    let mut fresh6 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    _lowband_scratch = fresh6.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    if encode != 0 && resynth != 0 {
        lowband_scratch = _lowband_scratch
    } else {
        lowband_scratch = X_.offset(
            (M * *eBands.offset(((*m).nbEBands - 1 as libc::c_int) as isize) as libc::c_int)
                as isize,
        )
    }
    let mut fresh7 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    X_save = fresh7.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    Y_save = fresh8.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    X_save2 = fresh9.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    Y_save2 = fresh10.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong)
            .wrapping_mul(resynth_alloc as libc::c_ulong) as usize,
    );
    norm_save2 = fresh11.as_mut_ptr() as *mut crate::arch_h::celt_norm;
    lowband_offset = 0 as libc::c_int;
    ctx.bandE = bandE;
    ctx.ec = ec;
    ctx.encode = encode;
    ctx.intensity = intensity;
    ctx.m = m;
    ctx.seed = *seed;
    ctx.spread = spread;
    ctx.arch = arch;
    ctx.disable_inv = disable_inv;
    ctx.resynth = resynth;
    ctx.theta_round = 0 as libc::c_int;
    /* Avoid injecting noise in the first band on transients. */
    ctx.avoid_split_noise = (B > 1 as libc::c_int) as libc::c_int;
    i = start;
    while i < end {
        let mut tell: crate::opus_types_h::opus_int32 = 0;
        let mut b: libc::c_int = 0;
        let mut N: libc::c_int = 0;
        let mut curr_balance: crate::opus_types_h::opus_int32 = 0;
        let mut effective_lowband: libc::c_int = -(1 as libc::c_int);
        let mut X: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
        let mut Y: *mut crate::arch_h::celt_norm = 0 as *mut crate::arch_h::celt_norm;
        let mut tf_change: libc::c_int = 0 as libc::c_int;
        let mut x_cm: libc::c_uint = 0;
        let mut y_cm: libc::c_uint = 0;
        let mut last: libc::c_int = 0;
        ctx.i = i;
        last = (i == end - 1 as libc::c_int) as libc::c_int;
        X = X_.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize);
        if !Y_.is_null() {
            Y = Y_.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
        } else {
            Y = 0 as *mut crate::arch_h::celt_norm
        }
        N = M * *eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - M * *eBands.offset(i as isize) as libc::c_int;
        tell = crate::src::opus_1_2_1::celt::entcode::ec_tell_frac(
            ec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        ) as crate::opus_types_h::opus_int32;
        /* Compute how many bits we want to allocate to this band */
        if i != start {
            balance -= tell
        }
        remaining_bits = total_bits - tell - 1 as libc::c_int;
        ctx.remaining_bits = remaining_bits;
        if i <= codedBands - 1 as libc::c_int {
            curr_balance = celt_sudiv(
                balance,
                if (3 as libc::c_int) < codedBands - i {
                    3 as libc::c_int
                } else {
                    (codedBands) - i
                },
            );
            b = if 0 as libc::c_int
                > (if (16383 as libc::c_int)
                    < (if (remaining_bits + 1 as libc::c_int)
                        < *pulses.offset(i as isize) + curr_balance
                    {
                        (remaining_bits) + 1 as libc::c_int
                    } else {
                        (*pulses.offset(i as isize)) + curr_balance
                    })
                {
                    16383 as libc::c_int
                } else {
                    (if (remaining_bits + 1 as libc::c_int)
                        < *pulses.offset(i as isize) + curr_balance
                    {
                        (remaining_bits) + 1 as libc::c_int
                    } else {
                        (*pulses.offset(i as isize)) + curr_balance
                    })
                }) {
                0 as libc::c_int
            } else if (16383 as libc::c_int)
                < (if (remaining_bits + 1 as libc::c_int)
                    < *pulses.offset(i as isize) + curr_balance
                {
                    (remaining_bits) + 1 as libc::c_int
                } else {
                    (*pulses.offset(i as isize)) + curr_balance
                })
            {
                16383 as libc::c_int
            } else if (remaining_bits + 1 as libc::c_int)
                < *pulses.offset(i as isize) + curr_balance
            {
                (remaining_bits) + 1 as libc::c_int
            } else {
                (*pulses.offset(i as isize)) + curr_balance
            }
        } else {
            b = 0 as libc::c_int
        }
        if resynth != 0
            && M * *eBands.offset(i as isize) as libc::c_int - N
                >= M * *eBands.offset(start as isize) as libc::c_int
            && (update_lowband != 0 || lowband_offset == 0 as libc::c_int)
        {
            lowband_offset = i
        }
        tf_change = *tf_res.offset(i as isize);
        ctx.tf_change = tf_change;
        if i >= (*m).effEBands {
            X = norm;
            if !Y_.is_null() {
                Y = norm
            }
            lowband_scratch = 0 as *mut crate::arch_h::celt_norm
        }
        if last != 0 && theta_rdo == 0 {
            lowband_scratch = 0 as *mut crate::arch_h::celt_norm
        }
        /* Get a conservative estimate of the collapse_mask's for the bands we're
        going to be folding from. */
        if lowband_offset != 0 as libc::c_int
            && (spread != 3 as libc::c_int || B > 1 as libc::c_int || tf_change < 0 as libc::c_int)
        {
            let mut fold_start: libc::c_int = 0;
            let mut fold_end: libc::c_int = 0;
            let mut fold_i: libc::c_int = 0;
            /* This ensures we never repeat spectral content within one band */
            effective_lowband = if 0 as libc::c_int
                > M * *eBands.offset(lowband_offset as isize) as libc::c_int - norm_offset - N
            {
                0 as libc::c_int
            } else {
                (M * *eBands.offset(lowband_offset as isize) as libc::c_int - norm_offset) - N
            };
            fold_start = lowband_offset;
            loop {
                fold_start -= 1;
                if !(M * *eBands.offset(fold_start as isize) as libc::c_int
                    > effective_lowband + norm_offset)
                {
                    break;
                }
            }
            fold_end = lowband_offset - 1 as libc::c_int;
            loop {
                fold_end += 1;
                if !((M * *eBands.offset(fold_end as isize) as libc::c_int)
                    < effective_lowband + norm_offset + N)
                {
                    break;
                }
            }
            y_cm = 0 as libc::c_int as libc::c_uint;
            x_cm = y_cm;
            fold_i = fold_start;
            loop {
                x_cm |= *collapse_masks.offset((fold_i * C + 0 as libc::c_int) as isize)
                    as libc::c_uint;
                y_cm |= *collapse_masks.offset((fold_i * C + C - 1 as libc::c_int) as isize)
                    as libc::c_uint;
                fold_i += 1;
                if !(fold_i < fold_end) {
                    break;
                }
            }
        } else {
            /* Otherwise, we'll be using the LCG to fold, so all blocks will (almost
            always) be non-zero. */
            y_cm = (((1 as libc::c_int) << B) - 1 as libc::c_int) as libc::c_uint;
            x_cm = y_cm
        }
        if dual_stereo != 0 && i == intensity {
            let mut j: libc::c_int = 0;
            /* Switch off dual stereo to do intensity. */
            dual_stereo = 0 as libc::c_int;
            if resynth != 0 {
                j = 0 as libc::c_int;
                while j < M * *eBands.offset(i as isize) as libc::c_int - norm_offset {
                    *norm.offset(j as isize) =
                        0.5f32 * (*norm.offset(j as isize) + *norm2.offset(j as isize));
                    j += 1
                }
            }
        }
        if dual_stereo != 0 {
            x_cm = quant_band(
                &mut ctx,
                X,
                N,
                b / 2 as libc::c_int,
                B,
                if effective_lowband != -(1 as libc::c_int) {
                    norm.offset(effective_lowband as isize)
                } else {
                    0 as *mut crate::arch_h::celt_norm
                },
                LM,
                if last != 0 {
                    0 as *mut crate::arch_h::celt_norm
                } else {
                    norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                        .offset(-(norm_offset as isize))
                },
                1.0f32,
                lowband_scratch,
                x_cm as libc::c_int,
            );
            y_cm = quant_band(
                &mut ctx,
                Y,
                N,
                b / 2 as libc::c_int,
                B,
                if effective_lowband != -(1 as libc::c_int) {
                    norm2.offset(effective_lowband as isize)
                } else {
                    0 as *mut crate::arch_h::celt_norm
                },
                LM,
                if last != 0 {
                    0 as *mut crate::arch_h::celt_norm
                } else {
                    norm2
                        .offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                        .offset(-(norm_offset as isize))
                },
                1.0f32,
                lowband_scratch,
                y_cm as libc::c_int,
            )
        } else {
            if !Y.is_null() {
                if theta_rdo != 0 && i < intensity {
                    let mut ec_save: crate::src::opus_1_2_1::celt::entcode::ec_ctx =
                        crate::src::opus_1_2_1::celt::entcode::ec_ctx {
                            buf: 0 as *mut libc::c_uchar,
                            storage: 0,
                            end_offs: 0,
                            end_window: 0,
                            nend_bits: 0,
                            nbits_total: 0,
                            offs: 0,
                            rng: 0,
                            val: 0,
                            ext: 0,
                            rem: 0,
                            error: 0,
                        };
                    let mut ec_save2: crate::src::opus_1_2_1::celt::entcode::ec_ctx =
                        crate::src::opus_1_2_1::celt::entcode::ec_ctx {
                            buf: 0 as *mut libc::c_uchar,
                            storage: 0,
                            end_offs: 0,
                            end_window: 0,
                            nend_bits: 0,
                            nbits_total: 0,
                            offs: 0,
                            rng: 0,
                            val: 0,
                            ext: 0,
                            rem: 0,
                            error: 0,
                        };
                    let mut ctx_save: band_ctx = band_ctx {
                        encode: 0,
                        resynth: 0,
                        m: 0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                        i: 0,
                        intensity: 0,
                        spread: 0,
                        tf_change: 0,
                        ec: 0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        remaining_bits: 0,
                        bandE: 0 as *const crate::arch_h::celt_ener,
                        seed: 0,
                        arch: 0,
                        theta_round: 0,
                        disable_inv: 0,
                        avoid_split_noise: 0,
                    };
                    let mut ctx_save2: band_ctx = band_ctx {
                        encode: 0,
                        resynth: 0,
                        m: 0 as *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
                        i: 0,
                        intensity: 0,
                        spread: 0,
                        tf_change: 0,
                        ec: 0 as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        remaining_bits: 0,
                        bandE: 0 as *const crate::arch_h::celt_ener,
                        seed: 0,
                        arch: 0,
                        theta_round: 0,
                        disable_inv: 0,
                        avoid_split_noise: 0,
                    };
                    let mut dist0: crate::arch_h::opus_val32 = 0.;
                    let mut dist1: crate::arch_h::opus_val32 = 0.;
                    let mut cm: libc::c_uint = 0;
                    let mut cm2: libc::c_uint = 0;
                    let mut nstart_bytes: libc::c_int = 0;
                    let mut nend_bytes: libc::c_int = 0;
                    let mut save_bytes: libc::c_int = 0;
                    let mut bytes_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut bytes_save: [libc::c_uchar; 1275] = [0; 1275];
                    let mut w: [crate::arch_h::opus_val16; 2] = [0.; 2];
                    compute_channel_weights(
                        *bandE.offset(i as isize),
                        *bandE.offset((i + (*m).nbEBands) as isize),
                        w.as_mut_ptr(),
                    );
                    /* Make a copy. */
                    cm = x_cm | y_cm;
                    ec_save = *ec;
                    ctx_save = ctx;
                    crate::stdlib::memcpy(
                        X_save as *mut libc::c_void,
                        X as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X_save.wrapping_offset_from(X) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    crate::stdlib::memcpy(
                        Y_save as *mut libc::c_void,
                        Y as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y_save.wrapping_offset_from(Y) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    /* Encode and round down. */
                    ctx.theta_round = -(1 as libc::c_int);
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            0 as *mut crate::arch_h::celt_norm
                        },
                        LM,
                        if last != 0 {
                            0 as *mut crate::arch_h::celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        cm as libc::c_int,
                    );
                    dist0 = w[0 as libc::c_int as usize] * celt_inner_prod_c(X_save, X, N)
                        + w[1 as libc::c_int as usize] * celt_inner_prod_c(Y_save, Y, N);
                    /* Save first result. */
                    cm2 = x_cm;
                    ec_save2 = *ec;
                    ctx_save2 = ctx;
                    crate::stdlib::memcpy(
                        X_save2 as *mut libc::c_void,
                        X as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X_save2.wrapping_offset_from(X) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    crate::stdlib::memcpy(
                        Y_save2 as *mut libc::c_void,
                        Y as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y_save2.wrapping_offset_from(Y) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    if last == 0 {
                        crate::stdlib::memcpy(
                            norm_save2 as *mut libc::c_void,
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                                as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>()
                                    as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * norm_save2.wrapping_offset_from(
                                            norm.offset(
                                                (M * *eBands.offset(i as isize) as libc::c_int)
                                                    as isize,
                                            )
                                            .offset(-(norm_offset as isize)),
                                        ) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                    }
                    nstart_bytes = ec_save.offs as libc::c_int;
                    nend_bytes = ec_save.storage as libc::c_int;
                    bytes_buf = ec_save.buf.offset(nstart_bytes as isize);
                    save_bytes = nend_bytes - nstart_bytes;
                    crate::stdlib::memcpy(
                        bytes_save.as_mut_ptr() as *mut libc::c_void,
                        bytes_buf as *const libc::c_void,
                        (save_bytes as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * bytes_save.as_mut_ptr().wrapping_offset_from(bytes_buf)
                                        as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    /* Restore */
                    *ec = ec_save;
                    ctx = ctx_save;
                    crate::stdlib::memcpy(
                        X as *mut libc::c_void,
                        X_save as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * X.wrapping_offset_from(X_save) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    crate::stdlib::memcpy(
                        Y as *mut libc::c_void,
                        Y_save as *const libc::c_void,
                        (N as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<crate::arch_h::celt_norm>() as libc::c_ulong
                            )
                            .wrapping_add(
                                (0 as libc::c_int as libc::c_long
                                    * Y.wrapping_offset_from(Y_save) as libc::c_long)
                                    as libc::c_ulong,
                            ),
                    );
                    if i == start + 1 as libc::c_int {
                        special_hybrid_folding(m, norm, norm2, start, M, dual_stereo);
                    }
                    /* Encode and round up. */
                    ctx.theta_round = 1 as libc::c_int;
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            0 as *mut crate::arch_h::celt_norm
                        },
                        LM,
                        if last != 0 {
                            0 as *mut crate::arch_h::celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        cm as libc::c_int,
                    );
                    dist1 = w[0 as libc::c_int as usize] * celt_inner_prod_c(X_save, X, N)
                        + w[1 as libc::c_int as usize] * celt_inner_prod_c(Y_save, Y, N);
                    if dist0 >= dist1 {
                        x_cm = cm2;
                        *ec = ec_save2;
                        ctx = ctx_save2;
                        crate::stdlib::memcpy(
                            X as *mut libc::c_void,
                            X_save2 as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>()
                                    as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * X.wrapping_offset_from(X_save2) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                        crate::stdlib::memcpy(
                            Y as *mut libc::c_void,
                            Y_save2 as *const libc::c_void,
                            (N as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>()
                                    as libc::c_ulong)
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * Y.wrapping_offset_from(Y_save2) as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                        if last == 0 {
                            crate::stdlib::memcpy(
                                norm.offset(
                                    (M * *eBands.offset(i as isize) as libc::c_int) as isize,
                                )
                                .offset(-(norm_offset as isize))
                                    as *mut libc::c_void,
                                norm_save2 as *const libc::c_void,
                                (N as libc::c_ulong)
                                    .wrapping_mul(::std::mem::size_of::<crate::arch_h::celt_norm>()
                                        as libc::c_ulong)
                                    .wrapping_add(
                                        (0 as libc::c_int as libc::c_long
                                            * norm
                                                .offset(
                                                    (M * *eBands.offset(i as isize) as libc::c_int)
                                                        as isize,
                                                )
                                                .offset(-(norm_offset as isize))
                                                .wrapping_offset_from(norm_save2)
                                                as libc::c_long)
                                            as libc::c_ulong,
                                    ),
                            );
                        }
                        crate::stdlib::memcpy(
                            bytes_buf as *mut libc::c_void,
                            bytes_save.as_mut_ptr() as *const libc::c_void,
                            (save_bytes as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                )
                                .wrapping_add(
                                    (0 as libc::c_int as libc::c_long
                                        * bytes_buf.wrapping_offset_from(bytes_save.as_mut_ptr())
                                            as libc::c_long)
                                        as libc::c_ulong,
                                ),
                        );
                    }
                } else {
                    ctx.theta_round = 0 as libc::c_int;
                    x_cm = quant_band_stereo(
                        &mut ctx,
                        X,
                        Y,
                        N,
                        b,
                        B,
                        if effective_lowband != -(1 as libc::c_int) {
                            norm.offset(effective_lowband as isize)
                        } else {
                            0 as *mut crate::arch_h::celt_norm
                        },
                        LM,
                        if last != 0 {
                            0 as *mut crate::arch_h::celt_norm
                        } else {
                            norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                                .offset(-(norm_offset as isize))
                        },
                        lowband_scratch,
                        (x_cm | y_cm) as libc::c_int,
                    )
                }
            } else {
                x_cm = quant_band(
                    &mut ctx,
                    X,
                    N,
                    b,
                    B,
                    if effective_lowband != -(1 as libc::c_int) {
                        norm.offset(effective_lowband as isize)
                    } else {
                        0 as *mut crate::arch_h::celt_norm
                    },
                    LM,
                    if last != 0 {
                        0 as *mut crate::arch_h::celt_norm
                    } else {
                        norm.offset((M * *eBands.offset(i as isize) as libc::c_int) as isize)
                            .offset(-(norm_offset as isize))
                    },
                    1.0f32,
                    lowband_scratch,
                    (x_cm | y_cm) as libc::c_int,
                )
            }
            y_cm = x_cm
        }
        *collapse_masks.offset((i * C + 0 as libc::c_int) as isize) = x_cm as libc::c_uchar;
        *collapse_masks.offset((i * C + C - 1 as libc::c_int) as isize) = y_cm as libc::c_uchar;
        balance += *pulses.offset(i as isize) + tell;
        /* Update the folding position only as long as we have 1 bit/sample depth. */
        update_lowband = (b > N << 3 as libc::c_int) as libc::c_int;
        /* We only need to avoid noise on a split for the first band. After that, we
        have folding. */
        ctx.avoid_split_noise = 0 as libc::c_int;
        i += 1
    }
    *seed = ctx.seed;
}

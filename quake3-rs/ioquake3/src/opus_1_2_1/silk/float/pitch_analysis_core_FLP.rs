use ::libc;

pub mod SigProc_FIX_h {
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

    pub unsafe extern "C" fn silk_min_int(mut a: i32, mut b: i32) -> i32 {
        return if a < b { a } else { b };
    }
    /* silk_min() versions with typecast in the function call */
    #[inline]

    pub unsafe extern "C" fn silk_max_int(mut a: i32, mut b: i32) -> i32 {
        return if a > b { a } else { b };
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

pub mod SigProc_FLP_h {
    /* floating-point to integer conversion (rounding) */
    #[inline]

    pub unsafe extern "C" fn silk_float2short_array(
        mut out: *mut crate::opus_types_h::opus_int16,
        mut in_0: *const f32,
        mut length: crate::opus_types_h::opus_int32,
    ) {
        let mut k: crate::opus_types_h::opus_int32 = 0;
        k = length - 1 as i32;
        while k >= 0 as i32 {
            *out.offset(k as isize) = if float2int(*in_0.offset(k as isize)) > 0x7fff as i32
            {
                0x7fff as i32
            } else if float2int(*in_0.offset(k as isize))
                < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            {
                0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
            } else {
                float2int(*in_0.offset(k as isize))
            } as crate::opus_types_h::opus_int16;
            k -= 1
        }
    }
    /* integer to floating-point conversion */
    #[inline]

    pub unsafe extern "C" fn silk_short2float_array(
        mut out: *mut f32,
        mut in_0: *const crate::opus_types_h::opus_int16,
        mut length: crate::opus_types_h::opus_int32,
    ) {
        let mut k: crate::opus_types_h::opus_int32 = 0;
        k = length - 1 as i32;
        while k >= 0 as i32 {
            *out.offset(k as isize) = *in_0.offset(k as isize) as f32;
            k -= 1
        }
    }
    /* using log2() helps the fixed-point conversion */
    #[inline]

    pub unsafe extern "C" fn silk_log2(mut x: f64) -> f32 {
        return (3.32192809488736f64 * crate::stdlib::log10(x)) as f32;
    }

    use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::float_cast_h::float2int;

    /* SILK_SIGPROC_FLP_H */
}

pub mod float_cast_h {
    #[inline]

    pub unsafe extern "C" fn float2int(mut x: f32) -> crate::opus_types_h::opus_int32 {
        return _mm_cvt_ss2si(_mm_set_ss(x));
    }

    use ::std::arch::x86_64::_mm_cvt_ss2si;
    use ::std::arch::x86_64::_mm_set_ss;
    /* FLOAT_CAST_H */
    /* DISABLE_FLOAT_API */
}

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;

pub use crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP;
pub use crate::src::opus_1_2_1::silk::float::inner_product_FLP::silk_inner_product_FLP;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::float_cast_h::float2int;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::SigProc_FIX_h::silk_max_int;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::SigProc_FIX_h::silk_min_int;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::SigProc_FLP_h::silk_float2short_array;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::SigProc_FLP_h::silk_log2;
pub use crate::src::opus_1_2_1::silk::float::pitch_analysis_core_FLP::SigProc_FLP_h::silk_short2float_array;
pub use crate::src::opus_1_2_1::silk::float::sort_FLP::silk_insertion_sort_decreasing_FLP;

pub use crate::src::opus_1_2_1::silk::resampler_down2::silk_resampler_down2;
pub use crate::src::opus_1_2_1::silk::resampler_down2_3::silk_resampler_down2_3;

/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
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
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* *******************************************************************/
/*                    SIGNAL PROCESSING FUNCTIONS                   */
/* *******************************************************************/
/* Chirp (bw expand) LP AR filter */
/* I/O  AR filter to be expanded (without leading 1)                */
/* I    length of ar                                                */
/* I    chirp factor (typically in range (0..1) )                   */
/* compute inverse of LPC prediction gain, and                          */
/* test if LPC coefficients are stable (all poles within unit circle)   */
/* this code is based on silk_FLP_a2k()                                 */
/* O    return inverse prediction gain, energy domain               */
/* I    prediction coefficients [order]                             */
/* I    prediction order                                            */
/* O    returns residual energy                                     */
/* O    reflection coefficients (length order)                      */
/* I    autocorrelation sequence (length order+1)                   */
/* I    order                                                       */
/* O     prediction coefficients [order]                            */
/* I     reflection coefficients [order]                            */
/* I     prediction order                                           */
/* compute autocorrelation */
/* O    result (length correlationCount)                            */
/* I    input data to correlate                                     */
/* I    length of input                                             */
/* I    number of correlation taps to compute                       */
/* ***********************************************************/
/* CORE PITCH ANALYSIS FUNCTION                             */
/* ***********************************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_pitch_analysis_core_FLP(
    mut frame: *const f32,
    mut pitch_out: *mut i32,
    mut lagIndex: *mut crate::opus_types_h::opus_int16,
    mut contourIndex: *mut i8,
    mut LTPCorr: *mut f32,
    mut prevLag: i32,
    search_thres1: f32,
    search_thres2: f32,
    Fs_kHz: i32,
    complexity: i32,
    nb_subfr: i32,
    mut arch: i32,
) -> i32
/* I    Run-time architecture                                       */ {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut d: i32 = 0;
    let mut j: i32 = 0;
    let mut frame_8kHz: [f32; 320] = [0.; 320];
    let mut frame_4kHz: [f32; 160] = [0.; 160];
    let mut frame_8_FIX: [crate::opus_types_h::opus_int16; 320] = [0; 320];
    let mut frame_4_FIX: [crate::opus_types_h::opus_int16; 160] = [0; 160];
    let mut filt_state: [crate::opus_types_h::opus_int32; 6] = [0; 6];
    let mut threshold: f32 = 0.;
    let mut contour_bias: f32 = 0.;
    let mut C: [[f32; 149]; 4] = [[0.; 149]; 4];
    let mut xcorr: [crate::arch_h::opus_val32; 65] = [0.; 65];
    let mut CC: [f32; 11] = [0.; 11];
    let mut target_ptr: *const f32 = 0 as *const f32;
    let mut basis_ptr: *const f32 = 0 as *const f32;
    let mut cross_corr: f64 = 0.;
    let mut normalizer: f64 = 0.;
    let mut energy: f64 = 0.;
    let mut energy_tmp: f64 = 0.;
    let mut d_srch: [i32; 24] = [0; 24];
    let mut d_comp: [crate::opus_types_h::opus_int16; 149] = [0; 149];
    let mut length_d_srch: i32 = 0;
    let mut length_d_comp: i32 = 0;
    let mut Cmax: f32 = 0.;
    let mut CCmax: f32 = 0.;
    let mut CCmax_b: f32 = 0.;
    let mut CCmax_new_b: f32 = 0.;
    let mut CCmax_new: f32 = 0.;
    let mut CBimax: i32 = 0;
    let mut CBimax_new: i32 = 0;
    let mut lag: i32 = 0;
    let mut start_lag: i32 = 0;
    let mut end_lag: i32 = 0;
    let mut lag_new: i32 = 0;
    let mut cbk_size: i32 = 0;
    let mut lag_log2: f32 = 0.;
    let mut prevLag_log2: f32 = 0.;
    let mut delta_lag_log2_sqr: f32 = 0.;
    let mut energies_st3: [[[f32; 5]; 34]; 4] = [[[0.; 5]; 34]; 4];
    let mut cross_corr_st3: [[[f32; 5]; 34]; 4] = [[[0.; 5]; 34]; 4];
    let mut lag_counter: i32 = 0;
    let mut frame_length: i32 = 0;
    let mut frame_length_8kHz: i32 = 0;
    let mut frame_length_4kHz: i32 = 0;
    let mut sf_length: i32 = 0;
    let mut sf_length_8kHz: i32 = 0;
    let mut sf_length_4kHz: i32 = 0;
    let mut min_lag: i32 = 0;
    let mut min_lag_8kHz: i32 = 0;
    let mut min_lag_4kHz: i32 = 0;
    let mut max_lag: i32 = 0;
    let mut max_lag_8kHz: i32 = 0;
    let mut max_lag_4kHz: i32 = 0;
    let mut nb_cbk_search: i32 = 0;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    /* Check for valid sampling frequency */
    /* Check for valid complexity setting */
    /* Set up frame lengths max / min lag for the sampling frequency */
    frame_length = (4 as i32 * 5 as i32 + nb_subfr * 5 as i32) * Fs_kHz;
    frame_length_4kHz =
        (4 as i32 * 5 as i32 + nb_subfr * 5 as i32) * 4 as i32;
    frame_length_8kHz =
        (4 as i32 * 5 as i32 + nb_subfr * 5 as i32) * 8 as i32;
    sf_length = 5 as i32 * Fs_kHz;
    sf_length_4kHz = 5 as i32 * 4 as i32;
    sf_length_8kHz = 5 as i32 * 8 as i32;
    min_lag = 2 as i32 * Fs_kHz;
    min_lag_4kHz = 2 as i32 * 4 as i32;
    min_lag_8kHz = 2 as i32 * 8 as i32;
    max_lag = 18 as i32 * Fs_kHz - 1 as i32;
    max_lag_4kHz = 18 as i32 * 4 as i32;
    max_lag_8kHz = 18 as i32 * 8 as i32 - 1 as i32;
    /* Resample from input sampled at Fs_kHz to 8 kHz */
    if Fs_kHz == 16 as i32 {
        /* Resample to 16 -> 8 khz */
        let mut frame_16_FIX: [crate::opus_types_h::opus_int16; 640] = [0; 640];
        silk_float2short_array(frame_16_FIX.as_mut_ptr(), frame, frame_length);
        crate::stdlib::memset(
            filt_state.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (2 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
        crate::src::opus_1_2_1::silk::resampler_down2::silk_resampler_down2(
            filt_state.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_16_FIX.as_mut_ptr(),
            frame_length,
        );
        silk_short2float_array(
            frame_8kHz.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_length_8kHz,
        );
    } else if Fs_kHz == 12 as i32 {
        /* Resample to 12 -> 8 khz */
        let mut frame_12_FIX: [crate::opus_types_h::opus_int16; 480] = [0; 480];
        silk_float2short_array(frame_12_FIX.as_mut_ptr(), frame, frame_length);
        crate::stdlib::memset(
            filt_state.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (6 as i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::opus_types_h::opus_int32,
            >() as libc::c_ulong),
        );
        crate::src::opus_1_2_1::silk::resampler_down2_3::silk_resampler_down2_3(
            filt_state.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_12_FIX.as_mut_ptr(),
            frame_length,
        );
        silk_short2float_array(
            frame_8kHz.as_mut_ptr(),
            frame_8_FIX.as_mut_ptr(),
            frame_length_8kHz,
        );
    } else {
        silk_float2short_array(frame_8_FIX.as_mut_ptr(), frame, frame_length_8kHz);
    }
    /* Decimate again to 4 kHz */
    crate::stdlib::memset(filt_state.as_mut_ptr() as *mut libc::c_void, 0 as i32,
           (2 as i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<crate::opus_types_h::opus_int32>()
                                                as libc::c_ulong));
    crate::src::opus_1_2_1::silk::resampler_down2::silk_resampler_down2(
        filt_state.as_mut_ptr(),
        frame_4_FIX.as_mut_ptr(),
        frame_8_FIX.as_mut_ptr(),
        frame_length_8kHz,
    );
    silk_short2float_array(
        frame_4kHz.as_mut_ptr(),
        frame_4_FIX.as_mut_ptr(),
        frame_length_4kHz,
    );
    /* Low-pass filter */
    i = frame_length_4kHz - 1 as i32;
    while i > 0 as i32 {
        frame_4kHz[i as usize] = if frame_4kHz[i as usize] as crate::opus_types_h::opus_int32
            as f32
            + frame_4kHz[(i - 1 as i32) as usize]
            > 0x7fff as i32 as f32
        {
            0x7fff as i32 as f32
        } else if frame_4kHz[i as usize] as crate::opus_types_h::opus_int32 as f32
            + frame_4kHz[(i - 1 as i32) as usize]
            < 0x8000 as i32 as crate::opus_types_h::opus_int16 as i32
                as f32
        {
            0x8000 as i32 as crate::opus_types_h::opus_int16 as i32 as f32
        } else {
            (frame_4kHz[i as usize] as crate::opus_types_h::opus_int32 as f32)
                + frame_4kHz[(i - 1 as i32) as usize]
        } as crate::opus_types_h::opus_int16 as f32;
        i -= 1
    }
    /* *****************************************************************************
     * FIRST STAGE, operating in 4 khz
     ******************************************************************************/
    crate::stdlib::memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul(nb_subfr as libc::c_ulong)
            .wrapping_mul(
                ((18 as i32 * 16 as i32 >> 1 as i32) + 5 as i32)
                    as libc::c_ulong,
            ),
    );
    target_ptr = &mut *frame_4kHz.as_mut_ptr().offset(
        ((sf_length_4kHz as crate::opus_types_h::opus_uint32) << 2 as i32)
            as crate::opus_types_h::opus_int32 as isize,
    ) as *mut f32;
    k = 0 as i32;
    while k < nb_subfr >> 1 as i32 {
        /* Check that we are within range of the array */
        basis_ptr = target_ptr.offset(-(min_lag_4kHz as isize));
        /* Check that we are within range of the array */
        crate::src::opus_1_2_1::celt::pitch::celt_pitch_xcorr_c(
            target_ptr,
            target_ptr.offset(-(max_lag_4kHz as isize)),
            xcorr.as_mut_ptr(),
            sf_length_8kHz,
            max_lag_4kHz - min_lag_4kHz + 1 as i32,
            arch,
        );
        /* Calculate first vector products before loop */
        cross_corr = xcorr[(max_lag_4kHz - min_lag_4kHz) as usize] as f64;
        normalizer = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            target_ptr,
            sf_length_8kHz,
        ) + crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            basis_ptr,
            sf_length_8kHz,
        ) + (sf_length_8kHz as f32 * 4000.0f32) as f64;
        C[0 as i32 as usize][min_lag_4kHz as usize] +=
            (2 as i32 as f64 * cross_corr / normalizer) as f32;
        /* From now on normalizer is computed recursively */
        d = min_lag_4kHz + 1 as i32;
        while d <= max_lag_4kHz {
            basis_ptr = basis_ptr.offset(-1);
            /* Check that we are within range of the array */
            cross_corr = xcorr[(max_lag_4kHz - d) as usize] as f64;
            /* Add contribution of new sample and remove contribution from oldest sample */
            normalizer += *basis_ptr.offset(0 as i32 as isize) as f64
                * *basis_ptr.offset(0 as i32 as isize) as f64
                - *basis_ptr.offset(sf_length_8kHz as isize) as f64
                    * *basis_ptr.offset(sf_length_8kHz as isize) as f64;
            C[0 as i32 as usize][d as usize] +=
                (2 as i32 as f64 * cross_corr / normalizer) as f32;
            d += 1
        }
        /* Update target pointer */
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1
    }
    /* Apply short-lag bias */
    i = max_lag_4kHz;
    while i >= min_lag_4kHz {
        C[0 as i32 as usize][i as usize] -=
            C[0 as i32 as usize][i as usize] * i as f32 / 4096.0f32;
        i -= 1
    }
    /* Sort */
    length_d_srch = 4 as i32 + 2 as i32 * complexity;
    crate::src::opus_1_2_1::silk::float::sort_FLP::silk_insertion_sort_decreasing_FLP(
        &mut *(*C.as_mut_ptr().offset(0 as i32 as isize))
            .as_mut_ptr()
            .offset(min_lag_4kHz as isize),
        d_srch.as_mut_ptr(),
        max_lag_4kHz - min_lag_4kHz + 1 as i32,
        length_d_srch,
    );
    /* Escape if correlation is very low already here */
    Cmax = C[0 as i32 as usize][min_lag_4kHz as usize];
    if Cmax < 0.2f32 {
        crate::stdlib::memset(
            pitch_out as *mut libc::c_void,
            0 as i32,
            (nb_subfr as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        );
        *LTPCorr = 0.0f32;
        *lagIndex = 0 as i32 as crate::opus_types_h::opus_int16;
        *contourIndex = 0 as i32 as i8;
        return 1 as i32;
    }
    threshold = search_thres1 * Cmax;
    i = 0 as i32;
    while i < length_d_srch {
        /* Convert to 8 kHz indices for the sorted correlation that exceeds the threshold */
        if C[0 as i32 as usize][(min_lag_4kHz + i) as usize] > threshold {
            d_srch[i as usize] =
                (((d_srch[i as usize] + min_lag_4kHz) as crate::opus_types_h::opus_uint32)
                    << 1 as i32) as crate::opus_types_h::opus_int32;
            i += 1
        } else {
            length_d_srch = i;
            break;
        }
    }
    i = min_lag_8kHz - 5 as i32;
    while i < max_lag_8kHz + 5 as i32 {
        d_comp[i as usize] = 0 as i32 as crate::opus_types_h::opus_int16;
        i += 1
    }
    i = 0 as i32;
    while i < length_d_srch {
        d_comp[d_srch[i as usize] as usize] = 1 as i32 as crate::opus_types_h::opus_int16;
        i += 1
    }
    /* Convolution */
    i = max_lag_8kHz + 3 as i32;
    while i >= min_lag_8kHz {
        d_comp[i as usize] = (d_comp[i as usize] as i32
            + (d_comp[(i - 1 as i32) as usize] as i32
                + d_comp[(i - 2 as i32) as usize] as i32))
            as crate::opus_types_h::opus_int16;
        i -= 1
    }
    length_d_srch = 0 as i32;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 1 as i32 {
        if d_comp[(i + 1 as i32) as usize] as i32 > 0 as i32 {
            d_srch[length_d_srch as usize] = i;
            length_d_srch += 1
        }
        i += 1
    }
    /* Convolution */
    i = max_lag_8kHz + 3 as i32;
    while i >= min_lag_8kHz {
        d_comp[i as usize] = (d_comp[i as usize] as i32
            + (d_comp[(i - 1 as i32) as usize] as i32
                + d_comp[(i - 2 as i32) as usize] as i32
                + d_comp[(i - 3 as i32) as usize] as i32))
            as crate::opus_types_h::opus_int16;
        i -= 1
    }
    length_d_comp = 0 as i32;
    i = min_lag_8kHz;
    while i < max_lag_8kHz + 4 as i32 {
        if d_comp[i as usize] as i32 > 0 as i32 {
            d_comp[length_d_comp as usize] =
                (i - 2 as i32) as crate::opus_types_h::opus_int16;
            length_d_comp += 1
        }
        i += 1
    }
    /* *********************************************************************************
     ** SECOND STAGE, operating at 8 kHz, on lag sections with high correlation
     *************************************************************************************/
    /* ********************************************************************************
     * Find energy of each subframe projected onto its history, for a range of delays
     *********************************************************************************/
    crate::stdlib::memset(
        C.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ((4 as i32
            * ((18 as i32 * 16 as i32 >> 1 as i32) + 5 as i32))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
    );
    if Fs_kHz == 8 as i32 {
        target_ptr = &*frame
            .offset((4 as i32 * 5 as i32 * 8 as i32) as isize)
            as *const f32
    } else {
        target_ptr = &mut *frame_8kHz
            .as_mut_ptr()
            .offset((4 as i32 * 5 as i32 * 8 as i32) as isize)
            as *mut f32
    }
    k = 0 as i32;
    while k < nb_subfr {
        energy_tmp = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            target_ptr,
            sf_length_8kHz,
        ) + 1.0f64;
        j = 0 as i32;
        while j < length_d_comp {
            d = d_comp[j as usize] as i32;
            basis_ptr = target_ptr.offset(-(d as isize));
            cross_corr =
                crate::src::opus_1_2_1::silk::float::inner_product_FLP::silk_inner_product_FLP(
                    basis_ptr,
                    target_ptr,
                    sf_length_8kHz,
                );
            if cross_corr > 0.0f32 as f64 {
                energy = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                    basis_ptr,
                    sf_length_8kHz,
                );
                C[k as usize][d as usize] = (2 as i32 as f64 * cross_corr
                    / (energy + energy_tmp))
                    as f32
            } else {
                C[k as usize][d as usize] = 0.0f32
            }
            j += 1
        }
        target_ptr = target_ptr.offset(sf_length_8kHz as isize);
        k += 1
    }
    /* search over lag range and lags codebook */
    /* scale factor for lag codebook, as a function of center lag */
    CCmax = 0.0f32; /* This value doesn't matter */
    CCmax_b = -1000.0f32; /* To avoid returning undefined lag values */
    CBimax = 0 as i32; /* To check if lag with strong enough correlation has been found */
    lag = -(1 as i32);
    if prevLag > 0 as i32 {
        if Fs_kHz == 12 as i32 {
            prevLag = ((prevLag as crate::opus_types_h::opus_uint32) << 1 as i32)
                as crate::opus_types_h::opus_int32
                / 3 as i32
        } else if Fs_kHz == 16 as i32 {
            prevLag = prevLag >> 1 as i32
        }
        prevLag_log2 = silk_log2(prevLag as f32 as f64)
    } else {
        prevLag_log2 = 0 as i32 as f32
    }
    /* Set up stage 2 codebook based on number of subframes */
    if nb_subfr == 4 as i32 {
        cbk_size = 11 as i32;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage2
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        if Fs_kHz == 8 as i32 && complexity > 0 as i32 {
            /* If input is 8 khz use a larger codebook here because it is last stage */
            nb_cbk_search = 11 as i32
        } else {
            nb_cbk_search = 3 as i32
        }
    } else {
        cbk_size = 3 as i32;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage2_10_ms
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        nb_cbk_search = 3 as i32
    }
    k = 0 as i32;
    while k < length_d_srch {
        d = d_srch[k as usize];
        j = 0 as i32;
        while j < nb_cbk_search {
            CC[j as usize] = 0.0f32;
            i = 0 as i32;
            while i < nb_subfr {
                /* Try all codebooks */
                CC[j as usize] += C[i as usize]
                    [(d + *Lag_CB_ptr.offset((i * cbk_size + j) as isize) as i32) as usize];
                i += 1
            }
            j += 1
        }
        /* Find best codebook */
        CCmax_new = -1000.0f32;
        CBimax_new = 0 as i32;
        i = 0 as i32;
        while i < nb_cbk_search {
            if CC[i as usize] > CCmax_new {
                CCmax_new = CC[i as usize];
                CBimax_new = i
            }
            i += 1
        }
        /* Bias towards shorter lags */
        lag_log2 = silk_log2(d as f32 as f64);
        CCmax_new_b = CCmax_new - 0.2f32 * nb_subfr as f32 * lag_log2;
        /* Bias towards previous lag */
        if prevLag > 0 as i32 {
            delta_lag_log2_sqr = lag_log2 - prevLag_log2;
            delta_lag_log2_sqr *= delta_lag_log2_sqr;
            CCmax_new_b -= 0.2f32 * nb_subfr as f32 * *LTPCorr * delta_lag_log2_sqr
                / (delta_lag_log2_sqr + 0.5f32)
        }
        if CCmax_new_b > CCmax_b && CCmax_new > nb_subfr as f32 * search_thres2 {
            /* Correlation needs to be high enough to be voiced */
            CCmax_b = CCmax_new_b;
            CCmax = CCmax_new;
            lag = d;
            CBimax = CBimax_new
        }
        k += 1
    }
    if lag == -(1 as i32) {
        /* No suitable candidate found */
        crate::stdlib::memset(
            pitch_out as *mut libc::c_void,
            0 as i32,
            (4 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        );
        *LTPCorr = 0.0f32;
        *lagIndex = 0 as i32 as crate::opus_types_h::opus_int16;
        *contourIndex = 0 as i32 as i8;
        return 1 as i32;
    }
    /* Output normalized correlation */
    *LTPCorr = CCmax / nb_subfr as f32; /* Fs_kHz == 8 */
    if Fs_kHz > 8 as i32 {
        if Fs_kHz == 12 as i32 {
            lag = if 1 as i32 == 1 as i32 {
                (lag as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * 3 as i32 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
                    >> 1 as i32)
                    + (lag as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                        * 3 as i32 as crate::opus_types_h::opus_int16
                            as crate::opus_types_h::opus_int32
                        & 1 as i32)
            } else {
                ((lag as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
                    * 3 as i32 as crate::opus_types_h::opus_int16
                        as crate::opus_types_h::opus_int32
                    >> 1 as i32 - 1 as i32)
                    + 1 as i32)
                    >> 1 as i32
            }
        } else {
            lag = ((lag as crate::opus_types_h::opus_uint32) << 1 as i32)
                as crate::opus_types_h::opus_int32
        } /* Fs_kHz == 16 */
        lag = if min_lag > max_lag {
            if lag > min_lag {
                min_lag
            } else if lag < max_lag {
                max_lag
            } else {
                lag
            }
        } else if lag > max_lag {
            max_lag
        } else if lag < min_lag {
            min_lag
        } else {
            lag
        }; /* to avoid undefined lag */
        start_lag = silk_max_int(lag - 2 as i32, min_lag); /* to avoid undefined lag */
        end_lag = silk_min_int(lag + 2 as i32, max_lag);
        lag_new = lag;
        CBimax = 0 as i32;
        CCmax = -1000.0f32;
        /* Calculate the correlations and energies needed in stage 3 */
        silk_P_Ana_calc_corr_st3(
            cross_corr_st3.as_mut_ptr(),
            frame,
            start_lag,
            sf_length,
            nb_subfr,
            complexity,
            arch,
        );
        silk_P_Ana_calc_energy_st3(
            energies_st3.as_mut_ptr(),
            frame,
            start_lag,
            sf_length,
            nb_subfr,
            complexity,
        );
        lag_counter = 0 as i32;
        contour_bias = 0.05f32 / lag as f32;
        /* Set up cbk parameters according to complexity setting and frame length */
        if nb_subfr == 4 as i32 {
            nb_cbk_search =
                crate::src::opus_1_2_1::silk::pitch_est_tables::silk_nb_cbk_searchs_stage3
                    [complexity as usize] as i32;
            cbk_size = 34 as i32;
            Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3
                .as_ptr()
                .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8
        } else {
            nb_cbk_search = 12 as i32;
            cbk_size = 12 as i32;
            Lag_CB_ptr =
                &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3_10_ms
                    .as_ptr()
                    .offset(0 as i32 as isize))
                .as_ptr()
                .offset(0 as i32 as isize) as *const i8
        }
        target_ptr = &*frame.offset((4 as i32 * 5 as i32 * Fs_kHz) as isize)
            as *const f32;
        energy_tmp = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            target_ptr,
            nb_subfr * sf_length,
        ) + 1.0f64;
        d = start_lag;
        while d <= end_lag {
            j = 0 as i32;
            while j < nb_cbk_search {
                cross_corr = 0.0f64;
                energy = energy_tmp;
                k = 0 as i32;
                while k < nb_subfr {
                    cross_corr += cross_corr_st3[k as usize][j as usize][lag_counter as usize]
                        as f64;
                    energy += energies_st3[k as usize][j as usize][lag_counter as usize]
                        as f64;
                    k += 1
                }
                if cross_corr > 0.0f64 {
                    CCmax_new =
                        (2 as i32 as f64 * cross_corr / energy) as f32;
                    /* Reduce depending on flatness of contour */
                    CCmax_new *= 1.0f32 - contour_bias * j as f32
                } else {
                    CCmax_new = 0.0f32
                }
                if CCmax_new > CCmax
                    && d + crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3
                        [0 as i32 as usize][j as usize]
                        as i32
                        <= max_lag
                {
                    CCmax = CCmax_new;
                    lag_new = d;
                    CBimax = j
                }
                j += 1
            }
            lag_counter += 1;
            d += 1
        }
        k = 0 as i32;
        while k < nb_subfr {
            *pitch_out.offset(k as isize) =
                lag_new + *Lag_CB_ptr.offset((k * cbk_size + CBimax) as isize) as i32;
            *pitch_out.offset(k as isize) = if min_lag > 18 as i32 * Fs_kHz {
                if *pitch_out.offset(k as isize) > min_lag {
                    min_lag
                } else if *pitch_out.offset(k as isize) < 18 as i32 * Fs_kHz {
                    (18 as i32) * Fs_kHz
                } else {
                    *pitch_out.offset(k as isize)
                }
            } else if *pitch_out.offset(k as isize) > 18 as i32 * Fs_kHz {
                (18 as i32) * Fs_kHz
            } else if *pitch_out.offset(k as isize) < min_lag {
                min_lag
            } else {
                *pitch_out.offset(k as isize)
            };
            k += 1
        }
        *lagIndex = (lag_new - min_lag) as crate::opus_types_h::opus_int16;
        *contourIndex = CBimax as i8
    } else {
        /* Save Lags */
        k = 0 as i32;
        while k < nb_subfr {
            *pitch_out.offset(k as isize) =
                lag + *Lag_CB_ptr.offset((k * cbk_size + CBimax) as isize) as i32;
            *pitch_out.offset(k as isize) = if min_lag_8kHz > 18 as i32 * 8 as i32 {
                if *pitch_out.offset(k as isize) > min_lag_8kHz {
                    min_lag_8kHz
                } else if *pitch_out.offset(k as isize) < 18 as i32 * 8 as i32 {
                    (18 as i32) * 8 as i32
                } else {
                    *pitch_out.offset(k as isize)
                }
            } else if *pitch_out.offset(k as isize) > 18 as i32 * 8 as i32 {
                (18 as i32) * 8 as i32
            } else if *pitch_out.offset(k as isize) < min_lag_8kHz {
                min_lag_8kHz
            } else {
                *pitch_out.offset(k as isize)
            };
            k += 1
        }
        *lagIndex = (lag - min_lag_8kHz) as crate::opus_types_h::opus_int16;
        *contourIndex = CBimax as i8
    }
    /* return as voiced */
    return 0 as i32;
}
/* **********************************************************************
Copyright (c) 2006-2011, Skype Limited. All rights reserved.
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
- Redistributions of source code must retain the above copyright notice,
this list of conditions and the following disclaimer.
- Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.
- Neither the name of Internet Society, IETF or IETF Trust, nor the
names of specific contributors, may be used to endorse or promote
products derived from this software without specific prior written
permission.
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
POSSIBILITY OF SUCH DAMAGE.
***********************************************************************/
/* ****************************************************************************
* Pitch analyser function
******************************************************************************/
/* ***********************************************************/
/* Internally used functions                                */
/* ***********************************************************/
/* **********************************************************************
 * Calculates the correlations used in stage 3 search. In order to cover
 * the whole lag codebook for all the searched offset lags (lag +- 2),
 * the following correlations are needed in each sub frame:
 *
 * sf1: lag range [-8,...,7] total 16 correlations
 * sf2: lag range [-4,...,4] total 9 correlations
 * sf3: lag range [-3,....4] total 8 correltions
 * sf4: lag range [-6,....8] total 15 correlations
 *
 * In total 48 correlations. The direct implementation computed in worst
 * case 4*12*5 = 240 correlations, but more likely around 120.
 ***********************************************************************/

unsafe extern "C" fn silk_P_Ana_calc_corr_st3(
    mut cross_corr_st3: *mut [[f32; 5]; 34],
    mut frame: *const f32,
    mut start_lag: i32,
    mut sf_length: i32,
    mut nb_subfr: i32,
    mut complexity: i32,
    mut arch: i32,
)
/* I Run-time architecture                                          */
{
    let mut target_ptr: *const f32 = 0 as *const f32; /* Pointer to middle of frame */
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut lag_counter: i32 = 0;
    let mut lag_low: i32 = 0;
    let mut lag_high: i32 = 0;
    let mut nb_cbk_search: i32 = 0;
    let mut delta: i32 = 0;
    let mut idx: i32 = 0;
    let mut cbk_size: i32 = 0;
    let mut scratch_mem: [f32; 22] = [0.; 22];
    let mut xcorr: [crate::arch_h::opus_val32; 22] = [0.; 22];
    let mut Lag_range_ptr: *const i8 = 0 as *const i8;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if nb_subfr == 4 as i32 {
        Lag_range_ptr =
            &*(*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_Lag_range_stage3
                .as_ptr()
                .offset(complexity as isize))
            .as_ptr()
            .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        nb_cbk_search = crate::src::opus_1_2_1::silk::pitch_est_tables::silk_nb_cbk_searchs_stage3
            [complexity as usize] as i32;
        cbk_size = 34 as i32
    } else {
        Lag_range_ptr =
            &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_Lag_range_stage3_10_ms
                .as_ptr()
                .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        nb_cbk_search = 12 as i32;
        cbk_size = 12 as i32
    }
    target_ptr = &*frame.offset(
        ((sf_length as crate::opus_types_h::opus_uint32) << 2 as i32)
            as crate::opus_types_h::opus_int32 as isize,
    ) as *const f32;
    k = 0 as i32;
    while k < nb_subfr {
        lag_counter = 0 as i32;
        /* Calculate the correlations for each subframe */
        lag_low = *Lag_range_ptr.offset((k * 2 as i32 + 0 as i32) as isize)
            as i32;
        lag_high = *Lag_range_ptr.offset((k * 2 as i32 + 1 as i32) as isize)
            as i32;
        crate::src::opus_1_2_1::celt::pitch::celt_pitch_xcorr_c(
            target_ptr,
            target_ptr
                .offset(-(start_lag as isize))
                .offset(-(lag_high as isize)),
            xcorr.as_mut_ptr(),
            sf_length,
            lag_high - lag_low + 1 as i32,
            arch,
        );
        j = lag_low;
        while j <= lag_high {
            scratch_mem[lag_counter as usize] = xcorr[(lag_high - j) as usize];
            lag_counter += 1;
            j += 1
        }
        delta = *Lag_range_ptr.offset((k * 2 as i32 + 0 as i32) as isize)
            as i32;
        i = 0 as i32;
        while i < nb_cbk_search {
            /* Fill out the 3 dim array that stores the correlations for */
            /* each code_book vector for each start lag */
            idx = *Lag_CB_ptr.offset((k * cbk_size + i) as isize) as i32 - delta;
            j = 0 as i32;
            while j < 5 as i32 {
                (*cross_corr_st3.offset(k as isize))[i as usize][j as usize] =
                    scratch_mem[(idx + j) as usize];
                j += 1
            }
            i += 1
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1
    }
}
/* *******************************************************************/
/* Calculate the energies for first two subframes. The energies are */
/* calculated recursively.                                          */
/* *******************************************************************/

unsafe extern "C" fn silk_P_Ana_calc_energy_st3(
    mut energies_st3: *mut [[f32; 5]; 34],
    mut frame: *const f32,
    mut start_lag: i32,
    mut sf_length: i32,
    mut nb_subfr: i32,
    mut complexity: i32,
)
/* I Complexity setting                                             */
{
    let mut target_ptr: *const f32 = 0 as *const f32;
    let mut basis_ptr: *const f32 = 0 as *const f32;
    let mut energy: f64 = 0.;
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut lag_counter: i32 = 0;
    let mut nb_cbk_search: i32 = 0;
    let mut delta: i32 = 0;
    let mut idx: i32 = 0;
    let mut cbk_size: i32 = 0;
    let mut lag_diff: i32 = 0;
    let mut scratch_mem: [f32; 22] = [0.; 22];
    let mut Lag_range_ptr: *const i8 = 0 as *const i8;
    let mut Lag_CB_ptr: *const i8 = 0 as *const i8;
    if nb_subfr == 4 as i32 {
        Lag_range_ptr =
            &*(*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_Lag_range_stage3
                .as_ptr()
                .offset(complexity as isize))
            .as_ptr()
            .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        nb_cbk_search = crate::src::opus_1_2_1::silk::pitch_est_tables::silk_nb_cbk_searchs_stage3
            [complexity as usize] as i32;
        cbk_size = 34 as i32
    } else {
        Lag_range_ptr =
            &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_Lag_range_stage3_10_ms
                .as_ptr()
                .offset(0 as i32 as isize))
            .as_ptr()
            .offset(0 as i32 as isize) as *const i8;
        Lag_CB_ptr = &*(*crate::src::opus_1_2_1::silk::pitch_est_tables::silk_CB_lags_stage3_10_ms
            .as_ptr()
            .offset(0 as i32 as isize))
        .as_ptr()
        .offset(0 as i32 as isize) as *const i8;
        nb_cbk_search = 12 as i32;
        cbk_size = 12 as i32
    }
    target_ptr = &*frame.offset(
        ((sf_length as crate::opus_types_h::opus_uint32) << 2 as i32)
            as crate::opus_types_h::opus_int32 as isize,
    ) as *const f32;
    k = 0 as i32;
    while k < nb_subfr {
        lag_counter = 0 as i32;
        /* Calculate the energy for first lag */
        basis_ptr = target_ptr.offset(
            -((start_lag
                + *Lag_range_ptr.offset((k * 2 as i32 + 0 as i32) as isize)
                    as i32) as isize),
        );
        energy =
            crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(basis_ptr, sf_length)
                + 1e-3f64;
        scratch_mem[lag_counter as usize] = energy as f32;
        lag_counter += 1;
        lag_diff = *Lag_range_ptr.offset((k * 2 as i32 + 1 as i32) as isize)
            as i32
            - *Lag_range_ptr.offset((k * 2 as i32 + 0 as i32) as isize)
                as i32
            + 1 as i32;
        i = 1 as i32;
        while i < lag_diff {
            /* remove part outside new window */
            energy -= *basis_ptr.offset((sf_length - i) as isize) as f64
                * *basis_ptr.offset((sf_length - i) as isize) as f64;
            /* add part that comes into window */
            energy += *basis_ptr.offset(-i as isize) as f64
                * *basis_ptr.offset(-i as isize) as f64;
            scratch_mem[lag_counter as usize] = energy as f32;
            lag_counter += 1;
            i += 1
        }
        delta = *Lag_range_ptr.offset((k * 2 as i32 + 0 as i32) as isize)
            as i32;
        i = 0 as i32;
        while i < nb_cbk_search {
            /* Fill out the 3 dim array that stores the correlations for    */
            /* each code_book vector for each start lag                     */
            idx = *Lag_CB_ptr.offset((k * cbk_size + i) as isize) as i32 - delta;
            j = 0 as i32;
            while j < 5 as i32 {
                (*energies_st3.offset(k as isize))[i as usize][j as usize] =
                    scratch_mem[(idx + j) as usize];
                j += 1
            }
            i += 1
        }
        target_ptr = target_ptr.offset(sf_length as isize);
        k += 1
    }
}

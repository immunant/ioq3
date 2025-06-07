use ::libc;

pub use crate::arch_h::opus_val16;
pub use crate::arch_h::opus_val32;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::src::opus_1_2_1::celt::kiss_fft::arch_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_fft_state;
pub use crate::src::opus_1_2_1::celt::kiss_fft::kiss_twiddle_cpx;
pub use crate::src::opus_1_2_1::celt::mdct::mdct_lookup;
pub use crate::src::opus_1_2_1::celt::modes::OpusCustomMode;
pub use crate::src::opus_1_2_1::celt::modes::PulseCache;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

/* CUSTOM_MODES */
/* Copyright (c) 2007-2008 CSIRO
Copyright (c) 2007-2010 Xiph.Org Foundation
Copyright (c) 2008 Gregory Maxwell
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

pub unsafe extern "C" fn resampling_factor(
    mut rate: crate::opus_types_h::opus_int32,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    match rate {
        48000 => ret = 1 as libc::c_int,
        24000 => ret = 2 as libc::c_int,
        16000 => ret = 3 as libc::c_int,
        12000 => ret = 4 as libc::c_int,
        8000 => ret = 6 as libc::c_int,
        _ => ret = 0 as libc::c_int,
    }
    return ret;
}
/* This version should be faster on ARM */

unsafe extern "C" fn comb_filter_const_c(
    mut y: *mut crate::arch_h::opus_val32,
    mut x: *mut crate::arch_h::opus_val32,
    mut T: libc::c_int,
    mut N: libc::c_int,
    mut g10: crate::arch_h::opus_val16,
    mut g11: crate::arch_h::opus_val16,
    mut g12: crate::arch_h::opus_val16,
) {
    let mut x0: crate::arch_h::opus_val32 = 0.;
    let mut x1: crate::arch_h::opus_val32 = 0.;
    let mut x2: crate::arch_h::opus_val32 = 0.;
    let mut x3: crate::arch_h::opus_val32 = 0.;
    let mut x4: crate::arch_h::opus_val32 = 0.;
    let mut i: libc::c_int = 0;
    x4 = *x.offset((-T - 2 as libc::c_int) as isize);
    x3 = *x.offset((-T - 1 as libc::c_int) as isize);
    x2 = *x.offset(-T as isize);
    x1 = *x.offset((-T + 1 as libc::c_int) as isize);
    i = 0 as libc::c_int;
    while i < N {
        x0 = *x.offset((i - T + 2 as libc::c_int) as isize);
        *y.offset(i as isize) =
            *x.offset(i as isize) + g10 * x2 + g11 * (x1 + x3) + g12 * (x0 + x4);
        *y.offset(i as isize) = *y.offset(i as isize);
        x4 = x3;
        x3 = x2;
        x2 = x1;
        x1 = x0;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn comb_filter(
    mut y: *mut crate::arch_h::opus_val32,
    mut x: *mut crate::arch_h::opus_val32,
    mut T0: libc::c_int,
    mut T1: libc::c_int,
    mut N: libc::c_int,
    mut g0: crate::arch_h::opus_val16,
    mut g1: crate::arch_h::opus_val16,
    mut tapset0: libc::c_int,
    mut tapset1: libc::c_int,
    mut window: *const crate::arch_h::opus_val16,
    mut overlap: libc::c_int,
    mut _arch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    /* printf ("%d %d %f %f\n", T0, T1, g0, g1); */
    let mut g00: crate::arch_h::opus_val16 = 0.;
    let mut g01: crate::arch_h::opus_val16 = 0.;
    let mut g02: crate::arch_h::opus_val16 = 0.;
    let mut g10: crate::arch_h::opus_val16 = 0.;
    let mut g11: crate::arch_h::opus_val16 = 0.;
    let mut g12: crate::arch_h::opus_val16 = 0.;
    let mut x0: crate::arch_h::opus_val32 = 0.;
    let mut x1: crate::arch_h::opus_val32 = 0.;
    let mut x2: crate::arch_h::opus_val32 = 0.;
    let mut x3: crate::arch_h::opus_val32 = 0.;
    let mut x4: crate::arch_h::opus_val32 = 0.;
    static mut gains: [[crate::arch_h::opus_val16; 3]; 3] = [
        [0.3066406250f32, 0.2170410156f32, 0.1296386719f32],
        [0.4638671875f32, 0.2680664062f32, 0.0f32],
        [0.7998046875f32, 0.1000976562f32, 0.0f32],
    ];
    if g0 == 0 as libc::c_int as libc::c_float && g1 == 0 as libc::c_int as libc::c_float {
        /* OPT: Happens to work without the OPUS_MOVE(), but only because the current encoder already copies x to y */
        if x != y {
            crate::stdlib::memmove(
                y as *mut libc::c_void,
                x as *const libc::c_void,
                (N as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                    )
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * y.wrapping_offset_from(x) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
        }
        return;
    }
    /* When the gain is zero, T0 and/or T1 is set to zero. We need
    to have then be at least 2 to avoid processing garbage data. */
    T0 = if T0 > 15 as libc::c_int {
        T0
    } else {
        15 as libc::c_int
    };
    T1 = if T1 > 15 as libc::c_int {
        T1
    } else {
        15 as libc::c_int
    };
    g00 = g0 * gains[tapset0 as usize][0 as libc::c_int as usize];
    g01 = g0 * gains[tapset0 as usize][1 as libc::c_int as usize];
    g02 = g0 * gains[tapset0 as usize][2 as libc::c_int as usize];
    g10 = g1 * gains[tapset1 as usize][0 as libc::c_int as usize];
    g11 = g1 * gains[tapset1 as usize][1 as libc::c_int as usize];
    g12 = g1 * gains[tapset1 as usize][2 as libc::c_int as usize];
    x1 = *x.offset((-T1 + 1 as libc::c_int) as isize);
    x2 = *x.offset(-T1 as isize);
    x3 = *x.offset((-T1 - 1 as libc::c_int) as isize);
    x4 = *x.offset((-T1 - 2 as libc::c_int) as isize);
    /* If the filter didn't change, we don't need the overlap */
    if g0 == g1 && T0 == T1 && tapset0 == tapset1 {
        overlap = 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < overlap {
        let mut f: crate::arch_h::opus_val16 = 0.;
        x0 = *x.offset((i - T1 + 2 as libc::c_int) as isize);
        f = *window.offset(i as isize) * *window.offset(i as isize);
        *y.offset(i as isize) = *x.offset(i as isize)
            + (1.0f32 - f) * g00 * *x.offset((i - T0) as isize)
            + (1.0f32 - f)
                * g01
                * (*x.offset((i - T0 + 1 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 1 as libc::c_int) as isize))
            + (1.0f32 - f)
                * g02
                * (*x.offset((i - T0 + 2 as libc::c_int) as isize)
                    + *x.offset((i - T0 - 2 as libc::c_int) as isize))
            + f * g10 * x2
            + f * g11 * (x1 + x3)
            + f * g12 * (x0 + x4);
        *y.offset(i as isize) = *y.offset(i as isize);
        x4 = x3;
        x3 = x2;
        x2 = x1;
        x1 = x0;
        i += 1
    }
    if g1 == 0 as libc::c_int as libc::c_float {
        /* OPT: Happens to work without the OPUS_MOVE(), but only because the current encoder already copies x to y */
        if x != y {
            crate::stdlib::memmove(
                y.offset(overlap as isize) as *mut libc::c_void,
                x.offset(overlap as isize) as *const libc::c_void,
                ((N - overlap) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::arch_h::opus_val32>() as libc::c_ulong
                    )
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * y.offset(overlap as isize)
                                .wrapping_offset_from(x.offset(overlap as isize))
                                as libc::c_long) as libc::c_ulong,
                    ),
            );
        }
        return;
    }
    /* Compute the part with the constant filter. */
    comb_filter_const_c(
        y.offset(i as isize),
        x.offset(i as isize),
        T1,
        N - i,
        g10,
        g11,
        g12,
    );
}
/* OVERRIDE_comb_filter */
/* TF change table. Positive values mean better frequency resolution (longer
effective window), whereas negative values mean better time resolution
(shorter effective window). The second index is computed as:
4*isTransient + 2*tf_select + per_band_flag */
#[no_mangle]

pub static mut tf_select_table: [[libc::c_schar; 8]; 4] = [
    [
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
    [
        0 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
    ],
];
#[no_mangle]

pub unsafe extern "C" fn init_caps(
    mut m: *const crate::src::opus_1_2_1::celt::modes::OpusCustomMode,
    mut cap: *mut libc::c_int,
    mut LM: libc::c_int,
    mut C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*m).nbEBands {
        let mut N: libc::c_int = 0;
        N = (*(*m).eBands.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            - *(*m).eBands.offset(i as isize) as libc::c_int)
            << LM;
        *cap.offset(i as isize) =
            (*(*m).cache.caps.offset(
                ((*m).nbEBands * (2 as libc::c_int * LM + C - 1 as libc::c_int) + i) as isize,
            ) as libc::c_int
                + 64 as libc::c_int)
                * C
                * N
                >> 2 as libc::c_int;
        i += 1
    }
}
/* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
Written by Jean-Marc Valin and Koen Vos */
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
/* *
 * @file opus_defines.h
 * @brief Opus reference implementation constants
 */
/* * @defgroup opus_errorcodes Error codes
 * @{
 */
/* * No error @hideinitializer*/
/* * One or more invalid/out of range arguments @hideinitializer*/
/* * Not enough bytes allocated in the buffer @hideinitializer*/
/* * An internal error was detected @hideinitializer*/
/* * The compressed data passed is corrupted @hideinitializer*/
/* * Invalid/unsupported request number @hideinitializer*/
/* * An encoder or decoder structure is invalid or already freed @hideinitializer*/
/* * Memory allocation has failed @hideinitializer*/
/* *@}*/
/* * @cond OPUS_INTERNAL_DOC */
/* *Export control for opus functions */
/* *Warning attributes for opus functions
 * NONNULL is not used in OPUS_BUILD to avoid the compiler optimizing out
 * some paranoid null checks. */
/* * These are the actual Encoder CTL ID numbers.
 * They should not be used directly by applications.
 * In general, SETs should be even and GETs should be odd.*/
/* #define OPUS_RESET_STATE 4028 */
/* Should have been 4035 */
/* Don't use 4045, it's already taken by OPUS_GET_GAIN_REQUEST */
/* Macros to trigger compilation errors when the wrong types are provided to a CTL */
/* * @endcond */
/* * @defgroup opus_ctlvalues Pre-defined values for CTL interface
 * @see opus_genericctls, opus_encoderctls
 * @{
 */
/* Values for the various encoder CTLs */
/* *<Auto/default setting @hideinitializer*/
/* *<Maximum bitrate @hideinitializer*/
/* * Best for most VoIP/videoconference applications where listening quality and intelligibility matter most
 * @hideinitializer */
/* * Best for broadcast/high-fidelity application where the decoded audio should be as close as possible to the input
 * @hideinitializer */
/* * Only use when lowest-achievable latency is what matters most. Voice-optimized modes cannot be used.
 * @hideinitializer */
/* *< Signal being encoded is voice */
/* *< Signal being encoded is music */
/* *< 4 kHz bandpass @hideinitializer*/
/* *< 6 kHz bandpass @hideinitializer*/
/* *< 8 kHz bandpass @hideinitializer*/
/* *<12 kHz bandpass @hideinitializer*/
/* *<20 kHz bandpass @hideinitializer*/
/* *< Select frame size from the argument (default) */
/* *< Use 2.5 ms frames */
/* *< Use 5 ms frames */
/* *< Use 10 ms frames */
/* *< Use 20 ms frames */
/* *< Use 40 ms frames */
/* *< Use 60 ms frames */
/* *< Use 80 ms frames */
/* *< Use 100 ms frames */
/* *< Use 120 ms frames */
/* *@}*/
/* * @defgroup opus_encoderctls Encoder related CTLs
 *
 * These are convenience macros for use with the \c opus_encode_ctl
 * interface. They are used to generate the appropriate series of
 * arguments for that call, passing the correct type, size and so
 * on as expected for each particular request.
 *
 * Some usage examples:
 *
 * @code
 * int ret;
 * ret = opus_encoder_ctl(enc_ctx, OPUS_SET_BANDWIDTH(OPUS_AUTO));
 * if (ret != OPUS_OK) return ret;
 *
 * opus_int32 rate;
 * opus_encoder_ctl(enc_ctx, OPUS_GET_BANDWIDTH(&rate));
 *
 * opus_encoder_ctl(enc_ctx, OPUS_RESET_STATE);
 * @endcode
 *
 * @see opus_genericctls, opus_encoder
 * @{
 */
/* * Configures the encoder's computational complexity.
 * The supported range is 0-10 inclusive with 10 representing the highest complexity.
 * @see OPUS_GET_COMPLEXITY
 * @param[in] x <tt>opus_int32</tt>: Allowed values: 0-10, inclusive.
 *
 * @hideinitializer */
/* * Gets the encoder's complexity configuration.
 * @see OPUS_SET_COMPLEXITY
 * @param[out] x <tt>opus_int32 *</tt>: Returns a value in the range 0-10,
 *                                      inclusive.
 * @hideinitializer */
/* * Configures the bitrate in the encoder.
 * Rates from 500 to 512000 bits per second are meaningful, as well as the
 * special values #OPUS_AUTO and #OPUS_BITRATE_MAX.
 * The value #OPUS_BITRATE_MAX can be used to cause the codec to use as much
 * rate as it can, which is useful for controlling the rate by adjusting the
 * output buffer size.
 * @see OPUS_GET_BITRATE
 * @param[in] x <tt>opus_int32</tt>: Bitrate in bits per second. The default
 *                                   is determined based on the number of
 *                                   channels and the input sampling rate.
 * @hideinitializer */
/* * Gets the encoder's bitrate configuration.
 * @see OPUS_SET_BITRATE
 * @param[out] x <tt>opus_int32 *</tt>: Returns the bitrate in bits per second.
 *                                      The default is determined based on the
 *                                      number of channels and the input
 *                                      sampling rate.
 * @hideinitializer */
/* * Enables or disables variable bitrate (VBR) in the encoder.
 * The configured bitrate may not be met exactly because frames must
 * be an integer number of bytes in length.
 * @see OPUS_GET_VBR
 * @see OPUS_SET_VBR_CONSTRAINT
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Hard CBR. For LPC/hybrid modes at very low bit-rate, this can
 *               cause noticeable quality degradation.</dd>
 * <dt>1</dt><dd>VBR (default). The exact type of VBR is controlled by
 *               #OPUS_SET_VBR_CONSTRAINT.</dd>
 * </dl>
 * @hideinitializer */
/* * Determine if variable bitrate (VBR) is enabled in the encoder.
 * @see OPUS_SET_VBR
 * @see OPUS_GET_VBR_CONSTRAINT
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>Hard CBR.</dd>
 * <dt>1</dt><dd>VBR (default). The exact type of VBR may be retrieved via
 *               #OPUS_GET_VBR_CONSTRAINT.</dd>
 * </dl>
 * @hideinitializer */
/* * Enables or disables constrained VBR in the encoder.
 * This setting is ignored when the encoder is in CBR mode.
 * @warning Only the MDCT mode of Opus currently heeds the constraint.
 *  Speech mode ignores it completely, hybrid mode may fail to obey it
 *  if the LPC layer uses more bitrate than the constraint would have
 *  permitted.
 * @see OPUS_GET_VBR_CONSTRAINT
 * @see OPUS_SET_VBR
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Unconstrained VBR.</dd>
 * <dt>1</dt><dd>Constrained VBR (default). This creates a maximum of one
 *               frame of buffering delay assuming a transport with a
 *               serialization speed of the nominal bitrate.</dd>
 * </dl>
 * @hideinitializer */
/* * Determine if constrained VBR is enabled in the encoder.
 * @see OPUS_SET_VBR_CONSTRAINT
 * @see OPUS_GET_VBR
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>Unconstrained VBR.</dd>
 * <dt>1</dt><dd>Constrained VBR (default).</dd>
 * </dl>
 * @hideinitializer */
/* * Configures mono/stereo forcing in the encoder.
 * This can force the encoder to produce packets encoded as either mono or
 * stereo, regardless of the format of the input audio. This is useful when
 * the caller knows that the input signal is currently a mono source embedded
 * in a stereo stream.
 * @see OPUS_GET_FORCE_CHANNELS
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>#OPUS_AUTO</dt><dd>Not forced (default)</dd>
 * <dt>1</dt>         <dd>Forced mono</dd>
 * <dt>2</dt>         <dd>Forced stereo</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's forced channel configuration.
 * @see OPUS_SET_FORCE_CHANNELS
 * @param[out] x <tt>opus_int32 *</tt>:
 * <dl>
 * <dt>#OPUS_AUTO</dt><dd>Not forced (default)</dd>
 * <dt>1</dt>         <dd>Forced mono</dd>
 * <dt>2</dt>         <dd>Forced stereo</dd>
 * </dl>
 * @hideinitializer */
/* * Configures the maximum bandpass that the encoder will select automatically.
 * Applications should normally use this instead of #OPUS_SET_BANDWIDTH
 * (leaving that set to the default, #OPUS_AUTO). This allows the
 * application to set an upper bound based on the type of input it is
 * providing, but still gives the encoder the freedom to reduce the bandpass
 * when the bitrate becomes too low, for better overall quality.
 * @see OPUS_GET_MAX_BANDWIDTH
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>OPUS_BANDWIDTH_NARROWBAND</dt>    <dd>4 kHz passband</dd>
 * <dt>OPUS_BANDWIDTH_MEDIUMBAND</dt>    <dd>6 kHz passband</dd>
 * <dt>OPUS_BANDWIDTH_WIDEBAND</dt>      <dd>8 kHz passband</dd>
 * <dt>OPUS_BANDWIDTH_SUPERWIDEBAND</dt><dd>12 kHz passband</dd>
 * <dt>OPUS_BANDWIDTH_FULLBAND</dt>     <dd>20 kHz passband (default)</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured maximum allowed bandpass.
 * @see OPUS_SET_MAX_BANDWIDTH
 * @param[out] x <tt>opus_int32 *</tt>: Allowed values:
 * <dl>
 * <dt>#OPUS_BANDWIDTH_NARROWBAND</dt>    <dd>4 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_MEDIUMBAND</dt>    <dd>6 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_WIDEBAND</dt>      <dd>8 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_SUPERWIDEBAND</dt><dd>12 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_FULLBAND</dt>     <dd>20 kHz passband (default)</dd>
 * </dl>
 * @hideinitializer */
/* * Sets the encoder's bandpass to a specific value.
 * This prevents the encoder from automatically selecting the bandpass based
 * on the available bitrate. If an application knows the bandpass of the input
 * audio it is providing, it should normally use #OPUS_SET_MAX_BANDWIDTH
 * instead, which still gives the encoder the freedom to reduce the bandpass
 * when the bitrate becomes too low, for better overall quality.
 * @see OPUS_GET_BANDWIDTH
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>#OPUS_AUTO</dt>                    <dd>(default)</dd>
 * <dt>#OPUS_BANDWIDTH_NARROWBAND</dt>    <dd>4 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_MEDIUMBAND</dt>    <dd>6 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_WIDEBAND</dt>      <dd>8 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_SUPERWIDEBAND</dt><dd>12 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_FULLBAND</dt>     <dd>20 kHz passband</dd>
 * </dl>
 * @hideinitializer */
/* * Configures the type of signal being encoded.
 * This is a hint which helps the encoder's mode selection.
 * @see OPUS_GET_SIGNAL
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>#OPUS_AUTO</dt>        <dd>(default)</dd>
 * <dt>#OPUS_SIGNAL_VOICE</dt><dd>Bias thresholds towards choosing LPC or Hybrid modes.</dd>
 * <dt>#OPUS_SIGNAL_MUSIC</dt><dd>Bias thresholds towards choosing MDCT modes.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured signal type.
 * @see OPUS_SET_SIGNAL
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>#OPUS_AUTO</dt>        <dd>(default)</dd>
 * <dt>#OPUS_SIGNAL_VOICE</dt><dd>Bias thresholds towards choosing LPC or Hybrid modes.</dd>
 * <dt>#OPUS_SIGNAL_MUSIC</dt><dd>Bias thresholds towards choosing MDCT modes.</dd>
 * </dl>
 * @hideinitializer */
/* * Configures the encoder's intended application.
 * The initial value is a mandatory argument to the encoder_create function.
 * @see OPUS_GET_APPLICATION
 * @param[in] x <tt>opus_int32</tt>: Returns one of the following values:
 * <dl>
 * <dt>#OPUS_APPLICATION_VOIP</dt>
 * <dd>Process signal for improved speech intelligibility.</dd>
 * <dt>#OPUS_APPLICATION_AUDIO</dt>
 * <dd>Favor faithfulness to the original input.</dd>
 * <dt>#OPUS_APPLICATION_RESTRICTED_LOWDELAY</dt>
 * <dd>Configure the minimum possible coding delay by disabling certain modes
 * of operation.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured application.
 * @see OPUS_SET_APPLICATION
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>#OPUS_APPLICATION_VOIP</dt>
 * <dd>Process signal for improved speech intelligibility.</dd>
 * <dt>#OPUS_APPLICATION_AUDIO</dt>
 * <dd>Favor faithfulness to the original input.</dd>
 * <dt>#OPUS_APPLICATION_RESTRICTED_LOWDELAY</dt>
 * <dd>Configure the minimum possible coding delay by disabling certain modes
 * of operation.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the total samples of delay added by the entire codec.
 * This can be queried by the encoder and then the provided number of samples can be
 * skipped on from the start of the decoder's output to provide time aligned input
 * and output. From the perspective of a decoding application the real data begins this many
 * samples late.
 *
 * The decoder contribution to this delay is identical for all decoders, but the
 * encoder portion of the delay may vary from implementation to implementation,
 * version to version, or even depend on the encoder's initial configuration.
 * Applications needing delay compensation should call this CTL rather than
 * hard-coding a value.
 * @param[out] x <tt>opus_int32 *</tt>:   Number of lookahead samples
 * @hideinitializer */
/* * Configures the encoder's use of inband forward error correction (FEC).
 * @note This is only applicable to the LPC layer
 * @see OPUS_GET_INBAND_FEC
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Disable inband FEC (default).</dd>
 * <dt>1</dt><dd>Enable inband FEC.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets encoder's configured use of inband forward error correction.
 * @see OPUS_SET_INBAND_FEC
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>Inband FEC disabled (default).</dd>
 * <dt>1</dt><dd>Inband FEC enabled.</dd>
 * </dl>
 * @hideinitializer */
/* * Configures the encoder's expected packet loss percentage.
 * Higher values trigger progressively more loss resistant behavior in the encoder
 * at the expense of quality at a given bitrate in the absence of packet loss, but
 * greater quality under loss.
 * @see OPUS_GET_PACKET_LOSS_PERC
 * @param[in] x <tt>opus_int32</tt>:   Loss percentage in the range 0-100, inclusive (default: 0).
 * @hideinitializer */
/* * Gets the encoder's configured packet loss percentage.
 * @see OPUS_SET_PACKET_LOSS_PERC
 * @param[out] x <tt>opus_int32 *</tt>: Returns the configured loss percentage
 *                                      in the range 0-100, inclusive (default: 0).
 * @hideinitializer */
/* * Configures the encoder's use of discontinuous transmission (DTX).
 * @note This is only applicable to the LPC layer
 * @see OPUS_GET_DTX
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Disable DTX (default).</dd>
 * <dt>1</dt><dd>Enabled DTX.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets encoder's configured use of discontinuous transmission.
 * @see OPUS_SET_DTX
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>DTX disabled (default).</dd>
 * <dt>1</dt><dd>DTX enabled.</dd>
 * </dl>
 * @hideinitializer */
/* * Configures the depth of signal being encoded.
 *
 * This is a hint which helps the encoder identify silence and near-silence.
 * It represents the number of significant bits of linear intensity below
 * which the signal contains ignorable quantization or other noise.
 *
 * For example, OPUS_SET_LSB_DEPTH(14) would be an appropriate setting
 * for G.711 u-law input. OPUS_SET_LSB_DEPTH(16) would be appropriate
 * for 16-bit linear pcm input with opus_encode_float().
 *
 * When using opus_encode() instead of opus_encode_float(), or when libopus
 * is compiled for fixed-point, the encoder uses the minimum of the value
 * set here and the value 16.
 *
 * @see OPUS_GET_LSB_DEPTH
 * @param[in] x <tt>opus_int32</tt>: Input precision in bits, between 8 and 24
 *                                   (default: 24).
 * @hideinitializer */
/* * Gets the encoder's configured signal depth.
 * @see OPUS_SET_LSB_DEPTH
 * @param[out] x <tt>opus_int32 *</tt>: Input precision in bits, between 8 and
 *                                      24 (default: 24).
 * @hideinitializer */
/* * Configures the encoder's use of variable duration frames.
 * When variable duration is enabled, the encoder is free to use a shorter frame
 * size than the one requested in the opus_encode*() call.
 * It is then the user's responsibility
 * to verify how much audio was encoded by checking the ToC byte of the encoded
 * packet. The part of the audio that was not encoded needs to be resent to the
 * encoder for the next call. Do not use this option unless you <b>really</b>
 * know what you are doing.
 * @see OPUS_GET_EXPERT_FRAME_DURATION
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>OPUS_FRAMESIZE_ARG</dt><dd>Select frame size from the argument (default).</dd>
 * <dt>OPUS_FRAMESIZE_2_5_MS</dt><dd>Use 2.5 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_5_MS</dt><dd>Use 5 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_10_MS</dt><dd>Use 10 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_20_MS</dt><dd>Use 20 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_40_MS</dt><dd>Use 40 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_60_MS</dt><dd>Use 60 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_80_MS</dt><dd>Use 80 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_100_MS</dt><dd>Use 100 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_120_MS</dt><dd>Use 120 ms frames.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured use of variable duration frames.
 * @see OPUS_SET_EXPERT_FRAME_DURATION
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>OPUS_FRAMESIZE_ARG</dt><dd>Select frame size from the argument (default).</dd>
 * <dt>OPUS_FRAMESIZE_2_5_MS</dt><dd>Use 2.5 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_5_MS</dt><dd>Use 5 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_10_MS</dt><dd>Use 10 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_20_MS</dt><dd>Use 20 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_40_MS</dt><dd>Use 40 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_60_MS</dt><dd>Use 60 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_80_MS</dt><dd>Use 80 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_100_MS</dt><dd>Use 100 ms frames.</dd>
 * <dt>OPUS_FRAMESIZE_120_MS</dt><dd>Use 120 ms frames.</dd>
 * </dl>
 * @hideinitializer */
/* * If set to 1, disables almost all use of prediction, making frames almost
 * completely independent. This reduces quality.
 * @see OPUS_GET_PREDICTION_DISABLED
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Enable prediction (default).</dd>
 * <dt>1</dt><dd>Disable prediction.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured prediction status.
 * @see OPUS_SET_PREDICTION_DISABLED
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>Prediction enabled (default).</dd>
 * <dt>1</dt><dd>Prediction disabled.</dd>
 * </dl>
 * @hideinitializer */
/* *@}*/
/* * @defgroup opus_genericctls Generic CTLs
 *
 * These macros are used with the \c opus_decoder_ctl and
 * \c opus_encoder_ctl calls to generate a particular
 * request.
 *
 * When called on an \c OpusDecoder they apply to that
 * particular decoder instance. When called on an
 * \c OpusEncoder they apply to the corresponding setting
 * on that encoder instance, if present.
 *
 * Some usage examples:
 *
 * @code
 * int ret;
 * opus_int32 pitch;
 * ret = opus_decoder_ctl(dec_ctx, OPUS_GET_PITCH(&pitch));
 * if (ret == OPUS_OK) return ret;
 *
 * opus_encoder_ctl(enc_ctx, OPUS_RESET_STATE);
 * opus_decoder_ctl(dec_ctx, OPUS_RESET_STATE);
 *
 * opus_int32 enc_bw, dec_bw;
 * opus_encoder_ctl(enc_ctx, OPUS_GET_BANDWIDTH(&enc_bw));
 * opus_decoder_ctl(dec_ctx, OPUS_GET_BANDWIDTH(&dec_bw));
 * if (enc_bw != dec_bw) {
 *   printf("packet bandwidth mismatch!\n");
 * }
 * @endcode
 *
 * @see opus_encoder, opus_decoder_ctl, opus_encoder_ctl, opus_decoderctls, opus_encoderctls
 * @{
 */
/* * Resets the codec state to be equivalent to a freshly initialized state.
 * This should be called when switching streams in order to prevent
 * the back to back decoding from giving different results from
 * one at a time decoding.
 * @hideinitializer */
/* * Gets the final state of the codec's entropy coder.
 * This is used for testing purposes,
 * The encoder and decoder state should be identical after coding a payload
 * (assuming no data corruption or software bugs)
 *
 * @param[out] x <tt>opus_uint32 *</tt>: Entropy coder state
 *
 * @hideinitializer */
/* * Gets the encoder's configured bandpass or the decoder's last bandpass.
 * @see OPUS_SET_BANDWIDTH
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>#OPUS_AUTO</dt>                    <dd>(default)</dd>
 * <dt>#OPUS_BANDWIDTH_NARROWBAND</dt>    <dd>4 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_MEDIUMBAND</dt>    <dd>6 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_WIDEBAND</dt>      <dd>8 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_SUPERWIDEBAND</dt><dd>12 kHz passband</dd>
 * <dt>#OPUS_BANDWIDTH_FULLBAND</dt>     <dd>20 kHz passband</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the sampling rate the encoder or decoder was initialized with.
 * This simply returns the <code>Fs</code> value passed to opus_encoder_init()
 * or opus_decoder_init().
 * @param[out] x <tt>opus_int32 *</tt>: Sampling rate of encoder or decoder.
 * @hideinitializer
 */
/* * If set to 1, disables the use of phase inversion for intensity stereo,
 * improving the quality of mono downmixes, but slightly reducing normal
 * stereo quality. Disabling phase inversion in the decoder does not comply
 * with RFC 6716, although it does not cause any interoperability issue and
 * is expected to become part of the Opus standard once RFC 6716 is updated
 * by draft-ietf-codec-opus-update.
 * @see OPUS_GET_PHASE_INVERSION_DISABLED
 * @param[in] x <tt>opus_int32</tt>: Allowed values:
 * <dl>
 * <dt>0</dt><dd>Enable phase inversion (default).</dd>
 * <dt>1</dt><dd>Disable phase inversion.</dd>
 * </dl>
 * @hideinitializer */
/* * Gets the encoder's configured phase inversion status.
 * @see OPUS_SET_PHASE_INVERSION_DISABLED
 * @param[out] x <tt>opus_int32 *</tt>: Returns one of the following values:
 * <dl>
 * <dt>0</dt><dd>Stereo phase inversion enabled (default).</dd>
 * <dt>1</dt><dd>Stereo phase inversion disabled.</dd>
 * </dl>
 * @hideinitializer */
/* *@}*/
/* * @defgroup opus_decoderctls Decoder related CTLs
 * @see opus_genericctls, opus_encoderctls, opus_decoder
 * @{
 */
/* * Configures decoder gain adjustment.
 * Scales the decoded output by a factor specified in Q8 dB units.
 * This has a maximum range of -32768 to 32767 inclusive, and returns
 * OPUS_BAD_ARG otherwise. The default is zero indicating no adjustment.
 * This setting survives decoder reset.
 *
 * gain = pow(10, x/(20.0*256))
 *
 * @param[in] x <tt>opus_int32</tt>:   Amount to scale PCM signal by in Q8 dB units.
 * @hideinitializer */
/* * Gets the decoder's configured gain adjustment. @see OPUS_SET_GAIN
 *
 * @param[out] x <tt>opus_int32 *</tt>: Amount to scale PCM signal by in Q8 dB units.
 * @hideinitializer */
/* * Gets the duration (in samples) of the last packet successfully decoded or concealed.
 * @param[out] x <tt>opus_int32 *</tt>: Number of samples (at current sampling rate).
 * @hideinitializer */
/* * Gets the pitch of the last decoded frame, if available.
 * This can be used for any post-processing algorithm requiring the use of pitch,
 * e.g. time stretching/shortening. If the last frame was not voiced, or if the
 * pitch was not coded in the frame, then zero is returned.
 *
 * This CTL is only implemented for decoder instances.
 *
 * @param[out] x <tt>opus_int32 *</tt>: pitch period at 48 kHz (or 0 if not available)
 *
 * @hideinitializer */
/* *@}*/
/* * @defgroup opus_libinfo Opus library information functions
 * @{
 */
/* * Converts an opus error code into a human readable string.
 *
 * @param[in] error <tt>int</tt>: Error number
 * @returns Error string
 */
#[no_mangle]

pub unsafe extern "C" fn opus_strerror(mut error: libc::c_int) -> *const libc::c_char {
    static mut error_strings: [*const libc::c_char; 8] = [
        b"success\x00" as *const u8 as *const libc::c_char,
        b"invalid argument\x00" as *const u8 as *const libc::c_char,
        b"buffer too small\x00" as *const u8 as *const libc::c_char,
        b"internal error\x00" as *const u8 as *const libc::c_char,
        b"corrupted stream\x00" as *const u8 as *const libc::c_char,
        b"request not implemented\x00" as *const u8 as *const libc::c_char,
        b"invalid state\x00" as *const u8 as *const libc::c_char,
        b"memory allocation failed\x00" as *const u8 as *const libc::c_char,
    ];
    if error > 0 as libc::c_int || error < -(7 as libc::c_int) {
        return b"unknown error\x00" as *const u8 as *const libc::c_char;
    } else {
        return error_strings[-error as usize];
    };
}
/* * Gets the libopus version string.
 *
 * Applications may look for the substring "-fixed" in the version string to
 * determine whether they have a fixed-point or floating-point build at
 * runtime.
 *
 * @returns Version string
 */
#[no_mangle]

pub unsafe extern "C" fn opus_get_version_string() -> *const libc::c_char {
    return b"libopus unknown\x00" as *const u8 as *const libc::c_char;
    /* Applications may rely on the presence of this substring in the version
    string to determine if they have a fixed-point or floating-point build
    at runtime. */
}

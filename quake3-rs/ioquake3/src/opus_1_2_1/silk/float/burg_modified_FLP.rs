use ::libc;

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
/* O    Voicing estimate: 0 voiced, 1 unvoiced                      */
/* I    Signal of length PE_FRAME_LENGTH_MS*Fs_kHz                  */
/* O    Pitch lag values [nb_subfr]                                 */
/* O    Lag Index                                                   */
/* O    Pitch contour Index                                         */
/* I/O  Normalized correlation; input: value from previous frame    */
/* I    Last lag of previous frame; set to zero is unvoiced         */
/* I    First stage threshold for lag candidates 0 - 1              */
/* I    Final threshold for lag candidates 0 - 1                    */
/* I    sample frequency (kHz)                                      */
/* I    Complexity setting, 0-2, where 2 is highest                 */
/* I    Number of 5 ms subframes                                    */
/* I    Run-time architecture                                       */
/* I/O  Unsorted / Sorted vector                                    */
/* O    Index vector for the sorted elements                        */
/* I    Vector length                                               */
/* I    Number of correctly sorted positions                        */
/* Compute reflection coefficients from input signal */
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
/* subfr_length * nb_subfr = ( 0.005 * 16000 + 16 ) * 4 = 384*/
/* Compute reflection coefficients from input signal */
#[no_mangle]

pub unsafe extern "C" fn silk_burg_modified_FLP(
    mut A: *mut f32,
    mut x: *const f32,
    minInvGain: f32,
    subfr_length: i32,
    nb_subfr: i32,
    D: i32,
) -> f32
/* I    order                                                       */ {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut s: i32 = 0;
    let mut reached_max_gain: i32 = 0;
    let mut C0: f64 = 0.;
    let mut invGain: f64 = 0.;
    let mut num: f64 = 0.;
    let mut nrg_f: f64 = 0.;
    let mut nrg_b: f64 = 0.;
    let mut rc: f64 = 0.;
    let mut Atmp: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut x_ptr: *const f32 = 0 as *const f32;
    let mut C_first_row: [f64; 24] = [0.; 24];
    let mut C_last_row: [f64; 24] = [0.; 24];
    let mut CAf: [f64; 25] = [0.; 25];
    let mut CAb: [f64; 25] = [0.; 25];
    let mut Af: [f64; 24] = [0.; 24];
    /* Compute autocorrelations, added over subframes */
    C0 = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
        x,
        nb_subfr * subfr_length,
    );
    crate::stdlib::memset(
        C_first_row.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        (24 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f64>() as libc::c_ulong),
    );
    s = 0 as i32;
    while s < nb_subfr {
        x_ptr = x.offset((s * subfr_length) as isize);
        n = 1 as i32;
        while n < D + 1 as i32 {
            C_first_row[(n - 1 as i32) as usize] +=
                crate::src::opus_1_2_1::silk::float::inner_product_FLP::silk_inner_product_FLP(
                    x_ptr,
                    x_ptr.offset(n as isize),
                    subfr_length - n,
                );
            n += 1
        }
        s += 1
    }
    crate::stdlib::memcpy(
        C_last_row.as_mut_ptr() as *mut libc::c_void,
        C_first_row.as_mut_ptr() as *const libc::c_void,
        (24 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f64>() as libc::c_ulong),
    );
    /* Initialize */
    CAf[0 as i32 as usize] =
        C0 + 1e-5f32 as f64 * C0 + 1e-9f32 as f64;
    CAb[0 as i32 as usize] = CAf[0 as i32 as usize];
    invGain = 1.0f32 as f64;
    reached_max_gain = 0 as i32;
    n = 0 as i32;
    while n < D {
        /* Update first row of correlation matrix (without first element) */
        /* Update last row of correlation matrix (without last element, stored in reversed order) */
        /* Update C * Af */
        /* Update C * flipud(Af) (stored in reversed order) */
        s = 0 as i32;
        while s < nb_subfr {
            x_ptr = x.offset((s * subfr_length) as isize);
            tmp1 = *x_ptr.offset(n as isize) as f64;
            tmp2 = *x_ptr.offset((subfr_length - n - 1 as i32) as isize) as f64;
            k = 0 as i32;
            while k < n {
                C_first_row[k as usize] -= (*x_ptr.offset(n as isize)
                    * *x_ptr.offset((n - k - 1 as i32) as isize))
                    as f64;
                C_last_row[k as usize] -= (*x_ptr
                    .offset((subfr_length - n - 1 as i32) as isize)
                    * *x_ptr.offset((subfr_length - n + k) as isize))
                    as f64;
                Atmp = Af[k as usize];
                tmp1 += *x_ptr.offset((n - k - 1 as i32) as isize) as f64 * Atmp;
                tmp2 += *x_ptr.offset((subfr_length - n + k) as isize) as f64 * Atmp;
                k += 1
            }
            k = 0 as i32;
            while k <= n {
                CAf[k as usize] -= tmp1 * *x_ptr.offset((n - k) as isize) as f64;
                CAb[k as usize] -= tmp2
                    * *x_ptr.offset((subfr_length - n + k - 1 as i32) as isize)
                        as f64;
                k += 1
            }
            s += 1
        }
        tmp1 = C_first_row[n as usize];
        tmp2 = C_last_row[n as usize];
        k = 0 as i32;
        while k < n {
            Atmp = Af[k as usize];
            tmp1 += C_last_row[(n - k - 1 as i32) as usize] * Atmp;
            tmp2 += C_first_row[(n - k - 1 as i32) as usize] * Atmp;
            k += 1
        }
        CAf[(n + 1 as i32) as usize] = tmp1;
        CAb[(n + 1 as i32) as usize] = tmp2;
        /* Calculate nominator and denominator for the next order reflection (parcor) coefficient */
        num = CAb[(n + 1 as i32) as usize];
        nrg_b = CAb[0 as i32 as usize];
        nrg_f = CAf[0 as i32 as usize];
        k = 0 as i32;
        while k < n {
            Atmp = Af[k as usize];
            num += CAb[(n - k) as usize] * Atmp;
            nrg_b += CAb[(k + 1 as i32) as usize] * Atmp;
            nrg_f += CAf[(k + 1 as i32) as usize] * Atmp;
            k += 1
        }
        /* Calculate the next order reflection (parcor) coefficient */
        rc = -2.0f64 * num / (nrg_f + nrg_b);
        /* Update inverse prediction gain */
        tmp1 = invGain * (1.0f64 - rc * rc);
        if tmp1 <= minInvGain as f64 {
            /* Max prediction gain exceeded; set reflection coefficient such that max prediction gain is exactly hit */
            rc = crate::stdlib::sqrt(1.0f64 - minInvGain as f64 / invGain);
            if num > 0 as i32 as f64 {
                /* Ensure adjusted reflection coefficients has the original sign */
                rc = -rc
            }
            invGain = minInvGain as f64;
            reached_max_gain = 1 as i32
        } else {
            invGain = tmp1
        }
        /* Update the AR coefficients */
        k = 0 as i32;
        while k < n + 1 as i32 >> 1 as i32 {
            tmp1 = Af[k as usize];
            tmp2 = Af[(n - k - 1 as i32) as usize];
            Af[k as usize] = tmp1 + rc * tmp2;
            Af[(n - k - 1 as i32) as usize] = tmp2 + rc * tmp1;
            k += 1
        }
        Af[n as usize] = rc;
        if reached_max_gain != 0 {
            /* Reached max prediction gain; set remaining coefficients to zero and exit loop */
            k = n + 1 as i32;
            while k < D {
                Af[k as usize] = 0.0f64;
                k += 1
            }
            break;
        } else {
            /* Update C * Af and C * Ab */
            k = 0 as i32;
            while k <= n + 1 as i32 {
                tmp1 = CAf[k as usize];
                CAf[k as usize] += rc * CAb[(n - k + 1 as i32) as usize];
                CAb[(n - k + 1 as i32) as usize] += rc * tmp1;
                k += 1
            }
            n += 1
        }
    }
    if reached_max_gain != 0 {
        /* Convert to silk_float */
        k = 0 as i32;
        while k < D {
            *A.offset(k as isize) = -Af[k as usize] as f32;
            k += 1
        }
        /* Subtract energy of preceding samples from C0 */
        s = 0 as i32;
        while s < nb_subfr {
            C0 -= crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                x.offset((s * subfr_length) as isize),
                D,
            );
            s += 1
        }
        /* Approximate residual energy */
        nrg_f = C0 * invGain
    } else {
        /* Compute residual energy and store coefficients as silk_float */
        nrg_f = CAf[0 as i32 as usize];
        tmp1 = 1.0f64;
        k = 0 as i32;
        while k < D {
            Atmp = Af[k as usize];
            nrg_f += CAf[(k + 1 as i32) as usize] * Atmp;
            tmp1 += Atmp * Atmp;
            *A.offset(k as isize) = -Atmp as f32;
            k += 1
        }
        nrg_f -= 1e-5f32 as f64 * C0 * tmp1
    }
    /* Return residual energy */
    return nrg_f as f32;
}

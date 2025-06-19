use ::libc;

pub use crate::opus_types_h::opus_int32;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int32_t;

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
/* compute inverse of LPC prediction gain, and                          */
/* test if LPC coefficients are stable (all poles within unit circle)   */
/* this code is based on silk_a2k_FLP()                                 */
#[no_mangle]

pub unsafe extern "C" fn silk_LPC_inverse_pred_gain_FLP(
    mut A: *const f32,
    mut order: opus_int32,
) -> f32
/* I    prediction order                                            */ {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut invGain: f64 = 0.;
    let mut rc: f64 = 0.;
    let mut rc_mult1: f64 = 0.;
    let mut rc_mult2: f64 = 0.;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut Atmp: [f32; 24] = [0.; 24];
    crate::stdlib::memcpy(
        Atmp.as_mut_ptr() as *mut libc::c_void,
        A as *const libc::c_void,
        (order as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize),
    );
    invGain = 1.0f64;
    k = order - 1 as i32;
    while k > 0 as i32 {
        rc = -Atmp[k as usize] as f64;
        rc_mult1 = 1.0f32 as f64 - rc * rc;
        invGain *= rc_mult1;
        if (invGain * 1e4f32 as f64) < 1.0f32 as f64 {
            return 0.0f32;
        }
        rc_mult2 = 1.0f32 as f64 / rc_mult1;
        n = 0 as i32;
        while n < k + 1 as i32 >> 1 as i32 {
            tmp1 = Atmp[n as usize] as f64;
            tmp2 = Atmp[(k - n - 1 as i32) as usize] as f64;
            Atmp[n as usize] = ((tmp1 - tmp2 * rc) * rc_mult2) as f32;
            Atmp[(k - n - 1 as i32) as usize] = ((tmp2 - tmp1 * rc) * rc_mult2) as f32;
            n += 1
        }
        k -= 1
    }
    rc = -Atmp[0 as i32 as usize] as f64;
    rc_mult1 = 1.0f32 as f64 - rc * rc;
    invGain *= rc_mult1;
    if (invGain * 1e4f32 as f64) < 1.0f32 as f64 {
        return 0.0f32;
    }
    return invGain as f32;
}

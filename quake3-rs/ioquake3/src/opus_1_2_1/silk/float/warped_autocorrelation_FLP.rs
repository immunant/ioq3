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
/* ********************/
/* Encoder Functions */
/* ********************/
/* High-pass filter with cutoff frequency adaptation based on pitch lag statistics */
/* I/O  Encoder states                              */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* Encoder main function */
/* I/O  Encoder state FLP                           */
/* O    Number of payload bytes;                    */
/* I/O  compressor data structure                   */
/* I    The type of conditional coding to use       */
/* I    If > 0: maximum number of output bits       */
/* I    Flag to force constant-bitrate operation    */
/* Initializes the Silk encoder state */
/* I/O  Encoder state FLP                           */
/* I    Run-tim architecture                        */
/* Control the Silk encoder */
/* I/O  Pointer to Silk encoder state FLP           */
/* I    Control structure                           */
/* I    Flag to allow switching audio bandwidth     */
/* I    Channel number                              */
/* *************************/
/* Noise shaping analysis */
/* *************************/
/* Compute noise shaping coefficients and initial gain values */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    LPC residual from pitch analysis            */
/* I    Input signal [frame_length + la_shape]      */
/* Autocorrelations for a warped frequency axis */
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
/* Autocorrelations for a warped frequency axis */
#[no_mangle]

pub unsafe extern "C" fn silk_warped_autocorrelation_FLP(
    mut corr: *mut f32,
    mut input: *const f32,
    warping: f32,
    length: i32,
    order: i32,
)
/* I    Correlation order (even)                    */
{
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut tmp1: f64 = 0.;
    let mut tmp2: f64 = 0.;
    let mut state: [f64; 25] = [
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut C: [f64; 25] = [
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0 as i32 as f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    /* Order must be even */
    /* Loop over samples */
    n = 0 as i32;
    while n < length {
        tmp1 = *input.offset(n as isize) as f64;
        /* Loop over allpass sections */
        i = 0 as i32;
        while i < order {
            /* Output of allpass section */
            tmp2 = state[i as usize] + warping as f64 * (state[(i + 1 as i32) as usize] - tmp1);
            state[i as usize] = tmp1;
            C[i as usize] += state[0 as i32 as usize] * tmp1;
            /* Output of allpass section */
            tmp1 = state[(i + 1 as i32) as usize]
                + warping as f64 * (state[(i + 2 as i32) as usize] - tmp2);
            state[(i + 1 as i32) as usize] = tmp2;
            C[(i + 1 as i32) as usize] += state[0 as i32 as usize] * tmp2;
            i += 2 as i32
        }
        state[order as usize] = tmp1;
        C[order as usize] += state[0 as i32 as usize] * tmp1;
        n += 1
    }
    /* Copy correlations in silk_float output format */
    i = 0 as i32;
    while i < order + 1 as i32 {
        *corr.offset(i as isize) = C[i as usize] as f32;
        i += 1
    }
}

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
/* ***********************************************/
/* LPC analysis filter                          */
/* NB! State is kept internally and the         */
/* filter always starts with zero state         */
/* first Order output samples are set to zero   */
/* ***********************************************/
/* 16th order LPC analysis filter, does not write first 16 samples */
#[inline]

unsafe extern "C" fn silk_LPC_analysis_filter16_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
)
/* I    Length of input signal                  */
{
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 16 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        /* short-term prediction */
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize)
            + *s_ptr.offset(-(10 as i32) as isize) * *PredCoef.offset(10 as i32 as isize)
            + *s_ptr.offset(-(11 as i32) as isize) * *PredCoef.offset(11 as i32 as isize)
            + *s_ptr.offset(-(12 as i32) as isize) * *PredCoef.offset(12 as i32 as isize)
            + *s_ptr.offset(-(13 as i32) as isize) * *PredCoef.offset(13 as i32 as isize)
            + *s_ptr.offset(-(14 as i32) as isize) * *PredCoef.offset(14 as i32 as isize)
            + *s_ptr.offset(-(15 as i32) as isize) * *PredCoef.offset(15 as i32 as isize);
        /* prediction error */
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1
    }
}
/* 12th order LPC analysis filter, does not write first 12 samples */
#[inline]

unsafe extern "C" fn silk_LPC_analysis_filter12_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
)
/* I    Length of input signal                  */
{
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 12 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        /* short-term prediction */
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize)
            + *s_ptr.offset(-(10 as i32) as isize) * *PredCoef.offset(10 as i32 as isize)
            + *s_ptr.offset(-(11 as i32) as isize) * *PredCoef.offset(11 as i32 as isize);
        /* prediction error */
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1
    }
}
/* 10th order LPC analysis filter, does not write first 10 samples */
#[inline]

unsafe extern "C" fn silk_LPC_analysis_filter10_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
)
/* I    Length of input signal                  */
{
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 10 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        /* short-term prediction */
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize)
            + *s_ptr.offset(-(8 as i32) as isize) * *PredCoef.offset(8 as i32 as isize)
            + *s_ptr.offset(-(9 as i32) as isize) * *PredCoef.offset(9 as i32 as isize);
        /* prediction error */
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1
    }
}
/* 8th order LPC analysis filter, does not write first 8 samples */
#[inline]

unsafe extern "C" fn silk_LPC_analysis_filter8_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
)
/* I    Length of input signal                  */
{
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 8 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        /* short-term prediction */
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize)
            + *s_ptr.offset(-(6 as i32) as isize) * *PredCoef.offset(6 as i32 as isize)
            + *s_ptr.offset(-(7 as i32) as isize) * *PredCoef.offset(7 as i32 as isize);
        /* prediction error */
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1
    }
}
/* 6th order LPC analysis filter, does not write first 6 samples */
#[inline]

unsafe extern "C" fn silk_LPC_analysis_filter6_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
)
/* I    Length of input signal                  */
{
    let mut ix: i32 = 0;
    let mut LPC_pred: f32 = 0.;
    let mut s_ptr: *const f32 = 0 as *const f32;
    ix = 6 as i32;
    while ix < length {
        s_ptr = &*s.offset((ix - 1 as i32) as isize) as *const f32;
        /* short-term prediction */
        LPC_pred = *s_ptr.offset(0 as i32 as isize) * *PredCoef.offset(0 as i32 as isize)
            + *s_ptr.offset(-(1 as i32) as isize) * *PredCoef.offset(1 as i32 as isize)
            + *s_ptr.offset(-(2 as i32) as isize) * *PredCoef.offset(2 as i32 as isize)
            + *s_ptr.offset(-(3 as i32) as isize) * *PredCoef.offset(3 as i32 as isize)
            + *s_ptr.offset(-(4 as i32) as isize) * *PredCoef.offset(4 as i32 as isize)
            + *s_ptr.offset(-(5 as i32) as isize) * *PredCoef.offset(5 as i32 as isize);
        /* prediction error */
        *r_LPC.offset(ix as isize) = *s_ptr.offset(1 as i32 as isize) - LPC_pred;
        ix += 1
    }
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
/* O    Result [order + 1]                          */
/* I    Input data to correlate                     */
/* I    Warping coefficient                         */
/* I    Length of input                             */
/* I    Correlation order (even)                    */
/* Calculation of LTP state scaling */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    The type of conditional coding to use       */
/* *********************************************/
/* Prediction Analysis                        */
/* *********************************************/
/* Find pitch lags */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* O    Residual                                    */
/* I    Speech signal                               */
/* I    Run-time architecture                       */
/* Find LPC and LTP coefficients */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    Residual from pitch analysis                */
/* I    Speech signal                               */
/* I    The type of conditional coding to use       */
/* LPC analysis */
/* I/O  Encoder state                               */
/* O    NLSFs                                       */
/* I    Input signal                                */
/* I    Prediction gain from LTP (dB)               */
/* LTP analysis */
/* O    Weight for LTP quantization         */
/* O    Weight for LTP quantization                 */
/* I    LPC residual                                */
/* I    LTP lags                                    */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* O    LTP res MAX_NB_SUBFR*(pre_lgth+subfr_lngth) */
/* I    Input signal, with preceding samples        */
/* I    LTP coefficients for each subframe          */
/* I    Pitch lags                                  */
/* I    Inverse quantization gains                  */
/* I    Length of each subframe                     */
/* I    number of subframes                         */
/* I    Preceding samples for each subframe         */
/* Calculates residual energies of input subframes where all subframes have LPC_order   */
/* of preceding samples                                                                 */
/* O    Residual energy per subframe                */
/* I    Input signal                                */
/* I    AR coefs for each frame half                */
/* I    Quantization gains                          */
/* I    Subframe length                             */
/* I    number of subframes                         */
/* I    LPC order                                   */
/* 16th order LPC analysis filter */
/* ***********************************************/
/* LPC analysis filter                          */
/* NB! State is kept internally and the         */
/* filter always starts with zero state         */
/* first Order output samples are set to zero   */
/* ***********************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_LPC_analysis_filter_FLP(
    mut r_LPC: *mut f32,
    mut PredCoef: *const f32,
    mut s: *const f32,
    length: i32,
    Order: i32,
)
/* I    LPC order                                   */
{
    match Order {
        6 => {
            silk_LPC_analysis_filter6_FLP(r_LPC, PredCoef, s, length);
        }
        8 => {
            silk_LPC_analysis_filter8_FLP(r_LPC, PredCoef, s, length);
        }
        10 => {
            silk_LPC_analysis_filter10_FLP(r_LPC, PredCoef, s, length);
        }
        12 => {
            silk_LPC_analysis_filter12_FLP(r_LPC, PredCoef, s, length);
        }
        16 => {
            silk_LPC_analysis_filter16_FLP(r_LPC, PredCoef, s, length);
        }
        _ => {}
    }
    /* Set first Order output samples to zero */
    crate::stdlib::memset(
        r_LPC as *mut libc::c_void,
        0 as i32,
        (Order as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
    );
}

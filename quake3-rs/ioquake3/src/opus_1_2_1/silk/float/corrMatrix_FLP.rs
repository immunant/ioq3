/* Calculates correlation vector X'*t */
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
/* *********************************************************************
 * Correlation matrix computations for LS estimate.
 **********************************************************************/
/* Calculates correlation vector X'*t */
#[no_mangle]

pub unsafe extern "C" fn silk_corrVector_FLP(
    mut x: *const f32,
    mut t: *const f32,
    L: i32,
    Order: i32,
    mut Xt: *mut f32,
)
/* O    X'*t correlation vector [order]             */
{
    let mut lag: i32 = 0; /* Points to first sample of column 0 of X: X[:,0] */
    let mut ptr1: *const f32 = std::ptr::null();
    ptr1 = &*x.offset((Order - 1 as i32) as isize) as *const f32;
    lag = 0 as i32;
    while lag < Order {
        /* Calculate X[:,lag]'*t */
        *Xt.offset(lag as isize) =
            crate::src::opus_1_2_1::silk::float::inner_product_FLP::silk_inner_product_FLP(
                ptr1, t, L,
            ) as f32;
        ptr1 = ptr1.offset(-1);
        lag += 1
        /* Next column of X */
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
/* O    LPC residual signal                         */
/* I    LPC coefficients                            */
/* I    Input signal                                */
/* I    Length of input signal                      */
/* I    LPC order                                   */
/* LTP tap quantizer */
/* O    Quantized LTP gains                         */
/* O    Codebook index                              */
/* O    Periodicity index                           */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain                         */
/* I    Correlation matrix                  */
/* I    Correlation vector                          */
/* I    Number of samples per subframe              */
/* I    Number of subframes                         */
/* I    Run-time architecture                       */
/* Residual energy: nrg = wxx - 2 * wXx * c + c' * wXX * c */
/* O    Weighted residual energy                    */
/* I    Filter coefficients                         */
/* I/O  Weighted correlation matrix, reg. out       */
/* I    Weighted correlation vector                 */
/* I    Weighted correlation value                  */
/* I    Dimension                                   */
/* Processing of gains */
/* I/O  Encoder state FLP                           */
/* I/O  Encoder control FLP                         */
/* I    The type of conditional coding to use       */
/* *****************/
/* Linear Algebra */
/* *****************/
/* Calculates correlation matrix X'*X */
/* Calculates correlation matrix X'*X */
#[no_mangle]

pub unsafe extern "C" fn silk_corrMatrix_FLP(
    mut x: *const f32,
    L: i32,
    Order: i32,
    mut XX: *mut f32,
)
/* O    X'*X correlation matrix [order x order]     */
{
    let mut j: i32 = 0; /* First sample of column 0 of X */
    let mut lag: i32 = 0; /* X[:,0]'*X[:,0] */
    let mut energy: f64 = 0.;
    let mut ptr1: *const f32 = std::ptr::null();
    let mut ptr2: *const f32 = std::ptr::null();
    ptr1 = &*x.offset((Order - 1 as i32) as isize) as *const f32;
    energy = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(ptr1, L);
    *XX.offset((0 as i32 * Order + 0 as i32) as isize) = energy as f32;
    j = 1 as i32;
    while j < Order {
        /* Calculate X[:,j]'*X[:,j] */
        energy += (*ptr1.offset(-j as isize) * *ptr1.offset(-j as isize)
            - *ptr1.offset((L - j) as isize) * *ptr1.offset((L - j) as isize))
            as f64; /* First sample of column 1 of X */
        *XX.offset((j * Order + j) as isize) = energy as f32;
        j += 1
    }
    ptr2 = &*x.offset((Order - 2 as i32) as isize) as *const f32;
    lag = 1 as i32;
    while lag < Order {
        /* Calculate X[:,0]'*X[:,lag] */
        energy = crate::src::opus_1_2_1::silk::float::inner_product_FLP::silk_inner_product_FLP(
            ptr1, ptr2, L,
        );
        *XX.offset((lag * Order + 0 as i32) as isize) = energy as f32;
        *XX.offset((0 as i32 * Order + lag) as isize) = energy as f32;
        /* Next column of X */
        j = 1 as i32;
        while j < Order - lag {
            energy += (*ptr1.offset(-j as isize) * *ptr2.offset(-j as isize)
                - *ptr1.offset((L - j) as isize) * *ptr2.offset((L - j) as isize))
                as f64;
            *XX.offset(((lag + j) * Order + j) as isize) = energy as f32;
            *XX.offset((j * Order + (lag + j)) as isize) = energy as f32;
            j += 1
        }
        ptr2 = ptr2.offset(-1);
        lag += 1
    }
}
/* Calculate X[:,j]'*X[:,j + lag] */

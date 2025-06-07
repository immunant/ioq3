use ::libc;

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
/* Residual energy: nrg = wxx - 2 * wXx * c + c' * wXX * c */
#[no_mangle]

pub unsafe extern "C" fn silk_residual_energy_covar_FLP(
    mut c: *const libc::c_float,
    mut wXX: *mut libc::c_float,
    mut wXx: *const libc::c_float,
    wxx: libc::c_float,
    D: libc::c_int,
) -> libc::c_float
/* I    Dimension                                   */ {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp: libc::c_float = 0.;
    let mut nrg: libc::c_float = 0.0f32;
    let mut regularization: libc::c_float = 0.;
    /* Safety checks */
    regularization = 1e-8f32
        * (*wXX.offset(0 as libc::c_int as isize)
            + *wXX.offset((D * D - 1 as libc::c_int) as isize));
    k = 0 as libc::c_int;
    while k < 10 as libc::c_int {
        nrg = wxx;
        tmp = 0.0f32;
        i = 0 as libc::c_int;
        while i < D {
            tmp += *wXx.offset(i as isize) * *c.offset(i as isize);
            i += 1
        }
        nrg -= 2.0f32 * tmp;
        /* compute c' * wXX * c, assuming wXX is symmetric */
        i = 0 as libc::c_int;
        while i < D {
            tmp = 0.0f32;
            j = i + 1 as libc::c_int;
            while j < D {
                tmp += *wXX.offset((i + D * j) as isize) * *c.offset(j as isize);
                j += 1
            }
            nrg += *c.offset(i as isize)
                * (2.0f32 * tmp + *wXX.offset((i + D * i) as isize) * *c.offset(i as isize));
            i += 1
        }
        if nrg > 0 as libc::c_int as libc::c_float {
            break;
        }
        /* Add white noise */
        i = 0 as libc::c_int;
        while i < D {
            *wXX.offset((i + D * i) as isize) += regularization;
            i += 1
        }
        /* Increase noise for next run */
        regularization *= 2.0f32;
        k += 1
    }
    if k == 10 as libc::c_int {
        nrg = 1.0f32
    }
    return nrg;
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
/* Calculates residual energies of input subframes where all subframes have LPC_order   */
/* of preceding samples                                                                 */
#[no_mangle]

pub unsafe extern "C" fn silk_residual_energy_FLP(
    mut nrgs: *mut libc::c_float,
    mut x: *const libc::c_float,
    mut a: *mut [libc::c_float; 16],
    mut gains: *const libc::c_float,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
    LPC_order: libc::c_int,
)
/* I    LPC order                                   */
{
    let mut shift: libc::c_int = 0;
    let mut LPC_res_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut LPC_res: [libc::c_float; 192] = [0.; 192];
    LPC_res_ptr = LPC_res.as_mut_ptr().offset(LPC_order as isize);
    shift = LPC_order + subfr_length;
    /* Filter input to create the LPC residual for each frame half, and measure subframe energies */
    crate::src::opus_1_2_1::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP(
        LPC_res.as_mut_ptr(),
        (*a.offset(0 as libc::c_int as isize)).as_mut_ptr() as *const libc::c_float,
        x.offset((0 as libc::c_int * shift) as isize),
        2 as libc::c_int * shift,
        LPC_order,
    );
    *nrgs.offset(0 as libc::c_int as isize) = ((*gains.offset(0 as libc::c_int as isize)
        * *gains.offset(0 as libc::c_int as isize))
        as libc::c_double
        * crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            LPC_res_ptr.offset((0 as libc::c_int * shift) as isize),
            subfr_length,
        )) as libc::c_float;
    *nrgs.offset(1 as libc::c_int as isize) = ((*gains.offset(1 as libc::c_int as isize)
        * *gains.offset(1 as libc::c_int as isize))
        as libc::c_double
        * crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            LPC_res_ptr.offset((1 as libc::c_int * shift) as isize),
            subfr_length,
        )) as libc::c_float;
    if nb_subfr == 4 as libc::c_int {
        crate::src::opus_1_2_1::silk::float::LPC_analysis_filter_FLP::silk_LPC_analysis_filter_FLP(
            LPC_res.as_mut_ptr(),
            (*a.offset(1 as libc::c_int as isize)).as_mut_ptr() as *const libc::c_float,
            x.offset((2 as libc::c_int * shift) as isize),
            2 as libc::c_int * shift,
            LPC_order,
        );
        *nrgs.offset(2 as libc::c_int as isize) = ((*gains.offset(2 as libc::c_int as isize)
            * *gains.offset(2 as libc::c_int as isize))
            as libc::c_double
            * crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                LPC_res_ptr.offset((0 as libc::c_int * shift) as isize),
                subfr_length,
            )) as libc::c_float;
        *nrgs.offset(3 as libc::c_int as isize) = ((*gains.offset(3 as libc::c_int as isize)
            * *gains.offset(3 as libc::c_int as isize))
            as libc::c_double
            * crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
                LPC_res_ptr.offset((1 as libc::c_int * shift) as isize),
                subfr_length,
            )) as libc::c_float
    };
}

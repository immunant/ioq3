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
#[no_mangle]

pub unsafe extern "C" fn silk_find_LTP_FLP(
    mut XX: *mut libc::c_float,
    mut xX: *mut libc::c_float,
    mut r_ptr: *const libc::c_float,
    mut lag: *const libc::c_int,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
)
/* I    number of subframes                         */
{
    let mut k: libc::c_int = 0;
    let mut xX_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut XX_ptr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut lag_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut xx: libc::c_float = 0.;
    let mut temp: libc::c_float = 0.;
    xX_ptr = xX;
    XX_ptr = XX;
    k = 0 as libc::c_int;
    while k < nb_subfr {
        lag_ptr = r_ptr
            .offset(-((*lag.offset(k as isize) + 5 as libc::c_int / 2 as libc::c_int) as isize));
        crate::src::opus_1_2_1::silk::float::corrMatrix_FLP::silk_corrMatrix_FLP(
            lag_ptr,
            subfr_length,
            5 as libc::c_int,
            XX_ptr,
        );
        crate::src::opus_1_2_1::silk::float::corrMatrix_FLP::silk_corrVector_FLP(
            lag_ptr,
            r_ptr,
            subfr_length,
            5 as libc::c_int,
            xX_ptr,
        );
        xx = crate::src::opus_1_2_1::silk::float::energy_FLP::silk_energy_FLP(
            r_ptr,
            subfr_length + 5 as libc::c_int,
        ) as libc::c_float;
        temp = 1.0f32
            / (if xx
                > 0.03f32
                    * 0.5f32
                    * (*XX_ptr.offset(0 as libc::c_int as isize)
                        + *XX_ptr.offset(24 as libc::c_int as isize))
                    + 1.0f32
            {
                xx
            } else {
                (0.03f32
                    * 0.5f32
                    * (*XX_ptr.offset(0 as libc::c_int as isize)
                        + *XX_ptr.offset(24 as libc::c_int as isize)))
                    + 1.0f32
            });
        crate::src::opus_1_2_1::silk::float::scale_vector_FLP::silk_scale_vector_FLP(
            XX_ptr,
            temp,
            5 as libc::c_int * 5 as libc::c_int,
        );
        crate::src::opus_1_2_1::silk::float::scale_vector_FLP::silk_scale_vector_FLP(
            xX_ptr,
            temp,
            5 as libc::c_int,
        );
        r_ptr = r_ptr.offset(subfr_length as isize);
        XX_ptr = XX_ptr.offset((5 as libc::c_int * 5 as libc::c_int) as isize);
        xX_ptr = xX_ptr.offset(5 as libc::c_int as isize);
        k += 1
    }
}

use ::libc;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;

pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::SideInfoIndices;
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
/* High-pass filter with cutoff frequency adaptation based on pitch lag statistics */
#[no_mangle]

pub unsafe extern "C" fn silk_HP_variable_cutoff(
    mut state_Fxx: *mut crate::structs_FLP_h::silk_encoder_state_FLP,
)
/* I/O  Encoder states                              */
{
    let mut quality_Q15: libc::c_int = 0;
    let mut pitch_freq_Hz_Q16: crate::opus_types_h::opus_int32 = 0;
    let mut pitch_freq_log_Q7: crate::opus_types_h::opus_int32 = 0;
    let mut delta_freq_Q7: crate::opus_types_h::opus_int32 = 0;
    let mut psEncC1: *mut crate::structs_h::silk_encoder_state =
        &mut (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn;
    /* Adaptive cutoff frequency: estimate low end of pitch frequency range */
    if (*psEncC1).prevSignalType as libc::c_int == 2 as libc::c_int {
        /* difference, in log domain */
        pitch_freq_Hz_Q16 = ((((*psEncC1).fs_kHz * 1000 as libc::c_int)
            as crate::opus_types_h::opus_uint32)
            << 16 as libc::c_int) as crate::opus_types_h::opus_int32
            / (*psEncC1).prevLag;
        pitch_freq_log_Q7 = crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(pitch_freq_Hz_Q16)
            - ((16 as libc::c_int) << 7 as libc::c_int);
        /* adjustment based on quality */
        quality_Q15 = (*psEncC1).input_quality_bands_Q15[0 as libc::c_int as usize];
        pitch_freq_log_Q7 = (pitch_freq_log_Q7 as libc::c_longlong
            + ((((-quality_Q15 as crate::opus_types_h::opus_uint32) << 2 as libc::c_int)
                as crate::opus_types_h::opus_int32 as libc::c_longlong
                * quality_Q15 as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int) as crate::opus_types_h::opus_int32
                as libc::c_longlong
                * (pitch_freq_log_Q7
                    - (crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(
                        ((60 as libc::c_int as libc::c_longlong
                            * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int))
                            as libc::c_double
                            + 0.5f64) as crate::opus_types_h::opus_int32,
                    ) - ((16 as libc::c_int) << 7 as libc::c_int)))
                    as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int))
            as crate::opus_types_h::opus_int32;
        /* delta_freq = pitch_freq_log - psEnc->variable_HP_smth1; */
        delta_freq_Q7 = pitch_freq_log_Q7 - ((*psEncC1).variable_HP_smth1_Q15 >> 8 as libc::c_int);
        if delta_freq_Q7 < 0 as libc::c_int {
            /* less smoothing for decreasing pitch frequency, to track something close to the minimum */
            delta_freq_Q7 = delta_freq_Q7 * 3 as libc::c_int
        }
        /* limit delta, to reduce impact of outliers in pitch estimation */
        delta_freq_Q7 = if -(((0.4f32
            * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32)
            > ((0.4f32
                * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        {
            if delta_freq_Q7
                > -(((0.4f32
                    * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32)
            {
                -(((0.4f32
                    * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32)
            } else if delta_freq_Q7
                < ((0.4f32
                    * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
            {
                ((0.4f32
                    * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
            } else {
                delta_freq_Q7
            }
        } else if delta_freq_Q7
            > ((0.4f32
                * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        {
            ((0.4f32
                * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32
        } else if delta_freq_Q7
            < -(((0.4f32
                * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32)
        {
            -(((0.4f32
                * ((1 as libc::c_int as libc::c_longlong) << 7 as libc::c_int) as libc::c_float)
                as libc::c_double
                + 0.5f64) as crate::opus_types_h::opus_int32)
        } else {
            delta_freq_Q7
        };
        /* update smoother */
        (*psEncC1).variable_HP_smth1_Q15 = ((*psEncC1).variable_HP_smth1_Q15 as libc::c_longlong
            + (((*psEncC1).speech_activity_Q8 as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * delta_freq_Q7 as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32) as libc::c_longlong
                * ((0.1f32
                    * ((1 as libc::c_int as libc::c_longlong) << 16 as libc::c_int)
                        as libc::c_float) as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
                    as crate::opus_types_h::opus_int16 as libc::c_longlong
                >> 16 as libc::c_int))
            as crate::opus_types_h::opus_int32;
        /* limit frequency range */
        (*psEncC1).variable_HP_smth1_Q15 =
            if ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                as crate::opus_types_h::opus_uint32)
                << 8 as libc::c_int) as crate::opus_types_h::opus_int32
                > ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(100 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            {
                if (*psEncC1).variable_HP_smth1_Q15
                    > ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                        as crate::opus_types_h::opus_uint32)
                        << 8 as libc::c_int)
                        as crate::opus_types_h::opus_int32
                {
                    ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                        as crate::opus_types_h::opus_uint32)
                        << 8 as libc::c_int) as crate::opus_types_h::opus_int32
                } else if (*psEncC1).variable_HP_smth1_Q15
                    < ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(100 as libc::c_int)
                        as crate::opus_types_h::opus_uint32)
                        << 8 as libc::c_int)
                        as crate::opus_types_h::opus_int32
                {
                    ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(100 as libc::c_int)
                        as crate::opus_types_h::opus_uint32)
                        << 8 as libc::c_int) as crate::opus_types_h::opus_int32
                } else {
                    (*psEncC1).variable_HP_smth1_Q15
                }
            } else if (*psEncC1).variable_HP_smth1_Q15
                > ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(100 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            {
                ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(100 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            } else if (*psEncC1).variable_HP_smth1_Q15
                < ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            {
                ((crate::src::opus_1_2_1::silk::lin2log::silk_lin2log(60 as libc::c_int)
                    as crate::opus_types_h::opus_uint32)
                    << 8 as libc::c_int) as crate::opus_types_h::opus_int32
            } else {
                (*psEncC1).variable_HP_smth1_Q15
            }
    };
}

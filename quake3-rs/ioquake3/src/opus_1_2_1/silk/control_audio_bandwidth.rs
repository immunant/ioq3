use ::libc;

pub use crate::control_h::silk_EncControlStruct;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;

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
/* Convert Left/Right stereo signal to adaptive Mid/Side representation */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* O    Quantization indices                        */
/* O    Flag: only mid signal coded                 */
/* O    Bitrates for mid and side signals           */
/* I    Total bitrate                               */
/* I    Speech activity level in previous frame     */
/* I    Last frame before a stereo->mono transition */
/* I    Sample rate (kHz)                           */
/* I    Number of samples                           */
/* Convert adaptive Mid/Side representation to Left/Right stereo signal */
/* I/O  State                                       */
/* I/O  Left input signal, becomes mid signal       */
/* I/O  Right input signal, becomes side signal     */
/* I    Predictors                                  */
/* I    Samples rate (kHz)                          */
/* I    Number of samples                           */
/* Find least-squares prediction gain for one signal based on another and quantize it */
/* O    Returns predictor in Q13                    */
/* O    Ratio of residual and mid energies          */
/* I    Basis signal                                */
/* I    Target signal                               */
/* I/O  Smoothed mid, residual norms                */
/* I    Number of samples                           */
/* I    Smoothing coefficient                       */
/* Quantize mid/side predictors */
/* I/O  Predictors (out: quantized)                 */
/* O    Quantization indices                        */
/* Entropy code the mid/side quantization indices */
/* I/O  Compressor data structure                   */
/* I    Quantization indices                        */
/* Entropy code the mid-only flag */
/* I/O  Compressor data structure                   */
/* Decode mid/side predictors */
/* I/O  Compressor data structure                   */
/* O    Predictors                                  */
/* Decode mid-only flag */
/* I/O  Compressor data structure                   */
/* O    Flag that only mid channel has been coded   */
/* Encodes signs of excitation */
/* I/O  Compressor data structure               */
/* I    pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Decodes signs of excitation */
/* I/O  Compressor data structure               */
/* I/O  pulse signal                            */
/* I    length of input                         */
/* I    Signal type                             */
/* I    Quantization offset type                */
/* I    Sum of absolute pulses per block        */
/* Check encoder control struct */
/* I    Control structure                           */
/* Control internal sampling rate */
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
/* Control internal sampling rate */
#[no_mangle]

pub unsafe extern "C" fn silk_control_audio_bandwidth(
    mut psEncC: *mut crate::structs_h::silk_encoder_state,
    mut encControl: *mut crate::control_h::silk_EncControlStruct,
) -> libc::c_int
/* I    Control structure                           */ {
    let mut fs_kHz: libc::c_int = 0;
    let mut fs_Hz: crate::opus_types_h::opus_int32 = 0;
    fs_kHz = (*psEncC).fs_kHz;
    fs_Hz = fs_kHz as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32
        * 1000 as libc::c_int as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
    if fs_Hz == 0 as libc::c_int {
        /* Encoder has just been initialized */
        fs_Hz = if (*psEncC).desiredInternal_fs_Hz < (*psEncC).API_fs_Hz {
            (*psEncC).desiredInternal_fs_Hz
        } else {
            (*psEncC).API_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int
    } else if fs_Hz > (*psEncC).API_fs_Hz
        || fs_Hz > (*psEncC).maxInternal_fs_Hz
        || fs_Hz < (*psEncC).minInternal_fs_Hz
    {
        /* Make sure internal rate is not higher than external rate or maximum allowed, or lower than minimum allowed */
        fs_Hz = (*psEncC).API_fs_Hz;
        fs_Hz = if fs_Hz < (*psEncC).maxInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).maxInternal_fs_Hz
        };
        fs_Hz = if fs_Hz > (*psEncC).minInternal_fs_Hz {
            fs_Hz
        } else {
            (*psEncC).minInternal_fs_Hz
        };
        fs_kHz = fs_Hz / 1000 as libc::c_int
    } else {
        /* State machine for the internal sampling rate switching */
        if (*psEncC).sLP.transition_frame_no
            >= 5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int)
        {
            /* Stop transition phase */
            (*psEncC).sLP.mode = 0 as libc::c_int
        }
        if (*psEncC).allow_bandwidth_switch != 0 || (*encControl).opusCanSwitch != 0 {
            /* Check if we should switch down */
            if (*psEncC).fs_kHz as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * 1000 as libc::c_int as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
                > (*psEncC).desiredInternal_fs_Hz
            {
                /* Switch down */
                if (*psEncC).sLP.mode == 0 as libc::c_int {
                    /* New transition */
                    (*psEncC).sLP.transition_frame_no =
                        5120 as libc::c_int / (5 as libc::c_int * 4 as libc::c_int);
                    /* Reset transition filter state */
                    crate::stdlib::memset(
                        (*psEncC).sLP.In_LP_State.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>()
                            as libc::c_ulong,
                    );
                }
                if (*encControl).opusCanSwitch != 0 {
                    /* Stop transition phase */
                    (*psEncC).sLP.mode = 0 as libc::c_int;
                    /* Switch to a lower sample frequency */
                    fs_kHz = if (*psEncC).fs_kHz == 16 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        8 as libc::c_int
                    }
                } else if (*psEncC).sLP.transition_frame_no <= 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    /* Make room for redundancy */
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int)
                } else {
                    /* Direction: down (at double speed) */
                    (*psEncC).sLP.mode = -(2 as libc::c_int)
                }
            } else if ((*psEncC).fs_kHz as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * 1000 as libc::c_int as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32)
                < (*psEncC).desiredInternal_fs_Hz
            {
                /* Check if we should switch up */
                /* Switch up */
                if (*encControl).opusCanSwitch != 0 {
                    /* Switch to a higher sample frequency */
                    fs_kHz = if (*psEncC).fs_kHz == 8 as libc::c_int {
                        12 as libc::c_int
                    } else {
                        16 as libc::c_int
                    };
                    /* New transition */
                    (*psEncC).sLP.transition_frame_no = 0 as libc::c_int;
                    /* Reset transition filter state */
                    crate::stdlib::memset(
                        (*psEncC).sLP.In_LP_State.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>()
                            as libc::c_ulong,
                    );
                    /* Direction: up */
                    (*psEncC).sLP.mode = 1 as libc::c_int
                } else if (*psEncC).sLP.mode == 0 as libc::c_int {
                    (*encControl).switchReady = 1 as libc::c_int;
                    /* Make room for redundancy */
                    (*encControl).maxBits -= (*encControl).maxBits * 5 as libc::c_int
                        / ((*encControl).payloadSize_ms + 5 as libc::c_int)
                } else {
                    /* Direction: up */
                    (*psEncC).sLP.mode = 1 as libc::c_int
                }
            } else if (*psEncC).sLP.mode < 0 as libc::c_int {
                (*psEncC).sLP.mode = 1 as libc::c_int
            }
        }
    }
    return fs_kHz;
}

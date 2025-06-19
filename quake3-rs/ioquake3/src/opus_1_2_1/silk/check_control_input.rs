pub use crate::control_h::silk_EncControlStruct;
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
/* Check encoder control struct */
#[no_mangle]

pub unsafe extern "C" fn check_control_input(mut encControl: *mut silk_EncControlStruct) -> i32
/* I    Control structure                           */ {
    if (*encControl).API_sampleRate != 8000 as i32
        && (*encControl).API_sampleRate != 12000 as i32
        && (*encControl).API_sampleRate != 16000 as i32
        && (*encControl).API_sampleRate != 24000 as i32
        && (*encControl).API_sampleRate != 32000 as i32
        && (*encControl).API_sampleRate != 44100 as i32
        && (*encControl).API_sampleRate != 48000 as i32
        || (*encControl).desiredInternalSampleRate != 8000 as i32
            && (*encControl).desiredInternalSampleRate != 12000 as i32
            && (*encControl).desiredInternalSampleRate != 16000 as i32
        || (*encControl).maxInternalSampleRate != 8000 as i32
            && (*encControl).maxInternalSampleRate != 12000 as i32
            && (*encControl).maxInternalSampleRate != 16000 as i32
        || (*encControl).minInternalSampleRate != 8000 as i32
            && (*encControl).minInternalSampleRate != 12000 as i32
            && (*encControl).minInternalSampleRate != 16000 as i32
        || (*encControl).minInternalSampleRate > (*encControl).desiredInternalSampleRate
        || (*encControl).maxInternalSampleRate < (*encControl).desiredInternalSampleRate
        || (*encControl).minInternalSampleRate > (*encControl).maxInternalSampleRate
    {
        return -(102 as i32);
    }
    if (*encControl).payloadSize_ms != 10 as i32
        && (*encControl).payloadSize_ms != 20 as i32
        && (*encControl).payloadSize_ms != 40 as i32
        && (*encControl).payloadSize_ms != 60 as i32
    {
        return -(103 as i32);
    }
    if (*encControl).packetLossPercentage < 0 as i32
        || (*encControl).packetLossPercentage > 100 as i32
    {
        return -(105 as i32);
    }
    if (*encControl).useDTX < 0 as i32 || (*encControl).useDTX > 1 as i32 {
        return -(108 as i32);
    }
    if (*encControl).useCBR < 0 as i32 || (*encControl).useCBR > 1 as i32 {
        return -(109 as i32);
    }
    if (*encControl).useInBandFEC < 0 as i32 || (*encControl).useInBandFEC > 1 as i32 {
        return -(107 as i32);
    }
    if (*encControl).nChannelsAPI < 1 as i32 || (*encControl).nChannelsAPI > 2 as i32 {
        return -(111 as i32);
    }
    if (*encControl).nChannelsInternal < 1 as i32 || (*encControl).nChannelsInternal > 2 as i32 {
        return -(111 as i32);
    }
    if (*encControl).nChannelsInternal > (*encControl).nChannelsAPI {
        return -(111 as i32);
    }
    if (*encControl).complexity < 0 as i32 || (*encControl).complexity > 10 as i32 {
        return -(106 as i32);
    }
    return 0 as i32;
}

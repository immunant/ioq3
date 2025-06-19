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
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_dec;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::structs_h::silk_CNG_struct;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_PLC_struct;
pub use crate::structs_h::silk_decoder_state;
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
/* I/O  Pointer to Silk encoder state               */
/* I    Control structure                           */
/* Control SNR of redidual quantizer */
/* I/O  Pointer to Silk encoder state               */
/* I    Target max bitrate (bps)                    */
/* **************/
/* Shell coder */
/* **************/
/* Encode quantization indices of excitation */
/* I/O  compressor data structure                   */
/* I    Signal type                                 */
/* I    quantOffsetType                             */
/* I    quantization indices                        */
/* I    Frame length                                */
/* Shell encoder, operates on one shell code frame of 16 pulses */
/* I/O  compressor data structure                   */
/* I    data: nonnegative pulse amplitudes          */
/* Shell decoder, operates on one shell code frame of 16 pulses */
/* O    data: nonnegative pulse amplitudes          */
/* I/O  Compressor data structure                   */
/* I    number of pulses per pulse-subframe         */
/* Gain scalar quantization with hysteresis, uniform on log scale */
/* O    gain indices                                */
/* I/O  gains (quantized out)                       */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                         */
/* Gains scalar dequantization, uniform on log scale */
/* O    quantized gains                             */
/* I    gain indices                                */
/* I/O  last index in previous frame                */
/* I    first gain is delta coded if 1              */
/* I    number of subframes                          */
/* Compute unique identifier of gain indices vector */
/* O    returns unique identifier of gains          */
/* I    gain indices                                */
/* I    number of subframes                         */
/* Interpolate two vectors */
/* O    interpolated vector                         */
/* I    first vector                                */
/* I    second vector                               */
/* I    interp. factor, weight on 2nd vector        */
/* I    number of parameters                        */
/* LTP tap quantizer */
/* O    Quantized LTP gains             */
/* O    Codebook Index                  */
/* O    Periodicity Index               */
/* I/O  Cumulative max prediction gain  */
/* O    LTP prediction gain             */
/* I    Correlation matrix in Q18       */
/* I    Correlation vector in Q18       */
/* I    Number of samples per subframe  */
/* I    Number of subframes             */
/* I    Run-time architecture           */
/* Entropy constrained matrix-weighted VQ, for a single input data vector */
/* O    index of best codebook vector               */
/* O    best residual energy                        */
/* O    best total bitrate                          */
/* O    sum of absolute LTP coefficients            */
/* I    correlation matrix                          */
/* I    correlation vector                          */
/* I    codebook                                    */
/* I    codebook effective gain                     */
/* I    code length for each codebook vector        */
/* I    number of samples per subframe              */
/* I    maximum sum of absolute LTP coefficients    */
/* I    number of vectors in codebook               */
/* ***********************************/
/* Noise shaping quantization (NSQ) */
/* ***********************************/
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* Noise shaping using delayed decision */
/* I    Encoder State                   */
/* I/O  NSQ state                       */
/* I/O  Quantization Indices            */
/* I    Input                           */
/* O    Quantized pulse signal          */
/* I    Short term prediction coefs     */
/* I    Long term prediction coefs      */
/* I  Noise shaping coefs             */
/* I    Long term shaping coefs         */
/* I    Spectral tilt                   */
/* I    Low frequency shaping coefs     */
/* I    Quantization step sizes         */
/* I    Pitch lags                      */
/* I    Rate/distortion tradeoff        */
/* I    LTP state scaling               */
/* ***********/
/* Silk VAD */
/* ***********/
/* Initialize the Silk VAD */
/* O    Return value, 0 if success                  */
/* I/O  Pointer to Silk VAD state                   */
/* Get speech activity level in Q8 */
/* O    Return value, 0 if success                  */
/* I/O  Encoder state                               */
/* I    PCM input                                   */
/* Low-pass filter with variable cutoff frequency based on  */
/* piece-wise linear interpolation between elliptic filters */
/* Start by setting transition_frame_no = 1;                */
/* I/O  LP filter state                             */
/* I/O  Low-pass filtered output signal             */
/* I    Frame length                                */
/* *****************/
/* NLSF Quantizer */
/* *****************/
/* Limit, stabilize, convert and quantize NLSFs */
/* I/O  Encoder state                               */
/* O    Prediction coefficients                     */
/* I/O  Normalized LSFs (quant out) (0 - (2^15-1))  */
/* I    Previous Normalized LSFs (0 - (2^15-1))     */
/* O    Returns RD value in Q25                     */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I/O  Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook object                             */
/* I    NLSF weight vector [ LPC_ORDER ]            */
/* I    Rate weight for the RD optimization         */
/* I    Max survivors after first stage             */
/* I    Signal type: 0/1/2                          */
/* Compute quantization errors for an LPC_order element input vector for a VQ codebook */
/* O    Quantization errors [K]                     */
/* I    Input vectors to be quantized [LPC_order]   */
/* I    Codebook vectors [K*LPC_order]              */
/* I    Codebook weights [K*LPC_order]              */
/* I    Number of codebook vectors                  */
/* I    Number of LPCs                              */
/* Delayed-decision quantizer for NLSF residuals */
/* O    Returns RD value in Q25                     */
/* O    Quantization indices [ order ]              */
/* I    Input [ order ]                             */
/* I    Weights [ order ]                           */
/* I    Backward predictor coefs [ order ]          */
/* I    Indices to entropy coding tables [ order ]  */
/* I    Rates []                                    */
/* I    Quantization step size                      */
/* I    Inverse quantization step size              */
/* I    R/D tradeoff                                */
/* I    Number of input values                      */
/* Unpack predictor values and indices for entropy coding tables */
/* O    Indices to entropy tables [ LPC_ORDER ]     */
/* O    LSF predictor [ LPC_ORDER ]                 */
/* I    Codebook object                             */
/* I    Index of vector in first LSF codebook       */
/* **********************/
/* NLSF vector decoder */
/* **********************/
/* O    Quantized NLSF vector [ LPC_ORDER ]         */
/* I    Codebook path vector [ LPC_ORDER + 1 ]      */
/* I    Codebook object                             */
/* ***************************************************/
/* Decoder Functions                                */
/* ***************************************************/
/* I/O  Decoder state pointer                       */
/* Set decoder sampling rate */
/* I/O  Decoder state pointer                       */
/* I    Sampling frequency (kHz)                    */
/* I    API Sampling frequency (Hz)                 */
/* ***************/
/* Decode frame */
/* ***************/
/* I/O  Pointer to Silk decoder state               */
/* I/O  Compressor data structure                   */
/* O    Pointer to output speech frame              */
/* O    Pointer to size of output frame             */
/* I    0: no loss, 1 loss, 2 decode fec            */
/* I    The type of conditional coding to use       */
/* I    Run-time architecture                       */
/* Decode indices from bitstream */
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
/* Decode side-information parameters from payload */
#[no_mangle]

pub unsafe extern "C" fn silk_decode_indices(
    mut psDec: *mut crate::structs_h::silk_decoder_state,
    mut psRangeDec: *mut crate::src::opus_1_2_1::celt::entcode::ec_dec,
    mut FrameIndex: i32,
    mut decode_LBRR: i32,
    mut condCoding: i32,
)
/* I    The type of conditional coding to use       */
{
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut Ix: i32 = 0;
    let mut decode_absolute_lagIndex: i32 = 0;
    let mut delta_lagIndex: i32 = 0;
    let mut ec_ix: [crate::opus_types_h::opus_int16; 16] = [0; 16];
    let mut pred_Q8: [u8; 16] = [0; 16];
    /* ******************************************/
    /* Decode signal type and quantizer offset */
    /* ******************************************/
    if decode_LBRR != 0 || (*psDec).VAD_flags[FrameIndex as usize] != 0 {
        Ix = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            crate::src::opus_1_2_1::silk::tables_other::silk_type_offset_VAD_iCDF.as_ptr(),
            8 as i32 as u32,
        ) + 2 as i32
    } else {
        Ix = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            crate::src::opus_1_2_1::silk::tables_other::silk_type_offset_no_VAD_iCDF.as_ptr(),
            8 as i32 as u32,
        )
    }
    (*psDec).indices.signalType = (Ix >> 1 as i32) as i8;
    (*psDec).indices.quantOffsetType = (Ix & 1 as i32) as i8;
    /* ***************/
    /* Decode gains */
    /* ***************/
    /* First subframe */
    if condCoding == 2 as i32 {
        /* Conditional coding */
        (*psDec).indices.GainsIndices[0 as i32 as usize] =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_gain::silk_delta_gain_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as i8
    } else {
        /* Independent coding, in two stages: MSB bits followed by 3 LSBs */
        (*psDec).indices.GainsIndices[0 as i32 as usize] =
            ((crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_gain::silk_gain_iCDF
                    [(*psDec).indices.signalType as usize]
                    .as_ptr(),
                8 as i32 as u32,
            ) as crate::opus_types_h::opus_uint32)
                << 3 as i32) as crate::opus_types_h::opus_int32 as i8;
        (*psDec).indices.GainsIndices[0 as i32 as usize] = ((*psDec).indices.GainsIndices
            [0 as i32 as usize] as i32
            + crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_uniform8_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as i8 as i32) as i8
    }
    /* Remaining subframes */
    i = 1 as i32;
    while i < (*psDec).nb_subfr {
        (*psDec).indices.GainsIndices[i as usize] =
            crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_gain::silk_delta_gain_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as i8;
        i += 1
    }
    /* *********************/
    /* Decode LSF Indices */
    /* *********************/
    (*psDec).indices.NLSFIndices[0 as i32 as usize] =
        crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            &*(*(*psDec).psNLSF_CB).CB1_iCDF.offset(
                (((*psDec).indices.signalType as i32 >> 1 as i32)
                    * (*(*psDec).psNLSF_CB).nVectors as i32) as isize,
            ),
            8 as i32 as u32,
        ) as i8;
    crate::src::opus_1_2_1::silk::NLSF_unpack::silk_NLSF_unpack(
        ec_ix.as_mut_ptr(),
        pred_Q8.as_mut_ptr(),
        (*psDec).psNLSF_CB as *const crate::structs_h::silk_NLSF_CB_struct,
        (*psDec).indices.NLSFIndices[0 as i32 as usize] as i32,
    );
    i = 0 as i32;
    while i < (*(*psDec).psNLSF_CB).order as i32 {
        Ix = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            &*(*(*psDec).psNLSF_CB)
                .ec_iCDF
                .offset(*ec_ix.as_mut_ptr().offset(i as isize) as isize),
            8 as i32 as u32,
        );
        if Ix == 0 as i32 {
            Ix -= crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_NLSF_EXT_iCDF.as_ptr(),
                8 as i32 as u32,
            )
        } else if Ix == 2 as i32 * 4 as i32 {
            Ix += crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_NLSF_EXT_iCDF.as_ptr(),
                8 as i32 as u32,
            )
        }
        (*psDec).indices.NLSFIndices[(i + 1 as i32) as usize] = (Ix - 4 as i32) as i8;
        i += 1
    }
    /* Decode LSF interpolation factor */
    if (*psDec).nb_subfr == 4 as i32 {
        (*psDec).indices.NLSFInterpCoef_Q2 = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            crate::src::opus_1_2_1::silk::tables_other::silk_NLSF_interpolation_factor_iCDF
                .as_ptr(),
            8 as i32 as u32,
        ) as i8
    } else {
        (*psDec).indices.NLSFInterpCoef_Q2 = 4 as i32 as i8
    }
    if (*psDec).indices.signalType as i32 == 2 as i32 {
        /* ********************/
        /* Decode pitch lags */
        /* ********************/
        /* Get lag index */
        decode_absolute_lagIndex = 1 as i32;
        if condCoding == 2 as i32 && (*psDec).ec_prevSignalType == 2 as i32 {
            /* Decode Delta index */
            delta_lagIndex = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_delta_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as crate::opus_types_h::opus_int16 as i32;
            if delta_lagIndex > 0 as i32 {
                delta_lagIndex = delta_lagIndex - 9 as i32;
                (*psDec).indices.lagIndex = ((*psDec).ec_prevLagIndex as i32 + delta_lagIndex)
                    as crate::opus_types_h::opus_int16;
                decode_absolute_lagIndex = 0 as i32
            }
        }
        if decode_absolute_lagIndex != 0 {
            /* Absolute decoding */
            (*psDec).indices.lagIndex = (crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_pitch_lag::silk_pitch_lag_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as crate::opus_types_h::opus_int16 as i32
                * ((*psDec).fs_kHz >> 1 as i32))
                as crate::opus_types_h::opus_int16;
            (*psDec).indices.lagIndex = ((*psDec).indices.lagIndex as i32
                + crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                    psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (*psDec).pitch_lag_low_bits_iCDF,
                    8 as i32 as u32,
                ) as crate::opus_types_h::opus_int16 as i32)
                as crate::opus_types_h::opus_int16
        }
        (*psDec).ec_prevLagIndex = (*psDec).indices.lagIndex;
        /* Get countour index */
        (*psDec).indices.contourIndex = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            (*psDec).pitch_contour_iCDF,
            8 as i32 as u32,
        ) as i8;
        /* *******************/
        /* Decode LTP gains */
        /* *******************/
        /* Decode PERIndex value */
        (*psDec).indices.PERIndex = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
            psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
            crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_per_index_iCDF.as_ptr(),
            8 as i32 as u32,
        ) as i8;
        k = 0 as i32;
        while k < (*psDec).nb_subfr {
            (*psDec).indices.LTPIndex[k as usize] =
                crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                    psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    crate::src::opus_1_2_1::silk::tables_LTP::silk_LTP_gain_iCDF_ptrs
                        [(*psDec).indices.PERIndex as usize],
                    8 as i32 as u32,
                ) as i8;
            k += 1
        }
        /* *********************/
        /* Decode LTP scaling */
        /* *********************/
        if condCoding == 0 as i32 {
            (*psDec).indices.LTP_scaleIndex = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
                psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                crate::src::opus_1_2_1::silk::tables_other::silk_LTPscale_iCDF.as_ptr(),
                8 as i32 as u32,
            ) as i8
        } else {
            (*psDec).indices.LTP_scaleIndex = 0 as i32 as i8
        }
    }
    (*psDec).ec_prevSignalType = (*psDec).indices.signalType as i32;
    /* **************/
    /* Decode seed */
    /* **************/
    (*psDec).indices.Seed = crate::src::opus_1_2_1::celt::entdec::ec_dec_icdf(
        psRangeDec as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
        crate::src::opus_1_2_1::silk::tables_other::silk_uniform4_iCDF.as_ptr(),
        8 as i32 as u32,
    ) as i8;
}

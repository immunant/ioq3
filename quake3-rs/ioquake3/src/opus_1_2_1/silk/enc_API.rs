use ::libc;

pub mod entcode_h {

    #[inline]

    pub unsafe extern "C" fn ec_tell(
        mut _this: *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
    ) -> libc::c_int {
        return (*_this).nbits_total
            - (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int
                * 8 as libc::c_int
                - (*_this).rng.leading_zeros() as i32);
    }
}

pub use crate::control_h::silk_EncControlStruct;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::resampler_structs_h::silk_resampler_state_struct;
pub use crate::resampler_structs_h::C2RustUnnamed_64;
pub use crate::resampler_structs_h::_silk_resampler_state_struct;
pub use crate::src::opus_1_2_1::celt::entcode::ec_ctx;
pub use crate::src::opus_1_2_1::celt::entcode::ec_enc;
pub use crate::src::opus_1_2_1::celt::entcode::ec_window;

pub use crate::src::opus_1_2_1::silk::enc_API::entcode_h::ec_tell;

pub use crate::structs_FLP_h::silk_encoder;
pub use crate::structs_FLP_h::silk_encoder_state_FLP;
pub use crate::structs_FLP_h::silk_shape_state_FLP;
pub use crate::structs_h::silk_LP_state;
pub use crate::structs_h::silk_NLSF_CB_struct;
pub use crate::structs_h::silk_VAD_state;
pub use crate::structs_h::silk_encoder_state;
pub use crate::structs_h::silk_nsq_state;
pub use crate::structs_h::stereo_enc_state;
pub use crate::structs_h::SideInfoIndices;

/* ***************************************/
/* Encoder functions                    */
/* ***************************************/
#[no_mangle]

pub unsafe extern "C" fn silk_Get_Encoder_Size(mut encSizeBytes: *mut libc::c_int) -> libc::c_int
/* O    Number of bytes in SILK encoder state           */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    *encSizeBytes =
        ::std::mem::size_of::<crate::structs_FLP_h::silk_encoder>() as libc::c_ulong as libc::c_int;
    return ret;
}
/* ************************/
/* Init or Reset encoder */
/* ************************/
#[no_mangle]

pub unsafe extern "C" fn silk_InitEncoder(
    mut encState: *mut libc::c_void,
    mut arch: libc::c_int,
    mut encStatus: *mut crate::control_h::silk_EncControlStruct,
) -> libc::c_int
/* O    Encoder Status                                  */ {
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        0 as *mut crate::structs_FLP_h::silk_encoder;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    psEnc = encState as *mut crate::structs_FLP_h::silk_encoder;
    /* Reset encoder */
    crate::stdlib::memset(
        psEnc as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::structs_FLP_h::silk_encoder>() as libc::c_ulong,
    );
    n = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        ret += crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize) as *mut _
                as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            arch,
        );
        (ret) != 0;
        n += 1
    }
    (*psEnc).nChannelsAPI = 1 as libc::c_int;
    (*psEnc).nChannelsInternal = 1 as libc::c_int;
    /* Read control structure */
    ret += silk_QueryEncoder(encState, encStatus);
    (ret) != 0;
    return ret;
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
/* **************************************/
/* Read control structure from encoder */
/* **************************************/
/* **************************************/
/* Read control structure from encoder */
/* **************************************/

unsafe extern "C" fn silk_QueryEncoder(
    mut encState: *const libc::c_void,
    mut encStatus: *mut crate::control_h::silk_EncControlStruct,
) -> libc::c_int
/* O    Encoder Status                                  */ {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut state_Fxx: *mut crate::structs_FLP_h::silk_encoder_state_FLP =
        0 as *mut crate::structs_FLP_h::silk_encoder_state_FLP;
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        encState as *mut crate::structs_FLP_h::silk_encoder;
    state_Fxx = (*psEnc).state_Fxx.as_mut_ptr();
    (*encStatus).nChannelsAPI = (*psEnc).nChannelsAPI;
    (*encStatus).nChannelsInternal = (*psEnc).nChannelsInternal;
    (*encStatus).API_sampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .API_fs_Hz;
    (*encStatus).maxInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .maxInternal_fs_Hz;
    (*encStatus).minInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .minInternal_fs_Hz;
    (*encStatus).desiredInternalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .desiredInternal_fs_Hz;
    (*encStatus).payloadSize_ms = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .PacketSize_ms;
    (*encStatus).bitRate = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .TargetRate_bps;
    (*encStatus).packetLossPercentage = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .PacketLoss_perc;
    (*encStatus).complexity = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .Complexity;
    (*encStatus).useInBandFEC = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .useInBandFEC;
    (*encStatus).useDTX = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.useDTX;
    (*encStatus).useCBR = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.useCBR;
    (*encStatus).internalSampleRate = (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.fs_kHz
        as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * 1000 as libc::c_int as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
    (*encStatus).allowBandwidthSwitch = (*state_Fxx.offset(0 as libc::c_int as isize))
        .sCmn
        .allow_bandwidth_switch;
    (*encStatus).inWBmodeWithoutVariableLP =
        ((*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.fs_kHz == 16 as libc::c_int
            && (*state_Fxx.offset(0 as libc::c_int as isize)).sCmn.sLP.mode == 0 as libc::c_int)
            as libc::c_int;
    return ret;
}
/* *************************/
/* Encode frame with Silk */
/* *************************/
/* Note: if prefillFlag is set, the input must contain 10 ms of audio, irrespective of what                     */
/* encControl->payloadSize_ms is set to                                                                         */
#[no_mangle]

pub unsafe extern "C" fn silk_Encode(
    mut encState: *mut libc::c_void,
    mut encControl: *mut crate::control_h::silk_EncControlStruct,
    mut samplesIn: *const crate::opus_types_h::opus_int16,
    mut nSamplesIn: libc::c_int,
    mut psRangeEnc: *mut crate::src::opus_1_2_1::celt::entcode::ec_enc,
    mut nBytesOut: *mut crate::opus_types_h::opus_int32,
    prefillFlag: libc::c_int,
) -> libc::c_int
/* I    Flag to indicate prefilling buffers no coding   */ {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nBits: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp_payloadSize_ms: libc::c_int = 0 as libc::c_int;
    let mut tmp_complexity: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nSamplesToBuffer: libc::c_int = 0;
    let mut nSamplesToBufferMax: libc::c_int = 0;
    let mut nBlocksOf10ms: libc::c_int = 0;
    let mut nSamplesFromInput: libc::c_int = 0 as libc::c_int;
    let mut nSamplesFromInputMax: libc::c_int = 0;
    let mut speech_act_thr_for_switch_Q8: libc::c_int = 0;
    let mut TargetRate_bps: crate::opus_types_h::opus_int32 = 0;
    let mut MStargetRates_bps: [crate::opus_types_h::opus_int32; 2] = [0; 2];
    let mut channelRate_bps: crate::opus_types_h::opus_int32 = 0;
    let mut LBRR_symbol: crate::opus_types_h::opus_int32 = 0;
    let mut sum: crate::opus_types_h::opus_int32 = 0;
    let mut psEnc: *mut crate::structs_FLP_h::silk_encoder =
        encState as *mut crate::structs_FLP_h::silk_encoder;
    let mut buf: *mut crate::opus_types_h::opus_int16 = 0 as *mut crate::opus_types_h::opus_int16;
    let mut transition: libc::c_int = 0;
    let mut curr_block: libc::c_int = 0;
    let mut tot_blocks: libc::c_int = 0;
    if (*encControl).reducedDependency != 0 {
        (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .first_frame_after_reset = 1 as libc::c_int;
        (*psEnc).state_Fxx[1 as libc::c_int as usize]
            .sCmn
            .first_frame_after_reset = 1 as libc::c_int
    }
    (*psEnc).state_Fxx[1 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded = 0 as libc::c_int;
    (*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded = (*psEnc).state_Fxx[1 as libc::c_int as usize]
        .sCmn
        .nFramesEncoded;
    /* Check values in encoder control structure */
    ret = crate::src::opus_1_2_1::silk::check_control_input::check_control_input(
        encControl as *mut crate::control_h::silk_EncControlStruct,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    (*encControl).switchReady = 0 as libc::c_int;
    if (*encControl).nChannelsInternal > (*psEnc).nChannelsInternal {
        /* Mono -> Stereo transition: init state of second channel and stereo state */
        ret += crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
            &mut *(*psEnc)
                .state_Fxx
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut _
                as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.arch,
        );
        crate::stdlib::memset(
            (*psEnc).sStereo.pred_prev_Q13.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            (*psEnc).sStereo.sSide.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 2]>() as libc::c_ulong,
        );
        (*psEnc).sStereo.mid_side_amp_Q0[0 as libc::c_int as usize] = 0 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[1 as libc::c_int as usize] = 1 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[2 as libc::c_int as usize] = 0 as libc::c_int;
        (*psEnc).sStereo.mid_side_amp_Q0[3 as libc::c_int as usize] = 1 as libc::c_int;
        (*psEnc).sStereo.width_prev_Q14 = 0 as libc::c_int as crate::opus_types_h::opus_int16;
        (*psEnc).sStereo.smth_width_Q14 = ((1 as libc::c_int as libc::c_longlong
            * ((1 as libc::c_int as libc::c_longlong) << 14 as libc::c_int))
            as libc::c_double
            + 0.5f64) as crate::opus_types_h::opus_int32
            as crate::opus_types_h::opus_int16;
        if (*psEnc).nChannelsAPI == 2 as libc::c_int {
            crate::stdlib::memcpy(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .resampler_state
                    as *mut crate::resampler_structs_h::silk_resampler_state_struct
                    as *mut libc::c_void,
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state
                    as *mut crate::resampler_structs_h::silk_resampler_state_struct
                    as *const libc::c_void,
                ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>()
                    as libc::c_ulong,
            );
            crate::stdlib::memcpy(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .In_HP_State as *mut [crate::opus_types_h::opus_int32; 2]
                    as *mut libc::c_void,
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .In_HP_State as *mut [crate::opus_types_h::opus_int32; 2]
                    as *const libc::c_void,
                ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>() as libc::c_ulong,
            );
        }
    }
    transition = ((*encControl).payloadSize_ms
        != (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .PacketSize_ms
        || (*psEnc).nChannelsInternal != (*encControl).nChannelsInternal)
        as libc::c_int;
    (*psEnc).nChannelsAPI = (*encControl).nChannelsAPI;
    (*psEnc).nChannelsInternal = (*encControl).nChannelsInternal;
    nBlocksOf10ms = 100 as libc::c_int * nSamplesIn / (*encControl).API_sampleRate;
    tot_blocks = if nBlocksOf10ms > 1 as libc::c_int {
        (nBlocksOf10ms) >> 1 as libc::c_int
    } else {
        1 as libc::c_int
    };
    curr_block = 0 as libc::c_int;
    if prefillFlag != 0 {
        /* Only accept input length of 10 ms */
        if nBlocksOf10ms != 1 as libc::c_int {
            return -(101 as libc::c_int);
        }
        /* Reset Encoder */
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            ret = crate::src::opus_1_2_1::silk::init_encoder::silk_init_encoder(
                &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize) as *mut _
                    as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
                (*psEnc).state_Fxx[n as usize].sCmn.arch,
            );
            n += 1
        }
        tmp_payloadSize_ms = (*encControl).payloadSize_ms;
        (*encControl).payloadSize_ms = 10 as libc::c_int;
        tmp_complexity = (*encControl).complexity;
        (*encControl).complexity = 0 as libc::c_int;
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 1 as libc::c_int;
            n += 1
        }
    } else {
        /* Only accept input lengths that are a multiple of 10 ms */
        if nBlocksOf10ms * (*encControl).API_sampleRate != 100 as libc::c_int * nSamplesIn
            || nSamplesIn < 0 as libc::c_int
        {
            return -(101 as libc::c_int);
        }
        /* Make sure no more than one packet can be produced */
        if 1000 as libc::c_int * nSamplesIn
            > (*encControl).payloadSize_ms * (*encControl).API_sampleRate
        {
            return -(101 as libc::c_int);
        }
    }
    n = 0 as libc::c_int;
    while n < (*encControl).nChannelsInternal {
        /* Force the side channel to the same rate as the mid */
        let mut force_fs_kHz: libc::c_int = if n == 1 as libc::c_int {
            (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz
        } else {
            0 as libc::c_int
        };
        ret = crate::src::opus_1_2_1::silk::control_codec::silk_control_encoder(
            &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize) as *mut _
                as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
            encControl as *mut crate::control_h::silk_EncControlStruct,
            (*psEnc).allowBandwidthSwitch,
            n,
            force_fs_kHz,
        );
        if ret != 0 as libc::c_int {
            return ret;
        }
        if (*psEnc).state_Fxx[n as usize].sCmn.first_frame_after_reset != 0 || transition != 0 {
            i = 0 as libc::c_int;
            while i
                < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
            {
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] = 0 as libc::c_int;
                i += 1
            }
        }
        (*psEnc).state_Fxx[n as usize].sCmn.inDTX = (*psEnc).state_Fxx[n as usize].sCmn.useDTX;
        n += 1
    }
    /* Input buffering/resampling and encoding */
    nSamplesToBufferMax = 10 as libc::c_int
        * nBlocksOf10ms
        * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz;
    nSamplesFromInputMax = nSamplesToBufferMax
        * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.API_fs_Hz
        / ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz * 1000 as libc::c_int);
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong)
            .wrapping_mul(nSamplesFromInputMax as libc::c_ulong) as usize,
    );
    buf = fresh0.as_mut_ptr() as *mut crate::opus_types_h::opus_int16;
    loop {
        nSamplesToBuffer = (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .frame_length
            - (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx;
        nSamplesToBuffer = if nSamplesToBuffer < nSamplesToBufferMax {
            nSamplesToBuffer
        } else {
            nSamplesToBufferMax
        };
        nSamplesFromInput = nSamplesToBuffer
            * (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.API_fs_Hz
            / ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz * 1000 as libc::c_int);
        /* Resample and write to buffer */
        if (*encControl).nChannelsAPI == 2 as libc::c_int
            && (*encControl).nChannelsInternal == 2 as libc::c_int
        {
            let mut id: libc::c_int = (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded;
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                *buf.offset(n as isize) = *samplesIn.offset((2 as libc::c_int * n) as isize);
                n += 1
            }
            /* Making sure to start both resamplers from the same state when switching from mono to stereo */
            if (*psEnc).nPrevChannelsInternal == 1 as libc::c_int && id == 0 as libc::c_int {
                crate::stdlib::memcpy(
                    &mut (*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .resampler_state
                        as *mut crate::resampler_structs_h::silk_resampler_state_struct
                        as *mut libc::c_void,
                    &mut (*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .resampler_state
                        as *mut crate::resampler_structs_h::silk_resampler_state_struct
                        as *const libc::c_void,
                    ::std::mem::size_of::<crate::resampler_structs_h::silk_resampler_state_struct>()
                        as libc::c_ulong,
                );
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(
                    ((*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .inputBufIx
                        + 2 as libc::c_int) as isize,
                ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer;
            nSamplesToBuffer = (*psEnc).state_Fxx[1 as libc::c_int as usize]
                .sCmn
                .frame_length
                - (*psEnc).state_Fxx[1 as libc::c_int as usize]
                    .sCmn
                    .inputBufIx;
            nSamplesToBuffer = if nSamplesToBuffer
                < 10 as libc::c_int
                    * nBlocksOf10ms
                    * (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.fs_kHz
            {
                nSamplesToBuffer
            } else {
                (10 as libc::c_int * nBlocksOf10ms)
                    * (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.fs_kHz
            };
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                *buf.offset(n as isize) =
                    *samplesIn.offset((2 as libc::c_int * n + 1 as libc::c_int) as isize);
                n += 1
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(
                    ((*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .inputBufIx
                        + 2 as libc::c_int) as isize,
                ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[1 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer
        } else if (*encControl).nChannelsAPI == 2 as libc::c_int
            && (*encControl).nChannelsInternal == 1 as libc::c_int
        {
            /* Combine left and right channels before resampling */
            n = 0 as libc::c_int;
            while n < nSamplesFromInput {
                sum = *samplesIn.offset((2 as libc::c_int * n) as isize) as libc::c_int
                    + *samplesIn.offset((2 as libc::c_int * n + 1 as libc::c_int) as isize)
                        as libc::c_int;
                *buf.offset(n as isize) = if 1 as libc::c_int == 1 as libc::c_int {
                    (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
                } else {
                    ((sum >> 1 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int)
                        >> 1 as libc::c_int
                } as crate::opus_types_h::opus_int16;
                n += 1
            }
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(
                    ((*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .inputBufIx
                        + 2 as libc::c_int) as isize,
                ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            /* On the first mono frame, average the results for the two resampler states  */
            if (*psEnc).nPrevChannelsInternal == 2 as libc::c_int
                && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    == 0 as libc::c_int
            {
                ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                    &mut (*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .resampler_state as *mut _
                        as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                    &mut *(*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize))
                    .sCmn
                    .inputBuf
                    .as_mut_ptr()
                    .offset(
                        ((*(*psEnc)
                            .state_Fxx
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .inputBufIx
                            + 2 as libc::c_int) as isize,
                    ),
                    buf as *const crate::opus_types_h::opus_int16,
                    nSamplesFromInput,
                );
                n = 0 as libc::c_int;
                while n
                    < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .frame_length
                {
                    (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inputBuf[((*psEnc).state_Fxx
                        [0 as libc::c_int as usize]
                        .sCmn
                        .inputBufIx
                        + n
                        + 2 as libc::c_int)
                        as usize] = ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inputBuf
                        [((*psEnc).state_Fxx[0 as libc::c_int as usize]
                            .sCmn
                            .inputBufIx
                            + n
                            + 2 as libc::c_int) as usize]
                        as libc::c_int
                        + (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.inputBuf[((*psEnc)
                            .state_Fxx[1 as libc::c_int as usize]
                            .sCmn
                            .inputBufIx
                            + n
                            + 2 as libc::c_int)
                            as usize] as libc::c_int
                        >> 1 as libc::c_int)
                        as crate::opus_types_h::opus_int16;
                    n += 1
                }
            }
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer
        } else {
            crate::stdlib::memcpy(
                buf as *mut libc::c_void,
                samplesIn as *const libc::c_void,
                (nSamplesFromInput as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
            ret += crate::src::opus_1_2_1::silk::resampler::silk_resampler(
                &mut (*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .resampler_state as *mut _
                    as *mut crate::resampler_structs_h::_silk_resampler_state_struct,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(
                    ((*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .inputBufIx
                        + 2 as libc::c_int) as isize,
                ),
                buf as *const crate::opus_types_h::opus_int16,
                nSamplesFromInput,
            );
            (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .inputBufIx += nSamplesToBuffer
        }
        samplesIn = samplesIn.offset((nSamplesFromInput * (*encControl).nChannelsAPI) as isize);
        nSamplesIn -= nSamplesFromInput;
        /* Default */
        (*psEnc).allowBandwidthSwitch = 0 as libc::c_int;
        /* Silk encoder */
        if !((*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .inputBufIx
            >= (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .frame_length)
        {
            break;
        }
        /* Enough data in input buffer, so encode */
        /* Deal with LBRR data */
        if (*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .nFramesEncoded
            == 0 as libc::c_int
            && prefillFlag == 0
        {
            /* Create space at start of payload for VAD and FEC flags */
            let mut iCDF: [libc::c_uchar; 2] = [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ];
            iCDF[0 as libc::c_int as usize] = (256 as libc::c_int
                - (256 as libc::c_int
                    >> ((*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesPerPacket
                        + 1 as libc::c_int)
                        * (*encControl).nChannelsInternal))
                as libc::c_uchar;
            crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                0 as libc::c_int,
                iCDF.as_mut_ptr(),
                8 as libc::c_int as libc::c_uint,
            );
            /* Encode any LBRR data from previous packet */
            /* Encode LBRR flags */
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                LBRR_symbol = 0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    LBRR_symbol |= (((*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize]
                        as crate::opus_types_h::opus_uint32)
                        << i) as crate::opus_types_h::opus_int32;
                    i += 1
                }
                (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag = if LBRR_symbol > 0 as libc::c_int {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                } as libc::c_schar;
                if LBRR_symbol != 0
                    && (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket > 1 as libc::c_int
                {
                    crate::src::opus_1_2_1::celt::entenc::ec_enc_icdf(
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        LBRR_symbol - 1 as libc::c_int,
                        crate::src::opus_1_2_1::silk::tables_other::silk_LBRR_flags_iCDF_ptr
                            [((*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket
                                - 2 as libc::c_int) as usize],
                        8 as libc::c_int as libc::c_uint,
                    );
                }
                n += 1
            }
            /* Code LBRR indices and excitation signals */
            i = 0 as libc::c_int;
            while i
                < (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
            {
                n = 0 as libc::c_int;
                while n < (*encControl).nChannelsInternal {
                    if (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags[i as usize] != 0 {
                        let mut condCoding: libc::c_int = 0;
                        if (*encControl).nChannelsInternal == 2 as libc::c_int
                            && n == 0 as libc::c_int
                        {
                            crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_pred(psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                                                    (*psEnc).sStereo.predIx[i
                                                                                as
                                                                                usize].as_mut_ptr());
                            /* For LBRR data there's no need to code the mid-only flag if the side-channel LBRR flag is set */
                            if (*psEnc).state_Fxx[1 as libc::c_int as usize]
                                .sCmn
                                .LBRR_flags[i as usize]
                                == 0 as libc::c_int
                            {
                                crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_mid_only(psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                                                            (*psEnc).sStereo.mid_only_flags[i
                                                                                                as
                                                                                                usize]);
                            }
                        }
                        /* Use conditional coding if previous frame available */
                        if i > 0 as libc::c_int
                            && (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags
                                [(i - 1 as libc::c_int) as usize]
                                != 0
                        {
                            condCoding = 2 as libc::c_int
                        } else {
                            condCoding = 0 as libc::c_int
                        }
                        crate::src::opus_1_2_1::silk::encode_indices::silk_encode_indices(
                            &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize)).sCmn
                                as *mut _
                                as *mut crate::structs_h::silk_encoder_state,
                            psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            i,
                            1 as libc::c_int,
                            condCoding,
                        );
                        crate::src::opus_1_2_1::silk::encode_pulses::silk_encode_pulses(
                            psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize].signalType
                                as libc::c_int,
                            (*psEnc).state_Fxx[n as usize].sCmn.indices_LBRR[i as usize]
                                .quantOffsetType as libc::c_int,
                            (*psEnc).state_Fxx[n as usize].sCmn.pulses_LBRR[i as usize]
                                .as_mut_ptr(),
                            (*psEnc).state_Fxx[n as usize].sCmn.frame_length,
                        );
                    }
                    n += 1
                }
                i += 1
            }
            /* Reset LBRR flags */
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                crate::stdlib::memset(
                    (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flags.as_mut_ptr()
                        as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_int; 3]>() as libc::c_ulong,
                );
                n += 1
            }
            (*psEnc).nBitsUsedLBRR = ec_tell(psRangeEnc)
        }
        crate::src::opus_1_2_1::silk::HP_variable_cutoff::silk_HP_variable_cutoff(
            (*psEnc).state_Fxx.as_mut_ptr() as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
        );
        /* Total target bits for packet */
        nBits = (*encControl).bitRate * (*encControl).payloadSize_ms / 1000 as libc::c_int;
        /* Subtract bits used for LBRR */
        if prefillFlag == 0 {
            nBits -= (*psEnc).nBitsUsedLBRR
        }
        /* Divide by number of uncoded frames left in packet */
        nBits = nBits
            / (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesPerPacket;
        /* Convert to bits/second */
        if (*encControl).payloadSize_ms == 10 as libc::c_int {
            TargetRate_bps = nBits as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * 100 as libc::c_int as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
        } else {
            TargetRate_bps = nBits as crate::opus_types_h::opus_int16
                as crate::opus_types_h::opus_int32
                * 50 as libc::c_int as crate::opus_types_h::opus_int16
                    as crate::opus_types_h::opus_int32
        }
        /* Subtract fraction of bits in excess of target in previous frames and packets */
        TargetRate_bps -= (*psEnc).nBitsExceeded * 1000 as libc::c_int / 500 as libc::c_int;
        if prefillFlag == 0
            && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                > 0 as libc::c_int
        {
            /* Compare actual vs target bits so far in this packet */
            let mut bitsBalance: crate::opus_types_h::opus_int32 = ec_tell(psRangeEnc)
                - (*psEnc).nBitsUsedLBRR
                - nBits
                    * (*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesEncoded;
            TargetRate_bps -= bitsBalance * 1000 as libc::c_int / 500 as libc::c_int
        }
        /* Never exceed input bitrate */
        TargetRate_bps = if (*encControl).bitRate > 5000 as libc::c_int {
            if TargetRate_bps > (*encControl).bitRate {
                (*encControl).bitRate
            } else if TargetRate_bps < 5000 as libc::c_int {
                5000 as libc::c_int
            } else {
                TargetRate_bps
            }
        } else if TargetRate_bps > 5000 as libc::c_int {
            5000 as libc::c_int
        } else if TargetRate_bps < (*encControl).bitRate {
            (*encControl).bitRate
        } else {
            TargetRate_bps
        };
        /* Convert Left/Right to Mid/Side */
        if (*encControl).nChannelsInternal == 2 as libc::c_int {
            crate::src::opus_1_2_1::silk::stereo_LR_to_MS::silk_stereo_LR_to_MS(
                &mut (*psEnc).sStereo as *mut _ as *mut crate::structs_h::stereo_enc_state,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
                (*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded as usize]
                    .as_mut_ptr(),
                &mut *(*psEnc).sStereo.mid_only_flags.as_mut_ptr().offset(
                    (*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .nFramesEncoded as isize,
                ),
                MStargetRates_bps.as_mut_ptr(),
                TargetRate_bps,
                (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .speech_activity_Q8,
                (*encControl).toMono,
                (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz,
                (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .frame_length,
            );
            if (*psEnc).sStereo.mid_only_flags[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded as usize] as libc::c_int
                == 0 as libc::c_int
            {
                /* Reset side channel encoder memory for first frame with side coding */
                if (*psEnc).prev_decode_only_middle == 1 as libc::c_int {
                    crate::stdlib::memset(
                        &mut (*(*psEnc)
                            .state_Fxx
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sShape
                            as *mut crate::structs_FLP_h::silk_shape_state_FLP
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<crate::structs_FLP_h::silk_shape_state_FLP>()
                            as libc::c_ulong,
                    );
                    crate::stdlib::memset(
                        &mut (*(*psEnc)
                            .state_Fxx
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .sNSQ as *mut crate::structs_h::silk_nsq_state
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<crate::structs_h::silk_nsq_state>() as libc::c_ulong,
                    );
                    crate::stdlib::memset(
                        (*psEnc).state_Fxx[1 as libc::c_int as usize]
                            .sCmn
                            .prev_NLSFq_Q15
                            .as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int16; 16]>()
                            as libc::c_ulong,
                    );
                    crate::stdlib::memset(
                        &mut (*(*psEnc)
                            .state_Fxx
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .sCmn
                        .sLP
                        .In_LP_State
                            as *mut [crate::opus_types_h::opus_int32; 2]
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<[crate::opus_types_h::opus_int32; 2]>()
                            as libc::c_ulong,
                    );
                    (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.prevLag = 100 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .sNSQ
                        .lagPrev = 100 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sShape
                        .LastGainIndex = 10 as libc::c_int as libc::c_schar;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .prevSignalType = 0 as libc::c_int as libc::c_schar;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .sNSQ
                        .prev_gain_Q16 = 65536 as libc::c_int;
                    (*psEnc).state_Fxx[1 as libc::c_int as usize]
                        .sCmn
                        .first_frame_after_reset = 1 as libc::c_int
                }
                crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_do_VAD_FLP(
                    &mut *(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as *mut _
                        as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
                );
            } else {
                (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.VAD_flags[(*psEnc).state_Fxx
                    [0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    as usize] = 0 as libc::c_int as libc::c_schar
            }
            if prefillFlag == 0 {
                crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_pred(
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    (*psEnc).sStereo.predIx[(*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesEncoded as usize]
                        .as_mut_ptr(),
                );
                if (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.VAD_flags[(*psEnc).state_Fxx
                    [0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    as usize] as libc::c_int
                    == 0 as libc::c_int
                {
                    crate::src::opus_1_2_1::silk::stereo_encode_pred::silk_stereo_encode_mid_only(
                        psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                        (*psEnc).sStereo.mid_only_flags[(*psEnc).state_Fxx
                            [0 as libc::c_int as usize]
                            .sCmn
                            .nFramesEncoded
                            as usize],
                    );
                }
            }
        } else {
            /* Buffering */
            crate::stdlib::memcpy(
                (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .inputBuf
                    .as_mut_ptr() as *mut libc::c_void,
                (*psEnc).sStereo.sMid.as_mut_ptr() as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
            crate::stdlib::memcpy(
                (*psEnc).sStereo.sMid.as_mut_ptr() as *mut libc::c_void,
                &mut *(*(*psEnc)
                    .state_Fxx
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .sCmn
                .inputBuf
                .as_mut_ptr()
                .offset(
                    (*(*psEnc)
                        .state_Fxx
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .sCmn
                    .frame_length as isize,
                ) as *mut crate::opus_types_h::opus_int16 as *const libc::c_void,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<crate::opus_types_h::opus_int16>() as libc::c_ulong
                    ),
            );
        }
        crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_do_VAD_FLP(
            &mut *(*psEnc)
                .state_Fxx
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut _
                as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
        );
        /* Encode */
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            let mut maxBits: libc::c_int = 0;
            let mut useCBR: libc::c_int = 0;
            /* Handling rate constraints */
            maxBits = (*encControl).maxBits;
            if tot_blocks == 2 as libc::c_int && curr_block == 0 as libc::c_int {
                maxBits = maxBits * 3 as libc::c_int / 5 as libc::c_int
            } else if tot_blocks == 3 as libc::c_int {
                if curr_block == 0 as libc::c_int {
                    maxBits = maxBits * 2 as libc::c_int / 5 as libc::c_int
                } else if curr_block == 1 as libc::c_int {
                    maxBits = maxBits * 3 as libc::c_int / 4 as libc::c_int
                }
            }
            useCBR = ((*encControl).useCBR != 0 && curr_block == tot_blocks - 1 as libc::c_int)
                as libc::c_int;
            if (*encControl).nChannelsInternal == 1 as libc::c_int {
                channelRate_bps = TargetRate_bps
            } else {
                channelRate_bps = MStargetRates_bps[n as usize];
                if n == 0 as libc::c_int
                    && MStargetRates_bps[1 as libc::c_int as usize] > 0 as libc::c_int
                {
                    useCBR = 0 as libc::c_int;
                    /* Give mid up to 1/2 of the max bits for that frame */
                    maxBits -= (*encControl).maxBits / (tot_blocks * 2 as libc::c_int)
                }
            }
            if channelRate_bps > 0 as libc::c_int {
                let mut condCoding_0: libc::c_int = 0;
                crate::src::opus_1_2_1::silk::control_SNR::silk_control_SNR(
                    &mut (*(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize)).sCmn as *mut _
                        as *mut crate::structs_h::silk_encoder_state,
                    channelRate_bps,
                );
                /* Use independent coding if no previous frame available */
                if (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesEncoded
                    - n
                    <= 0 as libc::c_int
                {
                    condCoding_0 = 0 as libc::c_int
                } else if n > 0 as libc::c_int && (*psEnc).prev_decode_only_middle != 0 {
                    /* If we skipped a side frame in this packet, we don't
                    need LTP scaling; the LTP state is well-defined. */
                    condCoding_0 = 1 as libc::c_int
                } else {
                    condCoding_0 = 2 as libc::c_int
                }
                ret = crate::src::opus_1_2_1::silk::float::encode_frame_FLP::silk_encode_frame_FLP(
                    &mut *(*psEnc).state_Fxx.as_mut_ptr().offset(n as isize) as *mut _
                        as *mut crate::structs_FLP_h::silk_encoder_state_FLP,
                    nBytesOut,
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    condCoding_0,
                    maxBits,
                    useCBR,
                );
                (ret) != 0 as libc::c_int;
            }
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.inputBufIx = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.nFramesEncoded += 1;
            n += 1
        }
        (*psEnc).prev_decode_only_middle =
            (*psEnc).sStereo.mid_only_flags[((*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                - 1 as libc::c_int) as usize] as libc::c_int;
        /* Insert VAD and FEC flags at beginning of bitstream */
        if *nBytesOut > 0 as libc::c_int
            && (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .nFramesEncoded
                == (*psEnc).state_Fxx[0 as libc::c_int as usize]
                    .sCmn
                    .nFramesPerPacket
        {
            flags = 0 as libc::c_int;
            n = 0 as libc::c_int;
            while n < (*encControl).nChannelsInternal {
                i = 0 as libc::c_int;
                while i < (*psEnc).state_Fxx[n as usize].sCmn.nFramesPerPacket {
                    flags = ((flags as crate::opus_types_h::opus_uint32) << 1 as libc::c_int)
                        as crate::opus_types_h::opus_int32;
                    flags |=
                        (*psEnc).state_Fxx[n as usize].sCmn.VAD_flags[i as usize] as libc::c_int;
                    i += 1
                }
                flags = ((flags as crate::opus_types_h::opus_uint32) << 1 as libc::c_int)
                    as crate::opus_types_h::opus_int32;
                flags |= (*psEnc).state_Fxx[n as usize].sCmn.LBRR_flag as libc::c_int;
                n += 1
            }
            if prefillFlag == 0 {
                crate::src::opus_1_2_1::celt::entenc::ec_enc_patch_initial_bits(
                    psRangeEnc as *mut crate::src::opus_1_2_1::celt::entcode::ec_ctx,
                    flags as libc::c_uint,
                    (((*psEnc).state_Fxx[0 as libc::c_int as usize]
                        .sCmn
                        .nFramesPerPacket
                        + 1 as libc::c_int)
                        * (*encControl).nChannelsInternal) as libc::c_uint,
                );
            }
            /* Return zero bytes if all channels DTXed */
            if (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.inDTX != 0
                && ((*encControl).nChannelsInternal == 1 as libc::c_int
                    || (*psEnc).state_Fxx[1 as libc::c_int as usize].sCmn.inDTX != 0)
            {
                *nBytesOut = 0 as libc::c_int
            }
            (*psEnc).nBitsExceeded += *nBytesOut * 8 as libc::c_int;
            (*psEnc).nBitsExceeded -=
                (*encControl).bitRate * (*encControl).payloadSize_ms / 1000 as libc::c_int;
            (*psEnc).nBitsExceeded = if 0 as libc::c_int > 10000 as libc::c_int {
                if (*psEnc).nBitsExceeded > 0 as libc::c_int {
                    0 as libc::c_int
                } else if (*psEnc).nBitsExceeded < 10000 as libc::c_int {
                    10000 as libc::c_int
                } else {
                    (*psEnc).nBitsExceeded
                }
            } else if (*psEnc).nBitsExceeded > 10000 as libc::c_int {
                10000 as libc::c_int
            } else if (*psEnc).nBitsExceeded < 0 as libc::c_int {
                0 as libc::c_int
            } else {
                (*psEnc).nBitsExceeded
            };
            /* Update flag indicating if bandwidth switching is allowed */
            speech_act_thr_for_switch_Q8 =
                (((0.05f32
                    * ((1 as libc::c_int as libc::c_longlong) << 8 as libc::c_int) as libc::c_float)
                    as libc::c_double
                    + 0.5f64) as crate::opus_types_h::opus_int32
                    as libc::c_longlong
                    + ((((1 as libc::c_int as libc::c_float - 0.05f32)
                        / 5000 as libc::c_int as libc::c_float
                        * ((1 as libc::c_int as libc::c_longlong)
                            << 16 as libc::c_int + 8 as libc::c_int)
                            as libc::c_float) as libc::c_double
                        + 0.5f64) as crate::opus_types_h::opus_int32
                        as libc::c_longlong
                        * (*psEnc).timeSinceSwitchAllowed_ms as crate::opus_types_h::opus_int16
                            as libc::c_longlong
                        >> 16 as libc::c_int)) as crate::opus_types_h::opus_int32;
            if (*psEnc).state_Fxx[0 as libc::c_int as usize]
                .sCmn
                .speech_activity_Q8
                < speech_act_thr_for_switch_Q8
            {
                (*psEnc).allowBandwidthSwitch = 1 as libc::c_int;
                (*psEnc).timeSinceSwitchAllowed_ms = 0 as libc::c_int
            } else {
                (*psEnc).allowBandwidthSwitch = 0 as libc::c_int;
                (*psEnc).timeSinceSwitchAllowed_ms += (*encControl).payloadSize_ms
            }
        }
        if nSamplesIn == 0 as libc::c_int {
            break;
        }
        curr_block += 1
    }
    (*psEnc).nPrevChannelsInternal = (*encControl).nChannelsInternal;
    (*encControl).allowBandwidthSwitch = (*psEnc).allowBandwidthSwitch;
    (*encControl).inWBmodeWithoutVariableLP =
        ((*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz == 16 as libc::c_int
            && (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.sLP.mode == 0 as libc::c_int)
            as libc::c_int;
    (*encControl).internalSampleRate = (*psEnc).state_Fxx[0 as libc::c_int as usize].sCmn.fs_kHz
        as crate::opus_types_h::opus_int16
        as crate::opus_types_h::opus_int32
        * 1000 as libc::c_int as crate::opus_types_h::opus_int16 as crate::opus_types_h::opus_int32;
    (*encControl).stereoWidth_Q14 = if (*encControl).toMono != 0 {
        0 as libc::c_int
    } else {
        (*psEnc).sStereo.smth_width_Q14 as libc::c_int
    };
    if prefillFlag != 0 {
        (*encControl).payloadSize_ms = tmp_payloadSize_ms;
        (*encControl).complexity = tmp_complexity;
        n = 0 as libc::c_int;
        while n < (*encControl).nChannelsInternal {
            (*psEnc).state_Fxx[n as usize]
                .sCmn
                .controlled_since_last_payload = 0 as libc::c_int;
            (*psEnc).state_Fxx[n as usize].sCmn.prefillFlag = 0 as libc::c_int;
            n += 1
        }
    }
    (*encControl).signalType = (*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .indices
        .signalType as libc::c_int;
    (*encControl).offset = crate::src::opus_1_2_1::silk::tables_other::silk_Quantization_Offsets_Q10
        [((*psEnc).state_Fxx[0 as libc::c_int as usize]
            .sCmn
            .indices
            .signalType as libc::c_int
            >> 1 as libc::c_int) as usize][(*psEnc).state_Fxx[0 as libc::c_int as usize]
        .sCmn
        .indices
        .quantOffsetType as usize] as libc::c_int;
    return ret;
}

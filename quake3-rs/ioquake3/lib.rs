#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![feature(stdsimd)]
#![register_tool(c2rust)]

pub mod mathops_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_61 {
        pub f: f32,
        pub i: crate::opus_types_h::opus_uint32,
    }
}
pub mod scales_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_58 {
        pub i: crate::config_types_h::ogg_uint32_t,
        pub f: f32,
    }
}
pub mod zlib_h {
    pub type alloc_func = Option<
        unsafe extern "C" fn(
            _: crate::zconf_h::voidpf,
            _: crate::zconf_h::uInt,
            _: crate::zconf_h::uInt,
        ) -> crate::zconf_h::voidpf,
    >;

    pub type free_func =
        Option<unsafe extern "C" fn(_: crate::zconf_h::voidpf, _: crate::zconf_h::voidpf) -> ()>;

    pub type z_stream = crate::zlib_h::z_stream_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct z_stream_s {
        pub next_in: *mut crate::zconf_h::Bytef,
        pub avail_in: crate::zconf_h::uInt,
        pub total_in: crate::zconf_h::uLong,
        pub next_out: *mut crate::zconf_h::Bytef,
        pub avail_out: crate::zconf_h::uInt,
        pub total_out: crate::zconf_h::uLong,
        pub msg: *mut libc::c_char,
        pub state: *mut crate::zlib_h::internal_state,
        pub zalloc: crate::zlib_h::alloc_func,
        pub zfree: crate::zlib_h::free_func,
        pub opaque: crate::zconf_h::voidpf,
        pub data_type: i32,
        pub adler: crate::zconf_h::uLong,
        pub reserved: crate::zconf_h::uLong,
    }

    pub type z_streamp = *mut crate::zlib_h::z_stream;

    pub type gz_header = crate::zlib_h::gz_header_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gz_header_s {
        pub text: i32,
        pub time: crate::zconf_h::uLong,
        pub xflags: i32,
        pub os: i32,
        pub extra: *mut crate::zconf_h::Bytef,
        pub extra_len: crate::zconf_h::uInt,
        pub extra_max: crate::zconf_h::uInt,
        pub name: *mut crate::zconf_h::Bytef,
        pub name_max: crate::zconf_h::uInt,
        pub comment: *mut crate::zconf_h::Bytef,
        pub comm_max: crate::zconf_h::uInt,
        pub hcrc: i32,
        pub done: i32,
    }

    pub type gz_headerp = *mut crate::zlib_h::gz_header;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct internal_state {
        pub dummy: i32,
    }
}
pub mod zconf_h {
    pub type Byte = u8;

    pub type uInt = u32;

    pub type uLong = usize;

    pub type Bytef = crate::zconf_h::Byte;

    pub type uLongf = crate::zconf_h::uLong;

    pub type voidpf = *mut libc::c_void;

    pub type voidp = *mut libc::c_void;
}
pub mod cm_local_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cNode_t {
        pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
        pub children: [i32; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cLeaf_t {
        pub cluster: i32,
        pub area: i32,
        pub firstLeafBrush: i32,
        pub numLeafBrushes: i32,
        pub firstLeafSurface: i32,
        pub numLeafSurfaces: i32,
    }

    pub type cmodel_t = crate::cm_local_h::cmodel_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cmodel_s {
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub leaf: crate::cm_local_h::cLeaf_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cbrushside_t {
        pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
        pub surfaceFlags: i32,
        pub shaderNum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cbrush_t {
        pub shaderNum: i32,
        pub contents: i32,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub numsides: i32,
        pub sides: *mut crate::cm_local_h::cbrushside_t,
        pub checkcount: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cPatch_t {
        pub checkcount: i32,
        pub surfaceFlags: i32,
        pub contents: i32,
        pub pc: *mut crate::src::qcommon::cm_patch::patchCollide_s,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cArea_t {
        pub floodnum: i32,
        pub floodvalid: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clipMap_t {
        pub name: [libc::c_char; 64],
        pub numShaders: i32,
        pub shaders: *mut crate::qfiles_h::dshader_t,
        pub numBrushSides: i32,
        pub brushsides: *mut crate::cm_local_h::cbrushside_t,
        pub numPlanes: i32,
        pub planes: *mut crate::src::qcommon::q_shared::cplane_t,
        pub numNodes: i32,
        pub nodes: *mut crate::cm_local_h::cNode_t,
        pub numLeafs: i32,
        pub leafs: *mut crate::cm_local_h::cLeaf_t,
        pub numLeafBrushes: i32,
        pub leafbrushes: *mut i32,
        pub numLeafSurfaces: i32,
        pub leafsurfaces: *mut i32,
        pub numSubModels: i32,
        pub cmodels: *mut crate::cm_local_h::cmodel_t,
        pub numBrushes: i32,
        pub brushes: *mut crate::cm_local_h::cbrush_t,
        pub numClusters: i32,
        pub clusterBytes: i32,
        pub visibility: *mut crate::src::qcommon::q_shared::byte,
        pub vised: crate::src::qcommon::q_shared::qboolean,
        pub numEntityChars: i32,
        pub entityString: *mut libc::c_char,
        pub numAreas: i32,
        pub areas: *mut crate::cm_local_h::cArea_t,
        pub areaPortals: *mut i32,
        pub numSurfaces: i32,
        pub surfaces: *mut *mut crate::cm_local_h::cPatch_t,
        pub floodvalid: i32,
        pub checkcount: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sphere_t {
        pub use_0: crate::src::qcommon::q_shared::qboolean,
        pub radius: f32,
        pub halfheight: f32,
        pub offset: crate::src::qcommon::q_shared::vec3_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct traceWork_t {
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub end: crate::src::qcommon::q_shared::vec3_t,
        pub size: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub offsets: [crate::src::qcommon::q_shared::vec3_t; 8],
        pub maxOffset: f32,
        pub extents: crate::src::qcommon::q_shared::vec3_t,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub modelOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub contents: i32,
        pub isPoint: crate::src::qcommon::q_shared::qboolean,
        pub trace: crate::src::qcommon::q_shared::trace_t,
        pub sphere: crate::cm_local_h::sphere_t,
    }

    pub type leafList_t = crate::cm_local_h::leafList_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct leafList_s {
        pub count: i32,
        pub maxcount: i32,
        pub overflowed: crate::src::qcommon::q_shared::qboolean,
        pub list: *mut i32,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub lastLeaf: i32,
        pub storeLeafs:
            Option<unsafe extern "C" fn(_: *mut crate::cm_local_h::leafList_s, _: i32) -> ()>,
    }
}
pub mod opus_private_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpusRepacketizer {
        pub toc: u8,
        pub nb_frames: i32,
        pub frames: [*const u8; 48],
        pub len: [crate::opus_types_h::opus_int16; 48],
        pub framesize: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ChannelLayout {
        pub nb_channels: i32,
        pub nb_streams: i32,
        pub nb_coupled_streams: i32,
        pub mapping: [u8; 256],
    }

    pub type downmix_func = Option<
        unsafe extern "C" fn(
            _: *const libc::c_void,
            _: *mut crate::arch_h::opus_val32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
            _: i32,
        ) -> (),
    >;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct foo {
        pub c: libc::c_char,
        pub u: crate::opus_private_h::C2RustUnnamed_98,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_98 {
        pub p: *mut libc::c_void,
        pub i: crate::opus_types_h::opus_int32,
        pub v: crate::arch_h::opus_val32,
    }
}
pub mod control_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_EncControlStruct {
        pub nChannelsAPI: crate::opus_types_h::opus_int32,
        pub nChannelsInternal: crate::opus_types_h::opus_int32,
        pub API_sampleRate: crate::opus_types_h::opus_int32,
        pub maxInternalSampleRate: crate::opus_types_h::opus_int32,
        pub minInternalSampleRate: crate::opus_types_h::opus_int32,
        pub desiredInternalSampleRate: crate::opus_types_h::opus_int32,
        pub payloadSize_ms: i32,
        pub bitRate: crate::opus_types_h::opus_int32,
        pub packetLossPercentage: i32,
        pub complexity: i32,
        pub useInBandFEC: i32,
        pub LBRR_coded: i32,
        pub useDTX: i32,
        pub useCBR: i32,
        pub maxBits: i32,
        pub toMono: i32,
        pub opusCanSwitch: i32,
        pub reducedDependency: i32,
        pub internalSampleRate: crate::opus_types_h::opus_int32,
        pub allowBandwidthSwitch: i32,
        pub inWBmodeWithoutVariableLP: i32,
        pub stereoWidth_Q14: i32,
        pub switchReady: i32,
        pub signalType: i32,
        pub offset: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_DecControlStruct {
        pub nChannelsAPI: crate::opus_types_h::opus_int32,
        pub nChannelsInternal: crate::opus_types_h::opus_int32,
        pub API_sampleRate: crate::opus_types_h::opus_int32,
        pub internalSampleRate: crate::opus_types_h::opus_int32,
        pub payloadSize_ms: i32,
        pub prevPitchLag: i32,
    }
}
pub mod structs_FLP_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_shape_state_FLP {
        pub LastGainIndex: i8,
        pub HarmShapeGain_smth: f32,
        pub Tilt_smth: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_encoder_state_FLP {
        pub sCmn: crate::structs_h::silk_encoder_state,
        pub sShape: crate::structs_FLP_h::silk_shape_state_FLP,
        pub x_buf: [f32; 720],
        pub LTPCorr: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_encoder_control_FLP {
        pub Gains: [f32; 4],
        pub PredCoef: [[f32; 16]; 2],
        pub LTPCoef: [f32; 20],
        pub LTP_scale: f32,
        pub pitchL: [i32; 4],
        pub AR: [f32; 96],
        pub LF_MA_shp: [f32; 4],
        pub LF_AR_shp: [f32; 4],
        pub Tilt: [f32; 4],
        pub HarmShapeGain: [f32; 4],
        pub Lambda: f32,
        pub input_quality: f32,
        pub coding_quality: f32,
        pub predGain: f32,
        pub LTPredCodGain: f32,
        pub ResNrg: [f32; 4],
        pub GainsUnq_Q16: [crate::opus_types_h::opus_int32; 4],
        pub lastGainIndexPrev: i8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_encoder {
        pub state_Fxx: [crate::structs_FLP_h::silk_encoder_state_FLP; 2],
        pub sStereo: crate::structs_h::stereo_enc_state,
        pub nBitsUsedLBRR: crate::opus_types_h::opus_int32,
        pub nBitsExceeded: crate::opus_types_h::opus_int32,
        pub nChannelsAPI: i32,
        pub nChannelsInternal: i32,
        pub nPrevChannelsInternal: i32,
        pub timeSinceSwitchAllowed_ms: i32,
        pub allowBandwidthSwitch: i32,
        pub prev_decode_only_middle: i32,
    }
}
pub mod structs_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_nsq_state {
        pub xq: [crate::opus_types_h::opus_int16; 640],
        pub sLTP_shp_Q14: [crate::opus_types_h::opus_int32; 640],
        pub sLPC_Q14: [crate::opus_types_h::opus_int32; 96],
        pub sAR2_Q14: [crate::opus_types_h::opus_int32; 24],
        pub sLF_AR_shp_Q14: crate::opus_types_h::opus_int32,
        pub sDiff_shp_Q14: crate::opus_types_h::opus_int32,
        pub lagPrev: i32,
        pub sLTP_buf_idx: i32,
        pub sLTP_shp_buf_idx: i32,
        pub rand_seed: crate::opus_types_h::opus_int32,
        pub prev_gain_Q16: crate::opus_types_h::opus_int32,
        pub rewhite_flag: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_VAD_state {
        pub AnaState: [crate::opus_types_h::opus_int32; 2],
        pub AnaState1: [crate::opus_types_h::opus_int32; 2],
        pub AnaState2: [crate::opus_types_h::opus_int32; 2],
        pub XnrgSubfr: [crate::opus_types_h::opus_int32; 4],
        pub NrgRatioSmth_Q8: [crate::opus_types_h::opus_int32; 4],
        pub HPstate: crate::opus_types_h::opus_int16,
        pub NL: [crate::opus_types_h::opus_int32; 4],
        pub inv_NL: [crate::opus_types_h::opus_int32; 4],
        pub NoiseLevelBias: [crate::opus_types_h::opus_int32; 4],
        pub counter: crate::opus_types_h::opus_int32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_LP_state {
        pub In_LP_State: [crate::opus_types_h::opus_int32; 2],
        pub transition_frame_no: crate::opus_types_h::opus_int32,
        pub mode: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_NLSF_CB_struct {
        pub nVectors: crate::opus_types_h::opus_int16,
        pub order: crate::opus_types_h::opus_int16,
        pub quantStepSize_Q16: crate::opus_types_h::opus_int16,
        pub invQuantStepSize_Q6: crate::opus_types_h::opus_int16,
        pub CB1_NLSF_Q8: *const u8,
        pub CB1_Wght_Q9: *const crate::opus_types_h::opus_int16,
        pub CB1_iCDF: *const u8,
        pub pred_Q8: *const u8,
        pub ec_sel: *const u8,
        pub ec_iCDF: *const u8,
        pub ec_Rates_Q5: *const u8,
        pub deltaMin_Q15: *const crate::opus_types_h::opus_int16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct stereo_enc_state {
        pub pred_prev_Q13: [crate::opus_types_h::opus_int16; 2],
        pub sMid: [crate::opus_types_h::opus_int16; 2],
        pub sSide: [crate::opus_types_h::opus_int16; 2],
        pub mid_side_amp_Q0: [crate::opus_types_h::opus_int32; 4],
        pub smth_width_Q14: crate::opus_types_h::opus_int16,
        pub width_prev_Q14: crate::opus_types_h::opus_int16,
        pub silent_side_len: crate::opus_types_h::opus_int16,
        pub predIx: [[[i8; 3]; 2]; 3],
        pub mid_only_flags: [i8; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct stereo_dec_state {
        pub pred_prev_Q13: [crate::opus_types_h::opus_int16; 2],
        pub sMid: [crate::opus_types_h::opus_int16; 2],
        pub sSide: [crate::opus_types_h::opus_int16; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SideInfoIndices {
        pub GainsIndices: [i8; 4],
        pub LTPIndex: [i8; 4],
        pub NLSFIndices: [i8; 17],
        pub lagIndex: crate::opus_types_h::opus_int16,
        pub contourIndex: i8,
        pub signalType: i8,
        pub quantOffsetType: i8,
        pub NLSFInterpCoef_Q2: i8,
        pub PERIndex: i8,
        pub LTP_scaleIndex: i8,
        pub Seed: i8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_encoder_state {
        pub In_HP_State: [crate::opus_types_h::opus_int32; 2],
        pub variable_HP_smth1_Q15: crate::opus_types_h::opus_int32,
        pub variable_HP_smth2_Q15: crate::opus_types_h::opus_int32,
        pub sLP: crate::structs_h::silk_LP_state,
        pub sVAD: crate::structs_h::silk_VAD_state,
        pub sNSQ: crate::structs_h::silk_nsq_state,
        pub prev_NLSFq_Q15: [crate::opus_types_h::opus_int16; 16],
        pub speech_activity_Q8: i32,
        pub allow_bandwidth_switch: i32,
        pub LBRRprevLastGainIndex: i8,
        pub prevSignalType: i8,
        pub prevLag: i32,
        pub pitch_LPC_win_length: i32,
        pub max_pitch_lag: i32,
        pub API_fs_Hz: crate::opus_types_h::opus_int32,
        pub prev_API_fs_Hz: crate::opus_types_h::opus_int32,
        pub maxInternal_fs_Hz: i32,
        pub minInternal_fs_Hz: i32,
        pub desiredInternal_fs_Hz: i32,
        pub fs_kHz: i32,
        pub nb_subfr: i32,
        pub frame_length: i32,
        pub subfr_length: i32,
        pub ltp_mem_length: i32,
        pub la_pitch: i32,
        pub la_shape: i32,
        pub shapeWinLength: i32,
        pub TargetRate_bps: crate::opus_types_h::opus_int32,
        pub PacketSize_ms: i32,
        pub PacketLoss_perc: i32,
        pub frameCounter: crate::opus_types_h::opus_int32,
        pub Complexity: i32,
        pub nStatesDelayedDecision: i32,
        pub useInterpolatedNLSFs: i32,
        pub shapingLPCOrder: i32,
        pub predictLPCOrder: i32,
        pub pitchEstimationComplexity: i32,
        pub pitchEstimationLPCOrder: i32,
        pub pitchEstimationThreshold_Q16: crate::opus_types_h::opus_int32,
        pub sum_log_gain_Q7: crate::opus_types_h::opus_int32,
        pub NLSF_MSVQ_Survivors: i32,
        pub first_frame_after_reset: i32,
        pub controlled_since_last_payload: i32,
        pub warping_Q16: i32,
        pub useCBR: i32,
        pub prefillFlag: i32,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub psNLSF_CB: *const crate::structs_h::silk_NLSF_CB_struct,
        pub input_quality_bands_Q15: [i32; 4],
        pub input_tilt_Q15: i32,
        pub SNR_dB_Q7: i32,
        pub VAD_flags: [i8; 3],
        pub LBRR_flag: i8,
        pub LBRR_flags: [i32; 3],
        pub indices: crate::structs_h::SideInfoIndices,
        pub pulses: [i8; 320],
        pub arch: i32,
        pub inputBuf: [crate::opus_types_h::opus_int16; 322],
        pub inputBufIx: i32,
        pub nFramesPerPacket: i32,
        pub nFramesEncoded: i32,
        pub nChannelsAPI: i32,
        pub nChannelsInternal: i32,
        pub channelNb: i32,
        pub frames_since_onset: i32,
        pub ec_prevSignalType: i32,
        pub ec_prevLagIndex: crate::opus_types_h::opus_int16,
        pub resampler_state: crate::resampler_structs_h::silk_resampler_state_struct,
        pub useDTX: i32,
        pub inDTX: i32,
        pub noSpeechCounter: i32,
        pub useInBandFEC: i32,
        pub LBRR_enabled: i32,
        pub LBRR_GainIncreases: i32,
        pub indices_LBRR: [crate::structs_h::SideInfoIndices; 3],
        pub pulses_LBRR: [[i8; 320]; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_PLC_struct {
        pub pitchL_Q8: crate::opus_types_h::opus_int32,
        pub LTPCoef_Q14: [crate::opus_types_h::opus_int16; 5],
        pub prevLPC_Q12: [crate::opus_types_h::opus_int16; 16],
        pub last_frame_lost: i32,
        pub rand_seed: crate::opus_types_h::opus_int32,
        pub randScale_Q14: crate::opus_types_h::opus_int16,
        pub conc_energy: crate::opus_types_h::opus_int32,
        pub conc_energy_shift: i32,
        pub prevLTP_scale_Q14: crate::opus_types_h::opus_int16,
        pub prevGain_Q16: [crate::opus_types_h::opus_int32; 2],
        pub fs_kHz: i32,
        pub nb_subfr: i32,
        pub subfr_length: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_CNG_struct {
        pub CNG_exc_buf_Q14: [crate::opus_types_h::opus_int32; 320],
        pub CNG_smth_NLSF_Q15: [crate::opus_types_h::opus_int16; 16],
        pub CNG_synth_state: [crate::opus_types_h::opus_int32; 16],
        pub CNG_smth_Gain_Q16: crate::opus_types_h::opus_int32,
        pub rand_seed: crate::opus_types_h::opus_int32,
        pub fs_kHz: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_decoder_state {
        pub prev_gain_Q16: crate::opus_types_h::opus_int32,
        pub exc_Q14: [crate::opus_types_h::opus_int32; 320],
        pub sLPC_Q14_buf: [crate::opus_types_h::opus_int32; 16],
        pub outBuf: [crate::opus_types_h::opus_int16; 480],
        pub lagPrev: i32,
        pub LastGainIndex: i8,
        pub fs_kHz: i32,
        pub fs_API_hz: crate::opus_types_h::opus_int32,
        pub nb_subfr: i32,
        pub frame_length: i32,
        pub subfr_length: i32,
        pub ltp_mem_length: i32,
        pub LPC_order: i32,
        pub prevNLSF_Q15: [crate::opus_types_h::opus_int16; 16],
        pub first_frame_after_reset: i32,
        pub pitch_lag_low_bits_iCDF: *const u8,
        pub pitch_contour_iCDF: *const u8,
        pub nFramesDecoded: i32,
        pub nFramesPerPacket: i32,
        pub ec_prevSignalType: i32,
        pub ec_prevLagIndex: crate::opus_types_h::opus_int16,
        pub VAD_flags: [i32; 3],
        pub LBRR_flag: i32,
        pub LBRR_flags: [i32; 3],
        pub resampler_state: crate::resampler_structs_h::silk_resampler_state_struct,
        pub psNLSF_CB: *const crate::structs_h::silk_NLSF_CB_struct,
        pub indices: crate::structs_h::SideInfoIndices,
        pub sCNG: crate::structs_h::silk_CNG_struct,
        pub lossCnt: i32,
        pub prevSignalType: i32,
        pub arch: i32,
        pub sPLC: crate::structs_h::silk_PLC_struct,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct silk_decoder_control {
        pub pitchL: [i32; 4],
        pub Gains_Q16: [crate::opus_types_h::opus_int32; 4],
        pub PredCoef_Q12: [[crate::opus_types_h::opus_int16; 16]; 2],
        pub LTPCoef_Q14: [crate::opus_types_h::opus_int16; 20],
        pub LTP_scale_Q14: i32,
    }
}
pub mod resampler_structs_h {
    pub type silk_resampler_state_struct = crate::resampler_structs_h::_silk_resampler_state_struct;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [crate::opus_types_h::opus_int32; 6],
        pub sFIR: crate::resampler_structs_h::C2RustUnnamed_64,
        pub delayBuf: [crate::opus_types_h::opus_int16; 48],
        pub resampler_function: i32,
        pub batchSize: i32,
        pub invRatio_Q16: crate::opus_types_h::opus_int32,
        pub FIR_Order: i32,
        pub FIR_Fracs: i32,
        pub Fs_in_kHz: i32,
        pub Fs_out_kHz: i32,
        pub inputDelay: i32,
        pub Coefs: *const crate::opus_types_h::opus_int16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_64 {
        pub i32_0: [crate::opus_types_h::opus_int32; 36],
        pub i16_0: [crate::opus_types_h::opus_int16; 36],
    }
}
pub mod celt_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AnalysisInfo {
        pub valid: i32,
        pub tonality: f32,
        pub tonality_slope: f32,
        pub noisiness: f32,
        pub activity: f32,
        pub music_prob: f32,
        pub vad_prob: f32,
        pub bandwidth: i32,
        pub activity_probability: f32,
        pub leak_boost: [u8; 19],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SILKInfo {
        pub signalType: i32,
        pub offset: i32,
    }
}
pub mod arch_h {
    pub type opus_val16 = f32;

    pub type opus_val32 = f32;

    pub type opus_val64 = f32;

    pub type celt_sig = f32;

    pub type celt_norm = f32;

    pub type celt_ener = f32;
}
pub mod os_h {
    pub type vorbis_fpu_control = crate::config_types_h::ogg_int16_t;
}
pub mod highlevel_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct highlevel_byblocktype {
        pub tone_mask_setting: f64,
        pub tone_peaklimit_setting: f64,
        pub noise_bias_setting: f64,
        pub noise_compand_setting: f64,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct highlevel_encode_setup {
        pub set_in_stone: i32,
        pub setup: *const libc::c_void,
        pub base_setting: f64,
        pub impulse_noisetune: f64,
        pub req: f32,
        pub managed: i32,
        pub bitrate_min: isize,
        pub bitrate_av: isize,
        pub bitrate_av_damp: f64,
        pub bitrate_max: isize,
        pub bitrate_reservoir: isize,
        pub bitrate_reservoir_bias: f64,
        pub impulse_block_p: i32,
        pub noise_normalize_p: i32,
        pub coupling_p: i32,
        pub stereo_point_setting: f64,
        pub lowpass_kHz: f64,
        pub lowpass_altered: i32,
        pub ath_floating_dB: f64,
        pub ath_absolute_dB: f64,
        pub amplitude_track_dBpersec: f64,
        pub trigger_setting: f64,
        pub block: [crate::highlevel_h::highlevel_byblocktype; 4],
    }
}
pub mod codec_internal_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_block_internal {
        pub pcmdelay: *mut *mut f32,
        pub ampmax: f32,
        pub blocktype: i32,
        pub packetblob: [*mut crate::ogg_h::oggpack_buffer; 15],
    }

    pub type vorbis_look_floor = ();

    pub type vorbis_look_residue = ();

    pub type vorbis_look_transform = ();

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info_mode {
        pub blockflag: i32,
        pub windowtype: i32,
        pub transformtype: i32,
        pub mapping: i32,
    }

    pub type vorbis_info_floor = ();

    pub type vorbis_info_residue = ();

    pub type vorbis_info_mapping = ();

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct private_state {
        pub ve: *mut crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup,
        pub window: [i32; 2],
        pub transform: [*mut *mut libc::c_void; 2],
        pub fft_look: [crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup; 2],
        pub modebits: i32,
        pub flr: *mut *mut libc::c_void,
        pub residue: *mut *mut libc::c_void,
        pub psy: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
        pub psy_g_look: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global,
        pub header: *mut u8,
        pub header1: *mut u8,
        pub header2: *mut u8,
        pub bms: crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state,
        pub sample_count: crate::config_types_h::ogg_int64_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct codec_setup_info {
        pub blocksizes: [isize; 2],
        pub modes: i32,
        pub maps: i32,
        pub floors: i32,
        pub residues: i32,
        pub books: i32,
        pub psys: i32,
        pub mode_param: [*mut crate::codec_internal_h::vorbis_info_mode; 64],
        pub map_type: [i32; 64],
        pub map_param: [*mut libc::c_void; 64],
        pub floor_type: [i32; 64],
        pub floor_param: [*mut libc::c_void; 64],
        pub residue_type: [i32; 64],
        pub residue_param: [*mut libc::c_void; 64],
        pub book_param: [*mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook; 256],
        pub fullbooks: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
        pub psy_param: [*mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy; 4],
        pub psy_g_param: crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
        pub bi: crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info,
        pub hi: crate::highlevel_h::highlevel_encode_setup,
        pub halfrate_flag: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_look_floor1 {
        pub sorted_index: [i32; 65],
        pub forward_index: [i32; 65],
        pub reverse_index: [i32; 65],
        pub hineighbor: [i32; 63],
        pub loneighbor: [i32; 63],
        pub posts: i32,
        pub n: i32,
        pub quant_q: i32,
        pub vi: *mut crate::backends_h::vorbis_info_floor1,
        pub phrasebits: isize,
        pub postbits: isize,
        pub frames: isize,
    }
}
pub mod backends_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_func_floor {
        pub pack: Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *mut crate::ogg_h::oggpack_buffer) -> (),
        >,
        pub unpack: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_info,
                _: *mut crate::ogg_h::oggpack_buffer,
            ) -> *mut libc::c_void,
        >,
        pub look: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_dsp_state,
                _: *mut libc::c_void,
            ) -> *mut libc::c_void,
        >,
        pub free_info: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub free_look: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub inverse1: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_block,
                _: *mut libc::c_void,
            ) -> *mut libc::c_void,
        >,
        pub inverse2: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_block,
                _: *mut libc::c_void,
                _: *mut libc::c_void,
                _: *mut f32,
            ) -> i32,
        >,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info_floor0 {
        pub order: i32,
        pub rate: isize,
        pub barkmap: isize,
        pub ampbits: i32,
        pub ampdB: i32,
        pub numbooks: i32,
        pub books: [i32; 16],
        pub lessthan: f32,
        pub greaterthan: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info_floor1 {
        pub partitions: i32,
        pub partitionclass: [i32; 31],
        pub class_dim: [i32; 16],
        pub class_subs: [i32; 16],
        pub class_book: [i32; 16],
        pub class_subbook: [[i32; 8]; 16],
        pub mult: i32,
        pub postlist: [i32; 65],
        pub maxover: f32,
        pub maxunder: f32,
        pub maxerr: f32,
        pub twofitweight: f32,
        pub twofitatten: f32,
        pub n: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_func_residue {
        pub pack: Option<
            unsafe extern "C" fn(_: *mut libc::c_void, _: *mut crate::ogg_h::oggpack_buffer) -> (),
        >,
        pub unpack: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_info,
                _: *mut crate::ogg_h::oggpack_buffer,
            ) -> *mut libc::c_void,
        >,
        pub look: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_dsp_state,
                _: *mut libc::c_void,
            ) -> *mut libc::c_void,
        >,
        pub free_info: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub free_look: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub class: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_block,
                _: *mut libc::c_void,
                _: *mut *mut i32,
                _: *mut i32,
                _: i32,
            ) -> *mut *mut isize,
        >,
        pub forward: Option<
            unsafe extern "C" fn(
                _: *mut crate::ogg_h::oggpack_buffer,
                _: *mut crate::codec_h::vorbis_block,
                _: *mut libc::c_void,
                _: *mut *mut i32,
                _: *mut i32,
                _: i32,
                _: *mut *mut isize,
                _: i32,
            ) -> i32,
        >,
        pub inverse: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_block,
                _: *mut libc::c_void,
                _: *mut *mut f32,
                _: *mut i32,
                _: i32,
            ) -> i32,
        >,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info_residue0 {
        pub begin: isize,
        pub end: isize,
        pub grouping: i32,
        pub partitions: i32,
        pub partvals: i32,
        pub groupbook: i32,
        pub secondstages: [i32; 64],
        pub booklist: [i32; 512],
        pub classmetric1: [i32; 64],
        pub classmetric2: [i32; 64],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_func_mapping {
        pub pack: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_info,
                _: *mut libc::c_void,
                _: *mut crate::ogg_h::oggpack_buffer,
            ) -> (),
        >,
        pub unpack: Option<
            unsafe extern "C" fn(
                _: *mut crate::codec_h::vorbis_info,
                _: *mut crate::ogg_h::oggpack_buffer,
            ) -> *mut libc::c_void,
        >,
        pub free_info: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub forward: Option<unsafe extern "C" fn(_: *mut crate::codec_h::vorbis_block) -> i32>,
        pub inverse: Option<
            unsafe extern "C" fn(_: *mut crate::codec_h::vorbis_block, _: *mut libc::c_void) -> i32,
        >,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info_mapping0 {
        pub submaps: i32,
        pub chmuxlist: [i32; 256],
        pub floorsubmap: [i32; 16],
        pub residuesubmap: [i32; 16],
        pub coupling_steps: i32,
        pub coupling_mag: [i32; 256],
        pub coupling_ang: [i32; 256],
    }
}
pub mod internal_h {
    pub type op_sample = f32;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OggOpusLink {
        pub offset: i64,
        pub data_offset: i64,
        pub end_offset: i64,
        pub pcm_file_offset: crate::config_types_h::ogg_int64_t,
        pub pcm_end: crate::config_types_h::ogg_int64_t,
        pub pcm_start: crate::config_types_h::ogg_int64_t,
        pub serialno: crate::config_types_h::ogg_uint32_t,
        pub head: crate::src::opusfile_0_9::src::opusfile::OpusHead,
        pub tags: crate::src::opusfile_0_9::src::opusfile::OpusTags,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OggOpusFile {
        pub callbacks: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
        pub stream: *mut libc::c_void,
        pub seekable: i32,
        pub nlinks: i32,
        pub links: *mut crate::internal_h::OggOpusLink,
        pub nserialnos: i32,
        pub cserialnos: i32,
        pub serialnos: *mut crate::config_types_h::ogg_uint32_t,
        pub offset: i64,
        pub end: i64,
        pub oy: crate::ogg_h::ogg_sync_state,
        pub ready_state: i32,
        pub cur_link: i32,
        pub cur_discard_count: crate::opus_types_h::opus_int32,
        pub prev_packet_gp: crate::config_types_h::ogg_int64_t,
        pub prev_page_offset: i64,
        pub bytes_tracked: i64,
        pub samples_tracked: crate::config_types_h::ogg_int64_t,
        pub os: crate::ogg_h::ogg_stream_state,
        pub op: [crate::ogg_h::ogg_packet; 255],
        pub op_pos: i32,
        pub op_count: i32,
        pub od: *mut crate::src::opus_1_2_1::src::opus_multistream_decoder::OpusMSDecoder,
        pub decode_cb: crate::src::opusfile_0_9::src::opusfile::op_decode_cb_func,
        pub decode_cb_ctx: *mut libc::c_void,
        pub od_stream_count: i32,
        pub od_coupled_count: i32,
        pub od_channel_count: i32,
        pub od_mapping: [u8; 8],
        pub od_buffer: *mut crate::internal_h::op_sample,
        pub od_buffer_pos: i32,
        pub od_buffer_size: i32,
        pub gain_type: i32,
        pub gain_offset_q8: crate::opus_types_h::opus_int32,
        pub clip_state: [f32; 8],
        pub dither_a: [f32; 32],
        pub dither_b: [f32; 32],
        pub dither_seed: crate::opus_types_h::opus_uint32,
        pub dither_mute: i32,
        pub dither_disabled: i32,
        pub state_channel_count: i32,
    }
}
pub mod codec_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_info {
        pub version: i32,
        pub channels: i32,
        pub rate: isize,
        pub bitrate_upper: isize,
        pub bitrate_nominal: isize,
        pub bitrate_lower: isize,
        pub bitrate_window: isize,
        pub codec_setup: *mut libc::c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_dsp_state {
        pub analysisp: i32,
        pub vi: *mut crate::codec_h::vorbis_info,
        pub pcm: *mut *mut f32,
        pub pcmret: *mut *mut f32,
        pub pcm_storage: i32,
        pub pcm_current: i32,
        pub pcm_returned: i32,
        pub preextrapolate: i32,
        pub eofflag: i32,
        pub lW: isize,
        pub W: isize,
        pub nW: isize,
        pub centerW: isize,
        pub granulepos: crate::config_types_h::ogg_int64_t,
        pub sequence: crate::config_types_h::ogg_int64_t,
        pub glue_bits: crate::config_types_h::ogg_int64_t,
        pub time_bits: crate::config_types_h::ogg_int64_t,
        pub floor_bits: crate::config_types_h::ogg_int64_t,
        pub res_bits: crate::config_types_h::ogg_int64_t,
        pub backend_state: *mut libc::c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_block {
        pub pcm: *mut *mut f32,
        pub opb: crate::ogg_h::oggpack_buffer,
        pub lW: isize,
        pub W: isize,
        pub nW: isize,
        pub pcmend: i32,
        pub mode: i32,
        pub eofflag: i32,
        pub granulepos: crate::config_types_h::ogg_int64_t,
        pub sequence: crate::config_types_h::ogg_int64_t,
        pub vd: *mut crate::codec_h::vorbis_dsp_state,
        pub localstore: *mut libc::c_void,
        pub localtop: isize,
        pub localalloc: isize,
        pub totaluse: isize,
        pub reap: *mut crate::codec_h::alloc_chain,
        pub glue_bits: isize,
        pub time_bits: isize,
        pub floor_bits: isize,
        pub res_bits: isize,
        pub internal: *mut libc::c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct alloc_chain {
        pub ptr: *mut libc::c_void,
        pub next: *mut crate::codec_h::alloc_chain,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vorbis_comment {
        pub user_comments: *mut *mut libc::c_char,
        pub comment_lengths: *mut i32,
        pub comments: i32,
        pub vendor: *mut libc::c_char,
    }
}
pub mod ogg_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ogg_iovec_t {
        pub iov_base: *mut libc::c_void,
        pub iov_len: crate::stddef_h::size_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct oggpack_buffer {
        pub endbyte: isize,
        pub endbit: i32,
        pub buffer: *mut u8,
        pub ptr: *mut u8,
        pub storage: isize,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ogg_page {
        pub header: *mut u8,
        pub header_len: isize,
        pub body: *mut u8,
        pub body_len: isize,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ogg_stream_state {
        pub body_data: *mut u8,
        pub body_storage: isize,
        pub body_fill: isize,
        pub body_returned: isize,
        pub lacing_vals: *mut i32,
        pub granule_vals: *mut crate::config_types_h::ogg_int64_t,
        pub lacing_storage: isize,
        pub lacing_fill: isize,
        pub lacing_packet: isize,
        pub lacing_returned: isize,
        pub header: [u8; 282],
        pub header_fill: i32,
        pub e_o_s: i32,
        pub b_o_s: i32,
        pub serialno: isize,
        pub pageno: isize,
        pub packetno: crate::config_types_h::ogg_int64_t,
        pub granulepos: crate::config_types_h::ogg_int64_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ogg_packet {
        pub packet: *mut u8,
        pub bytes: isize,
        pub b_o_s: isize,
        pub e_o_s: isize,
        pub granulepos: crate::config_types_h::ogg_int64_t,
        pub packetno: crate::config_types_h::ogg_int64_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ogg_sync_state {
        pub data: *mut u8,
        pub storage: i32,
        pub fill: i32,
        pub returned: i32,
        pub unsynced: i32,
        pub headerbytes: i32,
        pub bodybytes: i32,
    }
}
pub mod config_types_h {
    pub type ogg_int16_t = crate::stdlib::int16_t;

    pub type ogg_int32_t = crate::stdlib::int32_t;

    pub type ogg_uint32_t = crate::stdlib::uint32_t;

    pub type ogg_int64_t = crate::stdlib::int64_t;
}
pub mod alc_h {
    extern "C" {
        pub type ALCdevice_struct;

        pub type ALCcontext_struct;
    }
    pub type ALCdevice = crate::alc_h::ALCdevice_struct;

    pub type ALCcontext = crate::alc_h::ALCcontext_struct;

    pub type ALCboolean = libc::c_char;

    pub type ALCchar = libc::c_char;

    pub type ALCint = i32;

    pub type ALCuint = u32;

    pub type ALCsizei = i32;

    pub type ALCenum = i32;

    pub type ALCvoid = ();

    pub type LPALCCREATECONTEXT = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: *const crate::alc_h::ALCint,
        ) -> *mut crate::alc_h::ALCcontext,
    >;

    pub type LPALCMAKECONTEXTCURRENT =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCcontext) -> crate::alc_h::ALCboolean>;

    pub type LPALCPROCESSCONTEXT =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCcontext) -> ()>;

    pub type LPALCSUSPENDCONTEXT =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCcontext) -> ()>;

    pub type LPALCDESTROYCONTEXT =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCcontext) -> ()>;

    pub type LPALCGETCURRENTCONTEXT =
        Option<unsafe extern "C" fn() -> *mut crate::alc_h::ALCcontext>;

    pub type LPALCGETCONTEXTSDEVICE = Option<
        unsafe extern "C" fn(_: *mut crate::alc_h::ALCcontext) -> *mut crate::alc_h::ALCdevice,
    >;

    pub type LPALCOPENDEVICE = Option<
        unsafe extern "C" fn(_: *const crate::alc_h::ALCchar) -> *mut crate::alc_h::ALCdevice,
    >;

    pub type LPALCCLOSEDEVICE =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCdevice) -> crate::alc_h::ALCboolean>;

    pub type LPALCGETERROR =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCdevice) -> crate::alc_h::ALCenum>;

    pub type LPALCISEXTENSIONPRESENT = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: *const crate::alc_h::ALCchar,
        ) -> crate::alc_h::ALCboolean,
    >;

    pub type LPALCGETPROCADDRESS = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: *const crate::alc_h::ALCchar,
        ) -> *mut libc::c_void,
    >;

    pub type LPALCGETENUMVALUE = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: *const crate::alc_h::ALCchar,
        ) -> crate::alc_h::ALCenum,
    >;

    pub type LPALCGETSTRING = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: crate::alc_h::ALCenum,
        ) -> *const crate::alc_h::ALCchar,
    >;

    pub type LPALCGETINTEGERV = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: crate::alc_h::ALCenum,
            _: crate::alc_h::ALCsizei,
            _: *mut crate::alc_h::ALCint,
        ) -> (),
    >;

    pub type LPALCCAPTUREOPENDEVICE = Option<
        unsafe extern "C" fn(
            _: *const crate::alc_h::ALCchar,
            _: crate::alc_h::ALCuint,
            _: crate::alc_h::ALCenum,
            _: crate::alc_h::ALCsizei,
        ) -> *mut crate::alc_h::ALCdevice,
    >;

    pub type LPALCCAPTURECLOSEDEVICE =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCdevice) -> crate::alc_h::ALCboolean>;

    pub type LPALCCAPTURESTART =
        Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCdevice) -> ()>;

    pub type LPALCCAPTURESTOP = Option<unsafe extern "C" fn(_: *mut crate::alc_h::ALCdevice) -> ()>;

    pub type LPALCCAPTURESAMPLES = Option<
        unsafe extern "C" fn(
            _: *mut crate::alc_h::ALCdevice,
            _: *mut libc::c_void,
            _: crate::alc_h::ALCsizei,
        ) -> (),
    >;
}
pub mod al_h {
    pub type ALboolean = libc::c_char;

    pub type ALchar = libc::c_char;

    pub type ALint = i32;

    pub type ALuint = u32;

    pub type ALsizei = i32;

    pub type ALenum = i32;

    pub type ALfloat = f32;

    pub type ALdouble = f64;

    pub type ALvoid = ();

    pub type LPALENABLE = Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> ()>;

    pub type LPALDISABLE = Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> ()>;

    pub type LPALISENABLED =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> crate::al_h::ALboolean>;

    pub type LPALGETSTRING =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> *const crate::al_h::ALchar>;

    pub type LPALGETBOOLEANV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALboolean) -> ()>;

    pub type LPALGETINTEGERV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALint) -> ()>;

    pub type LPALGETFLOATV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALfloat) -> ()>;

    pub type LPALGETDOUBLEV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALdouble) -> ()>;

    pub type LPALGETBOOLEAN =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> crate::al_h::ALboolean>;

    pub type LPALGETINTEGER =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> crate::al_h::ALint>;

    pub type LPALGETFLOAT =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> crate::al_h::ALfloat>;

    pub type LPALGETDOUBLE =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> crate::al_h::ALdouble>;

    pub type LPALGETERROR = Option<unsafe extern "C" fn() -> crate::al_h::ALenum>;

    pub type LPALISEXTENSIONPRESENT =
        Option<unsafe extern "C" fn(_: *const crate::al_h::ALchar) -> crate::al_h::ALboolean>;

    pub type LPALGETPROCADDRESS =
        Option<unsafe extern "C" fn(_: *const crate::al_h::ALchar) -> *mut libc::c_void>;

    pub type LPALGETENUMVALUE =
        Option<unsafe extern "C" fn(_: *const crate::al_h::ALchar) -> crate::al_h::ALenum>;

    pub type LPALLISTENERF =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: crate::al_h::ALfloat) -> ()>;

    pub type LPALLISTENER3F = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALenum,
            _: crate::al_h::ALfloat,
            _: crate::al_h::ALfloat,
            _: crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALLISTENERFV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *const crate::al_h::ALfloat) -> ()>;

    pub type LPALLISTENERI =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: crate::al_h::ALint) -> ()>;

    pub type LPALGETLISTENERF =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALfloat) -> ()>;

    pub type LPALGETLISTENER3F = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALfloat,
            _: *mut crate::al_h::ALfloat,
            _: *mut crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALGETLISTENERFV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALfloat) -> ()>;

    pub type LPALGETLISTENERI =
        Option<unsafe extern "C" fn(_: crate::al_h::ALenum, _: *mut crate::al_h::ALint) -> ()>;

    pub type LPALGENSOURCES =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *mut crate::al_h::ALuint) -> ()>;

    pub type LPALDELETESOURCES =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALISSOURCE =
        Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> crate::al_h::ALboolean>;

    pub type LPALSOURCEF = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALSOURCE3F = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: crate::al_h::ALfloat,
            _: crate::al_h::ALfloat,
            _: crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALSOURCEFV = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *const crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALSOURCEI = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: crate::al_h::ALint,
        ) -> (),
    >;

    pub type LPALGETSOURCEF = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALGETSOURCE3F = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALfloat,
            _: *mut crate::al_h::ALfloat,
            _: *mut crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALGETSOURCEFV = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALGETSOURCEI = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALint,
        ) -> (),
    >;

    pub type LPALSOURCEPLAYV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCESTOPV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEREWINDV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEPAUSEV =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEPLAY = Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCESTOP = Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEREWIND = Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEPAUSE = Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> ()>;

    pub type LPALSOURCEQUEUEBUFFERS = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALsizei,
            _: *const crate::al_h::ALuint,
        ) -> (),
    >;

    pub type LPALSOURCEUNQUEUEBUFFERS = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALsizei,
            _: *mut crate::al_h::ALuint,
        ) -> (),
    >;

    pub type LPALGENBUFFERS =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *mut crate::al_h::ALuint) -> ()>;

    pub type LPALDELETEBUFFERS =
        Option<unsafe extern "C" fn(_: crate::al_h::ALsizei, _: *const crate::al_h::ALuint) -> ()>;

    pub type LPALISBUFFER =
        Option<unsafe extern "C" fn(_: crate::al_h::ALuint) -> crate::al_h::ALboolean>;

    pub type LPALBUFFERDATA = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *const libc::c_void,
            _: crate::al_h::ALsizei,
            _: crate::al_h::ALsizei,
        ) -> (),
    >;

    pub type LPALGETBUFFERF = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALfloat,
        ) -> (),
    >;

    pub type LPALGETBUFFERI = Option<
        unsafe extern "C" fn(
            _: crate::al_h::ALuint,
            _: crate::al_h::ALenum,
            _: *mut crate::al_h::ALint,
        ) -> (),
    >;

    pub type LPALDOPPLERFACTOR = Option<unsafe extern "C" fn(_: crate::al_h::ALfloat) -> ()>;

    pub type LPALSPEEDOFSOUND = Option<unsafe extern "C" fn(_: crate::al_h::ALfloat) -> ()>;

    pub type LPALDISTANCEMODEL = Option<unsafe extern "C" fn(_: crate::al_h::ALenum) -> ()>;
}
pub mod keys_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct qkey_t {
        pub down: crate::src::qcommon::q_shared::qboolean,
        pub repeats: i32,
        pub binding: *mut libc::c_char,
    }
}
pub mod curlbuild_h {
    pub type curl_off_t = isize;
}
pub mod opus_types_h {
    pub type opus_int16 = crate::stdlib::int16_t;

    pub type opus_uint16 = crate::stdlib::uint16_t;

    pub type opus_int32 = crate::stdlib::int32_t;

    pub type opus_uint32 = crate::stdlib::uint32_t;
}
pub mod client_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clSnapshot_t {
        pub valid: crate::src::qcommon::q_shared::qboolean,
        pub snapFlags: i32,
        pub serverTime: i32,
        pub messageNum: i32,
        pub deltaNum: i32,
        pub ping: i32,
        pub areamask: [crate::src::qcommon::q_shared::byte; 32],
        pub cmdNum: i32,
        pub ps: crate::src::qcommon::q_shared::playerState_t,
        pub numEntities: i32,
        pub parseEntitiesNum: i32,
        pub serverCommandNum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct outPacket_t {
        pub p_cmdNumber: i32,
        pub p_serverTime: i32,
        pub p_realtime: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientActive_t {
        pub timeoutcount: i32,
        pub snap: crate::client_h::clSnapshot_t,
        pub serverTime: i32,
        pub oldServerTime: i32,
        pub oldFrameServerTime: i32,
        pub serverTimeDelta: i32,
        pub extrapolatedSnapshot: crate::src::qcommon::q_shared::qboolean,
        pub newSnapshots: crate::src::qcommon::q_shared::qboolean,
        pub gameState: crate::src::qcommon::q_shared::gameState_t,
        pub mapname: [libc::c_char; 64],
        pub parseEntitiesNum: i32,
        pub mouseDx: [i32; 2],
        pub mouseDy: [i32; 2],
        pub mouseIndex: i32,
        pub joystickAxis: [i32; 16],
        pub cgameUserCmdValue: i32,
        pub cgameSensitivity: f32,
        pub cmds: [crate::src::qcommon::q_shared::usercmd_t; 64],
        pub cmdNumber: i32,
        pub outPackets: [crate::client_h::outPacket_t; 32],
        pub viewangles: crate::src::qcommon::q_shared::vec3_t,
        pub serverId: i32,
        pub snapshots: [crate::client_h::clSnapshot_t; 32],
        pub entityBaselines: [crate::src::qcommon::q_shared::entityState_t; 1024],
        pub parseEntities: [crate::src::qcommon::q_shared::entityState_t; 8192],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientConnection_t {
        pub state: crate::src::qcommon::q_shared::connstate_t,
        pub clientNum: i32,
        pub lastPacketSentTime: i32,
        pub lastPacketTime: i32,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: crate::qcommon_h::netadr_t,
        pub connectTime: i32,
        pub connectPacketCount: i32,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: i32,
        pub checksumFeed: i32,
        pub reliableSequence: i32,
        pub reliableAcknowledge: i32,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: i32,
        pub serverCommandSequence: i32,
        pub lastExecutedServerCommand: i32,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: crate::src::qcommon::q_shared::fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: crate::src::qcommon::q_shared::qboolean,
        pub cURLUsed: crate::src::qcommon::q_shared::qboolean,
        pub cURLDisconnected: crate::src::qcommon::q_shared::qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: i32,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: i32,
        pub downloadBlock: i32,
        pub downloadCount: i32,
        pub downloadSize: i32,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: crate::src::qcommon::q_shared::qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: crate::src::qcommon::q_shared::qboolean,
        pub demorecording: crate::src::qcommon::q_shared::qboolean,
        pub demoplaying: crate::src::qcommon::q_shared::qboolean,
        pub demowaiting: crate::src::qcommon::q_shared::qboolean,
        pub firstDemoFrameSkipped: crate::src::qcommon::q_shared::qboolean,
        pub demofile: crate::src::qcommon::q_shared::fileHandle_t,
        pub timeDemoFrames: i32,
        pub timeDemoStart: i32,
        pub timeDemoBaseTime: i32,
        pub timeDemoLastFrame: i32,
        pub timeDemoMinDuration: i32,
        pub timeDemoMaxDuration: i32,
        pub timeDemoDurations: [u8; 4096],
        pub aviVideoFrameRemainder: f32,
        pub aviSoundFrameRemainder: f32,
        pub voipEnabled: crate::src::qcommon::q_shared::qboolean,
        pub voipCodecInitialized: crate::src::qcommon::q_shared::qboolean,
        pub opusDecoder: [*mut crate::src::opus_1_2_1::src::opus_decoder::OpusDecoder; 64],
        pub voipIncomingGeneration: [crate::src::qcommon::q_shared::byte; 64],
        pub voipIncomingSequence: [i32; 64],
        pub voipGain: [f32; 64],
        pub voipIgnore: [crate::src::qcommon::q_shared::qboolean; 64],
        pub voipMuteAll: crate::src::qcommon::q_shared::qboolean,
        pub voipTargets: [crate::stdlib::uint8_t; 8],
        pub voipFlags: crate::stdlib::uint8_t,
        pub opusEncoder: *mut crate::src::opus_1_2_1::src::opus_encoder::OpusEncoder,
        pub voipOutgoingDataSize: i32,
        pub voipOutgoingDataFrames: i32,
        pub voipOutgoingSequence: i32,
        pub voipOutgoingGeneration: crate::src::qcommon::q_shared::byte,
        pub voipOutgoingData: [crate::src::qcommon::q_shared::byte; 1024],
        pub voipPower: f32,
        pub compat: crate::src::qcommon::q_shared::qboolean,
        pub netchan: crate::qcommon_h::netchan_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ping_t {
        pub adr: crate::qcommon_h::netadr_t,
        pub start: i32,
        pub time: i32,
        pub info: [libc::c_char; 1024],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct serverInfo_t {
        pub adr: crate::qcommon_h::netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: i32,
        pub gameType: i32,
        pub clients: i32,
        pub maxClients: i32,
        pub minPing: i32,
        pub maxPing: i32,
        pub ping: i32,
        pub visible: crate::src::qcommon::q_shared::qboolean,
        pub punkbuster: i32,
        pub g_humanplayers: i32,
        pub g_needpass: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientStatic_t {
        pub cddialog: crate::src::qcommon::q_shared::qboolean,
        pub rendererStarted: crate::src::qcommon::q_shared::qboolean,
        pub soundStarted: crate::src::qcommon::q_shared::qboolean,
        pub soundRegistered: crate::src::qcommon::q_shared::qboolean,
        pub uiStarted: crate::src::qcommon::q_shared::qboolean,
        pub cgameStarted: crate::src::qcommon::q_shared::qboolean,
        pub framecount: i32,
        pub frametime: i32,
        pub realtime: i32,
        pub realFrametime: i32,
        pub numlocalservers: i32,
        pub localServers: [crate::client_h::serverInfo_t; 128],
        pub numglobalservers: i32,
        pub globalServers: [crate::client_h::serverInfo_t; 4096],
        pub numGlobalServerAddresses: i32,
        pub globalServerAddresses: [crate::qcommon_h::netadr_t; 4096],
        pub numfavoriteservers: i32,
        pub favoriteServers: [crate::client_h::serverInfo_t; 128],
        pub pingUpdateSource: i32,
        pub updateServer: crate::qcommon_h::netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: crate::qcommon_h::netadr_t,
        pub rconAddress: crate::qcommon_h::netadr_t,
        pub glconfig: crate::tr_types_h::glconfig_t,
        pub charSetShader: crate::src::qcommon::q_shared::qhandle_t,
        pub whiteShader: crate::src::qcommon::q_shared::qhandle_t,
        pub consoleShader: crate::src::qcommon::q_shared::qhandle_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct kbutton_t {
        pub down: [i32; 2],
        pub downtime: u32,
        pub msec: u32,
        pub active: crate::src::qcommon::q_shared::qboolean,
        pub wasPressed: crate::src::qcommon::q_shared::qboolean,
    }
}
pub mod tr_public_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refexport_t {
        pub Shutdown:
            Option<unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> ()>,
        pub BeginRegistration:
            Option<unsafe extern "C" fn(_: *mut crate::tr_types_h::glconfig_t) -> ()>,
        pub RegisterModel: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
        >,
        pub RegisterSkin: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
        >,
        pub RegisterShader: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
        >,
        pub RegisterShaderNoMip: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
            ) -> crate::src::qcommon::q_shared::qhandle_t,
        >,
        pub LoadWorld: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
        pub SetWorldVisData:
            Option<unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::byte) -> ()>,
        pub EndRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub ClearScene: Option<unsafe extern "C" fn() -> ()>,
        pub AddRefEntityToScene:
            Option<unsafe extern "C" fn(_: *const crate::tr_types_h::refEntity_t) -> ()>,
        pub AddPolyToScene: Option<
            unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: i32,
                _: *const crate::tr_types_h::polyVert_t,
                _: i32,
            ) -> (),
        >,
        pub LightForPoint: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
        pub AddLightToScene: Option<
            unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
            ) -> (),
        >,
        pub AddAdditiveLightToScene: Option<
            unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
            ) -> (),
        >,
        pub RenderScene: Option<unsafe extern "C" fn(_: *const crate::tr_types_h::refdef_t) -> ()>,
        pub SetColor: Option<unsafe extern "C" fn(_: *const f32) -> ()>,
        pub DrawStretchPic: Option<
            unsafe extern "C" fn(
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: f32,
                _: crate::src::qcommon::q_shared::qhandle_t,
            ) -> (),
        >,
        pub DrawStretchRaw: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: i32,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
        >,
        pub UploadCinematic: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: i32,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
        >,
        pub BeginFrame: Option<unsafe extern "C" fn(_: crate::tr_types_h::stereoFrame_t) -> ()>,
        pub EndFrame: Option<unsafe extern "C" fn(_: *mut i32, _: *mut i32) -> ()>,
        pub MarkFragments: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec3_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::markFragment_t,
            ) -> i32,
        >,
        pub LerpTag: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::orientation_t,
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: i32,
                _: i32,
                _: f32,
                _: *const libc::c_char,
            ) -> i32,
        >,
        pub ModelBounds: Option<
            unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::qhandle_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub RegisterFont: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::fontInfo_t,
            ) -> (),
        >,
        pub RemapShader: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> (),
        >,
        pub GetEntityToken: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: i32,
            ) -> crate::src::qcommon::q_shared::qboolean,
        >,
        pub inPVS: Option<
            unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
            ) -> crate::src::qcommon::q_shared::qboolean,
        >,
        pub TakeVideoFrame: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::byte,
                _: *mut crate::src::qcommon::q_shared::byte,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
        >,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refimport_t {
        pub Printf: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char, _: ...) -> ()>,
        pub Error: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char, _: ...) -> !>,
        pub Milliseconds: Option<unsafe extern "C" fn() -> i32>,
        pub Hunk_Alloc: Option<
            unsafe extern "C" fn(
                _: i32,
                _: crate::src::qcommon::q_shared::ha_pref,
            ) -> *mut libc::c_void,
        >,
        pub Hunk_AllocateTempMemory: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
        pub Hunk_FreeTempMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub Malloc: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
        pub Free: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub Cvar_Get: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: i32,
            ) -> *mut crate::src::qcommon::q_shared::cvar_t,
        >,
        pub Cvar_Set:
            Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
        pub Cvar_SetValue: Option<unsafe extern "C" fn(_: *const libc::c_char, _: f32) -> ()>,
        pub Cvar_CheckRange: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::cvar_t,
                _: f32,
                _: f32,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> (),
        >,
        pub Cvar_SetDescription: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::cvar_t,
                _: *const libc::c_char,
            ) -> (),
        >,
        pub Cvar_VariableIntegerValue: Option<unsafe extern "C" fn(_: *const libc::c_char) -> i32>,
        pub Cmd_AddCommand: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: Option<unsafe extern "C" fn() -> ()>,
            ) -> (),
        >,
        pub Cmd_RemoveCommand: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
        pub Cmd_Argc: Option<unsafe extern "C" fn() -> i32>,
        pub Cmd_Argv: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_char>,
        pub Cmd_ExecuteText: Option<unsafe extern "C" fn(_: i32, _: *const libc::c_char) -> ()>,
        pub CM_ClusterPVS:
            Option<unsafe extern "C" fn(_: i32) -> *mut crate::src::qcommon::q_shared::byte>,
        pub CM_DrawDebugSurface: Option<
            unsafe extern "C" fn(
                _: Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> ()>,
            ) -> (),
        >,
        pub FS_FileIsInPAK:
            Option<unsafe extern "C" fn(_: *const libc::c_char, _: *mut i32) -> i32>,
        pub FS_ReadFile: Option<
            unsafe extern "C" fn(_: *const libc::c_char, _: *mut *mut libc::c_void) -> isize,
        >,
        pub FS_FreeFile: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub FS_ListFiles: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *const libc::c_char,
                _: *mut i32,
            ) -> *mut *mut libc::c_char,
        >,
        pub FS_FreeFileList: Option<unsafe extern "C" fn(_: *mut *mut libc::c_char) -> ()>,
        pub FS_WriteFile: Option<
            unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_void, _: i32) -> (),
        >,
        pub FS_FileExists: Option<
            unsafe extern "C" fn(_: *const libc::c_char) -> crate::src::qcommon::q_shared::qboolean,
        >,
        pub CIN_UploadCinematic: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub CIN_PlayCinematic: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
            ) -> i32,
        >,
        pub CIN_RunCinematic:
            Option<unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::e_status>,
        pub CL_WriteAVIVideoFrame: Option<
            unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::byte, _: i32) -> (),
        >,
        pub IN_Init: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub IN_Shutdown: Option<unsafe extern "C" fn() -> ()>,
        pub IN_Restart: Option<unsafe extern "C" fn() -> ()>,
        pub ftol: Option<unsafe extern "C" fn(_: f32) -> isize>,
        pub Sys_SetEnv:
            Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
        pub Sys_GLimpSafeInit: Option<unsafe extern "C" fn() -> ()>,
        pub Sys_GLimpInit: Option<unsafe extern "C" fn() -> ()>,
        pub Sys_LowPhysicalMemory:
            Option<unsafe extern "C" fn() -> crate::src::qcommon::q_shared::qboolean>,
    }

    pub type GetRefAPI_t = Option<
        unsafe extern "C" fn(
            _: i32,
            _: *mut crate::tr_public_h::refimport_t,
        ) -> *mut crate::tr_public_h::refexport_t,
    >;
}
pub mod stddef_h {
    pub type ptrdiff_t = isize;

    pub type size_t = usize;

    pub type wchar_t = i32;
}
pub mod stdarg_h {
    pub type va_list = crate::internal::__builtin_va_list;
}
pub mod internal {
    pub type __builtin_va_list = [crate::internal::__va_list_tag; 1];

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __va_list_tag {
        pub gp_offset: u32,
        pub fp_offset: u32,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
pub mod aasfile_h {
    pub type aas_bbox_t = crate::aasfile_h::aas_bbox_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_bbox_s {
        pub presencetype: i32,
        pub flags: i32,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
    }
    //============ settings ===========
    //reachability to another area

    pub type aas_reachability_t = crate::aasfile_h::aas_reachability_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reachability_s {
        pub areanum: i32,
        pub facenum: i32,
        pub edgenum: i32,
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub end: crate::src::qcommon::q_shared::vec3_t,
        pub traveltype: i32,
        pub traveltime: u16,
    }
    //number of the reachable area
    //number of the face towards the other area
    //number of the edge towards the other area
    //start point of inter area movement
    //end point of inter area movement
    //type of travel required to get to the area
    //travel time of the inter area movement
    //area settings

    pub type aas_areasettings_t = crate::aasfile_h::aas_areasettings_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_areasettings_s {
        pub contents: i32,
        pub areaflags: i32,
        pub presencetype: i32,
        pub cluster: i32,
        pub clusterareanum: i32,
        pub numreachableareas: i32,
        pub firstreachablearea: i32,
    }
    //could also add all kind of statistic fields
    //contents of the area
    //several area flags
    //how a bot can be present in this area
    //cluster the area belongs to, if negative it's a portal
    //number of the area in the cluster
    //number of reachable areas from this one
    //first reachable area in the reachable area index
    //cluster portal

    pub type aas_portal_t = crate::aasfile_h::aas_portal_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_portal_s {
        pub areanum: i32,
        pub frontcluster: i32,
        pub backcluster: i32,
        pub clusterareanum: [i32; 2],
    }
    //area that is the actual portal
    //cluster at front of portal
    //cluster at back of portal
    //number of the area in the front and back cluster
    //cluster portal index

    pub type aas_portalindex_t = i32;
    //cluster

    pub type aas_cluster_t = crate::aasfile_h::aas_cluster_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_cluster_s {
        pub numareas: i32,
        pub numreachabilityareas: i32,
        pub numportals: i32,
        pub firstportal: i32,
    }
    //number of areas in the cluster
    //number of areas with reachabilities
    //number of cluster portals
    //first cluster portal in the index
    //============ 3d definition ============

    pub type aas_vertex_t = crate::src::qcommon::q_shared::vec3_t;
    //just a plane in the third dimension

    pub type aas_plane_t = crate::aasfile_h::aas_plane_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_plane_s {
        pub normal: crate::src::qcommon::q_shared::vec3_t,
        pub dist: f32,
        pub type_0: i32,
    }
    //normal vector of the plane
    //distance of the plane (normal vector * distance = point in plane)
    //edge

    pub type aas_edge_t = crate::aasfile_h::aas_edge_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_edge_s {
        pub v: [i32; 2],
    }
    //numbers of the vertexes of this edge
    //edge index, negative if vertexes are reversed

    pub type aas_edgeindex_t = i32;
    //a face bounds an area, often it will also separate two areas

    pub type aas_face_t = crate::aasfile_h::aas_face_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_face_s {
        pub planenum: i32,
        pub faceflags: i32,
        pub numedges: i32,
        pub firstedge: i32,
        pub frontarea: i32,
        pub backarea: i32,
    }
    //number of the plane this face is in
    //face flags (no use to create face settings for just this field)
    //number of edges in the boundary of the face
    //first edge in the edge index
    //area at the front of this face
    //area at the back of this face
    //face index, stores a negative index if backside of face

    pub type aas_faceindex_t = i32;
    //area with a boundary of faces

    pub type aas_area_t = crate::aasfile_h::aas_area_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_area_s {
        pub areanum: i32,
        pub numfaces: i32,
        pub firstface: i32,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub center: crate::src::qcommon::q_shared::vec3_t,
    }
    //number of this area
    //3d definition
    //number of faces used for the boundary of the area
    //first face in the face index used for the boundary of the area
    //mins of the area
    //maxs of the area
    //'center' of the area
    //nodes of the bsp tree

    pub type aas_node_t = crate::aasfile_h::aas_node_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_node_s {
        pub planenum: i32,
        pub children: [i32; 2],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_lump_t {
        pub fileofs: i32,
        pub filelen: i32,
    }
    //child nodes of this node, or areas as leaves when negative
    //when a child is zero it's a solid leaf
    //aas file header

    pub type aas_header_t = crate::aasfile_h::aas_header_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_header_s {
        pub ident: i32,
        pub version: i32,
        pub bspchecksum: i32,
        pub lumps: [crate::aasfile_h::aas_lump_t; 14],
    }
}
pub mod be_aas_def_h {
    pub type aas_link_t = crate::be_aas_def_h::aas_link_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_link_s {
        pub entnum: i32,
        pub areanum: i32,
        pub next_ent: *mut crate::be_aas_def_h::aas_link_s,
        pub prev_ent: *mut crate::be_aas_def_h::aas_link_s,
        pub next_area: *mut crate::be_aas_def_h::aas_link_s,
        pub prev_area: *mut crate::be_aas_def_h::aas_link_s,
    }
    //structure to link entities to leaves and leaves to entities

    pub type bsp_link_t = crate::be_aas_def_h::bsp_link_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_link_s {
        pub entnum: i32,
        pub leafnum: i32,
        pub next_ent: *mut crate::be_aas_def_h::bsp_link_s,
        pub prev_ent: *mut crate::be_aas_def_h::bsp_link_s,
        pub next_leaf: *mut crate::be_aas_def_h::bsp_link_s,
        pub prev_leaf: *mut crate::be_aas_def_h::bsp_link_s,
    }

    pub type bsp_entdata_t = crate::be_aas_def_h::bsp_entdata_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_entdata_s {
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub absmins: crate::src::qcommon::q_shared::vec3_t,
        pub absmaxs: crate::src::qcommon::q_shared::vec3_t,
        pub solid: i32,
        pub modelnum: i32,
    }
    //entity

    pub type aas_entity_t = crate::be_aas_def_h::aas_entity_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_entity_s {
        pub i: crate::be_aas_h::aas_entityinfo_t,
        pub areas: *mut crate::be_aas_def_h::aas_link_t,
        pub leaves: *mut crate::be_aas_def_h::bsp_link_t,
    }

    pub type aas_settings_t = crate::be_aas_def_h::aas_settings_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_settings_s {
        pub phys_gravitydirection: crate::src::qcommon::q_shared::vec3_t,
        pub phys_friction: f32,
        pub phys_stopspeed: f32,
        pub phys_gravity: f32,
        pub phys_waterfriction: f32,
        pub phys_watergravity: f32,
        pub phys_maxvelocity: f32,
        pub phys_maxwalkvelocity: f32,
        pub phys_maxcrouchvelocity: f32,
        pub phys_maxswimvelocity: f32,
        pub phys_walkaccelerate: f32,
        pub phys_airaccelerate: f32,
        pub phys_swimaccelerate: f32,
        pub phys_maxstep: f32,
        pub phys_maxsteepness: f32,
        pub phys_maxwaterjump: f32,
        pub phys_maxbarrier: f32,
        pub phys_jumpvel: f32,
        pub phys_falldelta5: f32,
        pub phys_falldelta10: f32,
        pub rs_waterjump: f32,
        pub rs_teleport: f32,
        pub rs_barrierjump: f32,
        pub rs_startcrouch: f32,
        pub rs_startgrapple: f32,
        pub rs_startwalkoffledge: f32,
        pub rs_startjump: f32,
        pub rs_rocketjump: f32,
        pub rs_bfgjump: f32,
        pub rs_jumppad: f32,
        pub rs_aircontrolledjumppad: f32,
        pub rs_funcbob: f32,
        pub rs_startelevator: f32,
        pub rs_falldamage5: f32,
        pub rs_falldamage10: f32,
        pub rs_maxfallheight: f32,
        pub rs_maxjumpfallheight: f32,
    }
    //entity info
    //links into the AAS areas
    //links into the BSP leaves
    //routing cache

    pub type aas_routingcache_t = crate::be_aas_def_h::aas_routingcache_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_routingcache_s {
        pub type_0: crate::src::qcommon::q_shared::byte,
        pub time: f32,
        pub size: i32,
        pub cluster: i32,
        pub areanum: i32,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub starttraveltime: f32,
        pub travelflags: i32,
        pub prev: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub next: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub time_prev: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub time_next: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub reachabilities: *mut u8,
        pub traveltimes: [u16; 1],
    }
    //portal or area cache
    //last time accessed or updated
    //size of the routing cache
    //cluster the cache is for
    //area the cache is created for
    //origin within the area
    //travel time to start with
    //combinations of the travel flags
    //reachabilities used for routing
    //travel time for every area (variable sized)
    //fields for the routing algorithm

    pub type aas_routingupdate_t = crate::be_aas_def_h::aas_routingupdate_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_routingupdate_s {
        pub cluster: i32,
        pub areanum: i32,
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub tmptraveltime: u16,
        pub areatraveltimes: *mut u16,
        pub inlist: crate::src::qcommon::q_shared::qboolean,
        pub next: *mut crate::be_aas_def_h::aas_routingupdate_s,
        pub prev: *mut crate::be_aas_def_h::aas_routingupdate_s,
    }
    //area number of the update
    //start point the area was entered
    //temporary travel time
    //travel times within the area
    //true if the update is in the list
    //reversed reachability link

    pub type aas_reversedlink_t = crate::be_aas_def_h::aas_reversedlink_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reversedlink_s {
        pub linknum: i32,
        pub areanum: i32,
        pub next: *mut crate::be_aas_def_h::aas_reversedlink_s,
    }
    //the aas_areareachability_t
    //reachable from this area
    //next link
    //reversed area reachability

    pub type aas_reversedreachability_t = crate::be_aas_def_h::aas_reversedreachability_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reversedreachability_s {
        pub numlinks: i32,
        pub first: *mut crate::be_aas_def_h::aas_reversedlink_t,
    }
    //areas a reachability goes through

    pub type aas_reachabilityareas_t = crate::be_aas_def_h::aas_reachabilityareas_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reachabilityareas_s {
        pub firstarea: i32,
        pub numareas: i32,
    }

    pub type aas_t = crate::be_aas_def_h::aas_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_s {
        pub loaded: i32,
        pub initialized: i32,
        pub savefile: i32,
        pub bspchecksum: i32,
        pub time: f32,
        pub numframes: i32,
        pub filename: [libc::c_char; 64],
        pub mapname: [libc::c_char; 64],
        pub numbboxes: i32,
        pub bboxes: *mut crate::aasfile_h::aas_bbox_t,
        pub numvertexes: i32,
        pub vertexes: *mut crate::aasfile_h::aas_vertex_t,
        pub numplanes: i32,
        pub planes: *mut crate::aasfile_h::aas_plane_t,
        pub numedges: i32,
        pub edges: *mut crate::aasfile_h::aas_edge_t,
        pub edgeindexsize: i32,
        pub edgeindex: *mut crate::aasfile_h::aas_edgeindex_t,
        pub numfaces: i32,
        pub faces: *mut crate::aasfile_h::aas_face_t,
        pub faceindexsize: i32,
        pub faceindex: *mut crate::aasfile_h::aas_faceindex_t,
        pub numareas: i32,
        pub areas: *mut crate::aasfile_h::aas_area_t,
        pub numareasettings: i32,
        pub areasettings: *mut crate::aasfile_h::aas_areasettings_t,
        pub reachabilitysize: i32,
        pub reachability: *mut crate::aasfile_h::aas_reachability_t,
        pub numnodes: i32,
        pub nodes: *mut crate::aasfile_h::aas_node_t,
        pub numportals: i32,
        pub portals: *mut crate::aasfile_h::aas_portal_t,
        pub portalindexsize: i32,
        pub portalindex: *mut crate::aasfile_h::aas_portalindex_t,
        pub numclusters: i32,
        pub clusters: *mut crate::aasfile_h::aas_cluster_t,
        pub numreachabilityareas: i32,
        pub reachabilitytime: f32,
        pub linkheap: *mut crate::be_aas_def_h::aas_link_t,
        pub linkheapsize: i32,
        pub freelinks: *mut crate::be_aas_def_h::aas_link_t,
        pub arealinkedentities: *mut *mut crate::be_aas_def_h::aas_link_t,
        pub maxentities: i32,
        pub maxclients: i32,
        pub entities: *mut crate::be_aas_def_h::aas_entity_t,
        pub travelflagfortype: [i32; 32],
        pub areacontentstravelflags: *mut i32,
        pub areaupdate: *mut crate::be_aas_def_h::aas_routingupdate_t,
        pub portalupdate: *mut crate::be_aas_def_h::aas_routingupdate_t,
        pub frameroutingupdates: i32,
        pub reversedreachability: *mut crate::be_aas_def_h::aas_reversedreachability_t,
        pub areatraveltimes: *mut *mut *mut u16,
        pub clusterareacache: *mut *mut *mut crate::be_aas_def_h::aas_routingcache_t,
        pub portalcache: *mut *mut crate::be_aas_def_h::aas_routingcache_t,
        pub oldestcache: *mut crate::be_aas_def_h::aas_routingcache_t,
        pub newestcache: *mut crate::be_aas_def_h::aas_routingcache_t,
        pub portalmaxtraveltimes: *mut i32,
        pub reachabilityareaindex: *mut i32,
        pub reachabilityareas: *mut crate::be_aas_def_h::aas_reachabilityareas_t,
    }
}
pub mod botlib_h {
    pub type bot_input_t = crate::botlib_h::bot_input_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_input_s {
        pub thinktime: f32,
        pub dir: crate::src::qcommon::q_shared::vec3_t,
        pub speed: f32,
        pub viewangles: crate::src::qcommon::q_shared::vec3_t,
        pub actionflags: i32,
        pub weapon: i32,
    }
    //bsp_trace_t hit surface

    pub type bsp_surface_t = crate::botlib_h::bsp_surface_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: i32,
        pub value: i32,
    }
    //remove the bsp_trace_s structure definition l8r on
    //a trace is returned when a box is swept through the world

    pub type bsp_trace_t = crate::botlib_h::bsp_trace_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_trace_s {
        pub allsolid: crate::src::qcommon::q_shared::qboolean,
        pub startsolid: crate::src::qcommon::q_shared::qboolean,
        pub fraction: f32,
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub plane: crate::src::qcommon::q_shared::cplane_t,
        pub exp_dist: f32,
        pub sidenum: i32,
        pub surface: crate::botlib_h::bsp_surface_t,
        pub contents: i32,
        pub ent: i32,
    }
    // if true, plane is not valid
    // if true, the initial point was in a solid area
    // time completed, 1.0 = didn't hit anything
    // final position
    // surface normal at impact
    // expanded plane distance
    // number of the brush side hit
    // the hit point surface
    // contents on other side of surface hit
    // number of entity hit
    // BSPTRACE
    //entity state

    pub type bot_entitystate_t = crate::botlib_h::bot_entitystate_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_entitystate_s {
        pub type_0: i32,
        pub flags: i32,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub old_origin: crate::src::qcommon::q_shared::vec3_t,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub groundent: i32,
        pub solid: i32,
        pub modelindex: i32,
        pub modelindex2: i32,
        pub frame: i32,
        pub event: i32,
        pub eventParm: i32,
        pub powerups: i32,
        pub weapon: i32,
        pub legsAnim: i32,
        pub torsoAnim: i32,
    }
    // if true, plane is not valid
    // if true, the initial point was in a solid area
    // time completed, 1.0 = didn't hit anything
    // final position
    // surface normal at impact
    // expanded plane distance
    // number of the brush side hit
    // the hit point surface
    // contents on other side of surface hit
    // number of entity hit
    //bot AI library exported functions

    pub type botlib_import_t = crate::botlib_h::botlib_import_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: ...) -> ()>,
        pub Trace: Option<
            unsafe extern "C" fn(
                _: *mut crate::botlib_h::bsp_trace_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
            ) -> (),
        >,
        pub EntityTrace: Option<
            unsafe extern "C" fn(
                _: *mut crate::botlib_h::bsp_trace_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
            ) -> (),
        >,
        pub PointContents:
            Option<unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32>,
        pub inPVS: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
        pub BSPEntityData: Option<unsafe extern "C" fn() -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> i32>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: i32) -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *mut crate::src::qcommon::q_shared::fileHandle_t,
                _: crate::src::qcommon::q_shared::fsMode_t,
            ) -> i32,
        >,
        pub FS_Read: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: i32,
                _: crate::src::qcommon::q_shared::fileHandle_t,
            ) -> i32,
        >,
        pub FS_Write: Option<
            unsafe extern "C" fn(
                _: *const libc::c_void,
                _: i32,
                _: crate::src::qcommon::q_shared::fileHandle_t,
            ) -> i32,
        >,
        pub FS_FCloseFile:
            Option<unsafe extern "C" fn(_: crate::src::qcommon::q_shared::fileHandle_t) -> ()>,
        pub FS_Seek: Option<
            unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::fileHandle_t,
                _: isize,
                _: i32,
            ) -> i32,
        >,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> i32>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub DebugLineShow: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
            ) -> (),
        >,
        pub DebugPolygonCreate: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
            ) -> i32,
        >,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: i32) -> ()>,
    }

    pub type aas_export_t = crate::botlib_h::aas_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_export_s {
        pub AAS_EntityInfo:
            Option<unsafe extern "C" fn(_: i32, _: *mut crate::be_aas_h::aas_entityinfo_s) -> ()>,
        pub AAS_Initialized: Option<unsafe extern "C" fn() -> i32>,
        pub AAS_PresenceTypeBoundingBox: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub AAS_Time: Option<unsafe extern "C" fn() -> f32>,
        pub AAS_PointAreaNum:
            Option<unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32>,
        pub AAS_PointReachabilityAreaIndex:
            Option<unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32>,
        pub AAS_TraceAreas: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
                _: i32,
            ) -> i32,
        >,
        pub AAS_BBoxAreas: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
            ) -> i32,
        >,
        pub AAS_AreaInfo:
            Option<unsafe extern "C" fn(_: i32, _: *mut crate::be_aas_h::aas_areainfo_s) -> i32>,
        pub AAS_PointContents:
            Option<unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32>,
        pub AAS_NextBSPEntity: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub AAS_ValueForBSPEpairKey: Option<
            unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: *mut libc::c_char, _: i32) -> i32,
        >,
        pub AAS_VectorForBSPEpairKey: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut libc::c_char,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
        pub AAS_FloatForBSPEpairKey:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: *mut f32) -> i32>,
        pub AAS_IntForBSPEpairKey:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: *mut i32) -> i32>,
        pub AAS_AreaReachability: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub AAS_AreaTravelTimeToGoalArea: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
            ) -> i32,
        >,
        pub AAS_EnableRoutingArea: Option<unsafe extern "C" fn(_: i32, _: i32) -> i32>,
        pub AAS_PredictRoute: Option<
            unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_predictroute_s,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
            ) -> i32,
        >,
        pub AAS_AlternativeRouteGoals: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: *mut crate::be_aas_h::aas_altroutegoal_s,
                _: i32,
                _: i32,
            ) -> i32,
        >,
        pub AAS_Swimming:
            Option<unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32>,
        pub AAS_PredictClientMovement: Option<
            unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_clientmove_s,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: f32,
                _: i32,
                _: i32,
                _: i32,
            ) -> i32,
        >,
    }

    pub type ea_export_t = crate::botlib_h::ea_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ea_export_s {
        pub EA_Command: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ()>,
        pub EA_Say: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ()>,
        pub EA_SayTeam: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ()>,
        pub EA_Action: Option<unsafe extern "C" fn(_: i32, _: i32) -> ()>,
        pub EA_Gesture: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Talk: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Attack: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Use: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Respawn: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveUp: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveDown: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveForward: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveBack: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveLeft: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_MoveRight: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Crouch: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_SelectWeapon: Option<unsafe extern "C" fn(_: i32, _: i32) -> ()>,
        pub EA_Jump: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_DelayedJump: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub EA_Move: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
            ) -> (),
        >,
        pub EA_View: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::qcommon::q_shared::vec_t) -> (),
        >,
        pub EA_EndRegular: Option<unsafe extern "C" fn(_: i32, _: f32) -> ()>,
        pub EA_GetInput: Option<
            unsafe extern "C" fn(_: i32, _: f32, _: *mut crate::botlib_h::bot_input_t) -> (),
        >,
        pub EA_ResetInput: Option<unsafe extern "C" fn(_: i32) -> ()>,
    }

    pub type ai_export_t = crate::botlib_h::ai_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ai_export_s {
        pub BotLoadCharacter: Option<unsafe extern "C" fn(_: *mut libc::c_char, _: f32) -> i32>,
        pub BotFreeCharacter: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub Characteristic_Float: Option<unsafe extern "C" fn(_: i32, _: i32) -> f32>,
        pub Characteristic_BFloat:
            Option<unsafe extern "C" fn(_: i32, _: i32, _: f32, _: f32) -> f32>,
        pub Characteristic_Integer: Option<unsafe extern "C" fn(_: i32, _: i32) -> i32>,
        pub Characteristic_BInteger:
            Option<unsafe extern "C" fn(_: i32, _: i32, _: i32, _: i32) -> i32>,
        pub Characteristic_String:
            Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut libc::c_char, _: i32) -> ()>,
        pub BotAllocChatState: Option<unsafe extern "C" fn() -> i32>,
        pub BotFreeChatState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotQueueConsoleMessage:
            Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut libc::c_char) -> ()>,
        pub BotRemoveConsoleMessage: Option<unsafe extern "C" fn(_: i32, _: i32) -> ()>,
        pub BotNextConsoleMessage: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
            ) -> i32,
        >,
        pub BotNumConsoleMessages: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub BotInitialChat: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut libc::c_char,
                _: i32,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
            ) -> (),
        >,
        pub BotNumInitialChats: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> i32>,
        pub BotReplyChat: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut libc::c_char,
                _: i32,
                _: i32,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
            ) -> i32,
        >,
        pub BotChatLength: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub BotEnterChat: Option<unsafe extern "C" fn(_: i32, _: i32, _: i32) -> ()>,
        pub BotGetChatMessage:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: i32) -> ()>,
        pub StringContains:
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: *mut libc::c_char, _: i32) -> i32>,
        pub BotFindMatch: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_chat::bot_match_s,
                _: usize,
            ) -> i32,
        >,
        pub BotMatchVariable: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_chat::bot_match_s,
                _: i32,
                _: *mut libc::c_char,
                _: i32,
            ) -> (),
        >,
        pub UnifyWhiteSpaces: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
        pub BotReplaceSynonyms: Option<unsafe extern "C" fn(_: *mut libc::c_char, _: usize) -> ()>,
        pub BotLoadChatFile:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: *mut libc::c_char) -> i32>,
        pub BotSetChatGender: Option<unsafe extern "C" fn(_: i32, _: i32) -> ()>,
        pub BotSetChatName:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: i32) -> ()>,
        pub BotResetGoalState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotResetAvoidGoals: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotRemoveFromAvoidGoals: Option<unsafe extern "C" fn(_: i32, _: i32) -> ()>,
        pub BotPushGoal: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::botlib::be_ai_goal::bot_goal_s) -> (),
        >,
        pub BotPopGoal: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotEmptyGoalStack: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotDumpAvoidGoals: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotDumpGoalStack: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotGoalName: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: i32) -> ()>,
        pub BotGetTopGoal: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::botlib::be_ai_goal::bot_goal_s) -> i32,
        >,
        pub BotGetSecondGoal: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::botlib::be_ai_goal::bot_goal_s) -> i32,
        >,
        pub BotChooseLTGItem: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
            ) -> i32,
        >,
        pub BotChooseNBGItem: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: f32,
            ) -> i32,
        >,
        pub BotTouchingGoal: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> i32,
        >,
        pub BotItemGoalInVisButNotVisible: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> i32,
        >,
        pub BotGetLevelItemGoal: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> i32,
        >,
        pub BotGetNextCampSpotGoal: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::botlib::be_ai_goal::bot_goal_s) -> i32,
        >,
        pub BotGetMapLocationGoal: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> i32,
        >,
        pub BotAvoidGoalTime: Option<unsafe extern "C" fn(_: i32, _: i32) -> f32>,
        pub BotSetAvoidGoalTime: Option<unsafe extern "C" fn(_: i32, _: i32, _: f32) -> ()>,
        pub BotInitLevelItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotUpdateEntityItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotLoadItemWeights: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> i32>,
        pub BotFreeItemWeights: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotInterbreedGoalFuzzyLogic: Option<unsafe extern "C" fn(_: i32, _: i32, _: i32) -> ()>,
        pub BotSaveGoalFuzzyLogic: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> ()>,
        pub BotMutateGoalFuzzyLogic: Option<unsafe extern "C" fn(_: i32, _: f32) -> ()>,
        pub BotAllocGoalState: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub BotFreeGoalState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotResetMoveState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotMoveToGoal: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_move::bot_moveresult_s,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: i32,
            ) -> (),
        >,
        pub BotMoveInDirection: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: i32,
            ) -> i32,
        >,
        pub BotResetAvoidReach: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotResetLastAvoidReach: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotReachabilityArea: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t, _: i32) -> i32,
        >,
        pub BotMovementViewTarget: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: i32,
                _: f32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
        pub BotPredictVisiblePosition: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
        pub BotAllocMoveState: Option<unsafe extern "C" fn() -> i32>,
        pub BotFreeMoveState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotInitMoveState: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_move::bot_initmove_s,
            ) -> (),
        >,
        pub BotAddAvoidSpot: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: i32,
            ) -> (),
        >,
        pub BotChooseBestFightWeapon: Option<unsafe extern "C" fn(_: i32, _: *mut i32) -> i32>,
        pub BotGetWeaponInfo: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: *mut crate::src::botlib::be_ai_weap::weaponinfo_s,
            ) -> (),
        >,
        pub BotLoadWeaponWeights: Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char) -> i32>,
        pub BotAllocWeaponState: Option<unsafe extern "C" fn() -> i32>,
        pub BotFreeWeaponState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub BotResetWeaponState: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub GeneticParentsAndChildSelection: Option<
            unsafe extern "C" fn(_: i32, _: *mut f32, _: *mut i32, _: *mut i32, _: *mut i32) -> i32,
        >,
    }

    pub type botlib_export_t = crate::botlib_h::botlib_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct botlib_export_s {
        pub aas: crate::botlib_h::aas_export_t,
        pub ea: crate::botlib_h::ea_export_t,
        pub ai: crate::botlib_h::ai_export_t,
        pub BotLibSetup: Option<unsafe extern "C" fn() -> i32>,
        pub BotLibShutdown: Option<unsafe extern "C" fn() -> i32>,
        pub BotLibVarSet:
            Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> i32>,
        pub BotLibVarGet: Option<
            unsafe extern "C" fn(_: *const libc::c_char, _: *mut libc::c_char, _: i32) -> i32,
        >,
        pub PC_AddGlobalDefine: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> i32>,
        pub PC_LoadSourceHandle: Option<unsafe extern "C" fn(_: *const libc::c_char) -> i32>,
        pub PC_FreeSourceHandle: Option<unsafe extern "C" fn(_: i32) -> i32>,
        pub PC_ReadTokenHandle: Option<
            unsafe extern "C" fn(_: i32, _: *mut crate::src::qcommon::q_shared::pc_token_t) -> i32,
        >,
        pub PC_SourceFileAndLine:
            Option<unsafe extern "C" fn(_: i32, _: *mut libc::c_char, _: *mut i32) -> i32>,
        pub BotLibStartFrame: Option<unsafe extern "C" fn(_: f32) -> i32>,
        pub BotLibLoadMap: Option<unsafe extern "C" fn(_: *const libc::c_char) -> i32>,
        pub BotLibUpdateEntity:
            Option<unsafe extern "C" fn(_: i32, _: *mut crate::botlib_h::bot_entitystate_t) -> i32>,
        pub Test: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *mut libc::c_char,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
        >,
    }
}
pub mod server_h {
    pub use crate::src::server::sv_world::worldSector_s;

    pub type voipServerPacket_t = crate::server_h::voipServerPacket_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct voipServerPacket_s {
        pub generation: i32,
        pub sequence: i32,
        pub frames: i32,
        pub len: i32,
        pub sender: i32,
        pub flags: i32,
        pub data: [crate::src::qcommon::q_shared::byte; 4000],
    }

    pub type svEntity_t = crate::server_h::svEntity_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct svEntity_s {
        pub worldSector: *mut crate::server_h::worldSector_s,
        pub nextEntityInWorldSector: *mut crate::server_h::svEntity_s,
        pub baseline: crate::src::qcommon::q_shared::entityState_t,
        pub numClusters: i32,
        pub clusternums: [i32; 16],
        pub lastCluster: i32,
        pub areanum: i32,
        pub areanum2: i32,
        pub snapshotCounter: i32,
    }

    pub type serverState_t = u32;

    pub const SS_DEAD: crate::server_h::serverState_t = 0;

    pub const SS_LOADING: crate::server_h::serverState_t = 1;

    pub const SS_GAME: crate::server_h::serverState_t = 2;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct server_t {
        pub state: crate::server_h::serverState_t,
        pub restarting: crate::src::qcommon::q_shared::qboolean,
        pub serverId: i32,
        pub restartedServerId: i32,
        pub checksumFeed: i32,
        pub checksumFeedServerId: i32,
        pub snapshotCounter: i32,
        pub timeResidual: i32,
        pub nextFrameTime: i32,
        pub configstrings: [*mut libc::c_char; 1024],
        pub svEntities: [crate::server_h::svEntity_t; 1024],
        pub entityParsePoint: *mut libc::c_char,
        pub gentities: *mut crate::g_public_h::sharedEntity_t,
        pub gentitySize: i32,
        pub num_entities: i32,
        pub gameClients: *mut crate::src::qcommon::q_shared::playerState_t,
        pub gameClientSize: i32,
        pub restartTime: i32,
        pub time: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientSnapshot_t {
        pub areabytes: i32,
        pub areabits: [crate::src::qcommon::q_shared::byte; 32],
        pub ps: crate::src::qcommon::q_shared::playerState_t,
        pub num_entities: i32,
        pub first_entity: i32,
        pub messageSent: i32,
        pub messageAcked: i32,
        pub messageSize: i32,
    }

    pub type clientState_t = u32;

    pub const CS_FREE: crate::server_h::clientState_t = 0;

    pub const CS_ZOMBIE: crate::server_h::clientState_t = 1;

    pub const CS_CONNECTED: crate::server_h::clientState_t = 2;

    pub const CS_PRIMED: crate::server_h::clientState_t = 3;

    pub const CS_ACTIVE: crate::server_h::clientState_t = 4;

    pub type netchan_buffer_t = crate::server_h::netchan_buffer_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netchan_buffer_s {
        pub msg: crate::qcommon_h::msg_t,
        pub msgBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut crate::server_h::netchan_buffer_s,
    }

    pub type client_t = crate::server_h::client_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct client_s {
        pub state: crate::server_h::clientState_t,
        pub userinfo: [libc::c_char; 1024],
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub reliableSequence: i32,
        pub reliableAcknowledge: i32,
        pub reliableSent: i32,
        pub messageAcknowledge: i32,
        pub gamestateMessageNum: i32,
        pub challenge: i32,
        pub lastUsercmd: crate::src::qcommon::q_shared::usercmd_t,
        pub lastMessageNum: i32,
        pub lastClientCommand: i32,
        pub lastClientCommandString: [libc::c_char; 1024],
        pub gentity: *mut crate::g_public_h::sharedEntity_t,
        pub name: [libc::c_char; 32],
        pub downloadName: [libc::c_char; 64],
        pub download: crate::src::qcommon::q_shared::fileHandle_t,
        pub downloadSize: i32,
        pub downloadCount: i32,
        pub downloadClientBlock: i32,
        pub downloadCurrentBlock: i32,
        pub downloadXmitBlock: i32,
        pub downloadBlocks: [*mut u8; 48],
        pub downloadBlockSize: [i32; 48],
        pub downloadEOF: crate::src::qcommon::q_shared::qboolean,
        pub downloadSendTime: i32,
        pub deltaMessage: i32,
        pub nextReliableTime: i32,
        pub lastPacketTime: i32,
        pub lastConnectTime: i32,
        pub lastSnapshotTime: i32,
        pub rateDelayed: crate::src::qcommon::q_shared::qboolean,
        pub timeoutCount: i32,
        pub frames: [crate::server_h::clientSnapshot_t; 32],
        pub ping: i32,
        pub rate: i32,
        pub snapshotMsec: i32,
        pub pureAuthentic: i32,
        pub gotCP: crate::src::qcommon::q_shared::qboolean,
        pub netchan: crate::qcommon_h::netchan_t,
        pub netchan_start_queue: *mut crate::server_h::netchan_buffer_t,
        pub netchan_end_queue: *mut *mut crate::server_h::netchan_buffer_t,
        pub hasVoip: crate::src::qcommon::q_shared::qboolean,
        pub muteAllVoip: crate::src::qcommon::q_shared::qboolean,
        pub ignoreVoipFromClient: [crate::src::qcommon::q_shared::qboolean; 64],
        pub voipPacket: [*mut crate::server_h::voipServerPacket_t; 64],
        pub queuedVoipPackets: i32,
        pub queuedVoipIndex: i32,
        pub oldServerTime: i32,
        pub csUpdated: [crate::src::qcommon::q_shared::qboolean; 1024],
        pub compat: crate::src::qcommon::q_shared::qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct challenge_t {
        pub adr: crate::qcommon_h::netadr_t,
        pub challenge: i32,
        pub clientChallenge: i32,
        pub time: i32,
        pub pingTime: i32,
        pub firstTime: i32,
        pub wasrefused: crate::src::qcommon::q_shared::qboolean,
        pub connected: crate::src::qcommon::q_shared::qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct serverStatic_t {
        pub initialized: crate::src::qcommon::q_shared::qboolean,
        pub time: i32,
        pub snapFlagServerBit: i32,
        pub clients: *mut crate::server_h::client_t,
        pub numSnapshotEntities: i32,
        pub nextSnapshotEntities: i32,
        pub snapshotEntities: *mut crate::src::qcommon::q_shared::entityState_t,
        pub nextHeartbeatTime: i32,
        pub challenges: [crate::server_h::challenge_t; 2048],
        pub redirectAddress: crate::qcommon_h::netadr_t,
        pub authorizeAddress: crate::qcommon_h::netadr_t,
        pub masterResolveTime: [i32; 5],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct serverBan_t {
        pub ip: crate::qcommon_h::netadr_t,
        pub subnet: i32,
        pub isexception: crate::src::qcommon::q_shared::qboolean,
    }

    pub type leakyBucket_t = crate::server_h::leakyBucket_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct leakyBucket_s {
        pub type_0: crate::qcommon_h::netadrtype_t,
        pub ipv: crate::server_h::C2RustUnnamed_164,
        pub lastTime: i32,
        pub burst: i8,
        pub hash: isize,
        pub prev: *mut crate::server_h::leakyBucket_t,
        pub next: *mut crate::server_h::leakyBucket_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_164 {
        pub _4: [crate::src::qcommon::q_shared::byte; 4],
        pub _6: [crate::src::qcommon::q_shared::byte; 16],
    }
}
pub mod g_public_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityShared_t {
        pub unused: crate::src::qcommon::q_shared::entityState_t,
        pub linked: crate::src::qcommon::q_shared::qboolean,
        pub linkcount: i32,
        pub svFlags: i32,
        pub singleClient: i32,
        pub bmodel: crate::src::qcommon::q_shared::qboolean,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub contents: i32,
        pub absmin: crate::src::qcommon::q_shared::vec3_t,
        pub absmax: crate::src::qcommon::q_shared::vec3_t,
        pub currentOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub currentAngles: crate::src::qcommon::q_shared::vec3_t,
        pub ownerNum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sharedEntity_t {
        pub s: crate::src::qcommon::q_shared::entityState_t,
        pub r: crate::g_public_h::entityShared_t,
    }

    pub const G_PRINT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const G_ERROR: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const G_MILLISECONDS: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const G_CVAR_REGISTER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const G_CVAR_UPDATE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const G_CVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const G_CVAR_VARIABLE_INTEGER_VALUE: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const G_CVAR_VARIABLE_STRING_BUFFER: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const G_ARGC: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const G_ARGV: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const G_FS_FOPEN_FILE: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const G_FS_READ: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const G_FS_WRITE: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const G_FS_FCLOSE_FILE: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const G_SEND_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const G_LOCATE_GAME_DATA: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const G_DROP_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const G_SEND_SERVER_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const G_SET_CONFIGSTRING: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const G_GET_CONFIGSTRING: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const G_GET_USERINFO: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const G_SET_USERINFO: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const G_GET_SERVERINFO: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const G_SET_BRUSH_MODEL: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const G_TRACE: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const G_POINT_CONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const G_IN_PVS: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const G_IN_PVS_IGNORE_PORTALS: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const G_ADJUST_AREA_PORTAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const G_AREAS_CONNECTED: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const G_LINKENTITY: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const G_UNLINKENTITY: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const G_ENTITIES_IN_BOX: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const G_ENTITY_CONTACT: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const G_BOT_ALLOCATE_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const G_BOT_FREE_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const G_GET_USERCMD: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const G_GET_ENTITY_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const G_FS_GETFILELIST: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const G_DEBUG_POLYGON_CREATE: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const G_DEBUG_POLYGON_DELETE: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const G_REAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const G_SNAPVECTOR: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const G_TRACECAPSULE: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const G_ENTITY_CONTACTCAPSULE: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const G_FS_SEEK: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const BOTLIB_SETUP: crate::be_aas_h::C2RustUnnamed_0 = 200;

    pub const BOTLIB_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 201;

    pub const BOTLIB_LIBVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 202;

    pub const BOTLIB_LIBVAR_GET: crate::be_aas_h::C2RustUnnamed_0 = 203;

    pub const BOTLIB_PC_ADD_GLOBAL_DEFINE: crate::be_aas_h::C2RustUnnamed_0 = 204;

    pub const BOTLIB_START_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 205;

    pub const BOTLIB_LOAD_MAP: crate::be_aas_h::C2RustUnnamed_0 = 206;

    pub const BOTLIB_UPDATENTITY: crate::be_aas_h::C2RustUnnamed_0 = 207;

    pub const BOTLIB_TEST: crate::be_aas_h::C2RustUnnamed_0 = 208;

    pub const BOTLIB_GET_SNAPSHOT_ENTITY: crate::be_aas_h::C2RustUnnamed_0 = 209;

    pub const BOTLIB_GET_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 210;

    pub const BOTLIB_USER_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 211;

    pub const BOTLIB_AAS_ENABLE_ROUTING_AREA: crate::be_aas_h::C2RustUnnamed_0 = 300;

    pub const BOTLIB_AAS_BBOX_AREAS: crate::be_aas_h::C2RustUnnamed_0 = 301;

    pub const BOTLIB_AAS_AREA_INFO: crate::be_aas_h::C2RustUnnamed_0 = 302;

    pub const BOTLIB_AAS_ENTITY_INFO: crate::be_aas_h::C2RustUnnamed_0 = 303;

    pub const BOTLIB_AAS_INITIALIZED: crate::be_aas_h::C2RustUnnamed_0 = 304;

    pub const BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX: crate::be_aas_h::C2RustUnnamed_0 = 305;

    pub const BOTLIB_AAS_TIME: crate::be_aas_h::C2RustUnnamed_0 = 306;

    pub const BOTLIB_AAS_POINT_AREA_NUM: crate::be_aas_h::C2RustUnnamed_0 = 307;

    pub const BOTLIB_AAS_TRACE_AREAS: crate::be_aas_h::C2RustUnnamed_0 = 308;

    pub const BOTLIB_AAS_POINT_CONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 309;

    pub const BOTLIB_AAS_NEXT_BSP_ENTITY: crate::be_aas_h::C2RustUnnamed_0 = 310;

    pub const BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 311;

    pub const BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 312;

    pub const BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 313;

    pub const BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 314;

    pub const BOTLIB_AAS_AREA_REACHABILITY: crate::be_aas_h::C2RustUnnamed_0 = 315;

    pub const BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA: crate::be_aas_h::C2RustUnnamed_0 = 316;

    pub const BOTLIB_AAS_SWIMMING: crate::be_aas_h::C2RustUnnamed_0 = 317;

    pub const BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT: crate::be_aas_h::C2RustUnnamed_0 = 318;

    pub const BOTLIB_EA_SAY: crate::be_aas_h::C2RustUnnamed_0 = 400;

    pub const BOTLIB_EA_SAY_TEAM: crate::be_aas_h::C2RustUnnamed_0 = 401;

    pub const BOTLIB_EA_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 402;

    pub const BOTLIB_EA_ACTION: crate::be_aas_h::C2RustUnnamed_0 = 403;

    pub const BOTLIB_EA_GESTURE: crate::be_aas_h::C2RustUnnamed_0 = 404;

    pub const BOTLIB_EA_TALK: crate::be_aas_h::C2RustUnnamed_0 = 405;

    pub const BOTLIB_EA_ATTACK: crate::be_aas_h::C2RustUnnamed_0 = 406;

    pub const BOTLIB_EA_USE: crate::be_aas_h::C2RustUnnamed_0 = 407;

    pub const BOTLIB_EA_RESPAWN: crate::be_aas_h::C2RustUnnamed_0 = 408;

    pub const BOTLIB_EA_CROUCH: crate::be_aas_h::C2RustUnnamed_0 = 409;

    pub const BOTLIB_EA_MOVE_UP: crate::be_aas_h::C2RustUnnamed_0 = 410;

    pub const BOTLIB_EA_MOVE_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 411;

    pub const BOTLIB_EA_MOVE_FORWARD: crate::be_aas_h::C2RustUnnamed_0 = 412;

    pub const BOTLIB_EA_MOVE_BACK: crate::be_aas_h::C2RustUnnamed_0 = 413;

    pub const BOTLIB_EA_MOVE_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 414;

    pub const BOTLIB_EA_MOVE_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 415;

    pub const BOTLIB_EA_SELECT_WEAPON: crate::be_aas_h::C2RustUnnamed_0 = 416;

    pub const BOTLIB_EA_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 417;

    pub const BOTLIB_EA_DELAYED_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 418;

    pub const BOTLIB_EA_MOVE: crate::be_aas_h::C2RustUnnamed_0 = 419;

    pub const BOTLIB_EA_VIEW: crate::be_aas_h::C2RustUnnamed_0 = 420;

    pub const BOTLIB_EA_END_REGULAR: crate::be_aas_h::C2RustUnnamed_0 = 421;

    pub const BOTLIB_EA_GET_INPUT: crate::be_aas_h::C2RustUnnamed_0 = 422;

    pub const BOTLIB_EA_RESET_INPUT: crate::be_aas_h::C2RustUnnamed_0 = 423;

    pub const BOTLIB_AI_LOAD_CHARACTER: crate::be_aas_h::C2RustUnnamed_0 = 500;

    pub const BOTLIB_AI_FREE_CHARACTER: crate::be_aas_h::C2RustUnnamed_0 = 501;

    pub const BOTLIB_AI_CHARACTERISTIC_FLOAT: crate::be_aas_h::C2RustUnnamed_0 = 502;

    pub const BOTLIB_AI_CHARACTERISTIC_BFLOAT: crate::be_aas_h::C2RustUnnamed_0 = 503;

    pub const BOTLIB_AI_CHARACTERISTIC_INTEGER: crate::be_aas_h::C2RustUnnamed_0 = 504;

    pub const BOTLIB_AI_CHARACTERISTIC_BINTEGER: crate::be_aas_h::C2RustUnnamed_0 = 505;

    pub const BOTLIB_AI_CHARACTERISTIC_STRING: crate::be_aas_h::C2RustUnnamed_0 = 506;

    pub const BOTLIB_AI_ALLOC_CHAT_STATE: crate::be_aas_h::C2RustUnnamed_0 = 507;

    pub const BOTLIB_AI_FREE_CHAT_STATE: crate::be_aas_h::C2RustUnnamed_0 = 508;

    pub const BOTLIB_AI_QUEUE_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 509;

    pub const BOTLIB_AI_REMOVE_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 510;

    pub const BOTLIB_AI_NEXT_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 511;

    pub const BOTLIB_AI_NUM_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const BOTLIB_AI_INITIAL_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 513;

    pub const BOTLIB_AI_REPLY_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 514;

    pub const BOTLIB_AI_CHAT_LENGTH: crate::be_aas_h::C2RustUnnamed_0 = 515;

    pub const BOTLIB_AI_ENTER_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 516;

    pub const BOTLIB_AI_STRING_CONTAINS: crate::be_aas_h::C2RustUnnamed_0 = 517;

    pub const BOTLIB_AI_FIND_MATCH: crate::be_aas_h::C2RustUnnamed_0 = 518;

    pub const BOTLIB_AI_MATCH_VARIABLE: crate::be_aas_h::C2RustUnnamed_0 = 519;

    pub const BOTLIB_AI_UNIFY_WHITE_SPACES: crate::be_aas_h::C2RustUnnamed_0 = 520;

    pub const BOTLIB_AI_REPLACE_SYNONYMS: crate::be_aas_h::C2RustUnnamed_0 = 521;

    pub const BOTLIB_AI_LOAD_CHAT_FILE: crate::be_aas_h::C2RustUnnamed_0 = 522;

    pub const BOTLIB_AI_SET_CHAT_GENDER: crate::be_aas_h::C2RustUnnamed_0 = 523;

    pub const BOTLIB_AI_SET_CHAT_NAME: crate::be_aas_h::C2RustUnnamed_0 = 524;

    pub const BOTLIB_AI_RESET_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 525;

    pub const BOTLIB_AI_RESET_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 526;

    pub const BOTLIB_AI_PUSH_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 527;

    pub const BOTLIB_AI_POP_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 528;

    pub const BOTLIB_AI_EMPTY_GOAL_STACK: crate::be_aas_h::C2RustUnnamed_0 = 529;

    pub const BOTLIB_AI_DUMP_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 530;

    pub const BOTLIB_AI_DUMP_GOAL_STACK: crate::be_aas_h::C2RustUnnamed_0 = 531;

    pub const BOTLIB_AI_GOAL_NAME: crate::be_aas_h::C2RustUnnamed_0 = 532;

    pub const BOTLIB_AI_GET_TOP_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 533;

    pub const BOTLIB_AI_GET_SECOND_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 534;

    pub const BOTLIB_AI_CHOOSE_LTG_ITEM: crate::be_aas_h::C2RustUnnamed_0 = 535;

    pub const BOTLIB_AI_CHOOSE_NBG_ITEM: crate::be_aas_h::C2RustUnnamed_0 = 536;

    pub const BOTLIB_AI_TOUCHING_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 537;

    pub const BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE: crate::be_aas_h::C2RustUnnamed_0 = 538;

    pub const BOTLIB_AI_GET_LEVEL_ITEM_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 539;

    pub const BOTLIB_AI_AVOID_GOAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 540;

    pub const BOTLIB_AI_INIT_LEVEL_ITEMS: crate::be_aas_h::C2RustUnnamed_0 = 541;

    pub const BOTLIB_AI_UPDATE_ENTITY_ITEMS: crate::be_aas_h::C2RustUnnamed_0 = 542;

    pub const BOTLIB_AI_LOAD_ITEM_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 543;

    pub const BOTLIB_AI_FREE_ITEM_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 544;

    pub const BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 545;

    pub const BOTLIB_AI_ALLOC_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 546;

    pub const BOTLIB_AI_FREE_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 547;

    pub const BOTLIB_AI_RESET_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 548;

    pub const BOTLIB_AI_MOVE_TO_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 549;

    pub const BOTLIB_AI_MOVE_IN_DIRECTION: crate::be_aas_h::C2RustUnnamed_0 = 550;

    pub const BOTLIB_AI_RESET_AVOID_REACH: crate::be_aas_h::C2RustUnnamed_0 = 551;

    pub const BOTLIB_AI_RESET_LAST_AVOID_REACH: crate::be_aas_h::C2RustUnnamed_0 = 552;

    pub const BOTLIB_AI_REACHABILITY_AREA: crate::be_aas_h::C2RustUnnamed_0 = 553;

    pub const BOTLIB_AI_MOVEMENT_VIEW_TARGET: crate::be_aas_h::C2RustUnnamed_0 = 554;

    pub const BOTLIB_AI_ALLOC_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 555;

    pub const BOTLIB_AI_FREE_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 556;

    pub const BOTLIB_AI_INIT_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 557;

    pub const BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON: crate::be_aas_h::C2RustUnnamed_0 = 558;

    pub const BOTLIB_AI_GET_WEAPON_INFO: crate::be_aas_h::C2RustUnnamed_0 = 559;

    pub const BOTLIB_AI_LOAD_WEAPON_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 560;

    pub const BOTLIB_AI_ALLOC_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 561;

    pub const BOTLIB_AI_FREE_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 562;

    pub const BOTLIB_AI_RESET_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 563;

    pub const BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION: crate::be_aas_h::C2RustUnnamed_0 = 564;

    pub const BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 565;

    pub const BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 566;

    pub const BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 567;

    pub const BOTLIB_AI_GET_MAP_LOCATION_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 568;

    pub const BOTLIB_AI_NUM_INITIAL_CHATS: crate::be_aas_h::C2RustUnnamed_0 = 569;

    pub const BOTLIB_AI_GET_CHAT_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 570;

    pub const BOTLIB_AI_REMOVE_FROM_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 571;

    pub const BOTLIB_AI_PREDICT_VISIBLE_POSITION: crate::be_aas_h::C2RustUnnamed_0 = 572;

    pub const BOTLIB_AI_SET_AVOID_GOAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 573;

    pub const BOTLIB_AI_ADD_AVOID_SPOT: crate::be_aas_h::C2RustUnnamed_0 = 574;

    pub const BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 575;

    pub const BOTLIB_AAS_PREDICT_ROUTE: crate::be_aas_h::C2RustUnnamed_0 = 576;

    pub const BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX: crate::be_aas_h::C2RustUnnamed_0 = 577;

    pub const BOTLIB_PC_LOAD_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 578;

    pub const BOTLIB_PC_FREE_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 579;

    pub const BOTLIB_PC_READ_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 580;

    pub const BOTLIB_PC_SOURCE_FILE_AND_LINE: crate::be_aas_h::C2RustUnnamed_0 = 581;

    pub const GAME_INIT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const GAME_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const GAME_CLIENT_CONNECT: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const GAME_CLIENT_BEGIN: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const GAME_CLIENT_USERINFO_CHANGED: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const GAME_CLIENT_DISCONNECT: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const GAME_CLIENT_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const GAME_CLIENT_THINK: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const GAME_RUN_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const GAME_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const BOTAI_START_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 10;
}
pub mod vm_local_h {
    pub const OP_UNDEF: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const OP_IGNORE: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const OP_BREAK: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const OP_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const OP_LEAVE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const OP_CALL: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const OP_PUSH: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const OP_POP: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const OP_CONST: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const OP_LOCAL: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const OP_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const OP_EQ: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const OP_NE: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const OP_LTI: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const OP_LEI: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const OP_GTI: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const OP_GEI: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const OP_LTU: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const OP_LEU: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const OP_GTU: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const OP_GEU: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const OP_EQF: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const OP_NEF: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const OP_LTF: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const OP_LEF: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const OP_GTF: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const OP_GEF: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const OP_LOAD1: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const OP_LOAD2: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const OP_LOAD4: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const OP_STORE1: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const OP_STORE2: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const OP_STORE4: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const OP_ARG: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const OP_BLOCK_COPY: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const OP_SEX8: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const OP_SEX16: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const OP_NEGI: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const OP_ADD: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const OP_SUB: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const OP_DIVI: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const OP_DIVU: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const OP_MODI: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const OP_MODU: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const OP_MULI: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const OP_MULU: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const OP_BAND: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const OP_BOR: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const OP_BXOR: crate::be_aas_h::C2RustUnnamed_0 = 48;

    pub const OP_BCOM: crate::be_aas_h::C2RustUnnamed_0 = 49;

    pub const OP_LSH: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const OP_RSHI: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const OP_RSHU: crate::be_aas_h::C2RustUnnamed_0 = 52;

    pub const OP_NEGF: crate::be_aas_h::C2RustUnnamed_0 = 53;

    pub const OP_ADDF: crate::be_aas_h::C2RustUnnamed_0 = 54;

    pub const OP_SUBF: crate::be_aas_h::C2RustUnnamed_0 = 55;

    pub const OP_DIVF: crate::be_aas_h::C2RustUnnamed_0 = 56;

    pub const OP_MULF: crate::be_aas_h::C2RustUnnamed_0 = 57;

    pub const OP_CVIF: crate::be_aas_h::C2RustUnnamed_0 = 58;

    pub const OP_CVFI: crate::be_aas_h::C2RustUnnamed_0 = 59;

    pub type vmSymbol_t = crate::vm_local_h::vmSymbol_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmSymbol_s {
        pub next: *mut crate::vm_local_h::vmSymbol_s,
        pub symValue: i32,
        pub profileCount: i32,
        pub symName: [libc::c_char; 1],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vm_s {
        pub programStack: i32,
        pub systemCall: Option<
            unsafe extern "C" fn(_: *mut crate::stdlib::intptr_t) -> crate::stdlib::intptr_t,
        >,
        pub name: [libc::c_char; 64],
        pub searchPath: *mut libc::c_void,
        pub dllHandle: *mut libc::c_void,
        pub entryPoint: Option<unsafe extern "C" fn(_: i32, _: ...) -> crate::stdlib::intptr_t>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut crate::qcommon_h::vm_t) -> ()>,
        pub currentlyInterpreting: crate::src::qcommon::q_shared::qboolean,
        pub compiled: crate::src::qcommon::q_shared::qboolean,
        pub codeBase: *mut crate::src::qcommon::q_shared::byte,
        pub entryOfs: i32,
        pub codeLength: i32,
        pub instructionPointers: *mut crate::stdlib::intptr_t,
        pub instructionCount: i32,
        pub dataBase: *mut crate::src::qcommon::q_shared::byte,
        pub dataMask: i32,
        pub dataAlloc: i32,
        pub stackBottom: i32,
        pub numSymbols: i32,
        pub symbols: *mut crate::vm_local_h::vmSymbol_s,
        pub callLevel: i32,
        pub breakFunction: i32,
        pub breakCount: i32,
        pub jumpTableTargets: *mut crate::src::qcommon::q_shared::byte,
        pub numJumpTableTargets: i32,
    }
}
pub mod qfiles_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmHeader_t {
        pub vmMagic: i32,
        pub instructionCount: i32,
        pub codeOffset: i32,
        pub codeLength: i32,
        pub dataOffset: i32,
        pub dataLength: i32,
        pub litLength: i32,
        pub bssLength: i32,
        pub jtrgLength: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct lump_t {
        pub fileofs: i32,
        pub filelen: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dheader_t {
        pub ident: i32,
        pub version: i32,
        pub lumps: [crate::qfiles_h::lump_t; 17],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dmodel_t {
        pub mins: [f32; 3],
        pub maxs: [f32; 3],
        pub firstSurface: i32,
        pub numSurfaces: i32,
        pub firstBrush: i32,
        pub numBrushes: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dshader_t {
        pub shader: [libc::c_char; 64],
        pub surfaceFlags: i32,
        pub contentFlags: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dplane_t {
        pub normal: [f32; 3],
        pub dist: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dnode_t {
        pub planeNum: i32,
        pub children: [i32; 2],
        pub mins: [i32; 3],
        pub maxs: [i32; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dleaf_t {
        pub cluster: i32,
        pub area: i32,
        pub mins: [i32; 3],
        pub maxs: [i32; 3],
        pub firstLeafSurface: i32,
        pub numLeafSurfaces: i32,
        pub firstLeafBrush: i32,
        pub numLeafBrushes: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dbrushside_t {
        pub planeNum: i32,
        pub shaderNum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dbrush_t {
        pub firstSide: i32,
        pub numSides: i32,
        pub shaderNum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct drawVert_t {
        pub xyz: crate::src::qcommon::q_shared::vec3_t,
        pub st: [f32; 2],
        pub lightmap: [f32; 2],
        pub normal: crate::src::qcommon::q_shared::vec3_t,
        pub color: [crate::src::qcommon::q_shared::byte; 4],
    }

    pub const MST_BAD: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const MST_PLANAR: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const MST_PATCH: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const MST_TRIANGLE_SOUP: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const MST_FLARE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dsurface_t {
        pub shaderNum: i32,
        pub fogNum: i32,
        pub surfaceType: i32,
        pub firstVert: i32,
        pub numVerts: i32,
        pub firstIndex: i32,
        pub numIndexes: i32,
        pub lightmapNum: i32,
        pub lightmapX: i32,
        pub lightmapY: i32,
        pub lightmapWidth: i32,
        pub lightmapHeight: i32,
        pub lightmapOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub lightmapVecs: [crate::src::qcommon::q_shared::vec3_t; 3],
        pub patchWidth: i32,
        pub patchHeight: i32,
    }
}
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::__m128;
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::_mm_cvt_ss2si;
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::_mm_cvtss_si32;
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::_mm_set_ss;
}
pub mod emmintrin_h {
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::__m128d;
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::_mm_cvtsd_si32;
    #[cfg(target_arch = "x86_64")]
    pub use std::arch::x86_64::_mm_load_sd;

    #[repr(C, packed)]
    #[derive(Copy, Clone)]
    pub struct __mm_load_sd_struct {
        pub __u: f64,
    }
}
pub mod snd_local_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct portable_samplepair_t {
        pub left: i32,
        pub right: i32,
    }

    pub type adpcm_state_t = crate::snd_local_h::adpcm_state;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct adpcm_state {
        pub sample: i16,
        pub index: libc::c_char,
    }

    pub type sndBuffer = crate::snd_local_h::sndBuffer_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sndBuffer_s {
        pub sndChunk: [i16; 1024],
        pub next: *mut crate::snd_local_h::sndBuffer_s,
        pub size: i32,
        pub adpcm: crate::snd_local_h::adpcm_state_t,
    }

    pub type sfx_t = crate::snd_local_h::sfx_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sfx_s {
        pub soundData: *mut crate::snd_local_h::sndBuffer,
        pub defaultSound: crate::src::qcommon::q_shared::qboolean,
        pub inMemory: crate::src::qcommon::q_shared::qboolean,
        pub soundCompressed: crate::src::qcommon::q_shared::qboolean,
        pub soundCompressionMethod: i32,
        pub soundLength: i32,
        pub soundChannels: i32,
        pub soundName: [libc::c_char; 64],
        pub lastTimeUsed: i32,
        pub next: *mut crate::snd_local_h::sfx_s,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dma_t {
        pub channels: i32,
        pub samples: i32,
        pub fullsamples: i32,
        pub submission_chunk: i32,
        pub samplebits: i32,
        pub isfloat: i32,
        pub speed: i32,
        pub buffer: *mut crate::src::qcommon::q_shared::byte,
    }

    pub type loopSound_t = crate::snd_local_h::loopSound_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct loopSound_s {
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub velocity: crate::src::qcommon::q_shared::vec3_t,
        pub sfx: *mut crate::snd_local_h::sfx_t,
        pub mergeFrame: i32,
        pub active: crate::src::qcommon::q_shared::qboolean,
        pub kill: crate::src::qcommon::q_shared::qboolean,
        pub doppler: crate::src::qcommon::q_shared::qboolean,
        pub dopplerScale: f32,
        pub oldDopplerScale: f32,
        pub framenum: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct channel_t {
        pub allocTime: i32,
        pub startSample: i32,
        pub entnum: i32,
        pub entchannel: i32,
        pub leftvol: i32,
        pub rightvol: i32,
        pub master_vol: i32,
        pub dopplerScale: f32,
        pub oldDopplerScale: f32,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub fixed_origin: crate::src::qcommon::q_shared::qboolean,
        pub thesfx: *mut crate::snd_local_h::sfx_t,
        pub doppler: crate::src::qcommon::q_shared::qboolean,
        pub fullVolume: crate::src::qcommon::q_shared::qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct wavinfo_t {
        pub format: i32,
        pub rate: i32,
        pub width: i32,
        pub channels: i32,
        pub samples: i32,
        pub dataofs: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct soundInterface_t {
        pub Shutdown: Option<unsafe extern "C" fn() -> ()>,
        pub StartSound: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
        >,
        pub StartLocalSound: Option<
            unsafe extern "C" fn(_: crate::src::qcommon::q_shared::sfxHandle_t, _: i32) -> (),
        >,
        pub StartBackgroundTrack:
            Option<unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> ()>,
        pub StopBackgroundTrack: Option<unsafe extern "C" fn() -> ()>,
        pub RawSamples: Option<
            unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: *const crate::src::qcommon::q_shared::byte,
                _: f32,
                _: i32,
            ) -> (),
        >,
        pub StopAllSounds: Option<unsafe extern "C" fn() -> ()>,
        pub ClearLoopingSounds:
            Option<unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> ()>,
        pub AddLoopingSound: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
        >,
        pub AddRealLoopingSound: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: crate::src::qcommon::q_shared::sfxHandle_t,
            ) -> (),
        >,
        pub StopLoopingSound: Option<unsafe extern "C" fn(_: i32) -> ()>,
        pub Respatialize: Option<
            unsafe extern "C" fn(
                _: i32,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
                _: i32,
            ) -> (),
        >,
        pub UpdateEntityPosition: Option<
            unsafe extern "C" fn(_: i32, _: *const crate::src::qcommon::q_shared::vec_t) -> (),
        >,
        pub Update: Option<unsafe extern "C" fn() -> ()>,
        pub DisableSounds: Option<unsafe extern "C" fn() -> ()>,
        pub BeginRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub RegisterSound: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: crate::src::qcommon::q_shared::qboolean,
            ) -> crate::src::qcommon::q_shared::sfxHandle_t,
        >,
        pub ClearSoundBuffer: Option<unsafe extern "C" fn() -> ()>,
        pub SoundInfo: Option<unsafe extern "C" fn() -> ()>,
        pub SoundList: Option<unsafe extern "C" fn() -> ()>,
        pub StartCapture: Option<unsafe extern "C" fn() -> ()>,
        pub AvailableCaptureSamples: Option<unsafe extern "C" fn() -> i32>,
        pub Capture:
            Option<unsafe extern "C" fn(_: i32, _: *mut crate::src::qcommon::q_shared::byte) -> ()>,
        pub StopCapture: Option<unsafe extern "C" fn() -> ()>,
        pub MasterGain: Option<unsafe extern "C" fn(_: f32) -> ()>,
    }

    pub type alSrcPriority_t = u32;

    pub const SRCPRI_AMBIENT: crate::snd_local_h::alSrcPriority_t = 0;

    pub const SRCPRI_ENTITY: crate::snd_local_h::alSrcPriority_t = 1;

    pub const SRCPRI_ONESHOT: crate::snd_local_h::alSrcPriority_t = 2;

    pub const SRCPRI_LOCAL: crate::snd_local_h::alSrcPriority_t = 3;

    pub const SRCPRI_STREAM: crate::snd_local_h::alSrcPriority_t = 4;

    pub type srcHandle_t = i32;
}
pub mod multi_h {
    pub type CURLM = ();

    pub type CURLMcode = i32;

    pub const CURLM_CALL_MULTI_PERFORM: crate::multi_h::CURLMcode = -1;

    pub const CURLM_OK: crate::multi_h::CURLMcode = 0;

    pub const CURLM_BAD_HANDLE: crate::multi_h::CURLMcode = 1;

    pub const CURLM_BAD_EASY_HANDLE: crate::multi_h::CURLMcode = 2;

    pub const CURLM_OUT_OF_MEMORY: crate::multi_h::CURLMcode = 3;

    pub const CURLM_INTERNAL_ERROR: crate::multi_h::CURLMcode = 4;

    pub const CURLM_BAD_SOCKET: crate::multi_h::CURLMcode = 5;

    pub const CURLM_UNKNOWN_OPTION: crate::multi_h::CURLMcode = 6;

    pub const CURLM_ADDED_ALREADY: crate::multi_h::CURLMcode = 7;

    pub const CURLM_LAST: crate::multi_h::CURLMcode = 8;

    pub type CURLMSG = u32;

    pub const CURLMSG_NONE: crate::multi_h::CURLMSG = 0;

    pub const CURLMSG_DONE: crate::multi_h::CURLMSG = 1;

    pub const CURLMSG_LAST: crate::multi_h::CURLMSG = 2;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CURLMsg {
        pub msg: crate::multi_h::CURLMSG,
        pub easy_handle: *mut libc::c_void,
        pub data: crate::multi_h::C2RustUnnamed_20,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_20 {
        pub whatever: *mut libc::c_void,
        pub result: crate::curl_h::CURLcode,
    }
}
pub mod curl_h {
    pub type CURL = ();

    pub type CURLcode = u32;

    pub const CURLE_OK: crate::curl_h::CURLcode = 0;

    pub const CURLE_UNSUPPORTED_PROTOCOL: crate::curl_h::CURLcode = 1;

    pub const CURLE_FAILED_INIT: crate::curl_h::CURLcode = 2;

    pub const CURLE_URL_MALFORMAT: crate::curl_h::CURLcode = 3;

    pub const CURLE_NOT_BUILT_IN: crate::curl_h::CURLcode = 4;

    pub const CURLE_COULDNT_RESOLVE_PROXY: crate::curl_h::CURLcode = 5;

    pub const CURLE_COULDNT_RESOLVE_HOST: crate::curl_h::CURLcode = 6;

    pub const CURLE_COULDNT_CONNECT: crate::curl_h::CURLcode = 7;

    pub const CURLE_WEIRD_SERVER_REPLY: crate::curl_h::CURLcode = 8;

    pub const CURLE_REMOTE_ACCESS_DENIED: crate::curl_h::CURLcode = 9;

    pub const CURLE_FTP_ACCEPT_FAILED: crate::curl_h::CURLcode = 10;

    pub const CURLE_FTP_WEIRD_PASS_REPLY: crate::curl_h::CURLcode = 11;

    pub const CURLE_FTP_ACCEPT_TIMEOUT: crate::curl_h::CURLcode = 12;

    pub const CURLE_FTP_WEIRD_PASV_REPLY: crate::curl_h::CURLcode = 13;

    pub const CURLE_FTP_WEIRD_227_FORMAT: crate::curl_h::CURLcode = 14;

    pub const CURLE_FTP_CANT_GET_HOST: crate::curl_h::CURLcode = 15;

    pub const CURLE_HTTP2: crate::curl_h::CURLcode = 16;

    pub const CURLE_FTP_COULDNT_SET_TYPE: crate::curl_h::CURLcode = 17;

    pub const CURLE_PARTIAL_FILE: crate::curl_h::CURLcode = 18;

    pub const CURLE_FTP_COULDNT_RETR_FILE: crate::curl_h::CURLcode = 19;

    pub const CURLE_OBSOLETE20: crate::curl_h::CURLcode = 20;

    pub const CURLE_QUOTE_ERROR: crate::curl_h::CURLcode = 21;

    pub const CURLE_HTTP_RETURNED_ERROR: crate::curl_h::CURLcode = 22;

    pub const CURLE_WRITE_ERROR: crate::curl_h::CURLcode = 23;

    pub const CURLE_OBSOLETE24: crate::curl_h::CURLcode = 24;

    pub const CURLE_UPLOAD_FAILED: crate::curl_h::CURLcode = 25;

    pub const CURLE_READ_ERROR: crate::curl_h::CURLcode = 26;

    pub const CURLE_OUT_OF_MEMORY: crate::curl_h::CURLcode = 27;

    pub const CURLE_OPERATION_TIMEDOUT: crate::curl_h::CURLcode = 28;

    pub const CURLE_OBSOLETE29: crate::curl_h::CURLcode = 29;

    pub const CURLE_FTP_PORT_FAILED: crate::curl_h::CURLcode = 30;

    pub const CURLE_FTP_COULDNT_USE_REST: crate::curl_h::CURLcode = 31;

    pub const CURLE_OBSOLETE32: crate::curl_h::CURLcode = 32;

    pub const CURLE_RANGE_ERROR: crate::curl_h::CURLcode = 33;

    pub const CURLE_HTTP_POST_ERROR: crate::curl_h::CURLcode = 34;

    pub const CURLE_SSL_CONNECT_ERROR: crate::curl_h::CURLcode = 35;

    pub const CURLE_BAD_DOWNLOAD_RESUME: crate::curl_h::CURLcode = 36;

    pub const CURLE_FILE_COULDNT_READ_FILE: crate::curl_h::CURLcode = 37;

    pub const CURLE_LDAP_CANNOT_BIND: crate::curl_h::CURLcode = 38;

    pub const CURLE_LDAP_SEARCH_FAILED: crate::curl_h::CURLcode = 39;

    pub const CURLE_OBSOLETE40: crate::curl_h::CURLcode = 40;

    pub const CURLE_FUNCTION_NOT_FOUND: crate::curl_h::CURLcode = 41;

    pub const CURLE_ABORTED_BY_CALLBACK: crate::curl_h::CURLcode = 42;

    pub const CURLE_BAD_FUNCTION_ARGUMENT: crate::curl_h::CURLcode = 43;

    pub const CURLE_OBSOLETE44: crate::curl_h::CURLcode = 44;

    pub const CURLE_INTERFACE_FAILED: crate::curl_h::CURLcode = 45;

    pub const CURLE_OBSOLETE46: crate::curl_h::CURLcode = 46;

    pub const CURLE_TOO_MANY_REDIRECTS: crate::curl_h::CURLcode = 47;

    pub const CURLE_UNKNOWN_OPTION: crate::curl_h::CURLcode = 48;

    pub const CURLE_TELNET_OPTION_SYNTAX: crate::curl_h::CURLcode = 49;

    pub const CURLE_OBSOLETE50: crate::curl_h::CURLcode = 50;

    pub const CURLE_PEER_FAILED_VERIFICATION: crate::curl_h::CURLcode = 51;

    pub const CURLE_GOT_NOTHING: crate::curl_h::CURLcode = 52;

    pub const CURLE_SSL_ENGINE_NOTFOUND: crate::curl_h::CURLcode = 53;

    pub const CURLE_SSL_ENGINE_SETFAILED: crate::curl_h::CURLcode = 54;

    pub const CURLE_SEND_ERROR: crate::curl_h::CURLcode = 55;

    pub const CURLE_RECV_ERROR: crate::curl_h::CURLcode = 56;

    pub const CURLE_OBSOLETE57: crate::curl_h::CURLcode = 57;

    pub const CURLE_SSL_CERTPROBLEM: crate::curl_h::CURLcode = 58;

    pub const CURLE_SSL_CIPHER: crate::curl_h::CURLcode = 59;

    pub const CURLE_SSL_CACERT: crate::curl_h::CURLcode = 60;

    pub const CURLE_BAD_CONTENT_ENCODING: crate::curl_h::CURLcode = 61;

    pub const CURLE_LDAP_INVALID_URL: crate::curl_h::CURLcode = 62;

    pub const CURLE_FILESIZE_EXCEEDED: crate::curl_h::CURLcode = 63;

    pub const CURLE_USE_SSL_FAILED: crate::curl_h::CURLcode = 64;

    pub const CURLE_SEND_FAIL_REWIND: crate::curl_h::CURLcode = 65;

    pub const CURLE_SSL_ENGINE_INITFAILED: crate::curl_h::CURLcode = 66;

    pub const CURLE_LOGIN_DENIED: crate::curl_h::CURLcode = 67;

    pub const CURLE_TFTP_NOTFOUND: crate::curl_h::CURLcode = 68;

    pub const CURLE_TFTP_PERM: crate::curl_h::CURLcode = 69;

    pub const CURLE_REMOTE_DISK_FULL: crate::curl_h::CURLcode = 70;

    pub const CURLE_TFTP_ILLEGAL: crate::curl_h::CURLcode = 71;

    pub const CURLE_TFTP_UNKNOWNID: crate::curl_h::CURLcode = 72;

    pub const CURLE_REMOTE_FILE_EXISTS: crate::curl_h::CURLcode = 73;

    pub const CURLE_TFTP_NOSUCHUSER: crate::curl_h::CURLcode = 74;

    pub const CURLE_CONV_FAILED: crate::curl_h::CURLcode = 75;

    pub const CURLE_CONV_REQD: crate::curl_h::CURLcode = 76;

    pub const CURLE_SSL_CACERT_BADFILE: crate::curl_h::CURLcode = 77;

    pub const CURLE_REMOTE_FILE_NOT_FOUND: crate::curl_h::CURLcode = 78;

    pub const CURLE_SSH: crate::curl_h::CURLcode = 79;

    pub const CURLE_SSL_SHUTDOWN_FAILED: crate::curl_h::CURLcode = 80;

    pub const CURLE_AGAIN: crate::curl_h::CURLcode = 81;

    pub const CURLE_SSL_CRL_BADFILE: crate::curl_h::CURLcode = 82;

    pub const CURLE_SSL_ISSUER_ERROR: crate::curl_h::CURLcode = 83;

    pub const CURLE_FTP_PRET_FAILED: crate::curl_h::CURLcode = 84;

    pub const CURLE_RTSP_CSEQ_ERROR: crate::curl_h::CURLcode = 85;

    pub const CURLE_RTSP_SESSION_ERROR: crate::curl_h::CURLcode = 86;

    pub const CURLE_FTP_BAD_FILE_LIST: crate::curl_h::CURLcode = 87;

    pub const CURLE_CHUNK_FAILED: crate::curl_h::CURLcode = 88;

    pub const CURLE_NO_CONNECTION_AVAILABLE: crate::curl_h::CURLcode = 89;

    pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: crate::curl_h::CURLcode = 90;

    pub const CURLE_SSL_INVALIDCERTSTATUS: crate::curl_h::CURLcode = 91;

    pub const CURLE_HTTP2_STREAM: crate::curl_h::CURLcode = 92;

    pub const CURL_LAST: crate::curl_h::CURLcode = 93;

    pub type CURLoption = u32;

    pub const CURLOPT_WRITEDATA: crate::curl_h::CURLoption = 10001;

    pub const CURLOPT_URL: crate::curl_h::CURLoption = 10002;

    pub const CURLOPT_PORT: crate::curl_h::CURLoption = 3;

    pub const CURLOPT_PROXY: crate::curl_h::CURLoption = 10004;

    pub const CURLOPT_USERPWD: crate::curl_h::CURLoption = 10005;

    pub const CURLOPT_PROXYUSERPWD: crate::curl_h::CURLoption = 10006;

    pub const CURLOPT_RANGE: crate::curl_h::CURLoption = 10007;

    pub const CURLOPT_READDATA: crate::curl_h::CURLoption = 10009;

    pub const CURLOPT_ERRORBUFFER: crate::curl_h::CURLoption = 10010;

    pub const CURLOPT_WRITEFUNCTION: crate::curl_h::CURLoption = 20011;

    pub const CURLOPT_READFUNCTION: crate::curl_h::CURLoption = 20012;

    pub const CURLOPT_TIMEOUT: crate::curl_h::CURLoption = 13;

    pub const CURLOPT_INFILESIZE: crate::curl_h::CURLoption = 14;

    pub const CURLOPT_POSTFIELDS: crate::curl_h::CURLoption = 10015;

    pub const CURLOPT_REFERER: crate::curl_h::CURLoption = 10016;

    pub const CURLOPT_FTPPORT: crate::curl_h::CURLoption = 10017;

    pub const CURLOPT_USERAGENT: crate::curl_h::CURLoption = 10018;

    pub const CURLOPT_LOW_SPEED_LIMIT: crate::curl_h::CURLoption = 19;

    pub const CURLOPT_LOW_SPEED_TIME: crate::curl_h::CURLoption = 20;

    pub const CURLOPT_RESUME_FROM: crate::curl_h::CURLoption = 21;

    pub const CURLOPT_COOKIE: crate::curl_h::CURLoption = 10022;

    pub const CURLOPT_HTTPHEADER: crate::curl_h::CURLoption = 10023;

    pub const CURLOPT_HTTPPOST: crate::curl_h::CURLoption = 10024;

    pub const CURLOPT_SSLCERT: crate::curl_h::CURLoption = 10025;

    pub const CURLOPT_KEYPASSWD: crate::curl_h::CURLoption = 10026;

    pub const CURLOPT_CRLF: crate::curl_h::CURLoption = 27;

    pub const CURLOPT_QUOTE: crate::curl_h::CURLoption = 10028;

    pub const CURLOPT_HEADERDATA: crate::curl_h::CURLoption = 10029;

    pub const CURLOPT_COOKIEFILE: crate::curl_h::CURLoption = 10031;

    pub const CURLOPT_SSLVERSION: crate::curl_h::CURLoption = 32;

    pub const CURLOPT_TIMECONDITION: crate::curl_h::CURLoption = 33;

    pub const CURLOPT_TIMEVALUE: crate::curl_h::CURLoption = 34;

    pub const CURLOPT_CUSTOMREQUEST: crate::curl_h::CURLoption = 10036;

    pub const CURLOPT_STDERR: crate::curl_h::CURLoption = 10037;

    pub const CURLOPT_POSTQUOTE: crate::curl_h::CURLoption = 10039;

    pub const CURLOPT_OBSOLETE40: crate::curl_h::CURLoption = 10040;

    pub const CURLOPT_VERBOSE: crate::curl_h::CURLoption = 41;

    pub const CURLOPT_HEADER: crate::curl_h::CURLoption = 42;

    pub const CURLOPT_NOPROGRESS: crate::curl_h::CURLoption = 43;

    pub const CURLOPT_NOBODY: crate::curl_h::CURLoption = 44;

    pub const CURLOPT_FAILONERROR: crate::curl_h::CURLoption = 45;

    pub const CURLOPT_UPLOAD: crate::curl_h::CURLoption = 46;

    pub const CURLOPT_POST: crate::curl_h::CURLoption = 47;

    pub const CURLOPT_DIRLISTONLY: crate::curl_h::CURLoption = 48;

    pub const CURLOPT_APPEND: crate::curl_h::CURLoption = 50;

    pub const CURLOPT_NETRC: crate::curl_h::CURLoption = 51;

    pub const CURLOPT_FOLLOWLOCATION: crate::curl_h::CURLoption = 52;

    pub const CURLOPT_TRANSFERTEXT: crate::curl_h::CURLoption = 53;

    pub const CURLOPT_PUT: crate::curl_h::CURLoption = 54;

    pub const CURLOPT_PROGRESSFUNCTION: crate::curl_h::CURLoption = 20056;

    pub const CURLOPT_PROGRESSDATA: crate::curl_h::CURLoption = 10057;

    pub const CURLOPT_AUTOREFERER: crate::curl_h::CURLoption = 58;

    pub const CURLOPT_PROXYPORT: crate::curl_h::CURLoption = 59;

    pub const CURLOPT_POSTFIELDSIZE: crate::curl_h::CURLoption = 60;

    pub const CURLOPT_HTTPPROXYTUNNEL: crate::curl_h::CURLoption = 61;

    pub const CURLOPT_INTERFACE: crate::curl_h::CURLoption = 10062;

    pub const CURLOPT_KRBLEVEL: crate::curl_h::CURLoption = 10063;

    pub const CURLOPT_SSL_VERIFYPEER: crate::curl_h::CURLoption = 64;

    pub const CURLOPT_CAINFO: crate::curl_h::CURLoption = 10065;

    pub const CURLOPT_MAXREDIRS: crate::curl_h::CURLoption = 68;

    pub const CURLOPT_FILETIME: crate::curl_h::CURLoption = 69;

    pub const CURLOPT_TELNETOPTIONS: crate::curl_h::CURLoption = 10070;

    pub const CURLOPT_MAXCONNECTS: crate::curl_h::CURLoption = 71;

    pub const CURLOPT_OBSOLETE72: crate::curl_h::CURLoption = 72;

    pub const CURLOPT_FRESH_CONNECT: crate::curl_h::CURLoption = 74;

    pub const CURLOPT_FORBID_REUSE: crate::curl_h::CURLoption = 75;

    pub const CURLOPT_RANDOM_FILE: crate::curl_h::CURLoption = 10076;

    pub const CURLOPT_EGDSOCKET: crate::curl_h::CURLoption = 10077;

    pub const CURLOPT_CONNECTTIMEOUT: crate::curl_h::CURLoption = 78;

    pub const CURLOPT_HEADERFUNCTION: crate::curl_h::CURLoption = 20079;

    pub const CURLOPT_HTTPGET: crate::curl_h::CURLoption = 80;

    pub const CURLOPT_SSL_VERIFYHOST: crate::curl_h::CURLoption = 81;

    pub const CURLOPT_COOKIEJAR: crate::curl_h::CURLoption = 10082;

    pub const CURLOPT_SSL_CIPHER_LIST: crate::curl_h::CURLoption = 10083;

    pub const CURLOPT_HTTP_VERSION: crate::curl_h::CURLoption = 84;

    pub const CURLOPT_FTP_USE_EPSV: crate::curl_h::CURLoption = 85;

    pub const CURLOPT_SSLCERTTYPE: crate::curl_h::CURLoption = 10086;

    pub const CURLOPT_SSLKEY: crate::curl_h::CURLoption = 10087;

    pub const CURLOPT_SSLKEYTYPE: crate::curl_h::CURLoption = 10088;

    pub const CURLOPT_SSLENGINE: crate::curl_h::CURLoption = 10089;

    pub const CURLOPT_SSLENGINE_DEFAULT: crate::curl_h::CURLoption = 90;

    pub const CURLOPT_DNS_USE_GLOBAL_CACHE: crate::curl_h::CURLoption = 91;

    pub const CURLOPT_DNS_CACHE_TIMEOUT: crate::curl_h::CURLoption = 92;

    pub const CURLOPT_PREQUOTE: crate::curl_h::CURLoption = 10093;

    pub const CURLOPT_DEBUGFUNCTION: crate::curl_h::CURLoption = 20094;

    pub const CURLOPT_DEBUGDATA: crate::curl_h::CURLoption = 10095;

    pub const CURLOPT_COOKIESESSION: crate::curl_h::CURLoption = 96;

    pub const CURLOPT_CAPATH: crate::curl_h::CURLoption = 10097;

    pub const CURLOPT_BUFFERSIZE: crate::curl_h::CURLoption = 98;

    pub const CURLOPT_NOSIGNAL: crate::curl_h::CURLoption = 99;

    pub const CURLOPT_SHARE: crate::curl_h::CURLoption = 10100;

    pub const CURLOPT_PROXYTYPE: crate::curl_h::CURLoption = 101;

    pub const CURLOPT_ACCEPT_ENCODING: crate::curl_h::CURLoption = 10102;

    pub const CURLOPT_PRIVATE: crate::curl_h::CURLoption = 10103;

    pub const CURLOPT_HTTP200ALIASES: crate::curl_h::CURLoption = 10104;

    pub const CURLOPT_UNRESTRICTED_AUTH: crate::curl_h::CURLoption = 105;

    pub const CURLOPT_FTP_USE_EPRT: crate::curl_h::CURLoption = 106;

    pub const CURLOPT_HTTPAUTH: crate::curl_h::CURLoption = 107;

    pub const CURLOPT_SSL_CTX_FUNCTION: crate::curl_h::CURLoption = 20108;

    pub const CURLOPT_SSL_CTX_DATA: crate::curl_h::CURLoption = 10109;

    pub const CURLOPT_FTP_CREATE_MISSING_DIRS: crate::curl_h::CURLoption = 110;

    pub const CURLOPT_PROXYAUTH: crate::curl_h::CURLoption = 111;

    pub const CURLOPT_FTP_RESPONSE_TIMEOUT: crate::curl_h::CURLoption = 112;

    pub const CURLOPT_IPRESOLVE: crate::curl_h::CURLoption = 113;

    pub const CURLOPT_MAXFILESIZE: crate::curl_h::CURLoption = 114;

    pub const CURLOPT_INFILESIZE_LARGE: crate::curl_h::CURLoption = 30115;

    pub const CURLOPT_RESUME_FROM_LARGE: crate::curl_h::CURLoption = 30116;

    pub const CURLOPT_MAXFILESIZE_LARGE: crate::curl_h::CURLoption = 30117;

    pub const CURLOPT_NETRC_FILE: crate::curl_h::CURLoption = 10118;

    pub const CURLOPT_USE_SSL: crate::curl_h::CURLoption = 119;

    pub const CURLOPT_POSTFIELDSIZE_LARGE: crate::curl_h::CURLoption = 30120;

    pub const CURLOPT_TCP_NODELAY: crate::curl_h::CURLoption = 121;

    pub const CURLOPT_FTPSSLAUTH: crate::curl_h::CURLoption = 129;

    pub const CURLOPT_IOCTLFUNCTION: crate::curl_h::CURLoption = 20130;

    pub const CURLOPT_IOCTLDATA: crate::curl_h::CURLoption = 10131;

    pub const CURLOPT_FTP_ACCOUNT: crate::curl_h::CURLoption = 10134;

    pub const CURLOPT_COOKIELIST: crate::curl_h::CURLoption = 10135;

    pub const CURLOPT_IGNORE_CONTENT_LENGTH: crate::curl_h::CURLoption = 136;

    pub const CURLOPT_FTP_SKIP_PASV_IP: crate::curl_h::CURLoption = 137;

    pub const CURLOPT_FTP_FILEMETHOD: crate::curl_h::CURLoption = 138;

    pub const CURLOPT_LOCALPORT: crate::curl_h::CURLoption = 139;

    pub const CURLOPT_LOCALPORTRANGE: crate::curl_h::CURLoption = 140;

    pub const CURLOPT_CONNECT_ONLY: crate::curl_h::CURLoption = 141;

    pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: crate::curl_h::CURLoption = 20142;

    pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: crate::curl_h::CURLoption = 20143;

    pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: crate::curl_h::CURLoption = 20144;

    pub const CURLOPT_MAX_SEND_SPEED_LARGE: crate::curl_h::CURLoption = 30145;

    pub const CURLOPT_MAX_RECV_SPEED_LARGE: crate::curl_h::CURLoption = 30146;

    pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: crate::curl_h::CURLoption = 10147;

    pub const CURLOPT_SOCKOPTFUNCTION: crate::curl_h::CURLoption = 20148;

    pub const CURLOPT_SOCKOPTDATA: crate::curl_h::CURLoption = 10149;

    pub const CURLOPT_SSL_SESSIONID_CACHE: crate::curl_h::CURLoption = 150;

    pub const CURLOPT_SSH_AUTH_TYPES: crate::curl_h::CURLoption = 151;

    pub const CURLOPT_SSH_PUBLIC_KEYFILE: crate::curl_h::CURLoption = 10152;

    pub const CURLOPT_SSH_PRIVATE_KEYFILE: crate::curl_h::CURLoption = 10153;

    pub const CURLOPT_FTP_SSL_CCC: crate::curl_h::CURLoption = 154;

    pub const CURLOPT_TIMEOUT_MS: crate::curl_h::CURLoption = 155;

    pub const CURLOPT_CONNECTTIMEOUT_MS: crate::curl_h::CURLoption = 156;

    pub const CURLOPT_HTTP_TRANSFER_DECODING: crate::curl_h::CURLoption = 157;

    pub const CURLOPT_HTTP_CONTENT_DECODING: crate::curl_h::CURLoption = 158;

    pub const CURLOPT_NEW_FILE_PERMS: crate::curl_h::CURLoption = 159;

    pub const CURLOPT_NEW_DIRECTORY_PERMS: crate::curl_h::CURLoption = 160;

    pub const CURLOPT_POSTREDIR: crate::curl_h::CURLoption = 161;

    pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: crate::curl_h::CURLoption = 10162;

    pub const CURLOPT_OPENSOCKETFUNCTION: crate::curl_h::CURLoption = 20163;

    pub const CURLOPT_OPENSOCKETDATA: crate::curl_h::CURLoption = 10164;

    pub const CURLOPT_COPYPOSTFIELDS: crate::curl_h::CURLoption = 10165;

    pub const CURLOPT_PROXY_TRANSFER_MODE: crate::curl_h::CURLoption = 166;

    pub const CURLOPT_SEEKFUNCTION: crate::curl_h::CURLoption = 20167;

    pub const CURLOPT_SEEKDATA: crate::curl_h::CURLoption = 10168;

    pub const CURLOPT_CRLFILE: crate::curl_h::CURLoption = 10169;

    pub const CURLOPT_ISSUERCERT: crate::curl_h::CURLoption = 10170;

    pub const CURLOPT_ADDRESS_SCOPE: crate::curl_h::CURLoption = 171;

    pub const CURLOPT_CERTINFO: crate::curl_h::CURLoption = 172;

    pub const CURLOPT_USERNAME: crate::curl_h::CURLoption = 10173;

    pub const CURLOPT_PASSWORD: crate::curl_h::CURLoption = 10174;

    pub const CURLOPT_PROXYUSERNAME: crate::curl_h::CURLoption = 10175;

    pub const CURLOPT_PROXYPASSWORD: crate::curl_h::CURLoption = 10176;

    pub const CURLOPT_NOPROXY: crate::curl_h::CURLoption = 10177;

    pub const CURLOPT_TFTP_BLKSIZE: crate::curl_h::CURLoption = 178;

    pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: crate::curl_h::CURLoption = 10179;

    pub const CURLOPT_SOCKS5_GSSAPI_NEC: crate::curl_h::CURLoption = 180;

    pub const CURLOPT_PROTOCOLS: crate::curl_h::CURLoption = 181;

    pub const CURLOPT_REDIR_PROTOCOLS: crate::curl_h::CURLoption = 182;

    pub const CURLOPT_SSH_KNOWNHOSTS: crate::curl_h::CURLoption = 10183;

    pub const CURLOPT_SSH_KEYFUNCTION: crate::curl_h::CURLoption = 20184;

    pub const CURLOPT_SSH_KEYDATA: crate::curl_h::CURLoption = 10185;

    pub const CURLOPT_MAIL_FROM: crate::curl_h::CURLoption = 10186;

    pub const CURLOPT_MAIL_RCPT: crate::curl_h::CURLoption = 10187;

    pub const CURLOPT_FTP_USE_PRET: crate::curl_h::CURLoption = 188;

    pub const CURLOPT_RTSP_REQUEST: crate::curl_h::CURLoption = 189;

    pub const CURLOPT_RTSP_SESSION_ID: crate::curl_h::CURLoption = 10190;

    pub const CURLOPT_RTSP_STREAM_URI: crate::curl_h::CURLoption = 10191;

    pub const CURLOPT_RTSP_TRANSPORT: crate::curl_h::CURLoption = 10192;

    pub const CURLOPT_RTSP_CLIENT_CSEQ: crate::curl_h::CURLoption = 193;

    pub const CURLOPT_RTSP_SERVER_CSEQ: crate::curl_h::CURLoption = 194;

    pub const CURLOPT_INTERLEAVEDATA: crate::curl_h::CURLoption = 10195;

    pub const CURLOPT_INTERLEAVEFUNCTION: crate::curl_h::CURLoption = 20196;

    pub const CURLOPT_WILDCARDMATCH: crate::curl_h::CURLoption = 197;

    pub const CURLOPT_CHUNK_BGN_FUNCTION: crate::curl_h::CURLoption = 20198;

    pub const CURLOPT_CHUNK_END_FUNCTION: crate::curl_h::CURLoption = 20199;

    pub const CURLOPT_FNMATCH_FUNCTION: crate::curl_h::CURLoption = 20200;

    pub const CURLOPT_CHUNK_DATA: crate::curl_h::CURLoption = 10201;

    pub const CURLOPT_FNMATCH_DATA: crate::curl_h::CURLoption = 10202;

    pub const CURLOPT_RESOLVE: crate::curl_h::CURLoption = 10203;

    pub const CURLOPT_TLSAUTH_USERNAME: crate::curl_h::CURLoption = 10204;

    pub const CURLOPT_TLSAUTH_PASSWORD: crate::curl_h::CURLoption = 10205;

    pub const CURLOPT_TLSAUTH_TYPE: crate::curl_h::CURLoption = 10206;

    pub const CURLOPT_TRANSFER_ENCODING: crate::curl_h::CURLoption = 207;

    pub const CURLOPT_CLOSESOCKETFUNCTION: crate::curl_h::CURLoption = 20208;

    pub const CURLOPT_CLOSESOCKETDATA: crate::curl_h::CURLoption = 10209;

    pub const CURLOPT_GSSAPI_DELEGATION: crate::curl_h::CURLoption = 210;

    pub const CURLOPT_DNS_SERVERS: crate::curl_h::CURLoption = 10211;

    pub const CURLOPT_ACCEPTTIMEOUT_MS: crate::curl_h::CURLoption = 212;

    pub const CURLOPT_TCP_KEEPALIVE: crate::curl_h::CURLoption = 213;

    pub const CURLOPT_TCP_KEEPIDLE: crate::curl_h::CURLoption = 214;

    pub const CURLOPT_TCP_KEEPINTVL: crate::curl_h::CURLoption = 215;

    pub const CURLOPT_SSL_OPTIONS: crate::curl_h::CURLoption = 216;

    pub const CURLOPT_MAIL_AUTH: crate::curl_h::CURLoption = 10217;

    pub const CURLOPT_SASL_IR: crate::curl_h::CURLoption = 218;

    pub const CURLOPT_XFERINFOFUNCTION: crate::curl_h::CURLoption = 20219;

    pub const CURLOPT_XOAUTH2_BEARER: crate::curl_h::CURLoption = 10220;

    pub const CURLOPT_DNS_INTERFACE: crate::curl_h::CURLoption = 10221;

    pub const CURLOPT_DNS_LOCAL_IP4: crate::curl_h::CURLoption = 10222;

    pub const CURLOPT_DNS_LOCAL_IP6: crate::curl_h::CURLoption = 10223;

    pub const CURLOPT_LOGIN_OPTIONS: crate::curl_h::CURLoption = 10224;

    pub const CURLOPT_SSL_ENABLE_NPN: crate::curl_h::CURLoption = 225;

    pub const CURLOPT_SSL_ENABLE_ALPN: crate::curl_h::CURLoption = 226;

    pub const CURLOPT_EXPECT_100_TIMEOUT_MS: crate::curl_h::CURLoption = 227;

    pub const CURLOPT_PROXYHEADER: crate::curl_h::CURLoption = 10228;

    pub const CURLOPT_HEADEROPT: crate::curl_h::CURLoption = 229;

    pub const CURLOPT_PINNEDPUBLICKEY: crate::curl_h::CURLoption = 10230;

    pub const CURLOPT_UNIX_SOCKET_PATH: crate::curl_h::CURLoption = 10231;

    pub const CURLOPT_SSL_VERIFYSTATUS: crate::curl_h::CURLoption = 232;

    pub const CURLOPT_SSL_FALSESTART: crate::curl_h::CURLoption = 233;

    pub const CURLOPT_PATH_AS_IS: crate::curl_h::CURLoption = 234;

    pub const CURLOPT_PROXY_SERVICE_NAME: crate::curl_h::CURLoption = 10235;

    pub const CURLOPT_SERVICE_NAME: crate::curl_h::CURLoption = 10236;

    pub const CURLOPT_PIPEWAIT: crate::curl_h::CURLoption = 237;

    pub const CURLOPT_DEFAULT_PROTOCOL: crate::curl_h::CURLoption = 10238;

    pub const CURLOPT_STREAM_WEIGHT: crate::curl_h::CURLoption = 239;

    pub const CURLOPT_STREAM_DEPENDS: crate::curl_h::CURLoption = 10240;

    pub const CURLOPT_STREAM_DEPENDS_E: crate::curl_h::CURLoption = 10241;

    pub const CURLOPT_TFTP_NO_OPTIONS: crate::curl_h::CURLoption = 242;

    pub const CURLOPT_CONNECT_TO: crate::curl_h::CURLoption = 10243;

    pub const CURLOPT_TCP_FASTOPEN: crate::curl_h::CURLoption = 244;

    pub const CURLOPT_KEEP_SENDING_ON_ERROR: crate::curl_h::CURLoption = 245;

    pub const CURLOPT_PROXY_CAINFO: crate::curl_h::CURLoption = 10246;

    pub const CURLOPT_PROXY_CAPATH: crate::curl_h::CURLoption = 10247;

    pub const CURLOPT_PROXY_SSL_VERIFYPEER: crate::curl_h::CURLoption = 248;

    pub const CURLOPT_PROXY_SSL_VERIFYHOST: crate::curl_h::CURLoption = 249;

    pub const CURLOPT_PROXY_SSLVERSION: crate::curl_h::CURLoption = 250;

    pub const CURLOPT_PROXY_TLSAUTH_USERNAME: crate::curl_h::CURLoption = 10251;

    pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: crate::curl_h::CURLoption = 10252;

    pub const CURLOPT_PROXY_TLSAUTH_TYPE: crate::curl_h::CURLoption = 10253;

    pub const CURLOPT_PROXY_SSLCERT: crate::curl_h::CURLoption = 10254;

    pub const CURLOPT_PROXY_SSLCERTTYPE: crate::curl_h::CURLoption = 10255;

    pub const CURLOPT_PROXY_SSLKEY: crate::curl_h::CURLoption = 10256;

    pub const CURLOPT_PROXY_SSLKEYTYPE: crate::curl_h::CURLoption = 10257;

    pub const CURLOPT_PROXY_KEYPASSWD: crate::curl_h::CURLoption = 10258;

    pub const CURLOPT_PROXY_SSL_CIPHER_LIST: crate::curl_h::CURLoption = 10259;

    pub const CURLOPT_PROXY_CRLFILE: crate::curl_h::CURLoption = 10260;

    pub const CURLOPT_PROXY_SSL_OPTIONS: crate::curl_h::CURLoption = 261;

    pub const CURLOPT_PRE_PROXY: crate::curl_h::CURLoption = 10262;

    pub const CURLOPT_PROXY_PINNEDPUBLICKEY: crate::curl_h::CURLoption = 10263;

    pub const CURLOPT_ABSTRACT_UNIX_SOCKET: crate::curl_h::CURLoption = 10264;

    pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: crate::curl_h::CURLoption = 265;

    pub const CURLOPT_LASTENTRY: crate::curl_h::CURLoption = 266;

    pub type CURLINFO = u32;

    pub const CURLINFO_NONE: crate::curl_h::CURLINFO = 0;

    pub const CURLINFO_EFFECTIVE_URL: crate::curl_h::CURLINFO = 1048577;

    pub const CURLINFO_RESPONSE_CODE: crate::curl_h::CURLINFO = 2097154;

    pub const CURLINFO_TOTAL_TIME: crate::curl_h::CURLINFO = 3145731;

    pub const CURLINFO_NAMELOOKUP_TIME: crate::curl_h::CURLINFO = 3145732;

    pub const CURLINFO_CONNECT_TIME: crate::curl_h::CURLINFO = 3145733;

    pub const CURLINFO_PRETRANSFER_TIME: crate::curl_h::CURLINFO = 3145734;

    pub const CURLINFO_SIZE_UPLOAD: crate::curl_h::CURLINFO = 3145735;

    pub const CURLINFO_SIZE_DOWNLOAD: crate::curl_h::CURLINFO = 3145736;

    pub const CURLINFO_SPEED_DOWNLOAD: crate::curl_h::CURLINFO = 3145737;

    pub const CURLINFO_SPEED_UPLOAD: crate::curl_h::CURLINFO = 3145738;

    pub const CURLINFO_HEADER_SIZE: crate::curl_h::CURLINFO = 2097163;

    pub const CURLINFO_REQUEST_SIZE: crate::curl_h::CURLINFO = 2097164;

    pub const CURLINFO_SSL_VERIFYRESULT: crate::curl_h::CURLINFO = 2097165;

    pub const CURLINFO_FILETIME: crate::curl_h::CURLINFO = 2097166;

    pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: crate::curl_h::CURLINFO = 3145743;

    pub const CURLINFO_CONTENT_LENGTH_UPLOAD: crate::curl_h::CURLINFO = 3145744;

    pub const CURLINFO_STARTTRANSFER_TIME: crate::curl_h::CURLINFO = 3145745;

    pub const CURLINFO_CONTENT_TYPE: crate::curl_h::CURLINFO = 1048594;

    pub const CURLINFO_REDIRECT_TIME: crate::curl_h::CURLINFO = 3145747;

    pub const CURLINFO_REDIRECT_COUNT: crate::curl_h::CURLINFO = 2097172;

    pub const CURLINFO_PRIVATE: crate::curl_h::CURLINFO = 1048597;

    pub const CURLINFO_HTTP_CONNECTCODE: crate::curl_h::CURLINFO = 2097174;

    pub const CURLINFO_HTTPAUTH_AVAIL: crate::curl_h::CURLINFO = 2097175;

    pub const CURLINFO_PROXYAUTH_AVAIL: crate::curl_h::CURLINFO = 2097176;

    pub const CURLINFO_OS_ERRNO: crate::curl_h::CURLINFO = 2097177;

    pub const CURLINFO_NUM_CONNECTS: crate::curl_h::CURLINFO = 2097178;

    pub const CURLINFO_SSL_ENGINES: crate::curl_h::CURLINFO = 4194331;

    pub const CURLINFO_COOKIELIST: crate::curl_h::CURLINFO = 4194332;

    pub const CURLINFO_LASTSOCKET: crate::curl_h::CURLINFO = 2097181;

    pub const CURLINFO_FTP_ENTRY_PATH: crate::curl_h::CURLINFO = 1048606;

    pub const CURLINFO_REDIRECT_URL: crate::curl_h::CURLINFO = 1048607;

    pub const CURLINFO_PRIMARY_IP: crate::curl_h::CURLINFO = 1048608;

    pub const CURLINFO_APPCONNECT_TIME: crate::curl_h::CURLINFO = 3145761;

    pub const CURLINFO_CERTINFO: crate::curl_h::CURLINFO = 4194338;

    pub const CURLINFO_CONDITION_UNMET: crate::curl_h::CURLINFO = 2097187;

    pub const CURLINFO_RTSP_SESSION_ID: crate::curl_h::CURLINFO = 1048612;

    pub const CURLINFO_RTSP_CLIENT_CSEQ: crate::curl_h::CURLINFO = 2097189;

    pub const CURLINFO_RTSP_SERVER_CSEQ: crate::curl_h::CURLINFO = 2097190;

    pub const CURLINFO_RTSP_CSEQ_RECV: crate::curl_h::CURLINFO = 2097191;

    pub const CURLINFO_PRIMARY_PORT: crate::curl_h::CURLINFO = 2097192;

    pub const CURLINFO_LOCAL_IP: crate::curl_h::CURLINFO = 1048617;

    pub const CURLINFO_LOCAL_PORT: crate::curl_h::CURLINFO = 2097194;

    pub const CURLINFO_TLS_SESSION: crate::curl_h::CURLINFO = 4194347;

    pub const CURLINFO_ACTIVESOCKET: crate::curl_h::CURLINFO = 5242924;

    pub const CURLINFO_TLS_SSL_PTR: crate::curl_h::CURLINFO = 4194349;

    pub const CURLINFO_HTTP_VERSION: crate::curl_h::CURLINFO = 2097198;

    pub const CURLINFO_PROXY_SSL_VERIFYRESULT: crate::curl_h::CURLINFO = 2097199;

    pub const CURLINFO_PROTOCOL: crate::curl_h::CURLINFO = 2097200;

    pub const CURLINFO_SCHEME: crate::curl_h::CURLINFO = 1048625;

    pub const CURLINFO_LASTONE: crate::curl_h::CURLINFO = 49;
}
pub mod bg_public_h {
    pub const GT_FFA: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const GT_TOURNAMENT: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const GT_SINGLE_PLAYER: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const GT_TEAM: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const GT_CTF: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const GT_1FCTF: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const GT_OBELISK: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const GT_HARVESTER: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const GT_MAX_GAME_TYPE: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const PM_NORMAL: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const PM_NOCLIP: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const PM_SPECTATOR: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const PM_DEAD: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const PM_FREEZE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const PM_INTERMISSION: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const PM_SPINTERMISSION: crate::be_aas_h::C2RustUnnamed_0 = 6;
}
pub mod keycodes_h {
    pub type keyNum_t = u32;

    pub const K_TAB: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const K_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const K_ESCAPE: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const K_SPACE: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const K_BACKSPACE: crate::be_aas_h::C2RustUnnamed_0 = 127;

    pub const K_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 128;

    pub const K_CAPSLOCK: crate::be_aas_h::C2RustUnnamed_0 = 129;

    pub const K_POWER: crate::be_aas_h::C2RustUnnamed_0 = 130;

    pub const K_PAUSE: crate::be_aas_h::C2RustUnnamed_0 = 131;

    pub const K_UPARROW: crate::be_aas_h::C2RustUnnamed_0 = 132;

    pub const K_DOWNARROW: crate::be_aas_h::C2RustUnnamed_0 = 133;

    pub const K_LEFTARROW: crate::be_aas_h::C2RustUnnamed_0 = 134;

    pub const K_RIGHTARROW: crate::be_aas_h::C2RustUnnamed_0 = 135;

    pub const K_ALT: crate::be_aas_h::C2RustUnnamed_0 = 136;

    pub const K_CTRL: crate::be_aas_h::C2RustUnnamed_0 = 137;

    pub const K_SHIFT: crate::be_aas_h::C2RustUnnamed_0 = 138;

    pub const K_INS: crate::be_aas_h::C2RustUnnamed_0 = 139;

    pub const K_DEL: crate::be_aas_h::C2RustUnnamed_0 = 140;

    pub const K_PGDN: crate::be_aas_h::C2RustUnnamed_0 = 141;

    pub const K_PGUP: crate::be_aas_h::C2RustUnnamed_0 = 142;

    pub const K_HOME: crate::be_aas_h::C2RustUnnamed_0 = 143;

    pub const K_END: crate::be_aas_h::C2RustUnnamed_0 = 144;

    pub const K_F1: crate::be_aas_h::C2RustUnnamed_0 = 145;

    pub const K_F2: crate::be_aas_h::C2RustUnnamed_0 = 146;

    pub const K_F3: crate::be_aas_h::C2RustUnnamed_0 = 147;

    pub const K_F4: crate::be_aas_h::C2RustUnnamed_0 = 148;

    pub const K_F5: crate::be_aas_h::C2RustUnnamed_0 = 149;

    pub const K_F6: crate::be_aas_h::C2RustUnnamed_0 = 150;

    pub const K_F7: crate::be_aas_h::C2RustUnnamed_0 = 151;

    pub const K_F8: crate::be_aas_h::C2RustUnnamed_0 = 152;

    pub const K_F9: crate::be_aas_h::C2RustUnnamed_0 = 153;

    pub const K_F10: crate::be_aas_h::C2RustUnnamed_0 = 154;

    pub const K_F11: crate::be_aas_h::C2RustUnnamed_0 = 155;

    pub const K_F12: crate::be_aas_h::C2RustUnnamed_0 = 156;

    pub const K_F13: crate::be_aas_h::C2RustUnnamed_0 = 157;

    pub const K_F14: crate::be_aas_h::C2RustUnnamed_0 = 158;

    pub const K_F15: crate::be_aas_h::C2RustUnnamed_0 = 159;

    pub const K_KP_HOME: crate::be_aas_h::C2RustUnnamed_0 = 160;

    pub const K_KP_UPARROW: crate::be_aas_h::C2RustUnnamed_0 = 161;

    pub const K_KP_PGUP: crate::be_aas_h::C2RustUnnamed_0 = 162;

    pub const K_KP_LEFTARROW: crate::be_aas_h::C2RustUnnamed_0 = 163;

    pub const K_KP_5: crate::be_aas_h::C2RustUnnamed_0 = 164;

    pub const K_KP_RIGHTARROW: crate::be_aas_h::C2RustUnnamed_0 = 165;

    pub const K_KP_END: crate::be_aas_h::C2RustUnnamed_0 = 166;

    pub const K_KP_DOWNARROW: crate::be_aas_h::C2RustUnnamed_0 = 167;

    pub const K_KP_PGDN: crate::be_aas_h::C2RustUnnamed_0 = 168;

    pub const K_KP_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 169;

    pub const K_KP_INS: crate::be_aas_h::C2RustUnnamed_0 = 170;

    pub const K_KP_DEL: crate::be_aas_h::C2RustUnnamed_0 = 171;

    pub const K_KP_SLASH: crate::be_aas_h::C2RustUnnamed_0 = 172;

    pub const K_KP_MINUS: crate::be_aas_h::C2RustUnnamed_0 = 173;

    pub const K_KP_PLUS: crate::be_aas_h::C2RustUnnamed_0 = 174;

    pub const K_KP_NUMLOCK: crate::be_aas_h::C2RustUnnamed_0 = 175;

    pub const K_KP_STAR: crate::be_aas_h::C2RustUnnamed_0 = 176;

    pub const K_KP_EQUALS: crate::be_aas_h::C2RustUnnamed_0 = 177;

    pub const K_MOUSE1: crate::be_aas_h::C2RustUnnamed_0 = 178;

    pub const K_MOUSE2: crate::be_aas_h::C2RustUnnamed_0 = 179;

    pub const K_MOUSE3: crate::be_aas_h::C2RustUnnamed_0 = 180;

    pub const K_MOUSE4: crate::be_aas_h::C2RustUnnamed_0 = 181;

    pub const K_MOUSE5: crate::be_aas_h::C2RustUnnamed_0 = 182;

    pub const K_MWHEELDOWN: crate::be_aas_h::C2RustUnnamed_0 = 183;

    pub const K_MWHEELUP: crate::be_aas_h::C2RustUnnamed_0 = 184;

    pub const K_JOY1: crate::be_aas_h::C2RustUnnamed_0 = 185;

    pub const K_JOY2: crate::be_aas_h::C2RustUnnamed_0 = 186;

    pub const K_JOY3: crate::be_aas_h::C2RustUnnamed_0 = 187;

    pub const K_JOY4: crate::be_aas_h::C2RustUnnamed_0 = 188;

    pub const K_JOY5: crate::be_aas_h::C2RustUnnamed_0 = 189;

    pub const K_JOY6: crate::be_aas_h::C2RustUnnamed_0 = 190;

    pub const K_JOY7: crate::be_aas_h::C2RustUnnamed_0 = 191;

    pub const K_JOY8: crate::be_aas_h::C2RustUnnamed_0 = 192;

    pub const K_JOY9: crate::be_aas_h::C2RustUnnamed_0 = 193;

    pub const K_JOY10: crate::be_aas_h::C2RustUnnamed_0 = 194;

    pub const K_JOY11: crate::be_aas_h::C2RustUnnamed_0 = 195;

    pub const K_JOY12: crate::be_aas_h::C2RustUnnamed_0 = 196;

    pub const K_JOY13: crate::be_aas_h::C2RustUnnamed_0 = 197;

    pub const K_JOY14: crate::be_aas_h::C2RustUnnamed_0 = 198;

    pub const K_JOY15: crate::be_aas_h::C2RustUnnamed_0 = 199;

    pub const K_JOY16: crate::be_aas_h::C2RustUnnamed_0 = 200;

    pub const K_JOY17: crate::be_aas_h::C2RustUnnamed_0 = 201;

    pub const K_JOY18: crate::be_aas_h::C2RustUnnamed_0 = 202;

    pub const K_JOY19: crate::be_aas_h::C2RustUnnamed_0 = 203;

    pub const K_JOY20: crate::be_aas_h::C2RustUnnamed_0 = 204;

    pub const K_JOY21: crate::be_aas_h::C2RustUnnamed_0 = 205;

    pub const K_JOY22: crate::be_aas_h::C2RustUnnamed_0 = 206;

    pub const K_JOY23: crate::be_aas_h::C2RustUnnamed_0 = 207;

    pub const K_JOY24: crate::be_aas_h::C2RustUnnamed_0 = 208;

    pub const K_JOY25: crate::be_aas_h::C2RustUnnamed_0 = 209;

    pub const K_JOY26: crate::be_aas_h::C2RustUnnamed_0 = 210;

    pub const K_JOY27: crate::be_aas_h::C2RustUnnamed_0 = 211;

    pub const K_JOY28: crate::be_aas_h::C2RustUnnamed_0 = 212;

    pub const K_JOY29: crate::be_aas_h::C2RustUnnamed_0 = 213;

    pub const K_JOY30: crate::be_aas_h::C2RustUnnamed_0 = 214;

    pub const K_JOY31: crate::be_aas_h::C2RustUnnamed_0 = 215;

    pub const K_JOY32: crate::be_aas_h::C2RustUnnamed_0 = 216;

    pub const K_AUX1: crate::be_aas_h::C2RustUnnamed_0 = 217;

    pub const K_AUX2: crate::be_aas_h::C2RustUnnamed_0 = 218;

    pub const K_AUX3: crate::be_aas_h::C2RustUnnamed_0 = 219;

    pub const K_AUX4: crate::be_aas_h::C2RustUnnamed_0 = 220;

    pub const K_AUX5: crate::be_aas_h::C2RustUnnamed_0 = 221;

    pub const K_AUX6: crate::be_aas_h::C2RustUnnamed_0 = 222;

    pub const K_AUX7: crate::be_aas_h::C2RustUnnamed_0 = 223;

    pub const K_AUX8: crate::be_aas_h::C2RustUnnamed_0 = 224;

    pub const K_AUX9: crate::be_aas_h::C2RustUnnamed_0 = 225;

    pub const K_AUX10: crate::be_aas_h::C2RustUnnamed_0 = 226;

    pub const K_AUX11: crate::be_aas_h::C2RustUnnamed_0 = 227;

    pub const K_AUX12: crate::be_aas_h::C2RustUnnamed_0 = 228;

    pub const K_AUX13: crate::be_aas_h::C2RustUnnamed_0 = 229;

    pub const K_AUX14: crate::be_aas_h::C2RustUnnamed_0 = 230;

    pub const K_AUX15: crate::be_aas_h::C2RustUnnamed_0 = 231;

    pub const K_AUX16: crate::be_aas_h::C2RustUnnamed_0 = 232;

    pub const K_WORLD_0: crate::be_aas_h::C2RustUnnamed_0 = 233;

    pub const K_WORLD_1: crate::be_aas_h::C2RustUnnamed_0 = 234;

    pub const K_WORLD_2: crate::be_aas_h::C2RustUnnamed_0 = 235;

    pub const K_WORLD_3: crate::be_aas_h::C2RustUnnamed_0 = 236;

    pub const K_WORLD_4: crate::be_aas_h::C2RustUnnamed_0 = 237;

    pub const K_WORLD_5: crate::be_aas_h::C2RustUnnamed_0 = 238;

    pub const K_WORLD_6: crate::be_aas_h::C2RustUnnamed_0 = 239;

    pub const K_WORLD_7: crate::be_aas_h::C2RustUnnamed_0 = 240;

    pub const K_WORLD_8: crate::be_aas_h::C2RustUnnamed_0 = 241;

    pub const K_WORLD_9: crate::be_aas_h::C2RustUnnamed_0 = 242;

    pub const K_WORLD_10: crate::be_aas_h::C2RustUnnamed_0 = 243;

    pub const K_WORLD_11: crate::be_aas_h::C2RustUnnamed_0 = 244;

    pub const K_WORLD_12: crate::be_aas_h::C2RustUnnamed_0 = 245;

    pub const K_WORLD_13: crate::be_aas_h::C2RustUnnamed_0 = 246;

    pub const K_WORLD_14: crate::be_aas_h::C2RustUnnamed_0 = 247;

    pub const K_WORLD_15: crate::be_aas_h::C2RustUnnamed_0 = 248;

    pub const K_WORLD_16: crate::be_aas_h::C2RustUnnamed_0 = 249;

    pub const K_WORLD_17: crate::be_aas_h::C2RustUnnamed_0 = 250;

    pub const K_WORLD_18: crate::be_aas_h::C2RustUnnamed_0 = 251;

    pub const K_WORLD_19: crate::be_aas_h::C2RustUnnamed_0 = 252;

    pub const K_WORLD_20: crate::be_aas_h::C2RustUnnamed_0 = 253;

    pub const K_WORLD_21: crate::be_aas_h::C2RustUnnamed_0 = 254;

    pub const K_WORLD_22: crate::be_aas_h::C2RustUnnamed_0 = 255;

    pub const K_WORLD_23: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const K_WORLD_24: crate::be_aas_h::C2RustUnnamed_0 = 257;

    pub const K_WORLD_25: crate::be_aas_h::C2RustUnnamed_0 = 258;

    pub const K_WORLD_26: crate::be_aas_h::C2RustUnnamed_0 = 259;

    pub const K_WORLD_27: crate::be_aas_h::C2RustUnnamed_0 = 260;

    pub const K_WORLD_28: crate::be_aas_h::C2RustUnnamed_0 = 261;

    pub const K_WORLD_29: crate::be_aas_h::C2RustUnnamed_0 = 262;

    pub const K_WORLD_30: crate::be_aas_h::C2RustUnnamed_0 = 263;

    pub const K_WORLD_31: crate::be_aas_h::C2RustUnnamed_0 = 264;

    pub const K_WORLD_32: crate::be_aas_h::C2RustUnnamed_0 = 265;

    pub const K_WORLD_33: crate::be_aas_h::C2RustUnnamed_0 = 266;

    pub const K_WORLD_34: crate::be_aas_h::C2RustUnnamed_0 = 267;

    pub const K_WORLD_35: crate::be_aas_h::C2RustUnnamed_0 = 268;

    pub const K_WORLD_36: crate::be_aas_h::C2RustUnnamed_0 = 269;

    pub const K_WORLD_37: crate::be_aas_h::C2RustUnnamed_0 = 270;

    pub const K_WORLD_38: crate::be_aas_h::C2RustUnnamed_0 = 271;

    pub const K_WORLD_39: crate::be_aas_h::C2RustUnnamed_0 = 272;

    pub const K_WORLD_40: crate::be_aas_h::C2RustUnnamed_0 = 273;

    pub const K_WORLD_41: crate::be_aas_h::C2RustUnnamed_0 = 274;

    pub const K_WORLD_42: crate::be_aas_h::C2RustUnnamed_0 = 275;

    pub const K_WORLD_43: crate::be_aas_h::C2RustUnnamed_0 = 276;

    pub const K_WORLD_44: crate::be_aas_h::C2RustUnnamed_0 = 277;

    pub const K_WORLD_45: crate::be_aas_h::C2RustUnnamed_0 = 278;

    pub const K_WORLD_46: crate::be_aas_h::C2RustUnnamed_0 = 279;

    pub const K_WORLD_47: crate::be_aas_h::C2RustUnnamed_0 = 280;

    pub const K_WORLD_48: crate::be_aas_h::C2RustUnnamed_0 = 281;

    pub const K_WORLD_49: crate::be_aas_h::C2RustUnnamed_0 = 282;

    pub const K_WORLD_50: crate::be_aas_h::C2RustUnnamed_0 = 283;

    pub const K_WORLD_51: crate::be_aas_h::C2RustUnnamed_0 = 284;

    pub const K_WORLD_52: crate::be_aas_h::C2RustUnnamed_0 = 285;

    pub const K_WORLD_53: crate::be_aas_h::C2RustUnnamed_0 = 286;

    pub const K_WORLD_54: crate::be_aas_h::C2RustUnnamed_0 = 287;

    pub const K_WORLD_55: crate::be_aas_h::C2RustUnnamed_0 = 288;

    pub const K_WORLD_56: crate::be_aas_h::C2RustUnnamed_0 = 289;

    pub const K_WORLD_57: crate::be_aas_h::C2RustUnnamed_0 = 290;

    pub const K_WORLD_58: crate::be_aas_h::C2RustUnnamed_0 = 291;

    pub const K_WORLD_59: crate::be_aas_h::C2RustUnnamed_0 = 292;

    pub const K_WORLD_60: crate::be_aas_h::C2RustUnnamed_0 = 293;

    pub const K_WORLD_61: crate::be_aas_h::C2RustUnnamed_0 = 294;

    pub const K_WORLD_62: crate::be_aas_h::C2RustUnnamed_0 = 295;

    pub const K_WORLD_63: crate::be_aas_h::C2RustUnnamed_0 = 296;

    pub const K_WORLD_64: crate::be_aas_h::C2RustUnnamed_0 = 297;

    pub const K_WORLD_65: crate::be_aas_h::C2RustUnnamed_0 = 298;

    pub const K_WORLD_66: crate::be_aas_h::C2RustUnnamed_0 = 299;

    pub const K_WORLD_67: crate::be_aas_h::C2RustUnnamed_0 = 300;

    pub const K_WORLD_68: crate::be_aas_h::C2RustUnnamed_0 = 301;

    pub const K_WORLD_69: crate::be_aas_h::C2RustUnnamed_0 = 302;

    pub const K_WORLD_70: crate::be_aas_h::C2RustUnnamed_0 = 303;

    pub const K_WORLD_71: crate::be_aas_h::C2RustUnnamed_0 = 304;

    pub const K_WORLD_72: crate::be_aas_h::C2RustUnnamed_0 = 305;

    pub const K_WORLD_73: crate::be_aas_h::C2RustUnnamed_0 = 306;

    pub const K_WORLD_74: crate::be_aas_h::C2RustUnnamed_0 = 307;

    pub const K_WORLD_75: crate::be_aas_h::C2RustUnnamed_0 = 308;

    pub const K_WORLD_76: crate::be_aas_h::C2RustUnnamed_0 = 309;

    pub const K_WORLD_77: crate::be_aas_h::C2RustUnnamed_0 = 310;

    pub const K_WORLD_78: crate::be_aas_h::C2RustUnnamed_0 = 311;

    pub const K_WORLD_79: crate::be_aas_h::C2RustUnnamed_0 = 312;

    pub const K_WORLD_80: crate::be_aas_h::C2RustUnnamed_0 = 313;

    pub const K_WORLD_81: crate::be_aas_h::C2RustUnnamed_0 = 314;

    pub const K_WORLD_82: crate::be_aas_h::C2RustUnnamed_0 = 315;

    pub const K_WORLD_83: crate::be_aas_h::C2RustUnnamed_0 = 316;

    pub const K_WORLD_84: crate::be_aas_h::C2RustUnnamed_0 = 317;

    pub const K_WORLD_85: crate::be_aas_h::C2RustUnnamed_0 = 318;

    pub const K_WORLD_86: crate::be_aas_h::C2RustUnnamed_0 = 319;

    pub const K_WORLD_87: crate::be_aas_h::C2RustUnnamed_0 = 320;

    pub const K_WORLD_88: crate::be_aas_h::C2RustUnnamed_0 = 321;

    pub const K_WORLD_89: crate::be_aas_h::C2RustUnnamed_0 = 322;

    pub const K_WORLD_90: crate::be_aas_h::C2RustUnnamed_0 = 323;

    pub const K_WORLD_91: crate::be_aas_h::C2RustUnnamed_0 = 324;

    pub const K_WORLD_92: crate::be_aas_h::C2RustUnnamed_0 = 325;

    pub const K_WORLD_93: crate::be_aas_h::C2RustUnnamed_0 = 326;

    pub const K_WORLD_94: crate::be_aas_h::C2RustUnnamed_0 = 327;

    pub const K_WORLD_95: crate::be_aas_h::C2RustUnnamed_0 = 328;

    pub const K_SUPER: crate::be_aas_h::C2RustUnnamed_0 = 329;

    pub const K_COMPOSE: crate::be_aas_h::C2RustUnnamed_0 = 330;

    pub const K_MODE: crate::be_aas_h::C2RustUnnamed_0 = 331;

    pub const K_HELP: crate::be_aas_h::C2RustUnnamed_0 = 332;

    pub const K_PRINT: crate::be_aas_h::C2RustUnnamed_0 = 333;

    pub const K_SYSREQ: crate::be_aas_h::C2RustUnnamed_0 = 334;

    pub const K_SCROLLOCK: crate::be_aas_h::C2RustUnnamed_0 = 335;

    pub const K_BREAK: crate::be_aas_h::C2RustUnnamed_0 = 336;

    pub const K_MENU: crate::be_aas_h::C2RustUnnamed_0 = 337;

    pub const K_EURO: crate::be_aas_h::C2RustUnnamed_0 = 338;

    pub const K_UNDO: crate::be_aas_h::C2RustUnnamed_0 = 339;

    pub const K_PAD0_A: crate::be_aas_h::C2RustUnnamed_0 = 340;

    pub const K_PAD0_B: crate::be_aas_h::C2RustUnnamed_0 = 341;

    pub const K_PAD0_X: crate::be_aas_h::C2RustUnnamed_0 = 342;

    pub const K_PAD0_Y: crate::be_aas_h::C2RustUnnamed_0 = 343;

    pub const K_PAD0_BACK: crate::be_aas_h::C2RustUnnamed_0 = 344;

    pub const K_PAD0_GUIDE: crate::be_aas_h::C2RustUnnamed_0 = 345;

    pub const K_PAD0_START: crate::be_aas_h::C2RustUnnamed_0 = 346;

    pub const K_PAD0_LEFTSTICK_CLICK: crate::be_aas_h::C2RustUnnamed_0 = 347;

    pub const K_PAD0_RIGHTSTICK_CLICK: crate::be_aas_h::C2RustUnnamed_0 = 348;

    pub const K_PAD0_LEFTSHOULDER: crate::be_aas_h::C2RustUnnamed_0 = 349;

    pub const K_PAD0_RIGHTSHOULDER: crate::be_aas_h::C2RustUnnamed_0 = 350;

    pub const K_PAD0_DPAD_UP: crate::be_aas_h::C2RustUnnamed_0 = 351;

    pub const K_PAD0_DPAD_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 352;

    pub const K_PAD0_DPAD_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 353;

    pub const K_PAD0_DPAD_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 354;

    pub const K_PAD0_LEFTSTICK_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 355;

    pub const K_PAD0_LEFTSTICK_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 356;

    pub const K_PAD0_LEFTSTICK_UP: crate::be_aas_h::C2RustUnnamed_0 = 357;

    pub const K_PAD0_LEFTSTICK_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 358;

    pub const K_PAD0_RIGHTSTICK_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 359;

    pub const K_PAD0_RIGHTSTICK_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 360;

    pub const K_PAD0_RIGHTSTICK_UP: crate::be_aas_h::C2RustUnnamed_0 = 361;

    pub const K_PAD0_RIGHTSTICK_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 362;

    pub const K_PAD0_LEFTTRIGGER: crate::be_aas_h::C2RustUnnamed_0 = 363;

    pub const K_PAD0_RIGHTTRIGGER: crate::be_aas_h::C2RustUnnamed_0 = 364;

    pub const K_CONSOLE: crate::be_aas_h::C2RustUnnamed_0 = 365;

    pub const MAX_KEYS: crate::be_aas_h::C2RustUnnamed_0 = 366;
}
pub mod ui_public_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct uiClientState_t {
        pub connState: crate::src::qcommon::q_shared::connstate_t,
        pub connectPacketCount: i32,
        pub clientNum: i32,
        pub servername: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub messageString: [libc::c_char; 1024],
    }

    pub const UI_ERROR: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const UI_PRINT: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const UI_MILLISECONDS: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const UI_CVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const UI_CVAR_VARIABLEVALUE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const UI_CVAR_VARIABLESTRINGBUFFER: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const UI_CVAR_SETVALUE: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const UI_CVAR_RESET: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const UI_CVAR_CREATE: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const UI_CVAR_INFOSTRINGBUFFER: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const UI_ARGC: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const UI_ARGV: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const UI_CMD_EXECUTETEXT: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const UI_FS_FOPENFILE: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const UI_FS_READ: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const UI_FS_WRITE: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const UI_FS_FCLOSEFILE: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const UI_FS_GETFILELIST: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const UI_R_REGISTERMODEL: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const UI_R_REGISTERSKIN: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const UI_R_REGISTERSHADERNOMIP: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const UI_R_CLEARSCENE: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const UI_R_ADDREFENTITYTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const UI_R_ADDPOLYTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const UI_R_ADDLIGHTTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const UI_R_RENDERSCENE: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const UI_R_SETCOLOR: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const UI_R_DRAWSTRETCHPIC: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const UI_UPDATESCREEN: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const UI_CM_LERPTAG: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const UI_CM_LOADMODEL: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const UI_S_REGISTERSOUND: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const UI_S_STARTLOCALSOUND: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const UI_KEY_KEYNUMTOSTRINGBUF: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const UI_KEY_GETBINDINGBUF: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const UI_KEY_SETBINDING: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const UI_KEY_ISDOWN: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const UI_KEY_GETOVERSTRIKEMODE: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const UI_KEY_SETOVERSTRIKEMODE: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const UI_KEY_CLEARSTATES: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const UI_KEY_GETCATCHER: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const UI_KEY_SETCATCHER: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const UI_GETCLIPBOARDDATA: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const UI_GETGLCONFIG: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const UI_GETCLIENTSTATE: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const UI_GETCONFIGSTRING: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const UI_LAN_GETPINGQUEUECOUNT: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const UI_LAN_CLEARPING: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const UI_LAN_GETPING: crate::be_aas_h::C2RustUnnamed_0 = 48;

    pub const UI_LAN_GETPINGINFO: crate::be_aas_h::C2RustUnnamed_0 = 49;

    pub const UI_CVAR_REGISTER: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const UI_CVAR_UPDATE: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const UI_MEMORY_REMAINING: crate::be_aas_h::C2RustUnnamed_0 = 52;

    pub const UI_GET_CDKEY: crate::be_aas_h::C2RustUnnamed_0 = 53;

    pub const UI_SET_CDKEY: crate::be_aas_h::C2RustUnnamed_0 = 54;

    pub const UI_R_REGISTERFONT: crate::be_aas_h::C2RustUnnamed_0 = 55;

    pub const UI_R_MODELBOUNDS: crate::be_aas_h::C2RustUnnamed_0 = 56;

    pub const UI_PC_ADD_GLOBAL_DEFINE: crate::be_aas_h::C2RustUnnamed_0 = 57;

    pub const UI_PC_LOAD_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 58;

    pub const UI_PC_FREE_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 59;

    pub const UI_PC_READ_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 60;

    pub const UI_PC_SOURCE_FILE_AND_LINE: crate::be_aas_h::C2RustUnnamed_0 = 61;

    pub const UI_S_STOPBACKGROUNDTRACK: crate::be_aas_h::C2RustUnnamed_0 = 62;

    pub const UI_S_STARTBACKGROUNDTRACK: crate::be_aas_h::C2RustUnnamed_0 = 63;

    pub const UI_REAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const UI_LAN_GETSERVERCOUNT: crate::be_aas_h::C2RustUnnamed_0 = 65;

    pub const UI_LAN_GETSERVERADDRESSSTRING: crate::be_aas_h::C2RustUnnamed_0 = 66;

    pub const UI_LAN_GETSERVERINFO: crate::be_aas_h::C2RustUnnamed_0 = 67;

    pub const UI_LAN_MARKSERVERVISIBLE: crate::be_aas_h::C2RustUnnamed_0 = 68;

    pub const UI_LAN_UPDATEVISIBLEPINGS: crate::be_aas_h::C2RustUnnamed_0 = 69;

    pub const UI_LAN_RESETPINGS: crate::be_aas_h::C2RustUnnamed_0 = 70;

    pub const UI_LAN_LOADCACHEDSERVERS: crate::be_aas_h::C2RustUnnamed_0 = 71;

    pub const UI_LAN_SAVECACHEDSERVERS: crate::be_aas_h::C2RustUnnamed_0 = 72;

    pub const UI_LAN_ADDSERVER: crate::be_aas_h::C2RustUnnamed_0 = 73;

    pub const UI_LAN_REMOVESERVER: crate::be_aas_h::C2RustUnnamed_0 = 74;

    pub const UI_CIN_PLAYCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 75;

    pub const UI_CIN_STOPCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 76;

    pub const UI_CIN_RUNCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 77;

    pub const UI_CIN_DRAWCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 78;

    pub const UI_CIN_SETEXTENTS: crate::be_aas_h::C2RustUnnamed_0 = 79;

    pub const UI_R_REMAP_SHADER: crate::be_aas_h::C2RustUnnamed_0 = 80;

    pub const UI_VERIFY_CDKEY: crate::be_aas_h::C2RustUnnamed_0 = 81;

    pub const UI_LAN_SERVERSTATUS: crate::be_aas_h::C2RustUnnamed_0 = 82;

    pub const UI_LAN_GETSERVERPING: crate::be_aas_h::C2RustUnnamed_0 = 83;

    pub const UI_LAN_SERVERISVISIBLE: crate::be_aas_h::C2RustUnnamed_0 = 84;

    pub const UI_LAN_COMPARESERVERS: crate::be_aas_h::C2RustUnnamed_0 = 85;

    pub const UI_FS_SEEK: crate::be_aas_h::C2RustUnnamed_0 = 86;

    pub const UI_SET_PBCLSTATUS: crate::be_aas_h::C2RustUnnamed_0 = 87;

    pub const UI_MEMSET: crate::be_aas_h::C2RustUnnamed_0 = 100;

    pub const UI_MEMCPY: crate::be_aas_h::C2RustUnnamed_0 = 101;

    pub const UI_STRNCPY: crate::be_aas_h::C2RustUnnamed_0 = 102;

    pub const UI_SIN: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const UI_COS: crate::be_aas_h::C2RustUnnamed_0 = 104;

    pub const UI_ATAN2: crate::be_aas_h::C2RustUnnamed_0 = 105;

    pub const UI_SQRT: crate::be_aas_h::C2RustUnnamed_0 = 106;

    pub const UI_FLOOR: crate::be_aas_h::C2RustUnnamed_0 = 107;

    pub const UI_CEIL: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const UIMENU_NONE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const UIMENU_MAIN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const UIMENU_INGAME: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const UIMENU_NEED_CD: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const UIMENU_BAD_CD_KEY: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const UIMENU_TEAM: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const UIMENU_POSTGAME: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const UI_GETAPIVERSION: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const UI_INIT: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const UI_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const UI_KEY_EVENT: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const UI_MOUSE_EVENT: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const UI_REFRESH: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const UI_IS_FULLSCREEN: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const UI_SET_ACTIVE_MENU: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const UI_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const UI_DRAW_CONNECT_SCREEN: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const UI_HASUNIQUECDKEY: crate::be_aas_h::C2RustUnnamed_0 = 10;
}
pub mod cg_public_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct snapshot_t {
        pub snapFlags: i32,
        pub ping: i32,
        pub serverTime: i32,
        pub areamask: [crate::src::qcommon::q_shared::byte; 32],
        pub ps: crate::src::qcommon::q_shared::playerState_t,
        pub numEntities: i32,
        pub entities: [crate::src::qcommon::q_shared::entityState_t; 256],
        pub numServerCommands: i32,
        pub serverCommandSequence: i32,
    }

    pub const CGAME_EVENT_NONE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const CGAME_EVENT_TEAMMENU: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const CGAME_EVENT_SCOREBOARD: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const CGAME_EVENT_EDITHUD: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const CG_PRINT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const CG_ERROR: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const CG_MILLISECONDS: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const CG_CVAR_REGISTER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const CG_CVAR_UPDATE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const CG_CVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const CG_CVAR_VARIABLESTRINGBUFFER: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const CG_ARGC: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const CG_ARGV: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const CG_ARGS: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const CG_FS_FOPENFILE: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const CG_FS_READ: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const CG_FS_WRITE: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const CG_FS_FCLOSEFILE: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const CG_SENDCONSOLECOMMAND: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const CG_ADDCOMMAND: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const CG_SENDCLIENTCOMMAND: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const CG_UPDATESCREEN: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const CG_CM_LOADMAP: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const CG_CM_NUMINLINEMODELS: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const CG_CM_INLINEMODEL: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const CG_CM_LOADMODEL: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const CG_CM_TEMPBOXMODEL: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const CG_CM_POINTCONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const CG_CM_TRANSFORMEDPOINTCONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const CG_CM_BOXTRACE: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const CG_CM_TRANSFORMEDBOXTRACE: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const CG_CM_MARKFRAGMENTS: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const CG_S_STARTSOUND: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const CG_S_STARTLOCALSOUND: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const CG_S_CLEARLOOPINGSOUNDS: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const CG_S_ADDLOOPINGSOUND: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const CG_S_UPDATEENTITYPOSITION: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const CG_S_RESPATIALIZE: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const CG_S_REGISTERSOUND: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const CG_S_STARTBACKGROUNDTRACK: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const CG_R_LOADWORLDMAP: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const CG_R_REGISTERMODEL: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const CG_R_REGISTERSKIN: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const CG_R_REGISTERSHADER: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const CG_R_CLEARSCENE: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const CG_R_ADDREFENTITYTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const CG_R_ADDPOLYTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const CG_R_ADDLIGHTTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const CG_R_RENDERSCENE: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const CG_R_SETCOLOR: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const CG_R_DRAWSTRETCHPIC: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const CG_R_MODELBOUNDS: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const CG_R_LERPTAG: crate::be_aas_h::C2RustUnnamed_0 = 48;

    pub const CG_GETGLCONFIG: crate::be_aas_h::C2RustUnnamed_0 = 49;

    pub const CG_GETGAMESTATE: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const CG_GETCURRENTSNAPSHOTNUMBER: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const CG_GETSNAPSHOT: crate::be_aas_h::C2RustUnnamed_0 = 52;

    pub const CG_GETSERVERCOMMAND: crate::be_aas_h::C2RustUnnamed_0 = 53;

    pub const CG_GETCURRENTCMDNUMBER: crate::be_aas_h::C2RustUnnamed_0 = 54;

    pub const CG_GETUSERCMD: crate::be_aas_h::C2RustUnnamed_0 = 55;

    pub const CG_SETUSERCMDVALUE: crate::be_aas_h::C2RustUnnamed_0 = 56;

    pub const CG_R_REGISTERSHADERNOMIP: crate::be_aas_h::C2RustUnnamed_0 = 57;

    pub const CG_MEMORY_REMAINING: crate::be_aas_h::C2RustUnnamed_0 = 58;

    pub const CG_R_REGISTERFONT: crate::be_aas_h::C2RustUnnamed_0 = 59;

    pub const CG_KEY_ISDOWN: crate::be_aas_h::C2RustUnnamed_0 = 60;

    pub const CG_KEY_GETCATCHER: crate::be_aas_h::C2RustUnnamed_0 = 61;

    pub const CG_KEY_SETCATCHER: crate::be_aas_h::C2RustUnnamed_0 = 62;

    pub const CG_KEY_GETKEY: crate::be_aas_h::C2RustUnnamed_0 = 63;

    pub const CG_PC_ADD_GLOBAL_DEFINE: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const CG_PC_LOAD_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 65;

    pub const CG_PC_FREE_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 66;

    pub const CG_PC_READ_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 67;

    pub const CG_PC_SOURCE_FILE_AND_LINE: crate::be_aas_h::C2RustUnnamed_0 = 68;

    pub const CG_S_STOPBACKGROUNDTRACK: crate::be_aas_h::C2RustUnnamed_0 = 69;

    pub const CG_REAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 70;

    pub const CG_SNAPVECTOR: crate::be_aas_h::C2RustUnnamed_0 = 71;

    pub const CG_REMOVECOMMAND: crate::be_aas_h::C2RustUnnamed_0 = 72;

    pub const CG_R_LIGHTFORPOINT: crate::be_aas_h::C2RustUnnamed_0 = 73;

    pub const CG_CIN_PLAYCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 74;

    pub const CG_CIN_STOPCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 75;

    pub const CG_CIN_RUNCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 76;

    pub const CG_CIN_DRAWCINEMATIC: crate::be_aas_h::C2RustUnnamed_0 = 77;

    pub const CG_CIN_SETEXTENTS: crate::be_aas_h::C2RustUnnamed_0 = 78;

    pub const CG_R_REMAP_SHADER: crate::be_aas_h::C2RustUnnamed_0 = 79;

    pub const CG_S_ADDREALLOOPINGSOUND: crate::be_aas_h::C2RustUnnamed_0 = 80;

    pub const CG_S_STOPLOOPINGSOUND: crate::be_aas_h::C2RustUnnamed_0 = 81;

    pub const CG_CM_TEMPCAPSULEMODEL: crate::be_aas_h::C2RustUnnamed_0 = 82;

    pub const CG_CM_CAPSULETRACE: crate::be_aas_h::C2RustUnnamed_0 = 83;

    pub const CG_CM_TRANSFORMEDCAPSULETRACE: crate::be_aas_h::C2RustUnnamed_0 = 84;

    pub const CG_R_ADDADDITIVELIGHTTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 85;

    pub const CG_GET_ENTITY_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 86;

    pub const CG_R_ADDPOLYSTOSCENE: crate::be_aas_h::C2RustUnnamed_0 = 87;

    pub const CG_R_INPVS: crate::be_aas_h::C2RustUnnamed_0 = 88;

    pub const CG_FS_SEEK: crate::be_aas_h::C2RustUnnamed_0 = 89;

    pub const CG_MEMSET: crate::be_aas_h::C2RustUnnamed_0 = 100;

    pub const CG_MEMCPY: crate::be_aas_h::C2RustUnnamed_0 = 101;

    pub const CG_STRNCPY: crate::be_aas_h::C2RustUnnamed_0 = 102;

    pub const CG_SIN: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const CG_COS: crate::be_aas_h::C2RustUnnamed_0 = 104;

    pub const CG_ATAN2: crate::be_aas_h::C2RustUnnamed_0 = 105;

    pub const CG_SQRT: crate::be_aas_h::C2RustUnnamed_0 = 106;

    pub const CG_FLOOR: crate::be_aas_h::C2RustUnnamed_0 = 107;

    pub const CG_CEIL: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const CG_TESTPRINTINT: crate::be_aas_h::C2RustUnnamed_0 = 109;

    pub const CG_TESTPRINTFLOAT: crate::be_aas_h::C2RustUnnamed_0 = 110;

    pub const CG_ACOS: crate::be_aas_h::C2RustUnnamed_0 = 111;

    pub const CG_INIT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const CG_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const CG_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const CG_DRAW_ACTIVE_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const CG_CROSSHAIR_PLAYER: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const CG_LAST_ATTACKER: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const CG_KEY_EVENT: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const CG_MOUSE_EVENT: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const CG_EVENT_HANDLING: crate::be_aas_h::C2RustUnnamed_0 = 8;
}
pub mod tr_types_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct polyVert_t {
        pub xyz: crate::src::qcommon::q_shared::vec3_t,
        pub st: [f32; 2],
        pub modulate: [crate::src::qcommon::q_shared::byte; 4],
    }

    pub type refEntityType_t = u32;

    pub const RT_MODEL: crate::tr_types_h::refEntityType_t = 0;

    pub const RT_POLY: crate::tr_types_h::refEntityType_t = 1;

    pub const RT_SPRITE: crate::tr_types_h::refEntityType_t = 2;

    pub const RT_BEAM: crate::tr_types_h::refEntityType_t = 3;

    pub const RT_RAIL_CORE: crate::tr_types_h::refEntityType_t = 4;

    pub const RT_RAIL_RINGS: crate::tr_types_h::refEntityType_t = 5;

    pub const RT_LIGHTNING: crate::tr_types_h::refEntityType_t = 6;

    pub const RT_PORTALSURFACE: crate::tr_types_h::refEntityType_t = 7;

    pub const RT_MAX_REF_ENTITY_TYPE: crate::tr_types_h::refEntityType_t = 8;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refEntity_t {
        pub reType: crate::tr_types_h::refEntityType_t,
        pub renderfx: i32,
        pub hModel: crate::src::qcommon::q_shared::qhandle_t,
        pub lightingOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub shadowPlane: f32,
        pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
        pub nonNormalizedAxes: crate::src::qcommon::q_shared::qboolean,
        pub origin: [f32; 3],
        pub frame: i32,
        pub oldorigin: [f32; 3],
        pub oldframe: i32,
        pub backlerp: f32,
        pub skinNum: i32,
        pub customSkin: crate::src::qcommon::q_shared::qhandle_t,
        pub customShader: crate::src::qcommon::q_shared::qhandle_t,
        pub shaderRGBA: [crate::src::qcommon::q_shared::byte; 4],
        pub shaderTexCoord: [f32; 2],
        pub shaderTime: f32,
        pub radius: f32,
        pub rotation: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refdef_t {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
        pub fov_x: f32,
        pub fov_y: f32,
        pub vieworg: crate::src::qcommon::q_shared::vec3_t,
        pub viewaxis: [crate::src::qcommon::q_shared::vec3_t; 3],
        pub time: i32,
        pub rdflags: i32,
        pub areamask: [crate::src::qcommon::q_shared::byte; 32],
        pub text: [[libc::c_char; 32]; 8],
    }

    pub type stereoFrame_t = u32;

    pub const STEREO_CENTER: crate::tr_types_h::stereoFrame_t = 0;

    pub const STEREO_LEFT: crate::tr_types_h::stereoFrame_t = 1;

    pub const STEREO_RIGHT: crate::tr_types_h::stereoFrame_t = 2;

    pub type textureCompression_t = u32;

    pub const TC_NONE: crate::tr_types_h::textureCompression_t = 0;

    pub const TC_S3TC: crate::tr_types_h::textureCompression_t = 1;

    pub const TC_S3TC_ARB: crate::tr_types_h::textureCompression_t = 2;

    pub type glDriverType_t = u32;

    pub const GLDRV_ICD: crate::tr_types_h::glDriverType_t = 0;

    pub const GLDRV_STANDALONE: crate::tr_types_h::glDriverType_t = 1;

    pub const GLDRV_VOODOO: crate::tr_types_h::glDriverType_t = 2;

    pub type glHardwareType_t = u32;

    pub const GLHW_GENERIC: crate::tr_types_h::glHardwareType_t = 0;

    pub const GLHW_3DFX_2D3D: crate::tr_types_h::glHardwareType_t = 1;

    pub const GLHW_RIVA128: crate::tr_types_h::glHardwareType_t = 2;

    pub const GLHW_RAGEPRO: crate::tr_types_h::glHardwareType_t = 3;

    pub const GLHW_PERMEDIA2: crate::tr_types_h::glHardwareType_t = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: i32,
        pub numTextureUnits: i32,
        pub colorBits: i32,
        pub depthBits: i32,
        pub stencilBits: i32,
        pub driverType: crate::tr_types_h::glDriverType_t,
        pub hardwareType: crate::tr_types_h::glHardwareType_t,
        pub deviceSupportsGamma: crate::src::qcommon::q_shared::qboolean,
        pub textureCompression: crate::tr_types_h::textureCompression_t,
        pub textureEnvAddAvailable: crate::src::qcommon::q_shared::qboolean,
        pub vidWidth: i32,
        pub vidHeight: i32,
        pub windowAspect: f32,
        pub displayFrequency: i32,
        pub isFullscreen: crate::src::qcommon::q_shared::qboolean,
        pub stereoEnabled: crate::src::qcommon::q_shared::qboolean,
        pub smpActive: crate::src::qcommon::q_shared::qboolean,
    }
}
pub mod qcommon_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct msg_t {
        pub allowoverflow: crate::src::qcommon::q_shared::qboolean,
        pub overflowed: crate::src::qcommon::q_shared::qboolean,
        pub oob: crate::src::qcommon::q_shared::qboolean,
        pub data: *mut crate::src::qcommon::q_shared::byte,
        pub maxsize: i32,
        pub cursize: i32,
        pub readcount: i32,
        pub bit: i32,
    }

    pub type netadrtype_t = u32;

    pub const NA_BAD: crate::qcommon_h::netadrtype_t = 0;

    pub const NA_BOT: crate::qcommon_h::netadrtype_t = 1;

    pub const NA_LOOPBACK: crate::qcommon_h::netadrtype_t = 2;

    pub const NA_BROADCAST: crate::qcommon_h::netadrtype_t = 3;

    pub const NA_IP: crate::qcommon_h::netadrtype_t = 4;

    pub const NA_IP6: crate::qcommon_h::netadrtype_t = 5;

    pub const NA_MULTICAST6: crate::qcommon_h::netadrtype_t = 6;

    pub const NA_UNSPEC: crate::qcommon_h::netadrtype_t = 7;

    pub type netsrc_t = u32;

    pub const NS_CLIENT: crate::qcommon_h::netsrc_t = 0;

    pub const NS_SERVER: crate::qcommon_h::netsrc_t = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netadr_t {
        pub type_0: crate::qcommon_h::netadrtype_t,
        pub ip: [crate::src::qcommon::q_shared::byte; 4],
        pub ip6: [crate::src::qcommon::q_shared::byte; 16],
        pub port: u16,
        pub scope_id: usize,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netchan_t {
        pub sock: crate::qcommon_h::netsrc_t,
        pub dropped: i32,
        pub remoteAddress: crate::qcommon_h::netadr_t,
        pub qport: i32,
        pub incomingSequence: i32,
        pub outgoingSequence: i32,
        pub fragmentSequence: i32,
        pub fragmentLength: i32,
        pub fragmentBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub unsentFragments: crate::src::qcommon::q_shared::qboolean,
        pub unsentFragmentStart: i32,
        pub unsentLength: i32,
        pub unsentBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub challenge: i32,
        pub lastSentTime: i32,
        pub lastSentSize: i32,
        pub compat: crate::src::qcommon::q_shared::qboolean,
    }

    pub type svc_ops_e = u32;

    pub const svc_bad: crate::qcommon_h::svc_ops_e = 0;

    pub const svc_nop: crate::qcommon_h::svc_ops_e = 1;

    pub const svc_gamestate: crate::qcommon_h::svc_ops_e = 2;

    pub const svc_configstring: crate::qcommon_h::svc_ops_e = 3;

    pub const svc_baseline: crate::qcommon_h::svc_ops_e = 4;

    pub const svc_serverCommand: crate::qcommon_h::svc_ops_e = 5;

    pub const svc_download: crate::qcommon_h::svc_ops_e = 6;

    pub const svc_snapshot: crate::qcommon_h::svc_ops_e = 7;

    pub const svc_EOF: crate::qcommon_h::svc_ops_e = 8;

    pub const svc_voipSpeex: crate::qcommon_h::svc_ops_e = 9;

    pub const svc_voipOpus: crate::qcommon_h::svc_ops_e = 10;

    pub type clc_ops_e = u32;

    pub const clc_bad: crate::qcommon_h::clc_ops_e = 0;

    pub const clc_nop: crate::qcommon_h::clc_ops_e = 1;

    pub const clc_move: crate::qcommon_h::clc_ops_e = 2;

    pub const clc_moveNoDelta: crate::qcommon_h::clc_ops_e = 3;

    pub const clc_clientCommand: crate::qcommon_h::clc_ops_e = 4;

    pub const clc_EOF: crate::qcommon_h::clc_ops_e = 5;

    pub const clc_voipSpeex: crate::qcommon_h::clc_ops_e = 6;

    pub const clc_voipOpus: crate::qcommon_h::clc_ops_e = 7;

    pub type vm_t = crate::vm_local_h::vm_s;

    pub type vmInterpret_t = u32;

    pub const VMI_NATIVE: crate::qcommon_h::vmInterpret_t = 0;

    pub const VMI_BYTECODE: crate::qcommon_h::vmInterpret_t = 1;

    pub const VMI_COMPILED: crate::qcommon_h::vmInterpret_t = 2;

    pub const TRAP_MEMSET: crate::be_aas_h::C2RustUnnamed_0 = 100;

    pub const TRAP_MEMCPY: crate::be_aas_h::C2RustUnnamed_0 = 101;

    pub const TRAP_STRNCPY: crate::be_aas_h::C2RustUnnamed_0 = 102;

    pub const TRAP_SIN: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const TRAP_COS: crate::be_aas_h::C2RustUnnamed_0 = 104;

    pub const TRAP_ATAN2: crate::be_aas_h::C2RustUnnamed_0 = 105;

    pub const TRAP_SQRT: crate::be_aas_h::C2RustUnnamed_0 = 106;

    pub const TRAP_MATRIXMULTIPLY: crate::be_aas_h::C2RustUnnamed_0 = 107;

    pub const TRAP_ANGLEVECTORS: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const TRAP_PERPENDICULARVECTOR: crate::be_aas_h::C2RustUnnamed_0 = 109;

    pub const TRAP_FLOOR: crate::be_aas_h::C2RustUnnamed_0 = 110;

    pub const TRAP_CEIL: crate::be_aas_h::C2RustUnnamed_0 = 111;

    pub const TRAP_TESTPRINTINT: crate::be_aas_h::C2RustUnnamed_0 = 112;

    pub const TRAP_TESTPRINTFLOAT: crate::be_aas_h::C2RustUnnamed_0 = 113;

    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;

    pub type completionFunc_t = Option<unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()>;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct field_t {
        pub cursor: i32,
        pub scroll: i32,
        pub widthInChars: i32,
        pub buffer: [libc::c_char; 256],
    }

    pub type cpuFeatures_t = u32;

    pub const CF_RDTSC: crate::qcommon_h::cpuFeatures_t = 1;

    pub const CF_MMX: crate::qcommon_h::cpuFeatures_t = 2;

    pub const CF_MMX_EXT: crate::qcommon_h::cpuFeatures_t = 4;

    pub const CF_3DNOW: crate::qcommon_h::cpuFeatures_t = 8;

    pub const CF_3DNOW_EXT: crate::qcommon_h::cpuFeatures_t = 16;

    pub const CF_SSE: crate::qcommon_h::cpuFeatures_t = 32;

    pub const CF_SSE2: crate::qcommon_h::cpuFeatures_t = 64;

    pub const CF_ALTIVEC: crate::qcommon_h::cpuFeatures_t = 128;

    pub type sysEventType_t = u32;

    pub const SE_NONE: crate::qcommon_h::sysEventType_t = 0;

    pub const SE_KEY: crate::qcommon_h::sysEventType_t = 1;

    pub const SE_CHAR: crate::qcommon_h::sysEventType_t = 2;

    pub const SE_MOUSE: crate::qcommon_h::sysEventType_t = 3;

    pub const SE_JOYSTICK_AXIS: crate::qcommon_h::sysEventType_t = 4;

    pub const SE_CONSOLE: crate::qcommon_h::sysEventType_t = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sysEvent_t {
        pub evTime: i32,
        pub evType: crate::qcommon_h::sysEventType_t,
        pub evValue: i32,
        pub evValue2: i32,
        pub evPtrLength: i32,
        pub evPtr: *mut libc::c_void,
    }

    pub const TAG_FREE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const TAG_GENERAL: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const TAG_BOTLIB: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const TAG_RENDERER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const TAG_SMALL: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const TAG_STATIC: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub type dialogResult_t = u32;

    pub const DR_YES: crate::qcommon_h::dialogResult_t = 0;

    pub const DR_NO: crate::qcommon_h::dialogResult_t = 1;

    pub const DR_OK: crate::qcommon_h::dialogResult_t = 0;

    pub const DR_CANCEL: crate::qcommon_h::dialogResult_t = 1;

    pub type dialogType_t = u32;

    pub const DT_INFO: crate::qcommon_h::dialogType_t = 0;

    pub const DT_WARNING: crate::qcommon_h::dialogType_t = 1;

    pub const DT_ERROR: crate::qcommon_h::dialogType_t = 2;

    pub const DT_YES_NO: crate::qcommon_h::dialogType_t = 3;

    pub const DT_OK_CANCEL: crate::qcommon_h::dialogType_t = 4;

    pub type node_t = crate::qcommon_h::nodetype;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct nodetype {
        pub left: *mut crate::qcommon_h::nodetype,
        pub right: *mut crate::qcommon_h::nodetype,
        pub parent: *mut crate::qcommon_h::nodetype,
        pub next: *mut crate::qcommon_h::nodetype,
        pub prev: *mut crate::qcommon_h::nodetype,
        pub head: *mut *mut crate::qcommon_h::nodetype,
        pub weight: i32,
        pub symbol: i32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct huff_t {
        pub blocNode: i32,
        pub blocPtrs: i32,
        pub tree: *mut crate::qcommon_h::node_t,
        pub lhead: *mut crate::qcommon_h::node_t,
        pub ltail: *mut crate::qcommon_h::node_t,
        pub loc: [*mut crate::qcommon_h::node_t; 257],
        pub freelist: *mut *mut crate::qcommon_h::node_t,
        pub nodeList: [crate::qcommon_h::node_t; 768],
        pub nodePtrs: [*mut crate::qcommon_h::node_t; 768],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct huffman_t {
        pub compressor: crate::qcommon_h::huff_t,
        pub decompressor: crate::qcommon_h::huff_t,
    }
}
pub mod be_aas_h {
    pub type C2RustUnnamed_0 = u32;

    pub const SOLID_NOT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const SOLID_TRIGGER: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const SOLID_BBOX: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const SOLID_BSP: crate::be_aas_h::C2RustUnnamed_0 = 3;
    //a trace is returned when a box is swept through the AAS world

    pub type aas_trace_t = crate::be_aas_h::aas_trace_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_trace_s {
        pub startsolid: crate::src::qcommon::q_shared::qboolean,
        pub fraction: f32,
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub ent: i32,
        pub lastarea: i32,
        pub area: i32,
        pub planenum: i32,
    }
    /* Defined in botlib.h

    //bsp_trace_t hit surface
    typedef struct bsp_surface_s
    {
        char name[16];
        int flags;
        int value;
    } bsp_surface_t;

    //a trace is returned when a box is swept through the BSP world
    typedef struct bsp_trace_s
    {
        qboolean		allsolid;	// if true, plane is not valid
        qboolean		startsolid;	// if true, the initial point was in a solid area
        float			fraction;	// time completed, 1.0 = didn't hit anything
        vec3_t			endpos;		// final position
        cplane_t		plane;		// surface normal at impact
        float			exp_dist;	// expanded plane distance
        int				sidenum;	// number of the brush side hit
        bsp_surface_t	surface;	// hit surface
        int				contents;	// contents on other side of surface hit
        int				ent;		// number of entity hit
    } bsp_trace_t;
    //
    */
    //entity info

    pub type aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_entityinfo_s {
        pub valid: i32,
        pub type_0: i32,
        pub flags: i32,
        pub ltime: f32,
        pub update_time: f32,
        pub number: i32,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub old_origin: crate::src::qcommon::q_shared::vec3_t,
        pub lastvisorigin: crate::src::qcommon::q_shared::vec3_t,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub groundent: i32,
        pub solid: i32,
        pub modelindex: i32,
        pub modelindex2: i32,
        pub frame: i32,
        pub event: i32,
        pub eventParm: i32,
        pub powerups: i32,
        pub weapon: i32,
        pub legsAnim: i32,
        pub torsoAnim: i32,
    }

    pub type aas_areainfo_t = crate::be_aas_h::aas_areainfo_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_areainfo_s {
        pub contents: i32,
        pub flags: i32,
        pub presencetype: i32,
        pub cluster: i32,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub center: crate::src::qcommon::q_shared::vec3_t,
    }
    // true if updated this frame
    // entity type
    // entity flags
    // local time
    // time between last and current update
    // number of the entity
    // origin of the entity
    // angles of the model
    // for lerping
    // last visible origin
    // bounding box minimums
    // bounding box maximums
    // ground entity
    // solid type
    // model used
    // weapons, CTF flags, etc
    // model frame number
    // impulse events -- muzzle flashes, footsteps, etc
    // even parameter
    // bit flags
    // determines weapon and flash model, etc
    // mask off ANIM_TOGGLEBIT
    // mask off ANIM_TOGGLEBIT
    // a ground face in the area is hit
    // hit the specified bounding box
    // touching a cluster portal

    pub type aas_clientmove_t = crate::be_aas_h::aas_clientmove_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_clientmove_s {
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub endarea: i32,
        pub velocity: crate::src::qcommon::q_shared::vec3_t,
        pub trace: crate::be_aas_h::aas_trace_t,
        pub presencetype: i32,
        pub stopevent: i32,
        pub endcontents: i32,
        pub time: f32,
        pub frames: i32,
    }

    pub type aas_altroutegoal_t = crate::be_aas_h::aas_altroutegoal_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_altroutegoal_s {
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub areanum: i32,
        pub starttraveltime: u16,
        pub goaltraveltime: u16,
        pub extratraveltime: u16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_predictroute_s {
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub endarea: i32,
        pub stopevent: i32,
        pub endcontents: i32,
        pub endtravelflags: i32,
        pub numareas: i32,
        pub time: i32,
    }
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn SDL_GetCurrentAudioDriver() -> *const libc::c_char;

        #[no_mangle]
        pub fn SDL_OpenAudioDevice(
            device: *const libc::c_char,
            iscapture: i32,
            desired: *const crate::stdlib::SDL_AudioSpec,
            obtained: *mut crate::stdlib::SDL_AudioSpec,
            allowed_changes: i32,
        ) -> crate::stdlib::SDL_AudioDeviceID;

        #[no_mangle]
        pub fn SDL_PauseAudioDevice(dev: crate::stdlib::SDL_AudioDeviceID, pause_on: i32);

        #[no_mangle]
        pub fn SDL_DequeueAudio(
            dev: crate::stdlib::SDL_AudioDeviceID,
            data: *mut libc::c_void,
            len: crate::stdlib::Uint32,
        ) -> crate::stdlib::Uint32;

        #[no_mangle]
        pub fn SDL_GetQueuedAudioSize(
            dev: crate::stdlib::SDL_AudioDeviceID,
        ) -> crate::stdlib::Uint32;

        #[no_mangle]
        pub fn SDL_ClearQueuedAudio(dev: crate::stdlib::SDL_AudioDeviceID);

        #[no_mangle]
        pub fn SDL_LockAudioDevice(dev: crate::stdlib::SDL_AudioDeviceID);

        #[no_mangle]
        pub fn SDL_UnlockAudioDevice(dev: crate::stdlib::SDL_AudioDeviceID);

        #[no_mangle]
        pub fn SDL_CloseAudioDevice(dev: crate::stdlib::SDL_AudioDeviceID);
        #[no_mangle]
        pub fn SDL_GetClipboardText() -> *mut libc::c_char;
        #[no_mangle]
        pub fn SDL_HasRDTSC() -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_HasAltiVec() -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_HasMMX() -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_Has3DNow() -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_HasSSE() -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_HasSSE2() -> crate::stdlib::SDL_bool;
        #[no_mangle]
        pub fn SDL_GetError() -> *const libc::c_char;
        pub type SDL_SysWMmsg;

        #[no_mangle]
        pub fn SDL_PumpEvents();

        #[no_mangle]
        pub fn SDL_PeepEvents(
            events: *mut crate::stdlib::SDL_Event,
            numevents: i32,
            action: crate::stdlib::SDL_eventaction,
            minType: crate::stdlib::Uint32,
            maxType: crate::stdlib::Uint32,
        ) -> i32;

        #[no_mangle]
        pub fn SDL_PollEvent(event: *mut crate::stdlib::SDL_Event) -> i32;
        pub type _SDL_GameController;

        #[no_mangle]
        pub fn SDL_IsGameController(joystick_index: i32) -> crate::stdlib::SDL_bool;

        #[no_mangle]
        pub fn SDL_GameControllerOpen(
            joystick_index: i32,
        ) -> *mut crate::stdlib::SDL_GameController;

        #[no_mangle]
        pub fn SDL_GameControllerEventState(state: i32) -> i32;

        #[no_mangle]
        pub fn SDL_GameControllerUpdate();

        #[no_mangle]
        pub fn SDL_GameControllerGetAxis(
            gamecontroller: *mut crate::stdlib::SDL_GameController,
            axis: crate::stdlib::SDL_GameControllerAxis,
        ) -> crate::stdlib::Sint16;

        #[no_mangle]
        pub fn SDL_GameControllerGetButton(
            gamecontroller: *mut crate::stdlib::SDL_GameController,
            button: crate::stdlib::SDL_GameControllerButton,
        ) -> crate::stdlib::Uint8;

        #[no_mangle]
        pub fn SDL_GameControllerClose(gamecontroller: *mut crate::stdlib::SDL_GameController);
        #[no_mangle]
        pub fn SDL_Init(flags: crate::stdlib::Uint32) -> i32;

        #[no_mangle]
        pub fn SDL_QuitSubSystem(flags: crate::stdlib::Uint32);

        #[no_mangle]
        pub fn SDL_WasInit(flags: crate::stdlib::Uint32) -> crate::stdlib::Uint32;

        #[no_mangle]
        pub fn SDL_Quit();
        pub type _SDL_Joystick;

        #[no_mangle]
        pub fn SDL_NumJoysticks() -> i32;

        #[no_mangle]
        pub fn SDL_JoystickNameForIndex(device_index: i32) -> *const libc::c_char;

        #[no_mangle]
        pub fn SDL_JoystickOpen(device_index: i32) -> *mut crate::stdlib::SDL_Joystick;

        #[no_mangle]
        pub fn SDL_JoystickNumAxes(joystick: *mut crate::stdlib::SDL_Joystick) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickNumBalls(joystick: *mut crate::stdlib::SDL_Joystick) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickNumHats(joystick: *mut crate::stdlib::SDL_Joystick) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickNumButtons(joystick: *mut crate::stdlib::SDL_Joystick) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickUpdate();

        #[no_mangle]
        pub fn SDL_JoystickEventState(state: i32) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickGetAxis(
            joystick: *mut crate::stdlib::SDL_Joystick,
            axis: i32,
        ) -> crate::stdlib::Sint16;

        #[no_mangle]
        pub fn SDL_JoystickGetHat(
            joystick: *mut crate::stdlib::SDL_Joystick,
            hat: i32,
        ) -> crate::stdlib::Uint8;

        #[no_mangle]
        pub fn SDL_JoystickGetBall(
            joystick: *mut crate::stdlib::SDL_Joystick,
            ball: i32,
            dx: *mut i32,
            dy: *mut i32,
        ) -> i32;

        #[no_mangle]
        pub fn SDL_JoystickGetButton(
            joystick: *mut crate::stdlib::SDL_Joystick,
            button: i32,
        ) -> crate::stdlib::Uint8;

        #[no_mangle]
        pub fn SDL_JoystickClose(joystick: *mut crate::stdlib::SDL_Joystick);
        #[no_mangle]
        pub fn SDL_GetScancodeName(scancode: crate::stdlib::SDL_Scancode) -> *const libc::c_char;

        #[no_mangle]
        pub fn SDL_GetKeyName(key: crate::stdlib::SDL_Keycode) -> *const libc::c_char;

        #[no_mangle]
        pub fn SDL_StartTextInput();

        #[no_mangle]
        pub fn SDL_StopTextInput();
        #[no_mangle]
        pub fn SDL_LoadObject(sofile: *const libc::c_char) -> *mut libc::c_void;

        #[no_mangle]
        pub fn SDL_LoadFunction(
            handle: *mut libc::c_void,
            name: *const libc::c_char,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn SDL_UnloadObject(handle: *mut libc::c_void);
        #[no_mangle]
        pub fn SDL_WarpMouseInWindow(window: *mut crate::stdlib::SDL_Window, x: i32, y: i32);

        #[no_mangle]
        pub fn SDL_SetRelativeMouseMode(enabled: crate::stdlib::SDL_bool) -> i32;

        #[no_mangle]
        pub fn SDL_ShowCursor(toggle: i32) -> i32;
        #[no_mangle]
        pub fn SDL_free(mem: *mut libc::c_void);

        #[no_mangle]
        pub fn SDL_memset(
            dst: *mut libc::c_void,
            c: i32,
            len: crate::stddef_h::size_t,
        ) -> *mut libc::c_void;
        #[no_mangle]
        pub fn SDL_GetVersion(ver: *mut crate::stdlib::SDL_version);
        pub type SDL_Window;

        #[no_mangle]
        pub fn SDL_GetWindowFlags(window: *mut crate::stdlib::SDL_Window) -> crate::stdlib::Uint32;

        #[no_mangle]
        pub fn SDL_SetWindowGrab(
            window: *mut crate::stdlib::SDL_Window,
            grabbed: crate::stdlib::SDL_bool,
        );
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const u16;

        #[no_mangle]
        pub fn __ctype_tolower_loc() -> *mut *const crate::stdlib::__int32_t;

        #[no_mangle]
        pub fn __ctype_toupper_loc() -> *mut *const crate::stdlib::__int32_t;
        #[no_mangle]
        pub fn fesetround(__rounding_direction: i32) -> i32;
        #[no_mangle]
        pub fn getifaddrs(__ifap: *mut *mut crate::stdlib::ifaddrs) -> i32;

        #[no_mangle]
        pub fn freeifaddrs(__ifa: *mut crate::stdlib::ifaddrs);
        #[no_mangle]
        pub static in6addr_any: crate::stdlib::in6_addr;
        pub type __dirstream;

        #[no_mangle]
        pub fn opendir(__name: *const libc::c_char) -> *mut crate::stdlib::DIR;

        #[no_mangle]
        pub fn closedir(__dirp: *mut crate::stdlib::DIR) -> i32;

        #[no_mangle]
        pub fn readdir(__dirp: *mut crate::stdlib::DIR) -> *mut ::libc::dirent;
        #[no_mangle]
        pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> i32;

        #[no_mangle]
        pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: i32) -> !;
        #[no_mangle]
        pub fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn acos(_: f64) -> f64;

        #[no_mangle]
        pub fn atan(_: f64) -> f64;

        #[no_mangle]
        pub fn atan2(_: f64, _: f64) -> f64;

        #[no_mangle]
        pub fn cos(_: f64) -> f64;

        #[no_mangle]
        pub fn sin(_: f64) -> f64;

        #[no_mangle]
        pub fn tan(_: f64) -> f64;

        #[no_mangle]
        pub fn exp(_: f64) -> f64;

        #[no_mangle]
        pub fn ldexp(_: f64, _: i32) -> f64;

        #[no_mangle]
        pub fn log(_: f64) -> f64;

        #[no_mangle]
        pub fn log10(_: f64) -> f64;

        #[no_mangle]
        pub fn powf(_: f32, _: f32) -> f32;

        #[no_mangle]
        pub fn pow(_: f64, _: f64) -> f64;

        #[no_mangle]
        pub fn sqrt(_: f64) -> f64;

        #[no_mangle]
        pub fn ceil(_: f64) -> f64;

        #[no_mangle]
        pub fn fabsf(_: f32) -> f32;

        #[no_mangle]
        pub fn fabs(_: f64) -> f64;

        #[no_mangle]
        pub fn floor(_: f64) -> f64;

        #[no_mangle]
        pub fn fmodf(_: f32, _: f32) -> f32;

        #[no_mangle]
        pub fn rint(_: f64) -> f64;
        #[no_mangle]
        pub fn mmap(
            __addr: *mut libc::c_void,
            __len: crate::stddef_h::size_t,
            __prot: i32,
            __flags: i32,
            __fd: i32,
            __offset: crate::stdlib::__off_t,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn munmap(__addr: *mut libc::c_void, __len: crate::stddef_h::size_t) -> i32;

        #[no_mangle]
        pub fn mprotect(
            __addr: *mut libc::c_void,
            __len: crate::stddef_h::size_t,
            __prot: i32,
        ) -> i32;
        #[no_mangle]
        pub fn gethostbyname(__name: *const libc::c_char) -> *mut ::libc::hostent;
        #[no_mangle]
        pub fn select(
            __nfds: i32,
            __readfds: *mut crate::stdlib::fd_set,
            __writefds: *mut crate::stdlib::fd_set,
            __exceptfds: *mut crate::stdlib::fd_set,
            __timeout: *mut ::libc::timeval,
        ) -> i32;
        #[no_mangle]
        pub fn signal(
            __sig: i32,
            __handler: crate::stdlib::__sighandler_t,
        ) -> crate::stdlib::__sighandler_t;
        #[no_mangle]
        pub static mut stdout: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> i32;

        #[no_mangle]
        pub fn fflush(__stream: *mut crate::stdlib::FILE) -> i32;

        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn freopen(
            __filename: *const libc::c_char,
            __modes: *const libc::c_char,
            __stream: *mut crate::stdlib::FILE,
        ) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fdopen(__fd: i32, __modes: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn setvbuf(
            __stream: *mut crate::stdlib::FILE,
            __buf: *mut libc::c_char,
            __modes: i32,
            __n: crate::stddef_h::size_t,
        ) -> i32;

        #[no_mangle]
        pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> i32;

        #[no_mangle]
        pub fn vfprintf(
            _: *mut crate::stdlib::FILE,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> i32;

        #[no_mangle]
        pub fn snprintf(_: *mut libc::c_char, _: usize, _: *const libc::c_char, _: ...) -> i32;

        #[no_mangle]
        pub fn vsnprintf(
            _: *mut libc::c_char,
            _: usize,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> i32;

        #[no_mangle]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut crate::stdlib::FILE) -> i32;

        #[no_mangle]
        pub fn fread(
            _: *mut libc::c_void,
            _: usize,
            _: usize,
            _: *mut crate::stdlib::FILE,
        ) -> usize;

        #[no_mangle]
        pub fn fwrite(
            _: *const libc::c_void,
            _: usize,
            _: usize,
            _: *mut crate::stdlib::FILE,
        ) -> usize;

        #[no_mangle]
        pub fn fseek(__stream: *mut crate::stdlib::FILE, __off: isize, __whence: i32) -> i32;

        #[no_mangle]
        pub fn ftell(__stream: *mut crate::stdlib::FILE) -> isize;

        #[no_mangle]
        pub fn fseeko(
            __stream: *mut crate::stdlib::FILE,
            __off: crate::stdlib::__off64_t,
            __whence: i32,
        ) -> i32;

        #[no_mangle]
        pub fn ftello(__stream: *mut crate::stdlib::FILE) -> crate::stdlib::__off64_t;

        #[no_mangle]
        pub fn feof(__stream: *mut crate::stdlib::FILE) -> i32;

        #[no_mangle]
        pub fn ferror(__stream: *mut crate::stdlib::FILE) -> i32;

        #[no_mangle]
        pub fn fileno(__stream: *mut crate::stdlib::FILE) -> i32;
        #[no_mangle]
        pub fn malloc(_: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn calloc(_: usize, _: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn realloc(_: *mut libc::c_void, _: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nmemb: crate::stddef_h::size_t,
            __size: crate::stddef_h::size_t,
            __compar: crate::stdlib::__compar_fn_t,
        );

        #[no_mangle]
        pub fn mbstowcs(
            __pwcs: *mut crate::stddef_h::wchar_t,
            __s: *const libc::c_char,
            __n: crate::stddef_h::size_t,
        ) -> crate::stddef_h::size_t;
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: usize)
            -> *mut libc::c_void;

        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: i32, _: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: usize) -> i32;

        #[no_mangle]
        pub fn memchr(_: *const libc::c_void, _: i32, _: usize) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: usize)
            -> *mut libc::c_char;

        #[no_mangle]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: usize)
            -> *mut libc::c_char;

        #[no_mangle]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: usize) -> i32;

        #[no_mangle]
        pub fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> usize;

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> usize;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
        #[no_mangle]
        pub fn send(
            __fd: i32,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: i32,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn recv(
            __fd: i32,
            __buf: *mut libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: i32,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn sendto(
            __fd: i32,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: i32,
            __addr: *const ::libc::sockaddr,
            __addr_len: crate::stdlib::socklen_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn recvfrom(
            __fd: i32,
            __buf: *mut libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: i32,
            __addr: *mut ::libc::sockaddr,
            __addr_len: *mut crate::stdlib::socklen_t,
        ) -> crate::stdlib::ssize_t;
        #[no_mangle]
        pub fn __xstat(
            __ver: i32,
            __filename: *const libc::c_char,
            __stat_buf: *mut crate::stdlib::stat,
        ) -> i32;
        #[no_mangle]
        pub fn gettimeofday(
            __tv: *mut ::libc::timeval,
            __tz: crate::stdlib::__timezone_ptr_t,
        ) -> i32;

        #[no_mangle]
        pub fn clock() -> crate::stdlib::clock_t;

        #[no_mangle]
        pub fn asctime(__tp: *const ::libc::tm) -> *mut libc::c_char;

        #[no_mangle]
        pub fn ctime(__timer: *const crate::stdlib::time_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn read(
            __fd: i32,
            __buf: *mut libc::c_void,
            __nbytes: crate::stddef_h::size_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn write(
            __fd: i32,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn getcwd(
            __buf: *mut libc::c_char,
            __size: crate::stddef_h::size_t,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char) -> i32;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub type SDL_AudioFormat = crate::stdlib::Uint16;

    pub type SDL_AudioCallback = Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut crate::stdlib::Uint8, _: i32) -> (),
    >;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_AudioSpec {
        pub freq: i32,
        pub format: crate::stdlib::SDL_AudioFormat,
        pub channels: crate::stdlib::Uint8,
        pub silence: crate::stdlib::Uint8,
        pub samples: crate::stdlib::Uint16,
        pub padding: crate::stdlib::Uint16,
        pub size: crate::stdlib::Uint32,
        pub callback: crate::stdlib::SDL_AudioCallback,
        pub userdata: *mut libc::c_void,
    }

    pub type SDL_AudioDeviceID = crate::stdlib::Uint32;
    pub const SDL_FIRSTEVENT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const SDL_QUIT: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const SDL_APP_TERMINATING: crate::be_aas_h::C2RustUnnamed_0 = 257;

    pub const SDL_APP_LOWMEMORY: crate::be_aas_h::C2RustUnnamed_0 = 258;

    pub const SDL_APP_WILLENTERBACKGROUND: crate::be_aas_h::C2RustUnnamed_0 = 259;

    pub const SDL_APP_DIDENTERBACKGROUND: crate::be_aas_h::C2RustUnnamed_0 = 260;

    pub const SDL_APP_WILLENTERFOREGROUND: crate::be_aas_h::C2RustUnnamed_0 = 261;

    pub const SDL_APP_DIDENTERFOREGROUND: crate::be_aas_h::C2RustUnnamed_0 = 262;

    pub const SDL_DISPLAYEVENT: crate::be_aas_h::C2RustUnnamed_0 = 336;

    pub const SDL_WINDOWEVENT: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const SDL_SYSWMEVENT: crate::be_aas_h::C2RustUnnamed_0 = 513;

    pub const SDL_KEYDOWN: crate::be_aas_h::C2RustUnnamed_0 = 768;

    pub const SDL_KEYUP: crate::be_aas_h::C2RustUnnamed_0 = 769;

    pub const SDL_TEXTEDITING: crate::be_aas_h::C2RustUnnamed_0 = 770;

    pub const SDL_TEXTINPUT: crate::be_aas_h::C2RustUnnamed_0 = 771;

    pub const SDL_KEYMAPCHANGED: crate::be_aas_h::C2RustUnnamed_0 = 772;

    pub const SDL_MOUSEMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const SDL_MOUSEBUTTONDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1025;

    pub const SDL_MOUSEBUTTONUP: crate::be_aas_h::C2RustUnnamed_0 = 1026;

    pub const SDL_MOUSEWHEEL: crate::be_aas_h::C2RustUnnamed_0 = 1027;

    pub const SDL_JOYAXISMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1536;

    pub const SDL_JOYBALLMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1537;

    pub const SDL_JOYHATMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1538;

    pub const SDL_JOYBUTTONDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1539;

    pub const SDL_JOYBUTTONUP: crate::be_aas_h::C2RustUnnamed_0 = 1540;

    pub const SDL_JOYDEVICEADDED: crate::be_aas_h::C2RustUnnamed_0 = 1541;

    pub const SDL_JOYDEVICEREMOVED: crate::be_aas_h::C2RustUnnamed_0 = 1542;

    pub const SDL_CONTROLLERAXISMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1616;

    pub const SDL_CONTROLLERBUTTONDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1617;

    pub const SDL_CONTROLLERBUTTONUP: crate::be_aas_h::C2RustUnnamed_0 = 1618;

    pub const SDL_CONTROLLERDEVICEADDED: crate::be_aas_h::C2RustUnnamed_0 = 1619;

    pub const SDL_CONTROLLERDEVICEREMOVED: crate::be_aas_h::C2RustUnnamed_0 = 1620;

    pub const SDL_CONTROLLERDEVICEREMAPPED: crate::be_aas_h::C2RustUnnamed_0 = 1621;

    pub const SDL_FINGERDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1792;

    pub const SDL_FINGERUP: crate::be_aas_h::C2RustUnnamed_0 = 1793;

    pub const SDL_FINGERMOTION: crate::be_aas_h::C2RustUnnamed_0 = 1794;

    pub const SDL_DOLLARGESTURE: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const SDL_DOLLARRECORD: crate::be_aas_h::C2RustUnnamed_0 = 2049;

    pub const SDL_MULTIGESTURE: crate::be_aas_h::C2RustUnnamed_0 = 2050;

    pub const SDL_CLIPBOARDUPDATE: crate::be_aas_h::C2RustUnnamed_0 = 2304;

    pub const SDL_DROPFILE: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const SDL_DROPTEXT: crate::be_aas_h::C2RustUnnamed_0 = 4097;

    pub const SDL_DROPBEGIN: crate::be_aas_h::C2RustUnnamed_0 = 4098;

    pub const SDL_DROPCOMPLETE: crate::be_aas_h::C2RustUnnamed_0 = 4099;

    pub const SDL_AUDIODEVICEADDED: crate::be_aas_h::C2RustUnnamed_0 = 4352;

    pub const SDL_AUDIODEVICEREMOVED: crate::be_aas_h::C2RustUnnamed_0 = 4353;

    pub const SDL_SENSORUPDATE: crate::be_aas_h::C2RustUnnamed_0 = 4608;

    pub const SDL_RENDER_TARGETS_RESET: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const SDL_RENDER_DEVICE_RESET: crate::be_aas_h::C2RustUnnamed_0 = 8193;

    pub const SDL_USEREVENT: crate::be_aas_h::C2RustUnnamed_0 = 32768;

    pub const SDL_LASTEVENT: crate::be_aas_h::C2RustUnnamed_0 = 65535;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_CommonEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_DisplayEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub display: crate::stdlib::Uint32,
        pub event: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub data1: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_WindowEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub event: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub data1: crate::stdlib::Sint32,
        pub data2: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_KeyboardEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub state: crate::stdlib::Uint8,
        pub repeat: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub keysym: crate::stdlib::SDL_Keysym,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_TextEditingEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub text: [libc::c_char; 32],
        pub start: crate::stdlib::Sint32,
        pub length: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_TextInputEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub text: [libc::c_char; 32],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MouseMotionEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub which: crate::stdlib::Uint32,
        pub state: crate::stdlib::Uint32,
        pub x: crate::stdlib::Sint32,
        pub y: crate::stdlib::Sint32,
        pub xrel: crate::stdlib::Sint32,
        pub yrel: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MouseButtonEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub which: crate::stdlib::Uint32,
        pub button: crate::stdlib::Uint8,
        pub state: crate::stdlib::Uint8,
        pub clicks: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub x: crate::stdlib::Sint32,
        pub y: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MouseWheelEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub which: crate::stdlib::Uint32,
        pub x: crate::stdlib::Sint32,
        pub y: crate::stdlib::Sint32,
        pub direction: crate::stdlib::Uint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyAxisEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub axis: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub value: crate::stdlib::Sint16,
        pub padding4: crate::stdlib::Uint16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyBallEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub ball: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub xrel: crate::stdlib::Sint16,
        pub yrel: crate::stdlib::Sint16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyHatEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub hat: crate::stdlib::Uint8,
        pub value: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyButtonEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub button: crate::stdlib::Uint8,
        pub state: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_JoyDeviceEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ControllerAxisEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub axis: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
        pub value: crate::stdlib::Sint16,
        pub padding4: crate::stdlib::Uint16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ControllerButtonEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::SDL_JoystickID,
        pub button: crate::stdlib::Uint8,
        pub state: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_ControllerDeviceEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::Sint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_AudioDeviceEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::Uint32,
        pub iscapture: crate::stdlib::Uint8,
        pub padding1: crate::stdlib::Uint8,
        pub padding2: crate::stdlib::Uint8,
        pub padding3: crate::stdlib::Uint8,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_TouchFingerEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub touchId: crate::stdlib::SDL_TouchID,
        pub fingerId: crate::stdlib::SDL_FingerID,
        pub x: f32,
        pub y: f32,
        pub dx: f32,
        pub dy: f32,
        pub pressure: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_MultiGestureEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub touchId: crate::stdlib::SDL_TouchID,
        pub dTheta: f32,
        pub dDist: f32,
        pub x: f32,
        pub y: f32,
        pub numFingers: crate::stdlib::Uint16,
        pub padding: crate::stdlib::Uint16,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_DollarGestureEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub touchId: crate::stdlib::SDL_TouchID,
        pub gestureId: crate::stdlib::SDL_GestureID,
        pub numFingers: crate::stdlib::Uint32,
        pub error: f32,
        pub x: f32,
        pub y: f32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_DropEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub file: *mut libc::c_char,
        pub windowID: crate::stdlib::Uint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_SensorEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub which: crate::stdlib::Sint32,
        pub data: [f32; 6],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_QuitEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_UserEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub windowID: crate::stdlib::Uint32,
        pub code: crate::stdlib::Sint32,
        pub data1: *mut libc::c_void,
        pub data2: *mut libc::c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_SysWMEvent {
        pub type_0: crate::stdlib::Uint32,
        pub timestamp: crate::stdlib::Uint32,
        pub msg: *mut crate::stdlib::SDL_SysWMmsg,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union SDL_Event {
        pub type_0: crate::stdlib::Uint32,
        pub common: crate::stdlib::SDL_CommonEvent,
        pub display: crate::stdlib::SDL_DisplayEvent,
        pub window: crate::stdlib::SDL_WindowEvent,
        pub key: crate::stdlib::SDL_KeyboardEvent,
        pub edit: crate::stdlib::SDL_TextEditingEvent,
        pub text: crate::stdlib::SDL_TextInputEvent,
        pub motion: crate::stdlib::SDL_MouseMotionEvent,
        pub button: crate::stdlib::SDL_MouseButtonEvent,
        pub wheel: crate::stdlib::SDL_MouseWheelEvent,
        pub jaxis: crate::stdlib::SDL_JoyAxisEvent,
        pub jball: crate::stdlib::SDL_JoyBallEvent,
        pub jhat: crate::stdlib::SDL_JoyHatEvent,
        pub jbutton: crate::stdlib::SDL_JoyButtonEvent,
        pub jdevice: crate::stdlib::SDL_JoyDeviceEvent,
        pub caxis: crate::stdlib::SDL_ControllerAxisEvent,
        pub cbutton: crate::stdlib::SDL_ControllerButtonEvent,
        pub cdevice: crate::stdlib::SDL_ControllerDeviceEvent,
        pub adevice: crate::stdlib::SDL_AudioDeviceEvent,
        pub sensor: crate::stdlib::SDL_SensorEvent,
        pub quit: crate::stdlib::SDL_QuitEvent,
        pub user: crate::stdlib::SDL_UserEvent,
        pub syswm: crate::stdlib::SDL_SysWMEvent,
        pub tfinger: crate::stdlib::SDL_TouchFingerEvent,
        pub mgesture: crate::stdlib::SDL_MultiGestureEvent,
        pub dgesture: crate::stdlib::SDL_DollarGestureEvent,
        pub drop: crate::stdlib::SDL_DropEvent,
        pub padding: [crate::stdlib::Uint8; 56],
    }

    pub type SDL_eventaction = u32;

    pub const SDL_ADDEVENT: crate::stdlib::SDL_eventaction = 0;

    pub const SDL_PEEKEVENT: crate::stdlib::SDL_eventaction = 1;

    pub const SDL_GETEVENT: crate::stdlib::SDL_eventaction = 2;
    pub type SDL_GameController = crate::stdlib::_SDL_GameController;

    pub type SDL_GameControllerAxis = i32;

    pub const SDL_CONTROLLER_AXIS_INVALID: crate::stdlib::SDL_GameControllerAxis = -1;

    pub const SDL_CONTROLLER_AXIS_LEFTX: crate::stdlib::SDL_GameControllerAxis = 0;

    pub const SDL_CONTROLLER_AXIS_LEFTY: crate::stdlib::SDL_GameControllerAxis = 1;

    pub const SDL_CONTROLLER_AXIS_RIGHTX: crate::stdlib::SDL_GameControllerAxis = 2;

    pub const SDL_CONTROLLER_AXIS_RIGHTY: crate::stdlib::SDL_GameControllerAxis = 3;

    pub const SDL_CONTROLLER_AXIS_TRIGGERLEFT: crate::stdlib::SDL_GameControllerAxis = 4;

    pub const SDL_CONTROLLER_AXIS_TRIGGERRIGHT: crate::stdlib::SDL_GameControllerAxis = 5;

    pub const SDL_CONTROLLER_AXIS_MAX: crate::stdlib::SDL_GameControllerAxis = 6;

    pub type SDL_GameControllerButton = i32;

    pub const SDL_CONTROLLER_BUTTON_INVALID: crate::stdlib::SDL_GameControllerButton = -1;

    pub const SDL_CONTROLLER_BUTTON_A: crate::stdlib::SDL_GameControllerButton = 0;

    pub const SDL_CONTROLLER_BUTTON_B: crate::stdlib::SDL_GameControllerButton = 1;

    pub const SDL_CONTROLLER_BUTTON_X: crate::stdlib::SDL_GameControllerButton = 2;

    pub const SDL_CONTROLLER_BUTTON_Y: crate::stdlib::SDL_GameControllerButton = 3;

    pub const SDL_CONTROLLER_BUTTON_BACK: crate::stdlib::SDL_GameControllerButton = 4;

    pub const SDL_CONTROLLER_BUTTON_GUIDE: crate::stdlib::SDL_GameControllerButton = 5;

    pub const SDL_CONTROLLER_BUTTON_START: crate::stdlib::SDL_GameControllerButton = 6;

    pub const SDL_CONTROLLER_BUTTON_LEFTSTICK: crate::stdlib::SDL_GameControllerButton = 7;

    pub const SDL_CONTROLLER_BUTTON_RIGHTSTICK: crate::stdlib::SDL_GameControllerButton = 8;

    pub const SDL_CONTROLLER_BUTTON_LEFTSHOULDER: crate::stdlib::SDL_GameControllerButton = 9;

    pub const SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: crate::stdlib::SDL_GameControllerButton = 10;

    pub const SDL_CONTROLLER_BUTTON_DPAD_UP: crate::stdlib::SDL_GameControllerButton = 11;

    pub const SDL_CONTROLLER_BUTTON_DPAD_DOWN: crate::stdlib::SDL_GameControllerButton = 12;

    pub const SDL_CONTROLLER_BUTTON_DPAD_LEFT: crate::stdlib::SDL_GameControllerButton = 13;

    pub const SDL_CONTROLLER_BUTTON_DPAD_RIGHT: crate::stdlib::SDL_GameControllerButton = 14;

    pub const SDL_CONTROLLER_BUTTON_MAX: crate::stdlib::SDL_GameControllerButton = 15;
    pub type SDL_GestureID = crate::stdlib::Sint64;
    pub type SDL_Joystick = crate::stdlib::_SDL_Joystick;

    pub type SDL_JoystickID = crate::stdlib::Sint32;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_Keysym {
        pub scancode: crate::stdlib::SDL_Scancode,
        pub sym: crate::stdlib::SDL_Keycode,
        pub mod_0: crate::stdlib::Uint16,
        pub unused: crate::stdlib::Uint32,
    }
    pub type SDL_Keycode = crate::stdlib::Sint32;

    pub const SDLK_UNKNOWN: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const SDLK_RETURN: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const SDLK_ESCAPE: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const SDLK_BACKSPACE: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const SDLK_TAB: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const SDLK_SPACE: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const SDLK_EXCLAIM: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const SDLK_QUOTEDBL: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const SDLK_HASH: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const SDLK_PERCENT: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const SDLK_DOLLAR: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const SDLK_AMPERSAND: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const SDLK_QUOTE: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const SDLK_LEFTPAREN: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const SDLK_RIGHTPAREN: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const SDLK_ASTERISK: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const SDLK_PLUS: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const SDLK_COMMA: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const SDLK_MINUS: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const SDLK_PERIOD: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const SDLK_SLASH: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const SDLK_0: crate::be_aas_h::C2RustUnnamed_0 = 48;

    pub const SDLK_1: crate::be_aas_h::C2RustUnnamed_0 = 49;

    pub const SDLK_2: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const SDLK_3: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const SDLK_4: crate::be_aas_h::C2RustUnnamed_0 = 52;

    pub const SDLK_5: crate::be_aas_h::C2RustUnnamed_0 = 53;

    pub const SDLK_6: crate::be_aas_h::C2RustUnnamed_0 = 54;

    pub const SDLK_7: crate::be_aas_h::C2RustUnnamed_0 = 55;

    pub const SDLK_8: crate::be_aas_h::C2RustUnnamed_0 = 56;

    pub const SDLK_9: crate::be_aas_h::C2RustUnnamed_0 = 57;

    pub const SDLK_COLON: crate::be_aas_h::C2RustUnnamed_0 = 58;

    pub const SDLK_SEMICOLON: crate::be_aas_h::C2RustUnnamed_0 = 59;

    pub const SDLK_LESS: crate::be_aas_h::C2RustUnnamed_0 = 60;

    pub const SDLK_EQUALS: crate::be_aas_h::C2RustUnnamed_0 = 61;

    pub const SDLK_GREATER: crate::be_aas_h::C2RustUnnamed_0 = 62;

    pub const SDLK_QUESTION: crate::be_aas_h::C2RustUnnamed_0 = 63;

    pub const SDLK_AT: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const SDLK_LEFTBRACKET: crate::be_aas_h::C2RustUnnamed_0 = 91;

    pub const SDLK_BACKSLASH: crate::be_aas_h::C2RustUnnamed_0 = 92;

    pub const SDLK_RIGHTBRACKET: crate::be_aas_h::C2RustUnnamed_0 = 93;

    pub const SDLK_CARET: crate::be_aas_h::C2RustUnnamed_0 = 94;

    pub const SDLK_UNDERSCORE: crate::be_aas_h::C2RustUnnamed_0 = 95;

    pub const SDLK_BACKQUOTE: crate::be_aas_h::C2RustUnnamed_0 = 96;

    pub const SDLK_a: crate::be_aas_h::C2RustUnnamed_0 = 97;

    pub const SDLK_b: crate::be_aas_h::C2RustUnnamed_0 = 98;

    pub const SDLK_c: crate::be_aas_h::C2RustUnnamed_0 = 99;

    pub const SDLK_d: crate::be_aas_h::C2RustUnnamed_0 = 100;

    pub const SDLK_e: crate::be_aas_h::C2RustUnnamed_0 = 101;

    pub const SDLK_f: crate::be_aas_h::C2RustUnnamed_0 = 102;

    pub const SDLK_g: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const SDLK_h: crate::be_aas_h::C2RustUnnamed_0 = 104;

    pub const SDLK_i: crate::be_aas_h::C2RustUnnamed_0 = 105;

    pub const SDLK_j: crate::be_aas_h::C2RustUnnamed_0 = 106;

    pub const SDLK_k: crate::be_aas_h::C2RustUnnamed_0 = 107;

    pub const SDLK_l: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const SDLK_m: crate::be_aas_h::C2RustUnnamed_0 = 109;

    pub const SDLK_n: crate::be_aas_h::C2RustUnnamed_0 = 110;

    pub const SDLK_o: crate::be_aas_h::C2RustUnnamed_0 = 111;

    pub const SDLK_p: crate::be_aas_h::C2RustUnnamed_0 = 112;

    pub const SDLK_q: crate::be_aas_h::C2RustUnnamed_0 = 113;

    pub const SDLK_r: crate::be_aas_h::C2RustUnnamed_0 = 114;

    pub const SDLK_s: crate::be_aas_h::C2RustUnnamed_0 = 115;

    pub const SDLK_t: crate::be_aas_h::C2RustUnnamed_0 = 116;

    pub const SDLK_u: crate::be_aas_h::C2RustUnnamed_0 = 117;

    pub const SDLK_v: crate::be_aas_h::C2RustUnnamed_0 = 118;

    pub const SDLK_w: crate::be_aas_h::C2RustUnnamed_0 = 119;

    pub const SDLK_x: crate::be_aas_h::C2RustUnnamed_0 = 120;

    pub const SDLK_y: crate::be_aas_h::C2RustUnnamed_0 = 121;

    pub const SDLK_z: crate::be_aas_h::C2RustUnnamed_0 = 122;

    pub const SDLK_CAPSLOCK: crate::be_aas_h::C2RustUnnamed_0 = 1073741881;

    pub const SDLK_F1: crate::be_aas_h::C2RustUnnamed_0 = 1073741882;

    pub const SDLK_F2: crate::be_aas_h::C2RustUnnamed_0 = 1073741883;

    pub const SDLK_F3: crate::be_aas_h::C2RustUnnamed_0 = 1073741884;

    pub const SDLK_F4: crate::be_aas_h::C2RustUnnamed_0 = 1073741885;

    pub const SDLK_F5: crate::be_aas_h::C2RustUnnamed_0 = 1073741886;

    pub const SDLK_F6: crate::be_aas_h::C2RustUnnamed_0 = 1073741887;

    pub const SDLK_F7: crate::be_aas_h::C2RustUnnamed_0 = 1073741888;

    pub const SDLK_F8: crate::be_aas_h::C2RustUnnamed_0 = 1073741889;

    pub const SDLK_F9: crate::be_aas_h::C2RustUnnamed_0 = 1073741890;

    pub const SDLK_F10: crate::be_aas_h::C2RustUnnamed_0 = 1073741891;

    pub const SDLK_F11: crate::be_aas_h::C2RustUnnamed_0 = 1073741892;

    pub const SDLK_F12: crate::be_aas_h::C2RustUnnamed_0 = 1073741893;

    pub const SDLK_PRINTSCREEN: crate::be_aas_h::C2RustUnnamed_0 = 1073741894;

    pub const SDLK_SCROLLLOCK: crate::be_aas_h::C2RustUnnamed_0 = 1073741895;

    pub const SDLK_PAUSE: crate::be_aas_h::C2RustUnnamed_0 = 1073741896;

    pub const SDLK_INSERT: crate::be_aas_h::C2RustUnnamed_0 = 1073741897;

    pub const SDLK_HOME: crate::be_aas_h::C2RustUnnamed_0 = 1073741898;

    pub const SDLK_PAGEUP: crate::be_aas_h::C2RustUnnamed_0 = 1073741899;

    pub const SDLK_DELETE: crate::be_aas_h::C2RustUnnamed_0 = 127;

    pub const SDLK_END: crate::be_aas_h::C2RustUnnamed_0 = 1073741901;

    pub const SDLK_PAGEDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1073741902;

    pub const SDLK_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 1073741903;

    pub const SDLK_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 1073741904;

    pub const SDLK_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 1073741905;

    pub const SDLK_UP: crate::be_aas_h::C2RustUnnamed_0 = 1073741906;

    pub const SDLK_NUMLOCKCLEAR: crate::be_aas_h::C2RustUnnamed_0 = 1073741907;

    pub const SDLK_KP_DIVIDE: crate::be_aas_h::C2RustUnnamed_0 = 1073741908;

    pub const SDLK_KP_MULTIPLY: crate::be_aas_h::C2RustUnnamed_0 = 1073741909;

    pub const SDLK_KP_MINUS: crate::be_aas_h::C2RustUnnamed_0 = 1073741910;

    pub const SDLK_KP_PLUS: crate::be_aas_h::C2RustUnnamed_0 = 1073741911;

    pub const SDLK_KP_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 1073741912;

    pub const SDLK_KP_1: crate::be_aas_h::C2RustUnnamed_0 = 1073741913;

    pub const SDLK_KP_2: crate::be_aas_h::C2RustUnnamed_0 = 1073741914;

    pub const SDLK_KP_3: crate::be_aas_h::C2RustUnnamed_0 = 1073741915;

    pub const SDLK_KP_4: crate::be_aas_h::C2RustUnnamed_0 = 1073741916;

    pub const SDLK_KP_5: crate::be_aas_h::C2RustUnnamed_0 = 1073741917;

    pub const SDLK_KP_6: crate::be_aas_h::C2RustUnnamed_0 = 1073741918;

    pub const SDLK_KP_7: crate::be_aas_h::C2RustUnnamed_0 = 1073741919;

    pub const SDLK_KP_8: crate::be_aas_h::C2RustUnnamed_0 = 1073741920;

    pub const SDLK_KP_9: crate::be_aas_h::C2RustUnnamed_0 = 1073741921;

    pub const SDLK_KP_0: crate::be_aas_h::C2RustUnnamed_0 = 1073741922;

    pub const SDLK_KP_PERIOD: crate::be_aas_h::C2RustUnnamed_0 = 1073741923;

    pub const SDLK_APPLICATION: crate::be_aas_h::C2RustUnnamed_0 = 1073741925;

    pub const SDLK_POWER: crate::be_aas_h::C2RustUnnamed_0 = 1073741926;

    pub const SDLK_KP_EQUALS: crate::be_aas_h::C2RustUnnamed_0 = 1073741927;

    pub const SDLK_F13: crate::be_aas_h::C2RustUnnamed_0 = 1073741928;

    pub const SDLK_F14: crate::be_aas_h::C2RustUnnamed_0 = 1073741929;

    pub const SDLK_F15: crate::be_aas_h::C2RustUnnamed_0 = 1073741930;

    pub const SDLK_F16: crate::be_aas_h::C2RustUnnamed_0 = 1073741931;

    pub const SDLK_F17: crate::be_aas_h::C2RustUnnamed_0 = 1073741932;

    pub const SDLK_F18: crate::be_aas_h::C2RustUnnamed_0 = 1073741933;

    pub const SDLK_F19: crate::be_aas_h::C2RustUnnamed_0 = 1073741934;

    pub const SDLK_F20: crate::be_aas_h::C2RustUnnamed_0 = 1073741935;

    pub const SDLK_F21: crate::be_aas_h::C2RustUnnamed_0 = 1073741936;

    pub const SDLK_F22: crate::be_aas_h::C2RustUnnamed_0 = 1073741937;

    pub const SDLK_F23: crate::be_aas_h::C2RustUnnamed_0 = 1073741938;

    pub const SDLK_F24: crate::be_aas_h::C2RustUnnamed_0 = 1073741939;

    pub const SDLK_EXECUTE: crate::be_aas_h::C2RustUnnamed_0 = 1073741940;

    pub const SDLK_HELP: crate::be_aas_h::C2RustUnnamed_0 = 1073741941;

    pub const SDLK_MENU: crate::be_aas_h::C2RustUnnamed_0 = 1073741942;

    pub const SDLK_SELECT: crate::be_aas_h::C2RustUnnamed_0 = 1073741943;

    pub const SDLK_STOP: crate::be_aas_h::C2RustUnnamed_0 = 1073741944;

    pub const SDLK_AGAIN: crate::be_aas_h::C2RustUnnamed_0 = 1073741945;

    pub const SDLK_UNDO: crate::be_aas_h::C2RustUnnamed_0 = 1073741946;

    pub const SDLK_CUT: crate::be_aas_h::C2RustUnnamed_0 = 1073741947;

    pub const SDLK_COPY: crate::be_aas_h::C2RustUnnamed_0 = 1073741948;

    pub const SDLK_PASTE: crate::be_aas_h::C2RustUnnamed_0 = 1073741949;

    pub const SDLK_FIND: crate::be_aas_h::C2RustUnnamed_0 = 1073741950;

    pub const SDLK_MUTE: crate::be_aas_h::C2RustUnnamed_0 = 1073741951;

    pub const SDLK_VOLUMEUP: crate::be_aas_h::C2RustUnnamed_0 = 1073741952;

    pub const SDLK_VOLUMEDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1073741953;

    pub const SDLK_KP_COMMA: crate::be_aas_h::C2RustUnnamed_0 = 1073741957;

    pub const SDLK_KP_EQUALSAS400: crate::be_aas_h::C2RustUnnamed_0 = 1073741958;

    pub const SDLK_ALTERASE: crate::be_aas_h::C2RustUnnamed_0 = 1073741977;

    pub const SDLK_SYSREQ: crate::be_aas_h::C2RustUnnamed_0 = 1073741978;

    pub const SDLK_CANCEL: crate::be_aas_h::C2RustUnnamed_0 = 1073741979;

    pub const SDLK_CLEAR: crate::be_aas_h::C2RustUnnamed_0 = 1073741980;

    pub const SDLK_PRIOR: crate::be_aas_h::C2RustUnnamed_0 = 1073741981;

    pub const SDLK_RETURN2: crate::be_aas_h::C2RustUnnamed_0 = 1073741982;

    pub const SDLK_SEPARATOR: crate::be_aas_h::C2RustUnnamed_0 = 1073741983;

    pub const SDLK_OUT: crate::be_aas_h::C2RustUnnamed_0 = 1073741984;

    pub const SDLK_OPER: crate::be_aas_h::C2RustUnnamed_0 = 1073741985;

    pub const SDLK_CLEARAGAIN: crate::be_aas_h::C2RustUnnamed_0 = 1073741986;

    pub const SDLK_CRSEL: crate::be_aas_h::C2RustUnnamed_0 = 1073741987;

    pub const SDLK_EXSEL: crate::be_aas_h::C2RustUnnamed_0 = 1073741988;

    pub const SDLK_KP_00: crate::be_aas_h::C2RustUnnamed_0 = 1073742000;

    pub const SDLK_KP_000: crate::be_aas_h::C2RustUnnamed_0 = 1073742001;

    pub const SDLK_THOUSANDSSEPARATOR: crate::be_aas_h::C2RustUnnamed_0 = 1073742002;

    pub const SDLK_DECIMALSEPARATOR: crate::be_aas_h::C2RustUnnamed_0 = 1073742003;

    pub const SDLK_CURRENCYUNIT: crate::be_aas_h::C2RustUnnamed_0 = 1073742004;

    pub const SDLK_CURRENCYSUBUNIT: crate::be_aas_h::C2RustUnnamed_0 = 1073742005;

    pub const SDLK_KP_LEFTPAREN: crate::be_aas_h::C2RustUnnamed_0 = 1073742006;

    pub const SDLK_KP_RIGHTPAREN: crate::be_aas_h::C2RustUnnamed_0 = 1073742007;

    pub const SDLK_KP_LEFTBRACE: crate::be_aas_h::C2RustUnnamed_0 = 1073742008;

    pub const SDLK_KP_RIGHTBRACE: crate::be_aas_h::C2RustUnnamed_0 = 1073742009;

    pub const SDLK_KP_TAB: crate::be_aas_h::C2RustUnnamed_0 = 1073742010;

    pub const SDLK_KP_BACKSPACE: crate::be_aas_h::C2RustUnnamed_0 = 1073742011;

    pub const SDLK_KP_A: crate::be_aas_h::C2RustUnnamed_0 = 1073742012;

    pub const SDLK_KP_B: crate::be_aas_h::C2RustUnnamed_0 = 1073742013;

    pub const SDLK_KP_C: crate::be_aas_h::C2RustUnnamed_0 = 1073742014;

    pub const SDLK_KP_D: crate::be_aas_h::C2RustUnnamed_0 = 1073742015;

    pub const SDLK_KP_E: crate::be_aas_h::C2RustUnnamed_0 = 1073742016;

    pub const SDLK_KP_F: crate::be_aas_h::C2RustUnnamed_0 = 1073742017;

    pub const SDLK_KP_XOR: crate::be_aas_h::C2RustUnnamed_0 = 1073742018;

    pub const SDLK_KP_POWER: crate::be_aas_h::C2RustUnnamed_0 = 1073742019;

    pub const SDLK_KP_PERCENT: crate::be_aas_h::C2RustUnnamed_0 = 1073742020;

    pub const SDLK_KP_LESS: crate::be_aas_h::C2RustUnnamed_0 = 1073742021;

    pub const SDLK_KP_GREATER: crate::be_aas_h::C2RustUnnamed_0 = 1073742022;

    pub const SDLK_KP_AMPERSAND: crate::be_aas_h::C2RustUnnamed_0 = 1073742023;

    pub const SDLK_KP_DBLAMPERSAND: crate::be_aas_h::C2RustUnnamed_0 = 1073742024;

    pub const SDLK_KP_VERTICALBAR: crate::be_aas_h::C2RustUnnamed_0 = 1073742025;

    pub const SDLK_KP_DBLVERTICALBAR: crate::be_aas_h::C2RustUnnamed_0 = 1073742026;

    pub const SDLK_KP_COLON: crate::be_aas_h::C2RustUnnamed_0 = 1073742027;

    pub const SDLK_KP_HASH: crate::be_aas_h::C2RustUnnamed_0 = 1073742028;

    pub const SDLK_KP_SPACE: crate::be_aas_h::C2RustUnnamed_0 = 1073742029;

    pub const SDLK_KP_AT: crate::be_aas_h::C2RustUnnamed_0 = 1073742030;

    pub const SDLK_KP_EXCLAM: crate::be_aas_h::C2RustUnnamed_0 = 1073742031;

    pub const SDLK_KP_MEMSTORE: crate::be_aas_h::C2RustUnnamed_0 = 1073742032;

    pub const SDLK_KP_MEMRECALL: crate::be_aas_h::C2RustUnnamed_0 = 1073742033;

    pub const SDLK_KP_MEMCLEAR: crate::be_aas_h::C2RustUnnamed_0 = 1073742034;

    pub const SDLK_KP_MEMADD: crate::be_aas_h::C2RustUnnamed_0 = 1073742035;

    pub const SDLK_KP_MEMSUBTRACT: crate::be_aas_h::C2RustUnnamed_0 = 1073742036;

    pub const SDLK_KP_MEMMULTIPLY: crate::be_aas_h::C2RustUnnamed_0 = 1073742037;

    pub const SDLK_KP_MEMDIVIDE: crate::be_aas_h::C2RustUnnamed_0 = 1073742038;

    pub const SDLK_KP_PLUSMINUS: crate::be_aas_h::C2RustUnnamed_0 = 1073742039;

    pub const SDLK_KP_CLEAR: crate::be_aas_h::C2RustUnnamed_0 = 1073742040;

    pub const SDLK_KP_CLEARENTRY: crate::be_aas_h::C2RustUnnamed_0 = 1073742041;

    pub const SDLK_KP_BINARY: crate::be_aas_h::C2RustUnnamed_0 = 1073742042;

    pub const SDLK_KP_OCTAL: crate::be_aas_h::C2RustUnnamed_0 = 1073742043;

    pub const SDLK_KP_DECIMAL: crate::be_aas_h::C2RustUnnamed_0 = 1073742044;

    pub const SDLK_KP_HEXADECIMAL: crate::be_aas_h::C2RustUnnamed_0 = 1073742045;

    pub const SDLK_LCTRL: crate::be_aas_h::C2RustUnnamed_0 = 1073742048;

    pub const SDLK_LSHIFT: crate::be_aas_h::C2RustUnnamed_0 = 1073742049;

    pub const SDLK_LALT: crate::be_aas_h::C2RustUnnamed_0 = 1073742050;

    pub const SDLK_LGUI: crate::be_aas_h::C2RustUnnamed_0 = 1073742051;

    pub const SDLK_RCTRL: crate::be_aas_h::C2RustUnnamed_0 = 1073742052;

    pub const SDLK_RSHIFT: crate::be_aas_h::C2RustUnnamed_0 = 1073742053;

    pub const SDLK_RALT: crate::be_aas_h::C2RustUnnamed_0 = 1073742054;

    pub const SDLK_RGUI: crate::be_aas_h::C2RustUnnamed_0 = 1073742055;

    pub const SDLK_MODE: crate::be_aas_h::C2RustUnnamed_0 = 1073742081;

    pub const SDLK_AUDIONEXT: crate::be_aas_h::C2RustUnnamed_0 = 1073742082;

    pub const SDLK_AUDIOPREV: crate::be_aas_h::C2RustUnnamed_0 = 1073742083;

    pub const SDLK_AUDIOSTOP: crate::be_aas_h::C2RustUnnamed_0 = 1073742084;

    pub const SDLK_AUDIOPLAY: crate::be_aas_h::C2RustUnnamed_0 = 1073742085;

    pub const SDLK_AUDIOMUTE: crate::be_aas_h::C2RustUnnamed_0 = 1073742086;

    pub const SDLK_MEDIASELECT: crate::be_aas_h::C2RustUnnamed_0 = 1073742087;

    pub const SDLK_WWW: crate::be_aas_h::C2RustUnnamed_0 = 1073742088;

    pub const SDLK_MAIL: crate::be_aas_h::C2RustUnnamed_0 = 1073742089;

    pub const SDLK_CALCULATOR: crate::be_aas_h::C2RustUnnamed_0 = 1073742090;

    pub const SDLK_COMPUTER: crate::be_aas_h::C2RustUnnamed_0 = 1073742091;

    pub const SDLK_AC_SEARCH: crate::be_aas_h::C2RustUnnamed_0 = 1073742092;

    pub const SDLK_AC_HOME: crate::be_aas_h::C2RustUnnamed_0 = 1073742093;

    pub const SDLK_AC_BACK: crate::be_aas_h::C2RustUnnamed_0 = 1073742094;

    pub const SDLK_AC_FORWARD: crate::be_aas_h::C2RustUnnamed_0 = 1073742095;

    pub const SDLK_AC_STOP: crate::be_aas_h::C2RustUnnamed_0 = 1073742096;

    pub const SDLK_AC_REFRESH: crate::be_aas_h::C2RustUnnamed_0 = 1073742097;

    pub const SDLK_AC_BOOKMARKS: crate::be_aas_h::C2RustUnnamed_0 = 1073742098;

    pub const SDLK_BRIGHTNESSDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1073742099;

    pub const SDLK_BRIGHTNESSUP: crate::be_aas_h::C2RustUnnamed_0 = 1073742100;

    pub const SDLK_DISPLAYSWITCH: crate::be_aas_h::C2RustUnnamed_0 = 1073742101;

    pub const SDLK_KBDILLUMTOGGLE: crate::be_aas_h::C2RustUnnamed_0 = 1073742102;

    pub const SDLK_KBDILLUMDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1073742103;

    pub const SDLK_KBDILLUMUP: crate::be_aas_h::C2RustUnnamed_0 = 1073742104;

    pub const SDLK_EJECT: crate::be_aas_h::C2RustUnnamed_0 = 1073742105;

    pub const SDLK_SLEEP: crate::be_aas_h::C2RustUnnamed_0 = 1073742106;

    pub const SDLK_APP1: crate::be_aas_h::C2RustUnnamed_0 = 1073742107;

    pub const SDLK_APP2: crate::be_aas_h::C2RustUnnamed_0 = 1073742108;

    pub const SDLK_AUDIOREWIND: crate::be_aas_h::C2RustUnnamed_0 = 1073742109;

    pub const SDLK_AUDIOFASTFORWARD: crate::be_aas_h::C2RustUnnamed_0 = 1073742110;

    pub const KMOD_NONE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const KMOD_LSHIFT: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const KMOD_RSHIFT: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const KMOD_LCTRL: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const KMOD_RCTRL: crate::be_aas_h::C2RustUnnamed_0 = 128;

    pub const KMOD_LALT: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const KMOD_RALT: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const KMOD_LGUI: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const KMOD_RGUI: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const KMOD_NUM: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const KMOD_CAPS: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const KMOD_MODE: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const KMOD_RESERVED: crate::be_aas_h::C2RustUnnamed_0 = 32768;
    pub type SDL_Scancode = u32;

    pub const SDL_SCANCODE_UNKNOWN: crate::stdlib::SDL_Scancode = 0;

    pub const SDL_SCANCODE_A: crate::stdlib::SDL_Scancode = 4;

    pub const SDL_SCANCODE_B: crate::stdlib::SDL_Scancode = 5;

    pub const SDL_SCANCODE_C: crate::stdlib::SDL_Scancode = 6;

    pub const SDL_SCANCODE_D: crate::stdlib::SDL_Scancode = 7;

    pub const SDL_SCANCODE_E: crate::stdlib::SDL_Scancode = 8;

    pub const SDL_SCANCODE_F: crate::stdlib::SDL_Scancode = 9;

    pub const SDL_SCANCODE_G: crate::stdlib::SDL_Scancode = 10;

    pub const SDL_SCANCODE_H: crate::stdlib::SDL_Scancode = 11;

    pub const SDL_SCANCODE_I: crate::stdlib::SDL_Scancode = 12;

    pub const SDL_SCANCODE_J: crate::stdlib::SDL_Scancode = 13;

    pub const SDL_SCANCODE_K: crate::stdlib::SDL_Scancode = 14;

    pub const SDL_SCANCODE_L: crate::stdlib::SDL_Scancode = 15;

    pub const SDL_SCANCODE_M: crate::stdlib::SDL_Scancode = 16;

    pub const SDL_SCANCODE_N: crate::stdlib::SDL_Scancode = 17;

    pub const SDL_SCANCODE_O: crate::stdlib::SDL_Scancode = 18;

    pub const SDL_SCANCODE_P: crate::stdlib::SDL_Scancode = 19;

    pub const SDL_SCANCODE_Q: crate::stdlib::SDL_Scancode = 20;

    pub const SDL_SCANCODE_R: crate::stdlib::SDL_Scancode = 21;

    pub const SDL_SCANCODE_S: crate::stdlib::SDL_Scancode = 22;

    pub const SDL_SCANCODE_T: crate::stdlib::SDL_Scancode = 23;

    pub const SDL_SCANCODE_U: crate::stdlib::SDL_Scancode = 24;

    pub const SDL_SCANCODE_V: crate::stdlib::SDL_Scancode = 25;

    pub const SDL_SCANCODE_W: crate::stdlib::SDL_Scancode = 26;

    pub const SDL_SCANCODE_X: crate::stdlib::SDL_Scancode = 27;

    pub const SDL_SCANCODE_Y: crate::stdlib::SDL_Scancode = 28;

    pub const SDL_SCANCODE_Z: crate::stdlib::SDL_Scancode = 29;

    pub const SDL_SCANCODE_1: crate::stdlib::SDL_Scancode = 30;

    pub const SDL_SCANCODE_2: crate::stdlib::SDL_Scancode = 31;

    pub const SDL_SCANCODE_3: crate::stdlib::SDL_Scancode = 32;

    pub const SDL_SCANCODE_4: crate::stdlib::SDL_Scancode = 33;

    pub const SDL_SCANCODE_5: crate::stdlib::SDL_Scancode = 34;

    pub const SDL_SCANCODE_6: crate::stdlib::SDL_Scancode = 35;

    pub const SDL_SCANCODE_7: crate::stdlib::SDL_Scancode = 36;

    pub const SDL_SCANCODE_8: crate::stdlib::SDL_Scancode = 37;

    pub const SDL_SCANCODE_9: crate::stdlib::SDL_Scancode = 38;

    pub const SDL_SCANCODE_0: crate::stdlib::SDL_Scancode = 39;

    pub const SDL_SCANCODE_RETURN: crate::stdlib::SDL_Scancode = 40;

    pub const SDL_SCANCODE_ESCAPE: crate::stdlib::SDL_Scancode = 41;

    pub const SDL_SCANCODE_BACKSPACE: crate::stdlib::SDL_Scancode = 42;

    pub const SDL_SCANCODE_TAB: crate::stdlib::SDL_Scancode = 43;

    pub const SDL_SCANCODE_SPACE: crate::stdlib::SDL_Scancode = 44;

    pub const SDL_SCANCODE_MINUS: crate::stdlib::SDL_Scancode = 45;

    pub const SDL_SCANCODE_EQUALS: crate::stdlib::SDL_Scancode = 46;

    pub const SDL_SCANCODE_LEFTBRACKET: crate::stdlib::SDL_Scancode = 47;

    pub const SDL_SCANCODE_RIGHTBRACKET: crate::stdlib::SDL_Scancode = 48;

    pub const SDL_SCANCODE_BACKSLASH: crate::stdlib::SDL_Scancode = 49;

    pub const SDL_SCANCODE_NONUSHASH: crate::stdlib::SDL_Scancode = 50;

    pub const SDL_SCANCODE_SEMICOLON: crate::stdlib::SDL_Scancode = 51;

    pub const SDL_SCANCODE_APOSTROPHE: crate::stdlib::SDL_Scancode = 52;

    pub const SDL_SCANCODE_GRAVE: crate::stdlib::SDL_Scancode = 53;

    pub const SDL_SCANCODE_COMMA: crate::stdlib::SDL_Scancode = 54;

    pub const SDL_SCANCODE_PERIOD: crate::stdlib::SDL_Scancode = 55;

    pub const SDL_SCANCODE_SLASH: crate::stdlib::SDL_Scancode = 56;

    pub const SDL_SCANCODE_CAPSLOCK: crate::stdlib::SDL_Scancode = 57;

    pub const SDL_SCANCODE_F1: crate::stdlib::SDL_Scancode = 58;

    pub const SDL_SCANCODE_F2: crate::stdlib::SDL_Scancode = 59;

    pub const SDL_SCANCODE_F3: crate::stdlib::SDL_Scancode = 60;

    pub const SDL_SCANCODE_F4: crate::stdlib::SDL_Scancode = 61;

    pub const SDL_SCANCODE_F5: crate::stdlib::SDL_Scancode = 62;

    pub const SDL_SCANCODE_F6: crate::stdlib::SDL_Scancode = 63;

    pub const SDL_SCANCODE_F7: crate::stdlib::SDL_Scancode = 64;

    pub const SDL_SCANCODE_F8: crate::stdlib::SDL_Scancode = 65;

    pub const SDL_SCANCODE_F9: crate::stdlib::SDL_Scancode = 66;

    pub const SDL_SCANCODE_F10: crate::stdlib::SDL_Scancode = 67;

    pub const SDL_SCANCODE_F11: crate::stdlib::SDL_Scancode = 68;

    pub const SDL_SCANCODE_F12: crate::stdlib::SDL_Scancode = 69;

    pub const SDL_SCANCODE_PRINTSCREEN: crate::stdlib::SDL_Scancode = 70;

    pub const SDL_SCANCODE_SCROLLLOCK: crate::stdlib::SDL_Scancode = 71;

    pub const SDL_SCANCODE_PAUSE: crate::stdlib::SDL_Scancode = 72;

    pub const SDL_SCANCODE_INSERT: crate::stdlib::SDL_Scancode = 73;

    pub const SDL_SCANCODE_HOME: crate::stdlib::SDL_Scancode = 74;

    pub const SDL_SCANCODE_PAGEUP: crate::stdlib::SDL_Scancode = 75;

    pub const SDL_SCANCODE_DELETE: crate::stdlib::SDL_Scancode = 76;

    pub const SDL_SCANCODE_END: crate::stdlib::SDL_Scancode = 77;

    pub const SDL_SCANCODE_PAGEDOWN: crate::stdlib::SDL_Scancode = 78;

    pub const SDL_SCANCODE_RIGHT: crate::stdlib::SDL_Scancode = 79;

    pub const SDL_SCANCODE_LEFT: crate::stdlib::SDL_Scancode = 80;

    pub const SDL_SCANCODE_DOWN: crate::stdlib::SDL_Scancode = 81;

    pub const SDL_SCANCODE_UP: crate::stdlib::SDL_Scancode = 82;

    pub const SDL_SCANCODE_NUMLOCKCLEAR: crate::stdlib::SDL_Scancode = 83;

    pub const SDL_SCANCODE_KP_DIVIDE: crate::stdlib::SDL_Scancode = 84;

    pub const SDL_SCANCODE_KP_MULTIPLY: crate::stdlib::SDL_Scancode = 85;

    pub const SDL_SCANCODE_KP_MINUS: crate::stdlib::SDL_Scancode = 86;

    pub const SDL_SCANCODE_KP_PLUS: crate::stdlib::SDL_Scancode = 87;

    pub const SDL_SCANCODE_KP_ENTER: crate::stdlib::SDL_Scancode = 88;

    pub const SDL_SCANCODE_KP_1: crate::stdlib::SDL_Scancode = 89;

    pub const SDL_SCANCODE_KP_2: crate::stdlib::SDL_Scancode = 90;

    pub const SDL_SCANCODE_KP_3: crate::stdlib::SDL_Scancode = 91;

    pub const SDL_SCANCODE_KP_4: crate::stdlib::SDL_Scancode = 92;

    pub const SDL_SCANCODE_KP_5: crate::stdlib::SDL_Scancode = 93;

    pub const SDL_SCANCODE_KP_6: crate::stdlib::SDL_Scancode = 94;

    pub const SDL_SCANCODE_KP_7: crate::stdlib::SDL_Scancode = 95;

    pub const SDL_SCANCODE_KP_8: crate::stdlib::SDL_Scancode = 96;

    pub const SDL_SCANCODE_KP_9: crate::stdlib::SDL_Scancode = 97;

    pub const SDL_SCANCODE_KP_0: crate::stdlib::SDL_Scancode = 98;

    pub const SDL_SCANCODE_KP_PERIOD: crate::stdlib::SDL_Scancode = 99;

    pub const SDL_SCANCODE_NONUSBACKSLASH: crate::stdlib::SDL_Scancode = 100;

    pub const SDL_SCANCODE_APPLICATION: crate::stdlib::SDL_Scancode = 101;

    pub const SDL_SCANCODE_POWER: crate::stdlib::SDL_Scancode = 102;

    pub const SDL_SCANCODE_KP_EQUALS: crate::stdlib::SDL_Scancode = 103;

    pub const SDL_SCANCODE_F13: crate::stdlib::SDL_Scancode = 104;

    pub const SDL_SCANCODE_F14: crate::stdlib::SDL_Scancode = 105;

    pub const SDL_SCANCODE_F15: crate::stdlib::SDL_Scancode = 106;

    pub const SDL_SCANCODE_F16: crate::stdlib::SDL_Scancode = 107;

    pub const SDL_SCANCODE_F17: crate::stdlib::SDL_Scancode = 108;

    pub const SDL_SCANCODE_F18: crate::stdlib::SDL_Scancode = 109;

    pub const SDL_SCANCODE_F19: crate::stdlib::SDL_Scancode = 110;

    pub const SDL_SCANCODE_F20: crate::stdlib::SDL_Scancode = 111;

    pub const SDL_SCANCODE_F21: crate::stdlib::SDL_Scancode = 112;

    pub const SDL_SCANCODE_F22: crate::stdlib::SDL_Scancode = 113;

    pub const SDL_SCANCODE_F23: crate::stdlib::SDL_Scancode = 114;

    pub const SDL_SCANCODE_F24: crate::stdlib::SDL_Scancode = 115;

    pub const SDL_SCANCODE_EXECUTE: crate::stdlib::SDL_Scancode = 116;

    pub const SDL_SCANCODE_HELP: crate::stdlib::SDL_Scancode = 117;

    pub const SDL_SCANCODE_MENU: crate::stdlib::SDL_Scancode = 118;

    pub const SDL_SCANCODE_SELECT: crate::stdlib::SDL_Scancode = 119;

    pub const SDL_SCANCODE_STOP: crate::stdlib::SDL_Scancode = 120;

    pub const SDL_SCANCODE_AGAIN: crate::stdlib::SDL_Scancode = 121;

    pub const SDL_SCANCODE_UNDO: crate::stdlib::SDL_Scancode = 122;

    pub const SDL_SCANCODE_CUT: crate::stdlib::SDL_Scancode = 123;

    pub const SDL_SCANCODE_COPY: crate::stdlib::SDL_Scancode = 124;

    pub const SDL_SCANCODE_PASTE: crate::stdlib::SDL_Scancode = 125;

    pub const SDL_SCANCODE_FIND: crate::stdlib::SDL_Scancode = 126;

    pub const SDL_SCANCODE_MUTE: crate::stdlib::SDL_Scancode = 127;

    pub const SDL_SCANCODE_VOLUMEUP: crate::stdlib::SDL_Scancode = 128;

    pub const SDL_SCANCODE_VOLUMEDOWN: crate::stdlib::SDL_Scancode = 129;

    pub const SDL_SCANCODE_KP_COMMA: crate::stdlib::SDL_Scancode = 133;

    pub const SDL_SCANCODE_KP_EQUALSAS400: crate::stdlib::SDL_Scancode = 134;

    pub const SDL_SCANCODE_INTERNATIONAL1: crate::stdlib::SDL_Scancode = 135;

    pub const SDL_SCANCODE_INTERNATIONAL2: crate::stdlib::SDL_Scancode = 136;

    pub const SDL_SCANCODE_INTERNATIONAL3: crate::stdlib::SDL_Scancode = 137;

    pub const SDL_SCANCODE_INTERNATIONAL4: crate::stdlib::SDL_Scancode = 138;

    pub const SDL_SCANCODE_INTERNATIONAL5: crate::stdlib::SDL_Scancode = 139;

    pub const SDL_SCANCODE_INTERNATIONAL6: crate::stdlib::SDL_Scancode = 140;

    pub const SDL_SCANCODE_INTERNATIONAL7: crate::stdlib::SDL_Scancode = 141;

    pub const SDL_SCANCODE_INTERNATIONAL8: crate::stdlib::SDL_Scancode = 142;

    pub const SDL_SCANCODE_INTERNATIONAL9: crate::stdlib::SDL_Scancode = 143;

    pub const SDL_SCANCODE_LANG1: crate::stdlib::SDL_Scancode = 144;

    pub const SDL_SCANCODE_LANG2: crate::stdlib::SDL_Scancode = 145;

    pub const SDL_SCANCODE_LANG3: crate::stdlib::SDL_Scancode = 146;

    pub const SDL_SCANCODE_LANG4: crate::stdlib::SDL_Scancode = 147;

    pub const SDL_SCANCODE_LANG5: crate::stdlib::SDL_Scancode = 148;

    pub const SDL_SCANCODE_LANG6: crate::stdlib::SDL_Scancode = 149;

    pub const SDL_SCANCODE_LANG7: crate::stdlib::SDL_Scancode = 150;

    pub const SDL_SCANCODE_LANG8: crate::stdlib::SDL_Scancode = 151;

    pub const SDL_SCANCODE_LANG9: crate::stdlib::SDL_Scancode = 152;

    pub const SDL_SCANCODE_ALTERASE: crate::stdlib::SDL_Scancode = 153;

    pub const SDL_SCANCODE_SYSREQ: crate::stdlib::SDL_Scancode = 154;

    pub const SDL_SCANCODE_CANCEL: crate::stdlib::SDL_Scancode = 155;

    pub const SDL_SCANCODE_CLEAR: crate::stdlib::SDL_Scancode = 156;

    pub const SDL_SCANCODE_PRIOR: crate::stdlib::SDL_Scancode = 157;

    pub const SDL_SCANCODE_RETURN2: crate::stdlib::SDL_Scancode = 158;

    pub const SDL_SCANCODE_SEPARATOR: crate::stdlib::SDL_Scancode = 159;

    pub const SDL_SCANCODE_OUT: crate::stdlib::SDL_Scancode = 160;

    pub const SDL_SCANCODE_OPER: crate::stdlib::SDL_Scancode = 161;

    pub const SDL_SCANCODE_CLEARAGAIN: crate::stdlib::SDL_Scancode = 162;

    pub const SDL_SCANCODE_CRSEL: crate::stdlib::SDL_Scancode = 163;

    pub const SDL_SCANCODE_EXSEL: crate::stdlib::SDL_Scancode = 164;

    pub const SDL_SCANCODE_KP_00: crate::stdlib::SDL_Scancode = 176;

    pub const SDL_SCANCODE_KP_000: crate::stdlib::SDL_Scancode = 177;

    pub const SDL_SCANCODE_THOUSANDSSEPARATOR: crate::stdlib::SDL_Scancode = 178;

    pub const SDL_SCANCODE_DECIMALSEPARATOR: crate::stdlib::SDL_Scancode = 179;

    pub const SDL_SCANCODE_CURRENCYUNIT: crate::stdlib::SDL_Scancode = 180;

    pub const SDL_SCANCODE_CURRENCYSUBUNIT: crate::stdlib::SDL_Scancode = 181;

    pub const SDL_SCANCODE_KP_LEFTPAREN: crate::stdlib::SDL_Scancode = 182;

    pub const SDL_SCANCODE_KP_RIGHTPAREN: crate::stdlib::SDL_Scancode = 183;

    pub const SDL_SCANCODE_KP_LEFTBRACE: crate::stdlib::SDL_Scancode = 184;

    pub const SDL_SCANCODE_KP_RIGHTBRACE: crate::stdlib::SDL_Scancode = 185;

    pub const SDL_SCANCODE_KP_TAB: crate::stdlib::SDL_Scancode = 186;

    pub const SDL_SCANCODE_KP_BACKSPACE: crate::stdlib::SDL_Scancode = 187;

    pub const SDL_SCANCODE_KP_A: crate::stdlib::SDL_Scancode = 188;

    pub const SDL_SCANCODE_KP_B: crate::stdlib::SDL_Scancode = 189;

    pub const SDL_SCANCODE_KP_C: crate::stdlib::SDL_Scancode = 190;

    pub const SDL_SCANCODE_KP_D: crate::stdlib::SDL_Scancode = 191;

    pub const SDL_SCANCODE_KP_E: crate::stdlib::SDL_Scancode = 192;

    pub const SDL_SCANCODE_KP_F: crate::stdlib::SDL_Scancode = 193;

    pub const SDL_SCANCODE_KP_XOR: crate::stdlib::SDL_Scancode = 194;

    pub const SDL_SCANCODE_KP_POWER: crate::stdlib::SDL_Scancode = 195;

    pub const SDL_SCANCODE_KP_PERCENT: crate::stdlib::SDL_Scancode = 196;

    pub const SDL_SCANCODE_KP_LESS: crate::stdlib::SDL_Scancode = 197;

    pub const SDL_SCANCODE_KP_GREATER: crate::stdlib::SDL_Scancode = 198;

    pub const SDL_SCANCODE_KP_AMPERSAND: crate::stdlib::SDL_Scancode = 199;

    pub const SDL_SCANCODE_KP_DBLAMPERSAND: crate::stdlib::SDL_Scancode = 200;

    pub const SDL_SCANCODE_KP_VERTICALBAR: crate::stdlib::SDL_Scancode = 201;

    pub const SDL_SCANCODE_KP_DBLVERTICALBAR: crate::stdlib::SDL_Scancode = 202;

    pub const SDL_SCANCODE_KP_COLON: crate::stdlib::SDL_Scancode = 203;

    pub const SDL_SCANCODE_KP_HASH: crate::stdlib::SDL_Scancode = 204;

    pub const SDL_SCANCODE_KP_SPACE: crate::stdlib::SDL_Scancode = 205;

    pub const SDL_SCANCODE_KP_AT: crate::stdlib::SDL_Scancode = 206;

    pub const SDL_SCANCODE_KP_EXCLAM: crate::stdlib::SDL_Scancode = 207;

    pub const SDL_SCANCODE_KP_MEMSTORE: crate::stdlib::SDL_Scancode = 208;

    pub const SDL_SCANCODE_KP_MEMRECALL: crate::stdlib::SDL_Scancode = 209;

    pub const SDL_SCANCODE_KP_MEMCLEAR: crate::stdlib::SDL_Scancode = 210;

    pub const SDL_SCANCODE_KP_MEMADD: crate::stdlib::SDL_Scancode = 211;

    pub const SDL_SCANCODE_KP_MEMSUBTRACT: crate::stdlib::SDL_Scancode = 212;

    pub const SDL_SCANCODE_KP_MEMMULTIPLY: crate::stdlib::SDL_Scancode = 213;

    pub const SDL_SCANCODE_KP_MEMDIVIDE: crate::stdlib::SDL_Scancode = 214;

    pub const SDL_SCANCODE_KP_PLUSMINUS: crate::stdlib::SDL_Scancode = 215;

    pub const SDL_SCANCODE_KP_CLEAR: crate::stdlib::SDL_Scancode = 216;

    pub const SDL_SCANCODE_KP_CLEARENTRY: crate::stdlib::SDL_Scancode = 217;

    pub const SDL_SCANCODE_KP_BINARY: crate::stdlib::SDL_Scancode = 218;

    pub const SDL_SCANCODE_KP_OCTAL: crate::stdlib::SDL_Scancode = 219;

    pub const SDL_SCANCODE_KP_DECIMAL: crate::stdlib::SDL_Scancode = 220;

    pub const SDL_SCANCODE_KP_HEXADECIMAL: crate::stdlib::SDL_Scancode = 221;

    pub const SDL_SCANCODE_LCTRL: crate::stdlib::SDL_Scancode = 224;

    pub const SDL_SCANCODE_LSHIFT: crate::stdlib::SDL_Scancode = 225;

    pub const SDL_SCANCODE_LALT: crate::stdlib::SDL_Scancode = 226;

    pub const SDL_SCANCODE_LGUI: crate::stdlib::SDL_Scancode = 227;

    pub const SDL_SCANCODE_RCTRL: crate::stdlib::SDL_Scancode = 228;

    pub const SDL_SCANCODE_RSHIFT: crate::stdlib::SDL_Scancode = 229;

    pub const SDL_SCANCODE_RALT: crate::stdlib::SDL_Scancode = 230;

    pub const SDL_SCANCODE_RGUI: crate::stdlib::SDL_Scancode = 231;

    pub const SDL_SCANCODE_MODE: crate::stdlib::SDL_Scancode = 257;

    pub const SDL_SCANCODE_AUDIONEXT: crate::stdlib::SDL_Scancode = 258;

    pub const SDL_SCANCODE_AUDIOPREV: crate::stdlib::SDL_Scancode = 259;

    pub const SDL_SCANCODE_AUDIOSTOP: crate::stdlib::SDL_Scancode = 260;

    pub const SDL_SCANCODE_AUDIOPLAY: crate::stdlib::SDL_Scancode = 261;

    pub const SDL_SCANCODE_AUDIOMUTE: crate::stdlib::SDL_Scancode = 262;

    pub const SDL_SCANCODE_MEDIASELECT: crate::stdlib::SDL_Scancode = 263;

    pub const SDL_SCANCODE_WWW: crate::stdlib::SDL_Scancode = 264;

    pub const SDL_SCANCODE_MAIL: crate::stdlib::SDL_Scancode = 265;

    pub const SDL_SCANCODE_CALCULATOR: crate::stdlib::SDL_Scancode = 266;

    pub const SDL_SCANCODE_COMPUTER: crate::stdlib::SDL_Scancode = 267;

    pub const SDL_SCANCODE_AC_SEARCH: crate::stdlib::SDL_Scancode = 268;

    pub const SDL_SCANCODE_AC_HOME: crate::stdlib::SDL_Scancode = 269;

    pub const SDL_SCANCODE_AC_BACK: crate::stdlib::SDL_Scancode = 270;

    pub const SDL_SCANCODE_AC_FORWARD: crate::stdlib::SDL_Scancode = 271;

    pub const SDL_SCANCODE_AC_STOP: crate::stdlib::SDL_Scancode = 272;

    pub const SDL_SCANCODE_AC_REFRESH: crate::stdlib::SDL_Scancode = 273;

    pub const SDL_SCANCODE_AC_BOOKMARKS: crate::stdlib::SDL_Scancode = 274;

    pub const SDL_SCANCODE_BRIGHTNESSDOWN: crate::stdlib::SDL_Scancode = 275;

    pub const SDL_SCANCODE_BRIGHTNESSUP: crate::stdlib::SDL_Scancode = 276;

    pub const SDL_SCANCODE_DISPLAYSWITCH: crate::stdlib::SDL_Scancode = 277;

    pub const SDL_SCANCODE_KBDILLUMTOGGLE: crate::stdlib::SDL_Scancode = 278;

    pub const SDL_SCANCODE_KBDILLUMDOWN: crate::stdlib::SDL_Scancode = 279;

    pub const SDL_SCANCODE_KBDILLUMUP: crate::stdlib::SDL_Scancode = 280;

    pub const SDL_SCANCODE_EJECT: crate::stdlib::SDL_Scancode = 281;

    pub const SDL_SCANCODE_SLEEP: crate::stdlib::SDL_Scancode = 282;

    pub const SDL_SCANCODE_APP1: crate::stdlib::SDL_Scancode = 283;

    pub const SDL_SCANCODE_APP2: crate::stdlib::SDL_Scancode = 284;

    pub const SDL_SCANCODE_AUDIOREWIND: crate::stdlib::SDL_Scancode = 285;

    pub const SDL_SCANCODE_AUDIOFASTFORWARD: crate::stdlib::SDL_Scancode = 286;

    pub const SDL_NUM_SCANCODES: crate::stdlib::SDL_Scancode = 512;
    pub type SDL_bool = u32;

    pub const SDL_FALSE: crate::stdlib::SDL_bool = 0;

    pub const SDL_TRUE: crate::stdlib::SDL_bool = 1;

    pub type Uint8 = crate::stdlib::uint8_t;

    pub type Sint16 = crate::stdlib::int16_t;

    pub type Uint16 = crate::stdlib::uint16_t;

    pub type Sint32 = crate::stdlib::int32_t;

    pub type Uint32 = crate::stdlib::uint32_t;

    pub type Sint64 = crate::stdlib::int64_t;
    pub type SDL_TouchID = crate::stdlib::Sint64;

    pub type SDL_FingerID = crate::stdlib::Sint64;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SDL_version {
        pub major: crate::stdlib::Uint8,
        pub minor: crate::stdlib::Uint8,
        pub patch: crate::stdlib::Uint8,
    }
    pub const SDL_WINDOW_FULLSCREEN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const SDL_WINDOW_OPENGL: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const SDL_WINDOW_SHOWN: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const SDL_WINDOW_HIDDEN: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const SDL_WINDOW_BORDERLESS: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const SDL_WINDOW_RESIZABLE: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const SDL_WINDOW_MINIMIZED: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const SDL_WINDOW_MAXIMIZED: crate::be_aas_h::C2RustUnnamed_0 = 128;

    pub const SDL_WINDOW_INPUT_GRABBED: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const SDL_WINDOW_INPUT_FOCUS: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const SDL_WINDOW_MOUSE_FOCUS: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const SDL_WINDOW_FULLSCREEN_DESKTOP: crate::be_aas_h::C2RustUnnamed_0 = 4097;

    pub const SDL_WINDOW_FOREIGN: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const SDL_WINDOW_ALLOW_HIGHDPI: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const SDL_WINDOW_MOUSE_CAPTURE: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const SDL_WINDOW_ALWAYS_ON_TOP: crate::be_aas_h::C2RustUnnamed_0 = 32768;

    pub const SDL_WINDOW_SKIP_TASKBAR: crate::be_aas_h::C2RustUnnamed_0 = 65536;

    pub const SDL_WINDOW_UTILITY: crate::be_aas_h::C2RustUnnamed_0 = 131072;

    pub const SDL_WINDOW_TOOLTIP: crate::be_aas_h::C2RustUnnamed_0 = 262144;

    pub const SDL_WINDOW_POPUP_MENU: crate::be_aas_h::C2RustUnnamed_0 = 524288;

    pub const SDL_WINDOW_VULKAN: crate::be_aas_h::C2RustUnnamed_0 = 268435456;

    pub const SDL_WINDOWEVENT_NONE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const SDL_WINDOWEVENT_SHOWN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const SDL_WINDOWEVENT_HIDDEN: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const SDL_WINDOWEVENT_EXPOSED: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const SDL_WINDOWEVENT_MOVED: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const SDL_WINDOWEVENT_RESIZED: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const SDL_WINDOWEVENT_SIZE_CHANGED: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const SDL_WINDOWEVENT_MINIMIZED: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const SDL_WINDOWEVENT_MAXIMIZED: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const SDL_WINDOWEVENT_RESTORED: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const SDL_WINDOWEVENT_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const SDL_WINDOWEVENT_LEAVE: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const SDL_WINDOWEVENT_FOCUS_GAINED: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const SDL_WINDOWEVENT_FOCUS_LOST: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const SDL_WINDOWEVENT_CLOSE: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const SDL_WINDOWEVENT_TAKE_FOCUS: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const SDL_WINDOWEVENT_HIT_TEST: crate::be_aas_h::C2RustUnnamed_0 = 16;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __mbstate_t {
        pub __count: i32,
        pub __value: crate::stdlib::C2RustUnnamed_18,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_18 {
        pub __wch: u32,
        pub __wchb: [libc::c_char; 4],
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __sigset_t {
        pub __val: [usize; 16],
    }
    pub type clock_t = crate::stdlib::__clock_t;
    pub const _ISupper: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const _ISlower: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const _ISalpha: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const _ISdigit: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const _ISxdigit: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const _ISspace: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const _ISprint: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const _ISgraph: crate::be_aas_h::C2RustUnnamed_0 = 32768;

    pub const _ISblank: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const _IScntrl: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const _ISpunct: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const _ISalnum: crate::be_aas_h::C2RustUnnamed_0 = 8;
    pub const IFF_UP: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const IFF_BROADCAST: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const IFF_DEBUG: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const IFF_LOOPBACK: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const IFF_POINTOPOINT: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const IFF_NOTRAILERS: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const IFF_RUNNING: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const IFF_NOARP: crate::be_aas_h::C2RustUnnamed_0 = 128;

    pub const IFF_PROMISC: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const IFF_ALLMULTI: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const IFF_MASTER: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const IFF_SLAVE: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const IFF_MULTICAST: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const IFF_PORTSEL: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const IFF_AUTOMEDIA: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const IFF_DYNAMIC: crate::be_aas_h::C2RustUnnamed_0 = 32768;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ifaddrs {
        pub ifa_next: *mut crate::stdlib::ifaddrs,
        pub ifa_name: *mut libc::c_char,
        pub ifa_flags: u32,
        pub ifa_addr: *mut ::libc::sockaddr,
        pub ifa_netmask: *mut ::libc::sockaddr,
        pub ifa_ifu: crate::stdlib::C2RustUnnamed_131,
        pub ifa_data: *mut libc::c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_131 {
        pub ifu_broadaddr: *mut ::libc::sockaddr,
        pub ifu_dstaddr: *mut ::libc::sockaddr,
    }
    pub type in_addr_t = crate::stdlib::uint32_t;

    pub const IPPROTO_IP: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const IPPROTO_ICMP: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const IPPROTO_IGMP: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const IPPROTO_IPIP: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const IPPROTO_TCP: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const IPPROTO_EGP: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const IPPROTO_PUP: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const IPPROTO_UDP: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const IPPROTO_IDP: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const IPPROTO_TP: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const IPPROTO_DCCP: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const IPPROTO_IPV6: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const IPPROTO_RSVP: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const IPPROTO_GRE: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const IPPROTO_ESP: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const IPPROTO_AH: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const IPPROTO_MTP: crate::be_aas_h::C2RustUnnamed_0 = 92;

    pub const IPPROTO_BEETPH: crate::be_aas_h::C2RustUnnamed_0 = 94;

    pub const IPPROTO_ENCAP: crate::be_aas_h::C2RustUnnamed_0 = 98;

    pub const IPPROTO_PIM: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const IPPROTO_COMP: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const IPPROTO_SCTP: crate::be_aas_h::C2RustUnnamed_0 = 132;

    pub const IPPROTO_UDPLITE: crate::be_aas_h::C2RustUnnamed_0 = 136;

    pub const IPPROTO_MPLS: crate::be_aas_h::C2RustUnnamed_0 = 137;

    pub const IPPROTO_RAW: crate::be_aas_h::C2RustUnnamed_0 = 255;

    pub const IPPROTO_MAX: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub type in_port_t = crate::stdlib::uint16_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub __in6_u: crate::stdlib::C2RustUnnamed_129,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_129 {
        pub __u6_addr8: [crate::stdlib::uint8_t; 16],
        pub __u6_addr16: [crate::stdlib::uint16_t; 8],
        pub __u6_addr32: [crate::stdlib::uint32_t; 4],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: crate::stdlib::sa_family_t,
        pub sin6_port: crate::stdlib::in_port_t,
        pub sin6_flowinfo: crate::stdlib::uint32_t,
        pub sin6_addr: crate::stdlib::in6_addr,
        pub sin6_scope_id: crate::stdlib::uint32_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: crate::stdlib::in6_addr,
        pub ipv6mr_interface: u32,
    }
    pub type DIR = crate::stdlib::__dirstream;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: crate::stdlib::__jmp_buf,
        pub __mask_was_saved: i32,
        pub __saved_mask: crate::stdlib::__sigset_t,
    }

    pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
    pub type mbstate_t = crate::stdlib::__mbstate_t;
    pub type mode_t = crate::stdlib::__mode_t;
    pub type __fd_mask = isize;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct fd_set {
        pub __fds_bits: [crate::stdlib::__fd_mask; 16],
    }
    pub type __jmp_buf = [isize; 8];
    pub type __sighandler_t = Option<unsafe extern "C" fn(_: i32) -> ()>;
    pub type sa_family_t = u16;
    pub type socklen_t = crate::stdlib::__socklen_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_storage {
        pub ss_family: crate::stdlib::sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: usize,
    }
    pub type __socket_type = u32;

    pub const SOCK_STREAM: crate::stdlib::__socket_type = 1;

    pub const SOCK_DGRAM: crate::stdlib::__socket_type = 2;

    pub const SOCK_RAW: crate::stdlib::__socket_type = 3;

    pub const SOCK_RDM: crate::stdlib::__socket_type = 4;

    pub const SOCK_SEQPACKET: crate::stdlib::__socket_type = 5;

    pub const SOCK_DCCP: crate::stdlib::__socket_type = 6;

    pub const SOCK_PACKET: crate::stdlib::__socket_type = 10;

    pub const SOCK_CLOEXEC: crate::stdlib::__socket_type = 524288;

    pub const SOCK_NONBLOCK: crate::stdlib::__socket_type = 2048;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct stat {
        pub st_dev: crate::stdlib::__dev_t,
        pub st_ino: crate::stdlib::__ino_t,
        pub st_nlink: crate::stdlib::__nlink_t,
        pub st_mode: crate::stdlib::__mode_t,
        pub st_uid: crate::stdlib::__uid_t,
        pub st_gid: crate::stdlib::__gid_t,
        pub __pad0: i32,
        pub st_rdev: crate::stdlib::__dev_t,
        pub st_size: crate::stdlib::__off_t,
        pub st_blksize: crate::stdlib::__blksize_t,
        pub st_blocks: crate::stdlib::__blkcnt_t,
        pub st_atim: ::libc::timespec,
        pub st_mtim: ::libc::timespec,
        pub st_ctim: ::libc::timespec,
        pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
    }
    pub type intptr_t = isize;
    pub type int16_t = crate::stdlib::__int16_t;

    pub type int32_t = crate::stdlib::__int32_t;

    pub type int64_t = crate::stdlib::__int64_t;
    pub type uint8_t = crate::stdlib::__uint8_t;

    pub type uint16_t = crate::stdlib::__uint16_t;

    pub type uint32_t = crate::stdlib::__uint32_t;
    pub type ssize_t = crate::stdlib::__ssize_t;
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
    pub type _IO_lock_t = ();

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_FILE {
        pub _flags: i32,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: i32,
        pub _flags2: i32,
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: u16,
        pub _vtable_offset: i8,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: crate::stddef_h::size_t,
        pub _mode: i32,
        pub _unused2: [libc::c_char; 20],
    }
    pub type off_t = crate::stdlib::__off64_t;

    pub type pid_t = crate::stdlib::__pid_t;
    pub type cc_t = u8;

    pub type speed_t = u32;

    pub type tcflag_t = u32;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct timezone {
        pub tz_minuteswest: i32,
        pub tz_dsttime: i32,
    }

    pub type __timezone_ptr_t = *mut crate::stdlib::timezone;
    pub type time_t = crate::stdlib::__time_t;
    pub type __uint8_t = u8;

    pub type __int16_t = i16;

    pub type __uint16_t = u16;

    pub type __int32_t = i32;

    pub type __uint32_t = u32;

    pub type __int64_t = isize;

    pub type __dev_t = usize;

    pub type __uid_t = u32;

    pub type __gid_t = u32;

    pub type __ino_t = usize;

    pub type __mode_t = u32;

    pub type __nlink_t = usize;

    pub type __off_t = isize;

    pub type __off64_t = isize;

    pub type __pid_t = i32;

    pub type __clock_t = isize;

    pub type __time_t = isize;

    pub type __useconds_t = u32;

    pub type __suseconds_t = isize;

    pub type __blksize_t = isize;

    pub type __blkcnt_t = isize;

    pub type __ssize_t = isize;

    pub type __syscall_slong_t = isize;

    pub type __socklen_t = u32;
}
extern crate c2rust_asm_casts;
extern crate libc;

pub mod src {
    pub mod asm {
        pub mod ftola;
        pub mod snapvector;
    } // mod asm
    pub mod botlib {
        pub mod be_aas_bspq3;
        pub mod be_aas_cluster;
        pub mod be_aas_debug;
        pub mod be_aas_entity;
        pub mod be_aas_file;
        pub mod be_aas_main;
        pub mod be_aas_move;
        pub mod be_aas_optimize;
        pub mod be_aas_reach;
        pub mod be_aas_route;
        pub mod be_aas_routealt;
        pub mod be_aas_sample;
        pub mod be_ai_char;
        pub mod be_ai_chat;
        pub mod be_ai_gen;
        pub mod be_ai_goal;
        pub mod be_ai_move;
        pub mod be_ai_weap;
        pub mod be_ai_weight;
        pub mod be_ea;
        pub mod be_interface;
        pub mod l_crc;
        pub mod l_libvar;
        pub mod l_log;
        pub mod l_memory;
        pub mod l_precomp;
        pub mod l_script;
        pub mod l_struct;
    } // mod botlib
    pub mod client {
        pub mod cl_avi;
        pub mod cl_cgame;
        pub mod cl_cin;
        pub mod cl_console;
        pub mod cl_curl;
        pub mod cl_input;
        pub mod cl_keys;
        pub mod cl_main;
        pub mod cl_net_chan;
        pub mod cl_parse;
        pub mod cl_scrn;
        pub mod cl_ui;
        pub mod libmumblelink;
        pub mod qal;
        pub mod snd_adpcm;
        pub mod snd_altivec;
        pub mod snd_codec;
        pub mod snd_codec_ogg;
        pub mod snd_codec_opus;
        pub mod snd_codec_wav;
        pub mod snd_dma;
        pub mod snd_main;
        pub mod snd_mem;
        pub mod snd_mix;
        pub mod snd_openal;
        pub mod snd_wavelet;
    } // mod client
    pub mod libogg_1_3_3 {
        pub mod src {
            pub mod bitwise;
            pub mod framing;
        } // mod src
    } // mod libogg_1_3_3
    pub mod libvorbis_1_3_6 {
        pub mod lib {
            pub mod analysis;
            pub mod bitrate;
            pub mod block;
            pub mod codebook;
            pub mod envelope;
            pub mod floor0;
            pub mod floor1;
            pub mod info;
            pub mod lookup;
            pub mod lpc;
            pub mod lsp;
            pub mod mapping0;
            pub mod mdct;
            pub mod psy;
            pub mod registry;
            pub mod res0;
            pub mod sharedbook;
            pub mod smallft;
            pub mod synthesis;
            pub mod vorbisfile;
            pub mod window;
        } // mod lib
    } // mod libvorbis_1_3_6
    pub mod opus_1_2_1 {
        pub mod celt {
            pub mod bands;
            pub mod celt;
            pub mod celt_decoder;
            pub mod celt_encoder;
            pub mod celt_lpc;
            pub mod cwrs;
            pub mod entcode;
            pub mod entdec;
            pub mod entenc;
            pub mod kiss_fft;
            pub mod laplace;
            pub mod mathops;
            pub mod mdct;
            pub mod modes;
            pub mod pitch;
            pub mod quant_bands;
            pub mod rate;
            pub mod vq;
        } // mod celt
        pub mod silk {
            pub mod A2NLSF;
            pub mod CNG;
            pub mod HP_variable_cutoff;
            pub mod LPC_analysis_filter;
            pub mod LPC_fit;
            pub mod LPC_inv_pred_gain;
            pub mod LP_variable_cutoff;
            pub mod NLSF2A;
            pub mod NLSF_VQ;
            pub mod NLSF_VQ_weights_laroia;
            pub mod NLSF_decode;
            pub mod NLSF_del_dec_quant;
            pub mod NLSF_encode;
            pub mod NLSF_stabilize;
            pub mod NLSF_unpack;
            pub mod NSQ;
            pub mod NSQ_del_dec;
            pub mod PLC;
            pub mod VAD;
            pub mod VQ_WMat_EC;
            pub mod ana_filt_bank_1;
            pub mod biquad_alt;
            pub mod bwexpander;
            pub mod bwexpander_32;
            pub mod check_control_input;
            pub mod code_signs;
            pub mod control_SNR;
            pub mod control_audio_bandwidth;
            pub mod control_codec;
            pub mod debug;
            pub mod dec_API;
            pub mod decode_core;
            pub mod decode_frame;
            pub mod decode_indices;
            pub mod decode_parameters;
            pub mod decode_pitch;
            pub mod decode_pulses;
            pub mod decoder_set_fs;
            pub mod enc_API;
            pub mod encode_indices;
            pub mod encode_pulses;
            pub mod float {
                pub mod LPC_analysis_filter_FLP;
                pub mod LPC_inv_pred_gain_FLP;
                pub mod LTP_analysis_filter_FLP;
                pub mod LTP_scale_ctrl_FLP;
                pub mod apply_sine_window_FLP;
                pub mod autocorrelation_FLP;
                pub mod burg_modified_FLP;
                pub mod bwexpander_FLP;
                pub mod corrMatrix_FLP;
                pub mod encode_frame_FLP;
                pub mod energy_FLP;
                pub mod find_LPC_FLP;
                pub mod find_LTP_FLP;
                pub mod find_pitch_lags_FLP;
                pub mod find_pred_coefs_FLP;
                pub mod inner_product_FLP;
                pub mod k2a_FLP;
                pub mod noise_shape_analysis_FLP;
                pub mod pitch_analysis_core_FLP;
                pub mod process_gains_FLP;
                pub mod regularize_correlations_FLP;
                pub mod residual_energy_FLP;
                pub mod scale_copy_vector_FLP;
                pub mod scale_vector_FLP;
                pub mod schur_FLP;
                pub mod sort_FLP;
                pub mod warped_autocorrelation_FLP;
                pub mod wrappers_FLP;
            } // mod float
            pub mod gain_quant;
            pub mod init_decoder;
            pub mod init_encoder;
            pub mod inner_prod_aligned;
            pub mod interpolate;
            pub mod lin2log;
            pub mod log2lin;
            pub mod pitch_est_tables;
            pub mod process_NLSFs;
            pub mod quant_LTP_gains;
            pub mod resampler;
            pub mod resampler_down2;
            pub mod resampler_down2_3;
            pub mod resampler_private_AR2;
            pub mod resampler_private_IIR_FIR;
            pub mod resampler_private_down_FIR;
            pub mod resampler_private_up2_HQ;
            pub mod resampler_rom;
            pub mod shell_coder;
            pub mod sigm_Q15;
            pub mod sort;
            pub mod stereo_LR_to_MS;
            pub mod stereo_MS_to_LR;
            pub mod stereo_decode_pred;
            pub mod stereo_encode_pred;
            pub mod stereo_find_predictor;
            pub mod stereo_quant_pred;
            pub mod sum_sqr_shift;
            pub mod table_LSF_cos;
            pub mod tables_LTP;
            pub mod tables_NLSF_CB_NB_MB;
            pub mod tables_NLSF_CB_WB;
            pub mod tables_gain;
            pub mod tables_other;
            pub mod tables_pitch_lag;
            pub mod tables_pulses_per_block;
        } // mod silk
        pub mod src {
            pub mod analysis;
            pub mod mlp;
            pub mod mlp_data;
            pub mod opus;
            pub mod opus_decoder;
            pub mod opus_encoder;
            pub mod opus_multistream;
            pub mod opus_multistream_decoder;
            pub mod opus_multistream_encoder;
            pub mod repacketizer;
        } // mod src
    } // mod opus_1_2_1
    pub mod opusfile_0_9 {
        pub mod src {
            pub mod http;
            pub mod info;
            pub mod internal;
            pub mod opusfile;
            pub mod stream;
            pub mod wincerts;
        } // mod src
    } // mod opusfile_0_9
    pub mod qcommon {
        pub mod cm_load;
        pub mod cm_patch;
        pub mod cm_polylib;
        pub mod cm_test;
        pub mod cm_trace;
        pub mod cmd;
        pub mod common;
        pub mod cvar;
        pub mod files;
        pub mod huffman;
        pub mod ioapi;
        pub mod md4;
        pub mod md5;
        pub mod msg;
        pub mod net_chan;
        pub mod net_ip;
        pub mod puff;
        pub mod q_math;
        pub mod q_shared;
        pub mod unzip;
        pub mod vm;
        pub mod vm_interpreted;
        pub mod vm_x86;
    } // mod qcommon
    pub mod sdl {
        pub mod sdl_input;
        pub mod sdl_snd;
    } // mod sdl
    pub mod server {
        pub mod sv_bot;
        pub mod sv_ccmds;
        pub mod sv_client;
        pub mod sv_game;
        pub mod sv_init;
        pub mod sv_main;
        pub mod sv_net_chan;
        pub mod sv_snapshot;
        pub mod sv_world;
    } // mod server
    pub mod sys {
        pub mod con_log;
        pub mod con_tty;
        pub mod sys_autoupdater;
        pub mod sys_main;
        pub mod sys_unix;
    } // mod sys
    pub mod zlib {
        pub mod adler32;
        pub mod crc32;
        pub mod inffast;
        pub mod inflate;
        pub mod inftrees;
        pub mod zutil;
    } // mod zlib
} // mod src

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(src::sys::sys_main::main_0(
            (args.len() - 1) as i32,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}

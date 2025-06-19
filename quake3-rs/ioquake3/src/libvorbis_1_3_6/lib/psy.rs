// =============== BEGIN psy_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_info_psy {
    pub blockflag: i32,
    pub ath_adjatt: f32,
    pub ath_maxatt: f32,
    pub tone_masteratt: [f32; 3],
    pub tone_centerboost: f32,
    pub tone_decay: f32,
    pub tone_abs_limit: f32,
    pub toneatt: [f32; 17],
    pub noisemaskp: i32,
    pub noisemaxsupp: f32,
    pub noisewindowlo: f32,
    pub noisewindowhi: f32,
    pub noisewindowlomin: i32,
    pub noisewindowhimin: i32,
    pub noisewindowfixed: i32,
    pub noiseoff: [[f32; 17]; 3],
    pub noisecompand: [f32; 40],
    pub max_curve_dB: f32,
    pub normal_p: i32,
    pub normal_start: i32,
    pub normal_partition: i32,
    pub normal_thresh: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_info_psy_global {
    pub eighth_octave_lines: i32,
    pub preecho_thresh: [f32; 7],
    pub postecho_thresh: [f32; 7],
    pub stretch_penalty: f32,
    pub preecho_minenergy: f32,
    pub ampmax_att_per_sec: f32,
    pub coupling_pkHz: [i32; 15],
    pub coupling_pointlimit: [[i32; 15]; 2],
    pub coupling_prepointamp: [i32; 15],
    pub coupling_postpointamp: [i32; 15],
    pub sliding_lowpass: [[i32; 15]; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_psy_global {
    pub ampmax: f32,
    pub channels: i32,
    pub gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    pub coupling_pointlimit: [[i32; 3]; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_psy {
    pub n: i32,
    pub vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy,
    pub tonecurves: *mut *mut *mut f32,
    pub noiseoffset: *mut *mut f32,
    pub ath: *mut f32,
    pub octave: *mut isize,
    pub bark: *mut isize,
    pub firstoc: isize,
    pub shiftoc: isize,
    pub eighth_octave_lines: i32,
    pub total_octave_lines: i32,
    pub rate: isize,
    pub m_val: f32,
}
use ::libc;

pub mod scales_h {

    /* *******************************************************************
    *                                                                  *
    * THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
    * USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
    * GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
    * IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
    *                                                                  *
    * THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
    * by the Xiph.Org Foundation http://www.xiph.org/                  *
    *                                                                  *
    ********************************************************************

    function: linear scale -> dB, Bark and Mel scales

    ********************************************************************/
    /* 20log10(x) */
    #[inline]

    pub unsafe extern "C" fn unitnorm(mut x: f32) -> f32 {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = x;
        ix.i = ix.i & 0x80000000 as u32 | 0x3f800000 as u32;
        return ix.f;
    }

    /* Frequency to octave.  We arbitrarily declare 63.5 Hz to be octave
    0.0 */
    /* The bark scale equations are approximations, since the original
    table was somewhat hand rolled.  The below are chosen to have the
    best possible fit to the rolled tables, thus their somewhat odd
    appearance (these are more accurate and over a longer range than
    the oft-quoted bark equations found in the texts I have).  The
    approximations are valid from 0 - 30kHz (nyquist) or so.

    all f in Hz, z in Bark */
}

pub mod masking_h {

    pub static mut ATH: [f32; 88] = [
        -(51 as i32) as f32,
        -(52 as i32) as f32,
        -(53 as i32) as f32,
        -(54 as i32) as f32,
        -(55 as i32) as f32,
        -(56 as i32) as f32,
        -(57 as i32) as f32,
        -(58 as i32) as f32,
        -(59 as i32) as f32,
        -(60 as i32) as f32,
        -(61 as i32) as f32,
        -(62 as i32) as f32,
        -(63 as i32) as f32,
        -(64 as i32) as f32,
        -(65 as i32) as f32,
        -(66 as i32) as f32,
        -(67 as i32) as f32,
        -(68 as i32) as f32,
        -(69 as i32) as f32,
        -(70 as i32) as f32,
        -(71 as i32) as f32,
        -(72 as i32) as f32,
        -(73 as i32) as f32,
        -(74 as i32) as f32,
        -(75 as i32) as f32,
        -(76 as i32) as f32,
        -(77 as i32) as f32,
        -(78 as i32) as f32,
        -(80 as i32) as f32,
        -(81 as i32) as f32,
        -(82 as i32) as f32,
        -(83 as i32) as f32,
        -(84 as i32) as f32,
        -(85 as i32) as f32,
        -(86 as i32) as f32,
        -(87 as i32) as f32,
        -(88 as i32) as f32,
        -(88 as i32) as f32,
        -(89 as i32) as f32,
        -(89 as i32) as f32,
        -(90 as i32) as f32,
        -(91 as i32) as f32,
        -(91 as i32) as f32,
        -(92 as i32) as f32,
        -(93 as i32) as f32,
        -(94 as i32) as f32,
        -(95 as i32) as f32,
        -(96 as i32) as f32,
        -(96 as i32) as f32,
        -(97 as i32) as f32,
        -(98 as i32) as f32,
        -(98 as i32) as f32,
        -(99 as i32) as f32,
        -(99 as i32) as f32,
        -(100 as i32) as f32,
        -(100 as i32) as f32,
        -(101 as i32) as f32,
        -(102 as i32) as f32,
        -(103 as i32) as f32,
        -(104 as i32) as f32,
        -(106 as i32) as f32,
        -(107 as i32) as f32,
        -(107 as i32) as f32,
        -(107 as i32) as f32,
        -(107 as i32) as f32,
        -(105 as i32) as f32,
        -(103 as i32) as f32,
        -(102 as i32) as f32,
        -(101 as i32) as f32,
        -(99 as i32) as f32,
        -(98 as i32) as f32,
        -(96 as i32) as f32,
        -(95 as i32) as f32,
        -(95 as i32) as f32,
        -(96 as i32) as f32,
        -(97 as i32) as f32,
        -(96 as i32) as f32,
        -(95 as i32) as f32,
        -(93 as i32) as f32,
        -(90 as i32) as f32,
        -(80 as i32) as f32,
        -(70 as i32) as f32,
        -(50 as i32) as f32,
        -(40 as i32) as f32,
        -(30 as i32) as f32,
        -(30 as i32) as f32,
        -(30 as i32) as f32,
        -(30 as i32) as f32,
    ];
    /* masking tones from -50 to 0dB, 62.5 through 16kHz at half octaves
    test tones from -2 octaves to +5 octaves sampled at eighth octaves */
    /* (Vorbis 0dB, the loudest possible tone, is assumed to be ~100dB SPL
    for collection of these curves) */

    pub static mut tonemasks: [[[f32; 56]; 6]; 17] = [
        [
            [
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(65 as i32) as f32,
                -(73 as i32) as f32,
                -(69 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(75 as i32) as f32,
                -(79 as i32) as f32,
                -(79 as i32) as f32,
                -(80 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(53 as i32) as f32,
                -(61 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(76 as i32) as f32,
                -(76 as i32) as f32,
                -(72 as i32) as f32,
                -(73 as i32) as f32,
                -(75 as i32) as f32,
                -(76 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(38 as i32) as f32,
                -(40 as i32) as f32,
                -(42 as i32) as f32,
                -(46 as i32) as f32,
                -(48 as i32) as f32,
                -(53 as i32) as f32,
                -(55 as i32) as f32,
                -(62 as i32) as f32,
                -(65 as i32) as f32,
                -(58 as i32) as f32,
                -(56 as i32) as f32,
                -(56 as i32) as f32,
                -(61 as i32) as f32,
                -(60 as i32) as f32,
                -(65 as i32) as f32,
                -(67 as i32) as f32,
                -(69 as i32) as f32,
                -(71 as i32) as f32,
                -(77 as i32) as f32,
                -(77 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(106 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(25 as i32) as f32,
                -(26 as i32) as f32,
                -(27 as i32) as f32,
                -(29 as i32) as f32,
                -(32 as i32) as f32,
                -(38 as i32) as f32,
                -(48 as i32) as f32,
                -(52 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(52 as i32) as f32,
                -(54 as i32) as f32,
                -(60 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(69 as i32) as f32,
                -(73 as i32) as f32,
                -(73 as i32) as f32,
                -(76 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(85 as i32) as f32,
                -(86 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(16 as i32) as f32,
                -(17 as i32) as f32,
                -(19 as i32) as f32,
                -(20 as i32) as f32,
                -(22 as i32) as f32,
                -(26 as i32) as f32,
                -(28 as i32) as f32,
                -(31 as i32) as f32,
                -(40 as i32) as f32,
                -(47 as i32) as f32,
                -(39 as i32) as f32,
                -(39 as i32) as f32,
                -(40 as i32) as f32,
                -(42 as i32) as f32,
                -(43 as i32) as f32,
                -(47 as i32) as f32,
                -(51 as i32) as f32,
                -(57 as i32) as f32,
                -(52 as i32) as f32,
                -(55 as i32) as f32,
                -(55 as i32) as f32,
                -(60 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(70 as i32) as f32,
                -(67 as i32) as f32,
                -(69 as i32) as f32,
                -(72 as i32) as f32,
                -(73 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(83 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(94 as i32) as f32,
                -(98 as i32) as f32,
                -(104 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(8 as i32) as f32,
                -(10 as i32) as f32,
                -(11 as i32) as f32,
                -(15 as i32) as f32,
                -(19 as i32) as f32,
                -(25 as i32) as f32,
                -(30 as i32) as f32,
                -(34 as i32) as f32,
                -(31 as i32) as f32,
                -(30 as i32) as f32,
                -(31 as i32) as f32,
                -(29 as i32) as f32,
                -(32 as i32) as f32,
                -(35 as i32) as f32,
                -(42 as i32) as f32,
                -(48 as i32) as f32,
                -(42 as i32) as f32,
                -(44 as i32) as f32,
                -(46 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(51 as i32) as f32,
                -(52 as i32) as f32,
                -(59 as i32) as f32,
                -(54 as i32) as f32,
                -(55 as i32) as f32,
                -(55 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(72 as i32) as f32,
                -(73 as i32) as f32,
                -(76 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(88 as i32) as f32,
                -(90 as i32) as f32,
                -(94 as i32) as f32,
                -(98 as i32) as f32,
                -(101 as i32) as f32,
                -(106 as i32) as f32,
                -(110 as i32) as f32,
            ],
        ],
        [
            [
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(76 as i32) as f32,
                -(72 as i32) as f32,
                -(71 as i32) as f32,
                -(74 as i32) as f32,
                -(76 as i32) as f32,
                -(76 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(79 as i32) as f32,
                -(81 as i32) as f32,
                -(83 as i32) as f32,
                -(86 as i32) as f32,
                -(89 as i32) as f32,
                -(93 as i32) as f32,
                -(97 as i32) as f32,
                -(100 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(55 as i32) as f32,
                -(59 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(67 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(69 as i32) as f32,
                -(70 as i32) as f32,
                -(74 as i32) as f32,
                -(79 as i32) as f32,
                -(77 as i32) as f32,
                -(77 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(86 as i32) as f32,
                -(88 as i32) as f32,
                -(91 as i32) as f32,
                -(95 as i32) as f32,
                -(100 as i32) as f32,
                -(108 as i32) as f32,
                -(116 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(60 as i32) as f32,
                -(57 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(60 as i32) as f32,
                -(63 as i32) as f32,
                -(65 as i32) as f32,
                -(72 as i32) as f32,
                -(71 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(76 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(80 as i32) as f32,
                -(83 as i32) as f32,
                -(86 as i32) as f32,
                -(91 as i32) as f32,
                -(96 as i32) as f32,
                -(100 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(30 as i32) as f32,
                -(32 as i32) as f32,
                -(32 as i32) as f32,
                -(33 as i32) as f32,
                -(35 as i32) as f32,
                -(41 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(52 as i32) as f32,
                -(51 as i32) as f32,
                -(57 as i32) as f32,
                -(65 as i32) as f32,
                -(61 as i32) as f32,
                -(59 as i32) as f32,
                -(61 as i32) as f32,
                -(64 as i32) as f32,
                -(69 as i32) as f32,
                -(70 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(77 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(85 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(92 as i32) as f32,
                -(96 as i32) as f32,
                -(100 as i32) as f32,
                -(107 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(19 as i32) as f32,
                -(20 as i32) as f32,
                -(21 as i32) as f32,
                -(23 as i32) as f32,
                -(27 as i32) as f32,
                -(30 as i32) as f32,
                -(35 as i32) as f32,
                -(36 as i32) as f32,
                -(41 as i32) as f32,
                -(46 as i32) as f32,
                -(44 as i32) as f32,
                -(42 as i32) as f32,
                -(40 as i32) as f32,
                -(41 as i32) as f32,
                -(41 as i32) as f32,
                -(43 as i32) as f32,
                -(48 as i32) as f32,
                -(55 as i32) as f32,
                -(53 as i32) as f32,
                -(52 as i32) as f32,
                -(53 as i32) as f32,
                -(56 as i32) as f32,
                -(59 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(67 as i32) as f32,
                -(66 as i32) as f32,
                -(69 as i32) as f32,
                -(71 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(79 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(93 as i32) as f32,
                -(97 as i32) as f32,
                -(101 as i32) as f32,
                -(107 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(9 as i32) as f32,
                -(11 as i32) as f32,
                -(12 as i32) as f32,
                -(12 as i32) as f32,
                -(15 as i32) as f32,
                -(16 as i32) as f32,
                -(20 as i32) as f32,
                -(23 as i32) as f32,
                -(30 as i32) as f32,
                -(37 as i32) as f32,
                -(34 as i32) as f32,
                -(33 as i32) as f32,
                -(34 as i32) as f32,
                -(31 as i32) as f32,
                -(32 as i32) as f32,
                -(32 as i32) as f32,
                -(38 as i32) as f32,
                -(47 as i32) as f32,
                -(44 as i32) as f32,
                -(41 as i32) as f32,
                -(40 as i32) as f32,
                -(47 as i32) as f32,
                -(49 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(58 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(54 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(64 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(76 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(87 as i32) as f32,
                -(91 as i32) as f32,
                -(96 as i32) as f32,
                -(100 as i32) as f32,
                -(104 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(67 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(75 as i32) as f32,
                -(72 as i32) as f32,
                -(76 as i32) as f32,
                -(75 as i32) as f32,
                -(76 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(94 as i32) as f32,
                -(101 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(61 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(71 as i32) as f32,
                -(68 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(71 as i32) as f32,
                -(72 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(81 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(82 as i32) as f32,
                -(83 as i32) as f32,
                -(86 as i32) as f32,
                -(90 as i32) as f32,
                -(97 as i32) as f32,
                -(103 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(55 as i32) as f32,
                -(57 as i32) as f32,
                -(56 as i32) as f32,
                -(57 as i32) as f32,
                -(55 as i32) as f32,
                -(61 as i32) as f32,
                -(65 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(74 as i32) as f32,
                -(73 as i32) as f32,
                -(75 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(96 as i32) as f32,
                -(101 as i32) as f32,
                -(108 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(57 as i32) as f32,
                -(51 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(51 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(59 as i32) as f32,
                -(66 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(76 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(88 as i32) as f32,
                -(94 as i32) as f32,
                -(97 as i32) as f32,
                -(104 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(36 as i32) as f32,
                -(39 as i32) as f32,
                -(41 as i32) as f32,
                -(42 as i32) as f32,
                -(42 as i32) as f32,
                -(39 as i32) as f32,
                -(38 as i32) as f32,
                -(41 as i32) as f32,
                -(43 as i32) as f32,
                -(52 as i32) as f32,
                -(44 as i32) as f32,
                -(40 as i32) as f32,
                -(39 as i32) as f32,
                -(37 as i32) as f32,
                -(37 as i32) as f32,
                -(40 as i32) as f32,
                -(47 as i32) as f32,
                -(54 as i32) as f32,
                -(50 as i32) as f32,
                -(48 as i32) as f32,
                -(50 as i32) as f32,
                -(55 as i32) as f32,
                -(61 as i32) as f32,
                -(59 as i32) as f32,
                -(62 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(69 as i32) as f32,
                -(69 as i32) as f32,
                -(73 as i32) as f32,
                -(74 as i32) as f32,
                -(74 as i32) as f32,
                -(75 as i32) as f32,
                -(77 as i32) as f32,
                -(79 as i32) as f32,
                -(82 as i32) as f32,
                -(87 as i32) as f32,
                -(91 as i32) as f32,
                -(95 as i32) as f32,
                -(100 as i32) as f32,
                -(108 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(28 as i32) as f32,
                -(26 as i32) as f32,
                -(24 as i32) as f32,
                -(22 as i32) as f32,
                -(20 as i32) as f32,
                -(20 as i32) as f32,
                -(23 as i32) as f32,
                -(29 as i32) as f32,
                -(30 as i32) as f32,
                -(31 as i32) as f32,
                -(28 as i32) as f32,
                -(27 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(28 as i32) as f32,
                -(35 as i32) as f32,
                -(40 as i32) as f32,
                -(33 as i32) as f32,
                -(32 as i32) as f32,
                -(29 as i32) as f32,
                -(30 as i32) as f32,
                -(30 as i32) as f32,
                -(30 as i32) as f32,
                -(37 as i32) as f32,
                -(45 as i32) as f32,
                -(41 as i32) as f32,
                -(37 as i32) as f32,
                -(38 as i32) as f32,
                -(45 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(53 as i32) as f32,
                -(49 as i32) as f32,
                -(48 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(49 as i32) as f32,
                -(51 as i32) as f32,
                -(52 as i32) as f32,
                -(58 as i32) as f32,
                -(56 as i32) as f32,
                -(57 as i32) as f32,
                -(56 as i32) as f32,
                -(60 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(78 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(106 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(91 as i32) as f32,
                -(87 as i32) as f32,
                -(83 as i32) as f32,
                -(80 as i32) as f32,
                -(78 as i32) as f32,
                -(76 as i32) as f32,
                -(78 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(86 as i32) as f32,
                -(85 as i32) as f32,
                -(86 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(97 as i32) as f32,
                -(107 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(81 as i32) as f32,
                -(77 as i32) as f32,
                -(73 as i32) as f32,
                -(70 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(68 as i32) as f32,
                -(75 as i32) as f32,
                -(73 as i32) as f32,
                -(70 as i32) as f32,
                -(69 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(79 as i32) as f32,
                -(84 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(86 as i32) as f32,
                -(88 as i32) as f32,
                -(89 as i32) as f32,
                -(89 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(105 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(76 as i32) as f32,
                -(71 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(65 as i32) as f32,
                -(63 as i32) as f32,
                -(63 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(64 as i32) as f32,
                -(65 as i32) as f32,
                -(64 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(73 as i32) as f32,
                -(73 as i32) as f32,
                -(74 as i32) as f32,
                -(75 as i32) as f32,
                -(76 as i32) as f32,
                -(81 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(88 as i32) as f32,
                -(89 as i32) as f32,
                -(92 as i32) as f32,
                -(95 as i32) as f32,
                -(100 as i32) as f32,
                -(108 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(71 as i32) as f32,
                -(68 as i32) as f32,
                -(65 as i32) as f32,
                -(63 as i32) as f32,
                -(62 as i32) as f32,
                -(61 as i32) as f32,
                -(61 as i32) as f32,
                -(61 as i32) as f32,
                -(61 as i32) as f32,
                -(59 as i32) as f32,
                -(56 as i32) as f32,
                -(57 as i32) as f32,
                -(53 as i32) as f32,
                -(50 as i32) as f32,
                -(58 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(52 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(58 as i32) as f32,
                -(67 as i32) as f32,
                -(63 as i32) as f32,
                -(67 as i32) as f32,
                -(68 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(90 as i32) as f32,
                -(93 as i32) as f32,
                -(97 as i32) as f32,
                -(101 as i32) as f32,
                -(107 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(65 as i32) as f32,
                -(61 as i32) as f32,
                -(59 as i32) as f32,
                -(57 as i32) as f32,
                -(56 as i32) as f32,
                -(55 as i32) as f32,
                -(55 as i32) as f32,
                -(56 as i32) as f32,
                -(56 as i32) as f32,
                -(57 as i32) as f32,
                -(55 as i32) as f32,
                -(53 as i32) as f32,
                -(52 as i32) as f32,
                -(47 as i32) as f32,
                -(44 as i32) as f32,
                -(44 as i32) as f32,
                -(50 as i32) as f32,
                -(44 as i32) as f32,
                -(41 as i32) as f32,
                -(39 as i32) as f32,
                -(39 as i32) as f32,
                -(42 as i32) as f32,
                -(40 as i32) as f32,
                -(46 as i32) as f32,
                -(51 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(63 as i32) as f32,
                -(60 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(70 as i32) as f32,
                -(73 as i32) as f32,
                -(74 as i32) as f32,
                -(75 as i32) as f32,
                -(76 as i32) as f32,
                -(75 as i32) as f32,
                -(79 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(91 as i32) as f32,
                -(96 as i32) as f32,
                -(102 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(49 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(46 as i32) as f32,
                -(43 as i32) as f32,
                -(39 as i32) as f32,
                -(35 as i32) as f32,
                -(33 as i32) as f32,
                -(38 as i32) as f32,
                -(36 as i32) as f32,
                -(32 as i32) as f32,
                -(29 as i32) as f32,
                -(32 as i32) as f32,
                -(32 as i32) as f32,
                -(32 as i32) as f32,
                -(35 as i32) as f32,
                -(44 as i32) as f32,
                -(39 as i32) as f32,
                -(38 as i32) as f32,
                -(38 as i32) as f32,
                -(46 as i32) as f32,
                -(50 as i32) as f32,
                -(45 as i32) as f32,
                -(46 as i32) as f32,
                -(53 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(53 as i32) as f32,
                -(53 as i32) as f32,
                -(56 as i32) as f32,
                -(57 as i32) as f32,
                -(59 as i32) as f32,
                -(66 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(97 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(86 as i32) as f32,
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(75 as i32) as f32,
                -(79 as i32) as f32,
                -(80 as i32) as f32,
                -(79 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(88 as i32) as f32,
                -(95 as i32) as f32,
                -(103 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(88 as i32) as f32,
                -(83 as i32) as f32,
                -(79 as i32) as f32,
                -(78 as i32) as f32,
                -(75 as i32) as f32,
                -(71 as i32) as f32,
                -(67 as i32) as f32,
                -(68 as i32) as f32,
                -(73 as i32) as f32,
                -(73 as i32) as f32,
                -(72 as i32) as f32,
                -(73 as i32) as f32,
                -(75 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(107 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(105 as i32) as f32,
                -(101 as i32) as f32,
                -(96 as i32) as f32,
                -(90 as i32) as f32,
                -(86 as i32) as f32,
                -(81 as i32) as f32,
                -(77 as i32) as f32,
                -(73 as i32) as f32,
                -(69 as i32) as f32,
                -(66 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(66 as i32) as f32,
                -(64 as i32) as f32,
                -(62 as i32) as f32,
                -(65 as i32) as f32,
                -(66 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(76 as i32) as f32,
                -(81 as i32) as f32,
                -(80 as i32) as f32,
                -(84 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(102 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(103 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(88 as i32) as f32,
                -(83 as i32) as f32,
                -(79 as i32) as f32,
                -(74 as i32) as f32,
                -(70 as i32) as f32,
                -(66 as i32) as f32,
                -(59 as i32) as f32,
                -(53 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(55 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(58 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(72 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(80 as i32) as f32,
                -(83 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(107 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(70 as i32) as f32,
                -(66 as i32) as f32,
                -(62 as i32) as f32,
                -(56 as i32) as f32,
                -(48 as i32) as f32,
                -(44 as i32) as f32,
                -(48 as i32) as f32,
                -(46 as i32) as f32,
                -(46 as i32) as f32,
                -(43 as i32) as f32,
                -(46 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(58 as i32) as f32,
                -(58 as i32) as f32,
                -(59 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(61 as i32) as f32,
                -(61 as i32) as f32,
                -(65 as i32) as f32,
                -(64 as i32) as f32,
                -(65 as i32) as f32,
                -(68 as i32) as f32,
                -(70 as i32) as f32,
                -(74 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(86 as i32) as f32,
                -(95 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(70 as i32) as f32,
                -(65 as i32) as f32,
                -(61 as i32) as f32,
                -(55 as i32) as f32,
                -(49 as i32) as f32,
                -(39 as i32) as f32,
                -(33 as i32) as f32,
                -(40 as i32) as f32,
                -(35 as i32) as f32,
                -(32 as i32) as f32,
                -(38 as i32) as f32,
                -(40 as i32) as f32,
                -(33 as i32) as f32,
                -(35 as i32) as f32,
                -(37 as i32) as f32,
                -(46 as i32) as f32,
                -(41 as i32) as f32,
                -(45 as i32) as f32,
                -(44 as i32) as f32,
                -(46 as i32) as f32,
                -(42 as i32) as f32,
                -(45 as i32) as f32,
                -(46 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(55 as i32) as f32,
                -(57 as i32) as f32,
                -(62 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(70 as i32) as f32,
                -(76 as i32) as f32,
                -(81 as i32) as f32,
                -(90 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(98 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(82 as i32) as f32,
                -(83 as i32) as f32,
                -(80 as i32) as f32,
                -(78 as i32) as f32,
                -(84 as i32) as f32,
                -(79 as i32) as f32,
                -(80 as i32) as f32,
                -(83 as i32) as f32,
                -(87 as i32) as f32,
                -(89 as i32) as f32,
                -(91 as i32) as f32,
                -(93 as i32) as f32,
                -(99 as i32) as f32,
                -(106 as i32) as f32,
                -(117 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(98 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(70 as i32) as f32,
                -(68 as i32) as f32,
                -(74 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(85 as i32) as f32,
                -(87 as i32) as f32,
                -(92 as i32) as f32,
                -(89 as i32) as f32,
                -(91 as i32) as f32,
                -(95 as i32) as f32,
                -(100 as i32) as f32,
                -(106 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(98 as i32) as f32,
                -(90 as i32) as f32,
                -(83 as i32) as f32,
                -(75 as i32) as f32,
                -(71 as i32) as f32,
                -(63 as i32) as f32,
                -(64 as i32) as f32,
                -(67 as i32) as f32,
                -(62 as i32) as f32,
                -(64 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(73 as i32) as f32,
                -(77 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(90 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(104 as i32) as f32,
                -(109 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(103 as i32) as f32,
                -(96 as i32) as f32,
                -(88 as i32) as f32,
                -(81 as i32) as f32,
                -(75 as i32) as f32,
                -(68 as i32) as f32,
                -(58 as i32) as f32,
                -(54 as i32) as f32,
                -(56 as i32) as f32,
                -(54 as i32) as f32,
                -(56 as i32) as f32,
                -(56 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(74 as i32) as f32,
                -(69 as i32) as f32,
                -(72 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(87 as i32) as f32,
                -(93 as i32) as f32,
                -(96 as i32) as f32,
                -(99 as i32) as f32,
                -(104 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(102 as i32) as f32,
                -(96 as i32) as f32,
                -(91 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(74 as i32) as f32,
                -(68 as i32) as f32,
                -(60 as i32) as f32,
                -(51 as i32) as f32,
                -(46 as i32) as f32,
                -(48 as i32) as f32,
                -(46 as i32) as f32,
                -(43 as i32) as f32,
                -(45 as i32) as f32,
                -(47 as i32) as f32,
                -(47 as i32) as f32,
                -(49 as i32) as f32,
                -(48 as i32) as f32,
                -(56 as i32) as f32,
                -(53 as i32) as f32,
                -(55 as i32) as f32,
                -(58 as i32) as f32,
                -(57 as i32) as f32,
                -(63 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(66 as i32) as f32,
                -(64 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(84 as i32) as f32,
                -(86 as i32) as f32,
                -(89 as i32) as f32,
                -(91 as i32) as f32,
                -(93 as i32) as f32,
                -(94 as i32) as f32,
                -(101 as i32) as f32,
                -(109 as i32) as f32,
                -(118 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(88 as i32) as f32,
                -(83 as i32) as f32,
                -(78 as i32) as f32,
                -(73 as i32) as f32,
                -(68 as i32) as f32,
                -(60 as i32) as f32,
                -(53 as i32) as f32,
                -(44 as i32) as f32,
                -(35 as i32) as f32,
                -(38 as i32) as f32,
                -(38 as i32) as f32,
                -(34 as i32) as f32,
                -(34 as i32) as f32,
                -(36 as i32) as f32,
                -(40 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(51 as i32) as f32,
                -(45 as i32) as f32,
                -(46 as i32) as f32,
                -(47 as i32) as f32,
                -(46 as i32) as f32,
                -(54 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(51 as i32) as f32,
                -(54 as i32) as f32,
                -(57 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(64 as i32) as f32,
                -(65 as i32) as f32,
                -(68 as i32) as f32,
                -(77 as i32) as f32,
                -(82 as i32) as f32,
                -(87 as i32) as f32,
                -(95 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(102 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(87 as i32) as f32,
                -(83 as i32) as f32,
                -(78 as i32) as f32,
                -(75 as i32) as f32,
                -(82 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(92 as i32) as f32,
                -(95 as i32) as f32,
                -(98 as i32) as f32,
                -(101 as i32) as f32,
                -(105 as i32) as f32,
                -(109 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(106 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(86 as i32) as f32,
                -(81 as i32) as f32,
                -(78 as i32) as f32,
                -(74 as i32) as f32,
                -(69 as i32) as f32,
                -(74 as i32) as f32,
                -(74 as i32) as f32,
                -(76 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(86 as i32) as f32,
                -(89 as i32) as f32,
                -(92 as i32) as f32,
                -(97 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(103 as i32) as f32,
                -(107 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(106 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(87 as i32) as f32,
                -(83 as i32) as f32,
                -(80 as i32) as f32,
                -(75 as i32) as f32,
                -(69 as i32) as f32,
                -(60 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(70 as i32) as f32,
                -(74 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(87 as i32) as f32,
                -(93 as i32) as f32,
                -(96 as i32) as f32,
                -(99 as i32) as f32,
                -(103 as i32) as f32,
                -(107 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(89 as i32) as f32,
                -(85 as i32) as f32,
                -(82 as i32) as f32,
                -(78 as i32) as f32,
                -(71 as i32) as f32,
                -(62 as i32) as f32,
                -(55 as i32) as f32,
                -(58 as i32) as f32,
                -(58 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(55 as i32) as f32,
                -(59 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(70 as i32) as f32,
                -(66 as i32) as f32,
                -(66 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(72 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(84 as i32) as f32,
                -(84 as i32) as f32,
                -(84 as i32) as f32,
                -(88 as i32) as f32,
                -(91 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(98 as i32) as f32,
                -(102 as i32) as f32,
                -(103 as i32) as f32,
                -(106 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(94 as i32) as f32,
                -(90 as i32) as f32,
                -(87 as i32) as f32,
                -(82 as i32) as f32,
                -(79 as i32) as f32,
                -(73 as i32) as f32,
                -(67 as i32) as f32,
                -(58 as i32) as f32,
                -(47 as i32) as f32,
                -(50 as i32) as f32,
                -(45 as i32) as f32,
                -(41 as i32) as f32,
                -(45 as i32) as f32,
                -(48 as i32) as f32,
                -(44 as i32) as f32,
                -(44 as i32) as f32,
                -(49 as i32) as f32,
                -(54 as i32) as f32,
                -(51 as i32) as f32,
                -(48 as i32) as f32,
                -(47 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(51 as i32) as f32,
                -(57 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(63 as i32) as f32,
                -(69 as i32) as f32,
                -(70 as i32) as f32,
                -(69 as i32) as f32,
                -(71 as i32) as f32,
                -(74 as i32) as f32,
                -(78 as i32) as f32,
                -(82 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(101 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(101 as i32) as f32,
                -(97 as i32) as f32,
                -(93 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(77 as i32) as f32,
                -(72 as i32) as f32,
                -(65 as i32) as f32,
                -(56 as i32) as f32,
                -(48 as i32) as f32,
                -(37 as i32) as f32,
                -(40 as i32) as f32,
                -(36 as i32) as f32,
                -(34 as i32) as f32,
                -(40 as i32) as f32,
                -(50 as i32) as f32,
                -(47 as i32) as f32,
                -(38 as i32) as f32,
                -(41 as i32) as f32,
                -(47 as i32) as f32,
                -(38 as i32) as f32,
                -(35 as i32) as f32,
                -(39 as i32) as f32,
                -(38 as i32) as f32,
                -(43 as i32) as f32,
                -(40 as i32) as f32,
                -(45 as i32) as f32,
                -(50 as i32) as f32,
                -(45 as i32) as f32,
                -(44 as i32) as f32,
                -(47 as i32) as f32,
                -(50 as i32) as f32,
                -(55 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(52 as i32) as f32,
                -(66 as i32) as f32,
                -(70 as i32) as f32,
                -(76 as i32) as f32,
                -(82 as i32) as f32,
                -(90 as i32) as f32,
                -(97 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(86 as i32) as f32,
                -(79 as i32) as f32,
                -(76 as i32) as f32,
                -(83 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(87 as i32) as f32,
                -(89 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(102 as i32) as f32,
                -(107 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(86 as i32) as f32,
                -(79 as i32) as f32,
                -(71 as i32) as f32,
                -(77 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(79 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(92 as i32) as f32,
                -(93 as i32) as f32,
                -(92 as i32) as f32,
                -(98 as i32) as f32,
                -(101 as i32) as f32,
                -(108 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(93 as i32) as f32,
                -(87 as i32) as f32,
                -(78 as i32) as f32,
                -(68 as i32) as f32,
                -(65 as i32) as f32,
                -(66 as i32) as f32,
                -(62 as i32) as f32,
                -(65 as i32) as f32,
                -(67 as i32) as f32,
                -(70 as i32) as f32,
                -(73 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(82 as i32) as f32,
                -(82 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(91 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(102 as i32) as f32,
                -(106 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(82 as i32) as f32,
                -(74 as i32) as f32,
                -(62 as i32) as f32,
                -(57 as i32) as f32,
                -(58 as i32) as f32,
                -(56 as i32) as f32,
                -(51 as i32) as f32,
                -(52 as i32) as f32,
                -(52 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(58 as i32) as f32,
                -(66 as i32) as f32,
                -(59 as i32) as f32,
                -(60 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(69 as i32) as f32,
                -(73 as i32) as f32,
                -(79 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(88 as i32) as f32,
                -(92 as i32) as f32,
                -(98 as i32) as f32,
                -(105 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(102 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(84 as i32) as f32,
                -(79 as i32) as f32,
                -(69 as i32) as f32,
                -(57 as i32) as f32,
                -(47 as i32) as f32,
                -(52 as i32) as f32,
                -(47 as i32) as f32,
                -(44 as i32) as f32,
                -(45 as i32) as f32,
                -(50 as i32) as f32,
                -(52 as i32) as f32,
                -(42 as i32) as f32,
                -(42 as i32) as f32,
                -(53 as i32) as f32,
                -(43 as i32) as f32,
                -(43 as i32) as f32,
                -(48 as i32) as f32,
                -(51 as i32) as f32,
                -(56 as i32) as f32,
                -(55 as i32) as f32,
                -(52 as i32) as f32,
                -(57 as i32) as f32,
                -(59 as i32) as f32,
                -(61 as i32) as f32,
                -(62 as i32) as f32,
                -(67 as i32) as f32,
                -(71 as i32) as f32,
                -(78 as i32) as f32,
                -(83 as i32) as f32,
                -(86 as i32) as f32,
                -(94 as i32) as f32,
                -(98 as i32) as f32,
                -(103 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(84 as i32) as f32,
                -(78 as i32) as f32,
                -(70 as i32) as f32,
                -(61 as i32) as f32,
                -(51 as i32) as f32,
                -(41 as i32) as f32,
                -(40 as i32) as f32,
                -(38 as i32) as f32,
                -(40 as i32) as f32,
                -(46 as i32) as f32,
                -(52 as i32) as f32,
                -(51 as i32) as f32,
                -(41 as i32) as f32,
                -(40 as i32) as f32,
                -(46 as i32) as f32,
                -(40 as i32) as f32,
                -(38 as i32) as f32,
                -(38 as i32) as f32,
                -(41 as i32) as f32,
                -(46 as i32) as f32,
                -(41 as i32) as f32,
                -(46 as i32) as f32,
                -(47 as i32) as f32,
                -(43 as i32) as f32,
                -(43 as i32) as f32,
                -(45 as i32) as f32,
                -(41 as i32) as f32,
                -(45 as i32) as f32,
                -(56 as i32) as f32,
                -(67 as i32) as f32,
                -(68 as i32) as f32,
                -(83 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(102 as i32) as f32,
                -(107 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(109 as i32) as f32,
                -(105 as i32) as f32,
                -(101 as i32) as f32,
                -(96 as i32) as f32,
                -(91 as i32) as f32,
                -(84 as i32) as f32,
                -(77 as i32) as f32,
                -(82 as i32) as f32,
                -(82 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(94 as i32) as f32,
                -(100 as i32) as f32,
                -(106 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(106 as i32) as f32,
                -(103 as i32) as f32,
                -(98 as i32) as f32,
                -(92 as i32) as f32,
                -(85 as i32) as f32,
                -(80 as i32) as f32,
                -(71 as i32) as f32,
                -(75 as i32) as f32,
                -(72 as i32) as f32,
                -(76 as i32) as f32,
                -(80 as i32) as f32,
                -(84 as i32) as f32,
                -(86 as i32) as f32,
                -(89 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(107 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(104 as i32) as f32,
                -(101 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(88 as i32) as f32,
                -(84 as i32) as f32,
                -(80 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(63 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(69 as i32) as f32,
                -(73 as i32) as f32,
                -(77 as i32) as f32,
                -(83 as i32) as f32,
                -(83 as i32) as f32,
                -(86 as i32) as f32,
                -(91 as i32) as f32,
                -(98 as i32) as f32,
                -(104 as i32) as f32,
                -(111 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(104 as i32) as f32,
                -(101 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(90 as i32) as f32,
                -(84 as i32) as f32,
                -(74 as i32) as f32,
                -(57 as i32) as f32,
                -(58 as i32) as f32,
                -(52 as i32) as f32,
                -(55 as i32) as f32,
                -(54 as i32) as f32,
                -(50 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(52 as i32) as f32,
                -(63 as i32) as f32,
                -(62 as i32) as f32,
                -(69 as i32) as f32,
                -(76 as i32) as f32,
                -(77 as i32) as f32,
                -(78 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(82 as i32) as f32,
                -(88 as i32) as f32,
                -(94 as i32) as f32,
                -(100 as i32) as f32,
                -(106 as i32) as f32,
                -(111 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(106 as i32) as f32,
                -(102 as i32) as f32,
                -(98 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(83 as i32) as f32,
                -(78 as i32) as f32,
                -(70 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(49 as i32) as f32,
                -(47 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(44 as i32) as f32,
                -(55 as i32) as f32,
                -(46 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(48 as i32) as f32,
                -(54 as i32) as f32,
                -(49 as i32) as f32,
                -(49 as i32) as f32,
                -(58 as i32) as f32,
                -(62 as i32) as f32,
                -(71 as i32) as f32,
                -(81 as i32) as f32,
                -(87 as i32) as f32,
                -(92 as i32) as f32,
                -(97 as i32) as f32,
                -(102 as i32) as f32,
                -(108 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(106 as i32) as f32,
                -(102 as i32) as f32,
                -(98 as i32) as f32,
                -(95 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(83 as i32) as f32,
                -(78 as i32) as f32,
                -(70 as i32) as f32,
                -(45 as i32) as f32,
                -(43 as i32) as f32,
                -(41 as i32) as f32,
                -(47 as i32) as f32,
                -(50 as i32) as f32,
                -(51 as i32) as f32,
                -(50 as i32) as f32,
                -(49 as i32) as f32,
                -(45 as i32) as f32,
                -(47 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(41 as i32) as f32,
                -(39 as i32) as f32,
                -(43 as i32) as f32,
                -(38 as i32) as f32,
                -(37 as i32) as f32,
                -(40 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(50 as i32) as f32,
                -(58 as i32) as f32,
                -(65 as i32) as f32,
                -(73 as i32) as f32,
                -(79 as i32) as f32,
                -(85 as i32) as f32,
                -(92 as i32) as f32,
                -(97 as i32) as f32,
                -(101 as i32) as f32,
                -(105 as i32) as f32,
                -(109 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(100 as i32) as f32,
                -(95 as i32) as f32,
                -(87 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(93 as i32) as f32,
                -(100 as i32) as f32,
                -(107 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(101 as i32) as f32,
                -(95 as i32) as f32,
                -(88 as i32) as f32,
                -(83 as i32) as f32,
                -(76 as i32) as f32,
                -(73 as i32) as f32,
                -(72 as i32) as f32,
                -(79 as i32) as f32,
                -(84 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(100 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(104 as i32) as f32,
                -(98 as i32) as f32,
                -(92 as i32) as f32,
                -(87 as i32) as f32,
                -(81 as i32) as f32,
                -(70 as i32) as f32,
                -(65 as i32) as f32,
                -(62 as i32) as f32,
                -(67 as i32) as f32,
                -(71 as i32) as f32,
                -(74 as i32) as f32,
                -(80 as i32) as f32,
                -(85 as i32) as f32,
                -(91 as i32) as f32,
                -(95 as i32) as f32,
                -(99 as i32) as f32,
                -(103 as i32) as f32,
                -(108 as i32) as f32,
                -(111 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(103 as i32) as f32,
                -(97 as i32) as f32,
                -(90 as i32) as f32,
                -(85 as i32) as f32,
                -(76 as i32) as f32,
                -(60 as i32) as f32,
                -(56 as i32) as f32,
                -(54 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(61 as i32) as f32,
                -(56 as i32) as f32,
                -(63 as i32) as f32,
                -(65 as i32) as f32,
                -(73 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(75 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(86 as i32) as f32,
                -(87 as i32) as f32,
                -(88 as i32) as f32,
                -(91 as i32) as f32,
                -(94 as i32) as f32,
                -(98 as i32) as f32,
                -(103 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(86 as i32) as f32,
                -(81 as i32) as f32,
                -(79 as i32) as f32,
                -(70 as i32) as f32,
                -(57 as i32) as f32,
                -(51 as i32) as f32,
                -(47 as i32) as f32,
                -(51 as i32) as f32,
                -(58 as i32) as f32,
                -(60 as i32) as f32,
                -(56 as i32) as f32,
                -(53 as i32) as f32,
                -(50 as i32) as f32,
                -(58 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(53 as i32) as f32,
                -(55 as i32) as f32,
                -(64 as i32) as f32,
                -(69 as i32) as f32,
                -(71 as i32) as f32,
                -(85 as i32) as f32,
                -(82 as i32) as f32,
                -(78 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(95 as i32) as f32,
                -(102 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(100 as i32) as f32,
                -(97 as i32) as f32,
                -(92 as i32) as f32,
                -(85 as i32) as f32,
                -(83 as i32) as f32,
                -(79 as i32) as f32,
                -(72 as i32) as f32,
                -(49 as i32) as f32,
                -(40 as i32) as f32,
                -(43 as i32) as f32,
                -(43 as i32) as f32,
                -(54 as i32) as f32,
                -(56 as i32) as f32,
                -(51 as i32) as f32,
                -(50 as i32) as f32,
                -(40 as i32) as f32,
                -(43 as i32) as f32,
                -(38 as i32) as f32,
                -(36 as i32) as f32,
                -(35 as i32) as f32,
                -(37 as i32) as f32,
                -(38 as i32) as f32,
                -(37 as i32) as f32,
                -(44 as i32) as f32,
                -(54 as i32) as f32,
                -(60 as i32) as f32,
                -(57 as i32) as f32,
                -(60 as i32) as f32,
                -(70 as i32) as f32,
                -(75 as i32) as f32,
                -(84 as i32) as f32,
                -(92 as i32) as f32,
                -(103 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(102 as i32) as f32,
                -(95 as i32) as f32,
                -(89 as i32) as f32,
                -(82 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(90 as i32) as f32,
                -(92 as i32) as f32,
                -(99 as i32) as f32,
                -(107 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(101 as i32) as f32,
                -(95 as i32) as f32,
                -(89 as i32) as f32,
                -(83 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(78 as i32) as f32,
                -(85 as i32) as f32,
                -(88 as i32) as f32,
                -(88 as i32) as f32,
                -(90 as i32) as f32,
                -(92 as i32) as f32,
                -(98 as i32) as f32,
                -(105 as i32) as f32,
                -(111 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(109 as i32) as f32,
                -(103 as i32) as f32,
                -(97 as i32) as f32,
                -(93 as i32) as f32,
                -(87 as i32) as f32,
                -(81 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(67 as i32) as f32,
                -(75 as i32) as f32,
                -(73 as i32) as f32,
                -(76 as i32) as f32,
                -(79 as i32) as f32,
                -(81 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(89 as i32) as f32,
                -(97 as i32) as f32,
                -(103 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(107 as i32) as f32,
                -(100 as i32) as f32,
                -(94 as i32) as f32,
                -(88 as i32) as f32,
                -(83 as i32) as f32,
                -(75 as i32) as f32,
                -(63 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(63 as i32) as f32,
                -(66 as i32) as f32,
                -(60 as i32) as f32,
                -(62 as i32) as f32,
                -(67 as i32) as f32,
                -(67 as i32) as f32,
                -(77 as i32) as f32,
                -(76 as i32) as f32,
                -(81 as i32) as f32,
                -(88 as i32) as f32,
                -(86 as i32) as f32,
                -(92 as i32) as f32,
                -(96 as i32) as f32,
                -(102 as i32) as f32,
                -(109 as i32) as f32,
                -(116 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(98 as i32) as f32,
                -(92 as i32) as f32,
                -(86 as i32) as f32,
                -(81 as i32) as f32,
                -(73 as i32) as f32,
                -(56 as i32) as f32,
                -(52 as i32) as f32,
                -(47 as i32) as f32,
                -(55 as i32) as f32,
                -(60 as i32) as f32,
                -(58 as i32) as f32,
                -(52 as i32) as f32,
                -(51 as i32) as f32,
                -(45 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(61 as i32) as f32,
                -(71 as i32) as f32,
                -(70 as i32) as f32,
                -(69 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(96 as i32) as f32,
                -(104 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(103 as i32) as f32,
                -(96 as i32) as f32,
                -(90 as i32) as f32,
                -(86 as i32) as f32,
                -(78 as i32) as f32,
                -(70 as i32) as f32,
                -(51 as i32) as f32,
                -(42 as i32) as f32,
                -(47 as i32) as f32,
                -(48 as i32) as f32,
                -(55 as i32) as f32,
                -(54 as i32) as f32,
                -(54 as i32) as f32,
                -(53 as i32) as f32,
                -(42 as i32) as f32,
                -(35 as i32) as f32,
                -(28 as i32) as f32,
                -(33 as i32) as f32,
                -(38 as i32) as f32,
                -(37 as i32) as f32,
                -(44 as i32) as f32,
                -(47 as i32) as f32,
                -(49 as i32) as f32,
                -(54 as i32) as f32,
                -(63 as i32) as f32,
                -(68 as i32) as f32,
                -(78 as i32) as f32,
                -(82 as i32) as f32,
                -(89 as i32) as f32,
                -(94 as i32) as f32,
                -(99 as i32) as f32,
                -(104 as i32) as f32,
                -(109 as i32) as f32,
                -(114 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(90 as i32) as f32,
                -(79 as i32) as f32,
                -(85 as i32) as f32,
                -(81 as i32) as f32,
                -(82 as i32) as f32,
                -(82 as i32) as f32,
                -(89 as i32) as f32,
                -(94 as i32) as f32,
                -(99 as i32) as f32,
                -(103 as i32) as f32,
                -(109 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(97 as i32) as f32,
                -(85 as i32) as f32,
                -(72 as i32) as f32,
                -(74 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(76 as i32) as f32,
                -(85 as i32) as f32,
                -(91 as i32) as f32,
                -(93 as i32) as f32,
                -(97 as i32) as f32,
                -(103 as i32) as f32,
                -(109 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(112 as i32) as f32,
                -(93 as i32) as f32,
                -(81 as i32) as f32,
                -(68 as i32) as f32,
                -(62 as i32) as f32,
                -(60 as i32) as f32,
                -(60 as i32) as f32,
                -(57 as i32) as f32,
                -(63 as i32) as f32,
                -(70 as i32) as f32,
                -(77 as i32) as f32,
                -(82 as i32) as f32,
                -(90 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(104 as i32) as f32,
                -(109 as i32) as f32,
                -(113 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(113 as i32) as f32,
                -(100 as i32) as f32,
                -(93 as i32) as f32,
                -(84 as i32) as f32,
                -(63 as i32) as f32,
                -(58 as i32) as f32,
                -(48 as i32) as f32,
                -(53 as i32) as f32,
                -(54 as i32) as f32,
                -(52 as i32) as f32,
                -(52 as i32) as f32,
                -(57 as i32) as f32,
                -(64 as i32) as f32,
                -(66 as i32) as f32,
                -(76 as i32) as f32,
                -(83 as i32) as f32,
                -(81 as i32) as f32,
                -(85 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(95 as i32) as f32,
                -(98 as i32) as f32,
                -(101 as i32) as f32,
                -(103 as i32) as f32,
                -(106 as i32) as f32,
                -(108 as i32) as f32,
                -(111 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(105 as i32) as f32,
                -(95 as i32) as f32,
                -(86 as i32) as f32,
                -(74 as i32) as f32,
                -(53 as i32) as f32,
                -(50 as i32) as f32,
                -(38 as i32) as f32,
                -(43 as i32) as f32,
                -(49 as i32) as f32,
                -(43 as i32) as f32,
                -(42 as i32) as f32,
                -(39 as i32) as f32,
                -(39 as i32) as f32,
                -(46 as i32) as f32,
                -(52 as i32) as f32,
                -(57 as i32) as f32,
                -(56 as i32) as f32,
                -(72 as i32) as f32,
                -(69 as i32) as f32,
                -(74 as i32) as f32,
                -(81 as i32) as f32,
                -(87 as i32) as f32,
                -(92 as i32) as f32,
                -(94 as i32) as f32,
                -(97 as i32) as f32,
                -(99 as i32) as f32,
                -(102 as i32) as f32,
                -(105 as i32) as f32,
                -(108 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(108 as i32) as f32,
                -(99 as i32) as f32,
                -(90 as i32) as f32,
                -(76 as i32) as f32,
                -(66 as i32) as f32,
                -(45 as i32) as f32,
                -(43 as i32) as f32,
                -(41 as i32) as f32,
                -(44 as i32) as f32,
                -(47 as i32) as f32,
                -(43 as i32) as f32,
                -(47 as i32) as f32,
                -(40 as i32) as f32,
                -(30 as i32) as f32,
                -(31 as i32) as f32,
                -(31 as i32) as f32,
                -(39 as i32) as f32,
                -(33 as i32) as f32,
                -(40 as i32) as f32,
                -(41 as i32) as f32,
                -(43 as i32) as f32,
                -(53 as i32) as f32,
                -(59 as i32) as f32,
                -(70 as i32) as f32,
                -(73 as i32) as f32,
                -(77 as i32) as f32,
                -(79 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(87 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(91 as i32) as f32,
                -(76 as i32) as f32,
                -(75 as i32) as f32,
                -(85 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(104 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(91 as i32) as f32,
                -(70 as i32) as f32,
                -(70 as i32) as f32,
                -(75 as i32) as f32,
                -(86 as i32) as f32,
                -(89 as i32) as f32,
                -(94 as i32) as f32,
                -(98 as i32) as f32,
                -(101 as i32) as f32,
                -(106 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(95 as i32) as f32,
                -(80 as i32) as f32,
                -(60 as i32) as f32,
                -(65 as i32) as f32,
                -(64 as i32) as f32,
                -(74 as i32) as f32,
                -(83 as i32) as f32,
                -(88 as i32) as f32,
                -(91 as i32) as f32,
                -(95 as i32) as f32,
                -(99 as i32) as f32,
                -(103 as i32) as f32,
                -(107 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(95 as i32) as f32,
                -(80 as i32) as f32,
                -(58 as i32) as f32,
                -(55 as i32) as f32,
                -(49 as i32) as f32,
                -(66 as i32) as f32,
                -(68 as i32) as f32,
                -(71 as i32) as f32,
                -(78 as i32) as f32,
                -(78 as i32) as f32,
                -(80 as i32) as f32,
                -(88 as i32) as f32,
                -(85 as i32) as f32,
                -(89 as i32) as f32,
                -(97 as i32) as f32,
                -(100 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(95 as i32) as f32,
                -(80 as i32) as f32,
                -(53 as i32) as f32,
                -(52 as i32) as f32,
                -(41 as i32) as f32,
                -(59 as i32) as f32,
                -(59 as i32) as f32,
                -(49 as i32) as f32,
                -(58 as i32) as f32,
                -(56 as i32) as f32,
                -(63 as i32) as f32,
                -(86 as i32) as f32,
                -(79 as i32) as f32,
                -(90 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(103 as i32) as f32,
                -(107 as i32) as f32,
                -(112 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(97 as i32) as f32,
                -(91 as i32) as f32,
                -(73 as i32) as f32,
                -(45 as i32) as f32,
                -(40 as i32) as f32,
                -(33 as i32) as f32,
                -(53 as i32) as f32,
                -(61 as i32) as f32,
                -(49 as i32) as f32,
                -(54 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(60 as i32) as f32,
                -(52 as i32) as f32,
                -(67 as i32) as f32,
                -(74 as i32) as f32,
                -(81 as i32) as f32,
                -(92 as i32) as f32,
                -(96 as i32) as f32,
                -(100 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(113 as i32) as f32,
                -(106 as i32) as f32,
                -(99 as i32) as f32,
                -(92 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(88 as i32) as f32,
                -(97 as i32) as f32,
                -(106 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(116 as i32) as f32,
                -(109 as i32) as f32,
                -(102 as i32) as f32,
                -(95 as i32) as f32,
                -(89 as i32) as f32,
                -(74 as i32) as f32,
                -(72 as i32) as f32,
                -(88 as i32) as f32,
                -(87 as i32) as f32,
                -(95 as i32) as f32,
                -(102 as i32) as f32,
                -(109 as i32) as f32,
                -(116 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(116 as i32) as f32,
                -(109 as i32) as f32,
                -(102 as i32) as f32,
                -(95 as i32) as f32,
                -(89 as i32) as f32,
                -(75 as i32) as f32,
                -(66 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(78 as i32) as f32,
                -(86 as i32) as f32,
                -(87 as i32) as f32,
                -(90 as i32) as f32,
                -(96 as i32) as f32,
                -(105 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(115 as i32) as f32,
                -(108 as i32) as f32,
                -(101 as i32) as f32,
                -(94 as i32) as f32,
                -(88 as i32) as f32,
                -(66 as i32) as f32,
                -(56 as i32) as f32,
                -(61 as i32) as f32,
                -(70 as i32) as f32,
                -(65 as i32) as f32,
                -(78 as i32) as f32,
                -(72 as i32) as f32,
                -(83 as i32) as f32,
                -(84 as i32) as f32,
                -(93 as i32) as f32,
                -(98 as i32) as f32,
                -(105 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(105 as i32) as f32,
                -(95 as i32) as f32,
                -(89 as i32) as f32,
                -(82 as i32) as f32,
                -(57 as i32) as f32,
                -(52 as i32) as f32,
                -(52 as i32) as f32,
                -(59 as i32) as f32,
                -(56 as i32) as f32,
                -(59 as i32) as f32,
                -(58 as i32) as f32,
                -(69 as i32) as f32,
                -(67 as i32) as f32,
                -(88 as i32) as f32,
                -(82 as i32) as f32,
                -(82 as i32) as f32,
                -(89 as i32) as f32,
                -(94 as i32) as f32,
                -(100 as i32) as f32,
                -(108 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(101 as i32) as f32,
                -(96 as i32) as f32,
                -(90 as i32) as f32,
                -(83 as i32) as f32,
                -(77 as i32) as f32,
                -(54 as i32) as f32,
                -(43 as i32) as f32,
                -(38 as i32) as f32,
                -(50 as i32) as f32,
                -(48 as i32) as f32,
                -(52 as i32) as f32,
                -(48 as i32) as f32,
                -(42 as i32) as f32,
                -(42 as i32) as f32,
                -(51 as i32) as f32,
                -(52 as i32) as f32,
                -(53 as i32) as f32,
                -(59 as i32) as f32,
                -(65 as i32) as f32,
                -(71 as i32) as f32,
                -(78 as i32) as f32,
                -(85 as i32) as f32,
                -(95 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(120 as i32) as f32,
                -(105 as i32) as f32,
                -(86 as i32) as f32,
                -(68 as i32) as f32,
                -(78 as i32) as f32,
                -(79 as i32) as f32,
                -(90 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(120 as i32) as f32,
                -(105 as i32) as f32,
                -(86 as i32) as f32,
                -(66 as i32) as f32,
                -(73 as i32) as f32,
                -(77 as i32) as f32,
                -(88 as i32) as f32,
                -(96 as i32) as f32,
                -(105 as i32) as f32,
                -(115 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(120 as i32) as f32,
                -(105 as i32) as f32,
                -(92 as i32) as f32,
                -(80 as i32) as f32,
                -(61 as i32) as f32,
                -(64 as i32) as f32,
                -(68 as i32) as f32,
                -(80 as i32) as f32,
                -(87 as i32) as f32,
                -(92 as i32) as f32,
                -(100 as i32) as f32,
                -(110 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(120 as i32) as f32,
                -(104 as i32) as f32,
                -(91 as i32) as f32,
                -(79 as i32) as f32,
                -(52 as i32) as f32,
                -(60 as i32) as f32,
                -(54 as i32) as f32,
                -(64 as i32) as f32,
                -(69 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(82 as i32) as f32,
                -(84 as i32) as f32,
                -(85 as i32) as f32,
                -(87 as i32) as f32,
                -(88 as i32) as f32,
                -(90 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(118 as i32) as f32,
                -(100 as i32) as f32,
                -(87 as i32) as f32,
                -(77 as i32) as f32,
                -(49 as i32) as f32,
                -(50 as i32) as f32,
                -(44 as i32) as f32,
                -(58 as i32) as f32,
                -(61 as i32) as f32,
                -(61 as i32) as f32,
                -(67 as i32) as f32,
                -(65 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(65 as i32) as f32,
                -(68 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(115 as i32) as f32,
                -(98 as i32) as f32,
                -(84 as i32) as f32,
                -(62 as i32) as f32,
                -(49 as i32) as f32,
                -(44 as i32) as f32,
                -(38 as i32) as f32,
                -(46 as i32) as f32,
                -(49 as i32) as f32,
                -(49 as i32) as f32,
                -(46 as i32) as f32,
                -(39 as i32) as f32,
                -(37 as i32) as f32,
                -(39 as i32) as f32,
                -(40 as i32) as f32,
                -(42 as i32) as f32,
                -(43 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(88 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(82 as i32) as f32,
                -(82 as i32) as f32,
                -(85 as i32) as f32,
                -(90 as i32) as f32,
                -(94 as i32) as f32,
                -(99 as i32) as f32,
                -(104 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(88 as i32) as f32,
                -(66 as i32) as f32,
                -(70 as i32) as f32,
                -(81 as i32) as f32,
                -(80 as i32) as f32,
                -(81 as i32) as f32,
                -(84 as i32) as f32,
                -(88 as i32) as f32,
                -(91 as i32) as f32,
                -(93 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(88 as i32) as f32,
                -(61 as i32) as f32,
                -(63 as i32) as f32,
                -(70 as i32) as f32,
                -(71 as i32) as f32,
                -(74 as i32) as f32,
                -(77 as i32) as f32,
                -(80 as i32) as f32,
                -(83 as i32) as f32,
                -(85 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(86 as i32) as f32,
                -(62 as i32) as f32,
                -(63 as i32) as f32,
                -(62 as i32) as f32,
                -(62 as i32) as f32,
                -(58 as i32) as f32,
                -(52 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(52 as i32) as f32,
                -(54 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(118 as i32) as f32,
                -(108 as i32) as f32,
                -(84 as i32) as f32,
                -(53 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(50 as i32) as f32,
                -(55 as i32) as f32,
                -(47 as i32) as f32,
                -(45 as i32) as f32,
                -(40 as i32) as f32,
                -(40 as i32) as f32,
                -(40 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(118 as i32) as f32,
                -(100 as i32) as f32,
                -(73 as i32) as f32,
                -(43 as i32) as f32,
                -(37 as i32) as f32,
                -(42 as i32) as f32,
                -(43 as i32) as f32,
                -(53 as i32) as f32,
                -(38 as i32) as f32,
                -(37 as i32) as f32,
                -(35 as i32) as f32,
                -(35 as i32) as f32,
                -(38 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
        [
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(91 as i32) as f32,
                -(84 as i32) as f32,
                -(74 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(80 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(91 as i32) as f32,
                -(84 as i32) as f32,
                -(74 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(68 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(86 as i32) as f32,
                -(78 as i32) as f32,
                -(70 as i32) as f32,
                -(60 as i32) as f32,
                -(45 as i32) as f32,
                -(30 as i32) as f32,
                -(21 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(87 as i32) as f32,
                -(78 as i32) as f32,
                -(67 as i32) as f32,
                -(48 as i32) as f32,
                -(38 as i32) as f32,
                -(29 as i32) as f32,
                -(21 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(86 as i32) as f32,
                -(69 as i32) as f32,
                -(56 as i32) as f32,
                -(45 as i32) as f32,
                -(35 as i32) as f32,
                -(33 as i32) as f32,
                -(29 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
            [
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(110 as i32) as f32,
                -(100 as i32) as f32,
                -(83 as i32) as f32,
                -(71 as i32) as f32,
                -(48 as i32) as f32,
                -(27 as i32) as f32,
                -(38 as i32) as f32,
                -(37 as i32) as f32,
                -(34 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
                -(999 as i32) as f32,
            ],
        ],
    ];
}
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::calloc;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::malloc;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;
pub use ::libc::abs;
pub use ::libc::free;

pub use crate::backends_h::vorbis_info_mapping0;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;

pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;

pub use crate::scales_h::C2RustUnnamed_58;
pub use crate::src::libvorbis_1_3_6::lib::psy::masking_h::tonemasks;
pub use crate::src::libvorbis_1_3_6::lib::psy::masking_h::ATH;
pub use crate::src::libvorbis_1_3_6::lib::psy::scales_h::unitnorm;

static mut stereo_threshholds: [f64; 9] = [
    0.0f64, 0.5f64, 1.0f64, 1.5f64, 2.5f64, 4.5f64, 8.5f64, 16.5f64, 9e10f64,
];

static mut stereo_threshholds_limited: [f64; 9] = [
    0.0f64, 0.5f64, 1.0f64, 1.5f64, 2.0f64, 2.5f64, 4.5f64, 8.5f64, 9e10f64,
];
#[no_mangle]

pub unsafe extern "C" fn _vp_global_look(
    mut vi: *mut crate::codec_h::vorbis_info,
) -> *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut look: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global =
        crate::stdlib::calloc(
            1 as i32 as libc::c_ulong,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global>()
                as libc::c_ulong,
        ) as *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
    (*look).channels = (*vi).channels;
    (*look).ampmax = -9999.0f64 as f32;
    (*look).gi = gi;
    return look;
}
#[no_mangle]

pub unsafe extern "C" fn _vp_global_free(
    mut look: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global,
) {
    if !look.is_null() {
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global>()
                as libc::c_ulong,
        );
        ::libc::free(look as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn _vi_gpsy_free(
    mut i: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
) {
    if !i.is_null() {
        crate::stdlib::memset(
            i as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global>()
                as libc::c_ulong,
        );
        ::libc::free(i as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn _vi_psy_free(
    mut i: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy,
) {
    if !i.is_null() {
        crate::stdlib::memset(
            i as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy>()
                as libc::c_ulong,
        );
        ::libc::free(i as *mut libc::c_void);
    };
}

unsafe extern "C" fn min_curve(mut c: *mut f32, mut c2: *mut f32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 56 as i32 {
        if *c2.offset(i as isize) < *c.offset(i as isize) {
            *c.offset(i as isize) = *c2.offset(i as isize)
        }
        i += 1
    }
}

unsafe extern "C" fn max_curve(mut c: *mut f32, mut c2: *mut f32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 56 as i32 {
        if *c2.offset(i as isize) > *c.offset(i as isize) {
            *c.offset(i as isize) = *c2.offset(i as isize)
        }
        i += 1
    }
}

unsafe extern "C" fn attenuate_curve(mut c: *mut f32, mut att: f32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 56 as i32 {
        *c.offset(i as isize) += att;
        i += 1
    }
}

unsafe extern "C" fn setup_tone_curves(
    mut curveatt_dB: *mut f32,
    mut binHz: f32,
    mut n: i32,
    mut center_boost: f32,
    mut center_decay_rate: f32,
) -> *mut *mut *mut f32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut ath: [f32; 56] = [0.; 56];
    let mut workc: [[[f32; 56]; 8]; 17] = [[[0.; 56]; 8]; 17];
    let mut athc: [[f32; 56]; 8] = [[0.; 56]; 8];
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut brute_buffer: *mut f32 = fresh0.as_mut_ptr() as *mut f32;
    let mut ret: *mut *mut *mut f32 = crate::stdlib::malloc(
        (::std::mem::size_of::<*mut *mut f32>() as libc::c_ulong)
            .wrapping_mul(17 as i32 as libc::c_ulong),
    ) as *mut *mut *mut f32;
    crate::stdlib::memset(
        workc.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[[[f32; 56]; 8]; 17]>() as libc::c_ulong,
    );
    i = 0 as i32;
    while i < 17 as i32 {
        /* we add back in the ATH to avoid low level curves falling off to
        -infinity and unnecessarily cutting off high level curves in the
        curve limiting (last step). */
        /* A half-band's settings must be valid over the whole band, and
        it's better to mask too little than too much */
        let mut ath_offset: i32 = i * 4 as i32;
        j = 0 as i32;
        while j < 56 as i32 {
            let mut min: f32 = 999.0f64 as f32;
            k = 0 as i32;
            while k < 4 as i32 {
                if j + k + ath_offset < 88 as i32 {
                    if min > ATH[(j + k + ath_offset) as usize] {
                        min = ATH[(j + k + ath_offset) as usize]
                    }
                } else if min > ATH[(88 as i32 - 1 as i32) as usize] {
                    min = ATH[(88 as i32 - 1 as i32) as usize]
                }
                k += 1
            }
            ath[j as usize] = min;
            j += 1
        }
        /* copy curves into working space, replicate the 50dB curve to 30
        and 40, replicate the 100dB curve to 110 */
        j = 0 as i32;
        while j < 6 as i32 {
            crate::stdlib::memcpy(
                workc[i as usize][(j + 2 as i32) as usize].as_mut_ptr() as *mut libc::c_void,
                tonemasks[i as usize][j as usize].as_ptr() as *const libc::c_void,
                (56 as i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
            );
            j += 1
        }
        crate::stdlib::memcpy(
            workc[i as usize][0 as i32 as usize].as_mut_ptr() as *mut libc::c_void,
            tonemasks[i as usize][0 as i32 as usize].as_ptr() as *const libc::c_void,
            (56 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        );
        crate::stdlib::memcpy(
            workc[i as usize][1 as i32 as usize].as_mut_ptr() as *mut libc::c_void,
            tonemasks[i as usize][0 as i32 as usize].as_ptr() as *const libc::c_void,
            (56 as i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        );
        /* apply centered curve boost/decay */
        j = 0 as i32;
        while j < 8 as i32 {
            k = 0 as i32;
            while k < 56 as i32 {
                let mut adj: f32 =
                    center_boost + ::libc::abs(16 as i32 - k) as f32 * center_decay_rate;
                if (adj as f64) < 0.0f64 && center_boost > 0 as i32 as f32 {
                    adj = 0.0f64 as f32
                }
                if adj as f64 > 0.0f64 && center_boost < 0 as i32 as f32 {
                    adj = 0.0f64 as f32
                }
                workc[i as usize][j as usize][k as usize] += adj;
                k += 1
            }
            j += 1
        }
        /* normalize curves so the driving amplitude is 0dB */
        /* make temp curves with the ATH overlayed */
        j = 0 as i32;
        while j < 8 as i32 {
            attenuate_curve(
                workc[i as usize][j as usize].as_mut_ptr(),
                (*curveatt_dB.offset(i as isize) as f64 + 100.0f64
                    - (if j < 2 as i32 { 2 as i32 } else { j }) as f64 * 10.0f64
                    - 30.0f64) as f32,
            );
            crate::stdlib::memcpy(
                athc[j as usize].as_mut_ptr() as *mut libc::c_void,
                ath.as_mut_ptr() as *const libc::c_void,
                (56 as i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
            );
            attenuate_curve(
                athc[j as usize].as_mut_ptr(),
                (100.0f64 - (j as f32 * 10.0f32) as f64 - 30.0f64) as f32,
            );
            max_curve(
                athc[j as usize].as_mut_ptr(),
                workc[i as usize][j as usize].as_mut_ptr(),
            );
            j += 1
        }
        /* Now limit the louder curves.

        the idea is this: We don't know what the playback attenuation
        will be; 0dB SL moves every time the user twiddles the volume
        knob. So that means we have to use a single 'most pessimal' curve
        for all masking amplitudes, right?  Wrong.  The *loudest* sound
        can be in (we assume) a range of ...+100dB] SL.  However, sounds
        20dB down will be in a range ...+80], 40dB down is from ...+60],
        etc... */
        j = 1 as i32;
        while j < 8 as i32 {
            min_curve(
                athc[j as usize].as_mut_ptr(),
                athc[(j - 1 as i32) as usize].as_mut_ptr(),
            );
            min_curve(
                workc[i as usize][j as usize].as_mut_ptr(),
                athc[j as usize].as_mut_ptr(),
            );
            j += 1
        }
        i += 1
    }
    i = 0 as i32;
    while i < 17 as i32 {
        let mut hi_curve: i32 = 0;
        let mut lo_curve: i32 = 0;
        let mut bin: i32 = 0;
        let ref mut fresh1 = *ret.offset(i as isize);
        *fresh1 = crate::stdlib::malloc(
            (::std::mem::size_of::<*mut f32>() as libc::c_ulong)
                .wrapping_mul(8 as i32 as libc::c_ulong),
        ) as *mut *mut f32;
        /* low frequency curves are measured with greater resolution than
        the MDCT/FFT will actually give us; we want the curve applied
        to the tone data to be pessimistic and thus apply the minimum
        masking possible for a given bin.  That means that a single bin
        could span more than one octave and that the curve will be a
        composite of multiple octaves.  It also may mean that a single
        bin may span > an eighth of an octave and that the eighth
        octave values may also be composited. */
        /* which octave curves will we be compositing? */
        bin = crate::stdlib::floor(
            crate::stdlib::exp((i as f64 * 0.5f64 + 5.965784f32 as f64) * 0.693147f32 as f64)
                / binHz as f64,
        ) as i32;
        lo_curve = crate::stdlib::ceil(
            (crate::stdlib::log((bin as f32 * binHz + 1 as i32 as f32) as f64)
                * 1.442695f32 as f64
                - 5.965784f32 as f64)
                * 2 as i32 as f64,
        ) as i32;
        hi_curve = crate::stdlib::floor(
            (crate::stdlib::log(((bin + 1 as i32) as f32 * binHz) as f64) * 1.442695f32 as f64
                - 5.965784f32 as f64)
                * 2 as i32 as f64,
        ) as i32;
        if lo_curve > i {
            lo_curve = i
        }
        if lo_curve < 0 as i32 {
            lo_curve = 0 as i32
        }
        if hi_curve >= 17 as i32 {
            hi_curve = 17 as i32 - 1 as i32
        }
        m = 0 as i32;
        while m < 8 as i32 {
            let ref mut fresh2 = *(*ret.offset(i as isize)).offset(m as isize);
            *fresh2 = crate::stdlib::malloc(
                (::std::mem::size_of::<f32>() as libc::c_ulong)
                    .wrapping_mul((56 as i32 + 2 as i32) as libc::c_ulong),
            ) as *mut f32;
            j = 0 as i32;
            while j < n {
                *brute_buffer.offset(j as isize) = 999.0f64 as f32;
                j += 1
            }
            /* render the curve into bins, then pull values back into curve.
            The point is that any inherent subsampling aliasing results in
            a safe minimum */
            k = lo_curve;
            while k <= hi_curve {
                let mut l: i32 = 0 as i32;
                j = 0 as i32;
                while j < 56 as i32 {
                    let mut lo_bin: i32 = (crate::stdlib::exp(
                        (j as f64 * 0.125f64 + k as f64 * 0.5f64 - 2.0625f64 + 5.965784f32 as f64)
                            * 0.693147f32 as f64,
                    ) / binHz as f64) as i32;
                    let mut hi_bin: i32 = (crate::stdlib::exp(
                        (j as f64 * 0.125f64 + k as f64 * 0.5f64 - 1.9375f64 + 5.965784f32 as f64)
                            * 0.693147f32 as f64,
                    ) / binHz as f64
                        + 1 as i32 as f64) as i32;
                    if lo_bin < 0 as i32 {
                        lo_bin = 0 as i32
                    }
                    if lo_bin > n {
                        lo_bin = n
                    }
                    if lo_bin < l {
                        l = lo_bin
                    }
                    if hi_bin < 0 as i32 {
                        hi_bin = 0 as i32
                    }
                    if hi_bin > n {
                        hi_bin = n
                    }
                    while l < hi_bin && l < n {
                        if *brute_buffer.offset(l as isize)
                            > workc[k as usize][m as usize][j as usize]
                        {
                            *brute_buffer.offset(l as isize) =
                                workc[k as usize][m as usize][j as usize]
                        }
                        l += 1
                    }
                    j += 1
                }
                while l < n {
                    if *brute_buffer.offset(l as isize)
                        > workc[k as usize][m as usize][(56 as i32 - 1 as i32) as usize]
                    {
                        *brute_buffer.offset(l as isize) =
                            workc[k as usize][m as usize][(56 as i32 - 1 as i32) as usize]
                    }
                    l += 1
                }
                k += 1
            }
            /* be equally paranoid about being valid up to next half ocatve */
            if (i + 1 as i32) < 17 as i32 {
                let mut l_0: i32 = 0 as i32;
                k = i + 1 as i32;
                j = 0 as i32;
                while j < 56 as i32 {
                    let mut lo_bin_0: i32 = (crate::stdlib::exp(
                        (j as f64 * 0.125f64 + i as f64 * 0.5f64 - 2.0625f64 + 5.965784f32 as f64)
                            * 0.693147f32 as f64,
                    ) / binHz as f64) as i32;
                    let mut hi_bin_0: i32 = (crate::stdlib::exp(
                        (j as f64 * 0.125f64 + i as f64 * 0.5f64 - 1.9375f64 + 5.965784f32 as f64)
                            * 0.693147f32 as f64,
                    ) / binHz as f64
                        + 1 as i32 as f64) as i32;
                    if lo_bin_0 < 0 as i32 {
                        lo_bin_0 = 0 as i32
                    }
                    if lo_bin_0 > n {
                        lo_bin_0 = n
                    }
                    if lo_bin_0 < l_0 {
                        l_0 = lo_bin_0
                    }
                    if hi_bin_0 < 0 as i32 {
                        hi_bin_0 = 0 as i32
                    }
                    if hi_bin_0 > n {
                        hi_bin_0 = n
                    }
                    while l_0 < hi_bin_0 && l_0 < n {
                        if *brute_buffer.offset(l_0 as isize)
                            > workc[k as usize][m as usize][j as usize]
                        {
                            *brute_buffer.offset(l_0 as isize) =
                                workc[k as usize][m as usize][j as usize]
                        }
                        l_0 += 1
                    }
                    j += 1
                }
                while l_0 < n {
                    if *brute_buffer.offset(l_0 as isize)
                        > workc[k as usize][m as usize][(56 as i32 - 1 as i32) as usize]
                    {
                        *brute_buffer.offset(l_0 as isize) =
                            workc[k as usize][m as usize][(56 as i32 - 1 as i32) as usize]
                    }
                    l_0 += 1
                }
            }
            j = 0 as i32;
            while j < 56 as i32 {
                let mut bin_0: i32 = (crate::stdlib::exp(
                    (j as f64 * 0.125f64 + i as f64 * 0.5f64 - 2.0f64 + 5.965784f32 as f64)
                        * 0.693147f32 as f64,
                ) / binHz as f64) as i32;
                if bin_0 < 0 as i32 {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as i32) as isize) = -999.0f64 as f32
                } else if bin_0 >= n {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as i32) as isize) = -999.0f64 as f32
                } else {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as i32) as isize) = *brute_buffer.offset(bin_0 as isize)
                }
                j += 1
            }
            /* add fenceposts */
            j = 0 as i32;
            while j < 16 as i32 {
                if *(*(*ret.offset(i as isize)).offset(m as isize)).offset((j + 2 as i32) as isize)
                    > -200.0f32
                {
                    break;
                }
                j += 1
            }
            *(*(*ret.offset(i as isize)).offset(m as isize)).offset(0 as i32 as isize) = j as f32;
            j = 56 as i32 - 1 as i32;
            while j > 16 as i32 + 1 as i32 {
                if *(*(*ret.offset(i as isize)).offset(m as isize)).offset((j + 2 as i32) as isize)
                    > -200.0f32
                {
                    break;
                }
                j -= 1
            }
            *(*(*ret.offset(i as isize)).offset(m as isize)).offset(1 as i32 as isize) = j as f32;
            m += 1
        }
        i += 1
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn _vp_psy_init(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy,
    mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut n: i32,
    mut rate: isize,
) {
    let mut i: isize = 0;
    let mut j: isize = 0;
    let mut lo: isize = -(99 as i32) as isize;
    let mut hi: isize = 1 as i32 as isize;
    let mut maxoc: isize = 0;
    crate::stdlib::memset(
        p as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy>()
            as libc::c_ulong,
    );
    (*p).eighth_octave_lines = (*gi).eighth_octave_lines;
    (*p).shiftoc = (crate::stdlib::rint(
        crate::stdlib::log(((*gi).eighth_octave_lines as f32 * 8.0f32) as f64)
            / crate::stdlib::log(2.0f32 as f64),
    ) - 1 as i32 as f64) as isize;
    (*p).firstoc = ((crate::stdlib::log((0.25f32 * rate as f32) as f64 * 0.5f64 / n as f64)
        * 1.442695f32 as f64
        - 5.965784f32 as f64)
        * ((1 as i32) << (*p).shiftoc + 1 as i32 as isize) as f64
        - (*gi).eighth_octave_lines as f64) as isize;
    maxoc = ((crate::stdlib::log(((n as f32 + 0.25f32) * rate as f32) as f64 * 0.5f64 / n as f64)
        * 1.442695f32 as f64
        - 5.965784f32 as f64)
        * ((1 as i32) << (*p).shiftoc + 1 as i32 as isize) as f64
        + 0.5f32 as f64) as isize;
    (*p).total_octave_lines = (maxoc - (*p).firstoc + 1 as i32 as isize) as i32;
    (*p).ath = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
    ) as *mut f32;
    (*p).octave = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
    ) as *mut isize;
    (*p).bark = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong),
    ) as *mut isize;
    (*p).vi = vi;
    (*p).n = n;
    (*p).rate = rate;
    /* AoTuV HF weighting */
    (*p).m_val = 1.0f64 as f32; /* 48kHz */
    if rate < 26000 as i32 as isize {
        (*p).m_val = 0 as i32 as f32
    } else if rate < 38000 as i32 as isize {
        (*p).m_val = 0.94f64 as f32
    } else if rate > 46000 as i32 as isize {
        (*p).m_val = 1.275f64 as f32
    } /* 32kHz */
    /* set up the lookups for a given blocksize and sample rate */
    i = 0 as i32 as isize;
    j = 0 as i32 as isize;
    while i < (88 as i32 - 1 as i32) as isize {
        let mut endpos: i32 = crate::stdlib::rint(
            crate::stdlib::exp(
                ((i + 1 as i32 as isize) as f64 * 0.125f64 - 2.0f64 + 5.965784f32 as f64)
                    * 0.693147f32 as f64,
            ) * 2 as i32 as f64
                * n as f64
                / rate as f64,
        ) as i32;
        let mut base: f32 = ATH[i as usize];
        if j < endpos as isize {
            let mut delta: f32 =
                (ATH[(i + 1 as i32 as isize) as usize] - base) / (endpos as isize - j) as f32;
            while j < endpos as isize && j < n as isize {
                *(*p).ath.offset(j as isize) = (base as f64 + 100.0f64) as f32;
                base += delta;
                j += 1
            }
        }
        i += 1
    }
    while j < n as isize {
        *(*p).ath.offset(j as isize) = *(*p).ath.offset((j - 1 as i32 as isize) as isize);
        j += 1
    }
    i = 0 as i32 as isize;
    while i < n as isize {
        let mut bark: f32 = (13.1f32 as f64
            * crate::stdlib::atan(
                (0.00074f32 * (rate / (2 as i32 * n) as isize * i) as f32) as f64,
            )
            + 2.24f32 as f64
                * crate::stdlib::atan(
                    ((rate / (2 as i32 * n) as isize * i * (rate / (2 as i32 * n) as isize * i))
                        as f32
                        * 1.85e-8f32) as f64,
                )
            + (1e-4f32 * (rate / (2 as i32 * n) as isize * i) as f32) as f64)
            as f32;
        while (lo + (*vi).noisewindowlomin as isize) < i
            && (13.1f32 as f64
                * crate::stdlib::atan(
                    (0.00074f32 * (rate / (2 as i32 * n) as isize * lo) as f32) as f64,
                )
                + 2.24f32 as f64
                    * crate::stdlib::atan(
                        ((rate / (2 as i32 * n) as isize
                            * lo
                            * (rate / (2 as i32 * n) as isize * lo))
                            as f32
                            * 1.85e-8f32) as f64,
                    )
                + (1e-4f32 * (rate / (2 as i32 * n) as isize * lo) as f32) as f64)
                < (bark - (*vi).noisewindowlo) as f64
        {
            lo += 1
        }
        while hi <= n as isize
            && (hi < i + (*vi).noisewindowhimin as isize
                || (13.1f32 as f64
                    * crate::stdlib::atan(
                        (0.00074f32 * (rate / (2 as i32 * n) as isize * hi) as f32) as f64,
                    )
                    + 2.24f32 as f64
                        * crate::stdlib::atan(
                            ((rate / (2 as i32 * n) as isize
                                * hi
                                * (rate / (2 as i32 * n) as isize * hi))
                                as f32
                                * 1.85e-8f32) as f64,
                        )
                    + (1e-4f32 * (rate / (2 as i32 * n) as isize * hi) as f32) as f64)
                    < (bark + (*vi).noisewindowhi) as f64)
        {
            hi += 1
        }
        *(*p).bark.offset(i as isize) =
            ((lo - 1 as i32 as isize) << 16 as i32) + (hi - 1 as i32 as isize);
        i += 1
    }
    i = 0 as i32 as isize;
    while i < n as isize {
        *(*p).octave.offset(i as isize) =
            ((crate::stdlib::log((i as f32 + 0.25f32) as f64 * 0.5f64 * rate as f64 / n as f64)
                * 1.442695f32 as f64
                - 5.965784f32 as f64)
                * ((1 as i32) << (*p).shiftoc + 1 as i32 as isize) as f64
                + 0.5f32 as f64) as isize;
        i += 1
    }
    (*p).tonecurves = setup_tone_curves(
        (*vi).toneatt.as_mut_ptr(),
        (rate as f64 * 0.5f64 / n as f64) as f32,
        n,
        (*vi).tone_centerboost,
        (*vi).tone_decay,
    );
    /* set up rolling noise median */
    (*p).noiseoffset = crate::stdlib::malloc(
        (3 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut f32>() as libc::c_ulong),
    ) as *mut *mut f32;
    i = 0 as i32 as isize;
    while i < 3 as i32 as isize {
        let ref mut fresh3 = *(*p).noiseoffset.offset(i as isize);
        *fresh3 = crate::stdlib::malloc(
            (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong),
        ) as *mut f32;
        i += 1
    }
    i = 0 as i32 as isize;
    while i < n as isize {
        let mut halfoc: f32 =
            ((crate::stdlib::log((i as f64 + 0.5f64) * rate as f64 / (2.0f64 * n as f64))
                * 1.442695f32 as f64
                - 5.965784f32 as f64)
                * 2.0f64) as f32;
        let mut inthalfoc: i32 = 0;
        let mut del: f32 = 0.;
        if halfoc < 0 as i32 as f32 {
            halfoc = 0 as i32 as f32
        }
        if halfoc >= (17 as i32 - 1 as i32) as f32 {
            halfoc = (17 as i32 - 1 as i32) as f32
        }
        inthalfoc = halfoc as i32;
        del = halfoc - inthalfoc as f32;
        j = 0 as i32 as isize;
        while j < 3 as i32 as isize {
            *(*(*p).noiseoffset.offset(j as isize)).offset(i as isize) =
                ((*(*p).vi).noiseoff[j as usize][inthalfoc as usize] as f64 * (1.0f64 - del as f64)
                    + ((*(*p).vi).noiseoff[j as usize][(inthalfoc + 1 as i32) as usize] * del)
                        as f64) as f32;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_psy_clear(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if !p.is_null() {
        if !(*p).ath.is_null() {
            ::libc::free((*p).ath as *mut libc::c_void);
        }
        if !(*p).octave.is_null() {
            ::libc::free((*p).octave as *mut libc::c_void);
        }
        if !(*p).bark.is_null() {
            ::libc::free((*p).bark as *mut libc::c_void);
        }
        if !(*p).tonecurves.is_null() {
            i = 0 as i32;
            while i < 17 as i32 {
                j = 0 as i32;
                while j < 8 as i32 {
                    ::libc::free(*(*(*p).tonecurves.offset(i as isize)).offset(j as isize)
                        as *mut libc::c_void);
                    j += 1
                }
                ::libc::free(*(*p).tonecurves.offset(i as isize) as *mut libc::c_void);
                i += 1
            }
            ::libc::free((*p).tonecurves as *mut libc::c_void);
        }
        if !(*p).noiseoffset.is_null() {
            i = 0 as i32;
            while i < 3 as i32 {
                ::libc::free(*(*p).noiseoffset.offset(i as isize) as *mut libc::c_void);
                i += 1
            }
            ::libc::free((*p).noiseoffset as *mut libc::c_void);
        }
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy>()
                as libc::c_ulong,
        );
    };
}
/* octave/(8*eighth_octave_lines) x scale and dB y scale */

unsafe extern "C" fn seed_curve(
    mut seed: *mut f32,
    mut curves: *mut *const f32,
    mut amp: f32,
    mut oc: i32,
    mut n: i32,
    mut linesper: i32,
    mut dBoffset: f32,
) {
    let mut i: i32 = 0;
    let mut post1: i32 = 0;
    let mut seedptr: i32 = 0;
    let mut posts: *const f32 = 0 as *const f32;
    let mut curve: *const f32 = 0 as *const f32;
    let mut choice: i32 = (((amp + dBoffset) as f64 - 30.0f64) * 0.1f32 as f64) as i32;
    choice = if choice < 0 as i32 { 0 as i32 } else { choice };
    choice = if choice > 8 as i32 - 1 as i32 {
        (8 as i32) - 1 as i32
    } else {
        choice
    };
    posts = *curves.offset(choice as isize);
    curve = posts.offset(2 as i32 as isize);
    post1 = *posts.offset(1 as i32 as isize) as i32;
    seedptr = (oc as f32 + (*posts.offset(0 as i32 as isize) - 16 as i32 as f32) * linesper as f32
        - (linesper >> 1 as i32) as f32) as i32;
    i = *posts.offset(0 as i32 as isize) as i32;
    while i < post1 {
        if seedptr > 0 as i32 {
            let mut lin: f32 = amp + *curve.offset(i as isize);
            if *seed.offset(seedptr as isize) < lin {
                *seed.offset(seedptr as isize) = lin
            }
        }
        seedptr += linesper;
        if seedptr >= n {
            break;
        }
        i += 1
    }
}

unsafe extern "C" fn seed_loop(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut curves: *mut *mut *const f32,
    mut f: *const f32,
    mut flr: *const f32,
    mut seed: *mut f32,
    mut specmax: f32,
) {
    let mut vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy = (*p).vi;
    let mut n: isize = (*p).n as isize;
    let mut i: isize = 0;
    let mut dBoffset: f32 = (*vi).max_curve_dB - specmax;
    /* prime the working vector with peak values */
    i = 0 as i32 as isize;
    while i < n {
        let mut max: f32 = *f.offset(i as isize);
        let mut oc: isize = *(*p).octave.offset(i as isize);
        while (i + 1 as i32 as isize) < n
            && *(*p).octave.offset((i + 1 as i32 as isize) as isize) == oc
        {
            i += 1;
            if *f.offset(i as isize) > max {
                max = *f.offset(i as isize)
            }
        }
        if max + 6.0f32 > *flr.offset(i as isize) {
            oc = oc >> (*p).shiftoc;
            if oc >= 17 as i32 as isize {
                oc = (17 as i32 - 1 as i32) as isize
            }
            if oc < 0 as i32 as isize {
                oc = 0 as i32 as isize
            }
            seed_curve(
                seed,
                *curves.offset(oc as isize),
                max,
                (*(*p).octave.offset(i as isize) - (*p).firstoc) as i32,
                (*p).total_octave_lines,
                (*p).eighth_octave_lines,
                dBoffset,
            );
        }
        i += 1
    }
}

unsafe extern "C" fn seed_chase(mut seeds: *mut f32, mut linesper: i32, mut n: isize) {
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<isize>() as libc::c_ulong) as usize,
    );
    let mut posstack: *mut isize = fresh4.as_mut_ptr() as *mut isize;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut ampstack: *mut f32 = fresh5.as_mut_ptr() as *mut f32;
    let mut stack: isize = 0 as i32 as isize;
    let mut pos: isize = 0 as i32 as isize;
    let mut i: isize = 0;
    i = 0 as i32 as isize;
    while i < n {
        if stack < 2 as i32 as isize {
            *posstack.offset(stack as isize) = i;
            let fresh6 = stack;
            stack = stack + 1;
            *ampstack.offset(fresh6 as isize) = *seeds.offset(i as isize)
        } else {
            loop {
                if *seeds.offset(i as isize)
                    < *ampstack.offset((stack - 1 as i32 as isize) as isize)
                {
                    *posstack.offset(stack as isize) = i;
                    let fresh7 = stack;
                    stack = stack + 1;
                    *ampstack.offset(fresh7 as isize) = *seeds.offset(i as isize);
                    break;
                } else {
                    if i < *posstack.offset((stack - 1 as i32 as isize) as isize)
                        + linesper as isize
                    {
                        if stack > 1 as i32 as isize
                            && *ampstack.offset((stack - 1 as i32 as isize) as isize)
                                <= *ampstack.offset((stack - 2 as i32 as isize) as isize)
                            && i < *posstack.offset((stack - 2 as i32 as isize) as isize)
                                + linesper as isize
                        {
                            /* we completely overlap, making stack-1 irrelevant.  pop it */
                            stack -= 1;
                            continue;
                        }
                    }
                    *posstack.offset(stack as isize) = i;
                    let fresh8 = stack;
                    stack = stack + 1;
                    *ampstack.offset(fresh8 as isize) = *seeds.offset(i as isize);
                    break;
                }
            }
        }
        i += 1
    }
    /* the stack now contains only the positions that are relevant. Scan
    'em straight through */
    i = 0 as i32 as isize;
    while i < stack {
        let mut endpos: isize = 0;
        if i < stack - 1 as i32 as isize
            && *ampstack.offset((i + 1 as i32 as isize) as isize) > *ampstack.offset(i as isize)
        {
            endpos = *posstack.offset((i + 1 as i32 as isize) as isize)
        } else {
            endpos = *posstack.offset(i as isize) + linesper as isize + 1 as i32 as isize
            /* +1 is important, else bin 0 is
            discarded in short frames */
        }
        if endpos > n {
            endpos = n
        }
        while pos < endpos {
            *seeds.offset(pos as isize) = *ampstack.offset(i as isize);
            pos += 1
        }
        i += 1
    }
    /* there.  Linear time.  I now remember this was on a problem set I
    had in Grad Skool... I didn't solve it at the time ;-) */
}
/* bleaugh, this is more complicated than it needs to be */

unsafe extern "C" fn max_seeds(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut seed: *mut f32,
    mut flr: *mut f32,
) {
    let mut n: isize = (*p).total_octave_lines as isize; /* for masking */
    let mut linesper: i32 = (*p).eighth_octave_lines;
    let mut linpos: isize = 0 as i32 as isize;
    let mut pos: isize = 0;
    seed_chase(seed, linesper, n);
    pos = *(*p).octave.offset(0 as i32 as isize) - (*p).firstoc - (linesper >> 1 as i32) as isize;
    while (linpos + 1 as i32 as isize) < (*p).n as isize {
        let mut minV: f32 = *seed.offset(pos as isize);
        let mut end: isize = (*(*p).octave.offset(linpos as isize)
            + *(*p).octave.offset((linpos + 1 as i32 as isize) as isize)
            >> 1 as i32)
            - (*p).firstoc;
        if minV > (*(*p).vi).tone_abs_limit {
            minV = (*(*p).vi).tone_abs_limit
        }
        while pos + 1 as i32 as isize <= end {
            pos += 1;
            if *seed.offset(pos as isize) > -9999.0f32 && *seed.offset(pos as isize) < minV
                || minV == -9999.0f32
            {
                minV = *seed.offset(pos as isize)
            }
        }
        end = pos + (*p).firstoc;
        while linpos < (*p).n as isize && *(*p).octave.offset(linpos as isize) <= end {
            if *flr.offset(linpos as isize) < minV {
                *flr.offset(linpos as isize) = minV
            }
            linpos += 1
        }
    }
    let mut minV_0: f32 = *seed.offset(((*p).total_octave_lines - 1 as i32) as isize);
    while linpos < (*p).n as isize {
        if *flr.offset(linpos as isize) < minV_0 {
            *flr.offset(linpos as isize) = minV_0
        }
        linpos += 1
    }
}

unsafe extern "C" fn bark_noise_hybridmp(
    mut n: i32,
    mut b: *const isize,
    mut f: *const f32,
    mut noise: *mut f32,
    offset: f32,
    fixed: i32,
) {
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut N: *mut f32 = fresh9.as_mut_ptr() as *mut f32;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut X: *mut f32 = fresh10.as_mut_ptr() as *mut f32;
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut XX: *mut f32 = fresh11.as_mut_ptr() as *mut f32;
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut Y: *mut f32 = fresh12.as_mut_ptr() as *mut f32;
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut XY: *mut f32 = fresh13.as_mut_ptr() as *mut f32;
    let mut tN: f32 = 0.;
    let mut tX: f32 = 0.;
    let mut tXX: f32 = 0.;
    let mut tY: f32 = 0.;
    let mut tXY: f32 = 0.;
    let mut i: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    let mut R: f32 = 0.0f32;
    let mut A: f32 = 0.0f32;
    let mut B: f32 = 0.0f32;
    let mut D: f32 = 1.0f32;
    let mut w: f32 = 0.;
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    tXY = 0.0f32;
    tY = tXY;
    tXX = tY;
    tX = tXX;
    tN = tX;
    y = *f.offset(0 as i32 as isize) + offset;
    if y < 1.0f32 {
        y = 1.0f32
    }
    w = ((y * y) as f64 * 0.5f64) as f32;
    tN += w;
    tX += w;
    tY += w * y;
    *N.offset(0 as i32 as isize) = tN;
    *X.offset(0 as i32 as isize) = tX;
    *XX.offset(0 as i32 as isize) = tXX;
    *Y.offset(0 as i32 as isize) = tY;
    *XY.offset(0 as i32 as isize) = tXY;
    i = 1 as i32;
    x = 1.0f32;
    while i < n {
        y = *f.offset(i as isize) + offset;
        if y < 1.0f32 {
            y = 1.0f32
        }
        w = y * y;
        tN += w;
        tX += w * x;
        tXX += w * x * x;
        tY += w * y;
        tXY += w * x * y;
        *N.offset(i as isize) = tN;
        *X.offset(i as isize) = tX;
        *XX.offset(i as isize) = tXX;
        *Y.offset(i as isize) = tY;
        *XY.offset(i as isize) = tXY;
        i += 1;
        x += 1.0f32
    }
    i = 0 as i32;
    x = 0.0f32;
    loop {
        lo = (*b.offset(i as isize) >> 16 as i32) as i32;
        if lo >= 0 as i32 {
            break;
        }
        hi = (*b.offset(i as isize) & 0xffff as i32 as isize) as i32;
        tN = *N.offset(hi as isize) + *N.offset(-lo as isize);
        tX = *X.offset(hi as isize) - *X.offset(-lo as isize);
        tXX = *XX.offset(hi as isize) + *XX.offset(-lo as isize);
        tY = *Y.offset(hi as isize) + *Y.offset(-lo as isize);
        tXY = *XY.offset(hi as isize) - *XY.offset(-lo as isize);
        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if R < 0.0f32 {
            R = 0.0f32
        }
        *noise.offset(i as isize) = R - offset;
        i += 1;
        x += 1.0f32
    }
    loop {
        lo = (*b.offset(i as isize) >> 16 as i32) as i32;
        hi = (*b.offset(i as isize) & 0xffff as i32 as isize) as i32;
        if hi >= n {
            break;
        }
        tN = *N.offset(hi as isize) - *N.offset(lo as isize);
        tX = *X.offset(hi as isize) - *X.offset(lo as isize);
        tXX = *XX.offset(hi as isize) - *XX.offset(lo as isize);
        tY = *Y.offset(hi as isize) - *Y.offset(lo as isize);
        tXY = *XY.offset(hi as isize) - *XY.offset(lo as isize);
        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if R < 0.0f32 {
            R = 0.0f32
        }
        *noise.offset(i as isize) = R - offset;
        i += 1;
        x += 1.0f32
    }
    while i < n {
        R = (A + x * B) / D;
        if R < 0.0f32 {
            R = 0.0f32
        }
        *noise.offset(i as isize) = R - offset;
        i += 1;
        x += 1.0f32
    }
    if fixed <= 0 as i32 {
        return;
    }
    i = 0 as i32;
    x = 0.0f32;
    loop {
        hi = i + fixed / 2 as i32;
        lo = hi - fixed;
        if lo >= 0 as i32 {
            break;
        }
        tN = *N.offset(hi as isize) + *N.offset(-lo as isize);
        tX = *X.offset(hi as isize) - *X.offset(-lo as isize);
        tXX = *XX.offset(hi as isize) + *XX.offset(-lo as isize);
        tY = *Y.offset(hi as isize) + *Y.offset(-lo as isize);
        tXY = *XY.offset(hi as isize) - *XY.offset(-lo as isize);
        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if R - offset < *noise.offset(i as isize) {
            *noise.offset(i as isize) = R - offset
        }
        i += 1;
        x += 1.0f32
    }
    loop {
        hi = i + fixed / 2 as i32;
        lo = hi - fixed;
        if hi >= n {
            break;
        }
        tN = *N.offset(hi as isize) - *N.offset(lo as isize);
        tX = *X.offset(hi as isize) - *X.offset(lo as isize);
        tXX = *XX.offset(hi as isize) - *XX.offset(lo as isize);
        tY = *Y.offset(hi as isize) - *Y.offset(lo as isize);
        tXY = *XY.offset(hi as isize) - *XY.offset(lo as isize);
        A = tY * tXX - tX * tXY;
        B = tN * tXY - tX * tY;
        D = tN * tXX - tX * tX;
        R = (A + x * B) / D;
        if R - offset < *noise.offset(i as isize) {
            *noise.offset(i as isize) = R - offset
        }
        i += 1;
        x += 1.0f32
    }
    while i < n {
        R = (A + x * B) / D;
        if R - offset < *noise.offset(i as isize) {
            *noise.offset(i as isize) = R - offset
        }
        i += 1;
        x += 1.0f32
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_noisemask(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut logmdct: *mut f32,
    mut logmask: *mut f32,
) {
    let mut i: i32 = 0;
    let mut n: i32 = (*p).n;
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut work: *mut f32 = fresh14.as_mut_ptr() as *mut f32;
    bark_noise_hybridmp(n, (*p).bark, logmdct, logmask, 140.0f64 as f32, -(1 as i32));
    i = 0 as i32;
    while i < n {
        *work.offset(i as isize) = *logmdct.offset(i as isize) - *logmask.offset(i as isize);
        i += 1
    }
    bark_noise_hybridmp(
        n,
        (*p).bark,
        work,
        logmask,
        0.0f64 as f32,
        (*(*p).vi).noisewindowfixed,
    );
    i = 0 as i32;
    while i < n {
        *work.offset(i as isize) = *logmdct.offset(i as isize) - *work.offset(i as isize);
        i += 1
    }
    i = 0 as i32;
    while i < n {
        let mut dB: i32 = (*logmask.offset(i as isize) as f64 + 0.5f64) as i32;
        if dB >= 40 as i32 {
            dB = 40 as i32 - 1 as i32
        }
        if dB < 0 as i32 {
            dB = 0 as i32
        }
        *logmask.offset(i as isize) =
            *work.offset(i as isize) + (*(*p).vi).noisecompand[dB as usize];
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_tonemask(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut logfft: *mut f32,
    mut logmask: *mut f32,
    mut global_specmax: f32,
    mut local_specmax: f32,
) {
    let mut i: i32 = 0;
    let mut n: i32 = (*p).n;
    let mut fresh15 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as libc::c_ulong)
            .wrapping_mul((*p).total_octave_lines as libc::c_ulong) as usize,
    );
    let mut seed: *mut f32 = fresh15.as_mut_ptr() as *mut f32;
    let mut att: f32 = local_specmax + (*(*p).vi).ath_adjatt;
    i = 0 as i32;
    while i < (*p).total_octave_lines {
        *seed.offset(i as isize) = -9999.0f32;
        i += 1
    }
    /* set the ATH (floating below localmax, not global max by a
    specified att) */
    if att < (*(*p).vi).ath_maxatt {
        att = (*(*p).vi).ath_maxatt
    }
    i = 0 as i32;
    while i < n {
        *logmask.offset(i as isize) = *(*p).ath.offset(i as isize) + att;
        i += 1
    }
    /* tone masking */
    seed_loop(
        p,
        (*p).tonecurves as *mut *mut *const f32,
        logfft,
        logmask,
        seed,
        global_specmax,
    ); /* AoTuV */
    max_seeds(p, seed, logmask);
}
#[no_mangle]

pub unsafe extern "C" fn _vp_offset_and_mix(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut noise: *mut f32,
    mut tone: *mut f32,
    mut offset_select: i32,
    mut logmask: *mut f32,
    mut mdct: *mut f32,
    mut logmdct: *mut f32,
) {
    let mut i: i32 = 0;
    let mut n: i32 = (*p).n;
    let mut de: f32 = 0.;
    let mut coeffi: f32 = 0.;
    let mut cx: f32 = 0.;
    let mut toneatt: f32 = (*(*p).vi).tone_masteratt[offset_select as usize];
    cx = (*p).m_val;
    i = 0 as i32;
    while i < n {
        let mut val: f32 = *noise.offset(i as isize)
            + *(*(*p).noiseoffset.offset(offset_select as isize)).offset(i as isize);
        if val > (*(*p).vi).noisemaxsupp {
            val = (*(*p).vi).noisemaxsupp
        }
        *logmask.offset(i as isize) = if val < *tone.offset(i as isize) + toneatt {
            (*tone.offset(i as isize)) + toneatt
        } else {
            val
        };
        /* AoTuV */
        /* * @ M1 **
            The following codes improve a noise problem.
            A fundamental idea uses the value of masking and carries out
            the relative compensation of the MDCT.
            However, this code is not perfect and all noise problems cannot be solved.
            by Aoyumi @ 2004/04/18
        */
        if offset_select == 1 as i32 {
            coeffi = -17.2f64 as f32; /* coeffi is a -17.2dB threshold */
            val = val - *logmdct.offset(i as isize); /* val == mdct line value relative to floor in dB */
            if val > coeffi {
                /* mdct value is > -17.2 dB below floor */
                de = (1.0f64 - (val - coeffi) as f64 * 0.005f64 * cx as f64) as f32;
                /* pro-rated attenuation:
                -0.00 dB boost if mdct value is -17.2dB (relative to floor)
                -0.77 dB boost if mdct value is 0dB (relative to floor)
                -1.64 dB boost if mdct value is +17.2dB (relative to floor)
                etc... */
                if de < 0 as i32 as f32 {
                    de = 0.0001f64 as f32
                }
            } else {
                /* mdct value is <= -17.2 dB below floor */
                de = (1.0f64 - (val - coeffi) as f64 * 0.0003f64 * cx as f64) as f32
            }
            /* pro-rated attenuation:
            +0.00 dB atten if mdct value is -17.2dB (relative to floor)
            +0.45 dB atten if mdct value is -34.4dB (relative to floor)
            etc... */
            *mdct.offset(i as isize) *= de
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_ampmax_decay(
    mut amp: f32,
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
) -> f32 {
    let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut n: i32 = ((*ci).blocksizes[(*vd).W as usize] / 2 as i32 as isize) as i32;
    let mut secs: f32 = n as f32 / (*vi).rate as f32;
    amp += secs * (*gi).ampmax_att_per_sec;
    if amp < -(9999 as i32) as f32 {
        amp = -(9999 as i32) as f32
    }
    return amp;
}

static mut FLOOR1_fromdB_LOOKUP: [f32; 256] = [
    1.0649863e-07f32,
    1.1341951e-07f32,
    1.2079015e-07f32,
    1.2863978e-07f32,
    1.3699951e-07f32,
    1.4590251e-07f32,
    1.5538408e-07f32,
    1.6548181e-07f32,
    1.7623575e-07f32,
    1.8768855e-07f32,
    1.9988561e-07f32,
    2.128753e-07f32,
    2.2670913e-07f32,
    2.4144197e-07f32,
    2.5713223e-07f32,
    2.7384213e-07f32,
    2.9163793e-07f32,
    3.1059021e-07f32,
    3.3077411e-07f32,
    3.5226968e-07f32,
    3.7516214e-07f32,
    3.9954229e-07f32,
    4.2550680e-07f32,
    4.5315863e-07f32,
    4.8260743e-07f32,
    5.1396998e-07f32,
    5.4737065e-07f32,
    5.8294187e-07f32,
    6.2082472e-07f32,
    6.6116941e-07f32,
    7.0413592e-07f32,
    7.4989464e-07f32,
    7.9862701e-07f32,
    8.5052630e-07f32,
    9.0579828e-07f32,
    9.6466216e-07f32,
    1.0273513e-06f32,
    1.0941144e-06f32,
    1.1652161e-06f32,
    1.2409384e-06f32,
    1.3215816e-06f32,
    1.4074654e-06f32,
    1.4989305e-06f32,
    1.5963394e-06f32,
    1.7000785e-06f32,
    1.8105592e-06f32,
    1.9282195e-06f32,
    2.0535261e-06f32,
    2.1869758e-06f32,
    2.3290978e-06f32,
    2.4804557e-06f32,
    2.6416497e-06f32,
    2.8133190e-06f32,
    2.9961443e-06f32,
    3.1908506e-06f32,
    3.3982101e-06f32,
    3.6190449e-06f32,
    3.8542308e-06f32,
    4.1047004e-06f32,
    4.3714470e-06f32,
    4.6555282e-06f32,
    4.9580707e-06f32,
    5.2802740e-06f32,
    5.6234160e-06f32,
    5.9888572e-06f32,
    6.3780469e-06f32,
    6.7925283e-06f32,
    7.2339451e-06f32,
    7.7040476e-06f32,
    8.2047000e-06f32,
    8.7378876e-06f32,
    9.3057248e-06f32,
    9.9104632e-06f32,
    1.0554501e-05f32,
    1.1240392e-05f32,
    1.1970856e-05f32,
    1.2748789e-05f32,
    1.3577278e-05f32,
    1.4459606e-05f32,
    1.5399272e-05f32,
    1.6400004e-05f32,
    1.7465768e-05f32,
    1.8600792e-05f32,
    1.9809576e-05f32,
    2.1096914e-05f32,
    2.2467911e-05f32,
    2.3928002e-05f32,
    2.5482978e-05f32,
    2.7139006e-05f32,
    2.8902651e-05f32,
    3.0780908e-05f32,
    3.2781225e-05f32,
    3.4911534e-05f32,
    3.7180282e-05f32,
    3.9596466e-05f32,
    4.2169667e-05f32,
    4.4910090e-05f32,
    4.7828601e-05f32,
    5.0936773e-05f32,
    5.4246931e-05f32,
    5.7772202e-05f32,
    6.1526565e-05f32,
    6.5524908e-05f32,
    6.9783085e-05f32,
    7.4317983e-05f32,
    7.9147585e-05f32,
    8.4291040e-05f32,
    8.9768747e-05f32,
    9.5602426e-05f32,
    0.00010181521f32,
    0.00010843174f32,
    0.00011547824f32,
    0.00012298267f32,
    0.00013097477f32,
    0.00013948625f32,
    0.00014855085f32,
    0.00015820453f32,
    0.00016848555f32,
    0.00017943469f32,
    0.00019109536f32,
    0.00020351382f32,
    0.00021673929f32,
    0.00023082423f32,
    0.00024582449f32,
    0.00026179955f32,
    0.00027881276f32,
    0.00029693158f32,
    0.00031622787f32,
    0.00033677814f32,
    0.00035866388f32,
    0.00038197188f32,
    0.00040679456f32,
    0.00043323036f32,
    0.00046138411f32,
    0.00049136745f32,
    0.00052329927f32,
    0.00055730621f32,
    0.00059352311f32,
    0.00063209358f32,
    0.00067317058f32,
    0.00071691700f32,
    0.00076350630f32,
    0.00081312324f32,
    0.00086596457f32,
    0.00092223983f32,
    0.00098217216f32,
    0.0010459992f32,
    0.0011139742f32,
    0.0011863665f32,
    0.0012634633f32,
    0.0013455702f32,
    0.0014330129f32,
    0.0015261382f32,
    0.0016253153f32,
    0.0017309374f32,
    0.0018434235f32,
    0.0019632195f32,
    0.0020908006f32,
    0.0022266726f32,
    0.0023713743f32,
    0.0025254795f32,
    0.0026895994f32,
    0.0028643847f32,
    0.0030505286f32,
    0.0032487691f32,
    0.0034598925f32,
    0.0036847358f32,
    0.0039241906f32,
    0.0041792066f32,
    0.0044507950f32,
    0.0047400328f32,
    0.0050480668f32,
    0.0053761186f32,
    0.0057254891f32,
    0.0060975636f32,
    0.0064938176f32,
    0.0069158225f32,
    0.0073652516f32,
    0.0078438871f32,
    0.0083536271f32,
    0.0088964928f32,
    0.009474637f32,
    0.010090352f32,
    0.010746080f32,
    0.011444421f32,
    0.012188144f32,
    0.012980198f32,
    0.013823725f32,
    0.014722068f32,
    0.015678791f32,
    0.016697687f32,
    0.017782797f32,
    0.018938423f32,
    0.020169149f32,
    0.021479854f32,
    0.022875735f32,
    0.024362330f32,
    0.025945531f32,
    0.027631618f32,
    0.029427276f32,
    0.031339626f32,
    0.033376252f32,
    0.035545228f32,
    0.037855157f32,
    0.040315199f32,
    0.042935108f32,
    0.045725273f32,
    0.048696758f32,
    0.051861348f32,
    0.055231591f32,
    0.058820850f32,
    0.062643361f32,
    0.066714279f32,
    0.071049749f32,
    0.075666962f32,
    0.080584227f32,
    0.085821044f32,
    0.091398179f32,
    0.097337747f32,
    0.10366330f32,
    0.11039993f32,
    0.11757434f32,
    0.12521498f32,
    0.13335215f32,
    0.14201813f32,
    0.15124727f32,
    0.16107617f32,
    0.17154380f32,
    0.18269168f32,
    0.19456402f32,
    0.20720788f32,
    0.22067342f32,
    0.23501402f32,
    0.25028656f32,
    0.26655159f32,
    0.28387361f32,
    0.30232132f32,
    0.32196786f32,
    0.34289114f32,
    0.36517414f32,
    0.38890521f32,
    0.41417847f32,
    0.44109412f32,
    0.46975890f32,
    0.50028648f32,
    0.53279791f32,
    0.56742212f32,
    0.60429640f32,
    0.64356699f32,
    0.68538959f32,
    0.72993007f32,
    0.77736504f32,
    0.82788260f32,
    0.88168307f32,
    0.9389798f32,
    1.0f32,
];
/* this is for per-channel noise normalization */

unsafe extern "C" fn apsort(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    let mut f1: f32 = **(a as *mut *mut f32);
    let mut f2: f32 = **(b as *mut *mut f32);
    return (f1 < f2) as i32 - (f1 > f2) as i32;
}

unsafe extern "C" fn flag_lossless(
    mut limit: i32,
    mut prepoint: f32,
    mut postpoint: f32,
    mut mdct: *mut f32,
    mut floor_0: *mut f32,
    mut flag: *mut i32,
    mut i: i32,
    mut jn: i32,
) {
    let mut j: i32 = 0;
    j = 0 as i32;
    while j < jn {
        let mut point: f32 = if j >= limit - i { postpoint } else { prepoint };
        let mut r: f32 = (crate::stdlib::fabs(*mdct.offset(j as isize) as f64)
            / *floor_0.offset(j as isize) as f64) as f32;
        if r < point {
            *flag.offset(j as isize) = 0 as i32
        } else {
            *flag.offset(j as isize) = 1 as i32
        }
        j += 1
    }
}
/* Overload/Side effect: On input, the *q vector holds either the
quantized energy (for elements with the flag set) or the absolute
values of the *r vector (for elements with flag unset).  On output,
*q holds the quantized energy for all elements */

unsafe extern "C" fn noise_normalize(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut limit: i32,
    mut r: *mut f32,
    mut q: *mut f32,
    mut f: *mut f32,
    mut flags: *mut i32,
    mut acc: f32,
    mut i: i32,
    mut n: i32,
    mut out: *mut i32,
) -> f32 {
    let mut vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy = (*p).vi;
    let mut fresh16 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut f32>() as libc::c_ulong)
            as usize,
    );
    let mut sort: *mut *mut f32 = fresh16.as_mut_ptr() as *mut *mut f32;
    let mut j: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut start: i32 = if (*vi).normal_p != 0 {
        ((*vi).normal_start) - i
    } else {
        n
    };
    if start > n {
        start = n
    }
    /* force classic behavior where only energy in the current band is considered */
    acc = 0.0f32;
    /* still responsible for populating *out where noise norm not in
    effect.  There's no need to [re]populate *q in these areas */
    j = 0 as i32;
    while j < start {
        if flags.is_null() || *flags.offset(j as isize) == 0 {
            /* lossless coupling already quantized.
            Don't touch; requantizing based on
            energy would be incorrect. */
            let mut ve: f32 = *q.offset(j as isize) / *f.offset(j as isize);
            if *r.offset(j as isize) < 0 as i32 as f32 {
                *out.offset(j as isize) =
                    -crate::stdlib::rint(crate::stdlib::sqrt(ve as f64)) as i32
            } else {
                *out.offset(j as isize) = crate::stdlib::rint(crate::stdlib::sqrt(ve as f64)) as i32
            }
        }
        j += 1
    }
    /* sort magnitudes for noise norm portion of partition */
    while j < n {
        if flags.is_null() || *flags.offset(j as isize) == 0 {
            /* can't noise norm elements that have
            already been loslessly coupled; we can
            only account for their energy error */
            let mut ve_0: f32 = *q.offset(j as isize) / *f.offset(j as isize);
            /* Despite all the new, more capable coupling code, for now we
            implement noise norm as it has been up to this point. Only
            consider promotions to unit magnitude from 0.  In addition
            the only energy error counted is quantizations to zero. */
            /* also-- the original point code only applied noise norm at > pointlimit */
            if ve_0 < 0.25f32 && (flags.is_null() || j >= limit - i) {
                acc += ve_0;
                let fresh17 = count;
                count = count + 1;
                let ref mut fresh18 = *sort.offset(fresh17 as isize);
                *fresh18 = q.offset(j as isize)
            /* q is fabs(r) for unflagged element */
            } else {
                /* For now: no acc adjustment for nonzero quantization.  populate *out and q as this value is final. */
                if *r.offset(j as isize) < 0 as i32 as f32 {
                    *out.offset(j as isize) =
                        -crate::stdlib::rint(crate::stdlib::sqrt(ve_0 as f64)) as i32
                } else {
                    *out.offset(j as isize) =
                        crate::stdlib::rint(crate::stdlib::sqrt(ve_0 as f64)) as i32
                }
                *q.offset(j as isize) = (*out.offset(j as isize) * *out.offset(j as isize)) as f32
                    * *f.offset(j as isize)
            }
        }
        j += 1
        /* else{
        again, no energy adjustment for error in nonzero quant-- for now
        }*/
    }
    if count != 0 {
        /* noise norm to do */
        crate::stdlib::qsort(
            sort as *mut libc::c_void,
            count as crate::stddef_h::size_t,
            ::std::mem::size_of::<*mut f32>() as libc::c_ulong,
            Some(
                apsort
                    as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
            ),
        );
        j = 0 as i32;
        while j < count {
            let mut k: i32 = (*sort.offset(j as isize)).offset_from(q) as isize as i32;
            if acc as f64 >= (*vi).normal_thresh {
                *out.offset(k as isize) = unitnorm(*r.offset(k as isize)) as i32;
                acc -= 1.0f32;
                *q.offset(k as isize) = *f.offset(k as isize)
            } else {
                *out.offset(k as isize) = 0 as i32;
                *q.offset(k as isize) = 0.0f32
            }
            j += 1
        }
    }
    return acc;
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2009             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: random psychoacoustics (not including preecho)

********************************************************************/
/* psychoacoustic setup ********************************************/
/* 62Hz to 16kHz */
/* 30dB to 100dB */
/* 30 dB */
/* for block long/short tuning; encode only */
/* channel coupling config */
/* in n.ocshift format */
/* power of two, please */
/* cache it */
/* Masking compensation value */
/* Noise normalization, quantization and coupling are not wholly
seperable processes in depth>1 coupling. */
#[no_mangle]

pub unsafe extern "C" fn _vp_couple_quantize_normalize(
    mut blobno: i32,
    mut g: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut vi: *mut crate::backends_h::vorbis_info_mapping0,
    mut mdct: *mut *mut f32,
    mut iwork: *mut *mut i32,
    mut nonzero: *mut i32,
    mut sliding_lowpass: i32,
    mut ch: i32,
) {
    let mut i: i32 = 0;
    let mut n: i32 = (*p).n;
    let mut partition: i32 = if (*(*p).vi).normal_p != 0 {
        (*(*p).vi).normal_partition
    } else {
        16 as i32
    };
    let mut limit: i32 = (*g).coupling_pointlimit[(*(*p).vi).blockflag as usize][blobno as usize];
    let mut prepoint: f32 =
        stereo_threshholds[(*g).coupling_prepointamp[blobno as usize] as usize] as f32;
    let mut postpoint: f32 =
        stereo_threshholds[(*g).coupling_postpointamp[blobno as usize] as usize] as f32;
    /* mdct is our raw mdct output, floor not removed. */
    /* inout passes in the ifloor, passes back quantized result */
    /* unquantized energy (negative indicates amplitude has negative sign) */
    let mut fresh19 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut f32>() as libc::c_ulong)
            as usize,
    );
    let mut raw: *mut *mut f32 = fresh19.as_mut_ptr() as *mut *mut f32;
    /* dual pupose; quantized energy (if flag set), othersize fabs(raw) */
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut f32>() as libc::c_ulong)
            as usize,
    );
    let mut quant: *mut *mut f32 = fresh20.as_mut_ptr() as *mut *mut f32;
    /* floor energy */
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut f32>() as libc::c_ulong)
            as usize,
    );
    let mut floor_0: *mut *mut f32 = fresh21.as_mut_ptr() as *mut *mut f32;
    /* flags indicating raw/quantized status of elements in raw vector */
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut i32>() as libc::c_ulong)
            as usize,
    );
    let mut flag: *mut *mut i32 = fresh22.as_mut_ptr() as *mut *mut i32;
    /* non-zero flag working vector */
    let mut fresh23 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as usize,
    );
    let mut nz: *mut i32 = fresh23.as_mut_ptr() as *mut i32;
    /* energy surplus/defecit tracking */
    let mut fresh24 = ::std::vec::from_elem(
        0,
        ((ch + (*vi).coupling_steps) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let mut acc: *mut f32 = fresh24.as_mut_ptr() as *mut f32;
    /* The threshold of a stereo is changed with the size of n */
    if n > 1000 as i32 {
        postpoint =
            stereo_threshholds_limited[(*g).coupling_postpointamp[blobno as usize] as usize] as f32
    }
    let mut fresh25 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let ref mut fresh26 = *raw.offset(0 as i32 as isize);
    *fresh26 = fresh25.as_mut_ptr() as *mut f32;
    let mut fresh27 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let ref mut fresh28 = *quant.offset(0 as i32 as isize);
    *fresh28 = fresh27.as_mut_ptr() as *mut f32;
    let mut fresh29 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<f32>() as libc::c_ulong) as usize,
    );
    let ref mut fresh30 = *floor_0.offset(0 as i32 as isize);
    *fresh30 = fresh29.as_mut_ptr() as *mut f32;
    let mut fresh31 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as usize,
    );
    let ref mut fresh32 = *flag.offset(0 as i32 as isize);
    *fresh32 = fresh31.as_mut_ptr() as *mut i32;
    i = 1 as i32;
    while i < ch {
        let ref mut fresh33 = *raw.offset(i as isize);
        *fresh33 =
            &mut *(*raw.offset(0 as i32 as isize)).offset((partition * i) as isize) as *mut f32;
        let ref mut fresh34 = *quant.offset(i as isize);
        *fresh34 =
            &mut *(*quant.offset(0 as i32 as isize)).offset((partition * i) as isize) as *mut f32;
        let ref mut fresh35 = *floor_0.offset(i as isize);
        *fresh35 =
            &mut *(*floor_0.offset(0 as i32 as isize)).offset((partition * i) as isize) as *mut f32;
        let ref mut fresh36 = *flag.offset(i as isize);
        *fresh36 =
            &mut *(*flag.offset(0 as i32 as isize)).offset((partition * i) as isize) as *mut i32;
        i += 1
    }
    i = 0 as i32;
    while i < ch + (*vi).coupling_steps {
        *acc.offset(i as isize) = 0.0f32;
        i += 1
    }
    i = 0 as i32;
    while i < n {
        let mut k: i32 = 0;
        let mut j: i32 = 0;
        let mut jn: i32 = if partition > n - i {
            (n) - i
        } else {
            partition
        };
        let mut step: i32 = 0;
        let mut track: i32 = 0 as i32;
        crate::stdlib::memcpy(
            nz as *mut libc::c_void,
            nonzero as *const libc::c_void,
            (::std::mem::size_of::<i32>() as libc::c_ulong).wrapping_mul(ch as libc::c_ulong),
        );
        /* prefill */
        crate::stdlib::memset(
            *flag.offset(0 as i32 as isize) as *mut libc::c_void,
            0 as i32,
            ((ch * partition) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        );
        k = 0 as i32;
        while k < ch {
            let mut iout: *mut i32 =
                &mut *(*iwork.offset(k as isize)).offset(i as isize) as *mut i32;
            if *nz.offset(k as isize) != 0 {
                j = 0 as i32;
                while j < jn {
                    *(*floor_0.offset(k as isize)).offset(j as isize) =
                        FLOOR1_fromdB_LOOKUP[*iout.offset(j as isize) as usize];
                    j += 1
                }
                flag_lossless(
                    limit,
                    prepoint,
                    postpoint,
                    &mut *(*mdct.offset(k as isize)).offset(i as isize),
                    *floor_0.offset(k as isize),
                    *flag.offset(k as isize),
                    i,
                    jn,
                );
                j = 0 as i32;
                while j < jn {
                    let ref mut fresh37 = *(*raw.offset(k as isize)).offset(j as isize);
                    *fresh37 = *(*mdct.offset(k as isize)).offset((i + j) as isize)
                        * *(*mdct.offset(k as isize)).offset((i + j) as isize);
                    *(*quant.offset(k as isize)).offset(j as isize) = *fresh37;
                    if *(*mdct.offset(k as isize)).offset((i + j) as isize) < 0.0f32 {
                        *(*raw.offset(k as isize)).offset(j as isize) *= -1.0f32
                    }
                    *(*floor_0.offset(k as isize)).offset(j as isize) *=
                        *(*floor_0.offset(k as isize)).offset(j as isize);
                    j += 1
                }
                *acc.offset(track as isize) = noise_normalize(
                    p,
                    limit,
                    *raw.offset(k as isize),
                    *quant.offset(k as isize),
                    *floor_0.offset(k as isize),
                    0 as *mut i32,
                    *acc.offset(track as isize),
                    i,
                    jn,
                    iout,
                )
            } else {
                j = 0 as i32;
                while j < jn {
                    *(*floor_0.offset(k as isize)).offset(j as isize) = 1e-10f32;
                    *(*raw.offset(k as isize)).offset(j as isize) = 0.0f32;
                    *(*quant.offset(k as isize)).offset(j as isize) = 0.0f32;
                    *(*flag.offset(k as isize)).offset(j as isize) = 0 as i32;
                    *iout.offset(j as isize) = 0 as i32;
                    j += 1
                }
                *acc.offset(track as isize) = 0.0f32
            }
            track += 1;
            k += 1
        }
        /* coupling */
        step = 0 as i32;
        while step < (*vi).coupling_steps {
            let mut Mi: i32 = (*vi).coupling_mag[step as usize];
            let mut Ai: i32 = (*vi).coupling_ang[step as usize];
            let mut iM: *mut i32 =
                &mut *(*iwork.offset(Mi as isize)).offset(i as isize) as *mut i32;
            let mut iA: *mut i32 =
                &mut *(*iwork.offset(Ai as isize)).offset(i as isize) as *mut i32;
            let mut reM: *mut f32 = *raw.offset(Mi as isize);
            let mut reA: *mut f32 = *raw.offset(Ai as isize);
            let mut qeM: *mut f32 = *quant.offset(Mi as isize);
            let mut qeA: *mut f32 = *quant.offset(Ai as isize);
            let mut floorM: *mut f32 = *floor_0.offset(Mi as isize);
            let mut floorA: *mut f32 = *floor_0.offset(Ai as isize);
            let mut fM: *mut i32 = *flag.offset(Mi as isize);
            let mut fA: *mut i32 = *flag.offset(Ai as isize);
            if *nz.offset(Mi as isize) != 0 || *nz.offset(Ai as isize) != 0 {
                let ref mut fresh38 = *nz.offset(Ai as isize);
                *fresh38 = 1 as i32;
                *nz.offset(Mi as isize) = *fresh38;
                j = 0 as i32;
                while j < jn {
                    if j < sliding_lowpass - i {
                        if *fM.offset(j as isize) != 0 || *fA.offset(j as isize) != 0 {
                            /* lossless coupling */
                            *reM.offset(j as isize) =
                                (crate::stdlib::fabs(*reM.offset(j as isize) as f64)
                                    + crate::stdlib::fabs(*reA.offset(j as isize) as f64))
                                    as f32;
                            *qeM.offset(j as isize) =
                                *qeM.offset(j as isize) + *qeA.offset(j as isize);
                            let ref mut fresh39 = *fA.offset(j as isize);
                            *fresh39 = 1 as i32;
                            *fM.offset(j as isize) = *fresh39;
                            /* couple iM/iA */
                            let mut A: i32 = *iM.offset(j as isize);
                            let mut B: i32 = *iA.offset(j as isize);
                            if ::libc::abs(A) > ::libc::abs(B) {
                                *iA.offset(j as isize) =
                                    if A > 0 as i32 { (A) - B } else { (B) - A }
                            } else {
                                *iA.offset(j as isize) =
                                    if B > 0 as i32 { (A) - B } else { (B) - A };
                                *iM.offset(j as isize) = B
                            }
                            /* collapse two equivalent tuples to one */
                            if *iA.offset(j as isize)
                                >= ::libc::abs(*iM.offset(j as isize)) * 2 as i32
                            {
                                *iA.offset(j as isize) = -*iA.offset(j as isize);
                                *iM.offset(j as isize) = -*iM.offset(j as isize)
                            }
                        } else {
                            /* lossy (point) coupling */
                            if j < limit - i {
                                /* dipole */
                                *reM.offset(j as isize) += *reA.offset(j as isize);
                                *qeM.offset(j as isize) =
                                    crate::stdlib::fabs(*reM.offset(j as isize) as f64) as f32
                            } else if *reM.offset(j as isize) + *reA.offset(j as isize)
                                < 0 as i32 as f32
                            {
                                let ref mut fresh40 = *qeM.offset(j as isize);
                                *fresh40 = (crate::stdlib::fabs(*reM.offset(j as isize) as f64)
                                    + crate::stdlib::fabs(*reA.offset(j as isize) as f64))
                                    as f32;
                                *reM.offset(j as isize) = -*fresh40
                            } else {
                                let ref mut fresh41 = *qeM.offset(j as isize);
                                *fresh41 = (crate::stdlib::fabs(*reM.offset(j as isize) as f64)
                                    + crate::stdlib::fabs(*reA.offset(j as isize) as f64))
                                    as f32;
                                *reM.offset(j as isize) = *fresh41
                            }
                            let ref mut fresh42 = *qeA.offset(j as isize);
                            *fresh42 = 0.0f32;
                            *reA.offset(j as isize) = *fresh42;
                            *fA.offset(j as isize) = 1 as i32;
                            *iA.offset(j as isize) = 0 as i32
                        }
                    }
                    let ref mut fresh43 = *floorA.offset(j as isize);
                    *fresh43 = *floorM.offset(j as isize) + *floorA.offset(j as isize);
                    *floorM.offset(j as isize) = *fresh43;
                    j += 1
                }
                /* elliptical */
                /* normalize the resulting mag vector */
                *acc.offset(track as isize) = noise_normalize(
                    p,
                    limit,
                    *raw.offset(Mi as isize),
                    *quant.offset(Mi as isize),
                    *floor_0.offset(Mi as isize),
                    *flag.offset(Mi as isize),
                    *acc.offset(track as isize),
                    i,
                    jn,
                    iM,
                );
                track += 1
            }
            step += 1
        }
        i += partition
    }
    i = 0 as i32;
    while i < (*vi).coupling_steps {
        /* make sure coupling a zero and a nonzero channel results in two
        nonzero channels. */
        if *nonzero.offset((*vi).coupling_mag[i as usize] as isize) != 0
            || *nonzero.offset((*vi).coupling_ang[i as usize] as isize) != 0
        {
            *nonzero.offset((*vi).coupling_mag[i as usize] as isize) = 1 as i32;
            *nonzero.offset((*vi).coupling_ang[i as usize] as isize) = 1 as i32
        }
        i += 1
    }
}

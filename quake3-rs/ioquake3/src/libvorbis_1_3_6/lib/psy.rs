// =============== BEGIN psy_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_info_psy {
    pub blockflag: libc::c_int,
    pub ath_adjatt: libc::c_float,
    pub ath_maxatt: libc::c_float,
    pub tone_masteratt: [libc::c_float; 3],
    pub tone_centerboost: libc::c_float,
    pub tone_decay: libc::c_float,
    pub tone_abs_limit: libc::c_float,
    pub toneatt: [libc::c_float; 17],
    pub noisemaskp: libc::c_int,
    pub noisemaxsupp: libc::c_float,
    pub noisewindowlo: libc::c_float,
    pub noisewindowhi: libc::c_float,
    pub noisewindowlomin: libc::c_int,
    pub noisewindowhimin: libc::c_int,
    pub noisewindowfixed: libc::c_int,
    pub noiseoff: [[libc::c_float; 17]; 3],
    pub noisecompand: [libc::c_float; 40],
    pub max_curve_dB: libc::c_float,
    pub normal_p: libc::c_int,
    pub normal_start: libc::c_int,
    pub normal_partition: libc::c_int,
    pub normal_thresh: libc::c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_info_psy_global {
    pub eighth_octave_lines: libc::c_int,
    pub preecho_thresh: [libc::c_float; 7],
    pub postecho_thresh: [libc::c_float; 7],
    pub stretch_penalty: libc::c_float,
    pub preecho_minenergy: libc::c_float,
    pub ampmax_att_per_sec: libc::c_float,
    pub coupling_pkHz: [libc::c_int; 15],
    pub coupling_pointlimit: [[libc::c_int; 15]; 2],
    pub coupling_prepointamp: [libc::c_int; 15],
    pub coupling_postpointamp: [libc::c_int; 15],
    pub sliding_lowpass: [[libc::c_int; 15]; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_psy_global {
    pub ampmax: libc::c_float,
    pub channels: libc::c_int,
    pub gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    pub coupling_pointlimit: [[libc::c_int; 3]; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_psy {
    pub n: libc::c_int,
    pub vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy,
    pub tonecurves: *mut *mut *mut libc::c_float,
    pub noiseoffset: *mut *mut libc::c_float,
    pub ath: *mut libc::c_float,
    pub octave: *mut libc::c_long,
    pub bark: *mut libc::c_long,
    pub firstoc: libc::c_long,
    pub shiftoc: libc::c_long,
    pub eighth_octave_lines: libc::c_int,
    pub total_octave_lines: libc::c_int,
    pub rate: libc::c_long,
    pub m_val: libc::c_float,
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

    pub unsafe extern "C" fn unitnorm(mut x: libc::c_float) -> libc::c_float {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = x;
        ix.i = ix.i & 0x80000000 as libc::c_uint | 0x3f800000 as libc::c_uint;
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

    pub static mut ATH: [libc::c_float; 88] = [
        -(51 as libc::c_int) as libc::c_float,
        -(52 as libc::c_int) as libc::c_float,
        -(53 as libc::c_int) as libc::c_float,
        -(54 as libc::c_int) as libc::c_float,
        -(55 as libc::c_int) as libc::c_float,
        -(56 as libc::c_int) as libc::c_float,
        -(57 as libc::c_int) as libc::c_float,
        -(58 as libc::c_int) as libc::c_float,
        -(59 as libc::c_int) as libc::c_float,
        -(60 as libc::c_int) as libc::c_float,
        -(61 as libc::c_int) as libc::c_float,
        -(62 as libc::c_int) as libc::c_float,
        -(63 as libc::c_int) as libc::c_float,
        -(64 as libc::c_int) as libc::c_float,
        -(65 as libc::c_int) as libc::c_float,
        -(66 as libc::c_int) as libc::c_float,
        -(67 as libc::c_int) as libc::c_float,
        -(68 as libc::c_int) as libc::c_float,
        -(69 as libc::c_int) as libc::c_float,
        -(70 as libc::c_int) as libc::c_float,
        -(71 as libc::c_int) as libc::c_float,
        -(72 as libc::c_int) as libc::c_float,
        -(73 as libc::c_int) as libc::c_float,
        -(74 as libc::c_int) as libc::c_float,
        -(75 as libc::c_int) as libc::c_float,
        -(76 as libc::c_int) as libc::c_float,
        -(77 as libc::c_int) as libc::c_float,
        -(78 as libc::c_int) as libc::c_float,
        -(80 as libc::c_int) as libc::c_float,
        -(81 as libc::c_int) as libc::c_float,
        -(82 as libc::c_int) as libc::c_float,
        -(83 as libc::c_int) as libc::c_float,
        -(84 as libc::c_int) as libc::c_float,
        -(85 as libc::c_int) as libc::c_float,
        -(86 as libc::c_int) as libc::c_float,
        -(87 as libc::c_int) as libc::c_float,
        -(88 as libc::c_int) as libc::c_float,
        -(88 as libc::c_int) as libc::c_float,
        -(89 as libc::c_int) as libc::c_float,
        -(89 as libc::c_int) as libc::c_float,
        -(90 as libc::c_int) as libc::c_float,
        -(91 as libc::c_int) as libc::c_float,
        -(91 as libc::c_int) as libc::c_float,
        -(92 as libc::c_int) as libc::c_float,
        -(93 as libc::c_int) as libc::c_float,
        -(94 as libc::c_int) as libc::c_float,
        -(95 as libc::c_int) as libc::c_float,
        -(96 as libc::c_int) as libc::c_float,
        -(96 as libc::c_int) as libc::c_float,
        -(97 as libc::c_int) as libc::c_float,
        -(98 as libc::c_int) as libc::c_float,
        -(98 as libc::c_int) as libc::c_float,
        -(99 as libc::c_int) as libc::c_float,
        -(99 as libc::c_int) as libc::c_float,
        -(100 as libc::c_int) as libc::c_float,
        -(100 as libc::c_int) as libc::c_float,
        -(101 as libc::c_int) as libc::c_float,
        -(102 as libc::c_int) as libc::c_float,
        -(103 as libc::c_int) as libc::c_float,
        -(104 as libc::c_int) as libc::c_float,
        -(106 as libc::c_int) as libc::c_float,
        -(107 as libc::c_int) as libc::c_float,
        -(107 as libc::c_int) as libc::c_float,
        -(107 as libc::c_int) as libc::c_float,
        -(107 as libc::c_int) as libc::c_float,
        -(105 as libc::c_int) as libc::c_float,
        -(103 as libc::c_int) as libc::c_float,
        -(102 as libc::c_int) as libc::c_float,
        -(101 as libc::c_int) as libc::c_float,
        -(99 as libc::c_int) as libc::c_float,
        -(98 as libc::c_int) as libc::c_float,
        -(96 as libc::c_int) as libc::c_float,
        -(95 as libc::c_int) as libc::c_float,
        -(95 as libc::c_int) as libc::c_float,
        -(96 as libc::c_int) as libc::c_float,
        -(97 as libc::c_int) as libc::c_float,
        -(96 as libc::c_int) as libc::c_float,
        -(95 as libc::c_int) as libc::c_float,
        -(93 as libc::c_int) as libc::c_float,
        -(90 as libc::c_int) as libc::c_float,
        -(80 as libc::c_int) as libc::c_float,
        -(70 as libc::c_int) as libc::c_float,
        -(50 as libc::c_int) as libc::c_float,
        -(40 as libc::c_int) as libc::c_float,
        -(30 as libc::c_int) as libc::c_float,
        -(30 as libc::c_int) as libc::c_float,
        -(30 as libc::c_int) as libc::c_float,
        -(30 as libc::c_int) as libc::c_float,
    ];
    /* masking tones from -50 to 0dB, 62.5 through 16kHz at half octaves
    test tones from -2 octaves to +5 octaves sampled at eighth octaves */
    /* (Vorbis 0dB, the loudest possible tone, is assumed to be ~100dB SPL
    for collection of these curves) */

    pub static mut tonemasks: [[[libc::c_float; 56]; 6]; 17] = [
        [
            [
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(26 as libc::c_int) as libc::c_float,
                -(27 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(17 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(20 as libc::c_int) as libc::c_float,
                -(22 as libc::c_int) as libc::c_float,
                -(26 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(8 as libc::c_int) as libc::c_float,
                -(10 as libc::c_int) as libc::c_float,
                -(11 as libc::c_int) as libc::c_float,
                -(15 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(25 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(116 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(19 as libc::c_int) as libc::c_float,
                -(20 as libc::c_int) as libc::c_float,
                -(21 as libc::c_int) as libc::c_float,
                -(23 as libc::c_int) as libc::c_float,
                -(27 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(9 as libc::c_int) as libc::c_float,
                -(11 as libc::c_int) as libc::c_float,
                -(12 as libc::c_int) as libc::c_float,
                -(12 as libc::c_int) as libc::c_float,
                -(15 as libc::c_int) as libc::c_float,
                -(16 as libc::c_int) as libc::c_float,
                -(20 as libc::c_int) as libc::c_float,
                -(23 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(28 as libc::c_int) as libc::c_float,
                -(26 as libc::c_int) as libc::c_float,
                -(24 as libc::c_int) as libc::c_float,
                -(22 as libc::c_int) as libc::c_float,
                -(20 as libc::c_int) as libc::c_float,
                -(20 as libc::c_int) as libc::c_float,
                -(23 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(27 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(65 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(32 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(117 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(118 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(111 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(111 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(111 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(36 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(111 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(116 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(28 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(114 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(111 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(31 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(76 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(41 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(103 as libc::c_int) as libc::c_float,
                -(107 as libc::c_int) as libc::c_float,
                -(112 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(113 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(97 as libc::c_int) as libc::c_float,
                -(106 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(116 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(116 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(116 as libc::c_int) as libc::c_float,
                -(109 as libc::c_int) as libc::c_float,
                -(102 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(75 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(72 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(57 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(89 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(101 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(51 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(59 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(95 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(120 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(120 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(96 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(120 as libc::c_int) as libc::c_float,
                -(105 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(92 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(120 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(79 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(64 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(118 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(65 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(115 as libc::c_int) as libc::c_float,
                -(98 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(44 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(49 as libc::c_int) as libc::c_float,
                -(46 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(39 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(82 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(90 as libc::c_int) as libc::c_float,
                -(94 as libc::c_int) as libc::c_float,
                -(99 as libc::c_int) as libc::c_float,
                -(104 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(66 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(81 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(93 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(88 as libc::c_int) as libc::c_float,
                -(61 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(77 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(85 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(63 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(62 as libc::c_int) as libc::c_float,
                -(58 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(52 as libc::c_int) as libc::c_float,
                -(54 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(118 as libc::c_int) as libc::c_float,
                -(108 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(50 as libc::c_int) as libc::c_float,
                -(55 as libc::c_int) as libc::c_float,
                -(47 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(40 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(118 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(73 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(42 as libc::c_int) as libc::c_float,
                -(43 as libc::c_int) as libc::c_float,
                -(53 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
        ],
        [
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(80 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(91 as libc::c_int) as libc::c_float,
                -(84 as libc::c_int) as libc::c_float,
                -(74 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(68 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(70 as libc::c_int) as libc::c_float,
                -(60 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(30 as libc::c_int) as libc::c_float,
                -(21 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(87 as libc::c_int) as libc::c_float,
                -(78 as libc::c_int) as libc::c_float,
                -(67 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(21 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(86 as libc::c_int) as libc::c_float,
                -(69 as libc::c_int) as libc::c_float,
                -(56 as libc::c_int) as libc::c_float,
                -(45 as libc::c_int) as libc::c_float,
                -(35 as libc::c_int) as libc::c_float,
                -(33 as libc::c_int) as libc::c_float,
                -(29 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
            ],
            [
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(110 as libc::c_int) as libc::c_float,
                -(100 as libc::c_int) as libc::c_float,
                -(83 as libc::c_int) as libc::c_float,
                -(71 as libc::c_int) as libc::c_float,
                -(48 as libc::c_int) as libc::c_float,
                -(27 as libc::c_int) as libc::c_float,
                -(38 as libc::c_int) as libc::c_float,
                -(37 as libc::c_int) as libc::c_float,
                -(34 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
                -(999 as libc::c_int) as libc::c_float,
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

static mut stereo_threshholds: [libc::c_double; 9] = [
    0.0f64, 0.5f64, 1.0f64, 1.5f64, 2.5f64, 4.5f64, 8.5f64, 16.5f64, 9e10f64,
];

static mut stereo_threshholds_limited: [libc::c_double; 9] = [
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
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global>()
                as libc::c_ulong,
        ) as *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
    (*look).channels = (*vi).channels;
    (*look).ampmax = -9999.0f64 as libc::c_float;
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
            0 as libc::c_int,
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
            0 as libc::c_int,
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
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy>()
                as libc::c_ulong,
        );
        ::libc::free(i as *mut libc::c_void);
    };
}

unsafe extern "C" fn min_curve(mut c: *mut libc::c_float, mut c2: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 56 as libc::c_int {
        if *c2.offset(i as isize) < *c.offset(i as isize) {
            *c.offset(i as isize) = *c2.offset(i as isize)
        }
        i += 1
    }
}

unsafe extern "C" fn max_curve(mut c: *mut libc::c_float, mut c2: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 56 as libc::c_int {
        if *c2.offset(i as isize) > *c.offset(i as isize) {
            *c.offset(i as isize) = *c2.offset(i as isize)
        }
        i += 1
    }
}

unsafe extern "C" fn attenuate_curve(mut c: *mut libc::c_float, mut att: libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 56 as libc::c_int {
        *c.offset(i as isize) += att;
        i += 1
    }
}

unsafe extern "C" fn setup_tone_curves(
    mut curveatt_dB: *mut libc::c_float,
    mut binHz: libc::c_float,
    mut n: libc::c_int,
    mut center_boost: libc::c_float,
    mut center_decay_rate: libc::c_float,
) -> *mut *mut *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut ath: [libc::c_float; 56] = [0.; 56];
    let mut workc: [[[libc::c_float; 56]; 8]; 17] = [[[0.; 56]; 8]; 17];
    let mut athc: [[libc::c_float; 56]; 8] = [[0.; 56]; 8];
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut brute_buffer: *mut libc::c_float = fresh0.as_mut_ptr() as *mut libc::c_float;
    let mut ret: *mut *mut *mut libc::c_float = crate::stdlib::malloc(
        (::std::mem::size_of::<*mut *mut libc::c_float>() as libc::c_ulong)
            .wrapping_mul(17 as libc::c_int as libc::c_ulong),
    ) as *mut *mut *mut libc::c_float;
    crate::stdlib::memset(
        workc.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[[libc::c_float; 56]; 8]; 17]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 17 as libc::c_int {
        /* we add back in the ATH to avoid low level curves falling off to
        -infinity and unnecessarily cutting off high level curves in the
        curve limiting (last step). */
        /* A half-band's settings must be valid over the whole band, and
        it's better to mask too little than too much */
        let mut ath_offset: libc::c_int = i * 4 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 56 as libc::c_int {
            let mut min: libc::c_float = 999.0f64 as libc::c_float;
            k = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                if j + k + ath_offset < 88 as libc::c_int {
                    if min > ATH[(j + k + ath_offset) as usize] {
                        min = ATH[(j + k + ath_offset) as usize]
                    }
                } else if min > ATH[(88 as libc::c_int - 1 as libc::c_int) as usize] {
                    min = ATH[(88 as libc::c_int - 1 as libc::c_int) as usize]
                }
                k += 1
            }
            ath[j as usize] = min;
            j += 1
        }
        /* copy curves into working space, replicate the 50dB curve to 30
        and 40, replicate the 100dB curve to 110 */
        j = 0 as libc::c_int;
        while j < 6 as libc::c_int {
            crate::stdlib::memcpy(
                workc[i as usize][(j + 2 as libc::c_int) as usize].as_mut_ptr()
                    as *mut libc::c_void,
                tonemasks[i as usize][j as usize].as_ptr() as *const libc::c_void,
                (56 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            );
            j += 1
        }
        crate::stdlib::memcpy(
            workc[i as usize][0 as libc::c_int as usize].as_mut_ptr() as *mut libc::c_void,
            tonemasks[i as usize][0 as libc::c_int as usize].as_ptr() as *const libc::c_void,
            (56 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        crate::stdlib::memcpy(
            workc[i as usize][1 as libc::c_int as usize].as_mut_ptr() as *mut libc::c_void,
            tonemasks[i as usize][0 as libc::c_int as usize].as_ptr() as *const libc::c_void,
            (56 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        );
        /* apply centered curve boost/decay */
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            k = 0 as libc::c_int;
            while k < 56 as libc::c_int {
                let mut adj: libc::c_float = center_boost
                    + ::libc::abs(16 as libc::c_int - k) as libc::c_float * center_decay_rate;
                if (adj as libc::c_double) < 0.0f64
                    && center_boost > 0 as libc::c_int as libc::c_float
                {
                    adj = 0.0f64 as libc::c_float
                }
                if adj as libc::c_double > 0.0f64
                    && center_boost < 0 as libc::c_int as libc::c_float
                {
                    adj = 0.0f64 as libc::c_float
                }
                workc[i as usize][j as usize][k as usize] += adj;
                k += 1
            }
            j += 1
        }
        /* normalize curves so the driving amplitude is 0dB */
        /* make temp curves with the ATH overlayed */
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            attenuate_curve(
                workc[i as usize][j as usize].as_mut_ptr(),
                (*curveatt_dB.offset(i as isize) as libc::c_double + 100.0f64
                    - (if j < 2 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        j
                    }) as libc::c_double
                        * 10.0f64
                    - 30.0f64) as libc::c_float,
            );
            crate::stdlib::memcpy(
                athc[j as usize].as_mut_ptr() as *mut libc::c_void,
                ath.as_mut_ptr() as *const libc::c_void,
                (56 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
            );
            attenuate_curve(
                athc[j as usize].as_mut_ptr(),
                (100.0f64 - (j as libc::c_float * 10.0f32) as libc::c_double - 30.0f64)
                    as libc::c_float,
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
        j = 1 as libc::c_int;
        while j < 8 as libc::c_int {
            min_curve(
                athc[j as usize].as_mut_ptr(),
                athc[(j - 1 as libc::c_int) as usize].as_mut_ptr(),
            );
            min_curve(
                workc[i as usize][j as usize].as_mut_ptr(),
                athc[j as usize].as_mut_ptr(),
            );
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 17 as libc::c_int {
        let mut hi_curve: libc::c_int = 0;
        let mut lo_curve: libc::c_int = 0;
        let mut bin: libc::c_int = 0;
        let ref mut fresh1 = *ret.offset(i as isize);
        *fresh1 = crate::stdlib::malloc(
            (::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as *mut *mut libc::c_float;
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
            crate::stdlib::exp(
                (i as libc::c_double * 0.5f64 + 5.965784f32 as libc::c_double)
                    * 0.693147f32 as libc::c_double,
            ) / binHz as libc::c_double,
        ) as libc::c_int;
        lo_curve = crate::stdlib::ceil(
            (crate::stdlib::log(
                (bin as libc::c_float * binHz + 1 as libc::c_int as libc::c_float)
                    as libc::c_double,
            ) * 1.442695f32 as libc::c_double
                - 5.965784f32 as libc::c_double)
                * 2 as libc::c_int as libc::c_double,
        ) as libc::c_int;
        hi_curve = crate::stdlib::floor(
            (crate::stdlib::log(
                ((bin + 1 as libc::c_int) as libc::c_float * binHz) as libc::c_double,
            ) * 1.442695f32 as libc::c_double
                - 5.965784f32 as libc::c_double)
                * 2 as libc::c_int as libc::c_double,
        ) as libc::c_int;
        if lo_curve > i {
            lo_curve = i
        }
        if lo_curve < 0 as libc::c_int {
            lo_curve = 0 as libc::c_int
        }
        if hi_curve >= 17 as libc::c_int {
            hi_curve = 17 as libc::c_int - 1 as libc::c_int
        }
        m = 0 as libc::c_int;
        while m < 8 as libc::c_int {
            let ref mut fresh2 = *(*ret.offset(i as isize)).offset(m as isize);
            *fresh2 = crate::stdlib::malloc(
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul((56 as libc::c_int + 2 as libc::c_int) as libc::c_ulong),
            ) as *mut libc::c_float;
            j = 0 as libc::c_int;
            while j < n {
                *brute_buffer.offset(j as isize) = 999.0f64 as libc::c_float;
                j += 1
            }
            /* render the curve into bins, then pull values back into curve.
            The point is that any inherent subsampling aliasing results in
            a safe minimum */
            k = lo_curve;
            while k <= hi_curve {
                let mut l: libc::c_int = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < 56 as libc::c_int {
                    let mut lo_bin: libc::c_int = (crate::stdlib::exp(
                        (j as libc::c_double * 0.125f64 + k as libc::c_double * 0.5f64 - 2.0625f64
                            + 5.965784f32 as libc::c_double)
                            * 0.693147f32 as libc::c_double,
                    ) / binHz as libc::c_double)
                        as libc::c_int;
                    let mut hi_bin: libc::c_int = (crate::stdlib::exp(
                        (j as libc::c_double * 0.125f64 + k as libc::c_double * 0.5f64 - 1.9375f64
                            + 5.965784f32 as libc::c_double)
                            * 0.693147f32 as libc::c_double,
                    ) / binHz as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                        as libc::c_int;
                    if lo_bin < 0 as libc::c_int {
                        lo_bin = 0 as libc::c_int
                    }
                    if lo_bin > n {
                        lo_bin = n
                    }
                    if lo_bin < l {
                        l = lo_bin
                    }
                    if hi_bin < 0 as libc::c_int {
                        hi_bin = 0 as libc::c_int
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
                        > workc[k as usize][m as usize]
                            [(56 as libc::c_int - 1 as libc::c_int) as usize]
                    {
                        *brute_buffer.offset(l as isize) = workc[k as usize][m as usize]
                            [(56 as libc::c_int - 1 as libc::c_int) as usize]
                    }
                    l += 1
                }
                k += 1
            }
            /* be equally paranoid about being valid up to next half ocatve */
            if (i + 1 as libc::c_int) < 17 as libc::c_int {
                let mut l_0: libc::c_int = 0 as libc::c_int;
                k = i + 1 as libc::c_int;
                j = 0 as libc::c_int;
                while j < 56 as libc::c_int {
                    let mut lo_bin_0: libc::c_int = (crate::stdlib::exp(
                        (j as libc::c_double * 0.125f64 + i as libc::c_double * 0.5f64 - 2.0625f64
                            + 5.965784f32 as libc::c_double)
                            * 0.693147f32 as libc::c_double,
                    ) / binHz as libc::c_double)
                        as libc::c_int;
                    let mut hi_bin_0: libc::c_int = (crate::stdlib::exp(
                        (j as libc::c_double * 0.125f64 + i as libc::c_double * 0.5f64 - 1.9375f64
                            + 5.965784f32 as libc::c_double)
                            * 0.693147f32 as libc::c_double,
                    ) / binHz as libc::c_double
                        + 1 as libc::c_int as libc::c_double)
                        as libc::c_int;
                    if lo_bin_0 < 0 as libc::c_int {
                        lo_bin_0 = 0 as libc::c_int
                    }
                    if lo_bin_0 > n {
                        lo_bin_0 = n
                    }
                    if lo_bin_0 < l_0 {
                        l_0 = lo_bin_0
                    }
                    if hi_bin_0 < 0 as libc::c_int {
                        hi_bin_0 = 0 as libc::c_int
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
                        > workc[k as usize][m as usize]
                            [(56 as libc::c_int - 1 as libc::c_int) as usize]
                    {
                        *brute_buffer.offset(l_0 as isize) = workc[k as usize][m as usize]
                            [(56 as libc::c_int - 1 as libc::c_int) as usize]
                    }
                    l_0 += 1
                }
            }
            j = 0 as libc::c_int;
            while j < 56 as libc::c_int {
                let mut bin_0: libc::c_int = (crate::stdlib::exp(
                    (j as libc::c_double * 0.125f64 + i as libc::c_double * 0.5f64 - 2.0f64
                        + 5.965784f32 as libc::c_double)
                        * 0.693147f32 as libc::c_double,
                ) / binHz as libc::c_double)
                    as libc::c_int;
                if bin_0 < 0 as libc::c_int {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as libc::c_int) as isize) = -999.0f64 as libc::c_float
                } else if bin_0 >= n {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as libc::c_int) as isize) = -999.0f64 as libc::c_float
                } else {
                    *(*(*ret.offset(i as isize)).offset(m as isize))
                        .offset((j + 2 as libc::c_int) as isize) =
                        *brute_buffer.offset(bin_0 as isize)
                }
                j += 1
            }
            /* add fenceposts */
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                if *(*(*ret.offset(i as isize)).offset(m as isize))
                    .offset((j + 2 as libc::c_int) as isize)
                    > -200.0f32
                {
                    break;
                }
                j += 1
            }
            *(*(*ret.offset(i as isize)).offset(m as isize)).offset(0 as libc::c_int as isize) =
                j as libc::c_float;
            j = 56 as libc::c_int - 1 as libc::c_int;
            while j > 16 as libc::c_int + 1 as libc::c_int {
                if *(*(*ret.offset(i as isize)).offset(m as isize))
                    .offset((j + 2 as libc::c_int) as isize)
                    > -200.0f32
                {
                    break;
                }
                j -= 1
            }
            *(*(*ret.offset(i as isize)).offset(m as isize)).offset(1 as libc::c_int as isize) =
                j as libc::c_float;
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
    mut n: libc::c_int,
    mut rate: libc::c_long,
) {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut lo: libc::c_long = -(99 as libc::c_int) as libc::c_long;
    let mut hi: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut maxoc: libc::c_long = 0;
    crate::stdlib::memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy>()
            as libc::c_ulong,
    );
    (*p).eighth_octave_lines = (*gi).eighth_octave_lines;
    (*p).shiftoc = (crate::stdlib::rint(
        crate::stdlib::log(((*gi).eighth_octave_lines as libc::c_float * 8.0f32) as libc::c_double)
            / crate::stdlib::log(2.0f32 as libc::c_double),
    ) - 1 as libc::c_int as libc::c_double) as libc::c_long;
    (*p).firstoc = ((crate::stdlib::log(
        (0.25f32 * rate as libc::c_float) as libc::c_double * 0.5f64 / n as libc::c_double,
    ) * 1.442695f32 as libc::c_double
        - 5.965784f32 as libc::c_double)
        * ((1 as libc::c_int) << (*p).shiftoc + 1 as libc::c_int as libc::c_long) as libc::c_double
        - (*gi).eighth_octave_lines as libc::c_double) as libc::c_long;
    maxoc = ((crate::stdlib::log(
        ((n as libc::c_float + 0.25f32) * rate as libc::c_float) as libc::c_double * 0.5f64
            / n as libc::c_double,
    ) * 1.442695f32 as libc::c_double
        - 5.965784f32 as libc::c_double)
        * ((1 as libc::c_int) << (*p).shiftoc + 1 as libc::c_int as libc::c_long) as libc::c_double
        + 0.5f32 as libc::c_double) as libc::c_long;
    (*p).total_octave_lines =
        (maxoc - (*p).firstoc + 1 as libc::c_int as libc::c_long) as libc::c_int;
    (*p).ath = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    (*p).octave = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    (*p).bark = crate::stdlib::malloc(
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    ) as *mut libc::c_long;
    (*p).vi = vi;
    (*p).n = n;
    (*p).rate = rate;
    /* AoTuV HF weighting */
    (*p).m_val = 1.0f64 as libc::c_float; /* 48kHz */
    if rate < 26000 as libc::c_int as libc::c_long {
        (*p).m_val = 0 as libc::c_int as libc::c_float
    } else if rate < 38000 as libc::c_int as libc::c_long {
        (*p).m_val = 0.94f64 as libc::c_float
    } else if rate > 46000 as libc::c_int as libc::c_long {
        (*p).m_val = 1.275f64 as libc::c_float
    } /* 32kHz */
    /* set up the lookups for a given blocksize and sample rate */
    i = 0 as libc::c_int as libc::c_long;
    j = 0 as libc::c_int as libc::c_long;
    while i < (88 as libc::c_int - 1 as libc::c_int) as libc::c_long {
        let mut endpos: libc::c_int = crate::stdlib::rint(
            crate::stdlib::exp(
                ((i + 1 as libc::c_int as libc::c_long) as libc::c_double * 0.125f64 - 2.0f64
                    + 5.965784f32 as libc::c_double)
                    * 0.693147f32 as libc::c_double,
            ) * 2 as libc::c_int as libc::c_double
                * n as libc::c_double
                / rate as libc::c_double,
        ) as libc::c_int;
        let mut base: libc::c_float = ATH[i as usize];
        if j < endpos as libc::c_long {
            let mut delta: libc::c_float = (ATH[(i + 1 as libc::c_int as libc::c_long) as usize]
                - base)
                / (endpos as libc::c_long - j) as libc::c_float;
            while j < endpos as libc::c_long && j < n as libc::c_long {
                *(*p).ath.offset(j as isize) = (base as libc::c_double + 100.0f64) as libc::c_float;
                base += delta;
                j += 1
            }
        }
        i += 1
    }
    while j < n as libc::c_long {
        *(*p).ath.offset(j as isize) = *(*p)
            .ath
            .offset((j - 1 as libc::c_int as libc::c_long) as isize);
        j += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < n as libc::c_long {
        let mut bark: libc::c_float = (13.1f32 as libc::c_double
            * crate::stdlib::atan(
                (0.00074f32 * (rate / (2 as libc::c_int * n) as libc::c_long * i) as libc::c_float)
                    as libc::c_double,
            )
            + 2.24f32 as libc::c_double
                * crate::stdlib::atan(
                    ((rate / (2 as libc::c_int * n) as libc::c_long
                        * i
                        * (rate / (2 as libc::c_int * n) as libc::c_long * i))
                        as libc::c_float
                        * 1.85e-8f32) as libc::c_double,
                )
            + (1e-4f32 * (rate / (2 as libc::c_int * n) as libc::c_long * i) as libc::c_float)
                as libc::c_double) as libc::c_float;
        while (lo + (*vi).noisewindowlomin as libc::c_long) < i
            && (13.1f32 as libc::c_double
                * crate::stdlib::atan(
                    (0.00074f32
                        * (rate / (2 as libc::c_int * n) as libc::c_long * lo) as libc::c_float)
                        as libc::c_double,
                )
                + 2.24f32 as libc::c_double
                    * crate::stdlib::atan(
                        ((rate / (2 as libc::c_int * n) as libc::c_long
                            * lo
                            * (rate / (2 as libc::c_int * n) as libc::c_long * lo))
                            as libc::c_float
                            * 1.85e-8f32) as libc::c_double,
                    )
                + (1e-4f32 * (rate / (2 as libc::c_int * n) as libc::c_long * lo) as libc::c_float)
                    as libc::c_double)
                < (bark - (*vi).noisewindowlo) as libc::c_double
        {
            lo += 1
        }
        while hi <= n as libc::c_long
            && (hi < i + (*vi).noisewindowhimin as libc::c_long
                || (13.1f32 as libc::c_double
                    * crate::stdlib::atan(
                        (0.00074f32
                            * (rate / (2 as libc::c_int * n) as libc::c_long * hi) as libc::c_float)
                            as libc::c_double,
                    )
                    + 2.24f32 as libc::c_double
                        * crate::stdlib::atan(
                            ((rate / (2 as libc::c_int * n) as libc::c_long
                                * hi
                                * (rate / (2 as libc::c_int * n) as libc::c_long * hi))
                                as libc::c_float
                                * 1.85e-8f32) as libc::c_double,
                        )
                    + (1e-4f32
                        * (rate / (2 as libc::c_int * n) as libc::c_long * hi) as libc::c_float)
                        as libc::c_double)
                    < (bark + (*vi).noisewindowhi) as libc::c_double)
        {
            hi += 1
        }
        *(*p).bark.offset(i as isize) = ((lo - 1 as libc::c_int as libc::c_long)
            << 16 as libc::c_int)
            + (hi - 1 as libc::c_int as libc::c_long);
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < n as libc::c_long {
        *(*p).octave.offset(i as isize) = ((crate::stdlib::log(
            (i as libc::c_float + 0.25f32) as libc::c_double * 0.5f64 * rate as libc::c_double
                / n as libc::c_double,
        ) * 1.442695f32 as libc::c_double
            - 5.965784f32 as libc::c_double)
            * ((1 as libc::c_int) << (*p).shiftoc + 1 as libc::c_int as libc::c_long)
                as libc::c_double
            + 0.5f32 as libc::c_double) as libc::c_long;
        i += 1
    }
    (*p).tonecurves = setup_tone_curves(
        (*vi).toneatt.as_mut_ptr(),
        (rate as libc::c_double * 0.5f64 / n as libc::c_double) as libc::c_float,
        n,
        (*vi).tone_centerboost,
        (*vi).tone_decay,
    );
    /* set up rolling noise median */
    (*p).noiseoffset = crate::stdlib::malloc(
        (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong),
    ) as *mut *mut libc::c_float;
    i = 0 as libc::c_int as libc::c_long;
    while i < 3 as libc::c_int as libc::c_long {
        let ref mut fresh3 = *(*p).noiseoffset.offset(i as isize);
        *fresh3 = crate::stdlib::malloc(
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
        ) as *mut libc::c_float;
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < n as libc::c_long {
        let mut halfoc: libc::c_float = ((crate::stdlib::log(
            (i as libc::c_double + 0.5f64) * rate as libc::c_double
                / (2.0f64 * n as libc::c_double),
        ) * 1.442695f32 as libc::c_double
            - 5.965784f32 as libc::c_double)
            * 2.0f64) as libc::c_float;
        let mut inthalfoc: libc::c_int = 0;
        let mut del: libc::c_float = 0.;
        if halfoc < 0 as libc::c_int as libc::c_float {
            halfoc = 0 as libc::c_int as libc::c_float
        }
        if halfoc >= (17 as libc::c_int - 1 as libc::c_int) as libc::c_float {
            halfoc = (17 as libc::c_int - 1 as libc::c_int) as libc::c_float
        }
        inthalfoc = halfoc as libc::c_int;
        del = halfoc - inthalfoc as libc::c_float;
        j = 0 as libc::c_int as libc::c_long;
        while j < 3 as libc::c_int as libc::c_long {
            *(*(*p).noiseoffset.offset(j as isize)).offset(i as isize) =
                ((*(*p).vi).noiseoff[j as usize][inthalfoc as usize] as libc::c_double
                    * (1.0f64 - del as libc::c_double)
                    + ((*(*p).vi).noiseoff[j as usize][(inthalfoc + 1 as libc::c_int) as usize]
                        * del) as libc::c_double) as libc::c_float;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_psy_clear(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
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
            i = 0 as libc::c_int;
            while i < 17 as libc::c_int {
                j = 0 as libc::c_int;
                while j < 8 as libc::c_int {
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
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                ::libc::free(*(*p).noiseoffset.offset(i as isize) as *mut libc::c_void);
                i += 1
            }
            ::libc::free((*p).noiseoffset as *mut libc::c_void);
        }
        crate::stdlib::memset(
            p as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy>()
                as libc::c_ulong,
        );
    };
}
/* octave/(8*eighth_octave_lines) x scale and dB y scale */

unsafe extern "C" fn seed_curve(
    mut seed: *mut libc::c_float,
    mut curves: *mut *const libc::c_float,
    mut amp: libc::c_float,
    mut oc: libc::c_int,
    mut n: libc::c_int,
    mut linesper: libc::c_int,
    mut dBoffset: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut post1: libc::c_int = 0;
    let mut seedptr: libc::c_int = 0;
    let mut posts: *const libc::c_float = 0 as *const libc::c_float;
    let mut curve: *const libc::c_float = 0 as *const libc::c_float;
    let mut choice: libc::c_int =
        (((amp + dBoffset) as libc::c_double - 30.0f64) * 0.1f32 as libc::c_double) as libc::c_int;
    choice = if choice < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        choice
    };
    choice = if choice > 8 as libc::c_int - 1 as libc::c_int {
        (8 as libc::c_int) - 1 as libc::c_int
    } else {
        choice
    };
    posts = *curves.offset(choice as isize);
    curve = posts.offset(2 as libc::c_int as isize);
    post1 = *posts.offset(1 as libc::c_int as isize) as libc::c_int;
    seedptr = (oc as libc::c_float
        + (*posts.offset(0 as libc::c_int as isize) - 16 as libc::c_int as libc::c_float)
            * linesper as libc::c_float
        - (linesper >> 1 as libc::c_int) as libc::c_float) as libc::c_int;
    i = *posts.offset(0 as libc::c_int as isize) as libc::c_int;
    while i < post1 {
        if seedptr > 0 as libc::c_int {
            let mut lin: libc::c_float = amp + *curve.offset(i as isize);
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
    mut curves: *mut *mut *const libc::c_float,
    mut f: *const libc::c_float,
    mut flr: *const libc::c_float,
    mut seed: *mut libc::c_float,
    mut specmax: libc::c_float,
) {
    let mut vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy = (*p).vi;
    let mut n: libc::c_long = (*p).n as libc::c_long;
    let mut i: libc::c_long = 0;
    let mut dBoffset: libc::c_float = (*vi).max_curve_dB - specmax;
    /* prime the working vector with peak values */
    i = 0 as libc::c_int as libc::c_long;
    while i < n {
        let mut max: libc::c_float = *f.offset(i as isize);
        let mut oc: libc::c_long = *(*p).octave.offset(i as isize);
        while (i + 1 as libc::c_int as libc::c_long) < n
            && *(*p)
                .octave
                .offset((i + 1 as libc::c_int as libc::c_long) as isize)
                == oc
        {
            i += 1;
            if *f.offset(i as isize) > max {
                max = *f.offset(i as isize)
            }
        }
        if max + 6.0f32 > *flr.offset(i as isize) {
            oc = oc >> (*p).shiftoc;
            if oc >= 17 as libc::c_int as libc::c_long {
                oc = (17 as libc::c_int - 1 as libc::c_int) as libc::c_long
            }
            if oc < 0 as libc::c_int as libc::c_long {
                oc = 0 as libc::c_int as libc::c_long
            }
            seed_curve(
                seed,
                *curves.offset(oc as isize),
                max,
                (*(*p).octave.offset(i as isize) - (*p).firstoc) as libc::c_int,
                (*p).total_octave_lines,
                (*p).eighth_octave_lines,
                dBoffset,
            );
        }
        i += 1
    }
}

unsafe extern "C" fn seed_chase(
    mut seeds: *mut libc::c_float,
    mut linesper: libc::c_int,
    mut n: libc::c_long,
) {
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            as usize,
    );
    let mut posstack: *mut libc::c_long = fresh4.as_mut_ptr() as *mut libc::c_long;
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut ampstack: *mut libc::c_float = fresh5.as_mut_ptr() as *mut libc::c_float;
    let mut stack: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pos: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut i: libc::c_long = 0;
    i = 0 as libc::c_int as libc::c_long;
    while i < n {
        if stack < 2 as libc::c_int as libc::c_long {
            *posstack.offset(stack as isize) = i;
            let fresh6 = stack;
            stack = stack + 1;
            *ampstack.offset(fresh6 as isize) = *seeds.offset(i as isize)
        } else {
            loop {
                if *seeds.offset(i as isize)
                    < *ampstack.offset((stack - 1 as libc::c_int as libc::c_long) as isize)
                {
                    *posstack.offset(stack as isize) = i;
                    let fresh7 = stack;
                    stack = stack + 1;
                    *ampstack.offset(fresh7 as isize) = *seeds.offset(i as isize);
                    break;
                } else {
                    if i < *posstack.offset((stack - 1 as libc::c_int as libc::c_long) as isize)
                        + linesper as libc::c_long
                    {
                        if stack > 1 as libc::c_int as libc::c_long
                            && *ampstack.offset((stack - 1 as libc::c_int as libc::c_long) as isize)
                                <= *ampstack
                                    .offset((stack - 2 as libc::c_int as libc::c_long) as isize)
                            && i < *posstack
                                .offset((stack - 2 as libc::c_int as libc::c_long) as isize)
                                + linesper as libc::c_long
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
    i = 0 as libc::c_int as libc::c_long;
    while i < stack {
        let mut endpos: libc::c_long = 0;
        if i < stack - 1 as libc::c_int as libc::c_long
            && *ampstack.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                > *ampstack.offset(i as isize)
        {
            endpos = *posstack.offset((i + 1 as libc::c_int as libc::c_long) as isize)
        } else {
            endpos = *posstack.offset(i as isize)
                + linesper as libc::c_long
                + 1 as libc::c_int as libc::c_long
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
    mut seed: *mut libc::c_float,
    mut flr: *mut libc::c_float,
) {
    let mut n: libc::c_long = (*p).total_octave_lines as libc::c_long; /* for masking */
    let mut linesper: libc::c_int = (*p).eighth_octave_lines;
    let mut linpos: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut pos: libc::c_long = 0;
    seed_chase(seed, linesper, n);
    pos = *(*p).octave.offset(0 as libc::c_int as isize)
        - (*p).firstoc
        - (linesper >> 1 as libc::c_int) as libc::c_long;
    while (linpos + 1 as libc::c_int as libc::c_long) < (*p).n as libc::c_long {
        let mut minV: libc::c_float = *seed.offset(pos as isize);
        let mut end: libc::c_long = (*(*p).octave.offset(linpos as isize)
            + *(*p)
                .octave
                .offset((linpos + 1 as libc::c_int as libc::c_long) as isize)
            >> 1 as libc::c_int)
            - (*p).firstoc;
        if minV > (*(*p).vi).tone_abs_limit {
            minV = (*(*p).vi).tone_abs_limit
        }
        while pos + 1 as libc::c_int as libc::c_long <= end {
            pos += 1;
            if *seed.offset(pos as isize) > -9999.0f32 && *seed.offset(pos as isize) < minV
                || minV == -9999.0f32
            {
                minV = *seed.offset(pos as isize)
            }
        }
        end = pos + (*p).firstoc;
        while linpos < (*p).n as libc::c_long && *(*p).octave.offset(linpos as isize) <= end {
            if *flr.offset(linpos as isize) < minV {
                *flr.offset(linpos as isize) = minV
            }
            linpos += 1
        }
    }
    let mut minV_0: libc::c_float =
        *seed.offset(((*p).total_octave_lines - 1 as libc::c_int) as isize);
    while linpos < (*p).n as libc::c_long {
        if *flr.offset(linpos as isize) < minV_0 {
            *flr.offset(linpos as isize) = minV_0
        }
        linpos += 1
    }
}

unsafe extern "C" fn bark_noise_hybridmp(
    mut n: libc::c_int,
    mut b: *const libc::c_long,
    mut f: *const libc::c_float,
    mut noise: *mut libc::c_float,
    offset: libc::c_float,
    fixed: libc::c_int,
) {
    let mut fresh9 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut N: *mut libc::c_float = fresh9.as_mut_ptr() as *mut libc::c_float;
    let mut fresh10 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut X: *mut libc::c_float = fresh10.as_mut_ptr() as *mut libc::c_float;
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut XX: *mut libc::c_float = fresh11.as_mut_ptr() as *mut libc::c_float;
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut Y: *mut libc::c_float = fresh12.as_mut_ptr() as *mut libc::c_float;
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut XY: *mut libc::c_float = fresh13.as_mut_ptr() as *mut libc::c_float;
    let mut tN: libc::c_float = 0.;
    let mut tX: libc::c_float = 0.;
    let mut tXX: libc::c_float = 0.;
    let mut tY: libc::c_float = 0.;
    let mut tXY: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut hi: libc::c_int = 0;
    let mut R: libc::c_float = 0.0f32;
    let mut A: libc::c_float = 0.0f32;
    let mut B: libc::c_float = 0.0f32;
    let mut D: libc::c_float = 1.0f32;
    let mut w: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    tXY = 0.0f32;
    tY = tXY;
    tXX = tY;
    tX = tXX;
    tN = tX;
    y = *f.offset(0 as libc::c_int as isize) + offset;
    if y < 1.0f32 {
        y = 1.0f32
    }
    w = ((y * y) as libc::c_double * 0.5f64) as libc::c_float;
    tN += w;
    tX += w;
    tY += w * y;
    *N.offset(0 as libc::c_int as isize) = tN;
    *X.offset(0 as libc::c_int as isize) = tX;
    *XX.offset(0 as libc::c_int as isize) = tXX;
    *Y.offset(0 as libc::c_int as isize) = tY;
    *XY.offset(0 as libc::c_int as isize) = tXY;
    i = 1 as libc::c_int;
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
    i = 0 as libc::c_int;
    x = 0.0f32;
    loop {
        lo = (*b.offset(i as isize) >> 16 as libc::c_int) as libc::c_int;
        if lo >= 0 as libc::c_int {
            break;
        }
        hi = (*b.offset(i as isize) & 0xffff as libc::c_int as libc::c_long) as libc::c_int;
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
        lo = (*b.offset(i as isize) >> 16 as libc::c_int) as libc::c_int;
        hi = (*b.offset(i as isize) & 0xffff as libc::c_int as libc::c_long) as libc::c_int;
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
    if fixed <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    x = 0.0f32;
    loop {
        hi = i + fixed / 2 as libc::c_int;
        lo = hi - fixed;
        if lo >= 0 as libc::c_int {
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
        hi = i + fixed / 2 as libc::c_int;
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
    mut logmdct: *mut libc::c_float,
    mut logmask: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*p).n;
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut work: *mut libc::c_float = fresh14.as_mut_ptr() as *mut libc::c_float;
    bark_noise_hybridmp(
        n,
        (*p).bark,
        logmdct,
        logmask,
        140.0f64 as libc::c_float,
        -(1 as libc::c_int),
    );
    i = 0 as libc::c_int;
    while i < n {
        *work.offset(i as isize) = *logmdct.offset(i as isize) - *logmask.offset(i as isize);
        i += 1
    }
    bark_noise_hybridmp(
        n,
        (*p).bark,
        work,
        logmask,
        0.0f64 as libc::c_float,
        (*(*p).vi).noisewindowfixed,
    );
    i = 0 as libc::c_int;
    while i < n {
        *work.offset(i as isize) = *logmdct.offset(i as isize) - *work.offset(i as isize);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut dB: libc::c_int =
            (*logmask.offset(i as isize) as libc::c_double + 0.5f64) as libc::c_int;
        if dB >= 40 as libc::c_int {
            dB = 40 as libc::c_int - 1 as libc::c_int
        }
        if dB < 0 as libc::c_int {
            dB = 0 as libc::c_int
        }
        *logmask.offset(i as isize) =
            *work.offset(i as isize) + (*(*p).vi).noisecompand[dB as usize];
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn _vp_tonemask(
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut logfft: *mut libc::c_float,
    mut logmask: *mut libc::c_float,
    mut global_specmax: libc::c_float,
    mut local_specmax: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*p).n;
    let mut fresh15 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*p).total_octave_lines as libc::c_ulong) as usize,
    );
    let mut seed: *mut libc::c_float = fresh15.as_mut_ptr() as *mut libc::c_float;
    let mut att: libc::c_float = local_specmax + (*(*p).vi).ath_adjatt;
    i = 0 as libc::c_int;
    while i < (*p).total_octave_lines {
        *seed.offset(i as isize) = -9999.0f32;
        i += 1
    }
    /* set the ATH (floating below localmax, not global max by a
    specified att) */
    if att < (*(*p).vi).ath_maxatt {
        att = (*(*p).vi).ath_maxatt
    }
    i = 0 as libc::c_int;
    while i < n {
        *logmask.offset(i as isize) = *(*p).ath.offset(i as isize) + att;
        i += 1
    }
    /* tone masking */
    seed_loop(
        p,
        (*p).tonecurves as *mut *mut *const libc::c_float,
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
    mut noise: *mut libc::c_float,
    mut tone: *mut libc::c_float,
    mut offset_select: libc::c_int,
    mut logmask: *mut libc::c_float,
    mut mdct: *mut libc::c_float,
    mut logmdct: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*p).n;
    let mut de: libc::c_float = 0.;
    let mut coeffi: libc::c_float = 0.;
    let mut cx: libc::c_float = 0.;
    let mut toneatt: libc::c_float = (*(*p).vi).tone_masteratt[offset_select as usize];
    cx = (*p).m_val;
    i = 0 as libc::c_int;
    while i < n {
        let mut val: libc::c_float = *noise.offset(i as isize)
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
        if offset_select == 1 as libc::c_int {
            coeffi = -17.2f64 as libc::c_float; /* coeffi is a -17.2dB threshold */
            val = val - *logmdct.offset(i as isize); /* val == mdct line value relative to floor in dB */
            if val > coeffi {
                /* mdct value is > -17.2 dB below floor */
                de = (1.0f64 - (val - coeffi) as libc::c_double * 0.005f64 * cx as libc::c_double)
                    as libc::c_float;
                /* pro-rated attenuation:
                -0.00 dB boost if mdct value is -17.2dB (relative to floor)
                -0.77 dB boost if mdct value is 0dB (relative to floor)
                -1.64 dB boost if mdct value is +17.2dB (relative to floor)
                etc... */
                if de < 0 as libc::c_int as libc::c_float {
                    de = 0.0001f64 as libc::c_float
                }
            } else {
                /* mdct value is <= -17.2 dB below floor */
                de = (1.0f64 - (val - coeffi) as libc::c_double * 0.0003f64 * cx as libc::c_double)
                    as libc::c_float
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
    mut amp: libc::c_float,
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
) -> libc::c_float {
    let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut gi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global =
        &mut (*ci).psy_g_param;
    let mut n: libc::c_int =
        ((*ci).blocksizes[(*vd).W as usize] / 2 as libc::c_int as libc::c_long) as libc::c_int;
    let mut secs: libc::c_float = n as libc::c_float / (*vi).rate as libc::c_float;
    amp += secs * (*gi).ampmax_att_per_sec;
    if amp < -(9999 as libc::c_int) as libc::c_float {
        amp = -(9999 as libc::c_int) as libc::c_float
    }
    return amp;
}

static mut FLOOR1_fromdB_LOOKUP: [libc::c_float; 256] = [
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

unsafe extern "C" fn apsort(mut a: *const libc::c_void, mut b: *const libc::c_void) -> libc::c_int {
    let mut f1: libc::c_float = **(a as *mut *mut libc::c_float);
    let mut f2: libc::c_float = **(b as *mut *mut libc::c_float);
    return (f1 < f2) as libc::c_int - (f1 > f2) as libc::c_int;
}

unsafe extern "C" fn flag_lossless(
    mut limit: libc::c_int,
    mut prepoint: libc::c_float,
    mut postpoint: libc::c_float,
    mut mdct: *mut libc::c_float,
    mut floor_0: *mut libc::c_float,
    mut flag: *mut libc::c_int,
    mut i: libc::c_int,
    mut jn: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < jn {
        let mut point: libc::c_float = if j >= limit - i { postpoint } else { prepoint };
        let mut r: libc::c_float = (crate::stdlib::fabs(*mdct.offset(j as isize) as libc::c_double)
            / *floor_0.offset(j as isize) as libc::c_double)
            as libc::c_float;
        if r < point {
            *flag.offset(j as isize) = 0 as libc::c_int
        } else {
            *flag.offset(j as isize) = 1 as libc::c_int
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
    mut limit: libc::c_int,
    mut r: *mut libc::c_float,
    mut q: *mut libc::c_float,
    mut f: *mut libc::c_float,
    mut flags: *mut libc::c_int,
    mut acc: libc::c_float,
    mut i: libc::c_int,
    mut n: libc::c_int,
    mut out: *mut libc::c_int,
) -> libc::c_float {
    let mut vi: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy = (*p).vi;
    let mut fresh16 = ::std::vec::from_elem(
        0,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut sort: *mut *mut libc::c_float = fresh16.as_mut_ptr() as *mut *mut libc::c_float;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = if (*vi).normal_p != 0 {
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
    j = 0 as libc::c_int;
    while j < start {
        if flags.is_null() || *flags.offset(j as isize) == 0 {
            /* lossless coupling already quantized.
            Don't touch; requantizing based on
            energy would be incorrect. */
            let mut ve: libc::c_float = *q.offset(j as isize) / *f.offset(j as isize);
            if *r.offset(j as isize) < 0 as libc::c_int as libc::c_float {
                *out.offset(j as isize) =
                    -crate::stdlib::rint(crate::stdlib::sqrt(ve as libc::c_double)) as libc::c_int
            } else {
                *out.offset(j as isize) =
                    crate::stdlib::rint(crate::stdlib::sqrt(ve as libc::c_double)) as libc::c_int
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
            let mut ve_0: libc::c_float = *q.offset(j as isize) / *f.offset(j as isize);
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
                if *r.offset(j as isize) < 0 as libc::c_int as libc::c_float {
                    *out.offset(j as isize) =
                        -crate::stdlib::rint(crate::stdlib::sqrt(ve_0 as libc::c_double))
                            as libc::c_int
                } else {
                    *out.offset(j as isize) =
                        crate::stdlib::rint(crate::stdlib::sqrt(ve_0 as libc::c_double))
                            as libc::c_int
                }
                *q.offset(j as isize) = (*out.offset(j as isize) * *out.offset(j as isize))
                    as libc::c_float
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
            ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
            Some(
                apsort
                    as unsafe extern "C" fn(
                        _: *const libc::c_void,
                        _: *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        j = 0 as libc::c_int;
        while j < count {
            let mut k: libc::c_int =
                (*sort.offset(j as isize)).offset_from(q) as libc::c_long as libc::c_int;
            if acc as libc::c_double >= (*vi).normal_thresh {
                *out.offset(k as isize) = unitnorm(*r.offset(k as isize)) as libc::c_int;
                acc -= 1.0f32;
                *q.offset(k as isize) = *f.offset(k as isize)
            } else {
                *out.offset(k as isize) = 0 as libc::c_int;
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
    mut blobno: libc::c_int,
    mut g: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global,
    mut p: *mut crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy,
    mut vi: *mut crate::backends_h::vorbis_info_mapping0,
    mut mdct: *mut *mut libc::c_float,
    mut iwork: *mut *mut libc::c_int,
    mut nonzero: *mut libc::c_int,
    mut sliding_lowpass: libc::c_int,
    mut ch: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*p).n;
    let mut partition: libc::c_int = if (*(*p).vi).normal_p != 0 {
        (*(*p).vi).normal_partition
    } else {
        16 as libc::c_int
    };
    let mut limit: libc::c_int =
        (*g).coupling_pointlimit[(*(*p).vi).blockflag as usize][blobno as usize];
    let mut prepoint: libc::c_float =
        stereo_threshholds[(*g).coupling_prepointamp[blobno as usize] as usize] as libc::c_float;
    let mut postpoint: libc::c_float =
        stereo_threshholds[(*g).coupling_postpointamp[blobno as usize] as usize] as libc::c_float;
    /* mdct is our raw mdct output, floor not removed. */
    /* inout passes in the ifloor, passes back quantized result */
    /* unquantized energy (negative indicates amplitude has negative sign) */
    let mut fresh19 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut raw: *mut *mut libc::c_float = fresh19.as_mut_ptr() as *mut *mut libc::c_float;
    /* dual pupose; quantized energy (if flag set), othersize fabs(raw) */
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut quant: *mut *mut libc::c_float = fresh20.as_mut_ptr() as *mut *mut libc::c_float;
    /* floor energy */
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            as usize,
    );
    let mut floor_0: *mut *mut libc::c_float = fresh21.as_mut_ptr() as *mut *mut libc::c_float;
    /* flags indicating raw/quantized status of elements in raw vector */
    let mut fresh22 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            as usize,
    );
    let mut flag: *mut *mut libc::c_int = fresh22.as_mut_ptr() as *mut *mut libc::c_int;
    /* non-zero flag working vector */
    let mut fresh23 = ::std::vec::from_elem(
        0,
        (ch as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as usize,
    );
    let mut nz: *mut libc::c_int = fresh23.as_mut_ptr() as *mut libc::c_int;
    /* energy surplus/defecit tracking */
    let mut fresh24 = ::std::vec::from_elem(
        0,
        ((ch + (*vi).coupling_steps) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong) as usize,
    );
    let mut acc: *mut libc::c_float = fresh24.as_mut_ptr() as *mut libc::c_float;
    /* The threshold of a stereo is changed with the size of n */
    if n > 1000 as libc::c_int {
        postpoint = stereo_threshholds_limited[(*g).coupling_postpointamp[blobno as usize] as usize]
            as libc::c_float
    }
    let mut fresh25 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong) as usize,
    );
    let ref mut fresh26 = *raw.offset(0 as libc::c_int as isize);
    *fresh26 = fresh25.as_mut_ptr() as *mut libc::c_float;
    let mut fresh27 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong) as usize,
    );
    let ref mut fresh28 = *quant.offset(0 as libc::c_int as isize);
    *fresh28 = fresh27.as_mut_ptr() as *mut libc::c_float;
    let mut fresh29 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong) as usize,
    );
    let ref mut fresh30 = *floor_0.offset(0 as libc::c_int as isize);
    *fresh30 = fresh29.as_mut_ptr() as *mut libc::c_float;
    let mut fresh31 = ::std::vec::from_elem(
        0,
        ((ch * partition) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as usize,
    );
    let ref mut fresh32 = *flag.offset(0 as libc::c_int as isize);
    *fresh32 = fresh31.as_mut_ptr() as *mut libc::c_int;
    i = 1 as libc::c_int;
    while i < ch {
        let ref mut fresh33 = *raw.offset(i as isize);
        *fresh33 = &mut *(*raw.offset(0 as libc::c_int as isize)).offset((partition * i) as isize)
            as *mut libc::c_float;
        let ref mut fresh34 = *quant.offset(i as isize);
        *fresh34 = &mut *(*quant.offset(0 as libc::c_int as isize)).offset((partition * i) as isize)
            as *mut libc::c_float;
        let ref mut fresh35 = *floor_0.offset(i as isize);
        *fresh35 = &mut *(*floor_0.offset(0 as libc::c_int as isize))
            .offset((partition * i) as isize) as *mut libc::c_float;
        let ref mut fresh36 = *flag.offset(i as isize);
        *fresh36 = &mut *(*flag.offset(0 as libc::c_int as isize)).offset((partition * i) as isize)
            as *mut libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < ch + (*vi).coupling_steps {
        *acc.offset(i as isize) = 0.0f32;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut k: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut jn: libc::c_int = if partition > n - i {
            (n) - i
        } else {
            partition
        };
        let mut step: libc::c_int = 0;
        let mut track: libc::c_int = 0 as libc::c_int;
        crate::stdlib::memcpy(
            nz as *mut libc::c_void,
            nonzero as *const libc::c_void,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(ch as libc::c_ulong),
        );
        /* prefill */
        crate::stdlib::memset(
            *flag.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((ch * partition) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        k = 0 as libc::c_int;
        while k < ch {
            let mut iout: *mut libc::c_int =
                &mut *(*iwork.offset(k as isize)).offset(i as isize) as *mut libc::c_int;
            if *nz.offset(k as isize) != 0 {
                j = 0 as libc::c_int;
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
                j = 0 as libc::c_int;
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
                    0 as *mut libc::c_int,
                    *acc.offset(track as isize),
                    i,
                    jn,
                    iout,
                )
            } else {
                j = 0 as libc::c_int;
                while j < jn {
                    *(*floor_0.offset(k as isize)).offset(j as isize) = 1e-10f32;
                    *(*raw.offset(k as isize)).offset(j as isize) = 0.0f32;
                    *(*quant.offset(k as isize)).offset(j as isize) = 0.0f32;
                    *(*flag.offset(k as isize)).offset(j as isize) = 0 as libc::c_int;
                    *iout.offset(j as isize) = 0 as libc::c_int;
                    j += 1
                }
                *acc.offset(track as isize) = 0.0f32
            }
            track += 1;
            k += 1
        }
        /* coupling */
        step = 0 as libc::c_int;
        while step < (*vi).coupling_steps {
            let mut Mi: libc::c_int = (*vi).coupling_mag[step as usize];
            let mut Ai: libc::c_int = (*vi).coupling_ang[step as usize];
            let mut iM: *mut libc::c_int =
                &mut *(*iwork.offset(Mi as isize)).offset(i as isize) as *mut libc::c_int;
            let mut iA: *mut libc::c_int =
                &mut *(*iwork.offset(Ai as isize)).offset(i as isize) as *mut libc::c_int;
            let mut reM: *mut libc::c_float = *raw.offset(Mi as isize);
            let mut reA: *mut libc::c_float = *raw.offset(Ai as isize);
            let mut qeM: *mut libc::c_float = *quant.offset(Mi as isize);
            let mut qeA: *mut libc::c_float = *quant.offset(Ai as isize);
            let mut floorM: *mut libc::c_float = *floor_0.offset(Mi as isize);
            let mut floorA: *mut libc::c_float = *floor_0.offset(Ai as isize);
            let mut fM: *mut libc::c_int = *flag.offset(Mi as isize);
            let mut fA: *mut libc::c_int = *flag.offset(Ai as isize);
            if *nz.offset(Mi as isize) != 0 || *nz.offset(Ai as isize) != 0 {
                let ref mut fresh38 = *nz.offset(Ai as isize);
                *fresh38 = 1 as libc::c_int;
                *nz.offset(Mi as isize) = *fresh38;
                j = 0 as libc::c_int;
                while j < jn {
                    if j < sliding_lowpass - i {
                        if *fM.offset(j as isize) != 0 || *fA.offset(j as isize) != 0 {
                            /* lossless coupling */
                            *reM.offset(j as isize) = (crate::stdlib::fabs(
                                *reM.offset(j as isize) as libc::c_double
                            ) + crate::stdlib::fabs(
                                *reA.offset(j as isize) as libc::c_double,
                            ))
                                as libc::c_float;
                            *qeM.offset(j as isize) =
                                *qeM.offset(j as isize) + *qeA.offset(j as isize);
                            let ref mut fresh39 = *fA.offset(j as isize);
                            *fresh39 = 1 as libc::c_int;
                            *fM.offset(j as isize) = *fresh39;
                            /* couple iM/iA */
                            let mut A: libc::c_int = *iM.offset(j as isize);
                            let mut B: libc::c_int = *iA.offset(j as isize);
                            if ::libc::abs(A) > ::libc::abs(B) {
                                *iA.offset(j as isize) = if A > 0 as libc::c_int {
                                    (A) - B
                                } else {
                                    (B) - A
                                }
                            } else {
                                *iA.offset(j as isize) = if B > 0 as libc::c_int {
                                    (A) - B
                                } else {
                                    (B) - A
                                };
                                *iM.offset(j as isize) = B
                            }
                            /* collapse two equivalent tuples to one */
                            if *iA.offset(j as isize)
                                >= ::libc::abs(*iM.offset(j as isize)) * 2 as libc::c_int
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
                                    crate::stdlib::fabs(*reM.offset(j as isize) as libc::c_double)
                                        as libc::c_float
                            } else if *reM.offset(j as isize) + *reA.offset(j as isize)
                                < 0 as libc::c_int as libc::c_float
                            {
                                let ref mut fresh40 = *qeM.offset(j as isize);
                                *fresh40 =
                                    (crate::stdlib::fabs(*reM.offset(j as isize) as libc::c_double)
                                        + crate::stdlib::fabs(
                                            *reA.offset(j as isize) as libc::c_double
                                        )) as libc::c_float;
                                *reM.offset(j as isize) = -*fresh40
                            } else {
                                let ref mut fresh41 = *qeM.offset(j as isize);
                                *fresh41 =
                                    (crate::stdlib::fabs(*reM.offset(j as isize) as libc::c_double)
                                        + crate::stdlib::fabs(
                                            *reA.offset(j as isize) as libc::c_double
                                        )) as libc::c_float;
                                *reM.offset(j as isize) = *fresh41
                            }
                            let ref mut fresh42 = *qeA.offset(j as isize);
                            *fresh42 = 0.0f32;
                            *reA.offset(j as isize) = *fresh42;
                            *fA.offset(j as isize) = 1 as libc::c_int;
                            *iA.offset(j as isize) = 0 as libc::c_int
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
    i = 0 as libc::c_int;
    while i < (*vi).coupling_steps {
        /* make sure coupling a zero and a nonzero channel results in two
        nonzero channels. */
        if *nonzero.offset((*vi).coupling_mag[i as usize] as isize) != 0
            || *nonzero.offset((*vi).coupling_ang[i as usize] as isize) != 0
        {
            *nonzero.offset((*vi).coupling_mag[i as usize] as isize) = 1 as libc::c_int;
            *nonzero.offset((*vi).coupling_ang[i as usize] as isize) = 1 as libc::c_int
        }
        i += 1
    }
}

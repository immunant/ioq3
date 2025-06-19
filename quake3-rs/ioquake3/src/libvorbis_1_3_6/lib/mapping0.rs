use ::libc;

pub mod scales_h {

    /* Segher was off (too high) by ~ .3 decibel.  Center the conversion correctly. */
    #[inline]

    pub unsafe extern "C" fn todB(mut x: *const f32) -> f32 {
        let mut ix: crate::scales_h::C2RustUnnamed_58 = crate::scales_h::C2RustUnnamed_58 { i: 0 };
        ix.f = *x;
        ix.i = ix.i & 0x7fffffff as i32 as u32;
        return ix.i as f32 * 7.17711438e-7f32 - 764.6161886f32;
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

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_func_mapping;
pub use crate::backends_h::vorbis_func_residue;
pub use crate::backends_h::vorbis_info_floor1;
pub use crate::backends_h::vorbis_info_mapping0;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::private_state;
pub use crate::codec_internal_h::vorbis_block_internal;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_floor1;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::scales_h::C2RustUnnamed_58;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_managed;

pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::floor1::floor1_encode;
pub use crate::src::libvorbis_1_3_6::lib::floor1::floor1_fit;
pub use crate::src::libvorbis_1_3_6::lib::floor1::floor1_interpolate_fit;
pub use crate::src::libvorbis_1_3_6::lib::mapping0::scales_h::todB;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_backward;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_forward;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_couple_quantize_normalize;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_noisemask;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_offset_and_mix;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vp_tonemask;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;

pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_forward;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2010             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: channel mapping 0 implementation

********************************************************************/
/* simplistic, wasteful way of doing this (unique lookup for each
mode/submapping); there should be a central repository for
identical lookups.  That will require minor work, so I'm putting it
off as low priority.

Why a lookup for each backend in a given mode?  Because the
blocksize is set by the mode, and low backend lookups may require
parameters from other areas of the mode/mapping */

unsafe extern "C" fn mapping0_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut vorbis_info_mapping0 = i as *mut vorbis_info_mapping0;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<vorbis_info_mapping0>() as usize,
        );
        libc::free(info as *mut libc::c_void);
    };
}

unsafe extern "C" fn mapping0_pack(
    mut vi: *mut vorbis_info,
    mut vm: *mut libc::c_void,
    mut opb: *mut oggpack_buffer,
) {
    let mut i: i32 = 0;
    let mut info: *mut vorbis_info_mapping0 = vm as *mut vorbis_info_mapping0;
    /* another 'we meant to do it this way' hack...  up to beta 4, we
    packed 4 binary zeros here to signify one submapping in use.  We
    now redefine that to mean four bitflags that indicate use of
    deeper features; bit0:submappings, bit1:coupling,
    bit2,3:reserved. This is backward compatable with all actual uses
    of the beta code. */
    if (*info).submaps > 1 as i32 {
        oggpack_write(opb as *mut oggpack_buffer, 1 as i32 as usize, 1 as i32); /* 2,3:reserved */
        oggpack_write(
            opb as *mut oggpack_buffer,
            ((*info).submaps - 1 as i32) as usize,
            4 as i32,
        );
    } else {
        oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 1 as i32);
    }
    if (*info).coupling_steps > 0 as i32 {
        oggpack_write(opb as *mut oggpack_buffer, 1 as i32 as usize, 1 as i32);
        oggpack_write(
            opb as *mut oggpack_buffer,
            ((*info).coupling_steps - 1 as i32) as usize,
            8 as i32,
        );
        i = 0 as i32;
        while i < (*info).coupling_steps {
            oggpack_write(
                opb as *mut oggpack_buffer,
                (*info).coupling_mag[i as usize] as usize,
                crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                    ((*vi).channels - 1 as i32) as ogg_uint32_t,
                ),
            );
            oggpack_write(
                opb as *mut oggpack_buffer,
                (*info).coupling_ang[i as usize] as usize,
                crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                    ((*vi).channels - 1 as i32) as ogg_uint32_t,
                ),
            );
            i += 1
        }
    } else {
        oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 1 as i32);
    }
    oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 2 as i32);
    /* we don't write the channel submappings if we only have one... */
    if (*info).submaps > 1 as i32 {
        i = 0 as i32; /* time submap unused */
        while i < (*vi).channels {
            oggpack_write(
                opb as *mut oggpack_buffer,
                (*info).chmuxlist[i as usize] as usize,
                4 as i32,
            );
            i += 1
        }
    }
    i = 0 as i32;
    while i < (*info).submaps {
        oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 8 as i32);
        oggpack_write(
            opb as *mut oggpack_buffer,
            (*info).floorsubmap[i as usize] as usize,
            8 as i32,
        );
        oggpack_write(
            opb as *mut oggpack_buffer,
            (*info).residuesubmap[i as usize] as usize,
            8 as i32,
        );
        i += 1
    }
}
/* also responsible for range checking */

unsafe extern "C" fn mapping0_unpack(
    mut vi: *mut vorbis_info,
    mut opb: *mut oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut b: i32 = 0;
    let mut info: *mut vorbis_info_mapping0 = crate::stdlib::calloc(
        1 as i32 as usize,
        ::std::mem::size_of::<vorbis_info_mapping0>() as usize,
    ) as *mut vorbis_info_mapping0;
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    if !((*vi).channels <= 0 as i32) {
        b = oggpack_read(opb as *mut oggpack_buffer, 1 as i32) as i32;
        if !(b < 0 as i32) {
            if b != 0 {
                (*info).submaps =
                    (oggpack_read(opb as *mut oggpack_buffer, 4 as i32) + 1 as i32 as isize) as i32;
                if (*info).submaps <= 0 as i32 {
                    current_block = 1977384903651761240;
                } else {
                    current_block = 3276175668257526147;
                }
            } else {
                (*info).submaps = 1 as i32;
                current_block = 3276175668257526147;
            }
            match current_block {
                1977384903651761240 => {}
                _ => {
                    b = oggpack_read(opb as *mut oggpack_buffer, 1 as i32) as i32;
                    if !(b < 0 as i32) {
                        if b != 0 {
                            (*info).coupling_steps =
                                (oggpack_read(opb as *mut oggpack_buffer, 8 as i32)
                                    + 1 as i32 as isize) as i32;
                            if (*info).coupling_steps <= 0 as i32 {
                                current_block = 1977384903651761240;
                            } else {
                                i = 0 as i32;
                                loop {
                                    if !(i < (*info).coupling_steps) {
                                        current_block = 13797916685926291137;
                                        break;
                                    }
                                    /* vi->channels > 0 is enforced in the caller */
                                    (*info).coupling_mag[i as usize] = oggpack_read(
                                        opb as *mut oggpack_buffer,
                                        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                                            ((*vi).channels - 1 as i32) as ogg_uint32_t,
                                        ),
                                    )
                                        as i32; /* 2,3:reserved */
                                    let mut testM: i32 = (*info).coupling_mag[i as usize]; /* time submap unused */
                                    (*info).coupling_ang[i as usize] = oggpack_read(
                                        opb as *mut oggpack_buffer,
                                        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                                            ((*vi).channels - 1 as i32) as ogg_uint32_t,
                                        ),
                                    )
                                        as i32; /* + .345 is a hack; the original
                                                todB estimation used on IEEE 754
                                                compliant machines had a bug that
                                                returned dB values about a third
                                                of a decibel too high.  The bug
                                                was harmless because tunings
                                                implicitly took that into
                                                account.  However, fixing the bug
                                                in the estimator requires
                                                changing all the tunings as well.
                                                For now, it's easier to sync
                                                things back up here, and
                                                recalibrate the tunings in the
                                                next major model upgrade. */
                                    let mut testA: i32 = (*info).coupling_ang[i as usize];
                                    if testM < 0 as i32
                                        || testA < 0 as i32
                                        || testM == testA
                                        || testM >= (*vi).channels
                                        || testA >= (*vi).channels
                                    {
                                        current_block = 1977384903651761240;
                                        break;
                                    }
                                    i += 1
                                }
                            }
                        } else {
                            current_block = 13797916685926291137;
                        }
                        match current_block {
                            1977384903651761240 => {}
                            _ => {
                                if !(oggpack_read(opb as *mut oggpack_buffer, 2 as i32)
                                    != 0 as i32 as isize)
                                {
                                    if (*info).submaps > 1 as i32 {
                                        i = 0 as i32;
                                        loop {
                                            if !(i < (*vi).channels) {
                                                current_block = 15345278821338558188;
                                                break;
                                            }
                                            (*info).chmuxlist[i as usize] =
                                                oggpack_read(opb as *mut oggpack_buffer, 4 as i32)
                                                    as i32;
                                            if (*info).chmuxlist[i as usize] >= (*info).submaps
                                                || (*info).chmuxlist[i as usize] < 0 as i32
                                            {
                                                current_block = 1977384903651761240;
                                                break;
                                            }
                                            i += 1
                                        }
                                    } else {
                                        current_block = 15345278821338558188;
                                    }
                                    match current_block {
                                        1977384903651761240 => {}
                                        _ => {
                                            i = 0 as i32;
                                            loop {
                                                if !(i < (*info).submaps) {
                                                    current_block = 2873832966593178012;
                                                    break;
                                                }
                                                oggpack_read(opb as *mut oggpack_buffer, 8 as i32);
                                                (*info).floorsubmap[i as usize] = oggpack_read(
                                                    opb as *mut oggpack_buffer,
                                                    8 as i32,
                                                )
                                                    as i32;
                                                if (*info).floorsubmap[i as usize] >= (*ci).floors
                                                    || (*info).floorsubmap[i as usize] < 0 as i32
                                                {
                                                    current_block = 1977384903651761240;
                                                    break;
                                                }
                                                (*info).residuesubmap[i as usize] = oggpack_read(
                                                    opb as *mut oggpack_buffer,
                                                    8 as i32,
                                                )
                                                    as i32;
                                                if (*info).residuesubmap[i as usize]
                                                    >= (*ci).residues
                                                    || (*info).residuesubmap[i as usize] < 0 as i32
                                                {
                                                    current_block = 1977384903651761240;
                                                    break;
                                                }
                                                i += 1
                                            }
                                            match current_block {
                                                1977384903651761240 => {}
                                                _ => return info as *mut libc::c_void,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    mapping0_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn mapping0_forward(mut vb: *mut vorbis_block) -> i32 {
    let mut vd: *mut vorbis_dsp_state = (*vb).vd;
    let mut vi: *mut vorbis_info = (*vd).vi;
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    let mut b: *mut private_state = (*(*vb).vd).backend_state as *mut private_state;
    let mut vbi: *mut vorbis_block_internal = (*vb).internal as *mut vorbis_block_internal;
    let mut n: i32 = (*vb).pcmend;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut nonzero: *mut i32 = fresh0.as_mut_ptr() as *mut i32;
    let mut gmdct: *mut *mut f32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut vorbis_block,
        ((*vi).channels as usize).wrapping_mul(::std::mem::size_of::<*mut f32>() as usize) as isize,
    ) as *mut *mut f32;
    let mut iwork: *mut *mut i32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut vorbis_block,
        ((*vi).channels as usize).wrapping_mul(::std::mem::size_of::<*mut i32>() as usize) as isize,
    ) as *mut *mut i32;
    let mut floor_posts: *mut *mut *mut i32 =
        crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut vorbis_block,
            ((*vi).channels as usize).wrapping_mul(::std::mem::size_of::<*mut *mut i32>() as usize)
                as isize,
        ) as *mut *mut *mut i32;
    let mut global_ampmax: f32 = (*vbi).ampmax;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<f32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut local_ampmax: *mut f32 = fresh1.as_mut_ptr() as *mut f32;
    let mut blocktype: i32 = (*vbi).blocktype;
    let mut modenumber: i32 = (*vb).W as i32;
    let mut info: *mut vorbis_info_mapping0 =
        (*ci).map_param[modenumber as usize] as *mut vorbis_info_mapping0;
    let mut psy_look: *mut vorbis_look_psy = (*b)
        .psy
        .offset(blocktype as isize)
        .offset((if (*vb).W != 0 { 2 as i32 } else { 0 as i32 }) as isize);
    (*vb).mode = modenumber;
    i = 0 as i32;
    while i < (*vi).channels {
        let mut scale: f32 = 4.0f32 / n as f32;
        let mut scale_dB: f32 = 0.;
        let mut pcm: *mut f32 = *(*vb).pcm.offset(i as isize);
        let mut logfft: *mut f32 = pcm;
        let ref mut fresh2 = *iwork.offset(i as isize);
        *fresh2 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut vorbis_block,
            ((n / 2 as i32) as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize) as isize,
        ) as *mut i32;
        let ref mut fresh3 = *gmdct.offset(i as isize);
        *fresh3 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut vorbis_block,
            ((n / 2 as i32) as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize) as isize,
        ) as *mut f32;
        scale_dB = (todB(&mut scale) as f64 + 0.345f64) as f32;
        /* window the PCM data */
        crate::src::libvorbis_1_3_6::lib::window::_vorbis_apply_window(
            pcm,
            (*b).window.as_mut_ptr(),
            (*ci).blocksizes.as_mut_ptr(),
            (*vb).lW as i32,
            (*vb).W as i32,
            (*vb).nW as i32,
        );
        /* transform the PCM data */
        /* only MDCT right now.... */
        mdct_forward(
            *(*b).transform[(*vb).W as usize].offset(0 as i32 as isize) as *mut mdct_lookup
                as *mut mdct_lookup,
            pcm,
            *gmdct.offset(i as isize),
        );
        /* FFT yields more accurate tonal estimation (not phase sensitive) */
        drft_forward(
            &mut *(*b).fft_look.as_mut_ptr().offset((*vb).W as isize) as *mut _ as *mut drft_lookup,
            pcm,
        ); /* + .345 is a hack; the
           original todB estimation used on
           IEEE 754 compliant machines had a
           bug that returned dB values about
           a third of a decibel too high.
           The bug was harmless because
           tunings implicitly took that into
           account.  However, fixing the bug
           in the estimator requires
           changing all the tunings as well.
           For now, it's easier to sync
           things back up here, and
           recalibrate the tunings in the
           next major model upgrade. */
        *logfft.offset(0 as i32 as isize) = ((scale_dB + todB(pcm)) as f64 + 0.345f64) as f32; /* +
                                                                                               .345 is a hack; the original todB
                                                                                               estimation used on IEEE 754
                                                                                               compliant machines had a bug that
                                                                                               returned dB values about a third
                                                                                               of a decibel too high.  The bug
                                                                                               was harmless because tunings
                                                                                               implicitly took that into
                                                                                               account.  However, fixing the bug
                                                                                               in the estimator requires
                                                                                               changing all the tunings as well.
                                                                                               For now, it's easier to sync
                                                                                               things back up here, and
                                                                                               recalibrate the tunings in the
                                                                                               next major model upgrade. */
        *local_ampmax.offset(i as isize) = *logfft.offset(0 as i32 as isize);
        j = 1 as i32;
        while j < n - 1 as i32 {
            let mut temp: f32 = *pcm.offset(j as isize) * *pcm.offset(j as isize)
                + *pcm.offset((j + 1 as i32) as isize) * *pcm.offset((j + 1 as i32) as isize);
            let ref mut fresh4 = *logfft.offset((j + 1 as i32 >> 1 as i32) as isize);
            *fresh4 = ((scale_dB + 0.5f32 * todB(&mut temp)) as f64 + 0.345f64) as f32;
            temp = *fresh4;
            if temp > *local_ampmax.offset(i as isize) {
                *local_ampmax.offset(i as isize) = temp
            }
            j += 2 as i32
        }
        if *local_ampmax.offset(i as isize) > 0.0f32 {
            *local_ampmax.offset(i as isize) = 0.0f32
        }
        if *local_ampmax.offset(i as isize) > global_ampmax {
            global_ampmax = *local_ampmax.offset(i as isize)
        }
        i += 1
    }
    let mut noise: *mut f32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut vorbis_block,
        ((n / 2 as i32) as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize) as isize,
    ) as *mut f32;
    let mut tone: *mut f32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut vorbis_block,
        ((n / 2 as i32) as usize).wrapping_mul(::std::mem::size_of::<f32>() as usize) as isize,
    ) as *mut f32;
    i = 0 as i32;
    while i < (*vi).channels {
        /* the encoder setup assumes that all the modes used by any
        specific bitrate tweaking use the same floor */
        let mut submap: i32 = (*info).chmuxlist[i as usize];
        /* the following makes things clearer to *me* anyway */
        let mut mdct: *mut f32 = *gmdct.offset(i as isize); /* + .345 is a hack; the original
                                                            todB estimation used on IEEE 754
                                                            compliant machines had a bug that
                                                            returned dB values about a third
                                                            of a decibel too high.  The bug
                                                            was harmless because tunings
                                                            implicitly took that into
                                                            account.  However, fixing the bug
                                                            in the estimator requires
                                                            changing all the tunings as well.
                                                            For now, it's easier to sync
                                                            things back up here, and
                                                            recalibrate the tunings in the
                                                            next major model upgrade. */
        let mut logfft_0: *mut f32 = *(*vb).pcm.offset(i as isize);
        let mut logmdct: *mut f32 = logfft_0.offset((n / 2 as i32) as isize);
        let mut logmask: *mut f32 = logfft_0;
        (*vb).mode = modenumber;
        let ref mut fresh5 = *floor_posts.offset(i as isize);
        *fresh5 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut vorbis_block,
            (15 as i32 as usize).wrapping_mul(::std::mem::size_of::<*mut i32>() as usize) as isize,
        ) as *mut *mut i32;
        crate::stdlib::memset(
            *floor_posts.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<*mut i32>() as usize).wrapping_mul(15 as i32 as usize),
        );
        j = 0 as i32;
        while j < n / 2 as i32 {
            *logmdct.offset(j as isize) = (todB(mdct.offset(j as isize)) as f64 + 0.345f64) as f32;
            j += 1
        }
        /* first step; noise masking.  Not only does 'noise masking'
        give us curves from which we can decide how much resolution
        to give noise parts of the spectrum, it also implicitly hands
        us a tonality estimate (the larger the value in the
        'noise_depth' vector, the more tonal that area is) */
        _vp_noisemask(psy_look as *mut vorbis_look_psy, logmdct, noise); /* noise does not have by-frequency offset
                                                                         bias applied yet */
        /* second step: 'all the other crap'; all the stuff that isn't
        computed/fit for bitrate management goes in the second psy
        vector.  This includes tone masking, peak limiting and ATH */
        _vp_tonemask(
            psy_look as *mut vorbis_look_psy,
            logfft_0,
            tone,
            global_ampmax,
            *local_ampmax.offset(i as isize),
        );
        /* third step; we offset the noise vectors, overlay tone
        masking.  We then do a floor1-specific line fit.  If we're
        performing bitrate management, the line fit is performed
        multiple times for up/down tweakage on demand. */
        _vp_offset_and_mix(
            psy_look as *mut vorbis_look_psy,
            noise,
            tone,
            1 as i32,
            logmask,
            mdct,
            logmdct,
        );
        /* this algorithm is hardwired to floor 1 for now; abort out if
        we're *not* floor1.  This won't happen unless someone has
        broken the encode setup lib.  Guard it anyway. */
        if (*ci).floor_type[(*info).floorsubmap[submap as usize] as usize] != 1 as i32 {
            return -(1 as i32);
        }
        let ref mut fresh6 =
            *(*floor_posts.offset(i as isize)).offset((15 as i32 / 2 as i32) as isize);
        *fresh6 = floor1_fit(
            vb as *mut vorbis_block,
            *(*b)
                .flr
                .offset((*info).floorsubmap[submap as usize] as isize)
                as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
            logmdct,
            logmask,
        );
        /* are we managing bitrate?  If so, perform two more fits for
        later rate tweaking (fits represent hi/lo) */
        if vorbis_bitrate_managed(vb as *mut vorbis_block) != 0
            && !(*(*floor_posts.offset(i as isize)).offset((15 as i32 / 2 as i32) as isize))
                .is_null()
        {
            /* higher rate by way of lower noise curve */
            _vp_offset_and_mix(
                psy_look as *mut vorbis_look_psy,
                noise,
                tone,
                2 as i32,
                logmask,
                mdct,
                logmdct,
            );
            let ref mut fresh7 =
                *(*floor_posts.offset(i as isize)).offset((15 as i32 - 1 as i32) as isize);
            *fresh7 = floor1_fit(
                vb as *mut vorbis_block,
                *(*b)
                    .flr
                    .offset((*info).floorsubmap[submap as usize] as isize)
                    as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
                logmdct,
                logmask,
            );
            /* lower rate by way of higher noise curve */
            _vp_offset_and_mix(
                psy_look as *mut vorbis_look_psy,
                noise,
                tone,
                0 as i32,
                logmask,
                mdct,
                logmdct,
            );
            let ref mut fresh8 = *(*floor_posts.offset(i as isize)).offset(0 as i32 as isize);
            *fresh8 = floor1_fit(
                vb as *mut vorbis_block,
                *(*b)
                    .flr
                    .offset((*info).floorsubmap[submap as usize] as isize)
                    as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
                logmdct,
                logmask,
            );
            /* we also interpolate a range of intermediate curves for
            intermediate rates */
            k = 1 as i32;
            while k < 15 as i32 / 2 as i32 {
                let ref mut fresh9 = *(*floor_posts.offset(i as isize)).offset(k as isize);
                *fresh9 = floor1_interpolate_fit(
                    vb as *mut vorbis_block,
                    *(*b)
                        .flr
                        .offset((*info).floorsubmap[submap as usize] as isize)
                        as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
                    *(*floor_posts.offset(i as isize)).offset(0 as i32 as isize),
                    *(*floor_posts.offset(i as isize)).offset((15 as i32 / 2 as i32) as isize),
                    k * 65536 as i32 / (15 as i32 / 2 as i32),
                );
                k += 1
            }
            k = 15 as i32 / 2 as i32 + 1 as i32;
            while k < 15 as i32 - 1 as i32 {
                let ref mut fresh10 = *(*floor_posts.offset(i as isize)).offset(k as isize);
                *fresh10 = floor1_interpolate_fit(
                    vb as *mut vorbis_block,
                    *(*b)
                        .flr
                        .offset((*info).floorsubmap[submap as usize] as isize)
                        as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
                    *(*floor_posts.offset(i as isize)).offset((15 as i32 / 2 as i32) as isize),
                    *(*floor_posts.offset(i as isize)).offset((15 as i32 - 1 as i32) as isize),
                    (k - 15 as i32 / 2 as i32) * 65536 as i32 / (15 as i32 / 2 as i32),
                );
                k += 1
            }
        }
        i += 1
    }
    (*vbi).ampmax = global_ampmax;
    /*
      the next phases are performed once for vbr-only and PACKETBLOB
      times for bitrate managed modes.

      1) encode actual mode being used
      2) encode the floor for each channel, compute coded mask curve/res
      3) normalize and couple.
      4) encode residue
      5) save packet bytes to the packetblob vector

    */
    /* iterate over the many masking curve fits we've created */
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut i32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut couple_bundle: *mut *mut i32 = fresh11.as_mut_ptr() as *mut *mut i32;
    let mut fresh12 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut zerobundle: *mut i32 = fresh12.as_mut_ptr() as *mut i32;
    k = if vorbis_bitrate_managed(vb as *mut vorbis_block) != 0 {
        0 as i32
    } else {
        (15 as i32) / 2 as i32
    };
    while k
        <= (if vorbis_bitrate_managed(vb as *mut vorbis_block) != 0 {
            (15 as i32) - 1 as i32
        } else {
            (15 as i32) / 2 as i32
        })
    {
        let mut opb: *mut oggpack_buffer = (*vbi).packetblob[k as usize];
        /* ok, done encoding.  Next protopacket. */
        oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 1 as i32);
        oggpack_write(
            opb as *mut oggpack_buffer,
            modenumber as usize,
            (*b).modebits,
        );
        if (*vb).W != 0 {
            oggpack_write(opb as *mut oggpack_buffer, (*vb).lW as usize, 1 as i32);
            oggpack_write(opb as *mut oggpack_buffer, (*vb).nW as usize, 1 as i32);
        }
        i = 0 as i32;
        while i < (*vi).channels {
            let mut submap_0: i32 = (*info).chmuxlist[i as usize];
            let mut ilogmask: *mut i32 = *iwork.offset(i as isize);
            *nonzero.offset(i as isize) = floor1_encode(
                opb as *mut oggpack_buffer,
                vb as *mut vorbis_block,
                *(*b)
                    .flr
                    .offset((*info).floorsubmap[submap_0 as usize] as isize)
                    as *mut vorbis_look_floor1 as *mut vorbis_look_floor1,
                *(*floor_posts.offset(i as isize)).offset(k as isize),
                ilogmask,
            );
            i += 1
        }
        _vp_couple_quantize_normalize(
            k,
            &mut (*ci).psy_g_param as *mut _ as *mut vorbis_info_psy_global,
            psy_look as *mut vorbis_look_psy,
            info as *mut vorbis_info_mapping0,
            gmdct,
            iwork,
            nonzero,
            (*ci).psy_g_param.sliding_lowpass[(*vb).W as usize][k as usize],
            (*vi).channels,
        );
        i = 0 as i32;
        while i < (*info).submaps {
            let mut ch_in_bundle: i32 = 0 as i32;
            let mut classifications: *mut *mut isize = 0 as *mut *mut isize;
            let mut resnum: i32 = (*info).residuesubmap[i as usize];
            j = 0 as i32;
            while j < (*vi).channels {
                if (*info).chmuxlist[j as usize] == i {
                    *zerobundle.offset(ch_in_bundle as isize) = 0 as i32;
                    if *nonzero.offset(j as isize) != 0 {
                        *zerobundle.offset(ch_in_bundle as isize) = 1 as i32
                    }
                    let fresh13 = ch_in_bundle;
                    ch_in_bundle = ch_in_bundle + 1;
                    let ref mut fresh14 = *couple_bundle.offset(fresh13 as isize);
                    *fresh14 = *iwork.offset(j as isize)
                }
                j += 1
            }
            classifications = (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                .as_ptr()
                .offset((*ci).residue_type[resnum as usize] as isize))
            .class
            .expect("non-null function pointer")(
                vb,
                *(*b).residue.offset(resnum as isize),
                couple_bundle,
                zerobundle,
                ch_in_bundle,
            );
            ch_in_bundle = 0 as i32;
            j = 0 as i32;
            while j < (*vi).channels {
                if (*info).chmuxlist[j as usize] == i {
                    let fresh15 = ch_in_bundle;
                    ch_in_bundle = ch_in_bundle + 1;
                    let ref mut fresh16 = *couple_bundle.offset(fresh15 as isize);
                    *fresh16 = *iwork.offset(j as isize)
                }
                j += 1
            }
            (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                .as_ptr()
                .offset((*ci).residue_type[resnum as usize] as isize))
            .forward
            .expect("non-null function pointer")(
                opb,
                vb,
                *(*b).residue.offset(resnum as isize),
                couple_bundle,
                zerobundle,
                ch_in_bundle,
                classifications,
                i,
            );
            i += 1
        }
        k += 1
    }
    return 0 as i32;
}

unsafe extern "C" fn mapping0_inverse(mut vb: *mut vorbis_block, mut l: *mut libc::c_void) -> i32 {
    let mut vd: *mut vorbis_dsp_state = (*vb).vd;
    let mut vi: *mut vorbis_info = (*vd).vi;
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    let mut b: *mut private_state = (*vd).backend_state as *mut private_state;
    let mut info: *mut vorbis_info_mapping0 = l as *mut vorbis_info_mapping0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    (*vb).pcmend = (*ci).blocksizes[(*vb).W as usize] as i32;
    let mut n: isize = (*vb).pcmend as isize;
    let mut fresh17 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut f32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut pcmbundle: *mut *mut f32 = fresh17.as_mut_ptr() as *mut *mut f32;
    let mut fresh18 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut zerobundle: *mut i32 = fresh18.as_mut_ptr() as *mut i32;
    let mut fresh19 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<i32>() as usize).wrapping_mul((*vi).channels as usize) as usize,
    );
    let mut nonzero: *mut i32 = fresh19.as_mut_ptr() as *mut i32;
    let mut fresh20 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut libc::c_void>() as usize).wrapping_mul((*vi).channels as usize)
            as usize,
    );
    let mut floormemo: *mut *mut libc::c_void = fresh20.as_mut_ptr() as *mut *mut libc::c_void;
    /* start out our new packet blob with packet type and mode */
    /* Encode the packet type */
    /* Encode the modenumber */
    /* Encode frame mode, pre,post windowsize, then dispatch */
    /* encode floor, compute masking curve, sep out residue */
    /* our iteration is now based on masking curve, not prequant and
    coupling.  Only one prequant/coupling step */
    /* quantize/couple */
    /* incomplete implementation that assumes the tree is all depth
    one, or no tree at all */
    /* classify and encode by submap */
    /* recover the spectral envelope; store it in the PCM vector for now */
    i = 0 as i32;
    while i < (*vi).channels {
        let mut submap: i32 = (*info).chmuxlist[i as usize];
        let ref mut fresh21 = *floormemo.offset(i as isize);
        *fresh21 = (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
            .as_ptr()
            .offset((*ci).floor_type[(*info).floorsubmap[submap as usize] as usize] as isize))
        .inverse1
        .expect("non-null function pointer")(
            vb,
            *(*b)
                .flr
                .offset((*info).floorsubmap[submap as usize] as isize),
        );
        if !(*floormemo.offset(i as isize)).is_null() {
            *nonzero.offset(i as isize) = 1 as i32
        } else {
            *nonzero.offset(i as isize) = 0 as i32
        }
        crate::stdlib::memset(
            *(*vb).pcm.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            (::std::mem::size_of::<f32>() as usize)
                .wrapping_mul(n as usize)
                .wrapping_div(2 as i32 as usize),
        );
        i += 1
    }
    /* channel coupling can 'dirty' the nonzero listing */
    i = 0 as i32;
    while i < (*info).coupling_steps {
        if *nonzero.offset((*info).coupling_mag[i as usize] as isize) != 0
            || *nonzero.offset((*info).coupling_ang[i as usize] as isize) != 0
        {
            *nonzero.offset((*info).coupling_mag[i as usize] as isize) = 1 as i32;
            *nonzero.offset((*info).coupling_ang[i as usize] as isize) = 1 as i32
        }
        i += 1
    }
    /* recover the residue into our working vectors */
    i = 0 as i32;
    while i < (*info).submaps {
        let mut ch_in_bundle: i32 = 0 as i32;
        j = 0 as i32;
        while j < (*vi).channels {
            if (*info).chmuxlist[j as usize] == i {
                if *nonzero.offset(j as isize) != 0 {
                    *zerobundle.offset(ch_in_bundle as isize) = 1 as i32
                } else {
                    *zerobundle.offset(ch_in_bundle as isize) = 0 as i32
                }
                let fresh22 = ch_in_bundle;
                ch_in_bundle = ch_in_bundle + 1;
                let ref mut fresh23 = *pcmbundle.offset(fresh22 as isize);
                *fresh23 = *(*vb).pcm.offset(j as isize)
            }
            j += 1
        }
        (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
            .as_ptr()
            .offset((*ci).residue_type[(*info).residuesubmap[i as usize] as usize] as isize))
        .inverse
        .expect("non-null function pointer")(
            vb,
            *(*b)
                .residue
                .offset((*info).residuesubmap[i as usize] as isize),
            pcmbundle,
            zerobundle,
            ch_in_bundle,
        );
        i += 1
    }
    /* channel coupling */
    i = (*info).coupling_steps - 1 as i32;
    while i >= 0 as i32 {
        let mut pcmM: *mut f32 = *(*vb).pcm.offset((*info).coupling_mag[i as usize] as isize);
        let mut pcmA: *mut f32 = *(*vb).pcm.offset((*info).coupling_ang[i as usize] as isize);
        j = 0 as i32;
        while (j as isize) < n / 2 as i32 as isize {
            let mut mag: f32 = *pcmM.offset(j as isize);
            let mut ang: f32 = *pcmA.offset(j as isize);
            if mag > 0 as i32 as f32 {
                if ang > 0 as i32 as f32 {
                    *pcmM.offset(j as isize) = mag;
                    *pcmA.offset(j as isize) = mag - ang
                } else {
                    *pcmA.offset(j as isize) = mag;
                    *pcmM.offset(j as isize) = mag + ang
                }
            } else if ang > 0 as i32 as f32 {
                *pcmM.offset(j as isize) = mag;
                *pcmA.offset(j as isize) = mag + ang
            } else {
                *pcmA.offset(j as isize) = mag;
                *pcmM.offset(j as isize) = mag - ang
            }
            j += 1
        }
        i -= 1
    }
    /* compute and apply spectral envelope */
    i = 0 as i32;
    while i < (*vi).channels {
        let mut pcm: *mut f32 = *(*vb).pcm.offset(i as isize);
        let mut submap_0: i32 = (*info).chmuxlist[i as usize];
        (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
            .as_ptr()
            .offset((*ci).floor_type[(*info).floorsubmap[submap_0 as usize] as usize] as isize))
        .inverse2
        .expect("non-null function pointer")(
            vb,
            *(*b)
                .flr
                .offset((*info).floorsubmap[submap_0 as usize] as isize),
            *floormemo.offset(i as isize),
            pcm,
        );
        i += 1
    }
    /* transform the PCM data; takes PCM vector, vb; modifies PCM vector */
    /* only MDCT right now.... */
    i = 0 as i32;
    while i < (*vi).channels {
        let mut pcm_0: *mut f32 = *(*vb).pcm.offset(i as isize);
        mdct_backward(
            *(*b).transform[(*vb).W as usize].offset(0 as i32 as isize) as *mut mdct_lookup
                as *mut mdct_lookup,
            pcm_0,
            pcm_0,
        );
        i += 1
    }
    /* all done! */
    return 0 as i32;
}
/* export hooks */
#[no_mangle]

pub static mut mapping0_exportbundle: vorbis_func_mapping = {
    let mut init = vorbis_func_mapping {
        pack: Some(
            mapping0_pack
                as unsafe extern "C" fn(
                    _: *mut vorbis_info,
                    _: *mut libc::c_void,
                    _: *mut oggpack_buffer,
                ) -> (),
        ),
        unpack: Some(
            mapping0_unpack
                as unsafe extern "C" fn(
                    _: *mut vorbis_info,
                    _: *mut oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(mapping0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        forward: Some(mapping0_forward as unsafe extern "C" fn(_: *mut vorbis_block) -> i32),
        inverse: Some(
            mapping0_inverse
                as unsafe extern "C" fn(_: *mut vorbis_block, _: *mut libc::c_void) -> i32,
        ),
    };
    init
};

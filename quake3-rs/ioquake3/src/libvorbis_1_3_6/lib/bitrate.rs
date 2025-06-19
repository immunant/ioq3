// =============== BEGIN bitrate_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitrate_manager_state {
    pub managed: i32,
    pub avg_reservoir: isize,
    pub minmax_reservoir: isize,
    pub avg_bitsper: isize,
    pub min_bitsper: isize,
    pub max_bitsper: isize,
    pub short_per_long: isize,
    pub avgfloat: f64,
    pub vb: *mut crate::codec_h::vorbis_block,
    pub choice: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitrate_manager_info {
    pub avg_rate: isize,
    pub min_rate: isize,
    pub max_rate: isize,
    pub reservoir_bits: isize,
    pub reservoir_bias: f64,
    pub slew_damp: f64,
}
use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

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
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_get_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writetrunc;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;

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

function: bitrate tracking and management

********************************************************************/
/* compute bitrate tracking setup  */
#[no_mangle]

pub unsafe extern "C" fn vorbis_bitrate_init(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut bm: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state,
) {
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut bi: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info =
        &mut (*ci).bi;
    crate::stdlib::memset(
        bm as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state>()
            as libc::c_ulong,
    );
    if !bi.is_null() && (*bi).reservoir_bits > 0 as i32 as isize {
        let mut ratesamples: isize = (*vi).rate;
        let mut halfsamples: i32 = ((*ci).blocksizes[0 as i32 as usize] >> 1 as i32) as i32;
        (*bm).short_per_long =
            (*ci).blocksizes[1 as i32 as usize] / (*ci).blocksizes[0 as i32 as usize];
        (*bm).managed = 1 as i32;
        (*bm).avg_bitsper = crate::stdlib::rint(
            1.0f64 * (*bi).avg_rate as f64 * halfsamples as f64 / ratesamples as f64,
        ) as isize;
        (*bm).min_bitsper = crate::stdlib::rint(
            1.0f64 * (*bi).min_rate as f64 * halfsamples as f64 / ratesamples as f64,
        ) as isize;
        (*bm).max_bitsper = crate::stdlib::rint(
            1.0f64 * (*bi).max_rate as f64 * halfsamples as f64 / ratesamples as f64,
        ) as isize;
        (*bm).avgfloat = (15 as i32 / 2 as i32) as f64;
        /* not a necessary fix, but one that leads to a more balanced
        typical initialization */
        let mut desired_fill: isize =
            ((*bi).reservoir_bits as f64 * (*bi).reservoir_bias) as isize;
        (*bm).minmax_reservoir = desired_fill;
        (*bm).avg_reservoir = desired_fill
    };
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_bitrate_clear(
    mut bm: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state,
) {
    crate::stdlib::memset(
        bm as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state>()
            as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_bitrate_managed(mut vb: *mut crate::codec_h::vorbis_block) -> i32 {
    let mut vd: *mut crate::codec_h::vorbis_dsp_state = (*vb).vd;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*vd).backend_state as *mut crate::codec_internal_h::private_state;
    let mut bm: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state =
        &mut (*b).bms;
    if !bm.is_null() && (*bm).managed != 0 {
        return 1 as i32;
    }
    return 0 as i32;
}
/* finish taking in the block we just processed */
#[no_mangle]

pub unsafe extern "C" fn vorbis_bitrate_addblock(mut vb: *mut crate::codec_h::vorbis_block) -> i32 {
    let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
        (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
    let mut vd: *mut crate::codec_h::vorbis_dsp_state = (*vb).vd;
    let mut b: *mut crate::codec_internal_h::private_state =
        (*vd).backend_state as *mut crate::codec_internal_h::private_state;
    let mut bm: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state =
        &mut (*b).bms;
    let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut bi: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info =
        &mut (*ci).bi;
    let mut choice: i32 = crate::stdlib::rint((*bm).avgfloat) as i32;
    let mut this_bits: isize = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
        (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
    ) * 8 as i32 as isize;
    let mut min_target_bits: isize = if (*vb).W != 0 {
        ((*bm).min_bitsper) * (*bm).short_per_long
    } else {
        (*bm).min_bitsper
    };
    let mut max_target_bits: isize = if (*vb).W != 0 {
        ((*bm).max_bitsper) * (*bm).short_per_long
    } else {
        (*bm).max_bitsper
    };
    let mut samples: i32 = ((*ci).blocksizes[(*vb).W as usize] >> 1 as i32) as i32;
    let mut desired_fill: isize =
        ((*bi).reservoir_bits as f64 * (*bi).reservoir_bias) as isize;
    if (*bm).managed == 0 {
        /* not a bitrate managed stream, but for API simplicity, we'll
        buffer the packet to keep the code path clean */
        if !(*bm).vb.is_null() {
            return -(1 as i32);
        } /* one has been submitted without
          being claimed */
        (*bm).vb = vb;
        return 0 as i32;
    }
    (*bm).vb = vb;
    /* look ahead for avg floater */
    if (*bm).avg_bitsper > 0 as i32 as isize {
        let mut slew: f64 = 0.0f64;
        let mut avg_target_bits: isize = if (*vb).W != 0 {
            ((*bm).avg_bitsper) * (*bm).short_per_long
        } else {
            (*bm).avg_bitsper
        };
        let mut slewlimit: f64 = 15.0f64 / (*bi).slew_damp;
        /* choosing a new floater:
        if we're over target, we slew down
        if we're under target, we slew up

        choose slew as follows: look through packetblobs of this frame
        and set slew as the first in the appropriate direction that
        gives us the slew we want.  This may mean no slew if delta is
        already favorable.

        Then limit slew to slew max */
        if (*bm).avg_reservoir + (this_bits - avg_target_bits) > desired_fill {
            while choice > 0 as i32
                && this_bits > avg_target_bits
                && (*bm).avg_reservoir + (this_bits - avg_target_bits) > desired_fill
            {
                choice -= 1;
                this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                ) * 8 as i32 as isize
            }
        } else if (*bm).avg_reservoir + (this_bits - avg_target_bits) < desired_fill {
            while (choice + 1 as i32) < 15 as i32
                && this_bits < avg_target_bits
                && (*bm).avg_reservoir + (this_bits - avg_target_bits) < desired_fill
            {
                choice += 1;
                this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                ) * 8 as i32 as isize
            }
        }
        slew = crate::stdlib::rint(choice as f64 - (*bm).avgfloat) / samples as f64
            * (*vi).rate as f64;
        if slew < -slewlimit {
            slew = -slewlimit
        }
        if slew > slewlimit {
            slew = slewlimit
        }
        (*bm).avgfloat += slew / (*vi).rate as f64 * samples as f64;
        choice = crate::stdlib::rint((*bm).avgfloat) as i32;
        this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        ) * 8 as i32 as isize
    }
    /* enforce min(if used) on the current floater (if used) */
    if (*bm).min_bitsper > 0 as i32 as isize {
        /* do we need to force the bitrate up? */
        if this_bits < min_target_bits {
            while (*bm).minmax_reservoir - (min_target_bits - this_bits) < 0 as i32 as isize
            {
                choice += 1;
                if choice >= 15 as i32 {
                    break;
                }
                this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                ) * 8 as i32 as isize
            }
        }
    }
    /* enforce max (if used) on the current floater (if used) */
    if (*bm).max_bitsper > 0 as i32 as isize {
        /* do we need to force the bitrate down? */
        if this_bits > max_target_bits {
            while (*bm).minmax_reservoir + (this_bits - max_target_bits) > (*bi).reservoir_bits {
                choice -= 1;
                if choice < 0 as i32 {
                    break;
                }
                this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                    (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                ) * 8 as i32 as isize
            }
        }
    }
    /* Choice of packetblobs now made based on floater, and min/max
    requirements. Now boundary check extreme choices */
    if choice < 0 as i32 {
        /* choosing a smaller packetblob is insufficient to trim bitrate.
        frame will need to be truncated */
        let mut maxsize: isize = (max_target_bits
            + ((*bi).reservoir_bits - (*bm).minmax_reservoir))
            / 8 as i32 as isize;
        choice = 0 as i32;
        (*bm).choice = choice;
        if crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        ) > maxsize
        {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_writetrunc(
                (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                maxsize * 8 as i32 as isize,
            );
            this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
                (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
            ) * 8 as i32 as isize
        }
    } else {
        let mut minsize: isize = (min_target_bits - (*bm).minmax_reservoir
            + 7 as i32 as isize)
            / 8 as i32 as isize;
        if choice >= 15 as i32 {
            choice = 15 as i32 - 1 as i32
        }
        (*bm).choice = choice;
        /* prop up bitrate according to demand. pad this frame out with zeroes */
        minsize -= crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        );
        loop {
            let fresh0 = minsize;
            minsize = minsize - 1;
            if !(fresh0 > 0 as i32 as isize) {
                break;
            }
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
                0 as i32 as libc::c_ulong,
                8 as i32,
            );
        }
        this_bits = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        ) * 8 as i32 as isize
    }
    /* now we have the final packet and the final packet size.  Update statistics */
    /* min and max reservoir */
    if (*bm).min_bitsper > 0 as i32 as isize || (*bm).max_bitsper > 0 as i32 as isize
    {
        if max_target_bits > 0 as i32 as isize && this_bits > max_target_bits {
            (*bm).minmax_reservoir += this_bits - max_target_bits
        } else if min_target_bits > 0 as i32 as isize && this_bits < min_target_bits {
            (*bm).minmax_reservoir += this_bits - min_target_bits
        } else if (*bm).minmax_reservoir > desired_fill {
            if max_target_bits > 0 as i32 as isize {
                /* inbetween; we want to take reservoir toward but not past desired_fill */
                /* logical bulletproofing against initialization state */
                (*bm).minmax_reservoir += this_bits - max_target_bits;
                if (*bm).minmax_reservoir < desired_fill {
                    (*bm).minmax_reservoir = desired_fill
                }
            } else {
                (*bm).minmax_reservoir = desired_fill
            }
        } else if min_target_bits > 0 as i32 as isize {
            /* logical bulletproofing against initialization state */
            (*bm).minmax_reservoir += this_bits - min_target_bits;
            if (*bm).minmax_reservoir > desired_fill {
                (*bm).minmax_reservoir = desired_fill
            }
        } else {
            (*bm).minmax_reservoir = desired_fill
        }
    }
    /* avg reservoir */
    if (*bm).avg_bitsper > 0 as i32 as isize {
        let mut avg_target_bits_0: isize = if (*vb).W != 0 {
            ((*bm).avg_bitsper) * (*bm).short_per_long
        } else {
            (*bm).avg_bitsper
        };
        (*bm).avg_reservoir += this_bits - avg_target_bits_0
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_bitrate_flushpacket(
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut b: *mut crate::codec_internal_h::private_state =
        (*vd).backend_state as *mut crate::codec_internal_h::private_state;
    let mut bm: *mut crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state =
        &mut (*b).bms;
    let mut vb: *mut crate::codec_h::vorbis_block = (*bm).vb;
    let mut choice: i32 = 15 as i32 / 2 as i32;
    if vb.is_null() {
        return 0 as i32;
    }
    if !op.is_null() {
        let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
            (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
        if vorbis_bitrate_managed(vb) != 0 {
            choice = (*bm).choice
        }
        (*op).packet = crate::src::libogg_1_3_3::src::bitwise::oggpack_get_buffer(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        );
        (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            (*vbi).packetblob[choice as usize] as *mut crate::ogg_h::oggpack_buffer,
        );
        (*op).b_o_s = 0 as i32 as isize;
        (*op).e_o_s = (*vb).eofflag as isize;
        (*op).granulepos = (*vb).granulepos;
        (*op).packetno = (*vb).sequence
        /* for sake of completeness */
    }
    (*bm).vb = 0 as *mut crate::codec_h::vorbis_block;
    return 1 as i32;
}

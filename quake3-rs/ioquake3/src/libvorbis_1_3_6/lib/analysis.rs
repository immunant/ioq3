use ::libc;

pub use crate::config_types_h::ogg_int64_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::backends_h::vorbis_func_mapping;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::vorbis_block_internal;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_get_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_reset;

/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2007             *
* by the Xiph.Org Foundation http://www.xiph.org/                  *
*                                                                  *
********************************************************************

function: single-block PCM analysis mode dispatch

********************************************************************/
/* decides between modes, dispatches to the appropriate mapping. */
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut op: *mut crate::ogg_h::ogg_packet,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    let mut vbi: *mut crate::codec_internal_h::vorbis_block_internal =
        (*vb).internal as *mut crate::codec_internal_h::vorbis_block_internal;
    (*vb).glue_bits = 0 as i32 as isize;
    (*vb).time_bits = 0 as i32 as isize;
    (*vb).floor_bits = 0 as i32 as isize;
    (*vb).res_bits = 0 as i32 as isize;
    /* first things first.  Make sure encode is ready */
    i = 0 as i32;
    while i < 15 as i32 {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_reset(
            (*vbi).packetblob[i as usize] as *mut crate::ogg_h::oggpack_buffer,
        );
        i += 1
    }
    /* we only have one mapping type (0), and we let the mapping code
    itself figure out what soft mode to use.  This allows easier
    bitrate management */
    ret = (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
        .as_ptr()
        .offset(0 as i32 as isize))
    .forward
    .expect("non-null function pointer")(vb);
    if ret != 0 {
        return ret;
    }
    if !op.is_null() {
        if crate::src::libvorbis_1_3_6::lib::bitrate::vorbis_bitrate_managed(
            vb as *mut crate::codec_h::vorbis_block,
        ) != 0
        {
            /* The app is using a bitmanaged mode... but not using the
            bitrate management interface. */
            return -(131 as i32);
        }
        (*op).packet = crate::src::libogg_1_3_3::src::bitwise::oggpack_get_buffer(
            &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        );
        (*op).bytes = crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes(
            &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        );
        (*op).b_o_s = 0 as i32 as isize;
        (*op).e_o_s = (*vb).eofflag as isize;
        (*op).granulepos = (*vb).granulepos;
        (*op).packetno = (*vb).sequence
        /* for sake of completeness */
    }
    return 0 as i32;
}

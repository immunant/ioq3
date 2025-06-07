pub use crate::config_types_h::ogg_int64_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_func_mapping;
pub use crate::backends_h::vorbis_func_residue;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_residue;
extern "C" {
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

    function: registry for time, floor, res backends and channel mappings

    ********************************************************************/
    /* seems like major overkill now; the backend numbers will grow into
    the infrastructure soon enough */
    #[no_mangle]
    pub static floor0_exportbundle: crate::backends_h::vorbis_func_floor;
    #[no_mangle]
    pub static floor1_exportbundle: crate::backends_h::vorbis_func_floor;
    #[no_mangle]
    pub static residue0_exportbundle: crate::backends_h::vorbis_func_residue;
    #[no_mangle]
    pub static residue1_exportbundle: crate::backends_h::vorbis_func_residue;
    #[no_mangle]
    pub static residue2_exportbundle: crate::backends_h::vorbis_func_residue;
    #[no_mangle]
    pub static mapping0_exportbundle: crate::backends_h::vorbis_func_mapping;
}
#[no_mangle]

pub static mut _floor_P: [*const crate::backends_h::vorbis_func_floor; 2] = unsafe {
    [
        &floor0_exportbundle as *const crate::backends_h::vorbis_func_floor,
        &floor1_exportbundle as *const crate::backends_h::vorbis_func_floor,
    ]
};
#[no_mangle]

pub static mut _residue_P: [*const crate::backends_h::vorbis_func_residue; 3] = unsafe {
    [
        &residue0_exportbundle as *const crate::backends_h::vorbis_func_residue,
        &residue1_exportbundle as *const crate::backends_h::vorbis_func_residue,
        &residue2_exportbundle as *const crate::backends_h::vorbis_func_residue,
    ]
};
#[no_mangle]

pub static mut _mapping_P: [*const crate::backends_h::vorbis_func_mapping; 1] =
    unsafe { [&mapping0_exportbundle as *const crate::backends_h::vorbis_func_mapping] };

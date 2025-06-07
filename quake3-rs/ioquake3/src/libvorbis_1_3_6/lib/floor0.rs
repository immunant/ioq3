use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_info_floor0;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_set;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;

pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_floor0 {
    pub ln: libc::c_int,
    pub m: libc::c_int,
    pub linearmap: *mut *mut libc::c_int,
    pub n: [libc::c_int; 2],
    pub vi: *mut crate::backends_h::vorbis_info_floor0,
    pub bits: libc::c_long,
    pub frames: libc::c_long,
}
/* **********************************************/

unsafe extern "C" fn floor0_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut crate::backends_h::vorbis_info_floor0 =
        i as *mut crate::backends_h::vorbis_info_floor0;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_floor0>() as libc::c_ulong,
        );
        ::libc::free(info as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor0_free_look(mut i: *mut libc::c_void) {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    if !look.is_null() {
        if !(*look).linearmap.is_null() {
            if !(*(*look).linearmap.offset(0 as libc::c_int as isize)).is_null() {
                ::libc::free(
                    *(*look).linearmap.offset(0 as libc::c_int as isize) as *mut libc::c_void
                );
            }
            if !(*(*look).linearmap.offset(1 as libc::c_int as isize)).is_null() {
                ::libc::free(
                    *(*look).linearmap.offset(1 as libc::c_int as isize) as *mut libc::c_void
                );
            }
            ::libc::free((*look).linearmap as *mut libc::c_void);
        }
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vorbis_look_floor0>() as libc::c_ulong,
        );
        ::libc::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor0_unpack(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: libc::c_int = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 = crate::stdlib::malloc(
        ::std::mem::size_of::<crate::backends_h::vorbis_info_floor0>() as libc::c_ulong,
    )
        as *mut crate::backends_h::vorbis_info_floor0;
    (*info).order = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as libc::c_int,
    ) as libc::c_int;
    (*info).rate = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        16 as libc::c_int,
    );
    (*info).barkmap = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        16 as libc::c_int,
    );
    (*info).ampbits = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        6 as libc::c_int,
    ) as libc::c_int;
    (*info).ampdB = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as libc::c_int,
    ) as libc::c_int;
    (*info).numbooks = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        4 as libc::c_int,
    ) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    if !((*info).order < 1 as libc::c_int) {
        if !((*info).rate < 1 as libc::c_int as libc::c_long) {
            if !((*info).barkmap < 1 as libc::c_int as libc::c_long) {
                if !((*info).numbooks < 1 as libc::c_int) {
                    j = 0 as libc::c_int;
                    loop {
                        if !(j < (*info).numbooks) {
                            current_block = 5948590327928692120;
                            break;
                        }
                        (*info).books[j as usize] =
                            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                opb as *mut crate::ogg_h::oggpack_buffer,
                                8 as libc::c_int,
                            ) as libc::c_int;
                        if (*info).books[j as usize] < 0 as libc::c_int
                            || (*info).books[j as usize] >= (*ci).books
                        {
                            current_block = 8027669132317143458;
                            break;
                        }
                        if (*(*ci).book_param[(*info).books[j as usize] as usize]).maptype
                            == 0 as libc::c_int
                        {
                            current_block = 8027669132317143458;
                            break;
                        }
                        if (*(*ci).book_param[(*info).books[j as usize] as usize]).dim
                            < 1 as libc::c_int as libc::c_long
                        {
                            current_block = 8027669132317143458;
                            break;
                        }
                        j += 1
                    }
                    match current_block {
                        8027669132317143458 => {}
                        _ => return info as *mut libc::c_void,
                    }
                }
            }
        }
    }
    floor0_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
/* initialize Bark scale and normalization lookups.  We could do this
with static tables, but Vorbis allows a number of possible
combinations, so it's best to do it computationally.

The below is authoritative in terms of defining scale mapping.
Note that the scale depends on the sampling rate as well as the
linear block and mapping sizes */

unsafe extern "C" fn floor0_map_lazy_init(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut infoX: *mut libc::c_void,
    mut look: *mut vorbis_look_floor0,
) {
    if (*(*look).linearmap.offset((*vb).W as isize)).is_null() {
        let mut vd: *mut crate::codec_h::vorbis_dsp_state = (*vb).vd;
        let mut vi: *mut crate::codec_h::vorbis_info = (*vd).vi;
        let mut ci: *mut crate::codec_internal_h::codec_setup_info =
            (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
        let mut info: *mut crate::backends_h::vorbis_info_floor0 =
            infoX as *mut crate::backends_h::vorbis_info_floor0;
        let mut W: libc::c_int = (*vb).W as libc::c_int;
        let mut n: libc::c_int =
            ((*ci).blocksizes[W as usize] / 2 as libc::c_int as libc::c_long) as libc::c_int;
        let mut j: libc::c_int = 0;
        /* we choose a scaling constant so that:
          floor(bark(rate/2-1)*C)=mapped-1
        floor(bark(rate/2)*C)=mapped */
        let mut scale: libc::c_float = ((*look).ln as libc::c_double
            / (13.1f32 as libc::c_double
                * crate::stdlib::atan(
                    (0.00074f32 * ((*info).rate as libc::c_float / 2.0f32)) as libc::c_double,
                )
                + 2.24f32 as libc::c_double
                    * crate::stdlib::atan(
                        ((*info).rate as libc::c_float / 2.0f32
                            * ((*info).rate as libc::c_float / 2.0f32)
                            * 1.85e-8f32) as libc::c_double,
                    )
                + (1e-4f32 * ((*info).rate as libc::c_float / 2.0f32)) as libc::c_double))
            as libc::c_float;
        /* the mapping from a linear scale to a smaller bark scale is
        straightforward.  We do *not* make sure that the linear mapping
        does not skip bark-scale bins; the decoder simply skips them and
        the encoder may do what it wishes in filling them.  They're
        necessary in some mapping combinations to keep the scale spacing
        accurate */
        let ref mut fresh0 = *(*look).linearmap.offset(W as isize); /* bark numbers represent band edges */
        *fresh0 = crate::stdlib::malloc(
            ((n + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int; /* guard against the approximation */
        j = 0 as libc::c_int;
        while j < n {
            let mut val: libc::c_int = crate::stdlib::floor(
                (13.1f32 as libc::c_double
                    * crate::stdlib::atan(
                        (0.00074f32
                            * ((*info).rate as libc::c_float / 2.0f32 / n as libc::c_float
                                * j as libc::c_float)) as libc::c_double,
                    )
                    + 2.24f32 as libc::c_double
                        * crate::stdlib::atan(
                            ((*info).rate as libc::c_float / 2.0f32 / n as libc::c_float
                                * j as libc::c_float
                                * ((*info).rate as libc::c_float / 2.0f32 / n as libc::c_float
                                    * j as libc::c_float)
                                * 1.85e-8f32) as libc::c_double,
                        )
                    + (1e-4f32
                        * ((*info).rate as libc::c_float / 2.0f32 / n as libc::c_float
                            * j as libc::c_float)) as libc::c_double)
                    * scale as libc::c_double,
            ) as libc::c_int;
            if val >= (*look).ln {
                val = (*look).ln - 1 as libc::c_int
            }
            *(*(*look).linearmap.offset(W as isize)).offset(j as isize) = val;
            j += 1
        }
        *(*(*look).linearmap.offset(W as isize)).offset(j as isize) = -(1 as libc::c_int);
        (*look).n[W as usize] = n
    };
}

unsafe extern "C" fn floor0_look(
    mut _vd: *mut crate::codec_h::vorbis_dsp_state,
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut info: *mut crate::backends_h::vorbis_info_floor0 =
        i as *mut crate::backends_h::vorbis_info_floor0;
    let mut look: *mut vorbis_look_floor0 = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<vorbis_look_floor0>() as libc::c_ulong,
    ) as *mut vorbis_look_floor0;
    (*look).m = (*info).order;
    (*look).ln = (*info).barkmap as libc::c_int;
    (*look).vi = info;
    (*look).linearmap = crate::stdlib::calloc(
        2 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
    ) as *mut *mut libc::c_int;
    return look as *mut libc::c_void;
}

unsafe extern "C" fn floor0_inverse1(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 = (*look).vi;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ampraw: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        (*info).ampbits,
    ) as libc::c_int;
    if ampraw > 0 as libc::c_int {
        /* also handles the -1 out of data case */
        let mut maxval: libc::c_long =
            (((1 as libc::c_int) << (*info).ampbits) - 1 as libc::c_int) as libc::c_long;
        let mut amp: libc::c_float =
            ampraw as libc::c_float / maxval as libc::c_float * (*info).ampdB as libc::c_float;
        let mut booknum: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                (*info).numbooks as crate::config_types_h::ogg_uint32_t,
            ),
        ) as libc::c_int;
        if booknum != -(1 as libc::c_int) && booknum < (*info).numbooks {
            /* be paranoid */
            let mut ci: *mut crate::codec_internal_h::codec_setup_info =
                (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
            let mut b: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci)
                .fullbooks
                .offset((*info).books[booknum as usize] as isize);
            let mut last: libc::c_float = 0.0f32;
            /* the additional b->dim is a guard against any possible stack
            smash; b->dim is provably more than we can overflow the
            vector */
            let mut lsp: *mut libc::c_float =
                crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                    vb as *mut crate::codec_h::vorbis_block,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong).wrapping_mul(
                        ((*look).m as libc::c_long + (*b).dim + 1 as libc::c_int as libc::c_long)
                            as libc::c_ulong,
                    ) as libc::c_long,
                ) as *mut libc::c_float;
            if !(crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_set(
                b as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                lsp,
                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                (*look).m,
            ) == -(1 as libc::c_int) as libc::c_long)
            {
                j = 0 as libc::c_int;
                while j < (*look).m {
                    k = 0 as libc::c_int;
                    while j < (*look).m && (k as libc::c_long) < (*b).dim {
                        *lsp.offset(j as isize) += last;
                        k += 1;
                        j += 1
                    }
                    last = *lsp.offset((j - 1 as libc::c_int) as isize)
                }
                *lsp.offset((*look).m as isize) = amp;
                return lsp as *mut libc::c_void;
            }
        }
    }
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor0_inverse2(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut i: *mut libc::c_void,
    mut memo: *mut libc::c_void,
    mut out: *mut libc::c_float,
) -> libc::c_int {
    let mut look: *mut vorbis_look_floor0 = i as *mut vorbis_look_floor0;
    let mut info: *mut crate::backends_h::vorbis_info_floor0 = (*look).vi;
    floor0_map_lazy_init(vb, info as *mut libc::c_void, look);
    if !memo.is_null() {
        let mut lsp: *mut libc::c_float = memo as *mut libc::c_float;
        let mut amp: libc::c_float = *lsp.offset((*look).m as isize);
        /* take the coefficients back to a spectral envelope curve */
        crate::src::libvorbis_1_3_6::lib::lsp::vorbis_lsp_to_curve(
            out,
            *(*look).linearmap.offset((*vb).W as isize),
            (*look).n[(*vb).W as usize],
            (*look).ln,
            lsp,
            (*look).m,
            amp,
            (*info).ampdB as libc::c_float,
        );
        return 1 as libc::c_int;
    }
    crate::stdlib::memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*look).n[(*vb).W as usize] as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
/* export hooks */
#[no_mangle]

pub static mut floor0_exportbundle: crate::backends_h::vorbis_func_floor = {
    let mut init = crate::backends_h::vorbis_func_floor {
        pack: None,
        unpack: Some(
            floor0_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            floor0_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(floor0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(floor0_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        inverse1: Some(
            floor0_inverse1
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        inverse2: Some(
            floor0_inverse2
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut libc::c_void,
                    _: *mut libc::c_float,
                ) -> libc::c_int,
        ),
    };
    init
};

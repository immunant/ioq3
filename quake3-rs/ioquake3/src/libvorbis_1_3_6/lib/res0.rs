use ::libc;

pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_residue;
pub use crate::backends_h::vorbis_info_residue0;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_add;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodevs_add;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodevv_add;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;

pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vorbis_look_residue0 {
    pub info: *mut crate::backends_h::vorbis_info_residue0,
    pub parts: libc::c_int,
    pub stages: libc::c_int,
    pub fullbooks: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub phrasebook: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub partbooks: *mut *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub partvals: libc::c_int,
    pub decodemap: *mut *mut libc::c_int,
    pub postbits: libc::c_long,
    pub phrasebits: libc::c_long,
    pub frames: libc::c_long,
}
#[no_mangle]

pub unsafe extern "C" fn res0_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut crate::backends_h::vorbis_info_residue0 =
        i as *mut crate::backends_h::vorbis_info_residue0;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_residue0>() as libc::c_ulong,
        );
        ::libc::free(info as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn res0_free_look(mut i: *mut libc::c_void) {
    let mut j: libc::c_int = 0;
    if !i.is_null() {
        let mut look: *mut vorbis_look_residue0 = i as *mut vorbis_look_residue0;
        /*vorbis_info_residue0 *info=look->info;

        fprintf(stderr,
                "%ld frames encoded in %ld phrasebits and %ld residue bits "
                "(%g/frame) \n",look->frames,look->phrasebits,
                look->resbitsflat,
                (look->phrasebits+look->resbitsflat)/(float)look->frames);

        for(j=0;j<look->parts;j++){
          long acc=0;
          fprintf(stderr,"\t[%d] == ",j);
          for(k=0;k<look->stages;k++)
            if((info->secondstages[j]>>k)&1){
              fprintf(stderr,"%ld,",look->resbits[j][k]);
              acc+=look->resbits[j][k];
            }

          fprintf(stderr,":: (%ld vals) %1.2fbits/sample\n",look->resvals[j],
                  acc?(float)acc/(look->resvals[j]*info->grouping):0);
        }
        fprintf(stderr,"\n");*/
        j = 0 as libc::c_int; /* residue vectors to group and
                              code with a partitioned book */
        while j < (*look).parts {
            if !(*(*look).partbooks.offset(j as isize)).is_null() {
                ::libc::free(*(*look).partbooks.offset(j as isize) as *mut libc::c_void);
                /* possible partition choices */
            } /* group huffman book */
            j += 1
        }
        ::libc::free((*look).partbooks as *mut libc::c_void);
        j = 0 as libc::c_int;
        while j < (*look).partvals {
            ::libc::free(*(*look).decodemap.offset(j as isize) as *mut libc::c_void);
            j += 1
        }
        ::libc::free((*look).decodemap as *mut libc::c_void);
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vorbis_look_residue0>() as libc::c_ulong,
        );
        ::libc::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn icount(mut v: libc::c_uint) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    while v != 0 {
        ret = (ret as libc::c_uint).wrapping_add(v & 1 as libc::c_int as libc::c_uint)
            as libc::c_int as libc::c_int;
        v >>= 1 as libc::c_int
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn res0_pack(
    mut vr: *mut libc::c_void,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) {
    let mut info: *mut crate::backends_h::vorbis_info_residue0 =
        vr as *mut crate::backends_h::vorbis_info_residue0;
    let mut j: libc::c_int = 0;
    let mut acc: libc::c_int = 0 as libc::c_int;
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).begin as libc::c_ulong,
        24 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).end as libc::c_ulong,
        24 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*info).grouping - 1 as libc::c_int) as libc::c_ulong,
        24 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*info).partitions - 1 as libc::c_int) as libc::c_ulong,
        6 as libc::c_int,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).groupbook as libc::c_ulong,
        8 as libc::c_int,
    );
    /* secondstages is a bitmask; as encoding progresses pass by pass, a
    bitmask of one indicates this partition class has bits to write
    this pass */
    j = 0 as libc::c_int; /* trailing zero */
    while j < (*info).partitions {
        if crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            (*info).secondstages[j as usize] as crate::config_types_h::ogg_uint32_t,
        ) > 3 as libc::c_int
        {
            /* yes, this is a minor hack due to not thinking ahead */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).secondstages[j as usize] as libc::c_ulong,
                3 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                1 as libc::c_int as libc::c_ulong,
                1 as libc::c_int,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                ((*info).secondstages[j as usize] >> 3 as libc::c_int) as libc::c_ulong,
                5 as libc::c_int,
            );
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).secondstages[j as usize] as libc::c_ulong,
                4 as libc::c_int,
            );
        }
        acc += icount((*info).secondstages[j as usize] as libc::c_uint);
        j += 1
    }
    j = 0 as libc::c_int;
    while j < acc {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (*info).booklist[j as usize] as libc::c_ulong,
            8 as libc::c_int,
        );
        j += 1
    }
}
/* vorbis_info is for range checking */
#[no_mangle]

pub unsafe extern "C" fn res0_unpack(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut acc: libc::c_int = 0 as libc::c_int;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::backends_h::vorbis_info_residue0>() as libc::c_ulong,
    )
        as *mut crate::backends_h::vorbis_info_residue0;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    (*info).begin = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as libc::c_int,
    );
    (*info).end = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as libc::c_int,
    );
    (*info).grouping = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as libc::c_int,
    ) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    (*info).partitions = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        6 as libc::c_int,
    ) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    (*info).groupbook = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as libc::c_int,
    ) as libc::c_int;
    /* check for premature EOP */
    if !((*info).groupbook < 0 as libc::c_int) {
        j = 0 as libc::c_int;
        loop {
            if !(j < (*info).partitions) {
                current_block = 5689001924483802034;
                break;
            }
            let mut cascade: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                opb as *mut crate::ogg_h::oggpack_buffer,
                3 as libc::c_int,
            ) as libc::c_int;
            let mut cflag: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                opb as *mut crate::ogg_h::oggpack_buffer,
                1 as libc::c_int,
            ) as libc::c_int;
            if cflag < 0 as libc::c_int {
                current_block = 3462665044408642796;
                break;
            }
            if cflag != 0 {
                let mut c: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    5 as libc::c_int,
                ) as libc::c_int;
                if c < 0 as libc::c_int {
                    current_block = 3462665044408642796;
                    break;
                }
                cascade |= c << 3 as libc::c_int
            }
            (*info).secondstages[j as usize] = cascade;
            acc += icount(cascade as libc::c_uint);
            j += 1
        }
        match current_block {
            3462665044408642796 => {}
            _ => {
                j = 0 as libc::c_int;
                loop {
                    if !(j < acc) {
                        current_block = 5634871135123216486;
                        break;
                    }
                    let mut book: libc::c_int = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        8 as libc::c_int,
                    ) as libc::c_int;
                    if book < 0 as libc::c_int {
                        current_block = 3462665044408642796;
                        break;
                    }
                    (*info).booklist[j as usize] = book;
                    j += 1
                }
                match current_block {
                    3462665044408642796 => {}
                    _ => {
                        if !((*info).groupbook >= (*ci).books) {
                            j = 0 as libc::c_int;
                            loop {
                                if !(j < acc) {
                                    current_block = 7205609094909031804;
                                    break;
                                }
                                if (*info).booklist[j as usize] >= (*ci).books {
                                    current_block = 3462665044408642796;
                                    break;
                                }
                                if (*(*ci).book_param[(*info).booklist[j as usize] as usize])
                                    .maptype
                                    == 0 as libc::c_int
                                {
                                    current_block = 3462665044408642796;
                                    break;
                                }
                                j += 1
                            }
                            match current_block {
                                3462665044408642796 => {}
                                _ =>
                                /* verify the phrasebook is not specifying an impossible or
                                   inconsistent partitioning scheme. */
                                /* modify the phrasebook ranging check from r16327; an early beta
                                   encoder had a bug where it used an oversized phrasebook by
                                   accident.  These files should continue to be playable, but don't
                                   allow an exploit */
                                {
                                    let mut entries: libc::c_int =
                                        (*(*ci).book_param[(*info).groupbook as usize]).entries
                                            as libc::c_int;
                                    let mut dim: libc::c_int =
                                        (*(*ci).book_param[(*info).groupbook as usize]).dim
                                            as libc::c_int;
                                    let mut partvals: libc::c_int = 1 as libc::c_int;
                                    if !(dim < 1 as libc::c_int) {
                                        loop {
                                            if !(dim > 0 as libc::c_int) {
                                                current_block = 15597372965620363352;
                                                break;
                                            }
                                            partvals *= (*info).partitions;
                                            if partvals > entries {
                                                current_block = 3462665044408642796;
                                                break;
                                            }
                                            dim -= 1
                                        }
                                        match current_block {
                                            3462665044408642796 => {}
                                            _ => {
                                                (*info).partvals = partvals;
                                                return info as *mut libc::c_void;
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
    res0_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
#[no_mangle]

pub unsafe extern "C" fn res0_look(
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
    mut vr: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut info: *mut crate::backends_h::vorbis_info_residue0 =
        vr as *mut crate::backends_h::vorbis_info_residue0;
    let mut look: *mut vorbis_look_residue0 = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<vorbis_look_residue0>() as libc::c_ulong,
    ) as *mut vorbis_look_residue0;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut acc: libc::c_int = 0 as libc::c_int;
    let mut dim: libc::c_int = 0;
    let mut maxstage: libc::c_int = 0 as libc::c_int;
    (*look).info = info;
    (*look).parts = (*info).partitions;
    (*look).fullbooks = (*ci).fullbooks;
    (*look).phrasebook = (*ci).fullbooks.offset((*info).groupbook as isize);
    dim = (*(*look).phrasebook).dim as libc::c_int;
    (*look).partbooks = crate::stdlib::calloc(
        (*look).parts as libc::c_ulong,
        ::std::mem::size_of::<*mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
            as libc::c_ulong,
    )
        as *mut *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook;
    j = 0 as libc::c_int;
    while j < (*look).parts {
        let mut stages: libc::c_int = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            (*info).secondstages[j as usize] as crate::config_types_h::ogg_uint32_t,
        );
        if stages != 0 {
            if stages > maxstage {
                maxstage = stages
            }
            let ref mut fresh0 = *(*look).partbooks.offset(j as isize);
            *fresh0 = crate::stdlib::calloc(
                stages as libc::c_ulong,
                ::std::mem::size_of::<*mut crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
                    as libc::c_ulong,
            )
                as *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook;
            k = 0 as libc::c_int;
            while k < stages {
                if (*info).secondstages[j as usize] & (1 as libc::c_int) << k != 0 {
                    let fresh1 = acc;
                    acc = acc + 1;
                    let ref mut fresh2 =
                        *(*(*look).partbooks.offset(j as isize)).offset(k as isize);
                    *fresh2 = (*ci)
                        .fullbooks
                        .offset((*info).booklist[fresh1 as usize] as isize)
                }
                k += 1
            }
        }
        j += 1
    }
    (*look).partvals = 1 as libc::c_int;
    j = 0 as libc::c_int;
    while j < dim {
        (*look).partvals *= (*look).parts;
        j += 1
    }
    (*look).stages = maxstage;
    (*look).decodemap = crate::stdlib::malloc(
        ((*look).partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    j = 0 as libc::c_int;
    while j < (*look).partvals {
        let mut val: libc::c_long = j as libc::c_long;
        let mut mult: libc::c_long = ((*look).partvals / (*look).parts) as libc::c_long;
        let ref mut fresh3 = *(*look).decodemap.offset(j as isize);
        *fresh3 = crate::stdlib::malloc(
            (dim as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        k = 0 as libc::c_int;
        while k < dim {
            let mut deco: libc::c_long = val / mult;
            val -= deco * mult;
            mult /= (*look).parts as libc::c_long;
            *(*(*look).decodemap.offset(j as isize)).offset(k as isize) = deco as libc::c_int;
            k += 1
        }
        j += 1
    }
    return look as *mut libc::c_void;
}
/* break an abstraction and copy some code for performance purposes */

unsafe extern "C" fn local_book_besterror(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut libc::c_int,
) -> libc::c_int {
    let mut dim: libc::c_int = (*book).dim as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut minval: libc::c_int = (*book).minval;
    let mut del: libc::c_int = (*book).delta;
    let mut qv: libc::c_int = (*book).quantvals;
    let mut ze: libc::c_int = qv >> 1 as libc::c_int;
    let mut index: libc::c_int = 0 as libc::c_int;
    /* assumes integer/centered encoder codebook maptype 1 no more than dim 8 */
    let mut p: [libc::c_int; 8] = [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ];
    if del != 1 as libc::c_int {
        i = 0 as libc::c_int;
        o = dim;
        while i < dim {
            o -= 1;
            let mut v: libc::c_int =
                (*a.offset(o as isize) - minval + (del >> 1 as libc::c_int)) / del;
            let mut m: libc::c_int = if v < ze {
                (ze - v << 1 as libc::c_int) - 1 as libc::c_int
            } else {
                (v - ze) << 1 as libc::c_int
            };
            index = index * qv
                + (if m < 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (if m >= qv { (qv) - 1 as libc::c_int } else { m })
                });
            p[o as usize] = v * del + minval;
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        o = dim;
        while i < dim {
            o -= 1;
            let mut v_0: libc::c_int = *a.offset(o as isize) - minval;
            let mut m_0: libc::c_int = if v_0 < ze {
                (ze - v_0 << 1 as libc::c_int) - 1 as libc::c_int
            } else {
                (v_0 - ze) << 1 as libc::c_int
            };
            index = index * qv
                + (if m_0 < 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (if m_0 >= qv {
                        (qv) - 1 as libc::c_int
                    } else {
                        m_0
                    })
                });
            p[o as usize] = v_0 * del + minval;
            i += 1
        }
    }
    if *(*(*book).c).lengthlist.offset(index as isize) as libc::c_int <= 0 as libc::c_int {
        let mut c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook = (*book).c;
        let mut best: libc::c_int = -(1 as libc::c_int);
        /* assumes integer/centered encoder codebook maptype 1 no more than dim 8 */
        let mut e: [libc::c_int; 8] = [
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ];
        let mut maxval: libc::c_int =
            (*book).minval + (*book).delta * ((*book).quantvals - 1 as libc::c_int);
        i = 0 as libc::c_int;
        while (i as libc::c_long) < (*book).entries {
            if *(*c).lengthlist.offset(i as isize) as libc::c_int > 0 as libc::c_int {
                let mut this: libc::c_int = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < dim {
                    let mut val: libc::c_int = e[j as usize] - *a.offset(j as isize);
                    this += val * val;
                    j += 1
                }
                if best == -(1 as libc::c_int) || this < best {
                    crate::stdlib::memcpy(
                        p.as_mut_ptr() as *mut libc::c_void,
                        e.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong,
                    );
                    best = this;
                    index = i
                }
            }
            /* assumes the value patterning created by the tools in vq/ */
            j = 0 as libc::c_int;
            while e[j as usize] >= maxval {
                let fresh4 = j;
                j = j + 1;
                e[fresh4 as usize] = 0 as libc::c_int
            }
            if e[j as usize] >= 0 as libc::c_int {
                e[j as usize] += (*book).delta
            }
            e[j as usize] = -e[j as usize];
            i += 1
        }
    }
    if index > -(1 as libc::c_int) {
        i = 0 as libc::c_int;
        while i < dim {
            let fresh5 = a;
            a = a.offset(1);
            *fresh5 -= p[i as usize];
            i += 1
        }
    }
    return index;
}

unsafe extern "C" fn _encodepart(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vec: *mut libc::c_int,
    mut n: libc::c_int,
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut bits: libc::c_int = 0 as libc::c_int;
    let mut dim: libc::c_int = (*book).dim as libc::c_int;
    let mut step: libc::c_int = n / dim;
    i = 0 as libc::c_int;
    while i < step {
        let mut entry: libc::c_int = local_book_besterror(book, vec.offset((i * dim) as isize));
        bits += crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
            book as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
            entry,
            opb as *mut crate::ogg_h::oggpack_buffer,
        );
        i += 1
    }
    return bits;
}

unsafe extern "C" fn _01class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut ch: libc::c_int,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: libc::c_int = (*info).grouping;
    let mut possible_partitions: libc::c_int = (*info).partitions;
    let mut n: libc::c_int = ((*info).end - (*info).begin) as libc::c_int;
    let mut partvals: libc::c_int = n / samples_per_partition;
    let mut partword: *mut *mut libc::c_long =
        crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            (ch as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut *mut libc::c_long;
    let mut scale: libc::c_float =
        (100.0f64 / samples_per_partition as libc::c_double) as libc::c_float;
    /* we find the partition type for each partition of each
    channel.  We'll go back and do the interleaved encoding in a
    bit.  For now, clarity */
    i = 0 as libc::c_int as libc::c_long;
    while i < ch as libc::c_long {
        let ref mut fresh6 = *partword.offset(i as isize);
        *fresh6 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            ((n / samples_per_partition) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut libc::c_long;
        crate::stdlib::memset(
            *partword.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            ((n / samples_per_partition) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        );
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < partvals as libc::c_long {
        let mut offset: libc::c_int =
            (i * samples_per_partition as libc::c_long + (*info).begin) as libc::c_int;
        j = 0 as libc::c_int as libc::c_long;
        while j < ch as libc::c_long {
            let mut max: libc::c_int = 0 as libc::c_int;
            let mut ent: libc::c_int = 0 as libc::c_int;
            k = 0 as libc::c_int as libc::c_long;
            while k < samples_per_partition as libc::c_long {
                if ::libc::abs(
                    *(*in_0.offset(j as isize)).offset((offset as libc::c_long + k) as isize),
                ) > max
                {
                    max = ::libc::abs(
                        *(*in_0.offset(j as isize)).offset((offset as libc::c_long + k) as isize),
                    )
                }
                ent += ::libc::abs(
                    *(*in_0.offset(j as isize)).offset((offset as libc::c_long + k) as isize),
                );
                k += 1
            }
            ent = (ent as libc::c_float * scale) as libc::c_int;
            k = 0 as libc::c_int as libc::c_long;
            while k < (possible_partitions - 1 as libc::c_int) as libc::c_long {
                if max <= (*info).classmetric1[k as usize]
                    && ((*info).classmetric2[k as usize] < 0 as libc::c_int
                        || ent < (*info).classmetric2[k as usize])
                {
                    break;
                }
                k += 1
            }
            *(*partword.offset(j as isize)).offset(i as isize) = k;
            j += 1
        }
        i += 1
    }
    (*look).frames += 1;
    return partword;
}
/* designed for stereo or other modes where the partition size is an
integer multiple of the number of channels encoded in the current
submap */

unsafe extern "C" fn _2class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut ch: libc::c_int,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: libc::c_int = (*info).grouping;
    let mut possible_partitions: libc::c_int = (*info).partitions;
    let mut n: libc::c_int = ((*info).end - (*info).begin) as libc::c_int;
    let mut partvals: libc::c_int = n / samples_per_partition;
    let mut partword: *mut *mut libc::c_long =
        crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            ::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong as libc::c_long,
        ) as *mut *mut libc::c_long;
    let ref mut fresh7 = *partword.offset(0 as libc::c_int as isize);
    *fresh7 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut crate::codec_h::vorbis_block,
        (partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_long;
    crate::stdlib::memset(
        *partword.offset(0 as libc::c_int as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    );
    i = 0 as libc::c_int as libc::c_long;
    l = (*info).begin / ch as libc::c_long;
    while i < partvals as libc::c_long {
        let mut magmax: libc::c_int = 0 as libc::c_int;
        let mut angmax: libc::c_int = 0 as libc::c_int;
        j = 0 as libc::c_int as libc::c_long;
        while j < samples_per_partition as libc::c_long {
            if ::libc::abs(*(*in_0.offset(0 as libc::c_int as isize)).offset(l as isize)) > magmax {
                magmax = ::libc::abs(*(*in_0.offset(0 as libc::c_int as isize)).offset(l as isize))
            }
            k = 1 as libc::c_int as libc::c_long;
            while k < ch as libc::c_long {
                if ::libc::abs(*(*in_0.offset(k as isize)).offset(l as isize)) > angmax {
                    angmax = ::libc::abs(*(*in_0.offset(k as isize)).offset(l as isize))
                }
                k += 1
            }
            l += 1;
            j += ch as libc::c_long
        }
        j = 0 as libc::c_int as libc::c_long;
        while j < (possible_partitions - 1 as libc::c_int) as libc::c_long {
            if magmax <= (*info).classmetric1[j as usize]
                && angmax <= (*info).classmetric2[j as usize]
            {
                break;
            }
            j += 1
        }
        *(*partword.offset(0 as libc::c_int as isize)).offset(i as isize) = j;
        i += 1
    }
    (*look).frames += 1;
    return partword;
}

unsafe extern "C" fn _01forward(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut ch: libc::c_int,
    mut partword: *mut *mut libc::c_long,
    mut encode: Option<
        unsafe extern "C" fn(
            _: *mut crate::ogg_h::oggpack_buffer,
            _: *mut libc::c_int,
            _: libc::c_int,
            _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: libc::c_int = (*info).grouping;
    let mut possible_partitions: libc::c_int = (*info).partitions;
    let mut partitions_per_word: libc::c_int = (*(*look).phrasebook).dim as libc::c_int;
    let mut n: libc::c_int = ((*info).end - (*info).begin) as libc::c_int;
    let mut partvals: libc::c_int = n / samples_per_partition;
    let mut resbits: [libc::c_long; 128] = [0; 128];
    let mut resvals: [libc::c_long; 128] = [0; 128];
    crate::stdlib::memset(
        resbits.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_long; 128]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        resvals.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_long; 128]>() as libc::c_ulong,
    );
    /* we code the partition words for each channel, then the residual
    words for a partition per channel until we've written all the
    residual words for that partition word.  Then write the next
    partition channel words... */
    s = 0 as libc::c_int as libc::c_long;
    while s < (*look).stages as libc::c_long {
        i = 0 as libc::c_int as libc::c_long;
        while i < partvals as libc::c_long {
            /* first we encode a partition codeword for each channel */
            if s == 0 as libc::c_int as libc::c_long {
                j = 0 as libc::c_int as libc::c_long;
                while j < ch as libc::c_long {
                    let mut val: libc::c_long = *(*partword.offset(j as isize)).offset(i as isize);
                    k = 1 as libc::c_int as libc::c_long;
                    while k < partitions_per_word as libc::c_long {
                        val *= possible_partitions as libc::c_long;
                        if i + k < partvals as libc::c_long {
                            val += *(*partword.offset(j as isize)).offset((i + k) as isize)
                        }
                        k += 1
                    }
                    /*def TRAIN_RES*/
                    if val < (*(*look).phrasebook).entries {
                        (*look).phrasebits +=
                            crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
                                (*look).phrasebook
                                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                                val as libc::c_int,
                                opb as *mut crate::ogg_h::oggpack_buffer,
                            ) as libc::c_long
                    }
                    j += 1
                }
            }
            /* training hack */
            /* now we encode interleaved residual values for the partitions */
            k = 0 as libc::c_int as libc::c_long;
            while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                let mut offset: libc::c_long =
                    i * samples_per_partition as libc::c_long + (*info).begin;
                j = 0 as libc::c_int as libc::c_long;
                while j < ch as libc::c_long {
                    if s == 0 as libc::c_int as libc::c_long {
                        resvals[*(*partword.offset(j as isize)).offset(i as isize) as usize] +=
                            samples_per_partition as libc::c_long
                    }
                    if (*info).secondstages
                        [*(*partword.offset(j as isize)).offset(i as isize) as usize]
                        & (1 as libc::c_int) << s
                        != 0
                    {
                        let mut statebook: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook =
                            *(*(*look).partbooks.offset(*(*partword.offset(j
                                                                               as
                                                                               isize)).offset(i
                                                                                                  as
                                                                                                  isize)
                                                            as
                                                            isize)).offset(s
                                                                               as
                                                                               isize);
                        if !statebook.is_null() {
                            let mut ret: libc::c_int = 0;
                            ret = encode.expect("non-null function pointer")(
                                opb,
                                (*in_0.offset(j as isize)).offset(offset as isize),
                                samples_per_partition,
                                statebook,
                            );
                            (*look).postbits += ret as libc::c_long;
                            resbits[*(*partword.offset(j as isize)).offset(i as isize) as usize] +=
                                ret as libc::c_long
                        }
                    }
                    j += 1
                }
                k += 1;
                i += 1
            }
        }
        s += 1
    }
    return 0 as libc::c_int;
}
/* a truncated packet here just means 'stop working'; it's not an error */

unsafe extern "C" fn _01inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_float,
    mut ch: libc::c_int,
    mut decodepart: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
            _: *mut libc::c_float,
            _: *mut crate::ogg_h::oggpack_buffer,
            _: libc::c_int,
        ) -> libc::c_long,
    >,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: libc::c_int = (*info).grouping;
    let mut partitions_per_word: libc::c_int = (*(*look).phrasebook).dim as libc::c_int;
    let mut max: libc::c_int = (*vb).pcmend >> 1 as libc::c_int;
    let mut end: libc::c_int = if (*info).end < max as libc::c_long {
        (*info).end
    } else {
        max as libc::c_long
    } as libc::c_int;
    let mut n: libc::c_int = (end as libc::c_long - (*info).begin) as libc::c_int;
    if n > 0 as libc::c_int {
        let mut partvals: libc::c_int = n / samples_per_partition;
        let mut partwords: libc::c_int =
            (partvals + partitions_per_word - 1 as libc::c_int) / partitions_per_word;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            (ch as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut *mut libc::c_int>() as libc::c_ulong)
                as usize,
        );
        let mut partword: *mut *mut *mut libc::c_int =
            fresh8.as_mut_ptr() as *mut *mut *mut libc::c_int;
        j = 0 as libc::c_int as libc::c_long;
        while j < ch as libc::c_long {
            let ref mut fresh9 = *partword.offset(j as isize);
            *fresh9 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb as *mut crate::codec_h::vorbis_block,
                (partwords as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
                    as libc::c_long,
            ) as *mut *mut libc::c_int;
            j += 1
        }
        s = 0 as libc::c_int as libc::c_long;
        's_55: while s < (*look).stages as libc::c_long {
            /* each loop decodes on partition codeword containing
            partitions_per_word partitions */
            i = 0 as libc::c_int as libc::c_long;
            l = 0 as libc::c_int as libc::c_long;
            while i < partvals as libc::c_long {
                if s == 0 as libc::c_int as libc::c_long {
                    /* fetch the partition word for each channel */
                    j = 0 as libc::c_int as libc::c_long;
                    while j < ch as libc::c_long {
                        let mut temp: libc::c_int =
                            crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                                (*look).phrasebook
                                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                            ) as libc::c_int;
                        if temp == -(1 as libc::c_int) || temp >= (*info).partvals {
                            break 's_55;
                        }
                        let ref mut fresh10 = *(*partword.offset(j as isize)).offset(l as isize);
                        *fresh10 = *(*look).decodemap.offset(temp as isize);
                        if (*(*partword.offset(j as isize)).offset(l as isize)).is_null() {
                            break 's_55;
                        }
                        j += 1
                    }
                }
                /* now we decode residual values for the partitions */
                k = 0 as libc::c_int as libc::c_long;
                while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                    j = 0 as libc::c_int as libc::c_long;
                    while j < ch as libc::c_long {
                        let mut offset: libc::c_long =
                            (*info).begin + i * samples_per_partition as libc::c_long;
                        if (*info).secondstages[*(*(*partword.offset(j as isize))
                            .offset(l as isize))
                        .offset(k as isize)
                            as usize]
                            & (1 as libc::c_int) << s
                            != 0
                        {
                            let mut stagebook: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook =
                                    *(*(*look).partbooks.offset(*(*(*partword.offset(j
                                                                                         as
                                                                                         isize)).offset(l
                                                                                                            as
                                                                                                            isize)).offset(k
                                                                                                                               as
                                                                                                                               isize)
                                                                    as
                                                                    isize)).offset(s
                                                                                       as
                                                                                       isize);
                            if !stagebook.is_null() {
                                if decodepart.expect("non-null function pointer")(
                                    stagebook,
                                    (*in_0.offset(j as isize)).offset(offset as isize),
                                    &mut (*vb).opb,
                                    samples_per_partition,
                                ) == -(1 as libc::c_int) as libc::c_long
                                {
                                    break 's_55;
                                }
                            }
                        }
                        j += 1
                    }
                    k += 1;
                    i += 1
                }
                l += 1
            }
            s += 1
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn res0_inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_float,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ch {
        if *nonzero.offset(i as isize) != 0 {
            let fresh11 = used;
            used = used + 1;
            let ref mut fresh12 = *in_0.offset(fresh11 as isize);
            *fresh12 = *in_0.offset(i as isize)
        }
        i += 1
    }
    if used != 0 {
        return _01inverse(
            vb,
            vl,
            in_0,
            used,
            Some(
                crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodevs_add
                    as unsafe extern "C" fn(
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                        _: *mut libc::c_float,
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: libc::c_int,
                    ) -> libc::c_long,
            ),
        );
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res1_forward(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut _vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
    mut partword: *mut *mut libc::c_long,
    mut _submap: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ch {
        if *nonzero.offset(i as isize) != 0 {
            let fresh13 = used;
            used = used + 1;
            let ref mut fresh14 = *in_0.offset(fresh13 as isize);
            *fresh14 = *in_0.offset(i as isize)
        }
        i += 1
    }
    if used != 0 {
        return _01forward(
            opb,
            vl,
            in_0,
            used,
            partword,
            Some(
                _encodepart
                    as unsafe extern "C" fn(
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: *mut libc::c_int,
                        _: libc::c_int,
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    ) -> libc::c_int,
            ),
        );
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res1_class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ch {
        if *nonzero.offset(i as isize) != 0 {
            let fresh15 = used;
            used = used + 1;
            let ref mut fresh16 = *in_0.offset(fresh15 as isize);
            *fresh16 = *in_0.offset(i as isize)
        }
        i += 1
    }
    if used != 0 {
        return _01class(vb, vl, in_0, used);
    } else {
        return 0 as *mut *mut libc::c_long;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res1_inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_float,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ch {
        if *nonzero.offset(i as isize) != 0 {
            let fresh17 = used;
            used = used + 1;
            let ref mut fresh18 = *in_0.offset(fresh17 as isize);
            *fresh18 = *in_0.offset(i as isize)
        }
        i += 1
    }
    if used != 0 {
        return _01inverse(
            vb,
            vl,
            in_0,
            used,
            Some(
                crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodev_add
                    as unsafe extern "C" fn(
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                        _: *mut libc::c_float,
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: libc::c_int,
                    ) -> libc::c_long,
            ),
        );
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res2_class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_int = 0;
    let mut used: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < ch {
        if *nonzero.offset(i as isize) != 0 {
            used += 1
        }
        i += 1
    }
    if used != 0 {
        return _2class(vb, vl, in_0, ch);
    } else {
        return 0 as *mut *mut libc::c_long;
    };
}
/* res2 is slightly more different; all the channels are interleaved
into a single vector and encoded. */
#[no_mangle]

pub unsafe extern "C" fn res2_forward(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_int,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
    mut partword: *mut *mut libc::c_long,
    mut _submap: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut n: libc::c_long = ((*vb).pcmend / 2 as libc::c_int) as libc::c_long;
    let mut used: libc::c_long = 0 as libc::c_int as libc::c_long;
    /* don't duplicate the code; use a working vector hack for now and
    reshape ourselves into a single channel res1 */
    /* ugly; reallocs for each coupling pass :-( */
    let mut work: *mut libc::c_int = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut crate::codec_h::vorbis_block,
        ((ch as libc::c_long * n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_int;
    i = 0 as libc::c_int as libc::c_long;
    while i < ch as libc::c_long {
        let mut pcm: *mut libc::c_int = *in_0.offset(i as isize);
        if *nonzero.offset(i as isize) != 0 {
            used += 1
        }
        j = 0 as libc::c_int as libc::c_long;
        k = i;
        while j < n {
            *work.offset(k as isize) = *pcm.offset(j as isize);
            j += 1;
            k += ch as libc::c_long
        }
        i += 1
    }
    if used != 0 {
        return _01forward(
            opb,
            vl,
            &mut work,
            1 as libc::c_int,
            partword,
            Some(
                _encodepart
                    as unsafe extern "C" fn(
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: *mut libc::c_int,
                        _: libc::c_int,
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    ) -> libc::c_int,
            ),
        );
    } else {
        return 0 as libc::c_int;
    };
}
/* duplicate code here as speed is somewhat more important */
#[no_mangle]

pub unsafe extern "C" fn res2_inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut libc::c_float,
    mut nonzero: *mut libc::c_int,
    mut ch: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: libc::c_int = (*info).grouping; /* no nonzero vectors */
    let mut partitions_per_word: libc::c_int = (*(*look).phrasebook).dim as libc::c_int;
    let mut max: libc::c_int = (*vb).pcmend * ch >> 1 as libc::c_int;
    let mut end: libc::c_int = if (*info).end < max as libc::c_long {
        (*info).end
    } else {
        max as libc::c_long
    } as libc::c_int;
    let mut n: libc::c_int = (end as libc::c_long - (*info).begin) as libc::c_int;
    if n > 0 as libc::c_int {
        let mut partvals: libc::c_int = n / samples_per_partition;
        let mut partwords: libc::c_int =
            (partvals + partitions_per_word - 1 as libc::c_int) / partitions_per_word;
        let mut partword: *mut *mut libc::c_int =
            crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb as *mut crate::codec_h::vorbis_block,
                (partwords as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
                    as libc::c_long,
            ) as *mut *mut libc::c_int;
        i = 0 as libc::c_int as libc::c_long;
        while i < ch as libc::c_long {
            if *nonzero.offset(i as isize) != 0 {
                break;
            }
            i += 1
        }
        if i == ch as libc::c_long {
            return 0 as libc::c_int;
        }
        s = 0 as libc::c_int as libc::c_long;
        's_65: while s < (*look).stages as libc::c_long {
            i = 0 as libc::c_int as libc::c_long;
            l = 0 as libc::c_int as libc::c_long;
            while i < partvals as libc::c_long {
                if s == 0 as libc::c_int as libc::c_long {
                    /* fetch the partition word */
                    let mut temp: libc::c_int =
                        crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                            (*look).phrasebook
                                as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                            &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                        ) as libc::c_int;
                    if temp == -(1 as libc::c_int) || temp >= (*info).partvals {
                        break 's_65;
                    }
                    let ref mut fresh19 = *partword.offset(l as isize);
                    *fresh19 = *(*look).decodemap.offset(temp as isize);
                    if (*partword.offset(l as isize)).is_null() {
                        break 's_65;
                    }
                }
                /* now we decode residual values for the partitions */
                k = 0 as libc::c_int as libc::c_long;
                while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                    if (*info).secondstages
                        [*(*partword.offset(l as isize)).offset(k as isize) as usize]
                        & (1 as libc::c_int) << s
                        != 0
                    {
                        let mut stagebook: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook =
                                *(*(*look).partbooks.offset(*(*partword.offset(l
                                                                                   as
                                                                                   isize)).offset(k
                                                                                                      as
                                                                                                      isize)
                                                                as
                                                                isize)).offset(s
                                                                                   as
                                                                                   isize);
                        if !stagebook.is_null() {
                            if crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decodevv_add(
                                stagebook
                                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                                in_0,
                                i * samples_per_partition as libc::c_long + (*info).begin,
                                ch,
                                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                                samples_per_partition,
                            ) == -(1 as libc::c_int) as libc::c_long
                            {
                                break 's_65;
                            }
                        }
                    }
                    k += 1;
                    i += 1
                }
                l += 1
            }
            s += 1
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub static mut residue0_exportbundle: crate::backends_h::vorbis_func_residue = {
    let mut init = crate::backends_h::vorbis_func_residue {
        pack: None,
        unpack: Some(
            res0_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            res0_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(res0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(res0_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        class: None,
        forward: None,
        inverse: Some(
            res0_inverse
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_float,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
    };
    init
};
#[no_mangle]

pub static mut residue1_exportbundle: crate::backends_h::vorbis_func_residue = {
    let mut init = crate::backends_h::vorbis_func_residue {
        pack: Some(
            res0_pack
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> (),
        ),
        unpack: Some(
            res0_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            res0_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(res0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(res0_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        class: Some(
            res1_class
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_int,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                ) -> *mut *mut libc::c_long,
        ),
        forward: Some(
            res1_forward
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_int,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                    _: *mut *mut libc::c_long,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        inverse: Some(
            res1_inverse
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_float,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
    };
    init
};
#[no_mangle]

pub static mut residue2_exportbundle: crate::backends_h::vorbis_func_residue = {
    let mut init = crate::backends_h::vorbis_func_residue {
        pack: Some(
            res0_pack
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> (),
        ),
        unpack: Some(
            res0_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            res0_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(res0_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(res0_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        class: Some(
            res2_class
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_int,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                ) -> *mut *mut libc::c_long,
        ),
        forward: Some(
            res2_forward
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_int,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                    _: *mut *mut libc::c_long,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        inverse: Some(
            res2_inverse
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut libc::c_float,
                    _: *mut libc::c_int,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
    };
    init
};

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
    pub parts: i32,
    pub stages: i32,
    pub fullbooks: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub phrasebook: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub partbooks: *mut *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    pub partvals: i32,
    pub decodemap: *mut *mut i32,
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
            0 as i32,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_residue0>() as libc::c_ulong,
        );
        ::libc::free(info as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn res0_free_look(mut i: *mut libc::c_void) {
    let mut j: i32 = 0;
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
        j = 0 as i32; /* residue vectors to group and
                      code with a partitioned book */
        while j < (*look).parts {
            if !(*(*look).partbooks.offset(j as isize)).is_null() {
                ::libc::free(*(*look).partbooks.offset(j as isize) as *mut libc::c_void);
                /* possible partition choices */
            } /* group huffman book */
            j += 1
        }
        ::libc::free((*look).partbooks as *mut libc::c_void);
        j = 0 as i32;
        while j < (*look).partvals {
            ::libc::free(*(*look).decodemap.offset(j as isize) as *mut libc::c_void);
            j += 1
        }
        ::libc::free((*look).decodemap as *mut libc::c_void);
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<vorbis_look_residue0>() as libc::c_ulong,
        );
        ::libc::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn icount(mut v: u32) -> i32 {
    let mut ret: i32 = 0 as i32;
    while v != 0 {
        ret = (ret as u32).wrapping_add(v & 1 as i32 as u32) as i32 as i32;
        v >>= 1 as i32
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
    let mut j: i32 = 0;
    let mut acc: i32 = 0 as i32;
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).begin as libc::c_ulong,
        24 as i32,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).end as libc::c_ulong,
        24 as i32,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*info).grouping - 1 as i32) as libc::c_ulong,
        24 as i32,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*info).partitions - 1 as i32) as libc::c_ulong,
        6 as i32,
    );
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).groupbook as libc::c_ulong,
        8 as i32,
    );
    /* secondstages is a bitmask; as encoding progresses pass by pass, a
    bitmask of one indicates this partition class has bits to write
    this pass */
    j = 0 as i32; /* trailing zero */
    while j < (*info).partitions {
        if crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            (*info).secondstages[j as usize] as crate::config_types_h::ogg_uint32_t,
        ) > 3 as i32
        {
            /* yes, this is a minor hack due to not thinking ahead */
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).secondstages[j as usize] as libc::c_ulong,
                3 as i32,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                1 as i32 as libc::c_ulong,
                1 as i32,
            );
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                ((*info).secondstages[j as usize] >> 3 as i32) as libc::c_ulong,
                5 as i32,
            );
        } else {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).secondstages[j as usize] as libc::c_ulong,
                4 as i32,
            );
        }
        acc += icount((*info).secondstages[j as usize] as u32);
        j += 1
    }
    j = 0 as i32;
    while j < acc {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (*info).booklist[j as usize] as libc::c_ulong,
            8 as i32,
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
    let mut j: i32 = 0;
    let mut acc: i32 = 0 as i32;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = crate::stdlib::calloc(
        1 as i32 as libc::c_ulong,
        ::std::mem::size_of::<crate::backends_h::vorbis_info_residue0>() as libc::c_ulong,
    )
        as *mut crate::backends_h::vorbis_info_residue0;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    (*info).begin = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as i32,
    );
    (*info).end = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as i32,
    );
    (*info).grouping = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        24 as i32,
    ) + 1 as i32 as libc::c_long) as i32;
    (*info).partitions = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        6 as i32,
    ) + 1 as i32 as libc::c_long) as i32;
    (*info).groupbook = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        8 as i32,
    ) as i32;
    /* check for premature EOP */
    if !((*info).groupbook < 0 as i32) {
        j = 0 as i32;
        loop {
            if !(j < (*info).partitions) {
                current_block = 5689001924483802034;
                break;
            }
            let mut cascade: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                opb as *mut crate::ogg_h::oggpack_buffer,
                3 as i32,
            ) as i32;
            let mut cflag: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                opb as *mut crate::ogg_h::oggpack_buffer,
                1 as i32,
            ) as i32;
            if cflag < 0 as i32 {
                current_block = 3462665044408642796;
                break;
            }
            if cflag != 0 {
                let mut c: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                    opb as *mut crate::ogg_h::oggpack_buffer,
                    5 as i32,
                ) as i32;
                if c < 0 as i32 {
                    current_block = 3462665044408642796;
                    break;
                }
                cascade |= c << 3 as i32
            }
            (*info).secondstages[j as usize] = cascade;
            acc += icount(cascade as u32);
            j += 1
        }
        match current_block {
            3462665044408642796 => {}
            _ => {
                j = 0 as i32;
                loop {
                    if !(j < acc) {
                        current_block = 5634871135123216486;
                        break;
                    }
                    let mut book: i32 = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        8 as i32,
                    ) as i32;
                    if book < 0 as i32 {
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
                            j = 0 as i32;
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
                                    == 0 as i32
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
                                    let mut entries: i32 =
                                        (*(*ci).book_param[(*info).groupbook as usize]).entries
                                            as i32;
                                    let mut dim: i32 =
                                        (*(*ci).book_param[(*info).groupbook as usize]).dim as i32;
                                    let mut partvals: i32 = 1 as i32;
                                    if !(dim < 1 as i32) {
                                        loop {
                                            if !(dim > 0 as i32) {
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
        1 as i32 as libc::c_ulong,
        ::std::mem::size_of::<vorbis_look_residue0>() as libc::c_ulong,
    ) as *mut vorbis_look_residue0;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut acc: i32 = 0 as i32;
    let mut dim: i32 = 0;
    let mut maxstage: i32 = 0 as i32;
    (*look).info = info;
    (*look).parts = (*info).partitions;
    (*look).fullbooks = (*ci).fullbooks;
    (*look).phrasebook = (*ci).fullbooks.offset((*info).groupbook as isize);
    dim = (*(*look).phrasebook).dim as i32;
    (*look).partbooks = crate::stdlib::calloc(
        (*look).parts as libc::c_ulong,
        ::std::mem::size_of::<*mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook>()
            as libc::c_ulong,
    )
        as *mut *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook;
    j = 0 as i32;
    while j < (*look).parts {
        let mut stages: i32 = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
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
            k = 0 as i32;
            while k < stages {
                if (*info).secondstages[j as usize] & (1 as i32) << k != 0 {
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
    (*look).partvals = 1 as i32;
    j = 0 as i32;
    while j < dim {
        (*look).partvals *= (*look).parts;
        j += 1
    }
    (*look).stages = maxstage;
    (*look).decodemap = crate::stdlib::malloc(
        ((*look).partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut i32>() as libc::c_ulong),
    ) as *mut *mut i32;
    j = 0 as i32;
    while j < (*look).partvals {
        let mut val: libc::c_long = j as libc::c_long;
        let mut mult: libc::c_long = ((*look).partvals / (*look).parts) as libc::c_long;
        let ref mut fresh3 = *(*look).decodemap.offset(j as isize);
        *fresh3 = crate::stdlib::malloc(
            (dim as libc::c_ulong).wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong),
        ) as *mut i32;
        k = 0 as i32;
        while k < dim {
            let mut deco: libc::c_long = val / mult;
            val -= deco * mult;
            mult /= (*look).parts as libc::c_long;
            *(*(*look).decodemap.offset(j as isize)).offset(k as isize) = deco as i32;
            k += 1
        }
        j += 1
    }
    return look as *mut libc::c_void;
}
/* break an abstraction and copy some code for performance purposes */

unsafe extern "C" fn local_book_besterror(
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
    mut a: *mut i32,
) -> i32 {
    let mut dim: i32 = (*book).dim as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut o: i32 = 0;
    let mut minval: i32 = (*book).minval;
    let mut del: i32 = (*book).delta;
    let mut qv: i32 = (*book).quantvals;
    let mut ze: i32 = qv >> 1 as i32;
    let mut index: i32 = 0 as i32;
    /* assumes integer/centered encoder codebook maptype 1 no more than dim 8 */
    let mut p: [i32; 8] = [
        0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32,
    ];
    if del != 1 as i32 {
        i = 0 as i32;
        o = dim;
        while i < dim {
            o -= 1;
            let mut v: i32 = (*a.offset(o as isize) - minval + (del >> 1 as i32)) / del;
            let mut m: i32 = if v < ze {
                (ze - v << 1 as i32) - 1 as i32
            } else {
                (v - ze) << 1 as i32
            };
            index = index * qv
                + (if m < 0 as i32 {
                    0 as i32
                } else {
                    (if m >= qv { (qv) - 1 as i32 } else { m })
                });
            p[o as usize] = v * del + minval;
            i += 1
        }
    } else {
        i = 0 as i32;
        o = dim;
        while i < dim {
            o -= 1;
            let mut v_0: i32 = *a.offset(o as isize) - minval;
            let mut m_0: i32 = if v_0 < ze {
                (ze - v_0 << 1 as i32) - 1 as i32
            } else {
                (v_0 - ze) << 1 as i32
            };
            index = index * qv
                + (if m_0 < 0 as i32 {
                    0 as i32
                } else {
                    (if m_0 >= qv { (qv) - 1 as i32 } else { m_0 })
                });
            p[o as usize] = v_0 * del + minval;
            i += 1
        }
    }
    if *(*(*book).c).lengthlist.offset(index as isize) as i32 <= 0 as i32 {
        let mut c: *const crate::src::libvorbis_1_3_6::lib::codebook::static_codebook = (*book).c;
        let mut best: i32 = -(1 as i32);
        /* assumes integer/centered encoder codebook maptype 1 no more than dim 8 */
        let mut e: [i32; 8] = [
            0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32,
        ];
        let mut maxval: i32 = (*book).minval + (*book).delta * ((*book).quantvals - 1 as i32);
        i = 0 as i32;
        while (i as libc::c_long) < (*book).entries {
            if *(*c).lengthlist.offset(i as isize) as i32 > 0 as i32 {
                let mut this: i32 = 0 as i32;
                j = 0 as i32;
                while j < dim {
                    let mut val: i32 = e[j as usize] - *a.offset(j as isize);
                    this += val * val;
                    j += 1
                }
                if best == -(1 as i32) || this < best {
                    crate::stdlib::memcpy(
                        p.as_mut_ptr() as *mut libc::c_void,
                        e.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[i32; 8]>() as libc::c_ulong,
                    );
                    best = this;
                    index = i
                }
            }
            /* assumes the value patterning created by the tools in vq/ */
            j = 0 as i32;
            while e[j as usize] >= maxval {
                let fresh4 = j;
                j = j + 1;
                e[fresh4 as usize] = 0 as i32
            }
            if e[j as usize] >= 0 as i32 {
                e[j as usize] += (*book).delta
            }
            e[j as usize] = -e[j as usize];
            i += 1
        }
    }
    if index > -(1 as i32) {
        i = 0 as i32;
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
    mut vec: *mut i32,
    mut n: i32,
    mut book: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
) -> i32 {
    let mut i: i32 = 0;
    let mut bits: i32 = 0 as i32;
    let mut dim: i32 = (*book).dim as i32;
    let mut step: i32 = n / dim;
    i = 0 as i32;
    while i < step {
        let mut entry: i32 = local_book_besterror(book, vec.offset((i * dim) as isize));
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
    mut in_0: *mut *mut i32,
    mut ch: i32,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: i32 = (*info).grouping;
    let mut possible_partitions: i32 = (*info).partitions;
    let mut n: i32 = ((*info).end - (*info).begin) as i32;
    let mut partvals: i32 = n / samples_per_partition;
    let mut partword: *mut *mut libc::c_long =
        crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            (ch as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong)
                as libc::c_long,
        ) as *mut *mut libc::c_long;
    let mut scale: f32 = (100.0f64 / samples_per_partition as f64) as f32;
    /* we find the partition type for each partition of each
    channel.  We'll go back and do the interleaved encoding in a
    bit.  For now, clarity */
    i = 0 as i32 as libc::c_long;
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
            0 as i32,
            ((n / samples_per_partition) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        );
        i += 1
    }
    i = 0 as i32 as libc::c_long;
    while i < partvals as libc::c_long {
        let mut offset: i32 = (i * samples_per_partition as libc::c_long + (*info).begin) as i32;
        j = 0 as i32 as libc::c_long;
        while j < ch as libc::c_long {
            let mut max: i32 = 0 as i32;
            let mut ent: i32 = 0 as i32;
            k = 0 as i32 as libc::c_long;
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
            ent = (ent as f32 * scale) as i32;
            k = 0 as i32 as libc::c_long;
            while k < (possible_partitions - 1 as i32) as libc::c_long {
                if max <= (*info).classmetric1[k as usize]
                    && ((*info).classmetric2[k as usize] < 0 as i32
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
    mut in_0: *mut *mut i32,
    mut ch: i32,
) -> *mut *mut libc::c_long {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: i32 = (*info).grouping;
    let mut possible_partitions: i32 = (*info).partitions;
    let mut n: i32 = ((*info).end - (*info).begin) as i32;
    let mut partvals: i32 = n / samples_per_partition;
    let mut partword: *mut *mut libc::c_long =
        crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            ::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong as libc::c_long,
        ) as *mut *mut libc::c_long;
    let ref mut fresh7 = *partword.offset(0 as i32 as isize);
    *fresh7 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut crate::codec_h::vorbis_block,
        (partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
            as libc::c_long,
    ) as *mut libc::c_long;
    crate::stdlib::memset(
        *partword.offset(0 as i32 as isize) as *mut libc::c_void,
        0 as i32,
        (partvals as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
    );
    i = 0 as i32 as libc::c_long;
    l = (*info).begin / ch as libc::c_long;
    while i < partvals as libc::c_long {
        let mut magmax: i32 = 0 as i32;
        let mut angmax: i32 = 0 as i32;
        j = 0 as i32 as libc::c_long;
        while j < samples_per_partition as libc::c_long {
            if ::libc::abs(*(*in_0.offset(0 as i32 as isize)).offset(l as isize)) > magmax {
                magmax = ::libc::abs(*(*in_0.offset(0 as i32 as isize)).offset(l as isize))
            }
            k = 1 as i32 as libc::c_long;
            while k < ch as libc::c_long {
                if ::libc::abs(*(*in_0.offset(k as isize)).offset(l as isize)) > angmax {
                    angmax = ::libc::abs(*(*in_0.offset(k as isize)).offset(l as isize))
                }
                k += 1
            }
            l += 1;
            j += ch as libc::c_long
        }
        j = 0 as i32 as libc::c_long;
        while j < (possible_partitions - 1 as i32) as libc::c_long {
            if magmax <= (*info).classmetric1[j as usize]
                && angmax <= (*info).classmetric2[j as usize]
            {
                break;
            }
            j += 1
        }
        *(*partword.offset(0 as i32 as isize)).offset(i as isize) = j;
        i += 1
    }
    (*look).frames += 1;
    return partword;
}

unsafe extern "C" fn _01forward(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut i32,
    mut ch: i32,
    mut partword: *mut *mut libc::c_long,
    mut encode: Option<
        unsafe extern "C" fn(
            _: *mut crate::ogg_h::oggpack_buffer,
            _: *mut i32,
            _: i32,
            _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
        ) -> i32,
    >,
) -> i32 {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: i32 = (*info).grouping;
    let mut possible_partitions: i32 = (*info).partitions;
    let mut partitions_per_word: i32 = (*(*look).phrasebook).dim as i32;
    let mut n: i32 = ((*info).end - (*info).begin) as i32;
    let mut partvals: i32 = n / samples_per_partition;
    let mut resbits: [libc::c_long; 128] = [0; 128];
    let mut resvals: [libc::c_long; 128] = [0; 128];
    crate::stdlib::memset(
        resbits.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[libc::c_long; 128]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        resvals.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<[libc::c_long; 128]>() as libc::c_ulong,
    );
    /* we code the partition words for each channel, then the residual
    words for a partition per channel until we've written all the
    residual words for that partition word.  Then write the next
    partition channel words... */
    s = 0 as i32 as libc::c_long;
    while s < (*look).stages as libc::c_long {
        i = 0 as i32 as libc::c_long;
        while i < partvals as libc::c_long {
            /* first we encode a partition codeword for each channel */
            if s == 0 as i32 as libc::c_long {
                j = 0 as i32 as libc::c_long;
                while j < ch as libc::c_long {
                    let mut val: libc::c_long = *(*partword.offset(j as isize)).offset(i as isize);
                    k = 1 as i32 as libc::c_long;
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
                                val as i32,
                                opb as *mut crate::ogg_h::oggpack_buffer,
                            ) as libc::c_long
                    }
                    j += 1
                }
            }
            /* training hack */
            /* now we encode interleaved residual values for the partitions */
            k = 0 as i32 as libc::c_long;
            while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                let mut offset: libc::c_long =
                    i * samples_per_partition as libc::c_long + (*info).begin;
                j = 0 as i32 as libc::c_long;
                while j < ch as libc::c_long {
                    if s == 0 as i32 as libc::c_long {
                        resvals[*(*partword.offset(j as isize)).offset(i as isize) as usize] +=
                            samples_per_partition as libc::c_long
                    }
                    if (*info).secondstages
                        [*(*partword.offset(j as isize)).offset(i as isize) as usize]
                        & (1 as i32) << s
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
                            let mut ret: i32 = 0;
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
    return 0 as i32;
}
/* a truncated packet here just means 'stop working'; it's not an error */

unsafe extern "C" fn _01inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut f32,
    mut ch: i32,
    mut decodepart: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
            _: *mut f32,
            _: *mut crate::ogg_h::oggpack_buffer,
            _: i32,
        ) -> libc::c_long,
    >,
) -> i32 {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: i32 = (*info).grouping;
    let mut partitions_per_word: i32 = (*(*look).phrasebook).dim as i32;
    let mut max: i32 = (*vb).pcmend >> 1 as i32;
    let mut end: i32 = if (*info).end < max as libc::c_long {
        (*info).end
    } else {
        max as libc::c_long
    } as i32;
    let mut n: i32 = (end as libc::c_long - (*info).begin) as i32;
    if n > 0 as i32 {
        let mut partvals: i32 = n / samples_per_partition;
        let mut partwords: i32 = (partvals + partitions_per_word - 1 as i32) / partitions_per_word;
        let mut fresh8 = ::std::vec::from_elem(
            0,
            (ch as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut *mut i32>() as libc::c_ulong)
                as usize,
        );
        let mut partword: *mut *mut *mut i32 = fresh8.as_mut_ptr() as *mut *mut *mut i32;
        j = 0 as i32 as libc::c_long;
        while j < ch as libc::c_long {
            let ref mut fresh9 = *partword.offset(j as isize);
            *fresh9 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb as *mut crate::codec_h::vorbis_block,
                (partwords as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut i32>() as libc::c_ulong)
                    as libc::c_long,
            ) as *mut *mut i32;
            j += 1
        }
        s = 0 as i32 as libc::c_long;
        's_55: while s < (*look).stages as libc::c_long {
            /* each loop decodes on partition codeword containing
            partitions_per_word partitions */
            i = 0 as i32 as libc::c_long;
            l = 0 as i32 as libc::c_long;
            while i < partvals as libc::c_long {
                if s == 0 as i32 as libc::c_long {
                    /* fetch the partition word for each channel */
                    j = 0 as i32 as libc::c_long;
                    while j < ch as libc::c_long {
                        let mut temp: i32 =
                            crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                                (*look).phrasebook
                                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                            ) as i32;
                        if temp == -(1 as i32) || temp >= (*info).partvals {
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
                k = 0 as i32 as libc::c_long;
                while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                    j = 0 as i32 as libc::c_long;
                    while j < ch as libc::c_long {
                        let mut offset: libc::c_long =
                            (*info).begin + i * samples_per_partition as libc::c_long;
                        if (*info).secondstages[*(*(*partword.offset(j as isize))
                            .offset(l as isize))
                        .offset(k as isize)
                            as usize]
                            & (1 as i32) << s
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
                                ) == -(1 as i32) as libc::c_long
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
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn res0_inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut f32,
    mut nonzero: *mut i32,
    mut ch: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut used: i32 = 0 as i32;
    i = 0 as i32;
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
                        _: *mut f32,
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: i32,
                    ) -> libc::c_long,
            ),
        );
    } else {
        return 0 as i32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res1_forward(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut _vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut i32,
    mut nonzero: *mut i32,
    mut ch: i32,
    mut partword: *mut *mut libc::c_long,
    mut _submap: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut used: i32 = 0 as i32;
    i = 0 as i32;
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
                        _: *mut i32,
                        _: i32,
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    ) -> i32,
            ),
        );
    } else {
        return 0 as i32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res1_class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut i32,
    mut nonzero: *mut i32,
    mut ch: i32,
) -> *mut *mut libc::c_long {
    let mut i: i32 = 0;
    let mut used: i32 = 0 as i32;
    i = 0 as i32;
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
    mut in_0: *mut *mut f32,
    mut nonzero: *mut i32,
    mut ch: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut used: i32 = 0 as i32;
    i = 0 as i32;
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
                        _: *mut f32,
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: i32,
                    ) -> libc::c_long,
            ),
        );
    } else {
        return 0 as i32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn res2_class(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut i32,
    mut nonzero: *mut i32,
    mut ch: i32,
) -> *mut *mut libc::c_long {
    let mut i: i32 = 0;
    let mut used: i32 = 0 as i32;
    i = 0 as i32;
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
    mut in_0: *mut *mut i32,
    mut nonzero: *mut i32,
    mut ch: i32,
    mut partword: *mut *mut libc::c_long,
    mut _submap: i32,
) -> i32 {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut n: libc::c_long = ((*vb).pcmend / 2 as i32) as libc::c_long;
    let mut used: libc::c_long = 0 as i32 as libc::c_long;
    /* don't duplicate the code; use a working vector hack for now and
    reshape ourselves into a single channel res1 */
    /* ugly; reallocs for each coupling pass :-( */
    let mut work: *mut i32 = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
        vb as *mut crate::codec_h::vorbis_block,
        ((ch as libc::c_long * n) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as libc::c_long,
    ) as *mut i32;
    i = 0 as i32 as libc::c_long;
    while i < ch as libc::c_long {
        let mut pcm: *mut i32 = *in_0.offset(i as isize);
        if *nonzero.offset(i as isize) != 0 {
            used += 1
        }
        j = 0 as i32 as libc::c_long;
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
            1 as i32,
            partword,
            Some(
                _encodepart
                    as unsafe extern "C" fn(
                        _: *mut crate::ogg_h::oggpack_buffer,
                        _: *mut i32,
                        _: i32,
                        _: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    ) -> i32,
            ),
        );
    } else {
        return 0 as i32;
    };
}
/* duplicate code here as speed is somewhat more important */
#[no_mangle]

pub unsafe extern "C" fn res2_inverse(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut vl: *mut libc::c_void,
    mut in_0: *mut *mut f32,
    mut nonzero: *mut i32,
    mut ch: i32,
) -> i32 {
    let mut i: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut s: libc::c_long = 0;
    let mut look: *mut vorbis_look_residue0 = vl as *mut vorbis_look_residue0;
    let mut info: *mut crate::backends_h::vorbis_info_residue0 = (*look).info;
    /* move all this setup out later */
    let mut samples_per_partition: i32 = (*info).grouping; /* no nonzero vectors */
    let mut partitions_per_word: i32 = (*(*look).phrasebook).dim as i32;
    let mut max: i32 = (*vb).pcmend * ch >> 1 as i32;
    let mut end: i32 = if (*info).end < max as libc::c_long {
        (*info).end
    } else {
        max as libc::c_long
    } as i32;
    let mut n: i32 = (end as libc::c_long - (*info).begin) as i32;
    if n > 0 as i32 {
        let mut partvals: i32 = n / samples_per_partition;
        let mut partwords: i32 = (partvals + partitions_per_word - 1 as i32) / partitions_per_word;
        let mut partword: *mut *mut i32 =
            crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb as *mut crate::codec_h::vorbis_block,
                (partwords as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<*mut i32>() as libc::c_ulong)
                    as libc::c_long,
            ) as *mut *mut i32;
        i = 0 as i32 as libc::c_long;
        while i < ch as libc::c_long {
            if *nonzero.offset(i as isize) != 0 {
                break;
            }
            i += 1
        }
        if i == ch as libc::c_long {
            return 0 as i32;
        }
        s = 0 as i32 as libc::c_long;
        's_65: while s < (*look).stages as libc::c_long {
            i = 0 as i32 as libc::c_long;
            l = 0 as i32 as libc::c_long;
            while i < partvals as libc::c_long {
                if s == 0 as i32 as libc::c_long {
                    /* fetch the partition word */
                    let mut temp: i32 =
                        crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                            (*look).phrasebook
                                as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                            &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                        ) as i32;
                    if temp == -(1 as i32) || temp >= (*info).partvals {
                        break 's_65;
                    }
                    let ref mut fresh19 = *partword.offset(l as isize);
                    *fresh19 = *(*look).decodemap.offset(temp as isize);
                    if (*partword.offset(l as isize)).is_null() {
                        break 's_65;
                    }
                }
                /* now we decode residual values for the partitions */
                k = 0 as i32 as libc::c_long;
                while k < partitions_per_word as libc::c_long && i < partvals as libc::c_long {
                    if (*info).secondstages
                        [*(*partword.offset(l as isize)).offset(k as isize) as usize]
                        & (1 as i32) << s
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
                            ) == -(1 as i32) as libc::c_long
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
    return 0 as i32;
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
                    _: *mut *mut f32,
                    _: *mut i32,
                    _: i32,
                ) -> i32,
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
                    _: *mut *mut i32,
                    _: *mut i32,
                    _: i32,
                ) -> *mut *mut libc::c_long,
        ),
        forward: Some(
            res1_forward
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut i32,
                    _: *mut i32,
                    _: i32,
                    _: *mut *mut libc::c_long,
                    _: i32,
                ) -> i32,
        ),
        inverse: Some(
            res1_inverse
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut f32,
                    _: *mut i32,
                    _: i32,
                ) -> i32,
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
                    _: *mut *mut i32,
                    _: *mut i32,
                    _: i32,
                ) -> *mut *mut libc::c_long,
        ),
        forward: Some(
            res2_forward
                as unsafe extern "C" fn(
                    _: *mut crate::ogg_h::oggpack_buffer,
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut i32,
                    _: *mut i32,
                    _: i32,
                    _: *mut *mut libc::c_long,
                    _: i32,
                ) -> i32,
        ),
        inverse: Some(
            res2_inverse
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                    _: *mut *mut f32,
                    _: *mut i32,
                    _: i32,
                ) -> i32,
        ),
    };
    init
};

use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::calloc;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::uint32_t;
pub use ::libc::abs;
pub use ::libc::exit;
pub use ::libc::free;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_info_floor1;
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
pub use crate::codec_internal_h::vorbis_look_floor1;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;

pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsfit_acc {
    pub x0: libc::c_int,
    pub x1: libc::c_int,
    pub xa: libc::c_int,
    pub ya: libc::c_int,
    pub x2a: libc::c_int,
    pub y2a: libc::c_int,
    pub xya: libc::c_int,
    pub an: libc::c_int,
    pub xb: libc::c_int,
    pub yb: libc::c_int,
    pub x2b: libc::c_int,
    pub y2b: libc::c_int,
    pub xyb: libc::c_int,
    pub bn: libc::c_int,
}
/* **********************************************/

unsafe extern "C" fn floor1_free_info(mut i: *mut libc::c_void) {
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        i as *mut crate::backends_h::vorbis_info_floor1;
    if !info.is_null() {
        crate::stdlib::memset(
            info as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::backends_h::vorbis_info_floor1>() as libc::c_ulong,
        );
        ::libc::free(info as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor1_free_look(mut i: *mut libc::c_void) {
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        i as *mut crate::codec_internal_h::vorbis_look_floor1;
    if !look.is_null() {
        /*fprintf(stderr,"floor 1 bit usage %f:%f (%f total)\n",
        (float)look->phrasebits/look->frames,
        (float)look->postbits/look->frames,
        (float)(look->postbits+look->phrasebits)/look->frames);*/
        crate::stdlib::memset(
            look as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::codec_internal_h::vorbis_look_floor1>() as libc::c_ulong,
        );
        ::libc::free(look as *mut libc::c_void);
    };
}

unsafe extern "C" fn floor1_pack(
    mut i: *mut libc::c_void,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) {
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        i as *mut crate::backends_h::vorbis_info_floor1;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut rangebits: libc::c_int = 0;
    let mut maxposit: libc::c_int = (*info).postlist[1 as libc::c_int as usize];
    let mut maxclass: libc::c_int = -(1 as libc::c_int);
    /* save out partitions */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        (*info).partitions as libc::c_ulong,
        5 as libc::c_int,
    ); /* only 0 to 31 legal */
    j = 0 as libc::c_int; /* only 0 to 15 legal */
    while j < (*info).partitions {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (*info).partitionclass[j as usize] as libc::c_ulong,
            4 as libc::c_int,
        );
        if maxclass < (*info).partitionclass[j as usize] {
            maxclass = (*info).partitionclass[j as usize]
        }
        j += 1
    }
    /* save out partition classes */
    j = 0 as libc::c_int; /* 1 to 8 */
    while j < maxclass + 1 as libc::c_int {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            ((*info).class_dim[j as usize] - 1 as libc::c_int) as libc::c_ulong,
            3 as libc::c_int,
        ); /* 0 to 3 */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            (*info).class_subs[j as usize] as libc::c_ulong,
            2 as libc::c_int,
        );
        if (*info).class_subs[j as usize] != 0 {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).class_book[j as usize] as libc::c_ulong,
                8 as libc::c_int,
            );
        }
        k = 0 as libc::c_int;
        while k < (1 as libc::c_int) << (*info).class_subs[j as usize] {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                ((*info).class_subbook[j as usize][k as usize] + 1 as libc::c_int) as libc::c_ulong,
                8 as libc::c_int,
            );
            k += 1
        }
        j += 1
    }
    /* save out the post list */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        ((*info).mult - 1 as libc::c_int) as libc::c_ulong,
        2 as libc::c_int,
    ); /* only 1,2,3,4 legal now */
    /* maxposit cannot legally be less than 1; this is encode-side, we
    can assume our setup is OK */
    crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
        opb as *mut crate::ogg_h::oggpack_buffer,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            (maxposit - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
        ) as libc::c_ulong,
        4 as libc::c_int,
    );
    rangebits = crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
        (maxposit - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
    );
    j = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while j < (*info).partitions {
        count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
        while k < count {
            crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
                opb as *mut crate::ogg_h::oggpack_buffer,
                (*info).postlist[(k + 2 as libc::c_int) as usize] as libc::c_ulong,
                rangebits,
            );
            k += 1
        }
        j += 1
    }
}

unsafe extern "C" fn icomp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> libc::c_int {
    return **(a as *mut *mut libc::c_int) - **(b as *mut *mut libc::c_int);
}

unsafe extern "C" fn floor1_unpack(
    mut vi: *mut crate::codec_h::vorbis_info,
    mut opb: *mut crate::ogg_h::oggpack_buffer,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut maxclass: libc::c_int = -(1 as libc::c_int);
    let mut rangebits: libc::c_int = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::backends_h::vorbis_info_floor1>() as libc::c_ulong,
    )
        as *mut crate::backends_h::vorbis_info_floor1;
    /* read partitions */
    (*info).partitions = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        opb as *mut crate::ogg_h::oggpack_buffer,
        5 as libc::c_int,
    ) as libc::c_int; /* only 0 to 31 legal */
    j = 0 as libc::c_int; /* only 0 to 15 legal */
    loop {
        if !(j < (*info).partitions) {
            current_block = 13183875560443969876;
            break;
        }
        (*info).partitionclass[j as usize] = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
            opb as *mut crate::ogg_h::oggpack_buffer,
            4 as libc::c_int,
        ) as libc::c_int;
        if (*info).partitionclass[j as usize] < 0 as libc::c_int {
            current_block = 4682380797242156875;
            break;
        }
        if maxclass < (*info).partitionclass[j as usize] {
            maxclass = (*info).partitionclass[j as usize]
        }
        j += 1
    }
    match current_block {
        13183875560443969876 =>
        /* read partition classes */
        {
            j = 0 as libc::c_int; /* 1 to 8 */
            's_49: loop {
                if !(j < maxclass + 1 as libc::c_int) {
                    current_block = 18317007320854588510; /* 0,1,2,3 bits */
                    break;
                }
                (*info).class_dim[j as usize] =
                    (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        3 as libc::c_int,
                    ) + 1 as libc::c_int as libc::c_long) as libc::c_int;
                (*info).class_subs[j as usize] =
                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        2 as libc::c_int,
                    ) as libc::c_int;
                if (*info).class_subs[j as usize] < 0 as libc::c_int {
                    current_block = 4682380797242156875;
                    break;
                }
                if (*info).class_subs[j as usize] != 0 {
                    (*info).class_book[j as usize] =
                        crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            8 as libc::c_int,
                        ) as libc::c_int
                }
                if (*info).class_book[j as usize] < 0 as libc::c_int
                    || (*info).class_book[j as usize] >= (*ci).books
                {
                    current_block = 4682380797242156875;
                    break;
                }
                k = 0 as libc::c_int;
                while k < (1 as libc::c_int) << (*info).class_subs[j as usize] {
                    (*info).class_subbook[j as usize][k as usize] =
                        (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                            opb as *mut crate::ogg_h::oggpack_buffer,
                            8 as libc::c_int,
                        ) - 1 as libc::c_int as libc::c_long)
                            as libc::c_int;
                    if (*info).class_subbook[j as usize][k as usize] < -(1 as libc::c_int)
                        || (*info).class_subbook[j as usize][k as usize] >= (*ci).books
                    {
                        current_block = 4682380797242156875;
                        break 's_49;
                    }
                    k += 1
                }
                j += 1
            }
            match current_block {
                4682380797242156875 => {}
                _ => {
                    /* read the post list */
                    (*info).mult = (crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        2 as libc::c_int,
                    ) + 1 as libc::c_int as libc::c_long)
                        as libc::c_int; /* only 1,2,3,4 legal now */
                    rangebits = crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                        opb as *mut crate::ogg_h::oggpack_buffer,
                        4 as libc::c_int,
                    ) as libc::c_int;
                    if !(rangebits < 0 as libc::c_int) {
                        j = 0 as libc::c_int;
                        k = 0 as libc::c_int;
                        's_130: loop {
                            if !(j < (*info).partitions) {
                                current_block = 14434620278749266018;
                                break;
                            }
                            count += (*info).class_dim[(*info).partitionclass[j as usize] as usize];
                            if count > 63 as libc::c_int {
                                current_block = 4682380797242156875;
                                break;
                            }
                            while k < count {
                                (*info).postlist[(k + 2 as libc::c_int) as usize] =
                                    crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                                        opb as *mut crate::ogg_h::oggpack_buffer,
                                        rangebits,
                                    ) as libc::c_int;
                                let mut t: libc::c_int =
                                    (*info).postlist[(k + 2 as libc::c_int) as usize];
                                if t < 0 as libc::c_int || t >= (1 as libc::c_int) << rangebits {
                                    current_block = 4682380797242156875;
                                    break 's_130;
                                }
                                k += 1
                            }
                            j += 1
                        }
                        match current_block {
                            4682380797242156875 => {}
                            _ => {
                                (*info).postlist[0 as libc::c_int as usize] = 0 as libc::c_int;
                                (*info).postlist[1 as libc::c_int as usize] =
                                    (1 as libc::c_int) << rangebits;
                                /* don't allow repeated values in post list as they'd result in
                                zero-length segments */
                                let mut sortpointer: [*mut libc::c_int; 65] =
                                    [0 as *mut libc::c_int; 65];
                                j = 0 as libc::c_int;
                                while j < count + 2 as libc::c_int {
                                    sortpointer[j as usize] =
                                        (*info).postlist.as_mut_ptr().offset(j as isize);
                                    j += 1
                                }
                                crate::stdlib::qsort(
                                    sortpointer.as_mut_ptr() as *mut libc::c_void,
                                    (count + 2 as libc::c_int) as crate::stddef_h::size_t,
                                    ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
                                    Some(
                                        icomp
                                            as unsafe extern "C" fn(
                                                _: *const libc::c_void,
                                                _: *const libc::c_void,
                                            )
                                                -> libc::c_int,
                                    ),
                                );
                                j = 1 as libc::c_int;
                                loop {
                                    if !(j < count + 2 as libc::c_int) {
                                        current_block = 1847472278776910194;
                                        break;
                                    }
                                    if *sortpointer[(j - 1 as libc::c_int) as usize]
                                        == *sortpointer[j as usize]
                                    {
                                        current_block = 4682380797242156875;
                                        break;
                                    }
                                    j += 1
                                }
                                match current_block {
                                    4682380797242156875 => {}
                                    _ => return info as *mut libc::c_void,
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    floor1_free_info(info as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor1_look(
    mut _vd: *mut crate::codec_h::vorbis_dsp_state,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut sortpointer: [*mut libc::c_int; 65] = [0 as *mut libc::c_int; 65];
    let mut info: *mut crate::backends_h::vorbis_info_floor1 =
        in_0 as *mut crate::backends_h::vorbis_info_floor1;
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 = crate::stdlib::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::codec_internal_h::vorbis_look_floor1>() as libc::c_ulong,
    )
        as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    (*look).vi = info;
    (*look).n = (*info).postlist[1 as libc::c_int as usize];
    /* we drop each position value in-between already decoded values,
    and use linear interpolation to predict each new value past the
    edges.  The positions are read in the order of the position
    list... we precompute the bounding positions in the lookup.  Of
    course, the neighbors can change (if a position is declined), but
    this is an initial mapping */
    i = 0 as libc::c_int;
    while i < (*info).partitions {
        n += (*info).class_dim[(*info).partitionclass[i as usize] as usize];
        i += 1
    }
    n += 2 as libc::c_int;
    (*look).posts = n;
    /* also store a sorted position index */
    i = 0 as libc::c_int;
    while i < n {
        sortpointer[i as usize] = (*info).postlist.as_mut_ptr().offset(i as isize);
        i += 1
    }
    crate::stdlib::qsort(
        sortpointer.as_mut_ptr() as *mut libc::c_void,
        n as crate::stddef_h::size_t,
        ::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
        Some(
            icomp
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    /* points from sort order back to range number */
    i = 0 as libc::c_int;
    while i < n {
        (*look).forward_index[i as usize] = sortpointer[i as usize]
            .offset_from((*info).postlist.as_mut_ptr())
            as libc::c_long as libc::c_int;
        i += 1
    }
    /* points from range order to sorted position */
    i = 0 as libc::c_int;
    while i < n {
        (*look).reverse_index[(*look).forward_index[i as usize] as usize] = i;
        i += 1
    }
    /* we actually need the post values too */
    i = 0 as libc::c_int;
    while i < n {
        (*look).sorted_index[i as usize] =
            (*info).postlist[(*look).forward_index[i as usize] as usize];
        i += 1
    }
    /* quantize values to multiplier spec */
    match (*info).mult {
        1 => {
            /* 1024 -> 256 */
            (*look).quant_q = 256 as libc::c_int
        }
        2 => {
            /* 1024 -> 128 */
            (*look).quant_q = 128 as libc::c_int
        }
        3 => {
            /* 1024 -> 86 */
            (*look).quant_q = 86 as libc::c_int
        }
        4 => {
            /* 1024 -> 64 */
            (*look).quant_q = 64 as libc::c_int
        }
        _ => {}
    }
    /* discover our neighbors for decode where we don't use fit flags
    (that would push the neighbors outward) */
    i = 0 as libc::c_int; /* mask off flag */
    while i < n - 2 as libc::c_int {
        let mut lo: libc::c_int = 0 as libc::c_int;
        let mut hi: libc::c_int = 1 as libc::c_int;
        let mut lx: libc::c_int = 0 as libc::c_int;
        let mut hx: libc::c_int = (*look).n;
        let mut currentx: libc::c_int = (*info).postlist[(i + 2 as libc::c_int) as usize];
        j = 0 as libc::c_int;
        while j < i + 2 as libc::c_int {
            let mut x: libc::c_int = (*info).postlist[j as usize];
            if x > lx && x < currentx {
                lo = j;
                lx = x
            }
            if x < hx && x > currentx {
                hi = j;
                hx = x
            }
            j += 1
        }
        (*look).loneighbor[i as usize] = lo;
        (*look).hineighbor[i as usize] = hi;
        i += 1
    }
    return look as *mut libc::c_void;
}

unsafe extern "C" fn render_point(
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
    mut x: libc::c_int,
) -> libc::c_int {
    y0 &= 0x7fff as libc::c_int;
    y1 &= 0x7fff as libc::c_int;
    let mut dy: libc::c_int = y1 - y0;
    let mut adx: libc::c_int = x1 - x0;
    let mut ady: libc::c_int = ::libc::abs(dy);
    let mut err: libc::c_int = ady * (x - x0);
    let mut off: libc::c_int = err / adx;
    if dy < 0 as libc::c_int {
        return y0 - off;
    }
    return y0 + off;
}

unsafe extern "C" fn vorbis_dBquant(mut x: *const libc::c_float) -> libc::c_int {
    let mut i: libc::c_int = (*x * 7.3142857f32 + 1023.5f32) as libc::c_int;
    if i > 1023 as libc::c_int {
        return 1023 as libc::c_int;
    }
    if i < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return i;
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

unsafe extern "C" fn render_line(
    mut n: libc::c_int,
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
    mut d: *mut libc::c_float,
) {
    let mut dy: libc::c_int = y1 - y0;
    let mut adx: libc::c_int = x1 - x0;
    let mut ady: libc::c_int = ::libc::abs(dy);
    let mut base: libc::c_int = dy / adx;
    let mut sy: libc::c_int = if dy < 0 as libc::c_int {
        (base) - 1 as libc::c_int
    } else {
        (base) + 1 as libc::c_int
    };
    let mut x: libc::c_int = x0;
    let mut y: libc::c_int = y0;
    let mut err: libc::c_int = 0 as libc::c_int;
    ady -= ::libc::abs(base * adx);
    if n > x1 {
        n = x1
    }
    if x < n {
        *d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize]
    }
    loop {
        x += 1;
        if !(x < n) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        *d.offset(x as isize) *= FLOOR1_fromdB_LOOKUP[y as usize]
    }
}

unsafe extern "C" fn render_line0(
    mut n: libc::c_int,
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
    mut d: *mut libc::c_int,
) {
    let mut dy: libc::c_int = y1 - y0;
    let mut adx: libc::c_int = x1 - x0;
    let mut ady: libc::c_int = ::libc::abs(dy);
    let mut base: libc::c_int = dy / adx;
    let mut sy: libc::c_int = if dy < 0 as libc::c_int {
        (base) - 1 as libc::c_int
    } else {
        (base) + 1 as libc::c_int
    };
    let mut x: libc::c_int = x0;
    let mut y: libc::c_int = y0;
    let mut err: libc::c_int = 0 as libc::c_int;
    ady -= ::libc::abs(base * adx);
    if n > x1 {
        n = x1
    }
    if x < n {
        *d.offset(x as isize) = y
    }
    loop {
        x += 1;
        if !(x < n) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        *d.offset(x as isize) = y
    }
}
/* the floor has already been filtered to only include relevant sections */

unsafe extern "C" fn accumulate_fit(
    mut flr: *const libc::c_float,
    mut mdct: *const libc::c_float,
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut a: *mut lsfit_acc,
    mut n: libc::c_int,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut xa: libc::c_int = 0 as libc::c_int;
    let mut ya: libc::c_int = 0 as libc::c_int;
    let mut x2a: libc::c_int = 0 as libc::c_int;
    let mut y2a: libc::c_int = 0 as libc::c_int;
    let mut xya: libc::c_int = 0 as libc::c_int;
    let mut na: libc::c_int = 0 as libc::c_int;
    let mut xb: libc::c_int = 0 as libc::c_int;
    let mut yb: libc::c_int = 0 as libc::c_int;
    let mut x2b: libc::c_int = 0 as libc::c_int;
    let mut y2b: libc::c_int = 0 as libc::c_int;
    let mut xyb: libc::c_int = 0 as libc::c_int;
    let mut nb: libc::c_int = 0 as libc::c_int;
    crate::stdlib::memset(
        a as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<lsfit_acc>() as libc::c_ulong,
    );
    (*a).x0 = x0;
    (*a).x1 = x1;
    if x1 >= n {
        x1 = n - 1 as libc::c_int
    }
    i = x0 as libc::c_long;
    while i <= x1 as libc::c_long {
        let mut quantized: libc::c_int = vorbis_dBquant(flr.offset(i as isize));
        if quantized != 0 {
            if *mdct.offset(i as isize) + (*info).twofitatten >= *flr.offset(i as isize) {
                xa = (xa as libc::c_long + i) as libc::c_int;
                ya += quantized;
                x2a = (x2a as libc::c_long + i * i) as libc::c_int;
                y2a += quantized * quantized;
                xya = (xya as libc::c_long + i * quantized as libc::c_long) as libc::c_int;
                na += 1
            } else {
                xb = (xb as libc::c_long + i) as libc::c_int;
                yb += quantized;
                x2b = (x2b as libc::c_long + i * i) as libc::c_int;
                y2b += quantized * quantized;
                xyb = (xyb as libc::c_long + i * quantized as libc::c_long) as libc::c_int;
                nb += 1
            }
        }
        i += 1
    }
    (*a).xa = xa;
    (*a).ya = ya;
    (*a).x2a = x2a;
    (*a).y2a = y2a;
    (*a).xya = xya;
    (*a).an = na;
    (*a).xb = xb;
    (*a).yb = yb;
    (*a).x2b = x2b;
    (*a).y2b = y2b;
    (*a).xyb = xyb;
    (*a).bn = nb;
    return na;
}

unsafe extern "C" fn fit_line(
    mut a: *mut lsfit_acc,
    mut fits: libc::c_int,
    mut y0: *mut libc::c_int,
    mut y1: *mut libc::c_int,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> libc::c_int {
    let mut xb: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut yb: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x2b: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut _y2b: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut xyb: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut bn: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    let mut x0: libc::c_int = (*a.offset(0 as libc::c_int as isize)).x0;
    let mut x1: libc::c_int = (*a.offset((fits - 1 as libc::c_int) as isize)).x1;
    i = 0 as libc::c_int;
    while i < fits {
        let mut weight: libc::c_double = (((*a.offset(i as isize)).bn + (*a.offset(i as isize)).an)
            as libc::c_float
            * (*info).twofitweight
            / ((*a.offset(i as isize)).an + 1 as libc::c_int) as libc::c_float)
            as libc::c_double
            + 1.0f64;
        xb += (*a.offset(i as isize)).xb as libc::c_double
            + (*a.offset(i as isize)).xa as libc::c_double * weight;
        yb += (*a.offset(i as isize)).yb as libc::c_double
            + (*a.offset(i as isize)).ya as libc::c_double * weight;
        x2b += (*a.offset(i as isize)).x2b as libc::c_double
            + (*a.offset(i as isize)).x2a as libc::c_double * weight;
        _y2b += (*a.offset(i as isize)).y2b as libc::c_double
            + (*a.offset(i as isize)).y2a as libc::c_double * weight;
        xyb += (*a.offset(i as isize)).xyb as libc::c_double
            + (*a.offset(i as isize)).xya as libc::c_double * weight;
        bn += (*a.offset(i as isize)).bn as libc::c_double
            + (*a.offset(i as isize)).an as libc::c_double * weight;
        i += 1
    }
    if *y0 >= 0 as libc::c_int {
        xb += x0 as libc::c_double;
        yb += *y0 as libc::c_double;
        x2b += (x0 * x0) as libc::c_double;
        _y2b += (*y0 * *y0) as libc::c_double;
        xyb += (*y0 * x0) as libc::c_double;
        bn += 1.
    }
    if *y1 >= 0 as libc::c_int {
        xb += x1 as libc::c_double;
        yb += *y1 as libc::c_double;
        x2b += (x1 * x1) as libc::c_double;
        _y2b += (*y1 * *y1) as libc::c_double;
        xyb += (*y1 * x1) as libc::c_double;
        bn += 1.
    }
    let mut denom: libc::c_double = bn * x2b - xb * xb;
    if denom > 0.0f64 {
        let mut a_0: libc::c_double = (yb * x2b - xyb * xb) / denom;
        let mut b: libc::c_double = (bn * xyb - xb * yb) / denom;
        *y0 = crate::stdlib::rint(a_0 + b * x0 as libc::c_double) as libc::c_int;
        *y1 = crate::stdlib::rint(a_0 + b * x1 as libc::c_double) as libc::c_int;
        /* limit to our range! */
        if *y0 > 1023 as libc::c_int {
            *y0 = 1023 as libc::c_int
        } /* index by range list position */
        if *y1 > 1023 as libc::c_int {
            *y1 = 1023 as libc::c_int
        } /* index by range list position */
        if *y0 < 0 as libc::c_int {
            *y0 = 0 as libc::c_int
        } /* sorted index of range list position (+2) */
        if *y1 < 0 as libc::c_int {
            *y1 = 0 as libc::c_int
        } /* mark all unused */
        return 0 as libc::c_int;
    } else {
        *y0 = 0 as libc::c_int; /* mark all unused */
        *y1 = 0 as libc::c_int; /* 0 for the implicit 0 post */
        return 1 as libc::c_int;
    }; /* 1 for the implicit post at n */
}

unsafe extern "C" fn inspect_error(
    mut x0: libc::c_int,
    mut x1: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
    mut mask: *const libc::c_float,
    mut mdct: *const libc::c_float,
    mut info: *mut crate::backends_h::vorbis_info_floor1,
) -> libc::c_int {
    let mut dy: libc::c_int = y1 - y0; /* no neighbor yet */
    let mut adx: libc::c_int = x1 - x0;
    let mut ady: libc::c_int = ::libc::abs(dy);
    let mut base: libc::c_int = dy / adx;
    let mut sy: libc::c_int = if dy < 0 as libc::c_int {
        (base) - 1 as libc::c_int
    } else {
        (base) + 1 as libc::c_int
    };
    let mut x: libc::c_int = x0;
    let mut y: libc::c_int = y0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_int = vorbis_dBquant(mask.offset(x as isize));
    let mut mse: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    ady -= ::libc::abs(base * adx);
    mse = y - val;
    mse *= mse;
    n += 1;
    if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
        if y as libc::c_float + (*info).maxover < val as libc::c_float {
            return 1 as libc::c_int;
        }
        if y as libc::c_float - (*info).maxunder > val as libc::c_float {
            return 1 as libc::c_int;
        }
    }
    loop {
        x += 1;
        if !(x < x1) {
            break;
        }
        err = err + ady;
        if err >= adx {
            err -= adx;
            y += sy
        } else {
            y += base
        }
        val = vorbis_dBquant(mask.offset(x as isize));
        mse += (y - val) * (y - val);
        n += 1;
        if *mdct.offset(x as isize) + (*info).twofitatten >= *mask.offset(x as isize) {
            if val != 0 {
                if y as libc::c_float + (*info).maxover < val as libc::c_float {
                    return 1 as libc::c_int;
                }
                if y as libc::c_float - (*info).maxunder > val as libc::c_float {
                    return 1 as libc::c_int;
                }
            }
        }
    }
    if (*info).maxover * (*info).maxover / n as libc::c_float > (*info).maxerr {
        return 0 as libc::c_int;
    }
    if (*info).maxunder * (*info).maxunder / n as libc::c_float > (*info).maxerr {
        return 0 as libc::c_int;
    }
    if (mse / n) as libc::c_float > (*info).maxerr {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn post_Y(
    mut A: *mut libc::c_int,
    mut B: *mut libc::c_int,
    mut pos: libc::c_int,
) -> libc::c_int {
    if *A.offset(pos as isize) < 0 as libc::c_int {
        return *B.offset(pos as isize);
    }
    if *B.offset(pos as isize) < 0 as libc::c_int {
        return *A.offset(pos as isize);
    }
    return *A.offset(pos as isize) + *B.offset(pos as isize) >> 1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn floor1_fit(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut logmdct: *const libc::c_float,
    mut logmask: *const libc::c_float,
) -> *mut libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut n: libc::c_long = (*look).n as libc::c_long;
    let mut posts: libc::c_long = (*look).posts as libc::c_long;
    let mut nonzero: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut fits: [lsfit_acc; 64] = [lsfit_acc {
        x0: 0,
        x1: 0,
        xa: 0,
        ya: 0,
        x2a: 0,
        y2a: 0,
        xya: 0,
        an: 0,
        xb: 0,
        yb: 0,
        x2b: 0,
        y2b: 0,
        xyb: 0,
        bn: 0,
    }; 64];
    let mut fit_valueA: [libc::c_int; 65] = [0; 65];
    let mut fit_valueB: [libc::c_int; 65] = [0; 65];
    let mut loneighbor: [libc::c_int; 65] = [0; 65];
    let mut hineighbor: [libc::c_int; 65] = [0; 65];
    let mut output: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut memo: [libc::c_int; 65] = [0; 65];
    i = 0 as libc::c_int as libc::c_long;
    while i < posts {
        fit_valueA[i as usize] = -(200 as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < posts {
        fit_valueB[i as usize] = -(200 as libc::c_int);
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < posts {
        loneighbor[i as usize] = 0 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < posts {
        hineighbor[i as usize] = 1 as libc::c_int;
        i += 1
    }
    i = 0 as libc::c_int as libc::c_long;
    while i < posts {
        memo[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    /* quantize the relevant floor points and collect them into line fit
    structures (one per minimal division) at the same time */
    if posts == 0 as libc::c_int as libc::c_long {
        nonzero += accumulate_fit(
            logmask,
            logmdct,
            0 as libc::c_int,
            n as libc::c_int,
            fits.as_mut_ptr(),
            n as libc::c_int,
            info,
        ) as libc::c_long
    } else {
        i = 0 as libc::c_int as libc::c_long;
        while i < posts - 1 as libc::c_int as libc::c_long {
            nonzero += accumulate_fit(
                logmask,
                logmdct,
                (*look).sorted_index[i as usize],
                (*look).sorted_index[(i + 1 as libc::c_int as libc::c_long) as usize],
                fits.as_mut_ptr().offset(i as isize),
                n as libc::c_int,
                info,
            ) as libc::c_long;
            i += 1
        }
    }
    if nonzero != 0 {
        /* start by fitting the implicit base case.... */
        let mut y0: libc::c_int = -(200 as libc::c_int);
        let mut y1: libc::c_int = -(200 as libc::c_int);
        fit_line(
            fits.as_mut_ptr(),
            (posts - 1 as libc::c_int as libc::c_long) as libc::c_int,
            &mut y0,
            &mut y1,
            info,
        );
        fit_valueA[0 as libc::c_int as usize] = y0;
        fit_valueB[0 as libc::c_int as usize] = y0;
        fit_valueB[1 as libc::c_int as usize] = y1;
        fit_valueA[1 as libc::c_int as usize] = y1;
        /* Non degenerate case */
        /* start progressive splitting.  This is a greedy, non-optimal
        algorithm, but simple and close enough to the best
        answer. */
        i = 2 as libc::c_int as libc::c_long;
        while i < posts {
            let mut sortpos: libc::c_int = (*look).reverse_index[i as usize];
            let mut ln: libc::c_int = loneighbor[sortpos as usize];
            let mut hn: libc::c_int = hineighbor[sortpos as usize];
            /* eliminate repeat searches of a particular range with a memo */
            if memo[ln as usize] != hn {
                /* haven't performed this error search yet */
                let mut lsortpos: libc::c_int = (*look).reverse_index[ln as usize];
                let mut hsortpos: libc::c_int = (*look).reverse_index[hn as usize];
                memo[ln as usize] = hn;
                /* A note: we want to bound/minimize *local*, not global, error */
                let mut lx: libc::c_int = (*info).postlist[ln as usize];
                let mut hx: libc::c_int = (*info).postlist[hn as usize];
                let mut ly: libc::c_int =
                    post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), ln);
                let mut hy: libc::c_int =
                    post_Y(fit_valueA.as_mut_ptr(), fit_valueB.as_mut_ptr(), hn);
                if ly == -(1 as libc::c_int) || hy == -(1 as libc::c_int) {
                    ::libc::exit(1 as libc::c_int);
                }
                if inspect_error(lx, hx, ly, hy, logmask, logmdct, info) != 0 {
                    /* outside error bounds/begin search area.  Split it. */
                    let mut ly0: libc::c_int = -(200 as libc::c_int);
                    let mut ly1: libc::c_int = -(200 as libc::c_int);
                    let mut hy0: libc::c_int = -(200 as libc::c_int);
                    let mut hy1: libc::c_int = -(200 as libc::c_int);
                    let mut ret0: libc::c_int = fit_line(
                        fits.as_mut_ptr().offset(lsortpos as isize),
                        sortpos - lsortpos,
                        &mut ly0,
                        &mut ly1,
                        info,
                    );
                    let mut ret1: libc::c_int = fit_line(
                        fits.as_mut_ptr().offset(sortpos as isize),
                        hsortpos - sortpos,
                        &mut hy0,
                        &mut hy1,
                        info,
                    );
                    if ret0 != 0 {
                        ly0 = ly;
                        ly1 = hy0
                    }
                    if ret1 != 0 {
                        hy0 = ly1;
                        hy1 = hy
                    }
                    if ret0 != 0 && ret1 != 0 {
                        fit_valueA[i as usize] = -(200 as libc::c_int);
                        fit_valueB[i as usize] = -(200 as libc::c_int)
                    } else {
                        /* store new edge values */
                        fit_valueB[ln as usize] = ly0;
                        if ln == 0 as libc::c_int {
                            fit_valueA[ln as usize] = ly0
                        }
                        fit_valueA[i as usize] = ly1;
                        fit_valueB[i as usize] = hy0;
                        fit_valueA[hn as usize] = hy1;
                        if hn == 1 as libc::c_int {
                            fit_valueB[hn as usize] = hy1
                        }
                        if ly1 >= 0 as libc::c_int || hy0 >= 0 as libc::c_int {
                            /* store new neighbor values */
                            j = (sortpos - 1 as libc::c_int) as libc::c_long;
                            while j >= 0 as libc::c_int as libc::c_long {
                                if !(hineighbor[j as usize] == hn) {
                                    break;
                                }
                                hineighbor[j as usize] = i as libc::c_int;
                                j -= 1
                            }
                            j = (sortpos + 1 as libc::c_int) as libc::c_long;
                            while j < posts {
                                if !(loneighbor[j as usize] == ln) {
                                    break;
                                }
                                loneighbor[j as usize] = i as libc::c_int;
                                j += 1
                            }
                        }
                    }
                } else {
                    fit_valueA[i as usize] = -(200 as libc::c_int);
                    fit_valueB[i as usize] = -(200 as libc::c_int)
                }
            }
            i += 1
        }
        output = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(posts as libc::c_ulong) as libc::c_long,
        ) as *mut libc::c_int;
        *output.offset(0 as libc::c_int as isize) = post_Y(
            fit_valueA.as_mut_ptr(),
            fit_valueB.as_mut_ptr(),
            0 as libc::c_int,
        );
        *output.offset(1 as libc::c_int as isize) = post_Y(
            fit_valueA.as_mut_ptr(),
            fit_valueB.as_mut_ptr(),
            1 as libc::c_int,
        );
        /* fill in posts marked as not using a fit; we will zero
        back out to 'unused' when encoding them so long as curve
        interpolation doesn't force them into use */
        i = 2 as libc::c_int as libc::c_long;
        while i < posts {
            let mut ln_0: libc::c_int =
                (*look).loneighbor[(i - 2 as libc::c_int as libc::c_long) as usize];
            let mut hn_0: libc::c_int =
                (*look).hineighbor[(i - 2 as libc::c_int as libc::c_long) as usize];
            let mut x0: libc::c_int = (*info).postlist[ln_0 as usize];
            let mut x1: libc::c_int = (*info).postlist[hn_0 as usize];
            let mut y0_0: libc::c_int = *output.offset(ln_0 as isize);
            let mut y1_0: libc::c_int = *output.offset(hn_0 as isize);
            let mut predicted: libc::c_int =
                render_point(x0, x1, y0_0, y1_0, (*info).postlist[i as usize]);
            let mut vx: libc::c_int = post_Y(
                fit_valueA.as_mut_ptr(),
                fit_valueB.as_mut_ptr(),
                i as libc::c_int,
            );
            if vx >= 0 as libc::c_int && predicted != vx {
                *output.offset(i as isize) = vx
            } else {
                *output.offset(i as isize) = predicted | 0x8000 as libc::c_int
            }
            i += 1
        }
    }
    return output;
}
#[no_mangle]

pub unsafe extern "C" fn floor1_interpolate_fit(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut A: *mut libc::c_int,
    mut B: *mut libc::c_int,
    mut del: libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_long = 0;
    let mut posts: libc::c_long = (*look).posts as libc::c_long;
    let mut output: *mut libc::c_int = 0 as *mut libc::c_int;
    if !A.is_null() && !B.is_null() {
        output = crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
            vb as *mut crate::codec_h::vorbis_block,
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(posts as libc::c_ulong) as libc::c_long,
        ) as *mut libc::c_int;
        /* overly simpleminded--- look again post 1.2 */
        i = 0 as libc::c_int as libc::c_long;
        while i < posts {
            *output.offset(i as isize) = (65536 as libc::c_int - del)
                * (*A.offset(i as isize) & 0x7fff as libc::c_int)
                + del * (*B.offset(i as isize) & 0x7fff as libc::c_int)
                + 32768 as libc::c_int
                >> 16 as libc::c_int;
            if *A.offset(i as isize) & 0x8000 as libc::c_int != 0
                && *B.offset(i as isize) & 0x8000 as libc::c_int != 0
            {
                *output.offset(i as isize) |= 0x8000 as libc::c_int
            }
            i += 1
        }
    }
    return output;
}
/* local lookup storage */
/* envelope lookup */
/* block, type */
/* local storage, only used on the encoding side.  This way the
application does not need to worry about freeing some packets'
memory and not others'; packet storage is always tracked.
Cleared next call to a _dsp_ function */
/* codec_setup_info contains all the setup information specific to the
   specific compression/decompression mode in progress (eg,
   psychoacoustic settings, channel setup, options, codebook
   etc).
*********************************************************************/
/* Vorbis supports only short and long blocks, but allows the
encoder to choose the sizes */
/* modes are the primary means of supporting on-the-fly different
blocksizes, different channel mappings (LR or M/A),
different residue backends, etc.  Each mode consists of a
blocksize flag and a mapping (along with the mapping setup */
/* encode only */
/* encode only */
/* used only by vorbisenc.c.  It's a
highly redundant structure, but
improves clarity of program flow. */
/* painless downsample for decode */
/* in */
#[no_mangle]

pub unsafe extern "C" fn floor1_encode(
    mut opb: *mut crate::ogg_h::oggpack_buffer,
    mut vb: *mut crate::codec_h::vorbis_block,
    mut look: *mut crate::codec_internal_h::vorbis_look_floor1,
    mut post: *mut libc::c_int,
    mut ilogmask: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_long = 0;
    let mut j: libc::c_long = 0;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut posts: libc::c_long = (*look).posts as libc::c_long;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut out: [libc::c_int; 65] = [0; 65];
    let mut sbooks: *mut *mut crate::src::libvorbis_1_3_6::lib::codebook::static_codebook =
        (*ci).book_param.as_mut_ptr();
    let mut books: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci).fullbooks;
    /* quantize values to multiplier spec */
    if !post.is_null() {
        i = 0 as libc::c_int as libc::c_long;
        while i < posts {
            let mut val: libc::c_int = *post.offset(i as isize) & 0x7fff as libc::c_int;
            match (*info).mult {
                1 => {
                    /* 1024 -> 256 */
                    val >>= 2 as libc::c_int
                }
                2 => {
                    /* 1024 -> 128 */
                    val >>= 3 as libc::c_int
                }
                3 => {
                    /* 1024 -> 86 */
                    val /= 12 as libc::c_int
                }
                4 => {
                    /* 1024 -> 64 */
                    val >>= 4 as libc::c_int
                }
                _ => {}
            }
            *post.offset(i as isize) = val | *post.offset(i as isize) & 0x8000 as libc::c_int;
            i += 1
        }
        out[0 as libc::c_int as usize] = *post.offset(0 as libc::c_int as isize);
        out[1 as libc::c_int as usize] = *post.offset(1 as libc::c_int as isize);
        /* find prediction values for each post and subtract them */
        i = 2 as libc::c_int as libc::c_long; /* in case there was roundoff jitter
                                              in interpolation */
        while i < posts {
            let mut ln: libc::c_int =
                (*look).loneighbor[(i - 2 as libc::c_int as libc::c_long) as usize];
            let mut hn: libc::c_int =
                (*look).hineighbor[(i - 2 as libc::c_int as libc::c_long) as usize];
            let mut x0: libc::c_int = (*info).postlist[ln as usize];
            let mut x1: libc::c_int = (*info).postlist[hn as usize];
            let mut y0: libc::c_int = *post.offset(ln as isize);
            let mut y1: libc::c_int = *post.offset(hn as isize);
            let mut predicted: libc::c_int =
                render_point(x0, x1, y0, y1, (*info).postlist[i as usize]);
            if *post.offset(i as isize) & 0x8000 as libc::c_int != 0
                || predicted == *post.offset(i as isize)
            {
                *post.offset(i as isize) = predicted | 0x8000 as libc::c_int;
                out[i as usize] = 0 as libc::c_int
            } else {
                let mut headroom: libc::c_int = if (*look).quant_q - predicted < predicted {
                    ((*look).quant_q) - predicted
                } else {
                    predicted
                };
                let mut val_0: libc::c_int = *post.offset(i as isize) - predicted;
                /* at this point the 'deviation' value is in the range +/- max
                range, but the real, unique range can always be mapped to
                only [0-maxrange).  So we want to wrap the deviation into
                this limited range, but do it in the way that least screws
                an essentially gaussian probability distribution. */
                if val_0 < 0 as libc::c_int {
                    if val_0 < -headroom {
                        val_0 = headroom - val_0 - 1 as libc::c_int
                    } else {
                        val_0 = -(1 as libc::c_int) - (val_0 << 1 as libc::c_int)
                    }
                } else if val_0 >= headroom {
                    val_0 = val_0 + headroom
                } else {
                    val_0 <<= 1 as libc::c_int
                }
                out[i as usize] = val_0;
                *post.offset(ln as isize) &= 0x7fff as libc::c_int;
                *post.offset(hn as isize) &= 0x7fff as libc::c_int
            }
            i += 1
        }
        /* we have everything we need. pack it out */
        /* mark nontrivial floor */
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            1 as libc::c_int as libc::c_ulong,
            1 as libc::c_int,
        );
        /* beginning/end post */
        (*look).frames += 1;
        (*look).postbits += (crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*look).quant_q - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
        ) * 2 as libc::c_int) as libc::c_long;
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            out[0 as libc::c_int as usize] as libc::c_ulong,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
            ),
        );
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            out[1 as libc::c_int as usize] as libc::c_ulong,
            crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                ((*look).quant_q - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
            ),
        );
        /* partition by partition */
        i = 0 as libc::c_int as libc::c_long;
        j = 2 as libc::c_int as libc::c_long;
        while i < (*info).partitions as libc::c_long {
            let mut class: libc::c_int = (*info).partitionclass[i as usize];
            let mut cdim: libc::c_int = (*info).class_dim[class as usize];
            let mut csubbits: libc::c_int = (*info).class_subs[class as usize];
            let mut csub: libc::c_int = (1 as libc::c_int) << csubbits;
            let mut bookas: [libc::c_int; 8] = [
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ];
            let mut cval: libc::c_int = 0 as libc::c_int;
            let mut cshift: libc::c_int = 0 as libc::c_int;
            let mut k: libc::c_int = 0;
            let mut l: libc::c_int = 0;
            /* generate the partition's first stage cascade value */
            if csubbits != 0 {
                let mut maxval: [libc::c_int; 8] = [
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ]; /* gcc's static analysis
                   issues a warning without
                   initialization */
                k = 0 as libc::c_int;
                while k < csub {
                    let mut booknum: libc::c_int =
                        (*info).class_subbook[class as usize][k as usize];
                    if booknum < 0 as libc::c_int {
                        maxval[k as usize] = 1 as libc::c_int
                    } else {
                        maxval[k as usize] = (**sbooks
                            .offset((*info).class_subbook[class as usize][k as usize] as isize))
                        .entries as libc::c_int
                    }
                    k += 1
                }
                k = 0 as libc::c_int;
                while k < cdim {
                    l = 0 as libc::c_int;
                    while l < csub {
                        let mut val_1: libc::c_int = out[(j + k as libc::c_long) as usize];
                        if val_1 < maxval[l as usize] {
                            bookas[k as usize] = l;
                            break;
                        } else {
                            l += 1
                        }
                    }
                    cval |= bookas[k as usize] << cshift;
                    cshift += csubbits;
                    k += 1
                }
                /* write it */
                (*look).phrasebits += crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
                    books.offset((*info).class_book[class as usize] as isize)
                        as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    cval,
                    opb as *mut crate::ogg_h::oggpack_buffer,
                ) as libc::c_long
            }
            /* write post values */
            k = 0 as libc::c_int;
            while k < cdim {
                let mut book: libc::c_int =
                    (*info).class_subbook[class as usize][bookas[k as usize] as usize];
                if book >= 0 as libc::c_int {
                    /* hack to allow training with 'bad' books */
                    if (out[(j + k as libc::c_long) as usize] as libc::c_long)
                        < (*books.offset(book as isize)).entries
                    {
                        (*look).postbits +=
                            crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_encode(
                                books.offset(book as isize)
                                    as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                                out[(j + k as libc::c_long) as usize],
                                opb as *mut crate::ogg_h::oggpack_buffer,
                            ) as libc::c_long
                    }
                    /*else
                    fprintf(stderr,"+!");*/
                }
                k += 1
            }
            j += cdim as libc::c_long;
            i += 1
        }
        /* generate quantized floor equivalent to what we'd unpack in decode */
        /* render the lines */
        let mut hx: libc::c_int = 0 as libc::c_int; /* be certain */
        let mut lx: libc::c_int = 0 as libc::c_int;
        let mut ly: libc::c_int = *post.offset(0 as libc::c_int as isize) * (*info).mult;
        let mut n: libc::c_int =
            ((*ci).blocksizes[(*vb).W as usize] / 2 as libc::c_int as libc::c_long) as libc::c_int;
        j = 1 as libc::c_int as libc::c_long;
        while j < (*look).posts as libc::c_long {
            let mut current: libc::c_int = (*look).forward_index[j as usize];
            let mut hy: libc::c_int = *post.offset(current as isize) & 0x7fff as libc::c_int;
            if hy == *post.offset(current as isize) {
                hy *= (*info).mult;
                hx = (*info).postlist[current as usize];
                render_line0(n, lx, hx, ly, hy, ilogmask);
                lx = hx;
                ly = hy
            }
            j += 1
        }
        j = hx as libc::c_long;
        while j < ((*vb).pcmend / 2 as libc::c_int) as libc::c_long {
            *ilogmask.offset(j as isize) = ly;
            j += 1
        }
        return 1 as libc::c_int;
    } else {
        crate::src::libogg_1_3_3::src::bitwise::oggpack_write(
            opb as *mut crate::ogg_h::oggpack_buffer,
            0 as libc::c_int as libc::c_ulong,
            1 as libc::c_int,
        );
        crate::stdlib::memset(
            ilogmask as *mut libc::c_void,
            0 as libc::c_int,
            (((*vb).pcmend / 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        return 0 as libc::c_int;
    };
}

unsafe extern "C" fn floor1_inverse1(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        in_0 as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut books: *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook = (*ci).fullbooks;
    /* unpack wrapped/predicted values from stream */
    if crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
        &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
        1 as libc::c_int,
    ) == 1 as libc::c_int as libc::c_long
    {
        let mut fit_value: *mut libc::c_int =
            crate::src::libvorbis_1_3_6::lib::block::_vorbis_block_alloc(
                vb as *mut crate::codec_h::vorbis_block,
                ((*look).posts as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as libc::c_long,
            ) as *mut libc::c_int;
        *fit_value.offset(0 as libc::c_int as isize) =
            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                    ((*look).quant_q - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
                ),
            ) as libc::c_int;
        *fit_value.offset(1 as libc::c_int as isize) =
            crate::src::libogg_1_3_3::src::bitwise::oggpack_read(
                &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
                    ((*look).quant_q - 1 as libc::c_int) as crate::config_types_h::ogg_uint32_t,
                ),
            ) as libc::c_int;
        /* partition by partition */
        i = 0 as libc::c_int;
        j = 2 as libc::c_int;
        's_33: loop {
            if !(i < (*info).partitions) {
                current_block = 2719512138335094285;
                break;
            }
            let mut class: libc::c_int = (*info).partitionclass[i as usize];
            let mut cdim: libc::c_int = (*info).class_dim[class as usize];
            let mut csubbits: libc::c_int = (*info).class_subs[class as usize];
            let mut csub: libc::c_int = (1 as libc::c_int) << csubbits;
            let mut cval: libc::c_int = 0 as libc::c_int;
            /* decode the partition's first stage cascade value */
            if csubbits != 0 {
                cval = crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                    books.offset((*info).class_book[class as usize] as isize)
                        as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                    &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                ) as libc::c_int;
                if cval == -(1 as libc::c_int) {
                    current_block = 11812956849860977184;
                    break;
                }
            }
            k = 0 as libc::c_int;
            while k < cdim {
                let mut book: libc::c_int = (*info).class_subbook[class as usize]
                    [(cval & csub - 1 as libc::c_int) as usize];
                cval >>= csubbits;
                if book >= 0 as libc::c_int {
                    let ref mut fresh0 = *fit_value.offset((j + k) as isize);
                    *fresh0 = crate::src::libvorbis_1_3_6::lib::codebook::vorbis_book_decode(
                        books.offset(book as isize)
                            as *mut crate::src::libvorbis_1_3_6::lib::codebook::codebook,
                        &mut (*vb).opb as *mut _ as *mut crate::ogg_h::oggpack_buffer,
                    ) as libc::c_int;
                    if *fresh0 == -(1 as libc::c_int) {
                        current_block = 11812956849860977184;
                        break 's_33;
                    }
                } else {
                    *fit_value.offset((j + k) as isize) = 0 as libc::c_int
                }
                k += 1
            }
            j += cdim;
            i += 1
        }
        match current_block {
            11812956849860977184 => {}
            _ => {
                /* unwrap positive values and reconsitute via linear interpolation */
                i = 2 as libc::c_int;
                while i < (*look).posts {
                    let mut predicted: libc::c_int = render_point(
                        (*info).postlist
                            [(*look).loneighbor[(i - 2 as libc::c_int) as usize] as usize],
                        (*info).postlist
                            [(*look).hineighbor[(i - 2 as libc::c_int) as usize] as usize],
                        *fit_value
                            .offset((*look).loneighbor[(i - 2 as libc::c_int) as usize] as isize),
                        *fit_value
                            .offset((*look).hineighbor[(i - 2 as libc::c_int) as usize] as isize),
                        (*info).postlist[i as usize],
                    );
                    let mut hiroom: libc::c_int = (*look).quant_q - predicted;
                    let mut loroom: libc::c_int = predicted;
                    let mut room: libc::c_int =
                        (if hiroom < loroom { hiroom } else { loroom }) << 1 as libc::c_int;
                    let mut val: libc::c_int = *fit_value.offset(i as isize);
                    if val != 0 {
                        if val >= room {
                            if hiroom > loroom {
                                val = val - loroom
                            } else {
                                val = -(1 as libc::c_int) - (val - hiroom)
                            }
                        } else if val & 1 as libc::c_int != 0 {
                            val = -(val + 1 as libc::c_int >> 1 as libc::c_int)
                        } else {
                            val >>= 1 as libc::c_int
                        }
                        *fit_value.offset(i as isize) = val + predicted & 0x7fff as libc::c_int;
                        *fit_value.offset(
                            (*look).loneighbor[(i - 2 as libc::c_int) as usize] as isize,
                        ) &= 0x7fff as libc::c_int;
                        *fit_value
                            .offset((*look).hineighbor[(i - 2 as libc::c_int) as usize] as isize) &=
                            0x7fff as libc::c_int
                    } else {
                        *fit_value.offset(i as isize) = predicted | 0x8000 as libc::c_int
                    }
                    i += 1
                }
                return fit_value as *mut libc::c_void;
            }
        }
    }
    return 0 as *mut libc::c_void;
}

unsafe extern "C" fn floor1_inverse2(
    mut vb: *mut crate::codec_h::vorbis_block,
    mut in_0: *mut libc::c_void,
    mut memo: *mut libc::c_void,
    mut out: *mut libc::c_float,
) -> libc::c_int {
    let mut look: *mut crate::codec_internal_h::vorbis_look_floor1 =
        in_0 as *mut crate::codec_internal_h::vorbis_look_floor1;
    let mut info: *mut crate::backends_h::vorbis_info_floor1 = (*look).vi;
    let mut ci: *mut crate::codec_internal_h::codec_setup_info =
        (*(*(*vb).vd).vi).codec_setup as *mut crate::codec_internal_h::codec_setup_info;
    let mut n: libc::c_int =
        ((*ci).blocksizes[(*vb).W as usize] / 2 as libc::c_int as libc::c_long) as libc::c_int;
    let mut j: libc::c_int = 0;
    if !memo.is_null() {
        /* render the lines */
        let mut fit_value: *mut libc::c_int = memo as *mut libc::c_int;
        let mut hx: libc::c_int = 0 as libc::c_int;
        let mut lx: libc::c_int = 0 as libc::c_int;
        let mut ly: libc::c_int = *fit_value.offset(0 as libc::c_int as isize) * (*info).mult;
        /* guard lookup against out-of-range values */
        ly = if ly < 0 as libc::c_int {
            0 as libc::c_int
        } else if ly > 255 as libc::c_int {
            255 as libc::c_int
        } else {
            ly
        };
        j = 1 as libc::c_int;
        while j < (*look).posts {
            let mut current: libc::c_int = (*look).forward_index[j as usize];
            let mut hy: libc::c_int = *fit_value.offset(current as isize) & 0x7fff as libc::c_int;
            if hy == *fit_value.offset(current as isize) {
                hx = (*info).postlist[current as usize];
                hy *= (*info).mult;
                /* guard lookup against out-of-range values */
                hy = if hy < 0 as libc::c_int {
                    0 as libc::c_int
                } else if hy > 255 as libc::c_int {
                    255 as libc::c_int
                } else {
                    hy
                }; /* be certain */
                render_line(n, lx, hx, ly, hy, out);
                lx = hx;
                ly = hy
            }
            j += 1
        }
        j = hx;
        while j < n {
            *out.offset(j as isize) *= FLOOR1_fromdB_LOOKUP[ly as usize];
            j += 1
        }
        return 1 as libc::c_int;
    }
    crate::stdlib::memset(
        out as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong),
    );
    return 0 as libc::c_int;
}
/* export hooks */
#[no_mangle]

pub static mut floor1_exportbundle: crate::backends_h::vorbis_func_floor = {
    let mut init = crate::backends_h::vorbis_func_floor {
        pack: Some(
            floor1_pack
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> (),
        ),
        unpack: Some(
            floor1_unpack
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_info,
                    _: *mut crate::ogg_h::oggpack_buffer,
                ) -> *mut libc::c_void,
        ),
        look: Some(
            floor1_look
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_dsp_state,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free_info: Some(floor1_free_info as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        free_look: Some(floor1_free_look as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
        inverse1: Some(
            floor1_inverse1
                as unsafe extern "C" fn(
                    _: *mut crate::codec_h::vorbis_block,
                    _: *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        inverse2: Some(
            floor1_inverse2
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

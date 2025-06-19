use ::libc;

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128 as i32) && __c < 256 as i32 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::backends_h::vorbis_func_floor;
pub use crate::backends_h::vorbis_func_mapping;
pub use crate::backends_h::vorbis_func_residue;
pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_comment;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::codec_internal_h::codec_setup_info;
pub use crate::codec_internal_h::private_state;
pub use crate::codec_internal_h::vorbis_info_floor;
pub use crate::codec_internal_h::vorbis_info_mapping;
pub use crate::codec_internal_h::vorbis_info_mode;
pub use crate::codec_internal_h::vorbis_info_residue;
pub use crate::codec_internal_h::vorbis_look_floor;
pub use crate::codec_internal_h::vorbis_look_residue;
pub use crate::codec_internal_h::vorbis_look_transform;
pub use crate::config_types_h::ogg_int32_t;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::highlevel_h::highlevel_byblocktype;
pub use crate::highlevel_h::highlevel_encode_setup;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_bytes;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_read;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_readinit;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_reset;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_write;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeclear;
pub use crate::src::libogg_1_3_3::src::bitwise::oggpack_writeinit;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_info;
pub use crate::src::libvorbis_1_3_6::lib::bitrate::bitrate_manager_state;
pub use crate::src::libvorbis_1_3_6::lib::codebook::codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::static_codebook;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_pack;
pub use crate::src::libvorbis_1_3_6::lib::codebook::vorbis_staticbook_unpack;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_band;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_filter_state;
pub use crate::src::libvorbis_1_3_6::lib::envelope::envelope_lookup;
pub use crate::src::libvorbis_1_3_6::lib::mdct::mdct_lookup;
pub use crate::src::libvorbis_1_3_6::lib::psy::_vi_psy_free;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_info_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy;
pub use crate::src::libvorbis_1_3_6::lib::psy::vorbis_look_psy_global;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_book_clear;
pub use crate::src::libvorbis_1_3_6::lib::sharedbook::vorbis_staticbook_destroy;
pub use crate::src::libvorbis_1_3_6::lib::smallft::drft_lookup;

pub use crate::src::libvorbis_1_3_6::lib::info::ctype_h::toupper;

pub use crate::stdlib::__ctype_toupper_loc;

/* helpers */

unsafe extern "C" fn _v_writestring(
    mut o: *mut oggpack_buffer,
    mut s: *const libc::c_char,
    mut bytes: i32,
) {
    loop {
        let fresh0 = bytes;
        bytes = bytes - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        oggpack_write(o as *mut oggpack_buffer, *fresh1 as usize, 8 as i32);
    }
}

unsafe extern "C" fn _v_readstring(
    mut o: *mut oggpack_buffer,
    mut buf: *mut libc::c_char,
    mut bytes: i32,
) {
    loop {
        let fresh2 = bytes;
        bytes = bytes - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = buf;
        buf = buf.offset(1);
        *fresh3 = oggpack_read(o as *mut oggpack_buffer, 8 as i32) as libc::c_char
    }
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_init(mut vc: *mut vorbis_comment) {
    crate::stdlib::memset(
        vc as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<vorbis_comment>() as usize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add(
    mut vc: *mut vorbis_comment,
    mut comment: *const libc::c_char,
) {
    (*vc).user_comments = crate::stdlib::realloc(
        (*vc).user_comments as *mut libc::c_void,
        (((*vc).comments + 2 as i32) as usize)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as usize),
    ) as *mut *mut libc::c_char;
    (*vc).comment_lengths = crate::stdlib::realloc(
        (*vc).comment_lengths as *mut libc::c_void,
        (((*vc).comments + 2 as i32) as usize).wrapping_mul(::std::mem::size_of::<i32>() as usize),
    ) as *mut i32;
    *(*vc).comment_lengths.offset((*vc).comments as isize) = crate::stdlib::strlen(comment) as i32;
    let ref mut fresh4 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh4 = crate::stdlib::malloc(
        (*(*vc).comment_lengths.offset((*vc).comments as isize) + 1 as i32) as usize,
    ) as *mut libc::c_char;
    libc::strcpy(
        *(*vc).user_comments.offset((*vc).comments as isize),
        comment,
    );
    (*vc).comments += 1;
    let ref mut fresh5 = *(*vc).user_comments.offset((*vc).comments as isize);
    *fresh5 = 0 as *mut libc::c_char;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_add_tag(
    mut vc: *mut vorbis_comment,
    mut tag: *const libc::c_char,
    mut contents: *const libc::c_char,
) {
    /* Length for key and value +2 for = and \0 */
    let mut comment: *mut libc::c_char = crate::stdlib::malloc(
        crate::stdlib::strlen(tag)
            .wrapping_add(crate::stdlib::strlen(contents))
            .wrapping_add(2 as i32 as usize),
    ) as *mut libc::c_char;
    libc::strcpy(comment, tag);
    libc::strcat(comment, b"=\x00" as *const u8 as *const libc::c_char);
    libc::strcat(comment, contents);
    vorbis_comment_add(vc, comment);
    libc::free(comment as *mut libc::c_void);
}
/* This is more or less the same as strncasecmp - but that doesn't exist
 * everywhere, and this is a fairly trivial function, so we include it */

unsafe extern "C" fn tagcompare(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: i32,
) -> i32 {
    let mut c: i32 = 0 as i32; /* +1 for the = we append */
    while c < n {
        if ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<libc::c_char>() as usize > 1 as i32 as usize {
                if 0 != 0 {
                    let mut __c: i32 = *s1.offset(c as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*s1.offset(c as isize) as i32)
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*s1.offset(c as isize) as i32 as isize)
            }
            __res
        }) != ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<libc::c_char>() as usize > 1 as i32 as usize {
                if 0 != 0 {
                    let mut __c: i32 = *s2.offset(c as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*s2.offset(c as isize) as i32)
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(*s2.offset(c as isize) as i32 as isize)
            }
            __res
        }) {
            return (0 as i32 == 0) as i32;
        }
        c += 1
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query(
    mut vc: *mut vorbis_comment,
    mut tag: *const libc::c_char,
    mut count: i32,
) -> *mut libc::c_char {
    let mut i: isize = 0;
    let mut found: i32 = 0 as i32;
    let mut taglen: i32 = crate::stdlib::strlen(tag).wrapping_add(1 as i32 as usize) as i32;
    let mut fulltag: *mut libc::c_char =
        crate::stdlib::malloc((taglen + 1 as i32) as usize) as *mut libc::c_char;
    libc::strcpy(fulltag, tag);
    libc::strcat(fulltag, b"=\x00" as *const u8 as *const libc::c_char);
    i = 0 as i32 as isize;
    while i < (*vc).comments as isize {
        if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
            if count == found {
                /* We return a pointer to the data, not a copy */
                libc::free(fulltag as *mut libc::c_void);
                return (*(*vc).user_comments.offset(i as isize)).offset(taglen as isize);
            } else {
                found += 1
            }
        }
        i += 1
    }
    libc::free(fulltag as *mut libc::c_void);
    return 0 as *mut libc::c_char;
    /* didn't find anything */
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_query_count(
    mut vc: *mut vorbis_comment,
    mut tag: *const libc::c_char,
) -> i32 {
    let mut i: i32 = 0; /* +1 for the = we append */
    let mut count: i32 = 0 as i32;
    let mut taglen: i32 = crate::stdlib::strlen(tag).wrapping_add(1 as i32 as usize) as i32;
    let mut fulltag: *mut libc::c_char =
        crate::stdlib::malloc((taglen + 1 as i32) as usize) as *mut libc::c_char;
    libc::strcpy(fulltag, tag);
    libc::strcat(fulltag, b"=\x00" as *const u8 as *const libc::c_char);
    i = 0 as i32;
    while i < (*vc).comments {
        if tagcompare(*(*vc).user_comments.offset(i as isize), fulltag, taglen) == 0 {
            count += 1
        }
        i += 1
    }
    libc::free(fulltag as *mut libc::c_void);
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_comment_clear(mut vc: *mut vorbis_comment) {
    if !vc.is_null() {
        let mut i: isize = 0;
        if !(*vc).user_comments.is_null() {
            i = 0 as i32 as isize;
            while i < (*vc).comments as isize {
                if !(*(*vc).user_comments.offset(i as isize)).is_null() {
                    libc::free(*(*vc).user_comments.offset(i as isize) as *mut libc::c_void);
                }
                i += 1
            }
            libc::free((*vc).user_comments as *mut libc::c_void);
        }
        if !(*vc).comment_lengths.is_null() {
            libc::free((*vc).comment_lengths as *mut libc::c_void);
        }
        if !(*vc).vendor.is_null() {
            libc::free((*vc).vendor as *mut libc::c_void);
        }
        crate::stdlib::memset(
            vc as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<vorbis_comment>() as usize,
        );
    };
}
/* blocksize 0 is guaranteed to be short, 1 is guaranteed to be long.
They may be equal, but short will never ge greater than long */
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_blocksize(mut vi: *mut vorbis_info, mut zo: i32) -> i32 {
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    return if !ci.is_null() {
        (*ci).blocksizes[zo as usize]
    } else {
        -(1 as i32) as isize
    } as i32;
}
/* libvorbis encodes in two abstraction layers; first we perform DSP
and produce a packet (see docs/analysis.txt).  The packet is then
coded into a framed OggSquish bitstream by the second layer (see
docs/framing.txt).  Decode is the reverse process; we sync/frame
the bitstream and extract individual packets, then decode the
packet back into PCM audio.

The extra framing/packetizing is used in streaming formats, such as
files.  Over the net (such as with UDP), the framing and
packetization aren't necessary as they're provided by the transport
and the streaming layer is not used */
/* Vorbis PRIMITIVES: general ***************************************/
/* used by synthesis, which has a full, alloced vi */
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_init(mut vi: *mut vorbis_info) {
    crate::stdlib::memset(
        vi as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<vorbis_info>() as usize,
    );
    (*vi).codec_setup = crate::stdlib::calloc(
        1 as i32 as usize,
        ::std::mem::size_of::<codec_setup_info>() as usize,
    );
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_info_clear(mut vi: *mut vorbis_info) {
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    let mut i: i32 = 0;
    if !ci.is_null() {
        i = 0 as i32;
        while i < (*ci).modes {
            if !(*ci).mode_param[i as usize].is_null() {
                libc::free((*ci).mode_param[i as usize] as *mut libc::c_void);
            }
            i += 1
        }
        i = 0 as i32;
        while i < (*ci).maps {
            /* unpack does the range checking */
            if !(*ci).map_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
                    .as_ptr()
                    .offset((*ci).map_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")((*ci).map_param[i as usize]);
            }
            i += 1
        }
        i = 0 as i32;
        while i < (*ci).floors {
            /* unpack does the range checking */
            if !(*ci).floor_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")((*ci).floor_param[i as usize]);
            }
            i += 1
        }
        i = 0 as i32;
        while i < (*ci).residues {
            /* unpack does the range checking */
            if !(*ci).residue_param[i as usize].is_null() {
                /* this may be cleaning up an aborted
                unpack, in which case the below type
                cannot be trusted */
                (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                    .as_ptr()
                    .offset((*ci).residue_type[i as usize] as isize))
                .free_info
                .expect("non-null function pointer")(
                    (*ci).residue_param[i as usize]
                );
            }
            i += 1
        }
        i = 0 as i32;
        while i < (*ci).books {
            if !(*ci).book_param[i as usize].is_null() {
                /* knows if the book was not alloced */
                vorbis_staticbook_destroy((*ci).book_param[i as usize] as *mut static_codebook);
            }
            if !(*ci).fullbooks.is_null() {
                vorbis_book_clear((*ci).fullbooks.offset(i as isize) as *mut codebook);
            }
            i += 1
        }
        if !(*ci).fullbooks.is_null() {
            libc::free((*ci).fullbooks as *mut libc::c_void);
        }
        i = 0 as i32;
        while i < (*ci).psys {
            _vi_psy_free((*ci).psy_param[i as usize] as *mut vorbis_info_psy);
            i += 1
        }
        libc::free(ci as *mut libc::c_void);
    }
    crate::stdlib::memset(
        vi as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<vorbis_info>() as usize,
    );
}
/* Header packing/unpacking ********************************************/

unsafe extern "C" fn _vorbis_unpack_info(
    mut vi: *mut vorbis_info,
    mut opb: *mut oggpack_buffer,
) -> i32 {
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info; /* EOP check */
    if ci.is_null() {
        return -(129 as i32);
    } /* EOP check */
    (*vi).version = oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as i32;
    if (*vi).version != 0 as i32 {
        return -(134 as i32);
    }
    (*vi).channels = oggpack_read(opb as *mut oggpack_buffer, 8 as i32) as i32;
    (*vi).rate = oggpack_read(opb as *mut oggpack_buffer, 32 as i32);
    (*vi).bitrate_upper =
        oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as ogg_int32_t as isize;
    (*vi).bitrate_nominal =
        oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as ogg_int32_t as isize;
    (*vi).bitrate_lower =
        oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as ogg_int32_t as isize;
    (*ci).blocksizes[0 as i32 as usize] =
        ((1 as i32) << oggpack_read(opb as *mut oggpack_buffer, 4 as i32)) as isize;
    (*ci).blocksizes[1 as i32 as usize] =
        ((1 as i32) << oggpack_read(opb as *mut oggpack_buffer, 4 as i32)) as isize;
    if !((*vi).rate < 1 as i32 as isize) {
        if !((*vi).channels < 1 as i32) {
            if !((*ci).blocksizes[0 as i32 as usize] < 64 as i32 as isize) {
                if !((*ci).blocksizes[1 as i32 as usize] < (*ci).blocksizes[0 as i32 as usize]) {
                    if !((*ci).blocksizes[1 as i32 as usize] > 8192 as i32 as isize) {
                        if !(oggpack_read(opb as *mut oggpack_buffer, 1 as i32)
                            != 1 as i32 as isize)
                        {
                            return 0 as i32;
                        }
                    }
                }
            }
        }
    }
    vorbis_info_clear(vi);
    return -(133 as i32);
}

unsafe extern "C" fn _vorbis_unpack_comment(
    mut vc: *mut vorbis_comment,
    mut opb: *mut oggpack_buffer,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut vendorlen: i32 = oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as i32;
    if !(vendorlen < 0 as i32) {
        if !(vendorlen as isize > (*opb).storage - 8 as i32 as isize) {
            (*vc).vendor = crate::stdlib::calloc((vendorlen + 1 as i32) as usize, 1 as i32 as usize)
                as *mut libc::c_char;
            _v_readstring(opb, (*vc).vendor, vendorlen);
            i = oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as i32;
            if !(i < 0 as i32) {
                if !(i as isize
                    > (*opb).storage - oggpack_bytes(opb as *mut oggpack_buffer) >> 2 as i32)
                {
                    (*vc).comments = i;
                    (*vc).user_comments = crate::stdlib::calloc(
                        ((*vc).comments + 1 as i32) as usize,
                        ::std::mem::size_of::<*mut libc::c_char>() as usize,
                    ) as *mut *mut libc::c_char;
                    (*vc).comment_lengths = crate::stdlib::calloc(
                        ((*vc).comments + 1 as i32) as usize,
                        ::std::mem::size_of::<i32>() as usize,
                    ) as *mut i32;
                    i = 0 as i32;
                    loop {
                        if !(i < (*vc).comments) {
                            current_block = 6057473163062296781;
                            break;
                        }
                        let mut len: i32 =
                            oggpack_read(opb as *mut oggpack_buffer, 32 as i32) as i32;
                        if len < 0 as i32 {
                            current_block = 16389255375241467587;
                            break;
                        }
                        if len as isize > (*opb).storage - oggpack_bytes(opb as *mut oggpack_buffer)
                        {
                            current_block = 16389255375241467587;
                            break;
                        }
                        *(*vc).comment_lengths.offset(i as isize) = len;
                        let ref mut fresh6 = *(*vc).user_comments.offset(i as isize);
                        *fresh6 =
                            crate::stdlib::calloc((len + 1 as i32) as usize, 1 as i32 as usize)
                                as *mut libc::c_char;
                        _v_readstring(opb, *(*vc).user_comments.offset(i as isize), len);
                        i += 1
                    }
                    match current_block {
                        16389255375241467587 => {}
                        _ => {
                            if !(oggpack_read(opb as *mut oggpack_buffer, 1 as i32)
                                != 1 as i32 as isize)
                            {
                                return 0 as i32;
                            }
                        }
                    }
                }
            }
        }
    }
    vorbis_comment_clear(vc);
    return -(133 as i32);
}
/* all of the real encoding details are here.  The modes, books,
everything */

unsafe extern "C" fn _vorbis_unpack_books(
    mut vi: *mut vorbis_info,
    mut opb: *mut oggpack_buffer,
) -> i32 {
    let mut current_block: u64;
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    let mut i: i32 = 0;
    /* codebooks */
    (*ci).books = (oggpack_read(opb as *mut oggpack_buffer, 8 as i32) + 1 as i32 as isize) as i32;
    if !((*ci).books <= 0 as i32) {
        i = 0 as i32;
        loop {
            if !(i < (*ci).books) {
                current_block = 14523784380283086299;
                break;
            }
            (*ci).book_param[i as usize] =
                vorbis_staticbook_unpack(opb as *mut oggpack_buffer) as *mut static_codebook;
            if (*ci).book_param[i as usize].is_null() {
                current_block = 9493312797378346061;
                break;
            }
            i += 1
        }
        match current_block {
            9493312797378346061 => {}
            _ =>
            /* time backend settings; hooks are unused */
            {
                let mut times: i32 =
                    (oggpack_read(opb as *mut oggpack_buffer, 6 as i32) + 1 as i32 as isize) as i32;
                if !(times <= 0 as i32) {
                    i = 0 as i32;
                    loop {
                        if !(i < times) {
                            current_block = 17860125682698302841;
                            break;
                        }
                        let mut test: i32 =
                            oggpack_read(opb as *mut oggpack_buffer, 16 as i32) as i32;
                        if test < 0 as i32 || test >= 1 as i32 {
                            current_block = 9493312797378346061;
                            break;
                        }
                        i += 1
                    }
                    match current_block {
                        9493312797378346061 => {}
                        _ => {
                            /* floor backend settings */
                            (*ci).floors = (oggpack_read(opb as *mut oggpack_buffer, 6 as i32)
                                + 1 as i32 as isize)
                                as i32;
                            if !((*ci).floors <= 0 as i32) {
                                i = 0 as i32;
                                loop {
                                    if !(i < (*ci).floors) {
                                        current_block = 10652014663920648156;
                                        break;
                                    }
                                    (*ci).floor_type[i as usize] =
                                        oggpack_read(opb as *mut oggpack_buffer, 16 as i32) as i32;
                                    if (*ci).floor_type[i as usize] < 0 as i32
                                        || (*ci).floor_type[i as usize] >= 2 as i32
                                    {
                                        current_block = 9493312797378346061;
                                        break;
                                    }
                                    (*ci).floor_param[i as usize] =
                                        (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                                            .as_ptr()
                                            .offset((*ci).floor_type[i as usize] as isize))
                                        .unpack
                                        .expect("non-null function pointer")(
                                            vi, opb
                                        );
                                    if (*ci).floor_param[i as usize].is_null() {
                                        current_block = 9493312797378346061;
                                        break;
                                    }
                                    i += 1
                                }
                                match current_block {
                                    9493312797378346061 => {}
                                    _ => {
                                        /* residue backend settings */
                                        (*ci).residues =
                                            (oggpack_read(opb as *mut oggpack_buffer, 6 as i32)
                                                + 1 as i32 as isize)
                                                as i32;
                                        if !((*ci).residues <= 0 as i32) {
                                            i = 0 as i32;
                                            loop {
                                                if !(i < (*ci).residues) {
                                                    current_block = 18377268871191777778;
                                                    break;
                                                }
                                                (*ci).residue_type[i as usize] = oggpack_read(
                                                    opb as *mut oggpack_buffer,
                                                    16 as i32,
                                                )
                                                    as i32;
                                                if (*ci).residue_type[i as usize] < 0 as i32
                                                    || (*ci).residue_type[i as usize] >= 3 as i32
                                                {
                                                    current_block = 9493312797378346061;
                                                    break;
                                                }
                                                (*ci).residue_param[i as
                                                                        usize]
                                                    =
                                                    (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P.as_ptr().offset((*ci).residue_type[i
                                                                                                         as
                                                                                                         usize]
                                                                                      as
                                                                                      isize)).unpack.expect("non-null function pointer")(vi,
                                                                                                                                         opb);
                                                if (*ci).residue_param[i as usize].is_null() {
                                                    current_block = 9493312797378346061;
                                                    break;
                                                }
                                                i += 1
                                            }
                                            match current_block {
                                                9493312797378346061 => {}
                                                _ => {
                                                    /* map backend settings */
                                                    (*ci).maps = (oggpack_read(
                                                        opb as *mut oggpack_buffer,
                                                        6 as i32,
                                                    ) + 1 as i32 as isize)
                                                        as i32;
                                                    if !((*ci).maps <= 0 as i32) {
                                                        i = 0 as i32;
                                                        loop {
                                                            if !(i < (*ci).maps) {
                                                                current_block =
                                                                    17784502470059252271;
                                                                break;
                                                            }
                                                            (*ci).map_type[i as usize] =
                                                                oggpack_read(
                                                                    opb as *mut oggpack_buffer,
                                                                    16 as i32,
                                                                )
                                                                    as i32;
                                                            if (*ci).map_type[i as usize] < 0 as i32
                                                                || (*ci).map_type[i as usize]
                                                                    >= 1 as i32
                                                            {
                                                                current_block = 9493312797378346061;
                                                                break;
                                                            }
                                                            (*ci).map_param[i
                                                                                as
                                                                                usize]
                                                                =
                                                                (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P.as_ptr().offset((*ci).map_type[i
                                                                                                                 as
                                                                                                                 usize]
                                                                                                  as
                                                                                                  isize)).unpack.expect("non-null function pointer")(vi,
                                                                                                                                                     opb);
                                                            if (*ci).map_param[i as usize].is_null()
                                                            {
                                                                current_block = 9493312797378346061;
                                                                break;
                                                            }
                                                            i += 1
                                                        }
                                                        match current_block {
                                                            9493312797378346061 => {}
                                                            _ => {
                                                                /* mode settings */
                                                                (*ci).modes = (oggpack_read(
                                                                    opb as *mut oggpack_buffer,
                                                                    6 as i32,
                                                                ) + 1 as i32
                                                                    as isize)
                                                                    as i32; /* top level EOP check */
                                                                if !((*ci).modes <= 0 as i32) {
                                                                    i = 0 as i32;
                                                                    loop {
                                                                        if !(i < (*ci).modes) {
                                                                            current_block =
                                                                                9353995356876505083;
                                                                            break;
                                                                        }
                                                                        (*ci).mode_param[i
                                                                                             as
                                                                                             usize]
                                                                            =
                                                                            crate::stdlib::calloc(1
                                                                                       as
                                                                                       i32
                                                                                       as
                                                                                       usize,
                                                                                   ::std::mem::size_of::<vorbis_info_mode>()
                                                                                       as
                                                                                       usize)
                                                                                as
                                                                                *mut vorbis_info_mode;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).blockflag
                                                                            =
                                                                            oggpack_read(opb as *mut oggpack_buffer,
                                                                                         1
                                                                                             as
                                                                                             i32)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).windowtype
                                                                            =
                                                                            oggpack_read(opb as *mut oggpack_buffer,
                                                                                         16
                                                                                             as
                                                                                             i32)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).transformtype
                                                                            =
                                                                            oggpack_read(opb as *mut oggpack_buffer,
                                                                                         16
                                                                                             as
                                                                                             i32)
                                                                                as
                                                                                i32;
                                                                        (*(*ci).mode_param[i
                                                                                               as
                                                                                               usize]).mapping
                                                                            =
                                                                            oggpack_read(opb as *mut oggpack_buffer,
                                                                                         8
                                                                                             as
                                                                                             i32)
                                                                                as
                                                                                i32;
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .windowtype
                                                                            >= 1 as i32
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .transformtype
                                                                            >= 1 as i32
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .mapping
                                                                            >= (*ci).maps
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        if (*(*ci).mode_param
                                                                            [i as usize])
                                                                            .mapping
                                                                            < 0 as i32
                                                                        {
                                                                            current_block =
                                                                                9493312797378346061;
                                                                            break;
                                                                        }
                                                                        i += 1
                                                                    }
                                                                    match current_block
                                                                        {
                                                                        9493312797378346061
                                                                        => {
                                                                        }
                                                                        _ => {
                                                                            if !(oggpack_read(opb as *mut oggpack_buffer,
                                                                                              1
                                                                                                  as
                                                                                                  i32)
                                                                                     !=
                                                                                     1
                                                                                         as
                                                                                         i32
                                                                                         as
                                                                                         isize)
                                                                               {
                                                                                return 0
                                                                                           as
                                                                                           i32
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vorbis_info_clear(vi);
    return -(133 as i32);
}
/* Vorbis PRIMITIVES: synthesis layer *******************************/
/* Is this packet a vorbis ID header? */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_idheader(mut op: *mut ogg_packet) -> i32 {
    let mut opb: oggpack_buffer = oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    }; /* Not the initial packet */
    let mut buffer: [libc::c_char; 6] = [0; 6]; /* not an ID header */
    if !op.is_null() {
        oggpack_readinit(
            &mut opb as *mut _ as *mut oggpack_buffer,
            (*op).packet,
            (*op).bytes as i32,
        ); /* not vorbis */
        if (*op).b_o_s == 0 {
            return 0 as i32;
        }
        if oggpack_read(&mut opb as *mut _ as *mut oggpack_buffer, 8 as i32) != 1 as i32 as isize {
            return 0 as i32;
        }
        crate::stdlib::memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            6 as i32 as usize,
        );
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6 as i32);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as i32 as usize,
        ) != 0
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
    return 0 as i32;
}
/* The Vorbis header is in three packets; the initial small packet in
the first page that identifies basic parameters, a second packet
with bitstream comments and a third packet that holds the
codebook. */
#[no_mangle]

pub unsafe extern "C" fn vorbis_synthesis_headerin(
    mut vi: *mut vorbis_info,
    mut vc: *mut vorbis_comment,
    mut op: *mut ogg_packet,
) -> i32 {
    let mut opb: oggpack_buffer = oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    if !op.is_null() {
        oggpack_readinit(
            &mut opb as *mut _ as *mut oggpack_buffer,
            (*op).packet,
            (*op).bytes as i32,
        );
        /* Which of the three types of header is this? */
        /* Also verify header-ness, vorbis */
        let mut buffer: [libc::c_char; 6] = [0; 6];
        let mut packtype: i32 =
            oggpack_read(&mut opb as *mut _ as *mut oggpack_buffer, 8 as i32) as i32;
        crate::stdlib::memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            6 as i32 as usize,
        );
        _v_readstring(&mut opb, buffer.as_mut_ptr(), 6 as i32);
        if crate::stdlib::memcmp(
            buffer.as_mut_ptr() as *const libc::c_void,
            b"vorbis\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as i32 as usize,
        ) != 0
        {
            /* not a vorbis header */
            return -(132 as i32);
        }
        match packtype {
            1 => {
                /* least significant *bit* is read first */
                if (*op).b_o_s == 0 {
                    /* Not the initial packet */
                    return -(133 as i32);
                }
                if (*vi).rate != 0 as i32 as isize {
                    /* previously initialized info header */
                    return -(133 as i32);
                }
                return _vorbis_unpack_info(vi, &mut opb);
            }
            3 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0 as i32 as isize {
                    /* um... we didn't get the initial header */
                    return -(133 as i32);
                }
                if !(*vc).vendor.is_null() {
                    /* previously initialized comment header */
                    return -(133 as i32);
                }
                return _vorbis_unpack_comment(vc, &mut opb);
            }
            5 => {
                /* least significant *bit* is read first */
                if (*vi).rate == 0 as i32 as isize || (*vc).vendor.is_null() {
                    /* um... we didn;t get the initial header or comments yet */
                    return -(133 as i32);
                }
                if (*vi).codec_setup.is_null() {
                    /* improperly initialized vorbis_info */
                    return -(129 as i32);
                }
                if (*((*vi).codec_setup as *mut codec_setup_info)).books > 0 as i32 {
                    /* previously initialized setup header */
                    return -(133 as i32);
                }
                return _vorbis_unpack_books(vi, &mut opb);
            }
            _ => {
                /* Not a valid vorbis header type */
                return -(133 as i32);
            }
        }
    }
    return -(133 as i32);
}
/* pack side **********************************************************/

unsafe extern "C" fn _vorbis_pack_info(
    mut opb: *mut oggpack_buffer,
    mut vi: *mut vorbis_info,
) -> i32 {
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    if ci.is_null()
        || (*ci).blocksizes[0 as i32 as usize] < 64 as i32 as isize
        || (*ci).blocksizes[1 as i32 as usize] < (*ci).blocksizes[0 as i32 as usize]
    {
        return -(129 as i32);
    }
    /* preamble */
    oggpack_write(opb as *mut oggpack_buffer, 0x1 as i32 as usize, 8 as i32);
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as i32,
    );
    /* basic information about the stream */
    oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 32 as i32);
    oggpack_write(
        opb as *mut oggpack_buffer,
        (*vi).channels as usize,
        8 as i32,
    );
    oggpack_write(opb as *mut oggpack_buffer, (*vi).rate as usize, 32 as i32);
    oggpack_write(
        opb as *mut oggpack_buffer,
        (*vi).bitrate_upper as usize,
        32 as i32,
    );
    oggpack_write(
        opb as *mut oggpack_buffer,
        (*vi).bitrate_nominal as usize,
        32 as i32,
    );
    oggpack_write(
        opb as *mut oggpack_buffer,
        (*vi).bitrate_lower as usize,
        32 as i32,
    );
    oggpack_write(
        opb as *mut oggpack_buffer,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[0 as i32 as usize] - 1 as i32 as isize) as ogg_uint32_t,
        ) as usize,
        4 as i32,
    );
    oggpack_write(
        opb as *mut oggpack_buffer,
        crate::src::libvorbis_1_3_6::lib::sharedbook::ov_ilog(
            ((*ci).blocksizes[1 as i32 as usize] - 1 as i32 as isize) as ogg_uint32_t,
        ) as usize,
        4 as i32,
    );
    oggpack_write(opb as *mut oggpack_buffer, 1 as i32 as usize, 1 as i32);
    return 0 as i32;
}

unsafe extern "C" fn _vorbis_pack_comment(
    mut opb: *mut oggpack_buffer,
    mut vc: *mut vorbis_comment,
) -> i32 {
    let mut bytes: i32 = crate::stdlib::strlen(
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8
            as *const libc::c_char,
    ) as i32;
    /* preamble */
    oggpack_write(opb as *mut oggpack_buffer, 0x3 as i32 as usize, 8 as i32);
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as i32,
    );
    /* vendor */
    oggpack_write(opb as *mut oggpack_buffer, bytes as usize, 32 as i32);
    _v_writestring(
        opb,
        b"Xiph.Org libVorbis I 20180316 (Now 100% fewer shells)\x00" as *const u8
            as *const libc::c_char,
        bytes,
    );
    /* comments */
    oggpack_write(
        opb as *mut oggpack_buffer,
        (*vc).comments as usize,
        32 as i32,
    );
    if (*vc).comments != 0 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*vc).comments {
            if !(*(*vc).user_comments.offset(i as isize)).is_null() {
                oggpack_write(
                    opb as *mut oggpack_buffer,
                    *(*vc).comment_lengths.offset(i as isize) as usize,
                    32 as i32,
                );
                _v_writestring(
                    opb,
                    *(*vc).user_comments.offset(i as isize),
                    *(*vc).comment_lengths.offset(i as isize),
                );
            } else {
                oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 32 as i32);
            }
            i += 1
        }
    }
    oggpack_write(opb as *mut oggpack_buffer, 1 as i32 as usize, 1 as i32);
    return 0 as i32;
}

unsafe extern "C" fn _vorbis_pack_books(
    mut opb: *mut oggpack_buffer,
    mut vi: *mut vorbis_info,
) -> i32 {
    let mut current_block: u64;
    let mut ci: *mut codec_setup_info = (*vi).codec_setup as *mut codec_setup_info;
    let mut i: i32 = 0;
    if ci.is_null() {
        return -(129 as i32);
    }
    oggpack_write(opb as *mut oggpack_buffer, 0x5 as i32 as usize, 8 as i32);
    _v_writestring(
        opb,
        b"vorbis\x00" as *const u8 as *const libc::c_char,
        6 as i32,
    );
    /* books */
    oggpack_write(
        opb as *mut oggpack_buffer,
        ((*ci).books - 1 as i32) as usize,
        8 as i32,
    );
    i = 0 as i32;
    loop {
        if !(i < (*ci).books) {
            current_block = 14523784380283086299;
            break;
        }
        if vorbis_staticbook_pack(
            (*ci).book_param[i as usize] as *const static_codebook,
            opb as *mut oggpack_buffer,
        ) != 0
        {
            current_block = 9715915742856433172;
            break;
        }
        i += 1
    }
    match current_block {
        14523784380283086299 => {
            /* times; hook placeholders */
            oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 6 as i32);
            oggpack_write(opb as *mut oggpack_buffer, 0 as i32 as usize, 16 as i32);
            /* floors */
            oggpack_write(
                opb as *mut oggpack_buffer,
                ((*ci).floors - 1 as i32) as usize,
                6 as i32,
            );
            i = 0 as i32;
            loop {
                if !(i < (*ci).floors) {
                    current_block = 8831408221741692167;
                    break;
                }
                oggpack_write(
                    opb as *mut oggpack_buffer,
                    (*ci).floor_type[i as usize] as usize,
                    16 as i32,
                );
                if !(**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .pack
                .is_some()
                {
                    current_block = 9715915742856433172;
                    break;
                }
                (**crate::src::libvorbis_1_3_6::lib::registry::_floor_P
                    .as_ptr()
                    .offset((*ci).floor_type[i as usize] as isize))
                .pack
                .expect("non-null function pointer")(
                    (*ci).floor_param[i as usize], opb
                );
                i += 1
            }
            match current_block {
                9715915742856433172 => {}
                _ => {
                    /* residues */
                    oggpack_write(
                        opb as *mut oggpack_buffer,
                        ((*ci).residues - 1 as i32) as usize,
                        6 as i32,
                    );
                    i = 0 as i32;
                    while i < (*ci).residues {
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*ci).residue_type[i as usize] as usize,
                            16 as i32,
                        );
                        (**crate::src::libvorbis_1_3_6::lib::registry::_residue_P
                            .as_ptr()
                            .offset((*ci).residue_type[i as usize] as isize))
                        .pack
                        .expect("non-null function pointer")(
                            (*ci).residue_param[i as usize], opb
                        );
                        i += 1
                    }
                    /* maps */
                    oggpack_write(
                        opb as *mut oggpack_buffer,
                        ((*ci).maps - 1 as i32) as usize,
                        6 as i32,
                    );
                    i = 0 as i32;
                    while i < (*ci).maps {
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*ci).map_type[i as usize] as usize,
                            16 as i32,
                        );
                        (**crate::src::libvorbis_1_3_6::lib::registry::_mapping_P
                            .as_ptr()
                            .offset((*ci).map_type[i as usize] as isize))
                        .pack
                        .expect("non-null function pointer")(
                            vi, (*ci).map_param[i as usize], opb
                        );
                        i += 1
                    }
                    /* modes */
                    oggpack_write(
                        opb as *mut oggpack_buffer,
                        ((*ci).modes - 1 as i32) as usize,
                        6 as i32,
                    );
                    i = 0 as i32;
                    while i < (*ci).modes {
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).blockflag as usize,
                            1 as i32,
                        );
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).windowtype as usize,
                            16 as i32,
                        );
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).transformtype as usize,
                            16 as i32,
                        );
                        oggpack_write(
                            opb as *mut oggpack_buffer,
                            (*(*ci).mode_param[i as usize]).mapping as usize,
                            8 as i32,
                        );
                        i += 1
                    }
                    oggpack_write(opb as *mut oggpack_buffer, 1 as i32 as usize, 1 as i32);
                    return 0 as i32;
                }
            }
        }
        _ => {}
    }
    return -(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_commentheader_out(
    mut vc: *mut vorbis_comment,
    mut op: *mut ogg_packet,
) -> i32 {
    let mut opb: oggpack_buffer = oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    oggpack_writeinit(&mut opb as *mut _ as *mut oggpack_buffer);
    if _vorbis_pack_comment(&mut opb, vc) != 0 {
        oggpack_writeclear(&mut opb as *mut _ as *mut oggpack_buffer);
        return -(130 as i32);
    }
    (*op).packet =
        crate::stdlib::malloc(oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer) as usize)
            as *mut u8;
    crate::stdlib::memcpy(
        (*op).packet as *mut libc::c_void,
        opb.buffer as *const libc::c_void,
        oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer) as usize,
    );
    (*op).bytes = oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer);
    (*op).b_o_s = 0 as i32 as isize;
    (*op).e_o_s = 0 as i32 as isize;
    (*op).granulepos = 0 as i32 as ogg_int64_t;
    (*op).packetno = 1 as i32 as ogg_int64_t;
    oggpack_writeclear(&mut opb as *mut _ as *mut oggpack_buffer);
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_analysis_headerout(
    mut v: *mut vorbis_dsp_state,
    mut vc: *mut vorbis_comment,
    mut op: *mut ogg_packet,
    mut op_comm: *mut ogg_packet,
    mut op_code: *mut ogg_packet,
) -> i32 {
    let mut ret: i32 = -(130 as i32);
    let mut vi: *mut vorbis_info = (*v).vi;
    let mut opb: oggpack_buffer = oggpack_buffer {
        endbyte: 0,
        endbit: 0,
        buffer: 0 as *mut u8,
        ptr: 0 as *mut u8,
        storage: 0,
    };
    let mut b: *mut private_state = (*v).backend_state as *mut private_state;
    if b.is_null() || (*vi).channels <= 0 as i32 || (*vi).channels > 256 as i32 {
        b = 0 as *mut private_state;
        ret = -(129 as i32)
    } else {
        /* first header packet **********************************************/
        oggpack_writeinit(&mut opb as *mut _ as *mut oggpack_buffer);
        if !(_vorbis_pack_info(&mut opb, vi) != 0) {
            /* build the packet */
            if !(*b).header.is_null() {
                libc::free((*b).header as *mut libc::c_void);
            }
            (*b).header = crate::stdlib::malloc(oggpack_bytes(
                &mut opb as *mut _ as *mut oggpack_buffer,
            ) as usize) as *mut u8;
            crate::stdlib::memcpy(
                (*b).header as *mut libc::c_void,
                opb.buffer as *const libc::c_void,
                oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer) as usize,
            );
            (*op).packet = (*b).header;
            (*op).bytes = oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer);
            (*op).b_o_s = 1 as i32 as isize;
            (*op).e_o_s = 0 as i32 as isize;
            (*op).granulepos = 0 as i32 as ogg_int64_t;
            (*op).packetno = 0 as i32 as ogg_int64_t;
            /* second header packet (comments) **********************************/
            oggpack_reset(&mut opb as *mut _ as *mut oggpack_buffer);
            if !(_vorbis_pack_comment(&mut opb, vc) != 0) {
                if !(*b).header1.is_null() {
                    libc::free((*b).header1 as *mut libc::c_void);
                }
                (*b).header1 = crate::stdlib::malloc(oggpack_bytes(
                    &mut opb as *mut _ as *mut oggpack_buffer,
                ) as usize) as *mut u8;
                crate::stdlib::memcpy(
                    (*b).header1 as *mut libc::c_void,
                    opb.buffer as *const libc::c_void,
                    oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer) as usize,
                );
                (*op_comm).packet = (*b).header1;
                (*op_comm).bytes = oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer);
                (*op_comm).b_o_s = 0 as i32 as isize;
                (*op_comm).e_o_s = 0 as i32 as isize;
                (*op_comm).granulepos = 0 as i32 as ogg_int64_t;
                (*op_comm).packetno = 1 as i32 as ogg_int64_t;
                /* third header packet (modes/codebooks) ****************************/
                oggpack_reset(&mut opb as *mut _ as *mut oggpack_buffer);
                if !(_vorbis_pack_books(&mut opb, vi) != 0) {
                    if !(*b).header2.is_null() {
                        libc::free((*b).header2 as *mut libc::c_void);
                    }
                    (*b).header2 = crate::stdlib::malloc(oggpack_bytes(
                        &mut opb as *mut _ as *mut oggpack_buffer,
                    ) as usize) as *mut u8;
                    crate::stdlib::memcpy(
                        (*b).header2 as *mut libc::c_void,
                        opb.buffer as *const libc::c_void,
                        oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer) as usize,
                    );
                    (*op_code).packet = (*b).header2;
                    (*op_code).bytes = oggpack_bytes(&mut opb as *mut _ as *mut oggpack_buffer);
                    (*op_code).b_o_s = 0 as i32 as isize;
                    (*op_code).e_o_s = 0 as i32 as isize;
                    (*op_code).granulepos = 0 as i32 as ogg_int64_t;
                    (*op_code).packetno = 2 as i32 as ogg_int64_t;
                    oggpack_writeclear(&mut opb as *mut _ as *mut oggpack_buffer);
                    return 0 as i32;
                }
            }
        }
    }
    crate::stdlib::memset(
        op as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<ogg_packet>() as usize,
    );
    crate::stdlib::memset(
        op_comm as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<ogg_packet>() as usize,
    );
    crate::stdlib::memset(
        op_code as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<ogg_packet>() as usize,
    );
    if !b.is_null() {
        if (*vi).channels > 0 as i32 {
            oggpack_writeclear(&mut opb as *mut _ as *mut oggpack_buffer);
        }
        if !(*b).header.is_null() {
            libc::free((*b).header as *mut libc::c_void);
        }
        if !(*b).header1.is_null() {
            libc::free((*b).header1 as *mut libc::c_void);
        }
        if !(*b).header2.is_null() {
            libc::free((*b).header2 as *mut libc::c_void);
        }
        (*b).header = 0 as *mut u8;
        (*b).header1 = 0 as *mut u8;
        (*b).header2 = 0 as *mut u8
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_granule_time(
    mut v: *mut vorbis_dsp_state,
    mut granulepos: ogg_int64_t,
) -> f64 {
    if granulepos == -(1 as i32) as isize {
        return -(1 as i32) as f64;
    }
    /* We're not guaranteed a 64 bit unsigned type everywhere, so we
    have to put the unsigned granpo in a signed type. */
    if granulepos >= 0 as i32 as isize {
        return granulepos as f64 / (*(*v).vi).rate as f64;
    } else {
        let mut granuleoff: ogg_int64_t = 0xffffffff as u32 as ogg_int64_t;
        granuleoff <<= 31 as i32;
        granuleoff |= 0x7ffffffff as isize;
        return (granulepos as f64 + 2 as i32 as f64 + granuleoff as f64 + granuleoff as f64)
            / (*(*v).vi).rate as f64;
    };
}
#[no_mangle]

pub unsafe extern "C" fn vorbis_version_string() -> *const libc::c_char {
    return b"Xiph.Org libVorbis 1.3.6\x00" as *const u8 as *const libc::c_char;
}

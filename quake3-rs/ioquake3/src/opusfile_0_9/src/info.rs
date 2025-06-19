use ::libc;

pub use crate::config_types_h::ogg_uint32_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;

pub use crate::src::opusfile_0_9::src::opusfile::OpusHead;
pub use crate::src::opusfile_0_9::src::opusfile::OpusPictureTag;
pub use crate::src::opusfile_0_9::src::opusfile::OpusTags;

/* *******************************************************************
 *                                                                  *
 * THIS FILE IS PART OF THE libopusfile SOFTWARE CODEC SOURCE CODE. *
 * USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
 * GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
 * IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
 *                                                                  *
 * THE libopusfile SOURCE CODE IS (C) COPYRIGHT 2012                *
 * by the Xiph.Org Foundation and contributors http://www.xiph.org/ *
 *                                                                  *
 ********************************************************************/

unsafe extern "C" fn op_parse_uint16le(mut _data: *const u8) -> u32 {
    return (*_data.offset(0 as i32 as isize) as i32
        | (*_data.offset(1 as i32 as isize) as i32) << 8 as i32) as u32;
}

unsafe extern "C" fn op_parse_int16le(mut _data: *const u8) -> i32 {
    let mut ret: i32 = 0;
    ret = *_data.offset(0 as i32 as isize) as i32
        | (*_data.offset(1 as i32 as isize) as i32) << 8 as i32;
    return (ret ^ 0x8000 as i32) - 0x8000 as i32;
}

unsafe extern "C" fn op_parse_uint32le(mut _data: *const u8) -> crate::opus_types_h::opus_uint32 {
    return *_data.offset(0 as i32 as isize) as u32
        | (*_data.offset(1 as i32 as isize) as crate::opus_types_h::opus_uint32) << 8 as i32
        | (*_data.offset(2 as i32 as isize) as crate::opus_types_h::opus_uint32) << 16 as i32
        | (*_data.offset(3 as i32 as isize) as crate::opus_types_h::opus_uint32) << 24 as i32;
}

unsafe extern "C" fn op_parse_uint32be(mut _data: *const u8) -> crate::opus_types_h::opus_uint32 {
    return *_data.offset(3 as i32 as isize) as u32
        | (*_data.offset(2 as i32 as isize) as crate::opus_types_h::opus_uint32) << 8 as i32
        | (*_data.offset(1 as i32 as isize) as crate::opus_types_h::opus_uint32) << 16 as i32
        | (*_data.offset(0 as i32 as isize) as crate::opus_types_h::opus_uint32) << 24 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_head_parse(
    mut _head: *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
    mut _data: *const u8,
    mut _len: crate::stddef_h::size_t,
) -> i32 {
    let mut head: crate::src::opusfile_0_9::src::opusfile::OpusHead =
        crate::src::opusfile_0_9::src::opusfile::OpusHead {
            version: 0,
            channel_count: 0,
            pre_skip: 0,
            input_sample_rate: 0,
            output_gain: 0,
            mapping_family: 0,
            stream_count: 0,
            coupled_count: 0,
            mapping: [0; 255],
        };
    if _len < 8 as i32 as libc::c_ulong {
        return -(132 as i32);
    }
    if crate::stdlib::memcmp(
        _data as *const libc::c_void,
        b"OpusHead\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as i32 as libc::c_ulong,
    ) != 0 as i32
    {
        return -(132 as i32);
    }
    if _len < 9 as i32 as libc::c_ulong {
        return -(133 as i32);
    }
    head.version = *_data.offset(8 as i32 as isize) as i32;
    if head.version > 15 as i32 {
        return -(134 as i32);
    }
    if _len < 19 as i32 as libc::c_ulong {
        return -(133 as i32);
    }
    head.channel_count = *_data.offset(9 as i32 as isize) as i32;
    head.pre_skip = op_parse_uint16le(_data.offset(10 as i32 as isize));
    head.input_sample_rate = op_parse_uint32le(_data.offset(12 as i32 as isize));
    head.output_gain = op_parse_int16le(_data.offset(16 as i32 as isize));
    head.mapping_family = *_data.offset(18 as i32 as isize) as i32;
    if head.mapping_family == 0 as i32 {
        if head.channel_count < 1 as i32 || head.channel_count > 2 as i32 {
            return -(133 as i32);
        }
        if head.version <= 1 as i32 && _len > 19 as i32 as libc::c_ulong {
            return -(133 as i32);
        }
        head.stream_count = 1 as i32;
        head.coupled_count = head.channel_count - 1 as i32;
        if !_head.is_null() {
            (*_head).mapping[0 as i32 as usize] = 0 as i32 as u8;
            (*_head).mapping[1 as i32 as usize] = 1 as i32 as u8
        }
    } else if head.mapping_family == 1 as i32 {
        let mut size: crate::stddef_h::size_t = 0;
        let mut ci: i32 = 0;
        if head.channel_count < 1 as i32 || head.channel_count > 8 as i32 {
            return -(133 as i32);
        }
        size = (21 as i32 + head.channel_count) as crate::stddef_h::size_t;
        if _len < size || head.version <= 1 as i32 && _len > size {
            return -(133 as i32);
        }
        head.stream_count = *_data.offset(19 as i32 as isize) as i32;
        if head.stream_count < 1 as i32 {
            return -(133 as i32);
        }
        head.coupled_count = *_data.offset(20 as i32 as isize) as i32;
        if head.coupled_count > head.stream_count {
            return -(133 as i32);
        }
        ci = 0 as i32;
        while ci < head.channel_count {
            if *_data.offset((21 as i32 + ci) as isize) as i32
                >= head.stream_count + head.coupled_count
                && *_data.offset((21 as i32 + ci) as isize) as i32 != 255 as i32
            {
                return -(133 as i32);
            }
            ci += 1
        }
        if !_head.is_null() {
            crate::stdlib::memcpy(
                (*_head).mapping.as_mut_ptr() as *mut libc::c_void,
                _data.offset(21 as i32 as isize) as *const libc::c_void,
                head.channel_count as libc::c_ulong,
            );
        }
    } else if head.mapping_family == 255 as i32 {
        return -(130 as i32);
    } else {
        /*General purpose players should not attempt to play back content with
        channel mapping family 255.*/
        /*No other channel mapping families are currently defined.*/
        return -(133 as i32);
    }
    if !_head.is_null() {
        crate::stdlib::memcpy(
            _head as *mut libc::c_void,
            &mut head as *mut crate::src::opusfile_0_9::src::opusfile::OpusHead
                as *const libc::c_void,
            head.mapping.as_mut_ptr().offset_from(
                &mut head as *mut crate::src::opusfile_0_9::src::opusfile::OpusHead as *mut u8,
            ) as libc::c_long as libc::c_ulong,
        );
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_init(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
) {
    crate::stdlib::memset(
        _tags as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::opusfile_0_9::src::opusfile::OpusTags>() as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_clear(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
) {
    let mut ncomments: i32 = 0;
    let mut ci: i32 = 0;
    ncomments = (*_tags).comments;
    if !(*_tags).user_comments.is_null() {
        ncomments += 1
    }
    ci = ncomments;
    loop {
        let fresh0 = ci;
        ci = ci - 1;
        if !(fresh0 > 0 as i32) {
            break;
        }
        ::libc::free(*(*_tags).user_comments.offset(ci as isize) as *mut libc::c_void);
    }
    ::libc::free((*_tags).user_comments as *mut libc::c_void);
    ::libc::free((*_tags).comment_lengths as *mut libc::c_void);
    ::libc::free((*_tags).vendor as *mut libc::c_void);
}
/*Ensure there's room for up to _ncomments comments.*/

unsafe extern "C" fn op_tags_ensure_capacity(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _ncomments: crate::stddef_h::size_t,
) -> i32 {
    let mut user_comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut comment_lengths: *mut i32 = 0 as *mut i32;
    let mut cur_ncomments: i32 = 0;
    let mut size: crate::stddef_h::size_t = 0;
    if (_ncomments >= 2147483647 as i32 as crate::stddef_h::size_t) as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    size = (::std::mem::size_of::<i32>() as libc::c_ulong)
        .wrapping_mul(_ncomments.wrapping_add(1 as i32 as libc::c_ulong));
    if size.wrapping_div(::std::mem::size_of::<i32>() as libc::c_ulong)
        != _ncomments.wrapping_add(1 as i32 as libc::c_ulong)
    {
        return -(129 as i32);
    }
    cur_ncomments = (*_tags).comments;
    /*We only support growing.
    Trimming requires cleaning up the allocated strings in the old space, and
     is best handled separately if it's ever needed.*/
    comment_lengths = (*_tags).comment_lengths;
    comment_lengths =
        crate::stdlib::realloc((*_tags).comment_lengths as *mut libc::c_void, size) as *mut i32;
    if comment_lengths.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    if (*_tags).comment_lengths.is_null() {
        *comment_lengths.offset(cur_ncomments as isize) = 0 as i32
    }
    *comment_lengths.offset(_ncomments as isize) = *comment_lengths.offset(cur_ncomments as isize);
    (*_tags).comment_lengths = comment_lengths;
    size = (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_mul(_ncomments.wrapping_add(1 as i32 as libc::c_ulong));
    if size.wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        != _ncomments.wrapping_add(1 as i32 as libc::c_ulong)
    {
        return -(129 as i32);
    }
    user_comments = crate::stdlib::realloc((*_tags).user_comments as *mut libc::c_void, size)
        as *mut *mut libc::c_char;
    if user_comments.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    if (*_tags).user_comments.is_null() {
        let ref mut fresh1 = *user_comments.offset(cur_ncomments as isize);
        *fresh1 = 0 as *mut libc::c_char
    }
    let ref mut fresh2 = *user_comments.offset(_ncomments as isize);
    *fresh2 = *user_comments.offset(cur_ncomments as isize);
    (*_tags).user_comments = user_comments;
    return 0 as i32;
}
/*Duplicate a (possibly non-NUL terminated) string with a known length.*/

unsafe extern "C" fn op_strdup_with_len(
    mut _s: *const libc::c_char,
    mut _len: crate::stddef_h::size_t,
) -> *mut libc::c_char {
    let mut size: crate::stddef_h::size_t = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    size = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
        .wrapping_mul(_len.wrapping_add(1 as i32 as libc::c_ulong));
    if (size < _len) as i32 as libc::c_long != 0 {
        return 0 as *mut libc::c_char;
    }
    ret = crate::stdlib::malloc(size) as *mut libc::c_char;
    if !ret.is_null() as i32 as libc::c_long != 0 {
        ret = crate::stdlib::memcpy(
            ret as *mut libc::c_void,
            _s as *const libc::c_void,
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(_len),
        ) as *mut libc::c_char;
        *ret.offset(_len as isize) = '\u{0}' as i32 as libc::c_char
    }
    return ret;
}
/*The actual implementation of opus_tags_parse().
Unlike the public API, this function requires _tags to already be
 initialized, modifies its contents before success is guaranteed, and assumes
 the caller will clear it on error.*/

unsafe extern "C" fn opus_tags_parse_impl(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _data: *const u8,
    mut _len: crate::stddef_h::size_t,
) -> i32 {
    let mut count: crate::opus_types_h::opus_uint32 = 0;
    let mut len: crate::stddef_h::size_t = 0;
    let mut ncomments: i32 = 0;
    let mut ci: i32 = 0;
    len = _len;
    if len < 8 as i32 as libc::c_ulong {
        return -(132 as i32);
    }
    if crate::stdlib::memcmp(
        _data as *const libc::c_void,
        b"OpusTags\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as i32 as libc::c_ulong,
    ) != 0 as i32
    {
        return -(132 as i32);
    }
    if len < 16 as i32 as libc::c_ulong {
        return -(133 as i32);
    }
    _data = _data.offset(8 as i32 as isize);
    len = (len as libc::c_ulong).wrapping_sub(8 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    count = op_parse_uint32le(_data);
    _data = _data.offset(4 as i32 as isize);
    len = (len as libc::c_ulong).wrapping_sub(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    if count as libc::c_ulong > len {
        return -(133 as i32);
    }
    if !_tags.is_null() {
        (*_tags).vendor =
            op_strdup_with_len(_data as *mut libc::c_char, count as crate::stddef_h::size_t);
        if (*_tags).vendor.is_null() {
            return -(129 as i32);
        }
    }
    _data = _data.offset(count as isize);
    len = (len as libc::c_ulong).wrapping_sub(count as libc::c_ulong) as crate::stddef_h::size_t;
    if len < 4 as i32 as libc::c_ulong {
        return -(133 as i32);
    }
    count = op_parse_uint32le(_data);
    _data = _data.offset(4 as i32 as isize);
    len = (len as libc::c_ulong).wrapping_sub(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    /*Check to make sure there's minimally sufficient data left in the packet.*/
    if count as libc::c_ulong > len >> 2 as i32 {
        return -(133 as i32);
    }
    /*Check for overflow (the API limits this to an int).*/
    if count > (2147483647 as i32 as crate::opus_types_h::opus_uint32).wrapping_sub(1 as i32 as u32)
    {
        return -(129 as i32);
    }
    if !_tags.is_null() {
        let mut ret: i32 = 0;
        ret = op_tags_ensure_capacity(_tags, count as crate::stddef_h::size_t);
        if ret < 0 as i32 {
            return ret;
        }
    }
    ncomments = count as i32;
    ci = 0 as i32;
    while ci < ncomments {
        /*Check to make sure there's minimally sufficient data left in the packet.*/
        if (ncomments - ci) as crate::stddef_h::size_t > len >> 2 as i32 {
            return -(133 as i32);
        }
        count = op_parse_uint32le(_data);
        _data = _data.offset(4 as i32 as isize);
        len = (len as libc::c_ulong).wrapping_sub(4 as i32 as libc::c_ulong)
            as crate::stddef_h::size_t;
        if count as libc::c_ulong > len {
            return -(133 as i32);
        }
        /*Check for overflow (the API limits this to an int).*/
        if count > 2147483647 as i32 as crate::opus_types_h::opus_uint32 {
            return -(129 as i32);
        }
        if !_tags.is_null() {
            let ref mut fresh3 = *(*_tags).user_comments.offset(ci as isize);
            *fresh3 =
                op_strdup_with_len(_data as *mut libc::c_char, count as crate::stddef_h::size_t);
            if (*(*_tags).user_comments.offset(ci as isize)).is_null() {
                return -(129 as i32);
            }
            *(*_tags).comment_lengths.offset(ci as isize) = count as i32;
            (*_tags).comments = ci + 1 as i32;
            /*Needed by opus_tags_clear() if we fail before parsing the (optional)
            binary metadata.*/
            let ref mut fresh4 = *(*_tags).user_comments.offset((ci + 1 as i32) as isize);
            *fresh4 = 0 as *mut libc::c_char
        }
        _data = _data.offset(count as isize);
        len =
            (len as libc::c_ulong).wrapping_sub(count as libc::c_ulong) as crate::stddef_h::size_t;
        ci += 1
    }
    if len > 0 as i32 as libc::c_ulong && *_data.offset(0 as i32 as isize) as i32 & 1 as i32 != 0 {
        if len > 2147483647 as i32 as crate::opus_types_h::opus_uint32 as libc::c_ulong {
            return -(129 as i32);
        }
        if !_tags.is_null() {
            let ref mut fresh5 = *(*_tags).user_comments.offset(ncomments as isize);
            *fresh5 = crate::stdlib::malloc(len) as *mut libc::c_char;
            if (*(*_tags).user_comments.offset(ncomments as isize)).is_null() as i32 as libc::c_long
                != 0
            {
                return -(129 as i32);
            }
            crate::stdlib::memcpy(
                *(*_tags).user_comments.offset(ncomments as isize) as *mut libc::c_void,
                _data as *const libc::c_void,
                len,
            );
            *(*_tags).comment_lengths.offset(ncomments as isize) = len as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_parse(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _data: *const u8,
    mut _len: crate::stddef_h::size_t,
) -> i32 {
    if !_tags.is_null() {
        let mut tags: crate::src::opusfile_0_9::src::opusfile::OpusTags =
            crate::src::opusfile_0_9::src::opusfile::OpusTags {
                user_comments: 0 as *mut *mut libc::c_char,
                comment_lengths: 0 as *mut i32,
                comments: 0,
                vendor: 0 as *mut libc::c_char,
            };
        let mut ret: i32 = 0;
        opus_tags_init(&mut tags);
        ret = opus_tags_parse_impl(&mut tags, _data, _len);
        if ret < 0 as i32 {
            opus_tags_clear(&mut tags);
        } else {
            *_tags = tags
        }
        return ret;
    } else {
        return opus_tags_parse_impl(
            0 as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
            _data,
            _len,
        );
    };
}
/*The actual implementation of opus_tags_copy().
Unlike the public API, this function requires _dst to already be
 initialized, modifies its contents before success is guaranteed, and assumes
 the caller will clear it on error.*/

unsafe extern "C" fn opus_tags_copy_impl(
    mut _dst: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _src: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
) -> i32 {
    let mut vendor: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ncomments: i32 = 0;
    let mut ret: i32 = 0;
    let mut ci: i32 = 0;
    vendor = (*_src).vendor;
    (*_dst).vendor = op_strdup_with_len(vendor, crate::stdlib::strlen(vendor));
    if (*_dst).vendor.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    ncomments = (*_src).comments;
    ret = op_tags_ensure_capacity(_dst, ncomments as crate::stddef_h::size_t);
    if (ret < 0 as i32) as i32 as libc::c_long != 0 {
        return ret;
    }
    ci = 0 as i32;
    while ci < ncomments {
        let mut len: i32 = 0;
        len = *(*_src).comment_lengths.offset(ci as isize);
        let ref mut fresh6 = *(*_dst).user_comments.offset(ci as isize);
        *fresh6 = op_strdup_with_len(
            *(*_src).user_comments.offset(ci as isize),
            len as crate::stddef_h::size_t,
        );
        if (*(*_dst).user_comments.offset(ci as isize)).is_null() as i32 as libc::c_long != 0 {
            return -(129 as i32);
        }
        *(*_dst).comment_lengths.offset(ci as isize) = len;
        (*_dst).comments = ci + 1 as i32;
        ci += 1
    }
    if !(*_src).comment_lengths.is_null() {
        let mut len_0: i32 = 0;
        len_0 = *(*_src).comment_lengths.offset(ncomments as isize);
        if len_0 > 0 as i32 {
            let ref mut fresh7 = *(*_dst).user_comments.offset(ncomments as isize);
            *fresh7 = crate::stdlib::malloc(len_0 as libc::c_ulong) as *mut libc::c_char;
            if (*(*_dst).user_comments.offset(ncomments as isize)).is_null() as i32 as libc::c_long
                != 0
            {
                return -(129 as i32);
            }
            crate::stdlib::memcpy(
                *(*_dst).user_comments.offset(ncomments as isize) as *mut libc::c_void,
                *(*_src).user_comments.offset(ncomments as isize) as *const libc::c_void,
                len_0 as libc::c_ulong,
            );
            *(*_dst).comment_lengths.offset(ncomments as isize) = len_0
        }
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_copy(
    mut _dst: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _src: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
) -> i32 {
    let mut dst: crate::src::opusfile_0_9::src::opusfile::OpusTags =
        crate::src::opusfile_0_9::src::opusfile::OpusTags {
            user_comments: 0 as *mut *mut libc::c_char,
            comment_lengths: 0 as *mut i32,
            comments: 0,
            vendor: 0 as *mut libc::c_char,
        };
    let mut ret: i32 = 0;
    opus_tags_init(&mut dst);
    ret = opus_tags_copy_impl(&mut dst, _src);
    if (ret < 0 as i32) as i32 as libc::c_long != 0 {
        opus_tags_clear(&mut dst);
    } else {
        *_dst = dst
    }
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_add(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _tag: *const libc::c_char,
    mut _value: *const libc::c_char,
) -> i32 {
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag_len: crate::stddef_h::size_t = 0;
    let mut value_len: crate::stddef_h::size_t = 0;
    let mut ncomments: i32 = 0;
    let mut ret: i32 = 0;
    ncomments = (*_tags).comments;
    ret = op_tags_ensure_capacity(_tags, (ncomments + 1 as i32) as crate::stddef_h::size_t);
    if (ret < 0 as i32) as i32 as libc::c_long != 0 {
        return ret;
    }
    tag_len = crate::stdlib::strlen(_tag);
    value_len = crate::stdlib::strlen(_value);
    /*+2 for '=' and '\0'.*/
    if tag_len.wrapping_add(value_len) < tag_len {
        return -(129 as i32);
    }
    if tag_len.wrapping_add(value_len)
        > (2147483647 as i32 as crate::stddef_h::size_t).wrapping_sub(2 as i32 as libc::c_ulong)
    {
        return -(129 as i32);
    }
    comment = crate::stdlib::malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(
            tag_len
                .wrapping_add(value_len)
                .wrapping_add(2 as i32 as libc::c_ulong),
        ),
    ) as *mut libc::c_char;
    if comment.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    crate::stdlib::memcpy(
        comment as *mut libc::c_void,
        _tag as *const libc::c_void,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong).wrapping_mul(tag_len),
    );
    *comment.offset(tag_len as isize) = '=' as i32 as libc::c_char;
    crate::stdlib::memcpy(
        comment.offset(tag_len as isize).offset(1 as i32 as isize) as *mut libc::c_void,
        _value as *const libc::c_void,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(value_len.wrapping_add(1 as i32 as libc::c_ulong)),
    );
    let ref mut fresh8 = *(*_tags).user_comments.offset(ncomments as isize);
    *fresh8 = comment;
    *(*_tags).comment_lengths.offset(ncomments as isize) = tag_len
        .wrapping_add(value_len)
        .wrapping_add(1 as i32 as libc::c_ulong)
        as i32;
    (*_tags).comments = ncomments + 1 as i32;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_add_comment(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _comment: *const libc::c_char,
) -> i32 {
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comment_len: i32 = 0;
    let mut ncomments: i32 = 0;
    let mut ret: i32 = 0;
    ncomments = (*_tags).comments;
    ret = op_tags_ensure_capacity(_tags, (ncomments + 1 as i32) as crate::stddef_h::size_t);
    if (ret < 0 as i32) as i32 as libc::c_long != 0 {
        return ret;
    }
    comment_len = crate::stdlib::strlen(_comment) as i32;
    comment = op_strdup_with_len(_comment, comment_len as crate::stddef_h::size_t);
    if comment.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    let ref mut fresh9 = *(*_tags).user_comments.offset(ncomments as isize);
    *fresh9 = comment;
    *(*_tags).comment_lengths.offset(ncomments as isize) = comment_len;
    (*_tags).comments = ncomments + 1 as i32;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_set_binary_suffix(
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _data: *const u8,
    mut _len: i32,
) -> i32 {
    let mut binary_suffix_data: *mut u8 = 0 as *mut u8;
    let mut ncomments: i32 = 0;
    let mut ret: i32 = 0;
    if _len < 0 as i32
        || _len > 0 as i32
            && (_data.is_null() || *_data.offset(0 as i32 as isize) as i32 & 1 as i32 == 0)
    {
        return -(131 as i32);
    }
    ncomments = (*_tags).comments;
    ret = op_tags_ensure_capacity(_tags, ncomments as crate::stddef_h::size_t);
    if (ret < 0 as i32) as i32 as libc::c_long != 0 {
        return ret;
    }
    binary_suffix_data = crate::stdlib::realloc(
        *(*_tags).user_comments.offset(ncomments as isize) as *mut libc::c_void,
        _len as libc::c_ulong,
    ) as *mut u8;
    if binary_suffix_data.is_null() as i32 as libc::c_long != 0 {
        return -(129 as i32);
    }
    crate::stdlib::memcpy(
        binary_suffix_data as *mut libc::c_void,
        _data as *const libc::c_void,
        _len as libc::c_ulong,
    );
    let ref mut fresh10 = *(*_tags).user_comments.offset(ncomments as isize);
    *fresh10 = binary_suffix_data as *mut libc::c_char;
    *(*_tags).comment_lengths.offset(ncomments as isize) = _len;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tagcompare(
    mut _tag_name: *const libc::c_char,
    mut _comment: *const libc::c_char,
) -> i32 {
    let mut tag_len: crate::stddef_h::size_t = 0;
    tag_len = crate::stdlib::strlen(_tag_name);
    if (tag_len > 2147483647 as i32 as crate::stddef_h::size_t) as i32 as libc::c_long != 0 {
        return -(1 as i32);
    }
    return opus_tagncompare(_tag_name, tag_len as i32, _comment);
}
#[no_mangle]

pub unsafe extern "C" fn opus_tagncompare(
    mut _tag_name: *const libc::c_char,
    mut _tag_len: i32,
    mut _comment: *const libc::c_char,
) -> i32 {
    let mut ret: i32 = 0;
    ret = crate::src::opusfile_0_9::src::internal::op_strncasecmp(_tag_name, _comment, _tag_len);
    return if ret != 0 {
        ret
    } else {
        ('=' as i32) - *_comment.offset(_tag_len as isize) as i32
    };
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_query(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _tag: *const libc::c_char,
    mut _count: i32,
) -> *const libc::c_char {
    let mut user_comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tag_len: crate::stddef_h::size_t = 0;
    let mut found: i32 = 0;
    let mut ncomments: i32 = 0;
    let mut ci: i32 = 0;
    tag_len = crate::stdlib::strlen(_tag);
    if (tag_len > 2147483647 as i32 as crate::stddef_h::size_t) as i32 as libc::c_long != 0 {
        return 0 as *const libc::c_char;
    }
    ncomments = (*_tags).comments;
    user_comments = (*_tags).user_comments;
    found = 0 as i32;
    ci = 0 as i32;
    while ci < ncomments {
        if opus_tagncompare(_tag, tag_len as i32, *user_comments.offset(ci as isize)) == 0 {
            /*We return a pointer to the data, not a copy.*/
            let fresh11 = found;
            found = found + 1;
            if _count == fresh11 {
                return (*user_comments.offset(ci as isize))
                    .offset(tag_len as isize)
                    .offset(1 as i32 as isize);
            }
        }
        ci += 1
    }
    /*Didn't find anything.*/
    return 0 as *const libc::c_char;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_query_count(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _tag: *const libc::c_char,
) -> i32 {
    let mut user_comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tag_len: crate::stddef_h::size_t = 0;
    let mut found: i32 = 0;
    let mut ncomments: i32 = 0;
    let mut ci: i32 = 0;
    tag_len = crate::stdlib::strlen(_tag);
    if (tag_len > 2147483647 as i32 as crate::stddef_h::size_t) as i32 as libc::c_long != 0 {
        return 0 as i32;
    }
    ncomments = (*_tags).comments;
    user_comments = (*_tags).user_comments;
    found = 0 as i32;
    ci = 0 as i32;
    while ci < ncomments {
        if opus_tagncompare(_tag, tag_len as i32, *user_comments.offset(ci as isize)) == 0 {
            found += 1
        }
        ci += 1
    }
    return found;
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_get_binary_suffix(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _len: *mut i32,
) -> *const u8 {
    let mut ncomments: i32 = 0;
    let mut len: i32 = 0;
    ncomments = (*_tags).comments;
    len = if (*_tags).comment_lengths.is_null() {
        0 as i32
    } else {
        *(*_tags).comment_lengths.offset(ncomments as isize)
    };
    *_len = len;
    return if len > 0 as i32 {
        *(*_tags).user_comments.offset(ncomments as isize) as *const u8
    } else {
        0 as *const u8
    };
}

unsafe extern "C" fn opus_tags_get_gain(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _gain_q8: *mut i32,
    mut _tag_name: *const libc::c_char,
    mut _tag_len: crate::stddef_h::size_t,
) -> i32 {
    let mut comments: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ncomments: i32 = 0;
    let mut ci: i32 = 0;
    comments = (*_tags).user_comments;
    ncomments = (*_tags).comments;
    /*Look for the first valid tag with the name _tag_name and use that.*/
    ci = 0 as i32;
    while ci < ncomments {
        if opus_tagncompare(_tag_name, _tag_len as i32, *comments.offset(ci as isize)) == 0 as i32 {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut gain_q8: crate::opus_types_h::opus_int32 = 0;
            let mut negative: i32 = 0;
            p = (*comments.offset(ci as isize))
                .offset(_tag_len as isize)
                .offset(1 as i32 as isize);
            negative = 0 as i32;
            if *p as i32 == '-' as i32 {
                negative = -(1 as i32);
                p = p.offset(1)
            } else if *p as i32 == '+' as i32 {
                p = p.offset(1)
            }
            gain_q8 = 0 as i32;
            while *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
                gain_q8 = 10 as i32 * gain_q8 + *p as i32 - '0' as i32;
                if gain_q8 > 32767 as i32 - negative {
                    break;
                }
                p = p.offset(1)
            }
            /*This didn't look like a signed 16-bit decimal integer.
            Not a valid gain tag.*/
            if !(*p as i32 != '\u{0}' as i32) {
                *_gain_q8 = gain_q8 + negative ^ negative;
                return 0 as i32;
            }
        }
        ci += 1
    }
    return -(1 as i32);
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_get_album_gain(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _gain_q8: *mut i32,
) -> i32 {
    return opus_tags_get_gain(
        _tags,
        _gain_q8,
        b"R128_ALBUM_GAIN\x00" as *const u8 as *const libc::c_char,
        15 as i32 as crate::stddef_h::size_t,
    );
}
#[no_mangle]

pub unsafe extern "C" fn opus_tags_get_track_gain(
    mut _tags: *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _gain_q8: *mut i32,
) -> i32 {
    return opus_tags_get_gain(
        _tags,
        _gain_q8,
        b"R128_TRACK_GAIN\x00" as *const u8 as *const libc::c_char,
        15 as i32 as crate::stddef_h::size_t,
    );
}

unsafe extern "C" fn op_is_jpeg(mut _buf: *const u8, mut _buf_sz: crate::stddef_h::size_t) -> i32 {
    return (_buf_sz >= 11 as i32 as libc::c_ulong
        && crate::stdlib::memcmp(
            _buf as *const libc::c_void,
            b"\xff\xd8\xff\xe0\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as i32 as libc::c_ulong,
        ) == 0 as i32
        && (*_buf.offset(4 as i32 as isize) as i32) << 8 as i32
            | *_buf.offset(5 as i32 as isize) as i32
            >= 16 as i32
        && crate::stdlib::memcmp(
            _buf.offset(6 as i32 as isize) as *const libc::c_void,
            b"JFIF\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as i32 as libc::c_ulong,
        ) == 0 as i32) as i32;
}
/*Tries to extract the width, height, bits per pixel, and palette size of a
 JPEG.
On failure, simply leaves its outputs unmodified.*/

unsafe extern "C" fn op_extract_jpeg_params(
    mut _buf: *const u8,
    mut _buf_sz: crate::stddef_h::size_t,
    mut _width: *mut crate::opus_types_h::opus_uint32,
    mut _height: *mut crate::opus_types_h::opus_uint32,
    mut _depth: *mut crate::opus_types_h::opus_uint32,
    mut _colors: *mut crate::opus_types_h::opus_uint32,
    mut _has_palette: *mut i32,
) {
    if op_is_jpeg(_buf, _buf_sz) != 0 {
        let mut offs: crate::stddef_h::size_t = 0;
        offs = 2 as i32 as crate::stddef_h::size_t;
        loop {
            let mut segment_len: crate::stddef_h::size_t = 0;
            let mut marker: i32 = 0;
            while offs < _buf_sz && *_buf.offset(offs as isize) as i32 != 0xff as i32 {
                offs = offs.wrapping_add(1)
            }
            while offs < _buf_sz && *_buf.offset(offs as isize) as i32 == 0xff as i32 {
                offs = offs.wrapping_add(1)
            }
            marker = *_buf.offset(offs as isize) as i32;
            offs = offs.wrapping_add(1);
            /*If we hit EOI* (end of image), or another SOI* (start of image),
            or SOS (start of scan), then stop now.*/
            if offs >= _buf_sz || marker >= 0xd8 as i32 && marker <= 0xda as i32 {
                break;
            }
            /*RST* (restart markers): skip (no segment length).*/
            if marker >= 0xd0 as i32 && marker <= 0xd7 as i32 {
                continue;
            }
            /*Read the length of the marker segment.*/
            if _buf_sz.wrapping_sub(offs) < 2 as i32 as libc::c_ulong {
                break;
            }
            segment_len = ((*_buf.offset(offs as isize) as i32) << 8 as i32
                | *_buf.offset(offs.wrapping_add(1 as i32 as libc::c_ulong) as isize) as i32)
                as crate::stddef_h::size_t;
            if segment_len < 2 as i32 as libc::c_ulong || _buf_sz.wrapping_sub(offs) < segment_len {
                break;
            }
            if marker == 0xc0 as i32
                || marker > 0xc0 as i32 && marker < 0xd0 as i32 && marker & 3 as i32 != 0 as i32
            {
                /*Found a SOFn (start of frame) marker segment:*/
                if segment_len >= 8 as i32 as libc::c_ulong {
                    *_height = ((*_buf.offset(offs.wrapping_add(3 as i32 as libc::c_ulong) as isize)
                        as i32)
                        << 8 as i32
                        | *_buf.offset(offs.wrapping_add(4 as i32 as libc::c_ulong) as isize)
                            as i32)
                        as crate::opus_types_h::opus_uint32;
                    *_width = ((*_buf.offset(offs.wrapping_add(5 as i32 as libc::c_ulong) as isize)
                        as i32)
                        << 8 as i32
                        | *_buf.offset(offs.wrapping_add(6 as i32 as libc::c_ulong) as isize)
                            as i32)
                        as crate::opus_types_h::opus_uint32;
                    *_depth =
                        (*_buf.offset(offs.wrapping_add(2 as i32 as libc::c_ulong) as isize) as i32
                            * *_buf.offset(offs.wrapping_add(7 as i32 as libc::c_ulong) as isize)
                                as i32) as crate::opus_types_h::opus_uint32;
                    *_colors = 0 as i32 as crate::opus_types_h::opus_uint32;
                    *_has_palette = 0 as i32
                }
                break;
            } else {
                /*Other markers: skip the whole marker segment.*/
                offs = (offs as libc::c_ulong).wrapping_add(segment_len) as crate::stddef_h::size_t
            }
        }
    };
}

unsafe extern "C" fn op_is_png(mut _buf: *const u8, mut _buf_sz: crate::stddef_h::size_t) -> i32 {
    return (_buf_sz >= 8 as i32 as libc::c_ulong
        && crate::stdlib::memcmp(
            _buf as *const libc::c_void,
            b"\x89PNG\r\n\x1a\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as i32 as libc::c_ulong,
        ) == 0 as i32) as i32;
}
/*Tries to extract the width, height, bits per pixel, and palette size of a
 PNG.
On failure, simply leaves its outputs unmodified.*/

unsafe extern "C" fn op_extract_png_params(
    mut _buf: *const u8,
    mut _buf_sz: crate::stddef_h::size_t,
    mut _width: *mut crate::opus_types_h::opus_uint32,
    mut _height: *mut crate::opus_types_h::opus_uint32,
    mut _depth: *mut crate::opus_types_h::opus_uint32,
    mut _colors: *mut crate::opus_types_h::opus_uint32,
    mut _has_palette: *mut i32,
) {
    if op_is_png(_buf, _buf_sz) != 0 {
        let mut offs: crate::stddef_h::size_t = 0;
        offs = 8 as i32 as crate::stddef_h::size_t;
        while _buf_sz.wrapping_sub(offs) >= 12 as i32 as libc::c_ulong {
            let mut chunk_len: crate::config_types_h::ogg_uint32_t = 0;
            chunk_len = op_parse_uint32be(_buf.offset(offs as isize));
            if chunk_len as libc::c_ulong
                > _buf_sz.wrapping_sub(offs.wrapping_add(12 as i32 as libc::c_ulong))
            {
                break;
            }
            if chunk_len == 13 as i32 as u32
                && crate::stdlib::memcmp(
                    _buf.offset(offs as isize).offset(4 as i32 as isize) as *const libc::c_void,
                    b"IHDR\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4 as i32 as libc::c_ulong,
                ) == 0 as i32
            {
                let mut color_type: i32 = 0;
                *_width = op_parse_uint32be(_buf.offset(offs as isize).offset(8 as i32 as isize));
                *_height = op_parse_uint32be(_buf.offset(offs as isize).offset(12 as i32 as isize));
                color_type =
                    *_buf.offset(offs.wrapping_add(17 as i32 as libc::c_ulong) as isize) as i32;
                if color_type == 3 as i32 {
                    *_depth = 24 as i32 as crate::opus_types_h::opus_uint32;
                    *_has_palette = 1 as i32
                } else {
                    let mut sample_depth: i32 = 0;
                    sample_depth =
                        *_buf.offset(offs.wrapping_add(16 as i32 as libc::c_ulong) as isize) as i32;
                    if color_type == 0 as i32 {
                        *_depth = sample_depth as crate::opus_types_h::opus_uint32
                    } else if color_type == 2 as i32 {
                        *_depth = (sample_depth * 3 as i32) as crate::opus_types_h::opus_uint32
                    } else if color_type == 4 as i32 {
                        *_depth = (sample_depth * 2 as i32) as crate::opus_types_h::opus_uint32
                    } else if color_type == 6 as i32 {
                        *_depth = (sample_depth * 4 as i32) as crate::opus_types_h::opus_uint32
                    }
                    *_colors = 0 as i32 as crate::opus_types_h::opus_uint32;
                    *_has_palette = 0 as i32;
                    break;
                }
            } else if *_has_palette > 0 as i32
                && crate::stdlib::memcmp(
                    _buf.offset(offs as isize).offset(4 as i32 as isize) as *const libc::c_void,
                    b"PLTE\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4 as i32 as libc::c_ulong,
                ) == 0 as i32
            {
                *_colors = chunk_len.wrapping_div(3 as i32 as u32);
                break;
            }
            offs = (offs as libc::c_ulong)
                .wrapping_add((12 as i32 as u32).wrapping_add(chunk_len) as libc::c_ulong)
                as crate::stddef_h::size_t
        }
    };
}

unsafe extern "C" fn op_is_gif(mut _buf: *const u8, mut _buf_sz: crate::stddef_h::size_t) -> i32 {
    return (_buf_sz >= 6 as i32 as libc::c_ulong
        && (crate::stdlib::memcmp(
            _buf as *const libc::c_void,
            b"GIF87a\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as i32 as libc::c_ulong,
        ) == 0 as i32
            || crate::stdlib::memcmp(
                _buf as *const libc::c_void,
                b"GIF89a\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as i32 as libc::c_ulong,
            ) == 0 as i32)) as i32;
}
/*Tries to extract the width, height, bits per pixel, and palette size of a
 GIF.
On failure, simply leaves its outputs unmodified.*/

unsafe extern "C" fn op_extract_gif_params(
    mut _buf: *const u8,
    mut _buf_sz: crate::stddef_h::size_t,
    mut _width: *mut crate::opus_types_h::opus_uint32,
    mut _height: *mut crate::opus_types_h::opus_uint32,
    mut _depth: *mut crate::opus_types_h::opus_uint32,
    mut _colors: *mut crate::opus_types_h::opus_uint32,
    mut _has_palette: *mut i32,
) {
    if op_is_gif(_buf, _buf_sz) != 0 && _buf_sz >= 14 as i32 as libc::c_ulong {
        *_width = (*_buf.offset(6 as i32 as isize) as i32
            | (*_buf.offset(7 as i32 as isize) as i32) << 8 as i32)
            as crate::opus_types_h::opus_uint32;
        *_height = (*_buf.offset(8 as i32 as isize) as i32
            | (*_buf.offset(9 as i32 as isize) as i32) << 8 as i32)
            as crate::opus_types_h::opus_uint32;
        /*libFLAC hard-codes the depth to 24.*/
        *_depth = 24 as i32 as crate::opus_types_h::opus_uint32;
        *_colors = ((1 as i32) << (*_buf.offset(10 as i32 as isize) as i32 & 7 as i32) + 1 as i32)
            as crate::opus_types_h::opus_uint32;
        *_has_palette = 1 as i32
    };
}
/*The actual implementation of opus_picture_tag_parse().
Unlike the public API, this function requires _pic to already be
 initialized, modifies its contents before success is guaranteed, and assumes
 the caller will clear it on error.*/

unsafe extern "C" fn opus_picture_tag_parse_impl(
    mut _pic: *mut crate::src::opusfile_0_9::src::opusfile::OpusPictureTag,
    mut _tag: *const libc::c_char,
    mut _buf: *mut u8,
    mut _buf_sz: crate::stddef_h::size_t,
    mut _base64_sz: crate::stddef_h::size_t,
) -> i32 {
    let mut picture_type: crate::opus_types_h::opus_int32 = 0;
    let mut mime_type_length: crate::opus_types_h::opus_uint32 = 0;
    let mut mime_type: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut description_length: crate::opus_types_h::opus_uint32 = 0;
    let mut description: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: crate::opus_types_h::opus_uint32 = 0;
    let mut height: crate::opus_types_h::opus_uint32 = 0;
    let mut depth: crate::opus_types_h::opus_uint32 = 0;
    let mut colors: crate::opus_types_h::opus_uint32 = 0;
    let mut data_length: crate::opus_types_h::opus_uint32 = 0;
    let mut file_width: crate::opus_types_h::opus_uint32 = 0;
    let mut file_height: crate::opus_types_h::opus_uint32 = 0;
    let mut file_depth: crate::opus_types_h::opus_uint32 = 0;
    let mut file_colors: crate::opus_types_h::opus_uint32 = 0;
    let mut format: i32 = 0;
    let mut has_palette: i32 = 0;
    let mut colors_set: i32 = 0;
    let mut i: crate::stddef_h::size_t = 0;
    /*Decode the BASE64 data.*/
    i = 0 as i32 as crate::stddef_h::size_t;
    while i < _base64_sz {
        let mut value: crate::opus_types_h::opus_uint32 = 0;
        let mut j: i32 = 0;
        value = 0 as i32 as crate::opus_types_h::opus_uint32;
        j = 0 as i32;
        while j < 4 as i32 {
            let mut c: u32 = 0;
            let mut d: u32 = 0;
            c = *_tag.offset(
                (4 as i32 as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(j as libc::c_ulong) as isize,
            ) as u8 as u32;
            if c == '+' as i32 as u32 {
                d = 62 as i32 as u32
            } else if c == '/' as i32 as u32 {
                d = 63 as i32 as u32
            } else if c >= '0' as i32 as u32 && c <= '9' as i32 as u32 {
                d = (52 as i32 as u32)
                    .wrapping_add(c)
                    .wrapping_sub('0' as i32 as u32)
            } else if c >= 'a' as i32 as u32 && c <= 'z' as i32 as u32 {
                d = (26 as i32 as u32)
                    .wrapping_add(c)
                    .wrapping_sub('a' as i32 as u32)
            } else if c >= 'A' as i32 as u32 && c <= 'Z' as i32 as u32 {
                d = c.wrapping_sub('A' as i32 as u32)
            } else if c == '=' as i32 as u32
                && (3 as i32 as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(j as libc::c_ulong)
                    > _buf_sz
            {
                d = 0 as i32 as u32
            } else {
                return -(132 as i32);
            }
            value = value << 6 as i32 | d;
            j += 1
        }
        *_buf.offset((3 as i32 as libc::c_ulong).wrapping_mul(i) as isize) =
            (value >> 16 as i32) as u8;
        if (3 as i32 as libc::c_ulong)
            .wrapping_mul(i)
            .wrapping_add(1 as i32 as libc::c_ulong)
            < _buf_sz
        {
            *_buf.offset(
                (3 as i32 as libc::c_ulong)
                    .wrapping_mul(i)
                    .wrapping_add(1 as i32 as libc::c_ulong) as isize,
            ) = (value >> 8 as i32) as u8;
            if (3 as i32 as libc::c_ulong)
                .wrapping_mul(i)
                .wrapping_add(2 as i32 as libc::c_ulong)
                < _buf_sz
            {
                *_buf.offset(
                    (3 as i32 as libc::c_ulong)
                        .wrapping_mul(i)
                        .wrapping_add(2 as i32 as libc::c_ulong) as isize,
                ) = value as u8
            }
        }
        i = i.wrapping_add(1)
    }
    i = 0 as i32 as crate::stddef_h::size_t;
    picture_type = op_parse_uint32be(_buf.offset(i as isize)) as crate::opus_types_h::opus_int32;
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    /*Extract the MIME type.*/
    mime_type_length = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    if mime_type_length as libc::c_ulong > _buf_sz.wrapping_sub(32 as i32 as libc::c_ulong) {
        return -(132 as i32);
    }
    mime_type = crate::stdlib::malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(mime_type_length.wrapping_add(1 as i32 as u32) as libc::c_ulong),
    ) as *mut libc::c_char;
    if mime_type.is_null() {
        return -(129 as i32);
    }
    crate::stdlib::memcpy(
        mime_type as *mut libc::c_void,
        _buf.offset(i as isize) as *const libc::c_void,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(mime_type_length as libc::c_ulong),
    );
    *mime_type.offset(mime_type_length as isize) = '\u{0}' as i32 as libc::c_char;
    (*_pic).mime_type = mime_type;
    i = (i as libc::c_ulong).wrapping_add(mime_type_length as libc::c_ulong)
        as crate::stddef_h::size_t;
    /*Extract the description string.*/
    description_length = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    if description_length as libc::c_ulong
        > _buf_sz
            .wrapping_sub(mime_type_length as libc::c_ulong)
            .wrapping_sub(32 as i32 as libc::c_ulong)
    {
        return -(132 as i32);
    }
    description = crate::stdlib::malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(description_length.wrapping_add(1 as i32 as u32) as libc::c_ulong),
    ) as *mut libc::c_char;
    if description.is_null() {
        return -(129 as i32);
    }
    crate::stdlib::memcpy(
        description as *mut libc::c_void,
        _buf.offset(i as isize) as *const libc::c_void,
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(description_length as libc::c_ulong),
    );
    *description.offset(description_length as isize) = '\u{0}' as i32 as libc::c_char;
    (*_pic).description = description;
    i = (i as libc::c_ulong).wrapping_add(description_length as libc::c_ulong)
        as crate::stddef_h::size_t;
    /*Extract the remaining fields.*/
    width = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    height = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    depth = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    colors = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    /*If one of these is set, they all must be, but colors==0 is a valid value.*/
    colors_set = (width != 0 as i32 as u32
        || height != 0 as i32 as u32
        || depth != 0 as i32 as u32
        || colors != 0 as i32 as u32) as i32;
    if (width == 0 as i32 as u32 || height == 0 as i32 as u32 || depth == 0 as i32 as u32)
        && colors_set != 0
    {
        return -(132 as i32);
    }
    data_length = op_parse_uint32be(_buf.offset(i as isize));
    i = (i as libc::c_ulong).wrapping_add(4 as i32 as libc::c_ulong) as crate::stddef_h::size_t;
    if data_length as libc::c_ulong > _buf_sz.wrapping_sub(i) {
        return -(132 as i32);
    }
    /*Trim extraneous data so we don't copy it below.*/
    _buf_sz = i.wrapping_add(data_length as libc::c_ulong);
    /*Attempt to determine the image format.*/
    format = -(1 as i32);
    if mime_type_length == 3 as i32 as u32
        && ::libc::strcmp(mime_type, b"-->\x00" as *const u8 as *const libc::c_char) == 0 as i32
    {
        format = 0 as i32;
        /*Picture type 1 must be a 32x32 PNG.*/
        if picture_type == 1 as i32
            && (width != 0 as i32 as u32 || height != 0 as i32 as u32)
            && (width != 32 as i32 as u32 || height != 32 as i32 as u32)
        {
            return -(132 as i32);
        }
        /*Append a terminating NUL for the convenience of our callers.*/
        let fresh12 = _buf_sz;
        _buf_sz = _buf_sz.wrapping_add(1);
        *_buf.offset(fresh12 as isize) = '\u{0}' as i32 as u8
    } else {
        if mime_type_length == 10 as i32 as u32
            && crate::src::opusfile_0_9::src::internal::op_strncasecmp(
                mime_type,
                b"image/jpeg\x00" as *const u8 as *const libc::c_char,
                mime_type_length as i32,
            ) == 0 as i32
        {
            if op_is_jpeg(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 1 as i32
            }
        } else if mime_type_length == 9 as i32 as u32
            && crate::src::opusfile_0_9::src::internal::op_strncasecmp(
                mime_type,
                b"image/png\x00" as *const u8 as *const libc::c_char,
                mime_type_length as i32,
            ) == 0 as i32
        {
            if op_is_png(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 2 as i32
            }
        } else if mime_type_length == 9 as i32 as u32
            && crate::src::opusfile_0_9::src::internal::op_strncasecmp(
                mime_type,
                b"image/gif\x00" as *const u8 as *const libc::c_char,
                mime_type_length as i32,
            ) == 0 as i32
        {
            if op_is_gif(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 3 as i32
            }
        } else if mime_type_length == 0 as i32 as u32
            || mime_type_length == 6 as i32 as u32
                && crate::src::opusfile_0_9::src::internal::op_strncasecmp(
                    mime_type,
                    b"image/\x00" as *const u8 as *const libc::c_char,
                    mime_type_length as i32,
                ) == 0 as i32
        {
            if op_is_jpeg(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 1 as i32
            } else if op_is_png(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 2 as i32
            } else if op_is_gif(
                _buf.offset(i as isize),
                data_length as crate::stddef_h::size_t,
            ) != 0
            {
                format = 3 as i32
            }
        }
        file_colors = 0 as i32 as crate::opus_types_h::opus_uint32;
        file_depth = file_colors;
        file_height = file_depth;
        file_width = file_height;
        has_palette = -(1 as i32);
        match format {
            1 => {
                op_extract_jpeg_params(
                    _buf.offset(i as isize),
                    data_length as crate::stddef_h::size_t,
                    &mut file_width,
                    &mut file_height,
                    &mut file_depth,
                    &mut file_colors,
                    &mut has_palette,
                );
            }
            2 => {
                op_extract_png_params(
                    _buf.offset(i as isize),
                    data_length as crate::stddef_h::size_t,
                    &mut file_width,
                    &mut file_height,
                    &mut file_depth,
                    &mut file_colors,
                    &mut has_palette,
                );
            }
            3 => {
                op_extract_gif_params(
                    _buf.offset(i as isize),
                    data_length as crate::stddef_h::size_t,
                    &mut file_width,
                    &mut file_height,
                    &mut file_depth,
                    &mut file_colors,
                    &mut has_palette,
                );
            }
            _ => {}
        }
        if has_palette >= 0 as i32 {
            /*If we successfully extracted these parameters from the image, override
            any declared values.*/
            width = file_width;
            height = file_height;
            depth = file_depth;
            colors = file_colors
        }
        /*Picture type 1 must be a 32x32 PNG.*/
        if picture_type == 1 as i32
            && (format != 2 as i32 || width != 32 as i32 as u32 || height != 32 as i32 as u32)
        {
            return -(132 as i32);
        }
    }
    /*Adjust _buf_sz instead of using data_length to capture the terminating NUL
    for URLs.*/
    _buf_sz = (_buf_sz as libc::c_ulong).wrapping_sub(i) as crate::stddef_h::size_t;
    crate::stdlib::memmove(
        _buf as *mut libc::c_void,
        _buf.offset(i as isize) as *const libc::c_void,
        (::std::mem::size_of::<u8>() as libc::c_ulong).wrapping_mul(_buf_sz),
    );
    _buf = crate::stdlib::realloc(_buf as *mut libc::c_void, _buf_sz) as *mut u8;
    if _buf_sz > 0 as i32 as libc::c_ulong && _buf.is_null() {
        return -(129 as i32);
    }
    (*_pic).type_0 = picture_type;
    (*_pic).width = width;
    (*_pic).height = height;
    (*_pic).depth = depth;
    (*_pic).colors = colors;
    (*_pic).data_length = data_length;
    (*_pic).data = _buf;
    (*_pic).format = format;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn opus_picture_tag_parse(
    mut _pic: *mut crate::src::opusfile_0_9::src::opusfile::OpusPictureTag,
    mut _tag: *const libc::c_char,
) -> i32 {
    let mut pic: crate::src::opusfile_0_9::src::opusfile::OpusPictureTag =
        crate::src::opusfile_0_9::src::opusfile::OpusPictureTag {
            type_0: 0,
            mime_type: 0 as *mut libc::c_char,
            description: 0 as *mut libc::c_char,
            width: 0,
            height: 0,
            depth: 0,
            colors: 0,
            data_length: 0,
            data: 0 as *mut u8,
            format: 0,
        };
    let mut buf: *mut u8 = 0 as *mut u8;
    let mut base64_sz: crate::stddef_h::size_t = 0;
    let mut buf_sz: crate::stddef_h::size_t = 0;
    let mut tag_length: crate::stddef_h::size_t = 0;
    let mut ret: i32 = 0;
    if opus_tagncompare(
        b"METADATA_BLOCK_PICTURE\x00" as *const u8 as *const libc::c_char,
        22 as i32,
        _tag,
    ) == 0 as i32
    {
        _tag = _tag.offset(23 as i32 as isize)
    }
    /*Figure out how much BASE64-encoded data we have.*/
    tag_length = crate::stdlib::strlen(_tag);
    if tag_length & 3 as i32 as libc::c_ulong != 0 {
        return -(132 as i32);
    }
    base64_sz = tag_length >> 2 as i32;
    buf_sz = (3 as i32 as libc::c_ulong).wrapping_mul(base64_sz);
    if buf_sz < 32 as i32 as libc::c_ulong {
        return -(132 as i32);
    }
    if *_tag.offset(tag_length.wrapping_sub(1 as i32 as libc::c_ulong) as isize) as i32
        == '=' as i32
    {
        buf_sz = buf_sz.wrapping_sub(1)
    }
    if *_tag.offset(tag_length.wrapping_sub(2 as i32 as libc::c_ulong) as isize) as i32
        == '=' as i32
    {
        buf_sz = buf_sz.wrapping_sub(1)
    }
    if buf_sz < 32 as i32 as libc::c_ulong {
        return -(132 as i32);
    }
    /*Allocate an extra byte to allow appending a terminating NUL to URL data.*/
    buf = crate::stdlib::malloc(
        (::std::mem::size_of::<u8>() as libc::c_ulong)
            .wrapping_mul(buf_sz.wrapping_add(1 as i32 as libc::c_ulong)),
    ) as *mut u8;
    if buf.is_null() {
        return -(129 as i32);
    }
    opus_picture_tag_init(&mut pic);
    ret = opus_picture_tag_parse_impl(&mut pic, _tag, buf, buf_sz, base64_sz);
    if ret < 0 as i32 {
        opus_picture_tag_clear(&mut pic);
        ::libc::free(buf as *mut libc::c_void);
    } else {
        *_pic = pic
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn opus_picture_tag_init(
    mut _pic: *mut crate::src::opusfile_0_9::src::opusfile::OpusPictureTag,
) {
    crate::stdlib::memset(
        _pic as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<crate::src::opusfile_0_9::src::opusfile::OpusPictureTag>()
            as libc::c_ulong,
    );
}
/* *******************************************************************
*                                                                  *
* THIS FILE IS PART OF THE libopusfile SOFTWARE CODEC SOURCE CODE. *
* USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
* GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
* IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
*                                                                  *
* THE libopusfile SOURCE CODE IS (C) COPYRIGHT 1994-2012           *
* by the Xiph.Org Foundation and contributors http://www.xiph.org/ *
*                                                                  *
********************************************************************

function: stdio-based convenience library for opening/seeking/decoding
last mod: $Id: vorbisfile.h 17182 2010-04-29 03:48:32Z xiphmont $

********************************************************************/
/* *\mainpage
\section Introduction

This is the documentation for the <tt>libopusfile</tt> C API.

The <tt>libopusfile</tt> package provides a convenient high-level API for
 decoding and basic manipulation of all Ogg Opus audio streams.
<tt>libopusfile</tt> is implemented as a layer on top of Xiph.Org's
 reference
 <tt><a href="https://www.xiph.org/ogg/doc/libogg/reference.html">libogg</a></tt>
 and
 <tt><a href="https://mf4.xiph.org/jenkins/view/opus/job/opus/ws/doc/html/index.html">libopus</a></tt>
 libraries.

<tt>libopusfile</tt> provides several sets of built-in routines for
 file/stream access, and may also use custom stream I/O routines provided by
 the embedded environment.
There are built-in I/O routines provided for ANSI-compliant
 <code>stdio</code> (<code>FILE *</code>), memory buffers, and URLs
 (including <file:> URLs, plus optionally <http:> and <https:> URLs).

\section Organization

The main API is divided into several sections:
- \ref stream_open_close
- \ref stream_info
- \ref stream_decoding
- \ref stream_seeking

Several additional sections are not tied to the main API.
- \ref stream_callbacks
- \ref header_info
- \ref error_codes

\section Overview

The <tt>libopusfile</tt> API always decodes files to 48&nbsp;kHz.
The original sample rate is not preserved by the lossy compression, though
 it is stored in the header to allow you to resample to it after decoding
 (the <tt>libopusfile</tt> API does not currently provide a resampler,
 but the
 <a href="http://www.speex.org/docs/manual/speex-manual/node7.html#SECTION00760000000000000000">the
 Speex resampler</a> is a good choice if you need one).
In general, if you are playing back the audio, you should leave it at
 48&nbsp;kHz, provided your audio hardware supports it.
When decoding to a file, it may be worth resampling back to the original
 sample rate, so as not to surprise users who might not expect the sample
 rate to change after encoding to Opus and decoding.

Opus files can contain anywhere from 1 to 255 channels of audio.
The channel mappings for up to 8 channels are the same as the
 <a href="http://www.xiph.org/vorbis/doc/Vorbis_I_spec.html#x1-800004.3.9">Vorbis
 mappings</a>.
A special stereo API can convert everything to 2 channels, making it simple
 to support multichannel files in an application which only has stereo
 output.
Although the <tt>libopusfile</tt> ABI provides support for the theoretical
 maximum number of channels, the current implementation does not support
 files with more than 8 channels, as they do not have well-defined channel
 mappings.

Like all Ogg files, Opus files may be "chained".
That is, multiple Opus files may be combined into a single, longer file just
 by concatenating the original files.
This is commonly done in internet radio streaming, as it allows the title
 and artist to be updated each time the song changes, since each link in the
 chain includes its own set of metadata.

<tt>libopusfile</tt> fully supports chained files.
It will decode the first Opus stream found in each link of a chained file
 (ignoring any other streams that might be concurrently multiplexed with it,
 such as a video stream).

The channel count can also change between links.
If your application is not prepared to deal with this, it can use the stereo
 API to ensure the audio from all links will always get decoded into a
 common format.
Since <tt>libopusfile</tt> always decodes to 48&nbsp;kHz, you do not have to
 worry about the sample rate changing between links (as was possible with
 Vorbis).
This makes application support for chained files with <tt>libopusfile</tt>
 very easy.*/
/* *@cond PRIVATE*/
/*Enable special features for gcc and gcc-compatible compilers.*/
/*Warning attributes for libopusfile functions.*/
/* *@endcond*/
/* *\defgroup error_codes Error Codes*/
/*@{*/
/* *\name List of possible error codes
Many of the functions in this library return a negative error code when a
 function fails.
This list provides a brief explanation of the common errors.
See each individual function for more details on what a specific error code
 means in that context.*/
/*@{*/
/* *A request did not succeed.*/
/*Currently not used externally.*/
/* *There was a hole in the page sequence numbers (e.g., a page was corrupt or
missing).*/
/* *An underlying read, seek, or tell operation failed when it should have
succeeded.*/
/* *A <code>NULL</code> pointer was passed where one was unexpected, or an
internal memory allocation failed, or an internal library error was
encountered.*/
/* *The stream used a feature that is not implemented, such as an unsupported
channel family.*/
/* *One or more parameters to a function were invalid.*/
/* *A purported Ogg Opus stream did not begin with an Ogg page, a purported
header packet did not start with one of the required strings, "OpusHead" or
"OpusTags", or a link in a chained file was encountered that did not
contain any logical Opus streams.*/
/* *A required header packet was not properly formatted, contained illegal
values, or was missing altogether.*/
/* *The ID header contained an unrecognized version number.*/
/*Currently not used at all.*/
/* *An audio packet failed to decode properly.
This is usually caused by a multistream Ogg packet where the durations of
 the individual Opus packets contained in it are not all the same.*/
/* *We failed to find data we had seen before, or the bitstream structure was
sufficiently malformed that seeking to the target destination was
impossible.*/
/* *An operation that requires seeking was requested on an unseekable stream.*/
/* *The first or last granule position of a link failed basic validity checks.*/
/*@}*/
/*@}*/
/* *\defgroup header_info Header Information*/
/*@{*/
/* *The maximum number of channels in an Ogg Opus stream.*/
/* *Ogg Opus bitstream information.
This contains the basic playback parameters for a stream, and corresponds to
 the initial ID header packet of an Ogg Opus stream.*/
/* *The Ogg Opus format version, in the range 0...255.
The top 4 bits represent a "major" version, and the bottom four bits
 represent backwards-compatible "minor" revisions.
The current specification describes version 1.
This library will recognize versions up through 15 as backwards compatible
 with the current specification.
An earlier draft of the specification described a version 0, but the only
 difference between version 1 and version 0 is that version 0 did
 not specify the semantics for handling the version field.*/
/* *The number of channels, in the range 1...255.*/
/* *The number of samples that should be discarded from the beginning of the
stream.*/
/* *The sampling rate of the original input.
All Opus audio is coded at 48 kHz, and should also be decoded at 48 kHz
 for playback (unless the target hardware does not support this sampling
 rate).
However, this field may be used to resample the audio back to the original
 sampling rate, for example, when saving the output to a file.*/
/* *The gain to apply to the decoded output, in dB, as a Q8 value in the range
 -32768...32767.
The <tt>libopusfile</tt> API will automatically apply this gain to the
 decoded output before returning it, scaling it by
 <code>pow(10,output_gain/(20.0*256))</code>.
You can adjust this behavior with op_set_gain_offset().*/
/* *The channel mapping family, in the range 0...255.
Channel mapping family 0 covers mono or stereo in a single stream.
Channel mapping family 1 covers 1 to 8 channels in one or more streams,
 using the Vorbis speaker assignments.
Channel mapping family 255 covers 1 to 255 channels in one or more
 streams, but without any defined speaker assignment.*/
/* *The number of Opus streams in each Ogg packet, in the range 1...255.*/
/* *The number of coupled Opus streams in each Ogg packet, in the range
 0...127.
This must satisfy <code>0 <= coupled_count <= stream_count</code> and
 <code>coupled_count + stream_count <= 255</code>.
The coupled streams appear first, before all uncoupled streams, in an Ogg
 Opus packet.*/
/* *The mapping from coded stream channels to output channels.
Let <code>index=mapping[k]</code> be the value for channel <code>k</code>.
If <code>index<2*coupled_count</code>, then it refers to the left channel
 from stream <code>(index/2)</code> if even, and the right channel from
 stream <code>(index/2)</code> if odd.
Otherwise, it refers to the output of the uncoupled stream
 <code>(index-coupled_count)</code>.*/
/* *The metadata from an Ogg Opus stream.

This structure holds the in-stream metadata corresponding to the 'comment'
 header packet of an Ogg Opus stream.
The comment header is meant to be used much like someone jotting a quick
 note on the label of a CD.
It should be a short, to the point text note that can be more than a couple
 words, but not more than a short paragraph.

The metadata is stored as a series of (tag, value) pairs, in length-encoded
 string vectors, using the same format as Vorbis (without the final "framing
 bit"), Theora, and Speex, except for the packet header.
The first occurrence of the '=' character delimits the tag and value.
A particular tag may occur more than once, and order is significant.
The character set encoding for the strings is always UTF-8, but the tag
 names are limited to ASCII, and treated as case-insensitive.
See <a href="http://www.xiph.org/vorbis/doc/v-comment.html">the Vorbis
 comment header specification</a> for details.

In filling in this structure, <tt>libopusfile</tt> will null-terminate the
 #user_comments strings for safety.
However, the bitstream format itself treats them as 8-bit clean vectors,
 possibly containing NUL characters, so the #comment_lengths array should be
 treated as their authoritative length.

This structure is binary and source-compatible with a
 <code>vorbis_comment</code>, and pointers to it may be freely cast to
 <code>vorbis_comment</code> pointers, and vice versa.
It is provided as a separate type to avoid introducing a compile-time
 dependency on the libvorbis headers.*/
/* *The array of comment string vectors.*/
/* *An array of the corresponding length of each vector, in bytes.*/
/* *The total number of comment streams.*/
/* *The null-terminated vendor string.
This identifies the software used to encode the stream.*/
/* *\name Picture tag image formats*/
/*@{*/
/* *The MIME type was not recognized, or the image data did not match the
declared MIME type.*/
/* *The MIME type indicates the image data is really a URL.*/
/* *The image is a JPEG.*/
/* *The image is a PNG.*/
/* *The image is a GIF.*/
/*@}*/
/* *The contents of a METADATA_BLOCK_PICTURE tag.*/
/* *The picture type according to the ID3v2 APIC frame:
<ol start="0">
<li>Other</li>
<li>32x32 pixels 'file icon' (PNG only)</li>
<li>Other file icon</li>
<li>Cover (front)</li>
<li>Cover (back)</li>
<li>Leaflet page</li>
<li>Media (e.g. label side of CD)</li>
<li>Lead artist/lead performer/soloist</li>
<li>Artist/performer</li>
<li>Conductor</li>
<li>Band/Orchestra</li>
<li>Composer</li>
<li>Lyricist/text writer</li>
<li>Recording Location</li>
<li>During recording</li>
<li>During performance</li>
<li>Movie/video screen capture</li>
<li>A bright colored fish</li>
<li>Illustration</li>
<li>Band/artist logotype</li>
<li>Publisher/Studio logotype</li>
</ol>
Others are reserved and should not be used.
There may only be one each of picture type 1 and 2 in a file.*/
/* *The MIME type of the picture, in printable ASCII characters 0x20-0x7E.
The MIME type may also be <code>"-->"</code> to signify that the data part
 is a URL pointing to the picture instead of the picture data itself.
In this case, a terminating NUL is appended to the URL string in #data,
 but #data_length is set to the length of the string excluding that
 terminating NUL.*/
/* *The description of the picture, in UTF-8.*/
/* *The width of the picture in pixels.*/
/* *The height of the picture in pixels.*/
/* *The color depth of the picture in bits-per-pixel (<em>not</em>
bits-per-channel).*/
/* *For indexed-color pictures (e.g., GIF), the number of colors used, or 0
for non-indexed pictures.*/
/* *The length of the picture data in bytes.*/
/* *The binary picture data.*/
/* *The format of the picture data, if known.
One of
<ul>
<li>#OP_PIC_FORMAT_UNKNOWN,</li>
<li>#OP_PIC_FORMAT_URL,</li>
<li>#OP_PIC_FORMAT_JPEG,</li>
<li>#OP_PIC_FORMAT_PNG, or</li>
<li>#OP_PIC_FORMAT_GIF.</li>
</ul>*/
/* *\name Functions for manipulating header data

These functions manipulate the #OpusHead and #OpusTags structures,
 which describe the audio parameters and tag-value metadata, respectively.
These can be used to query the headers returned by <tt>libopusfile</tt>, or
 to parse Opus headers from sources other than an Ogg Opus stream, provided
 they use the same format.*/
/*@{*/
/* *Parses the contents of the ID header packet of an Ogg Opus stream.
\param[out] _head Returns the contents of the parsed packet.
                  The contents of this structure are untouched on error.
                  This may be <code>NULL</code> to merely test the header
                   for validity.
\param[in]  _data The contents of the ID header packet.
\param      _len  The number of bytes of data in the ID header packet.
\return 0 on success or a negative value on error.
\retval #OP_ENOTFORMAT If the data does not start with the "OpusHead"
                        string.
\retval #OP_EVERSION   If the version field signaled a version this library
                        does not know how to parse.
\retval #OP_EIMPL      If the channel mapping family was 255, which general
                        purpose players should not attempt to play.
\retval #OP_EBADHEADER If the contents of the packet otherwise violate the
                        Ogg Opus specification:
                       <ul>
                        <li>Insufficient data,</li>
                        <li>Too much data for the known minor versions,</li>
                        <li>An unrecognized channel mapping family,</li>
                        <li>Zero channels or too many channels,</li>
                        <li>Zero coded streams,</li>
                        <li>Too many coupled streams, or</li>
                        <li>An invalid channel mapping index.</li>
                       </ul>*/
/* *Converts a granule position to a sample offset for a given Ogg Opus stream.
The sample offset is simply <code>_gp-_head->pre_skip</code>.
Granule position values smaller than OpusHead#pre_skip correspond to audio
 that should never be played, and thus have no associated sample offset.
This function returns -1 for such values.
This function also correctly handles extremely large granule positions,
 which may have wrapped around to a negative number when stored in a signed
 ogg_int64_t value.
\param _head The #OpusHead information from the ID header of the stream.
\param _gp   The granule position to convert.
\return The sample offset associated with the given granule position
         (counting at a 48 kHz sampling rate), or the special value -1 on
         error (i.e., the granule position was smaller than the pre-skip
         amount).*/
/* *Parses the contents of the 'comment' header packet of an Ogg Opus stream.
\param[out] _tags An uninitialized #OpusTags structure.
                  This returns the contents of the parsed packet.
                  The contents of this structure are untouched on error.
                  This may be <code>NULL</code> to merely test the header
                   for validity.
\param[in]  _data The contents of the 'comment' header packet.
\param      _len  The number of bytes of data in the 'info' header packet.
\retval 0              Success.
\retval #OP_ENOTFORMAT If the data does not start with the "OpusTags"
                        string.
\retval #OP_EBADHEADER If the contents of the packet otherwise violate the
                        Ogg Opus specification.
\retval #OP_EFAULT     If there wasn't enough memory to store the tags.*/
/* *Performs a deep copy of an #OpusTags structure.
\param _dst The #OpusTags structure to copy into.
            If this function fails, the contents of this structure remain
             untouched.
\param _src The #OpusTags structure to copy from.
\retval 0          Success.
\retval #OP_EFAULT If there wasn't enough memory to copy the tags.*/
/* *Initializes an #OpusTags structure.
This should be called on a freshly allocated #OpusTags structure before
 attempting to use it.
\param _tags The #OpusTags structure to initialize.*/
/* *Add a (tag, value) pair to an initialized #OpusTags structure.
\note Neither opus_tags_add() nor opus_tags_add_comment() support values
 containing embedded NULs, although the bitstream format does support them.
To add such tags, you will need to manipulate the #OpusTags structure
 directly.
\param _tags  The #OpusTags structure to add the (tag, value) pair to.
\param _tag   A NUL-terminated, case-insensitive, ASCII string containing
               the tag to add (without an '=' character).
\param _value A NUL-terminated UTF-8 containing the corresponding value.
\return 0 on success, or a negative value on failure.
\retval #OP_EFAULT An internal memory allocation failed.*/
/* *Add a comment to an initialized #OpusTags structure.
\note Neither opus_tags_add_comment() nor opus_tags_add() support comments
 containing embedded NULs, although the bitstream format does support them.
To add such tags, you will need to manipulate the #OpusTags structure
 directly.
\param _tags    The #OpusTags structure to add the comment to.
\param _comment A NUL-terminated UTF-8 string containing the comment in
                 "TAG=value" form.
\return 0 on success, or a negative value on failure.
\retval #OP_EFAULT An internal memory allocation failed.*/
/* *Replace the binary suffix data at the end of the packet (if any).
\param _tags An initialized #OpusTags structure.
\param _data A buffer of binary data to append after the encoded user
              comments.
             The least significant bit of the first byte of this data must
              be set (to ensure the data is preserved by other editors).
\param _len  The number of bytes of binary data to append.
             This may be zero to remove any existing binary suffix data.
\return 0 on success, or a negative value on error.
\retval #OP_EINVAL \a _len was negative, or \a _len was positive but
                    \a _data was <code>NULL</code> or the least significant
                    bit of the first byte was not set.
\retval #OP_EFAULT An internal memory allocation failed.*/
/* *Look up a comment value by its tag.
\param _tags  An initialized #OpusTags structure.
\param _tag   The tag to look up.
\param _count The instance of the tag.
              The same tag can appear multiple times, each with a distinct
               value, so an index is required to retrieve them all.
              The order in which these values appear is significant and
               should be preserved.
              Use opus_tags_query_count() to get the legal range for the
               \a _count parameter.
\return A pointer to the queried tag's value.
        This points directly to data in the #OpusTags structure.
        It should not be modified or freed by the application, and
         modifications to the structure may invalidate the pointer.
\retval NULL If no matching tag is found.*/
/* *Look up the number of instances of a tag.
Call this first when querying for a specific tag and then iterate over the
 number of instances with separate calls to opus_tags_query() to retrieve
 all the values for that tag in order.
\param _tags An initialized #OpusTags structure.
\param _tag  The tag to look up.
\return The number of instances of this particular tag.*/
/* *Retrieve the binary suffix data at the end of the packet (if any).
\param      _tags An initialized #OpusTags structure.
\param[out] _len  Returns the number of bytes of binary suffix data returned.
\return A pointer to the binary suffix data, or <code>NULL</code> if none
         was present.*/
/* *Get the album gain from an R128_ALBUM_GAIN tag, if one was specified.
This searches for the first R128_ALBUM_GAIN tag with a valid signed,
 16-bit decimal integer value and returns the value.
This routine is exposed merely for convenience for applications which wish
 to do something special with the album gain (i.e., display it).
If you simply wish to apply the album gain instead of the header gain, you
 can use op_set_gain_offset() with an #OP_ALBUM_GAIN type and no offset.
\param      _tags    An initialized #OpusTags structure.
\param[out] _gain_q8 The album gain, in 1/256ths of a dB.
                     This will lie in the range [-32768,32767], and should
                      be applied in <em>addition</em> to the header gain.
                     On error, no value is returned, and the previous
                      contents remain unchanged.
\return 0 on success, or a negative value on error.
\retval #OP_FALSE There was no album gain available in the given tags.*/
/* *Get the track gain from an R128_TRACK_GAIN tag, if one was specified.
This searches for the first R128_TRACK_GAIN tag with a valid signed,
 16-bit decimal integer value and returns the value.
This routine is exposed merely for convenience for applications which wish
 to do something special with the track gain (i.e., display it).
If you simply wish to apply the track gain instead of the header gain, you
 can use op_set_gain_offset() with an #OP_TRACK_GAIN type and no offset.
\param      _tags    An initialized #OpusTags structure.
\param[out] _gain_q8 The track gain, in 1/256ths of a dB.
                     This will lie in the range [-32768,32767], and should
                      be applied in <em>addition</em> to the header gain.
                     On error, no value is returned, and the previous
                      contents remain unchanged.
\return 0 on success, or a negative value on error.
\retval #OP_FALSE There was no track gain available in the given tags.*/
/* *Clears the #OpusTags structure.
This should be called on an #OpusTags structure after it is no longer
 needed.
It will free all memory used by the structure members.
\param _tags The #OpusTags structure to clear.*/
/* *Check if \a _comment is an instance of a \a _tag_name tag.
\see opus_tagncompare
\param _tag_name A NUL-terminated, case-insensitive, ASCII string containing
                  the name of the tag to check for (without the terminating
                  '=' character).
\param _comment  The comment string to check.
\return An integer less than, equal to, or greater than zero if \a _comment
         is found respectively, to be less than, to match, or be greater
         than a "tag=value" string whose tag matches \a _tag_name.*/
/* *Check if \a _comment is an instance of a \a _tag_name tag.
This version is slightly more efficient than opus_tagcompare() if the length
 of the tag name is already known (e.g., because it is a constant).
\see opus_tagcompare
\param _tag_name A case-insensitive ASCII string containing the name of the
                  tag to check for (without the terminating '=' character).
\param _tag_len  The number of characters in the tag name.
                 This must be non-negative.
\param _comment  The comment string to check.
\return An integer less than, equal to, or greater than zero if \a _comment
         is found respectively, to be less than, to match, or be greater
         than a "tag=value" string whose tag matches the first \a _tag_len
         characters of \a _tag_name.*/
/* *Parse a single METADATA_BLOCK_PICTURE tag.
This decodes the BASE64-encoded content of the tag and returns a structure
 with the MIME type, description, image parameters (if known), and the
 compressed image data.
If the MIME type indicates the presence of an image format we recognize
 (JPEG, PNG, or GIF) and the actual image data contains the magic signature
 associated with that format, then the OpusPictureTag::format field will be
 set to the corresponding format.
This is provided as a convenience to avoid requiring applications to parse
 the MIME type and/or do their own format detection for the commonly used
 formats.
In this case, we also attempt to extract the image parameters directly from
 the image data (overriding any that were present in the tag, which the
 specification says applications are not meant to rely on).
The application must still provide its own support for actually decoding the
 image data and, if applicable, retrieving that data from URLs.
\param[out] _pic Returns the parsed picture data.
                 No sanitation is done on the type, MIME type, or
                  description fields, so these might return invalid values.
                 The contents of this structure are left unmodified on
                  failure.
\param      _tag The METADATA_BLOCK_PICTURE tag contents.
                 The leading "METADATA_BLOCK_PICTURE=" portion is optional,
                  to allow the function to be used on either directly on the
                  values in OpusTags::user_comments or on the return value
                  of opus_tags_query().
\return 0 on success or a negative value on error.
\retval #OP_ENOTFORMAT The METADATA_BLOCK_PICTURE contents were not valid.
\retval #OP_EFAULT     There was not enough memory to store the picture tag
                        contents.*/
/* *Initializes an #OpusPictureTag structure.
This should be called on a freshly allocated #OpusPictureTag structure
 before attempting to use it.
\param _pic The #OpusPictureTag structure to initialize.*/
/* *Clears the #OpusPictureTag structure.
This should be called on an #OpusPictureTag structure after it is no longer
 needed.
It will free all memory used by the structure members.
\param _pic The #OpusPictureTag structure to clear.*/
#[no_mangle]

pub unsafe extern "C" fn opus_picture_tag_clear(
    mut _pic: *mut crate::src::opusfile_0_9::src::opusfile::OpusPictureTag,
) {
    ::libc::free((*_pic).description as *mut libc::c_void);
    ::libc::free((*_pic).mime_type as *mut libc::c_void);
    ::libc::free((*_pic).data as *mut libc::c_void);
}

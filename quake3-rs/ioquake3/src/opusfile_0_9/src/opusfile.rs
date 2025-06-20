// =============== BEGIN opusfile_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusHead {
    pub version: i32,
    pub channel_count: i32,
    pub pre_skip: u32,
    pub input_sample_rate: crate::opus_types_h::opus_uint32,
    pub output_gain: i32,
    pub mapping_family: i32,
    pub stream_count: i32,
    pub coupled_count: i32,
    pub mapping: [u8; 255],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusTags {
    pub user_comments: *mut *mut libc::c_char,
    pub comment_lengths: *mut i32,
    pub comments: i32,
    pub vendor: *mut libc::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusPictureTag {
    pub type_0: crate::opus_types_h::opus_int32,
    pub mime_type: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub width: crate::opus_types_h::opus_uint32,
    pub height: crate::opus_types_h::opus_uint32,
    pub depth: crate::opus_types_h::opus_uint32,
    pub colors: crate::opus_types_h::opus_uint32,
    pub data_length: crate::opus_types_h::opus_uint32,
    pub data: *mut u8,
    pub format: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusServerInfo {
    pub name: *mut libc::c_char,
    pub description: *mut libc::c_char,
    pub genre: *mut libc::c_char,
    pub url: *mut libc::c_char,
    pub server: *mut libc::c_char,
    pub content_type: *mut libc::c_char,
    pub bitrate_kbps: crate::opus_types_h::opus_int32,
    pub is_public: i32,
    pub is_ssl: i32,
}

pub type op_read_func =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut u8, _: i32) -> i32>;

pub type op_seek_func = Option<unsafe extern "C" fn(_: *mut libc::c_void, _: i64, _: i32) -> i32>;

pub type op_tell_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> i64>;

pub type op_close_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> i32>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusFileCallbacks {
    pub read: crate::src::opusfile_0_9::src::opusfile::op_read_func,
    pub seek: crate::src::opusfile_0_9::src::opusfile::op_seek_func,
    pub tell: crate::src::opusfile_0_9::src::opusfile::op_tell_func,
    pub close: crate::src::opusfile_0_9::src::opusfile::op_close_func,
}

pub type op_decode_cb_func = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *mut crate::src::opus_1_2_1::src::opus_multistream_decoder::OpusMSDecoder,
        _: *mut libc::c_void,
        _: *const crate::ogg_h::ogg_packet,
        _: i32,
        _: i32,
        _: i32,
        _: i32,
    ) -> i32,
>;
use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::config_types_h::ogg_int32_t;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::config_types_h::ogg_uint32_t;
pub use crate::internal_h::op_sample;
pub use crate::internal_h::OggOpusFile;
pub use crate::internal_h::OggOpusLink;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::ogg_page;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_int32;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_bos;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_packets;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_serialno;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_clear;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_init;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_reset;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_buffer;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_clear;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_init;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_pageout;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_pageseek;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_reset;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_wrote;

pub use crate::src::opusfile_0_9::src::info::opus_head_parse;
pub use crate::src::opusfile_0_9::src::info::opus_tags_clear;
pub use crate::src::opusfile_0_9::src::info::opus_tags_get_album_gain;
pub use crate::src::opusfile_0_9::src::info::opus_tags_get_track_gain;
pub use crate::src::opusfile_0_9::src::info::opus_tags_parse;
pub use crate::src::opusfile_0_9::src::stream::op_fopen;
pub use crate::src::opusfile_0_9::src::stream::op_mem_stream_create;

/*We use this to remember the pages we found while enumerating the links of a
 chained stream.
We keep track of the starting and ending offsets, as well as the point we
 started searching from, so we know where to bisect.
We also keep the serial number, so we can tell if the page belonged to the
 current link or not, as well as the granule position, to aid in estimating
 the start of the link.*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusSeekRecord {
    pub search_start: i64,
    pub offset: i64,
    pub size: opus_int32,
    pub serialno: ogg_uint32_t,
    pub gp: ogg_int64_t,
}
/*A generic filter to apply to the decoded audio data.
_src is non-const because we will destructively modify the contents of the
 source buffer that we consume in some cases.*/

pub type op_read_filter_func = Option<
    unsafe extern "C" fn(
        _: *mut OggOpusFile,
        _: *mut libc::c_void,
        _: i32,
        _: *mut op_sample,
        _: i32,
        _: i32,
    ) -> i32,
>;
#[no_mangle]

pub unsafe extern "C" fn op_test(
    mut _head: *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
    mut _initial_data: *const u8,
    mut _initial_bytes: size_t,
) -> i32 {
    let mut oy: ogg_sync_state = ogg_sync_state {
        data: std::ptr::null_mut(),
        storage: 0,
        fill: 0,
        returned: 0,
        unsynced: 0,
        headerbytes: 0,
        bodybytes: 0,
    };
    let mut data: *mut libc::c_char = std::ptr::null_mut();
    let mut err: i32 = 0;
    /*The first page of a normal Opus file will be at most 57 bytes (27 Ogg
     page header bytes + 1 lacing value + 21 Opus header bytes + 8 channel
     mapping bytes).
    It will be at least 47 bytes (27 Ogg page header bytes + 1 lacing value +
     19 Opus header bytes using channel mapping family 0).
    If we don't have at least that much data, give up now.*/
    if _initial_bytes < 47 as i32 as usize {
        return -(1 as i32);
    }
    /*Only proceed if we start with the magic OggS string.
    This is to prevent us spending a lot of time allocating memory and looking
     for Ogg pages in non-Ogg files.*/
    if crate::stdlib::memcmp(
        _initial_data as *const libc::c_void,
        b"OggS\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as i32 as usize,
    ) != 0 as i32
    {
        return -(132 as i32);
    }
    if (_initial_bytes > 9223372036854775807 as isize as size_t) as i32 as isize != 0 {
        return -(129 as i32);
    }
    ogg_sync_init(&mut oy as *mut _ as *mut ogg_sync_state);
    data = ogg_sync_buffer(
        &mut oy as *mut _ as *mut ogg_sync_state,
        _initial_bytes as isize,
    );
    if !data.is_null() {
        let mut os: ogg_stream_state = ogg_stream_state {
            body_data: std::ptr::null_mut(),
            body_storage: 0,
            body_fill: 0,
            body_returned: 0,
            lacing_vals: std::ptr::null_mut(),
            granule_vals: std::ptr::null_mut(),
            lacing_storage: 0,
            lacing_fill: 0,
            lacing_packet: 0,
            lacing_returned: 0,
            header: [0; 282],
            header_fill: 0,
            e_o_s: 0,
            b_o_s: 0,
            serialno: 0,
            pageno: 0,
            packetno: 0,
            granulepos: 0,
        };
        let mut og: ogg_page = ogg_page {
            header: std::ptr::null_mut(),
            header_len: 0,
            body: std::ptr::null_mut(),
            body_len: 0,
        };
        let mut ret: i32 = 0;
        crate::stdlib::memcpy(
            data as *mut libc::c_void,
            _initial_data as *const libc::c_void,
            _initial_bytes,
        );
        ogg_sync_wrote(
            &mut oy as *mut _ as *mut ogg_sync_state,
            _initial_bytes as isize,
        );
        ogg_stream_init(&mut os as *mut _ as *mut ogg_stream_state, -(1 as i32));
        err = -(1 as i32);
        loop {
            let mut op: ogg_packet = ogg_packet {
                packet: std::ptr::null_mut(),
                bytes: 0,
                b_o_s: 0,
                e_o_s: 0,
                granulepos: 0,
                packetno: 0,
            };
            ret = ogg_sync_pageout(
                &mut oy as *mut _ as *mut ogg_sync_state,
                &mut og as *mut _ as *mut ogg_page,
            );
            /*Ignore holes.*/
            if !(ret < 0 as i32) {
                /*Stop if we run out of data.*/
                if ret == 0 {
                    break;
                }
                ogg_stream_reset_serialno(
                    &mut os as *mut _ as *mut ogg_stream_state,
                    ogg_page_serialno(&mut og as *mut _ as *const ogg_page),
                );
                ogg_stream_pagein(
                    &mut os as *mut _ as *mut ogg_stream_state,
                    &mut og as *mut _ as *mut ogg_page,
                );
                /*Only process the first packet on this page (if it's a BOS packet,
                it's required to be the only one).*/
                if ogg_stream_packetout(
                    &mut os as *mut _ as *mut ogg_stream_state,
                    &mut op as *mut _ as *mut ogg_packet,
                ) == 1 as i32
                {
                    if op.b_o_s != 0 {
                        ret = opus_head_parse(
                            _head as *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
                            op.packet,
                            op.bytes as size_t,
                        );
                        /*If this didn't look like Opus, keep going.*/
                        if !(ret == -(132 as i32)) {
                            /*Otherwise we're done, one way or another.*/
                            err = ret
                        }
                    } else {
                        /*We finished parsing the headers.
                        There is no Opus to be found.*/
                        err = -(132 as i32)
                    }
                }
            }
            if !(err == -(1 as i32)) {
                break;
            }
        }
        ogg_stream_clear(&mut os as *mut _ as *mut ogg_stream_state);
    } else {
        err = -(129 as i32)
    }
    ogg_sync_clear(&mut oy as *mut _ as *mut ogg_sync_state);
    return err;
}
/*Many, many internal helpers.
The intention is not to be confusing.
Rampant duplication and monolithic function implementation (though we do have
 some large, omnibus functions still) would be harder to understand anyway.
The high level functions are last.
Begin grokking near the end of the file if you prefer to read things
 top-down.*/
/*The read/seek functions track absolute position within the stream.*/
/*Read a little more data from the file/pipe into the ogg_sync framer.
_nbytes: The maximum number of bytes to read.
Return: A positive number of bytes read on success, 0 on end-of-file, or a
         negative value on failure.*/

unsafe extern "C" fn op_get_data(mut _of: *mut OggOpusFile, mut _nbytes: i32) -> i32 {
    let mut buffer: *mut u8 = std::ptr::null_mut();
    let mut nbytes: i32 = 0;
    buffer = ogg_sync_buffer(
        &mut (*_of).oy as *mut _ as *mut ogg_sync_state,
        _nbytes as isize,
    ) as *mut u8;
    nbytes = Some((*_of).callbacks.read.expect("non-null function pointer"))
        .expect("non-null function pointer")((*_of).stream, buffer, _nbytes);
    if (nbytes > 0 as i32) as i32 as isize != 0 {
        ogg_sync_wrote(
            &mut (*_of).oy as *mut _ as *mut ogg_sync_state,
            nbytes as isize,
        );
    }
    return nbytes;
}
/*Save a tiny smidge of verbosity to make the code more readable.*/

unsafe extern "C" fn op_seek_helper(mut _of: *mut OggOpusFile, mut _offset: i64) -> i32 {
    if _offset == (*_of).offset {
        return 0 as i32;
    }
    if (*_of).callbacks.seek.is_none()
        || Some((*_of).callbacks.seek.expect("non-null function pointer"))
            .expect("non-null function pointer")((*_of).stream, _offset, 0 as i32)
            != 0
    {
        return -(128 as i32);
    }
    (*_of).offset = _offset;
    ogg_sync_reset(&mut (*_of).oy as *mut _ as *mut ogg_sync_state);
    return 0 as i32;
}
/*Get the current position indicator of the underlying stream.
This should be the same as the value reported by tell().*/

unsafe extern "C" fn op_position(mut _of: *const OggOpusFile) -> i64 {
    /*The current position indicator is _not_ simply offset.
    We may also have unprocessed, buffered data in the sync state.*/
    return (*_of).offset + (*_of).oy.fill as i64 - (*_of).oy.returned as i64;
}
/*From the head of the stream, get the next page.
_boundary specifies if the function is allowed to fetch more data from the
 stream (and how much) or only use internally buffered data.
_boundary: -1: Unbounded search.
            0: Read no additional data.
               Use only cached data.
            n: Search for the start of a new page up to file position n.
Return: n>=0:       Found a page at absolute offset n.
        OP_FALSE:   Hit the _boundary limit.
        OP_EREAD:   An underlying read operation failed.
        OP_BADLINK: We hit end-of-file before reaching _boundary.*/

unsafe extern "C" fn op_get_next_page(
    mut _of: *mut OggOpusFile,
    mut _og: *mut ogg_page,
    mut _boundary: i64,
) -> i64 {
    while _boundary <= 0 as i32 as i64 || (*_of).offset < _boundary {
        let mut more: i32 = 0;
        more = ogg_sync_pageseek(
            &mut (*_of).oy as *mut _ as *mut ogg_sync_state,
            _og as *mut ogg_page,
        ) as i32;
        /*Skipped (-more) bytes.*/
        if (more < 0 as i32) as i32 as isize != 0 {
            (*_of).offset -= more as i64
        } else if more == 0 as i32 {
            let mut read_nbytes: i32 = 0;
            let mut ret: i32 = 0;
            /*Send more paramedics.*/
            if _boundary == 0 {
                return -(1 as i32) as i64;
            }
            if _boundary < 0 as i32 as i64 {
                read_nbytes = 2048 as i32
            } else {
                let mut position: i64 = 0;
                position = op_position(_of);
                if position >= _boundary {
                    return -(1 as i32) as i64;
                }
                read_nbytes = if _boundary - position < 2048 as i32 as i64 {
                    (_boundary) - position
                } else {
                    2048 as i32 as i64
                } as i32
            }
            ret = op_get_data(_of, read_nbytes);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return -(128 as i32) as i64;
            }
            if (ret == 0 as i32) as i32 as isize != 0 {
                /*Only fail cleanly on EOF if we didn't have a known boundary.
                Otherwise, we should have been able to reach that boundary, and this
                 is a fatal error.*/
                return if (_boundary < 0 as i32 as i64) as i32 as isize != 0 {
                    -(1 as i32)
                } else {
                    -(137 as i32)
                } as i64;
            }
        } else {
            /*Got a page.
            Return the page start offset and advance the internal offset past the
             page end.*/
            let mut page_offset: i64 = 0;
            page_offset = (*_of).offset;
            (*_of).offset += more as i64;
            return page_offset;
        }
    }
    return -(1 as i32) as i64;
}

unsafe extern "C" fn op_add_serialno(
    mut _og: *const ogg_page,
    mut _serialnos: *mut *mut ogg_uint32_t,
    mut _nserialnos: *mut i32,
    mut _cserialnos: *mut i32,
) -> i32 {
    let mut serialnos: *mut ogg_uint32_t = std::ptr::null_mut();
    let mut nserialnos: i32 = 0;
    let mut cserialnos: i32 = 0;
    let mut s: ogg_uint32_t = 0;
    s = ogg_page_serialno(_og as *const ogg_page) as ogg_uint32_t;
    serialnos = *_serialnos;
    nserialnos = *_nserialnos;
    cserialnos = *_cserialnos;
    if (nserialnos >= cserialnos) as i32 as isize != 0 {
        if (cserialnos
            > 2147483647 as i32 / ::std::mem::size_of::<ogg_uint32_t>() as usize as i32 - 1 as i32
                >> 1 as i32) as i32 as isize
            != 0
        {
            return -(129 as i32);
        }
        cserialnos = 2 as i32 * cserialnos + 1 as i32;
        serialnos = crate::stdlib::realloc(
            serialnos as *mut libc::c_void,
            (::std::mem::size_of::<ogg_uint32_t>() as usize).wrapping_mul(cserialnos as usize),
        ) as *mut ogg_uint32_t;
        if serialnos.is_null() as i32 as isize != 0 {
            return -(129 as i32);
        }
    }
    let fresh0 = nserialnos;
    nserialnos = nserialnos + 1;
    *serialnos.offset(fresh0 as isize) = s;
    *_serialnos = serialnos;
    *_nserialnos = nserialnos;
    *_cserialnos = cserialnos;
    return 0 as i32;
}
/*Returns nonzero if found.*/

unsafe extern "C" fn op_lookup_serialno(
    mut _s: ogg_uint32_t,
    mut _serialnos: *const ogg_uint32_t,
    mut _nserialnos: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < _nserialnos && *_serialnos.offset(i as isize) != _s {
        i += 1
    }
    return (i < _nserialnos) as i32;
}

unsafe extern "C" fn op_lookup_page_serialno(
    mut _og: *const ogg_page,
    mut _serialnos: *const ogg_uint32_t,
    mut _nserialnos: i32,
) -> i32 {
    return op_lookup_serialno(
        ogg_page_serialno(_og as *const ogg_page) as ogg_uint32_t,
        _serialnos,
        _nserialnos,
    );
}
/*Find the last page beginning before _offset with a valid granule position.
There is no '_boundary' parameter as it will always have to read more data.
This is much dirtier than the above, as Ogg doesn't have any backward search
 linkage.
This search prefers pages of the specified serial number.
If a page of the specified serial number is spotted during the
 seek-back-and-read-forward, it will return the info of last page of the
 matching serial number, instead of the very last page, unless the very last
 page belongs to a different link than preferred serial number.
If no page of the specified serial number is seen, it will return the info of
 the last page.
[out] _sr:   Returns information about the page that was found on success.
_offset:     The _offset before which to find a page.
             Any page returned will consist of data entirely before _offset.
_serialno:   The preferred serial number.
             If a page with this serial number is found, it will be returned
              even if another page in the same link is found closer to
              _offset.
             This is purely opportunistic: there is no guarantee such a page
              will be found if it exists.
_serialnos:  The list of serial numbers in the link that contains the
              preferred serial number.
_nserialnos: The number of serial numbers in the current link.
Return: 0 on success, or a negative value on failure.
        OP_EREAD:    Failed to read more data (error or EOF).
        OP_EBADLINK: We couldn't find a page even after seeking back to the
                      start of the stream.*/

unsafe extern "C" fn op_get_prev_page_serial(
    mut _of: *mut OggOpusFile,
    mut _sr: *mut OpusSeekRecord,
    mut _offset: i64,
    mut _serialno: ogg_uint32_t,
    mut _serialnos: *const ogg_uint32_t,
    mut _nserialnos: i32,
) -> i32 {
    let mut preferred_sr: OpusSeekRecord = OpusSeekRecord {
        search_start: 0,
        offset: 0,
        size: 0,
        serialno: 0,
        gp: 0,
    };
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut begin: i64 = 0;
    let mut end: i64 = 0;
    let mut original_end: i64 = 0;
    let mut chunk_size: opus_int32 = 0;
    let mut preferred_found: i32 = 0;
    begin = _offset;
    end = begin;
    original_end = end;
    preferred_found = 0 as i32;
    _offset = -(1 as i32) as i64;
    chunk_size = 65536 as i32;
    loop {
        let mut search_start: i64 = 0;
        let mut ret: i32 = 0;
        begin = if begin - chunk_size as i64 > 0 as i32 as i64 {
            (begin) - chunk_size as i64
        } else {
            0 as i32 as i64
        };
        ret = op_seek_helper(_of, begin);
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret;
        }
        search_start = begin;
        while (*_of).offset < end {
            let mut llret: i64 = 0;
            let mut serialno: ogg_uint32_t = 0;
            llret = op_get_next_page(_of, &mut og, end);
            if (llret < -(1 as i32) as i64) as i32 as isize != 0 {
                return llret as i32;
            } else {
                if llret == -(1 as i32) as i64 {
                    break;
                }
                serialno = ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t;
                /*Save the information for this page.
                We're not interested in the page itself... just the serial number, byte
                 offset, page size, and granule position.*/
                (*_sr).search_start = search_start;
                _offset = llret;
                (*_sr).offset = _offset;
                (*_sr).serialno = serialno;
                (*_sr).size = ((*_of).offset - _offset) as opus_int32;
                (*_sr).gp = ogg_page_granulepos(&mut og as *mut _ as *const ogg_page);
                /*If this page is from the stream we're looking for, remember it.*/
                if serialno == _serialno {
                    preferred_found = 1 as i32;
                    preferred_sr = *_sr
                }
                if op_lookup_serialno(serialno, _serialnos, _nserialnos) == 0 {
                    /*We fell off the end of the link, which means we seeked back too far
                     and shouldn't have been looking in that link to begin with.
                    If we found the preferred serial number, forget that we saw it.*/
                    preferred_found = 0 as i32
                }
                search_start = llret + 1 as i32 as i64
            }
        }
        /*We started from the beginning of the stream and found nothing.
        This should be impossible unless the contents of the stream changed out
         from under us after we read from it.*/
        if (begin == 0) as i32 as isize != 0 && (_offset < 0 as i32 as i64) as i32 as isize != 0 {
            return -(137 as i32);
        }
        /*Bump up the chunk size.
        This is mildly helpful when seeks are very expensive (http).*/
        chunk_size = if 2 as i32 * chunk_size < 1024 as i32 * 1024 as i32 {
            (2 as i32) * chunk_size
        } else {
            (1024 as i32) * 1024 as i32
        };
        /*Avoid quadratic complexity if we hit an invalid patch of the file.*/
        end = if (begin + 65307 as i32 as i64 - 1 as i32 as i64) < original_end {
            (begin + 65307 as i32 as i64) - 1 as i32 as i64
        } else {
            original_end
        };
        if !(_offset < 0 as i32 as i64) {
            break;
        }
    }
    if preferred_found != 0 {
        *_sr = preferred_sr
    }
    return 0 as i32;
}
/*Find the last page beginning before _offset with the given serial number and
 a valid granule position.
Unlike the above search, this continues until it finds such a page, but does
 not stray outside the current link.
We could implement it (inefficiently) by calling op_get_prev_page_serial()
 repeatedly until it returned a page that had both our preferred serial
 number and a valid granule position, but doing it with a separate function
 allows us to avoid repeatedly re-scanning valid pages from other streams as
 we seek-back-and-read-forward.
[out] _gp:   Returns the granule position of the page that was found on
              success.
_offset:     The _offset before which to find a page.
             Any page returned will consist of data entirely before _offset.
_serialno:   The target serial number.
_serialnos:  The list of serial numbers in the link that contains the
              preferred serial number.
_nserialnos: The number of serial numbers in the current link.
Return: The offset of the page on success, or a negative value on failure.
        OP_EREAD:    Failed to read more data (error or EOF).
        OP_EBADLINK: We couldn't find a page even after seeking back past the
                      beginning of the link.*/

unsafe extern "C" fn op_get_last_page(
    mut _of: *mut OggOpusFile,
    mut _gp: *mut ogg_int64_t,
    mut _offset: i64,
    mut _serialno: ogg_uint32_t,
    mut _serialnos: *const ogg_uint32_t,
    mut _nserialnos: i32,
) -> i64 {
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut gp: ogg_int64_t = 0;
    let mut begin: i64 = 0;
    let mut end: i64 = 0;
    let mut original_end: i64 = 0;
    let mut chunk_size: opus_int32 = 0;
    /*The target serial number must belong to the current link.*/
    begin = _offset;
    end = begin;
    original_end = end;
    _offset = -(1 as i32) as i64;
    /*We shouldn't have to initialize gp, but gcc is too dumb to figure out that
    ret>=0 implies we entered the if(page_gp!=-1) block at least once.*/
    gp = -(1 as i32) as ogg_int64_t;
    chunk_size = 65536 as i32;
    loop {
        let mut left_link: i32 = 0;
        let mut ret: i32 = 0;
        begin = if begin - chunk_size as i64 > 0 as i32 as i64 {
            (begin) - chunk_size as i64
        } else {
            0 as i32 as i64
        };
        ret = op_seek_helper(_of, begin);
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret as i64;
        }
        left_link = 0 as i32;
        while (*_of).offset < end {
            let mut llret: i64 = 0;
            let mut serialno: ogg_uint32_t = 0;
            llret = op_get_next_page(_of, &mut og, end);
            if (llret < -(1 as i32) as i64) as i32 as isize != 0 {
                return llret;
            } else {
                if llret == -(1 as i32) as i64 {
                    break;
                }
                serialno = ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t;
                if serialno == _serialno {
                    let mut page_gp: ogg_int64_t = 0;
                    /*The page is from the right stream...*/
                    page_gp = ogg_page_granulepos(&mut og as *mut _ as *const ogg_page);
                    if page_gp != -(1 as i32) as isize {
                        /*And has a valid granule position.
                        Let's remember it.*/
                        _offset = llret;
                        gp = page_gp
                    }
                } else if (op_lookup_serialno(serialno, _serialnos, _nserialnos) == 0) as i32
                    as isize
                    != 0
                {
                    /*We fell off the start of the link, which means we don't need to keep
                    seeking any farther back.*/
                    left_link = 1 as i32
                }
            }
        }
        /*We started from at or before the beginning of the link and found nothing.
        This should be impossible unless the contents of the stream changed out
         from under us after we read from it.*/
        if ((left_link != 0) as i32 as isize != 0 || (begin == 0) as i32 as isize != 0)
            && (_offset < 0 as i32 as i64) as i32 as isize != 0
        {
            return -(137 as i32) as i64;
        }
        /*Bump up the chunk size.
        This is mildly helpful when seeks are very expensive (http).*/
        chunk_size = if 2 as i32 * chunk_size < 1024 as i32 * 1024 as i32 {
            (2 as i32) * chunk_size
        } else {
            (1024 as i32) * 1024 as i32
        };
        /*Avoid quadratic complexity if we hit an invalid patch of the file.*/
        end = if (begin + 65307 as i32 as i64 - 1 as i32 as i64) < original_end {
            (begin + 65307 as i32 as i64) - 1 as i32 as i64
        } else {
            original_end
        };
        if !(_offset < 0 as i32 as i64) {
            break;
        }
    }
    *_gp = gp;
    return _offset;
}
/*Uses the local ogg_stream storage in _of.
This is important for non-streaming input sources.*/

unsafe extern "C" fn op_fetch_headers_impl(
    mut _of: *mut OggOpusFile,
    mut _head: *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _serialnos: *mut *mut ogg_uint32_t,
    mut _nserialnos: *mut i32,
    mut _cserialnos: *mut i32,
    mut _og: *mut ogg_page,
) -> i32 {
    let mut op: ogg_packet = ogg_packet {
        packet: std::ptr::null_mut(),
        bytes: 0,
        b_o_s: 0,
        e_o_s: 0,
        granulepos: 0,
        packetno: 0,
    };
    let mut ret: i32 = 0;
    if !_serialnos.is_null() {
        *_nserialnos = 0 as i32
    }
    /*Extract the serialnos of all BOS pages plus the first set of Opus headers
    we see in the link.*/
    while ogg_page_bos(_og as *const ogg_page) != 0 {
        if !_serialnos.is_null() {
            if (op_lookup_page_serialno(_og, *_serialnos, *_nserialnos) != 0) as i32 as isize != 0 {
                /*A dupe serialnumber in an initial header packet set==invalid stream.*/
                return -(133 as i32);
            }
            ret = op_add_serialno(_og, _serialnos, _nserialnos, _cserialnos);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        if (*_of).ready_state < 3 as i32 {
            /*We don't have an Opus stream in this link yet, so begin prospective
             stream setup.
            We need a stream to get packets.*/
            ogg_stream_reset_serialno(
                &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                ogg_page_serialno(_og as *const ogg_page),
            );
            ogg_stream_pagein(
                &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                _og as *mut ogg_page,
            );
            if (ogg_stream_packetout(
                &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                &mut op as *mut _ as *mut ogg_packet,
            ) > 0 as i32) as i32 as isize
                != 0
            {
                ret = opus_head_parse(
                    _head as *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
                    op.packet,
                    op.bytes as size_t,
                );
                /*TODO: Should a BOS page with no packets be an error?*/
                /*Found a valid Opus header.
                Continue setup.*/
                if (ret >= 0 as i32) as i32 as isize != 0 {
                    (*_of).ready_state = 3 as i32
                } else if ret != -(132 as i32) {
                    return ret;
                }
            }
        }
        /*If it's just a stream type we don't recognize, ignore it.
        Everything else is fatal.*/
        /*Get the next page.
        No need to clamp the boundary offset against _of->end, as all errors
         become OP_ENOTFORMAT or OP_EBADHEADER.*/
        if (op_get_next_page(
            _of,
            _og,
            (if (*_of).offset
                < ((2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 65536 as i32 as isize) as i64
            {
                (*_of).offset
            } else {
                ((2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 65536 as i32 as isize) as i64
            }) + 65536 as i32 as i64,
        ) < 0 as i32 as i64) as i32 as isize
            != 0
        {
            return if (*_of).ready_state < 3 as i32 {
                -(132 as i32)
            } else {
                -(133 as i32)
            };
        }
    }
    if ((*_of).ready_state != 3 as i32) as i32 as isize != 0 {
        return -(132 as i32);
    }
    /*If the first non-header page belonged to our Opus stream, submit it.*/
    if (*_of).os.serialno == ogg_page_serialno(_og as *const ogg_page) as isize {
        ogg_stream_pagein(
            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
            _og as *mut ogg_page,
        );
    }
    loop
    /*Loop getting packets.*/
    {
        match ogg_stream_packetout(
            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
            &mut op as *mut _ as *mut ogg_packet,
        ) {
            0 => {
                loop
                /*Loop getting pages.*/
                        /*No need to clamp the boundary offset against _of->end, as all
                errors become OP_EBADHEADER.*/
                {
                    if (op_get_next_page(
                        _of,
                        _og,
                        (if (*_of).offset
                            < ((2 as i32 as isize
                                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                                | 1 as i32 as isize)
                                - 65536 as i32 as isize) as i64
                        {
                            (*_of).offset
                        } else {
                            ((2 as i32 as isize
                                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                                | 1 as i32 as isize)
                                - 65536 as i32 as isize) as i64
                        }) + 65536 as i32 as i64,
                    ) < 0 as i32 as i64) as i32 as isize
                        != 0
                    {
                        return -(133 as i32);
                    }
                    /*If this page belongs to the correct stream, go parse it.*/
                    if (*_of).os.serialno == ogg_page_serialno(_og as *const ogg_page) as isize {
                        ogg_stream_pagein(
                            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                            _og as *mut ogg_page,
                        );
                        break;
                    } else if (ogg_page_bos(_og as *const ogg_page) != 0) as i32 as isize != 0 {
                        return -(133 as i32);
                    }
                }
            }
            -1 => {
                /*If the link ends before we see the Opus comment header, abort.*/
                /*We shouldn't get a hole in the headers!*/
                return -(133 as i32);
            }
            _ => {
                /*Got a packet.
                It should be the comment header.*/
                ret = opus_tags_parse(
                    _tags as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
                    op.packet,
                    op.bytes as size_t,
                );
                if (ret < 0 as i32) as i32 as isize != 0 {
                    return ret;
                }
                /*Make sure the page terminated at the end of the comment header.
                If there is another packet on the page, or part of a packet, then
                 reject the stream.
                Otherwise seekable sources won't be able to seek back to the start
                 properly.*/
                ret = ogg_stream_packetout(
                    &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                    &mut op as *mut _ as *mut ogg_packet,
                );
                if (ret != 0 as i32) as i32 as isize != 0
                    || (*(*_og)
                        .header
                        .offset(((*_og).header_len - 1 as i32 as isize) as isize)
                        as i32
                        == 255 as i32) as i32 as isize
                        != 0
                {
                    /*If we fail, the caller assumes our tags are uninitialized.*/
                    opus_tags_clear(
                        _tags as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
                    );
                    return -(133 as i32);
                }
                return 0 as i32;
            }
        }
    }
}

unsafe extern "C" fn op_fetch_headers(
    mut _of: *mut OggOpusFile,
    mut _head: *mut crate::src::opusfile_0_9::src::opusfile::OpusHead,
    mut _tags: *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
    mut _serialnos: *mut *mut ogg_uint32_t,
    mut _nserialnos: *mut i32,
    mut _cserialnos: *mut i32,
    mut _og: *mut ogg_page,
) -> i32 {
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut ret: i32 = 0;
    if _og.is_null() {
        /*No need to clamp the boundary offset against _of->end, as all errors
        become OP_ENOTFORMAT.*/
        if (op_get_next_page(
            _of,
            &mut og,
            (if (*_of).offset
                < ((2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 65536 as i32 as isize) as i64
            {
                (*_of).offset
            } else {
                ((2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 65536 as i32 as isize) as i64
            }) + 65536 as i32 as i64,
        ) < 0 as i32 as i64) as i32 as isize
            != 0
        {
            return -(132 as i32);
        }
        _og = &mut og
    }
    (*_of).ready_state = 2 as i32;
    ret = op_fetch_headers_impl(_of, _head, _tags, _serialnos, _nserialnos, _cserialnos, _og);
    /*Revert back from OP_STREAMSET to OP_OPENED on failure, to prevent
    double-free of the tags in an unseekable stream.*/
    if (ret < 0 as i32) as i32 as isize != 0 {
        (*_of).ready_state = 2 as i32
    }
    return ret;
}
/*Granule position manipulation routines.
A granule position is defined to be an unsigned 64-bit integer, with the
 special value -1 in two's complement indicating an unset or invalid granule
 position.
We are not guaranteed to have an unsigned 64-bit type, so we construct the
 following routines that
 a) Properly order negative numbers as larger than positive numbers, and
 b) Check for underflow or overflow past the special -1 value.
This lets us operate on the full, valid range of granule positions in a
 consistent and safe manner.
This full range is organized into distinct regions:
 [ -1 (invalid) ][ 0 ... OP_INT64_MAX ][ OP_INT64_MIN ... -2 ][-1 (invalid) ]

No one should actually use granule positions so large that they're negative,
 even if they are technically valid, as very little software handles them
 correctly (including most of Xiph.Org's).
This library also refuses to support durations so large they won't fit in a
 signed 64-bit integer (to avoid exposing this mess to the application, and
 to simplify a good deal of internal arithmetic), so the only way to use them
 successfully is if pcm_start is very large.
This means there isn't anything you can do with negative granule positions
 that you couldn't have done with purely non-negative ones.
The main purpose of these routines is to allow us to think very explicitly
 about the possible failure cases of all granule position manipulations.*/
/*Safely adds a small signed integer to a valid (not -1) granule position.
The result can use the full 64-bit range of values (both positive and
 negative), but will fail on overflow (wrapping past -1; wrapping past
 OP_INT64_MAX is explicitly okay).
[out] _dst_gp: The resulting granule position.
               Only modified on success.
_src_gp:       The granule position to add to.
               This must not be -1.
_delta:        The amount to add.
               This is allowed to be up to 32 bits to support the maximum
                duration of a single Ogg page (255 packets * 120 ms per
                packet == 1,468,800 samples at 48 kHz).
Return: 0 on success, or OP_EINVAL if the result would wrap around past -1.*/

unsafe extern "C" fn op_granpos_add(
    mut _dst_gp: *mut ogg_int64_t,
    mut _src_gp: ogg_int64_t,
    mut _delta: opus_int32,
) -> i32 {
    /*The code below handles this case correctly, but there's no reason we
    should ever be called with these values, so make sure we aren't.*/
    if _delta > 0 as i32 {
        /*Adding this amount to the granule position would overflow its 64-bit
        range.*/
        if (_src_gp < 0 as i32 as isize) as i32 as isize != 0
            && (_src_gp >= (-(1 as i32) - _delta) as isize) as i32 as isize != 0
        {
            return -(131 as i32);
        }
        if (_src_gp
            > (2 as i32 as isize * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - _delta as isize) as i32 as isize
            != 0
        {
            /*Adding this amount to the granule position would overflow the positive
             half of its 64-bit range.
            Since signed overflow is undefined in C, do it in a way the compiler
             isn't allowed to screw up.*/
            _delta -= ((2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - _src_gp) as opus_int32
                + 1 as i32;
            _src_gp = -(2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - 1 as i32 as isize
        }
    } else if _delta < 0 as i32 {
        /*Subtracting this amount from the granule position would underflow its
        64-bit range.*/
        if _src_gp >= 0 as i32 as isize && (_src_gp < -_delta as isize) as i32 as isize != 0 {
            return -(131 as i32);
        }
        if (_src_gp
            < -(2 as i32 as isize * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - 1 as i32 as isize
                - _delta as isize) as i32 as isize
            != 0
        {
            /*Subtracting this amount from the granule position would underflow the
             negative half of its 64-bit range.
            Since signed underflow is undefined in C, do it in a way the compiler
             isn't allowed to screw up.*/
            _delta += (_src_gp
                - (-(2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 1 as i32 as isize)) as opus_int32
                + 1 as i32;
            _src_gp = 2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize
        }
    }
    *_dst_gp = _src_gp + _delta as isize;
    return 0 as i32;
}
/*Safely computes the difference between two granule positions.
The difference must fit in a signed 64-bit integer, or the function fails.
It correctly handles the case where the granule position has wrapped around
 from positive values to negative ones.
[out] _delta: The difference between the granule positions.
              Only modified on success.
_gp_a:        The granule position to subtract from.
              This must not be -1.
_gp_b:        The granule position to subtract.
              This must not be -1.
Return: 0 on success, or OP_EINVAL if the result would not fit in a signed
         64-bit integer.*/

unsafe extern "C" fn op_granpos_diff(
    mut _delta: *mut ogg_int64_t,
    mut _gp_a: ogg_int64_t,
    mut _gp_b: ogg_int64_t,
) -> i32 {
    let mut gp_a_negative: i32 = 0;
    let mut gp_b_negative: i32 = 0;
    /*The code below handles these cases correctly, but there's no reason we
    should ever be called with these values, so make sure we aren't.*/
    gp_a_negative = (_gp_a < 0 as i32 as isize) as i32 as isize as i32;
    gp_b_negative = (_gp_b < 0 as i32 as isize) as i32 as isize as i32;
    if (gp_a_negative ^ gp_b_negative != 0) as i32 as isize != 0 {
        let mut da: ogg_int64_t = 0;
        let mut db: ogg_int64_t = 0;
        if gp_a_negative != 0 {
            /*_gp_a has wrapped to a negative value but _gp_b hasn't: the difference
            should be positive.*/
            /*Step 1: Handle wrapping.*/
            /*_gp_a < 0 => da < 0.*/
            da = -(2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - 1 as i32 as isize
                - _gp_a
                - 1 as i32 as isize;
            /*_gp_b >= 0  => db >= 0.*/
            db = (2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - _gp_b;
            /*Step 2: Check for overflow.*/
            if ((2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                + da
                < db) as i32 as isize
                != 0
            {
                return -(131 as i32);
            }
            *_delta = db - da
        } else {
            /*_gp_b has wrapped to a negative value but _gp_a hasn't: the difference
            should be negative.*/
            /*Step 1: Handle wrapping.*/
            /*_gp_a >= 0 => da <= 0*/
            da = _gp_a
                + (-(2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 1 as i32 as isize);
            /*_gp_b < 0 => db <= 0*/
            db = -(2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize)
                - 1 as i32 as isize
                - _gp_b;
            /*Step 2: Check for overflow.*/
            if (da
                < -(2 as i32 as isize
                    * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                    | 1 as i32 as isize)
                    - 1 as i32 as isize
                    - db) as i32 as isize
                != 0
            {
                return -(131 as i32);
            }
            *_delta = da + db
        }
    } else {
        *_delta = _gp_a - _gp_b
    }
    return 0 as i32;
}

unsafe extern "C" fn op_granpos_cmp(mut _gp_a: ogg_int64_t, mut _gp_b: ogg_int64_t) -> i32 {
    /*The invalid granule position -1 should behave like NaN: neither greater
     than nor less than any other granule position, nor equal to any other
     granule position, including itself.
    However, that means there isn't anything we could sensibly return from this
     function for it.*/
    /*Handle the wrapping cases.*/
    if (_gp_a < 0 as i32 as isize) as i32 as isize != 0 {
        if _gp_b >= 0 as i32 as isize {
            return 1 as i32;
        }
    /*Else fall through.*/
    } else if (_gp_b < 0 as i32 as isize) as i32 as isize != 0 {
        return -(1 as i32);
    }
    /*No wrapping case.*/
    return (_gp_a > _gp_b) as i32 - (_gp_b > _gp_a) as i32;
}
/*Returns the duration of the packet (in samples at 48 kHz), or a negative
value on error.*/

unsafe extern "C" fn op_get_packet_duration(mut _data: *const u8, mut _len: i32) -> i32 {
    let mut nframes: i32 = 0;
    let mut frame_size: i32 = 0;
    let mut nsamples: i32 = 0;
    nframes = crate::src::opus_1_2_1::src::opus_decoder::opus_packet_get_nb_frames(_data, _len);
    if (nframes < 0 as i32) as i32 as isize != 0 {
        return -(136 as i32);
    }
    frame_size =
        crate::src::opus_1_2_1::src::opus::opus_packet_get_samples_per_frame(_data, 48000 as i32);
    nsamples = nframes * frame_size;
    if (nsamples > 120 as i32 * 48 as i32) as i32 as isize != 0 {
        return -(136 as i32);
    }
    return nsamples;
}
/*This function more properly belongs in info.c, but we define it here to allow
the static granule position manipulation functions to remain static.*/
#[no_mangle]

pub unsafe extern "C" fn opus_granule_sample(
    mut _head: *const crate::src::opusfile_0_9::src::opusfile::OpusHead,
    mut _gp: ogg_int64_t,
) -> ogg_int64_t {
    let mut _pre_skip: opus_int32 = 0;
    _pre_skip = (*_head).pre_skip as opus_int32;
    if _gp != -(1 as i32) as isize && op_granpos_add(&mut _gp, _gp, -_pre_skip) != 0 {
        _gp = -(1 as i32) as ogg_int64_t
    }
    return _gp;
}
/*Grab all the packets currently in the stream state, and compute their
 durations.
_of->op_count is set to the number of packets collected.
[out] _durations: Returns the durations of the individual packets.
Return: The total duration of all packets, or OP_HOLE if there was a hole.*/

unsafe extern "C" fn op_collect_audio_packets(
    mut _of: *mut OggOpusFile,
    mut _durations: *mut i32,
) -> opus_int32 {
    let mut total_duration: opus_int32 = 0;
    let mut op_count: i32 = 0;
    /*Count the durations of all packets in the page.*/
    op_count = 0 as i32;
    total_duration = 0 as i32;
    loop {
        let mut ret: i32 = 0;
        /*This takes advantage of undocumented libogg behavior that returned
         ogg_packet buffers are valid at least until the next page is
         submitted.
        Relying on this is not too terrible, as _none_ of the Ogg memory
         ownership/lifetime rules are well-documented.
        But I can read its code and know this will work.*/
        ret = ogg_stream_packetout(
            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
            (*_of).op.as_mut_ptr().offset(op_count as isize) as *mut ogg_packet,
        );
        if ret == 0 {
            break;
        }
        if (ret < 0 as i32) as i32 as isize != 0 {
            /*We shouldn't get holes in the middle of pages.*/
            /*Set the return value and break out of the loop.
            We want to make sure op_count gets set to 0, because we've ingested a
             page, so any previously loaded packets are now invalid.*/
            total_duration = -(3 as i32);
            break;
        } else {
            /*Unless libogg is broken, we can't get more than 255 packets from a
            single page.*/
            *_durations.offset(op_count as isize) = op_get_packet_duration(
                (*_of).op[op_count as usize].packet,
                (*_of).op[op_count as usize].bytes as i32,
            );
            if (*_durations.offset(op_count as isize) > 0 as i32) as i32 as isize != 0 {
                /*With at most 255 packets on a page, this can't overflow.*/
                let fresh1 = op_count;
                op_count = op_count + 1;
                total_duration += *_durations.offset(fresh1 as isize)
            } else if op_count > 0 as i32 {
                /*Ignore packets with an invalid TOC sequence.*/
                /*But save the granule position, if there was one.*/
                (*_of).op[(op_count - 1 as i32) as usize].granulepos =
                    (*_of).op[op_count as usize].granulepos
            }
        }
    }
    (*_of).op_pos = 0 as i32;
    (*_of).op_count = op_count;
    return total_duration;
}
/*Starting from current cursor position, get the initial PCM offset of the next
 page.
This also validates the granule position on the first page with a completed
 audio data packet, as required by the spec.
If this link is completely empty (no pages with completed packets), then this
 function sets pcm_start=pcm_end=0 and returns the BOS page of the next link
 (if any).
In the seekable case, we initialize pcm_end=-1 before calling this function,
 so that later we can detect that the link was empty before calling
 op_find_final_pcm_offset().
[inout] _link: The link for which to find pcm_start.
[out] _og:     Returns the BOS page of the next link if this link was empty.
               In the unseekable case, we can then feed this to
                op_fetch_headers() to start the next link.
               The caller may pass NULL (e.g., for seekable streams), in
                which case this page will be discarded.
Return: 0 on success, 1 if there is a buffered BOS page available, or a
         negative value on unrecoverable error.*/

unsafe extern "C" fn op_find_initial_pcm_offset(
    mut _of: *mut OggOpusFile,
    mut _link: *mut OggOpusLink,
    mut _og: *mut ogg_page,
) -> i32 {
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut page_offset: i64 = 0;
    let mut pcm_start: ogg_int64_t = 0;
    let mut prev_packet_gp: ogg_int64_t = 0;
    let mut cur_page_gp: ogg_int64_t = 0;
    let mut serialno: ogg_uint32_t = 0;
    let mut total_duration: opus_int32 = 0;
    let mut durations: [i32; 255] = [0; 255];
    let mut cur_page_eos: i32 = 0;
    let mut op_count: i32 = 0;
    let mut pi: i32 = 0;
    if _og.is_null() {
        _og = &mut og
    }
    serialno = (*_of).os.serialno as ogg_uint32_t;
    op_count = 0 as i32;
    /*We shouldn't have to initialize total_duration, but gcc is too dumb to
    figure out that op_count>0 implies we've been through the whole loop at
    least once.*/
    total_duration = 0 as i32;
    loop {
        page_offset = op_get_next_page(_of, _og, (*_of).end);
        /*We should get a page unless the file is truncated or mangled.
        Otherwise there are no audio data packets in the whole logical stream.*/
        if (page_offset < 0 as i32 as i64) as i32 as isize != 0 {
            /*Fail if there was a read error.*/
            if page_offset < -(1 as i32) as i64 {
                return page_offset as i32;
            }
            /*Fail if the pre-skip is non-zero, since it's asking us to skip more
            samples than exist.*/
            if (*_link).head.pre_skip > 0 as i32 as u32 {
                return -(139 as i32);
            }
            (*_link).pcm_file_offset = 0 as i32 as ogg_int64_t;
            /*Set pcm_end and end_offset so we can skip the call to
            op_find_final_pcm_offset().*/
            (*_link).pcm_end = 0 as i32 as ogg_int64_t;
            (*_link).pcm_start = (*_link).pcm_end;
            (*_link).end_offset = (*_link).data_offset;
            return 0 as i32;
        }
        /*Similarly, if we hit the next link in the chain, we've gone too far.*/
        if (ogg_page_bos(_og as *const ogg_page) != 0) as i32 as isize != 0 {
            if (*_link).head.pre_skip > 0 as i32 as u32 {
                return -(139 as i32);
            }
            /*Set pcm_end and end_offset so we can skip the call to
            op_find_final_pcm_offset().*/
            (*_link).pcm_file_offset = 0 as i32 as ogg_int64_t;
            (*_link).pcm_end = 0 as i32 as ogg_int64_t;
            (*_link).pcm_start = (*_link).pcm_end;
            (*_link).end_offset = (*_link).data_offset;
            /*Tell the caller we've got a buffered page for them.*/
            return 1 as i32;
        }
        /*Ignore pages from other streams (not strictly necessary, because of the
        checks in ogg_stream_pagein(), but saves some work).*/
        if !(serialno != ogg_page_serialno(_og as *const ogg_page) as ogg_uint32_t) {
            ogg_stream_pagein(
                &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                _og as *mut ogg_page,
            );
            /*Bitrate tracking: add the header's bytes here.
            The body bytes are counted when we consume the packets.*/
            (*_of).bytes_tracked += (*_og).header_len as i64;
            loop
            /*Count the durations of all packets in the page.*/
            {
                total_duration = op_collect_audio_packets(_of, durations.as_mut_ptr());
                if !((total_duration < 0 as i32) as i32 as isize != 0) {
                    break;
                }
            }
            op_count = (*_of).op_count
        }
        if !(op_count <= 0 as i32) {
            break;
        }
    }
    /*We found the first page with a completed audio data packet: actually look
     at the granule position.
    RFC 3533 says, "A special value of -1 (in two's complement) indicates that
     no packets finish on this page," which does not say that a granule
     position that is NOT -1 indicates that some packets DO finish on that page
     (even though this was the intention, libogg itself violated this intention
     for years before we fixed it).
    The Ogg Opus specification only imposes its start-time requirements
     on the granule position of the first page with completed packets,
     so we ignore any set granule positions until then.*/
    cur_page_gp = (*_of).op[(op_count - 1 as i32) as usize].granulepos;
    /*But getting a packet without a valid granule position on the page is not
    okay.*/
    if cur_page_gp == -(1 as i32) as isize {
        return -(139 as i32);
    }
    cur_page_eos = (*_of).op[(op_count - 1 as i32) as usize].e_o_s as i32;
    if (cur_page_eos == 0) as i32 as isize != 0 {
        /*The EOS flag wasn't set.
        Work backwards from the provided granule position to get the starting PCM
         offset.*/
        if (op_granpos_add(&mut pcm_start, cur_page_gp, -total_duration) < 0 as i32) as i32 as isize
            != 0
        {
            /*The starting granule position MUST not be smaller than the amount of
            audio on the first page with completed packets.*/
            return -(139 as i32);
        }
    } else if (op_granpos_add(&mut pcm_start, cur_page_gp, -total_duration) < 0 as i32) as i32
        as isize
        != 0
    {
        /*The first page with completed packets was also the last.*/
        /*If there's less audio on the page than indicated by the granule
        position, then we're doing end-trimming, and the starting PCM offset
        is zero by spec mandate.*/
        pcm_start = 0 as i32 as ogg_int64_t;
        /*However, the end-trimming MUST not ask us to trim more samples than
        exist after applying the pre-skip.*/
        if (op_granpos_cmp(cur_page_gp, (*_link).head.pre_skip as ogg_int64_t) < 0 as i32) as i32
            as isize
            != 0
        {
            return -(139 as i32);
        }
    }
    /*Timestamp the individual packets.*/
    prev_packet_gp = pcm_start;
    let mut current_block_53: u64;
    pi = 0 as i32;
    while pi < op_count {
        if cur_page_eos != 0 {
            let mut diff: ogg_int64_t = 0 as i32 as ogg_int64_t;
            diff = durations[pi as usize] as isize - diff;
            /*If we have samples to trim...*/
            if diff > 0 as i32 as isize {
                /*If we trimmed the entire packet, stop (the spec says encoders
                shouldn't do this, but we support it anyway).*/
                if (diff > durations[pi as usize] as isize) as i32 as isize != 0 {
                    break;
                }
                prev_packet_gp = cur_page_gp;
                (*_of).op[pi as usize].granulepos = prev_packet_gp;
                /*Move the EOS flag to this packet, if necessary, so we'll trim the
                samples.*/
                (*_of).op[pi as usize].e_o_s = 1 as i32 as isize;
                current_block_53 = 16738040538446813684;
            } else {
                current_block_53 = 2116367355679836638;
            }
        } else {
            current_block_53 = 2116367355679836638;
        }
        match current_block_53 {
            2116367355679836638 =>
            /*Update the granule position as normal.*/
            {
                prev_packet_gp = (*_of).op[pi as usize].granulepos
            }
            _ => {}
        }
        pi += 1
    }
    /*Update the packet count after end-trimming.*/
    (*_of).op_count = pi;
    (*_of).cur_discard_count = (*_link).head.pre_skip as opus_int32;
    (*_link).pcm_file_offset = 0 as i32 as ogg_int64_t;
    (*_link).pcm_start = pcm_start;
    (*_of).prev_packet_gp = (*_link).pcm_start;
    (*_of).prev_page_offset = page_offset;
    return 0 as i32;
}
/*Starting from current cursor position, get the final PCM offset of the
 previous page.
This also validates the duration of the link, which, while not strictly
 required by the spec, we need to ensure duration calculations don't
 overflow.
This is only done for seekable sources.
We must validate that op_find_initial_pcm_offset() succeeded for this link
 before calling this function, otherwise it will scan the entire stream
 backwards until it reaches the start, and then fail.*/

unsafe extern "C" fn op_find_final_pcm_offset(
    mut _of: *mut OggOpusFile,
    mut _serialnos: *const ogg_uint32_t,
    mut _nserialnos: i32,
    mut _link: *mut OggOpusLink,
    mut _offset: i64,
    mut _end_serialno: ogg_uint32_t,
    mut _end_gp: ogg_int64_t,
    mut _total_duration: *mut ogg_int64_t,
) -> i32 {
    let mut total_duration: ogg_int64_t = 0;
    let mut duration: ogg_int64_t = 0;
    let mut cur_serialno: ogg_uint32_t = 0;
    /*For the time being, fetch end PCM offset the simple way.*/
    cur_serialno = (*_link).serialno;
    if _end_serialno != cur_serialno || _end_gp == -(1 as i32) as isize {
        _offset = op_get_last_page(
            _of,
            &mut _end_gp,
            _offset,
            cur_serialno,
            _serialnos,
            _nserialnos,
        );
        if (_offset < 0 as i32 as i64) as i32 as isize != 0 {
            return _offset as i32;
        }
    }
    /*At worst we should have found the first page with completed packets.*/
    if (_offset < (*_link).data_offset) as i32 as isize != 0 {
        return -(137 as i32);
    }
    /*This implementation requires that the difference between the first and last
    granule positions in each link be representable in a signed, 64-bit
    number, and that each link also have at least as many samples as the
    pre-skip requires.*/
    if (op_granpos_diff(&mut duration, _end_gp, (*_link).pcm_start) < 0 as i32) as i32 as isize != 0
        || (duration < (*_link).head.pre_skip as isize) as i32 as isize != 0
    {
        return -(139 as i32);
    }
    /*We also require that the total duration be representable in a signed,
    64-bit number.*/
    duration -= (*_link).head.pre_skip as isize;
    total_duration = *_total_duration;
    if ((2 as i32 as isize * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
        | 1 as i32 as isize)
        - duration
        < total_duration) as i32 as isize
        != 0
    {
        return -(139 as i32);
    }
    *_total_duration = total_duration + duration;
    (*_link).pcm_end = _end_gp;
    (*_link).end_offset = _offset;
    return 0 as i32;
}
/*Rescale the number _x from the range [0,_from] to [0,_to].
_from and _to must be positive.*/

unsafe extern "C" fn op_rescale64(mut _x: i64, mut _from: i64, mut _to: i64) -> i64 {
    let mut frac: i64 = 0;
    let mut ret: i64 = 0;
    let mut i: i32 = 0;
    if _x >= _from {
        return _to;
    }
    if _x <= 0 as i32 as i64 {
        return 0 as i32 as i64;
    }
    frac = 0 as i32 as i64;
    i = 0 as i32;
    while i < 63 as i32 {
        frac <<= 1 as i32;
        if _x >= _from >> 1 as i32 {
            _x -= _from - _x;
            frac |= 1 as i32 as i64
        } else {
            _x <<= 1 as i32
        }
        i += 1
    }
    ret = 0 as i32 as i64;
    i = 0 as i32;
    while i < 63 as i32 {
        if frac & 1 as i32 as i64 != 0 {
            ret = (ret & _to & 1 as i32 as i64) + (ret >> 1 as i32) + (_to >> 1 as i32)
        } else {
            ret >>= 1 as i32
        }
        frac >>= 1 as i32;
        i += 1
    }
    return ret;
}
/*The minimum granule position spacing allowed for making predictions.
This corresponds to about 1 second of audio at 48 kHz for both Opus and
 Vorbis, or one keyframe interval in Theora with the default keyframe spacing
 of 256.*/
/*Try to estimate the location of the next link using the current seek
records, assuming the initial granule position of any streams we've found is
0.*/

unsafe extern "C" fn op_predict_link_start(
    mut _sr: *const OpusSeekRecord,
    mut _nsr: i32,
    mut _searched: i64,
    mut _end_searched: i64,
    mut _bias: opus_int32,
) -> i64 {
    let mut bisect: i64 = 0;
    let mut sri: i32 = 0;
    let mut srj: i32 = 0;
    /*Require that we be at least OP_CHUNK_SIZE from the end.
    We don't require that we be at least OP_CHUNK_SIZE from the beginning,
     because if we are we'll just scan forward without seeking.*/
    _end_searched -= 65536 as i32 as i64;
    if _searched >= _end_searched {
        return -(1 as i32) as i64;
    }
    bisect = _end_searched;
    sri = 0 as i32;
    while sri < _nsr {
        let mut gp1: ogg_int64_t = 0;
        let mut gp2_min: ogg_int64_t = 0;
        let mut serialno1: ogg_uint32_t = 0;
        let mut offset1: i64 = 0;
        /*If the granule position is negative, either it's invalid or we'd cause
        overflow.*/
        gp1 = (*_sr.offset(sri as isize)).gp;
        if !(gp1 < 0 as i32 as isize) {
            /*We require some minimum distance between granule positions to make an
             estimate.
            We don't actually know what granule position scheme is being used,
             because we have no idea what kind of stream these came from.
            Therefore we require a minimum spacing between them, with the
             expectation that while bitrates and granule position increments might
             vary locally in quite complex ways, they are globally smooth.*/
            if !((op_granpos_add(&mut gp2_min, gp1, 48000 as i32) < 0 as i32) as i32 as isize != 0)
            {
                offset1 = (*_sr.offset(sri as isize)).offset;
                serialno1 = (*_sr.offset(sri as isize)).serialno;
                srj = sri;
                loop {
                    let fresh2 = srj;
                    srj = srj - 1;
                    if !(fresh2 > 0 as i32) {
                        break;
                    }
                    let mut gp2: ogg_int64_t = 0;
                    let mut offset2: i64 = 0;
                    let mut num: i64 = 0;
                    let mut den: ogg_int64_t = 0;
                    let mut ipart: ogg_int64_t = 0;
                    gp2 = (*_sr.offset(srj as isize)).gp;
                    if gp2 < gp2_min {
                        continue;
                    }
                    /*Oh, and also make sure these came from the same stream.*/
                    if (*_sr.offset(srj as isize)).serialno != serialno1 {
                        continue;
                    }
                    offset2 = (*_sr.offset(srj as isize)).offset;
                    /*For once, we can subtract with impunity.*/
                    den = gp2 - gp1;
                    ipart = gp2 / den;
                    num = offset2 - offset1;
                    if ipart > 0 as i32 as isize && ((offset2 - _searched) / ipart as i64) < num {
                        continue;
                    }
                    offset2 -= ipart as i64 * num;
                    gp2 -= ipart * den;
                    offset2 -= op_rescale64(gp2 as i64, den as i64, num) - _bias as i64;
                    if offset2 < _searched {
                        continue;
                    }
                    bisect = if bisect < offset2 { bisect } else { offset2 };
                    break;
                }
            }
        }
        /*No granule position would satisfy us.*/
        sri += 1
    }
    return if bisect >= _end_searched {
        -(1 as i32) as i64
    } else {
        bisect
    };
}
/*Finds each bitstream link, one at a time, using a bisection search.
This has to begin by knowing the offset of the first link's initial page.*/

unsafe extern "C" fn op_bisect_forward_serialno(
    mut _of: *mut OggOpusFile,
    mut _searched: i64,
    mut _sr: *mut OpusSeekRecord,
    mut _csr: i32,
    mut _serialnos: *mut *mut ogg_uint32_t,
    mut _nserialnos: *mut i32,
    mut _cserialnos: *mut i32,
) -> i32 {
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut links: *mut OggOpusLink = std::ptr::null_mut();
    let mut nlinks: i32 = 0;
    let mut clinks: i32 = 0;
    let mut serialnos: *mut ogg_uint32_t = std::ptr::null_mut();
    let mut nserialnos: i32 = 0;
    let mut total_duration: ogg_int64_t = 0;
    let mut nsr: i32 = 0;
    let mut ret: i32 = 0;
    links = (*_of).links;
    clinks = (*_of).nlinks;
    nlinks = clinks;
    total_duration = 0 as i32 as ogg_int64_t;
    /*We start with one seek record, for the last page in the file.
    We build up a list of records for places we seek to during link
     enumeration.
    This list is kept sorted in reverse order.
    We only care about seek locations that were _not_ in the current link,
     therefore we can add them one at a time to the end of the list as we
     improve the lower bound on the location where the next link starts.*/
    nsr = 1 as i32;
    loop {
        let mut end_searched: i64 = 0;
        let mut bisect: i64 = 0;
        let mut next: i64 = 0;
        let mut last: i64 = 0;
        let mut end_offset: ogg_int64_t = 0;
        let mut end_gp: ogg_int64_t = 0;
        let mut sri: i32 = 0;
        serialnos = *_serialnos;
        nserialnos = *_nserialnos;
        if (nlinks >= clinks) as i32 as isize != 0 {
            if (clinks > 2147483647 as i32 - 1 as i32 >> 1 as i32) as i32 as isize != 0 {
                return -(129 as i32);
            }
            clinks = 2 as i32 * clinks + 1 as i32;
            links = crate::stdlib::realloc(
                links as *mut libc::c_void,
                (::std::mem::size_of::<OggOpusLink>() as usize).wrapping_mul(clinks as usize),
            ) as *mut OggOpusLink;
            if links.is_null() as i32 as isize != 0 {
                return -(129 as i32);
            }
            (*_of).links = links
        }
        /*Invariants:
        We have the headers and serial numbers for the link beginning at 'begin'.
        We have the offset and granule position of the last page in the file
         (potentially not a page we care about).*/
        /*Scan the seek records we already have to save us some bisection.*/
        sri = 0 as i32;
        while sri < nsr {
            if op_lookup_serialno((*_sr.offset(sri as isize)).serialno, serialnos, nserialnos) != 0
            {
                break;
            }
            sri += 1
        }
        /*Is the last page in our current list of serial numbers?*/
        if sri <= 0 as i32 {
            break;
        }
        /*Last page wasn't found.
        We have at least one more link.*/
        last = -(1 as i32) as i64;
        end_searched = (*_sr.offset((sri - 1 as i32) as isize)).search_start;
        next = (*_sr.offset((sri - 1 as i32) as isize)).offset;
        end_gp = -(1 as i32) as ogg_int64_t;
        if sri < nsr {
            _searched =
                (*_sr.offset(sri as isize)).offset + (*_sr.offset(sri as isize)).size as i64;
            if (*_sr.offset(sri as isize)).serialno
                == (*links.offset((nlinks - 1 as i32) as isize)).serialno
            {
                end_gp = (*_sr.offset(sri as isize)).gp;
                end_offset = (*_sr.offset(sri as isize)).offset as ogg_int64_t
            }
        }
        nsr = sri;
        bisect = -(1 as i32) as i64;
        /*If we've already found the end of at least one link, try to pick the
         first bisection point at twice the average link size.
        This is a good choice for files with lots of links that are all about the
         same size.*/
        if nlinks > 1 as i32 {
            let mut last_offset: i64 = 0;
            let mut avg_link_size: i64 = 0;
            let mut upper_limit: i64 = 0;
            last_offset = (*links.offset((nlinks - 1 as i32) as isize)).offset;
            avg_link_size = last_offset / (nlinks - 1 as i32) as i64;
            upper_limit = end_searched - 65536 as i32 as i64 - avg_link_size;
            if (last_offset > _searched - avg_link_size) as i32 as isize != 0
                && (last_offset < upper_limit) as i32 as isize != 0
            {
                bisect = last_offset + avg_link_size;
                if (bisect < upper_limit) as i32 as isize != 0 {
                    bisect += avg_link_size
                }
            }
        }
        /*We guard against garbage separating the last and first pages of two
        links below.*/
        while _searched < end_searched {
            let mut next_bias: opus_int32 = 0;
            /*If we don't have a better estimate, use simple bisection.*/
            if bisect == -(1 as i32) as i64 {
                bisect = _searched + (end_searched - _searched >> 1 as i32)
            }
            /*If we're within OP_CHUNK_SIZE of the start, scan forward.*/
            if bisect - _searched < 65536 as i32 as i64 {
                bisect = _searched
            } else {
                /*Otherwise we're skipping data.
                Forget the end page, if we saw one, as we might miss a later one.*/
                end_gp = -(1 as i32) as ogg_int64_t
            }
            ret = op_seek_helper(_of, bisect);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
            last = op_get_next_page(
                _of,
                &mut og,
                (*_sr.offset((nsr - 1 as i32) as isize)).offset,
            );
            if (last < -(1 as i32) as i64) as i32 as isize != 0 {
                return last as i32;
            }
            next_bias = 0 as i32;
            if last == -(1 as i32) as i64 {
                end_searched = bisect
            } else {
                let mut serialno: ogg_uint32_t = 0;
                let mut gp: ogg_int64_t = 0;
                serialno = ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t;
                gp = ogg_page_granulepos(&mut og as *mut _ as *const ogg_page);
                if op_lookup_serialno(serialno, serialnos, nserialnos) == 0 {
                    end_searched = bisect;
                    next = last;
                    /*In reality we should always have enough room, but be paranoid.*/
                    if (nsr < _csr) as i32 as isize != 0 {
                        (*_sr.offset(nsr as isize)).search_start = bisect;
                        (*_sr.offset(nsr as isize)).offset = last;
                        (*_sr.offset(nsr as isize)).size = ((*_of).offset - last) as opus_int32;
                        (*_sr.offset(nsr as isize)).serialno = serialno;
                        (*_sr.offset(nsr as isize)).gp = gp;
                        nsr += 1
                    }
                } else {
                    _searched = (*_of).offset;
                    next_bias = 65536 as i32;
                    if serialno == (*links.offset((nlinks - 1 as i32) as isize)).serialno {
                        /*This page was from the stream we want, remember it.
                        If it's the last such page in the link, we won't have to go back
                         looking for it later.*/
                        end_gp = gp;
                        end_offset = last as ogg_int64_t
                    }
                }
            }
            bisect = op_predict_link_start(_sr, nsr, _searched, end_searched, next_bias)
        }
        /*Bisection point found.
        Get the final granule position of the previous link, assuming
         op_find_initial_pcm_offset() didn't already determine the link was
         empty.*/
        if ((*links.offset((nlinks - 1 as i32) as isize)).pcm_end == -(1 as i32) as isize) as i32
            as isize
            != 0
        {
            if end_gp == -(1 as i32) as isize {
                /*If we don't know where the end page is, we'll have to seek back and
                look for it, starting from the end of the link.*/
                end_offset = next as ogg_int64_t;
                /*Also forget the last page we read.
                It won't be available after the seek.*/
                last = -(1 as i32) as i64
            }
            ret = op_find_final_pcm_offset(
                _of,
                serialnos,
                nserialnos,
                links.offset(nlinks as isize).offset(-(1 as i32 as isize)),
                end_offset as i64,
                (*links.offset((nlinks - 1 as i32) as isize)).serialno,
                end_gp,
                &mut total_duration,
            );
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        if last != next {
            /*The last page we read was not the first page the next link.
            Move the cursor position to the offset of that first page.
            This only performs an actual seek if the first page of the next link
             does not start at the end of the last page from the current Opus
             stream with a valid granule position.*/
            ret = op_seek_helper(_of, next);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        ret = op_fetch_headers(
            _of,
            &mut (*links.offset(nlinks as isize)).head,
            &mut (*links.offset(nlinks as isize)).tags,
            _serialnos,
            _nserialnos,
            _cserialnos,
            if last != next {
                std::ptr::null_mut()
            } else {
                &mut og
            },
        );
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret;
        }
        (*links.offset(nlinks as isize)).offset = next;
        (*links.offset(nlinks as isize)).data_offset = (*_of).offset;
        (*links.offset(nlinks as isize)).serialno = (*_of).os.serialno as ogg_uint32_t;
        (*links.offset(nlinks as isize)).pcm_end = -(1 as i32) as ogg_int64_t;
        /*This might consume a page from the next link, however the next bisection
        always starts with a seek.*/
        ret = op_find_initial_pcm_offset(_of, links.offset(nlinks as isize), std::ptr::null_mut());
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret;
        }
        (*links.offset(nlinks as isize)).pcm_file_offset = total_duration;
        _searched = (*_of).offset;
        /*Mark the current link count so it can be cleaned up on error.*/
        nlinks += 1;
        (*_of).nlinks = nlinks
    }
    /*Last page is in the starting serialno list, so we've reached the last link.
    Now find the last granule position for it (if we didn't the first time we
     looked at the end of the stream, and if op_find_initial_pcm_offset()
     didn't already determine the link was empty).*/
    if ((*links.offset((nlinks - 1 as i32) as isize)).pcm_end == -(1 as i32) as isize) as i32
        as isize
        != 0
    {
        ret = op_find_final_pcm_offset(
            _of,
            serialnos,
            nserialnos,
            links.offset(nlinks as isize).offset(-(1 as i32 as isize)),
            (*_sr.offset(0 as i32 as isize)).offset,
            (*_sr.offset(0 as i32 as isize)).serialno,
            (*_sr.offset(0 as i32 as isize)).gp,
            &mut total_duration,
        );
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret;
        }
    }
    /*Trim back the links array if necessary.*/
    links = crate::stdlib::realloc(
        links as *mut libc::c_void,
        (::std::mem::size_of::<OggOpusLink>() as usize).wrapping_mul(nlinks as usize),
    ) as *mut OggOpusLink;
    if !links.is_null() as i32 as isize != 0 {
        (*_of).links = links
    }
    /*We also don't need these anymore.*/
    libc::free(*_serialnos as *mut libc::c_void);
    *_serialnos = std::ptr::null_mut();
    *_nserialnos = 0 as i32;
    *_cserialnos = *_nserialnos;
    return 0 as i32;
}

unsafe extern "C" fn op_update_gain(mut _of: *mut OggOpusFile) {
    let mut head: *mut crate::src::opusfile_0_9::src::opusfile::OpusHead = std::ptr::null_mut();
    let mut gain_q8: opus_int32 = 0;
    let mut li: i32 = 0;
    /*If decode isn't ready, then we'll apply the gain when we initialize the
    decoder.*/
    if (*_of).ready_state < 4 as i32 {
        return;
    }
    gain_q8 = (*_of).gain_offset_q8;
    li = if (*_of).seekable != 0 {
        (*_of).cur_link
    } else {
        0 as i32
    };
    head = &mut (*(*_of).links.offset(li as isize)).head;
    /*We don't have to worry about overflow here because the header gain and
    track gain must lie in the range [-32768,32767], and the user-supplied
    offset has been pre-clamped to [-98302,98303].*/
    match (*_of).gain_type {
        3007 => {
            let mut album_gain_q8: i32 = 0;
            album_gain_q8 = 0 as i32;
            opus_tags_get_album_gain(
                &mut (*(*_of).links.offset(li as isize)).tags as *mut _
                    as *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
                &mut album_gain_q8,
            );
            gain_q8 += album_gain_q8;
            gain_q8 += (*head).output_gain
        }
        3008 => {
            let mut track_gain_q8: i32 = 0;
            track_gain_q8 = 0 as i32;
            opus_tags_get_track_gain(
                &mut (*(*_of).links.offset(li as isize)).tags as *mut _
                    as *const crate::src::opusfile_0_9::src::opusfile::OpusTags,
                &mut track_gain_q8,
            );
            gain_q8 += track_gain_q8;
            gain_q8 += (*head).output_gain
        }
        0 => gain_q8 += (*head).output_gain,
        3009 | _ => {}
    }
    gain_q8 = if -(32768 as i32)
        > (if gain_q8 < 32767 as i32 {
            gain_q8
        } else {
            32767 as i32
        }) {
        -(32768 as i32)
    } else if gain_q8 < 32767 as i32 {
        gain_q8
    } else {
        32767 as i32
    };
    crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decoder_ctl(
        (*_of).od,
        4034 as i32,
        gain_q8,
    );
}

unsafe extern "C" fn op_make_decode_ready(mut _of: *mut OggOpusFile) -> i32 {
    let mut head: *const crate::src::opusfile_0_9::src::opusfile::OpusHead = std::ptr::null();
    let mut li: i32 = 0;
    let mut stream_count: i32 = 0;
    let mut coupled_count: i32 = 0;
    let mut channel_count: i32 = 0;
    if (*_of).ready_state > 3 as i32 {
        return 0 as i32;
    }
    if ((*_of).ready_state < 3 as i32) as i32 as isize != 0 {
        return -(129 as i32);
    }
    li = if (*_of).seekable != 0 {
        (*_of).cur_link
    } else {
        0 as i32
    };
    head = &mut (*(*_of).links.offset(li as isize)).head;
    stream_count = (*head).stream_count;
    coupled_count = (*head).coupled_count;
    channel_count = (*head).channel_count;
    /*Check to see if the current decoder is compatible with the current link.*/
    if !(*_of).od.is_null()
        && (*_of).od_stream_count == stream_count
        && (*_of).od_coupled_count == coupled_count
        && (*_of).od_channel_count == channel_count
        && crate::stdlib::memcmp(
            (*_of).od_mapping.as_mut_ptr() as *const libc::c_void,
            (*head).mapping.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<u8>() as usize).wrapping_mul(channel_count as usize),
        ) == 0 as i32
    {
        crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decoder_ctl(
            (*_of).od,
            4028 as i32,
        );
    } else {
        let mut err: i32 = 0;
        crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decoder_destroy(
            (*_of).od,
        );
        (*_of).od =
            crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decoder_create(
                48000 as i32,
                channel_count,
                stream_count,
                coupled_count,
                (*head).mapping.as_ptr(),
                &mut err,
            );
        if (*_of).od.is_null() {
            return -(129 as i32);
        }
        (*_of).od_stream_count = stream_count;
        (*_of).od_coupled_count = coupled_count;
        (*_of).od_channel_count = channel_count;
        crate::stdlib::memcpy(
            (*_of).od_mapping.as_mut_ptr() as *mut libc::c_void,
            (*head).mapping.as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<u8>() as usize).wrapping_mul(channel_count as usize),
        );
    }
    (*_of).ready_state = 4 as i32;
    (*_of).bytes_tracked = 0 as i32 as i64;
    (*_of).samples_tracked = 0 as i32 as ogg_int64_t;
    (*_of).state_channel_count = 0 as i32;
    /*Use the serial number for the PRNG seed to get repeatable output for
    straight play-throughs.*/
    (*_of).dither_seed = (*(*_of).links.offset(li as isize)).serialno;
    op_update_gain(_of);
    return 0 as i32;
}

unsafe extern "C" fn op_open_seekable2_impl(mut _of: *mut OggOpusFile) -> i32 {
    /*64 seek records should be enough for anybody.
    Actually, with a bisection search in a 63-bit range down to OP_CHUNK_SIZE
     granularity, much more than enough.*/
    let mut sr: [OpusSeekRecord; 64] = [OpusSeekRecord {
        search_start: 0,
        offset: 0,
        size: 0,
        serialno: 0,
        gp: 0,
    }; 64];
    let mut data_offset: i64 = 0;
    let mut ret: i32 = 0;
    /*We can seek, so set out learning all about this file.*/
    Some((*_of).callbacks.seek.expect("non-null function pointer"))
        .expect("non-null function pointer")((*_of).stream, 0 as i32 as i64, 2 as i32);
    (*_of).end = Some((*_of).callbacks.tell.expect("non-null function pointer"))
        .expect("non-null function pointer")((*_of).stream);
    (*_of).offset = (*_of).end;
    if ((*_of).end < 0 as i32 as i64) as i32 as isize != 0 {
        return -(128 as i32);
    }
    data_offset = (*(*_of).links.offset(0 as i32 as isize)).data_offset;
    if ((*_of).end < data_offset) as i32 as isize != 0 {
        return -(137 as i32);
    }
    /*Get the offset of the last page of the physical bitstream, or, if we're
    lucky, the last Opus page of the first link, as most Ogg Opus files will
    contain a single logical bitstream.*/
    ret = op_get_prev_page_serial(
        _of,
        sr.as_mut_ptr(),
        (*_of).end,
        (*(*_of).links.offset(0 as i32 as isize)).serialno,
        (*_of).serialnos,
        (*_of).nserialnos,
    );
    if (ret < 0 as i32) as i32 as isize != 0 {
        return ret;
    }
    /*If there's any trailing junk, forget about it.*/
    (*_of).end = sr[0 as i32 as usize].offset + sr[0 as i32 as usize].size as i64;
    if ((*_of).end < data_offset) as i32 as isize != 0 {
        return -(137 as i32);
    }
    /*Now enumerate the bitstream structure.*/
    return op_bisect_forward_serialno(
        _of,
        data_offset,
        sr.as_mut_ptr(),
        (::std::mem::size_of::<[OpusSeekRecord; 64]>() as usize)
            .wrapping_div(::std::mem::size_of::<OpusSeekRecord>() as usize) as i32,
        &mut (*_of).serialnos,
        &mut (*_of).nserialnos,
        &mut (*_of).cserialnos,
    );
}

unsafe extern "C" fn op_open_seekable2(mut _of: *mut OggOpusFile) -> i32 {
    let mut oy_start: ogg_sync_state = ogg_sync_state {
        data: std::ptr::null_mut(),
        storage: 0,
        fill: 0,
        returned: 0,
        unsynced: 0,
        headerbytes: 0,
        bodybytes: 0,
    };
    let mut os_start: ogg_stream_state = ogg_stream_state {
        body_data: std::ptr::null_mut(),
        body_storage: 0,
        body_fill: 0,
        body_returned: 0,
        lacing_vals: std::ptr::null_mut(),
        granule_vals: std::ptr::null_mut(),
        lacing_storage: 0,
        lacing_fill: 0,
        lacing_packet: 0,
        lacing_returned: 0,
        header: [0; 282],
        header_fill: 0,
        e_o_s: 0,
        b_o_s: 0,
        serialno: 0,
        pageno: 0,
        packetno: 0,
        granulepos: 0,
    };
    let mut op_start: *mut ogg_packet = std::ptr::null_mut();
    let mut prev_page_offset: i64 = 0;
    let mut start_offset: i64 = 0;
    let mut start_op_count: i32 = 0;
    let mut ret: i32 = 0;
    /*We're partially open and have a first link header state in storage in _of.
    Save off that stream state so we can come back to it.
    It would be simpler to just dump all this state and seek back to
     links[0].data_offset when we're done.
    But we do the extra work to allow us to seek back to _exactly_ the same
     stream position we're at now.
    This allows, e.g., the HTTP backend to continue reading from the original
     connection (if it's still available), instead of opening a new one.
    This means we can open and start playing a normal Opus file with a single
     link and reasonable packet sizes using only two HTTP requests.*/
    start_op_count = (*_of).op_count;
    /*This is a bit too large to put on the stack unconditionally.*/
    op_start = crate::stdlib::malloc(
        (::std::mem::size_of::<ogg_packet>() as usize).wrapping_mul(start_op_count as usize),
    ) as *mut ogg_packet;
    if op_start.is_null() {
        return -(129 as i32);
    }
    oy_start = (*_of).oy;
    os_start = (*_of).os;
    prev_page_offset = (*_of).prev_page_offset;
    start_offset = (*_of).offset;
    crate::stdlib::memcpy(
        op_start as *mut libc::c_void,
        (*_of).op.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<ogg_packet>() as usize).wrapping_mul(start_op_count as usize),
    );
    ogg_sync_init(&mut (*_of).oy as *mut _ as *mut ogg_sync_state);
    ogg_stream_init(
        &mut (*_of).os as *mut _ as *mut ogg_stream_state,
        -(1 as i32),
    );
    ret = op_open_seekable2_impl(_of);
    /*Restore the old stream state.*/
    ogg_stream_clear(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
    ogg_sync_clear(&mut (*_of).oy as *mut _ as *mut ogg_sync_state);
    (*_of).oy = oy_start;
    (*_of).os = os_start;
    (*_of).offset = start_offset;
    (*_of).op_count = start_op_count;
    crate::stdlib::memcpy(
        (*_of).op.as_mut_ptr() as *mut libc::c_void,
        op_start as *const libc::c_void,
        (::std::mem::size_of::<ogg_packet>() as usize).wrapping_mul(start_op_count as usize),
    );
    libc::free(op_start as *mut libc::c_void);
    (*_of).prev_packet_gp = (*(*_of).links.offset(0 as i32 as isize)).pcm_start;
    (*_of).prev_page_offset = prev_page_offset;
    (*_of).cur_discard_count =
        (*(*_of).links.offset(0 as i32 as isize)).head.pre_skip as opus_int32;
    if (ret < 0 as i32) as i32 as isize != 0 {
        return ret;
    }
    /*And restore the position indicator.*/
    ret = Some((*_of).callbacks.seek.expect("non-null function pointer"))
        .expect("non-null function pointer")((*_of).stream, op_position(_of), 0 as i32);
    return if (ret < 0 as i32) as i32 as isize != 0 {
        -(128 as i32)
    } else {
        0 as i32
    };
}
/*Clear out the current logical bitstream decoder.*/

unsafe extern "C" fn op_decode_clear(mut _of: *mut OggOpusFile) {
    /*We don't actually free the decoder.
    We might be able to re-use it for the next link.*/
    (*_of).op_count = 0 as i32;
    (*_of).od_buffer_size = 0 as i32;
    (*_of).prev_packet_gp = -(1 as i32) as ogg_int64_t;
    (*_of).prev_page_offset = -(1 as i32) as i64;
    if (*_of).seekable == 0 {
        opus_tags_clear(
            &mut (*(*_of).links.offset(0 as i32 as isize)).tags as *mut _
                as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
        );
    }
    (*_of).ready_state = 2 as i32;
}

unsafe extern "C" fn op_clear(mut _of: *mut OggOpusFile) {
    let mut links: *mut OggOpusLink = std::ptr::null_mut();
    libc::free((*_of).od_buffer as *mut libc::c_void);
    if !(*_of).od.is_null() {
        crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decoder_destroy(
            (*_of).od,
        );
    }
    links = (*_of).links;
    if (*_of).seekable == 0 {
        if (*_of).ready_state > 2 as i32 || (*_of).ready_state == 1 as i32 {
            opus_tags_clear(
                &mut (*links.offset(0 as i32 as isize)).tags as *mut _
                    as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
            );
        }
    } else if !links.is_null() as i32 as isize != 0 {
        let mut nlinks: i32 = 0;
        let mut link: i32 = 0;
        nlinks = (*_of).nlinks;
        link = 0 as i32;
        while link < nlinks {
            opus_tags_clear(
                &mut (*links.offset(link as isize)).tags as *mut _
                    as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
            );
            link += 1
        }
    }
    libc::free(links as *mut libc::c_void);
    libc::free((*_of).serialnos as *mut libc::c_void);
    ogg_stream_clear(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
    ogg_sync_clear(&mut (*_of).oy as *mut _ as *mut ogg_sync_state);
    if (*_of).callbacks.close.is_some() {
        Some((*_of).callbacks.close.expect("non-null function pointer"))
            .expect("non-null function pointer")((*_of).stream);
    };
}

unsafe extern "C" fn op_open1(
    mut _of: *mut OggOpusFile,
    mut _stream: *mut libc::c_void,
    mut _cb: *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _initial_data: *const u8,
    mut _initial_bytes: size_t,
) -> i32 {
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut pog: *mut ogg_page = std::ptr::null_mut();
    let mut seekable: i32 = 0;
    let mut ret: i32 = 0;
    crate::stdlib::memset(
        _of as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<OggOpusFile>() as usize,
    );
    if (_initial_bytes > 9223372036854775807 as isize as size_t) as i32 as isize != 0 {
        return -(129 as i32);
    }
    (*_of).end = -(1 as i32) as i64;
    (*_of).stream = _stream;
    (*_of).callbacks = *_cb;
    /*At a minimum, we need to be able to read data.*/
    if (*_of).callbacks.read.is_none() as i32 as isize != 0 {
        return -(128 as i32);
    }
    /*Initialize the framing state.*/
    ogg_sync_init(&mut (*_of).oy as *mut _ as *mut ogg_sync_state);
    /*Perhaps some data was previously read into a buffer for testing against
     other stream types.
    Allow initialization from this previously read data (especially as we may
     be reading from a non-seekable stream).
    This requires copying it into a buffer allocated by ogg_sync_buffer() and
     doesn't support seeking, so this is not a good mechanism to use for
     decoding entire files from RAM.*/
    if _initial_bytes > 0 as i32 as usize {
        let mut buffer: *mut libc::c_char = std::ptr::null_mut();
        buffer = ogg_sync_buffer(
            &mut (*_of).oy as *mut _ as *mut ogg_sync_state,
            _initial_bytes as isize,
        );
        crate::stdlib::memcpy(
            buffer as *mut libc::c_void,
            _initial_data as *const libc::c_void,
            _initial_bytes.wrapping_mul(::std::mem::size_of::<libc::c_char>() as usize),
        );
        ogg_sync_wrote(
            &mut (*_of).oy as *mut _ as *mut ogg_sync_state,
            _initial_bytes as isize,
        );
    }
    /*Can we seek?
    Stevens suggests the seek test is portable.*/
    seekable = ((*_cb).seek.is_some()
        && Some((*_cb).seek.expect("non-null function pointer"))
            .expect("non-null function pointer")(_stream, 0 as i32 as i64, 1 as i32)
            != -(1 as i32)) as i32;
    /*If seek is implemented, tell must also be implemented.*/
    if seekable != 0 {
        let mut pos: i64 = 0;
        if (*_of).callbacks.tell.is_none() as i32 as isize != 0 {
            return -(131 as i32);
        }
        pos = Some((*_of).callbacks.tell.expect("non-null function pointer"))
            .expect("non-null function pointer")((*_of).stream);
        /*If the current position is not equal to the initial bytes consumed,
        absolute seeking will not work.*/
        if (pos != _initial_bytes as i64) as i32 as isize != 0 {
            return -(131 as i32);
        }
    }
    (*_of).seekable = seekable;
    /*Don't seek yet.
    Set up a 'single' (current) logical bitstream entry for partial open.*/
    (*_of).links =
        crate::stdlib::malloc(::std::mem::size_of::<OggOpusLink>() as usize) as *mut OggOpusLink;
    /*The serialno gets filled in later by op_fetch_headers().*/
    ogg_stream_init(
        &mut (*_of).os as *mut _ as *mut ogg_stream_state,
        -(1 as i32),
    );
    pog = std::ptr::null_mut();
    loop {
        /*Fetch all BOS pages, store the Opus header and all seen serial numbers,
        and load subsequent Opus setup headers.*/
        ret = op_fetch_headers(
            _of,
            &mut (*(*_of).links.offset(0 as i32 as isize)).head,
            &mut (*(*_of).links.offset(0 as i32 as isize)).tags,
            &mut (*_of).serialnos,
            &mut (*_of).nserialnos,
            &mut (*_of).cserialnos,
            pog,
        );
        if (ret < 0 as i32) as i32 as isize != 0 {
            break;
        }
        (*_of).nlinks = 1 as i32;
        (*(*_of).links.offset(0 as i32 as isize)).offset = 0 as i32 as i64;
        (*(*_of).links.offset(0 as i32 as isize)).data_offset = (*_of).offset;
        (*(*_of).links.offset(0 as i32 as isize)).pcm_end = -(1 as i32) as ogg_int64_t;
        (*(*_of).links.offset(0 as i32 as isize)).serialno = (*_of).os.serialno as ogg_uint32_t;
        /*Fetch the initial PCM offset.*/
        ret = op_find_initial_pcm_offset(_of, (*_of).links, &mut og);
        if seekable != 0 || (ret <= 0 as i32) as i32 as isize != 0 {
            break;
        }
        /*This link was empty, but we already have the BOS page for the next one in
         og.
        We can't seek, so start processing the next link right now.*/
        opus_tags_clear(
            &mut (*(*_of).links.offset(0 as i32 as isize)).tags as *mut _
                as *mut crate::src::opusfile_0_9::src::opusfile::OpusTags,
        );
        (*_of).nlinks = 0 as i32;
        if seekable == 0 {
            (*_of).cur_link += 1
        }
        pog = &mut og
    }
    if (ret >= 0 as i32) as i32 as isize != 0 {
        (*_of).ready_state = 1 as i32
    }
    return ret;
}

unsafe extern "C" fn op_open2(mut _of: *mut OggOpusFile) -> i32 {
    let mut ret: i32 = 0;
    if (*_of).seekable != 0 {
        (*_of).ready_state = 2 as i32;
        ret = op_open_seekable2(_of)
    } else {
        ret = 0 as i32
    }
    if (ret >= 0 as i32) as i32 as isize != 0 {
        /*We have buffered packets from op_find_initial_pcm_offset().
        Move to OP_INITSET so we can use them.*/
        (*_of).ready_state = 3 as i32;
        ret = op_make_decode_ready(_of);
        if (ret >= 0 as i32) as i32 as isize != 0 {
            return 0 as i32;
        }
    }
    /*Don't auto-close the stream on failure.*/
    (*_of).callbacks.close = None;
    op_clear(_of);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn op_test_callbacks(
    mut _stream: *mut libc::c_void,
    mut _cb: *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _initial_data: *const u8,
    mut _initial_bytes: size_t,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut of: *mut OggOpusFile = std::ptr::null_mut();
    let mut ret: i32 = 0;
    of = crate::stdlib::malloc(::std::mem::size_of::<OggOpusFile>() as usize) as *mut OggOpusFile;
    ret = -(129 as i32);
    if !of.is_null() as i32 as isize != 0 {
        ret = op_open1(of, _stream, _cb, _initial_data, _initial_bytes);
        if (ret >= 0 as i32) as i32 as isize != 0 {
            if !_error.is_null() {
                *_error = 0 as i32
            }
            return of;
        }
        /*Don't auto-close the stream on failure.*/
        (*of).callbacks.close = None;
        op_clear(of);
        libc::free(of as *mut libc::c_void);
    }
    if !_error.is_null() {
        *_error = ret
    }
    return std::ptr::null_mut();
}
#[no_mangle]

pub unsafe extern "C" fn op_open_callbacks(
    mut _stream: *mut libc::c_void,
    mut _cb: *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _initial_data: *const u8,
    mut _initial_bytes: size_t,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut of: *mut OggOpusFile = std::ptr::null_mut();
    of = op_test_callbacks(_stream, _cb, _initial_data, _initial_bytes, _error);
    if !of.is_null() as i32 as isize != 0 {
        let mut ret: i32 = 0;
        ret = op_open2(of);
        if (ret >= 0 as i32) as i32 as isize != 0 {
            return of;
        }
        if !_error.is_null() {
            *_error = ret
        }
        libc::free(of as *mut libc::c_void);
    }
    return std::ptr::null_mut();
}
/*Convenience routine to clean up from failure for the open functions that
create their own streams.*/

unsafe extern "C" fn op_open_close_on_failure(
    mut _stream: *mut libc::c_void,
    mut _cb: *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut of: *mut OggOpusFile = std::ptr::null_mut();
    if _stream.is_null() as i32 as isize != 0 {
        if !_error.is_null() {
            *_error = -(129 as i32)
        }
        return std::ptr::null_mut();
    }
    of = op_open_callbacks(_stream, _cb, std::ptr::null(), 0 as i32 as size_t, _error);
    if of.is_null() as i32 as isize != 0 {
        Some((*_cb).close.expect("non-null function pointer")).expect("non-null function pointer")(
            _stream,
        );
    }
    return of;
}
#[no_mangle]

pub unsafe extern "C" fn op_open_file(
    mut _path: *const libc::c_char,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    return op_open_close_on_failure(
        op_fopen(
            &mut cb as *mut _ as *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
            _path,
            b"rb\x00" as *const u8 as *const libc::c_char,
        ),
        &mut cb,
        _error,
    );
}
#[no_mangle]

pub unsafe extern "C" fn op_open_memory(
    mut _data: *const u8,
    mut _size: size_t,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    return op_open_close_on_failure(
        op_mem_stream_create(
            &mut cb as *mut _ as *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
            _data,
            _size,
        ),
        &mut cb,
        _error,
    );
}
/*Convenience routine to clean up from failure for the open functions that
create their own streams.*/

unsafe extern "C" fn op_test_close_on_failure(
    mut _stream: *mut libc::c_void,
    mut _cb: *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut of: *mut OggOpusFile = std::ptr::null_mut();
    if _stream.is_null() as i32 as isize != 0 {
        if !_error.is_null() {
            *_error = -(129 as i32)
        }
        return std::ptr::null_mut();
    }
    of = op_test_callbacks(_stream, _cb, std::ptr::null(), 0 as i32 as size_t, _error);
    if of.is_null() as i32 as isize != 0 {
        Some((*_cb).close.expect("non-null function pointer")).expect("non-null function pointer")(
            _stream,
        );
    }
    return of;
}
#[no_mangle]

pub unsafe extern "C" fn op_test_file(
    mut _path: *const libc::c_char,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    return op_test_close_on_failure(
        op_fopen(
            &mut cb as *mut _ as *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
            _path,
            b"rb\x00" as *const u8 as *const libc::c_char,
        ),
        &mut cb,
        _error,
    );
}
#[no_mangle]

pub unsafe extern "C" fn op_test_memory(
    mut _data: *const u8,
    mut _size: size_t,
    mut _error: *mut i32,
) -> *mut OggOpusFile {
    let mut cb: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks =
        crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
            read: None,
            seek: None,
            tell: None,
            close: None,
        };
    return op_test_close_on_failure(
        op_mem_stream_create(
            &mut cb as *mut _ as *mut crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
            _data,
            _size,
        ),
        &mut cb,
        _error,
    );
}
#[no_mangle]

pub unsafe extern "C" fn op_test_open(mut _of: *mut OggOpusFile) -> i32 {
    let mut ret: i32 = 0;
    if ((*_of).ready_state != 1 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    ret = op_open2(_of);
    /*op_open2() will clear this structure on failure.
    Reset its contents to prevent double-frees in op_free().*/
    if (ret < 0 as i32) as i32 as isize != 0 {
        crate::stdlib::memset(
            _of as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<OggOpusFile>() as usize,
        );
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn op_free(mut _of: *mut OggOpusFile) {
    if !_of.is_null() as i32 as isize != 0 {
        op_clear(_of);
        libc::free(_of as *mut libc::c_void);
    };
}
#[no_mangle]

pub unsafe extern "C" fn op_seekable(mut _of: *const OggOpusFile) -> i32 {
    return (*_of).seekable;
}
#[no_mangle]

pub unsafe extern "C" fn op_link_count(mut _of: *const OggOpusFile) -> i32 {
    return (*_of).nlinks;
}
#[no_mangle]

pub unsafe extern "C" fn op_serialno(mut _of: *const OggOpusFile, mut _li: i32) -> opus_uint32 {
    if (_li >= (*_of).nlinks) as i32 as isize != 0 {
        _li = (*_of).nlinks - 1 as i32
    }
    if (*_of).seekable == 0 {
        _li = 0 as i32
    }
    return (*(*_of)
        .links
        .offset(if _li < 0 as i32 { (*_of).cur_link } else { _li } as isize))
    .serialno;
}
#[no_mangle]

pub unsafe extern "C" fn op_channel_count(mut _of: *const OggOpusFile, mut _li: i32) -> i32 {
    return (*op_head(_of, _li)).channel_count;
}
#[no_mangle]

pub unsafe extern "C" fn op_raw_total(mut _of: *const OggOpusFile, mut _li: i32) -> i64 {
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0
        || ((*_of).seekable == 0) as i32 as isize != 0
        || (_li >= (*_of).nlinks) as i32 as isize != 0
    {
        return -(131 as i32) as i64;
    }
    if _li < 0 as i32 {
        return (*_of).end;
    }
    return (if _li + 1 as i32 >= (*_of).nlinks {
        (*_of).end
    } else {
        (*(*_of).links.offset((_li + 1 as i32) as isize)).offset
    }) - (if _li > 0 as i32 {
        (*(*_of).links.offset(_li as isize)).offset
    } else {
        0 as i32 as i64
    });
}
#[no_mangle]

pub unsafe extern "C" fn op_pcm_total(mut _of: *const OggOpusFile, mut _li: i32) -> ogg_int64_t {
    let mut links: *mut OggOpusLink = std::ptr::null_mut();
    let mut pcm_total: ogg_int64_t = 0;
    let mut diff: ogg_int64_t = 0 as i32 as ogg_int64_t;
    let mut nlinks: i32 = 0;
    nlinks = (*_of).nlinks;
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0
        || ((*_of).seekable == 0) as i32 as isize != 0
        || (_li >= nlinks) as i32 as isize != 0
    {
        return -(131 as i32) as ogg_int64_t;
    }
    links = (*_of).links;
    /*We verify that the granule position differences are larger than the
    pre-skip and that the total duration does not overflow during link
    enumeration, so we don't have to check here.*/
    pcm_total = 0 as i32 as ogg_int64_t;
    if _li < 0 as i32 {
        pcm_total = (*links.offset((nlinks - 1 as i32) as isize)).pcm_file_offset;
        _li = nlinks - 1 as i32
    }
    return pcm_total + diff - (*links.offset(_li as isize)).head.pre_skip as isize;
}
#[no_mangle]

pub unsafe extern "C" fn op_head(
    mut _of: *const OggOpusFile,
    mut _li: i32,
) -> *const crate::src::opusfile_0_9::src::opusfile::OpusHead {
    if (_li >= (*_of).nlinks) as i32 as isize != 0 {
        _li = (*_of).nlinks - 1 as i32
    }
    if (*_of).seekable == 0 {
        _li = 0 as i32
    }
    return &mut (*(*_of)
        .links
        .offset(if _li < 0 as i32 { (*_of).cur_link } else { _li } as isize))
    .head;
}
#[no_mangle]

pub unsafe extern "C" fn op_tags(
    mut _of: *const OggOpusFile,
    mut _li: i32,
) -> *const crate::src::opusfile_0_9::src::opusfile::OpusTags {
    if (_li >= (*_of).nlinks) as i32 as isize != 0 {
        _li = (*_of).nlinks - 1 as i32
    }
    if (*_of).seekable == 0 {
        if (*_of).ready_state < 3 as i32 && (*_of).ready_state != 1 as i32 {
            return std::ptr::null();
        }
        _li = 0 as i32
    } else if _li < 0 as i32 {
        _li = if (*_of).ready_state >= 3 as i32 {
            (*_of).cur_link
        } else {
            0 as i32
        }
    }
    return &mut (*(*_of).links.offset(_li as isize)).tags;
}
#[no_mangle]

pub unsafe extern "C" fn op_current_link(mut _of: *const OggOpusFile) -> i32 {
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    return (*_of).cur_link;
}
/*Compute an average bitrate given a byte and sample count.
Return: The bitrate in bits per second.*/

unsafe extern "C" fn op_calc_bitrate(mut _bytes: i64, mut _samples: ogg_int64_t) -> opus_int32 {
    /*These rates are absurd, but let's handle them anyway.*/
    if (_bytes
        > (((2 as i32 as isize * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
            | 1 as i32 as isize)
            - (_samples >> 1 as i32))
            / (48000 as i32 * 8 as i32) as isize) as i64) as i32 as isize
        != 0
    {
        let mut den: ogg_int64_t = 0;
        if (_bytes
            / ((2 as i32 * (((1 as i32) << 30 as i32) - 1 as i32) | 1 as i32)
                / (48000 as i32 * 8 as i32)) as i64
            >= _samples as i64) as i32 as isize
            != 0
        {
            return 2 as i32 * (((1 as i32) << 30 as i32) - 1 as i32) | 1 as i32;
        }
        den = _samples / (48000 as i32 * 8 as i32) as isize;
        return ((_bytes + (den >> 1 as i32) as i64) / den as i64) as opus_int32;
    }
    if (_samples <= 0 as i32 as isize) as i32 as isize != 0 {
        return 2 as i32 * (((1 as i32) << 30 as i32) - 1 as i32) | 1 as i32;
    }
    /*This can't actually overflow in normal operation: even with a pre-skip of
     545 2.5 ms frames with 8 streams running at 1282*8+1 bytes per packet
     (1275 byte frames + Opus framing overhead + Ogg lacing values), that all
     produce a single sample of decoded output, we still don't top 45 Mbps.
    The only way to get bitrates larger than that is with excessive Opus
     padding, more encoded streams than output channels, or lots and lots of
     Ogg pages with no packets on them.*/
    return if ((_bytes * 48000 as i32 as i64 * 8 as i32 as i64 + (_samples >> 1 as i32) as i64)
        / _samples as i64)
        < (2 as i32 * (((1 as i32) << 30 as i32) - 1 as i32) | 1 as i32) as i64
    {
        (_bytes * 48000 as i32 as i64 * 8 as i32 as i64 + (_samples >> 1 as i32) as i64)
            / _samples as i64
    } else {
        (2 as i32 * (((1 as i32) << 30 as i32) - 1 as i32) | 1 as i32) as i64
    } as opus_int32;
}
#[no_mangle]

pub unsafe extern "C" fn op_bitrate(mut _of: *const OggOpusFile, mut _li: i32) -> opus_int32 {
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0
        || ((*_of).seekable == 0) as i32 as isize != 0
        || (_li >= (*_of).nlinks) as i32 as isize != 0
    {
        return -(131 as i32);
    }
    return op_calc_bitrate(op_raw_total(_of, _li), op_pcm_total(_of, _li));
}
#[no_mangle]

pub unsafe extern "C" fn op_bitrate_instant(mut _of: *mut OggOpusFile) -> opus_int32 {
    let mut samples_tracked: ogg_int64_t = 0;
    let mut ret: opus_int32 = 0;
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    samples_tracked = (*_of).samples_tracked;
    if (samples_tracked == 0 as i32 as isize) as i32 as isize != 0 {
        return -(1 as i32);
    }
    ret = op_calc_bitrate((*_of).bytes_tracked, samples_tracked);
    (*_of).bytes_tracked = 0 as i32 as i64;
    (*_of).samples_tracked = 0 as i32 as ogg_int64_t;
    return ret;
}
/*Given a serialno, find a link with a corresponding Opus stream, if it exists.
Return: The index of the link to which the page belongs, or a negative number
         if it was not a desired Opus bitstream section.*/

unsafe extern "C" fn op_get_link_from_serialno(
    mut _of: *const OggOpusFile,
    mut _cur_link: i32,
    mut _page_offset: i64,
    mut _serialno: ogg_uint32_t,
) -> i32 {
    let mut links: *const OggOpusLink = std::ptr::null();
    let mut nlinks: i32 = 0;
    let mut li_lo: i32 = 0;
    let mut li_hi: i32 = 0;
    links = (*_of).links;
    nlinks = (*_of).nlinks;
    li_lo = 0 as i32;
    /*Start off by guessing we're just a multiplexed page in the current link.*/
    li_hi = if (_cur_link + 1 as i32) < nlinks
        && _page_offset < (*links.offset((nlinks + 1 as i32) as isize)).offset
    {
        (_cur_link) + 1 as i32
    } else {
        nlinks
    };
    loop {
        if _page_offset >= (*links.offset(_cur_link as isize)).offset {
            li_lo = _cur_link
        } else {
            li_hi = _cur_link
        }
        _cur_link = li_lo + (li_hi - li_lo >> 1 as i32);
        if !(li_hi - li_lo > 1 as i32) {
            break;
        }
    }
    /*We've identified the link that should contain this page.
    Make sure it's a page we care about.*/
    if (*links.offset(_cur_link as isize)).serialno != _serialno {
        return -(1 as i32);
    }
    return _cur_link;
}
/*Fetch and process a page.
This handles the case where we're at a bitstream boundary and dumps the
 decoding machine.
If the decoding machine is unloaded, it loads it.
It also keeps prev_packet_gp up to date (seek and read both use this).
Return: <0) Error, OP_HOLE (lost packet), or OP_EOF.
         0) Got at least one audio data packet.*/

unsafe extern "C" fn op_fetch_and_process_page(
    mut _of: *mut OggOpusFile,
    mut _og: *mut ogg_page,
    mut _page_offset: i64,
    mut _spanp: i32,
    mut _ignore_holes: i32,
) -> i32 {
    let mut links: *mut OggOpusLink = std::ptr::null_mut();
    let mut cur_serialno: ogg_uint32_t = 0;
    let mut seekable: i32 = 0;
    let mut cur_link: i32 = 0;
    let mut ret: i32 = 0;
    /*We shouldn't get here if we have unprocessed packets.*/
    seekable = (*_of).seekable;
    links = (*_of).links;
    cur_link = if seekable != 0 {
        (*_of).cur_link
    } else {
        0 as i32
    };
    cur_serialno = (*links.offset(cur_link as isize)).serialno;
    loop
    /*Handle one page.*/
    {
        let mut og: ogg_page = ogg_page {
            header: std::ptr::null_mut(),
            header_len: 0,
            body: std::ptr::null_mut(),
            body_len: 0,
        };
        /*If we were given a page to use, use it.*/
        if !_og.is_null() {
            og = *_og;
            _og = std::ptr::null_mut()
        } else {
            /*Keep reading until we get a page with the correct serialno.*/
            _page_offset = op_get_next_page(_of, &mut og, (*_of).end)
        }
        /*EOF: Leave uninitialized.*/
        if _page_offset < 0 as i32 as i64 {
            return if _page_offset < -(1 as i32) as i64 {
                _page_offset as i32
            } else {
                -(2 as i32)
            };
        }
        if ((*_of).ready_state >= 3 as i32) as i32 as isize != 0
            && cur_serialno
                != ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t
        {
            /*Two possibilities:
            1) Another stream is multiplexed into this logical section, or*/
            if (ogg_page_bos(&mut og as *mut _ as *const ogg_page) == 0) as i32 as isize != 0 {
                continue;
            }
            /* 2) Our decoding just traversed a bitstream boundary.*/
            if _spanp == 0 {
                return -(2 as i32);
            }
            if ((*_of).ready_state >= 4 as i32) as i32 as isize != 0 {
                op_decode_clear(_of);
            }
        } else {
            /*Bitrate tracking: add the header's bytes here.
            The body bytes are counted when we consume the packets.*/
            (*_of).bytes_tracked += og.header_len as i64
        }
        /*Do we need to load a new machine before submitting the page?
        This is different in the seekable and non-seekable cases.
        In the seekable case, we already have all the header information loaded
         and cached.
        We just initialize the machine with it and continue on our merry way.
        In the non-seekable (streaming) case, we'll only be at a boundary if we
         just left the previous logical bitstream, and we're now nominally at the
         header of the next bitstream.*/
        if ((*_of).ready_state < 3 as i32) as i32 as isize != 0 {
            if seekable != 0 {
                let mut serialno: ogg_uint32_t = 0;
                serialno = ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t;
                /*Match the serialno to bitstream section.*/
                if (*links.offset(cur_link as isize)).serialno != serialno {
                    /*It wasn't a page from the current link.
                    Is it from the next one?*/
                    if ((cur_link + 1 as i32) < (*_of).nlinks
                        && (*links.offset((cur_link + 1 as i32) as isize)).serialno == serialno)
                        as i32 as isize
                        != 0
                    {
                        cur_link += 1
                    } else {
                        let mut new_link: i32 = 0;
                        new_link = op_get_link_from_serialno(_of, cur_link, _page_offset, serialno);
                        /*Not a desired Opus bitstream section.
                        Keep trying.*/
                        if new_link < 0 as i32 {
                            continue;
                        }
                        cur_link = new_link
                    }
                }
                cur_serialno = serialno;
                (*_of).cur_link = cur_link;
                ogg_stream_reset_serialno(
                    &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                    serialno as i32,
                );
                (*_of).ready_state = 3 as i32;
                /*If we're at the start of this link, initialize the granule position
                and pre-skip tracking.*/
                if _page_offset <= (*links.offset(cur_link as isize)).data_offset {
                    (*_of).prev_packet_gp = (*links.offset(cur_link as isize)).pcm_start;
                    (*_of).prev_page_offset = -(1 as i32) as i64;
                    (*_of).cur_discard_count =
                        (*links.offset(cur_link as isize)).head.pre_skip as opus_int32;
                    /*Ignore a hole at the start of a new link (this is common for
                    streams joined in the middle) or after seeking.*/
                    _ignore_holes = 1 as i32
                }
            } else {
                loop {
                    /*We're streaming.
                    Fetch the two header packets, build the info struct.*/
                    ret = op_fetch_headers(
                        _of,
                        &mut (*links.offset(0 as i32 as isize)).head,
                        &mut (*links.offset(0 as i32 as isize)).tags,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        &mut og,
                    );
                    if (ret < 0 as i32) as i32 as isize != 0 {
                        return ret;
                    }
                    /*op_find_initial_pcm_offset() will suppress any initial hole for us,
                    so no need to set _ignore_holes.*/
                    ret = op_find_initial_pcm_offset(_of, links, &mut og);
                    if (ret < 0 as i32) as i32 as isize != 0 {
                        return ret;
                    }
                    cur_serialno = (*_of).os.serialno as ogg_uint32_t;
                    (*(*_of).links.offset(0 as i32 as isize)).serialno = cur_serialno;
                    (*_of).cur_link += 1;
                    if !((ret > 0 as i32) as i32 as isize != 0) {
                        break;
                    }
                }
                /*If we didn't get any packets out of op_find_initial_pcm_offset(),
                keep going (this is possible if end-trimming trimmed them all).*/
                if (*_of).op_count <= 0 as i32 {
                    continue;
                }
                /*Otherwise, we're done.
                TODO: This resets bytes_tracked, which misses the header bytes
                 already processed by op_find_initial_pcm_offset().*/
                ret = op_make_decode_ready(_of);
                if (ret < 0 as i32) as i32 as isize != 0 {
                    return ret;
                }
                return 0 as i32;
            }
        }
        /*The buffered page is the data we want, and we're ready for it.
        Add it to the stream state.*/
        if ((*_of).ready_state == 3 as i32) as i32 as isize != 0 {
            ret = op_make_decode_ready(_of);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        /*Extract all the packets from the current page.*/
        ogg_stream_pagein(
            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
            &mut og as *mut _ as *mut ogg_page,
        );
        if !(((*_of).ready_state >= 4 as i32) as i32 as isize != 0) {
            continue;
        }
        let mut total_duration: opus_int32 = 0;
        let mut durations: [i32; 255] = [0; 255];
        let mut op_count: i32 = 0;
        let mut report_hole: i32 = 0;
        report_hole = 0 as i32;
        total_duration = op_collect_audio_packets(_of, durations.as_mut_ptr());
        if (total_duration < 0 as i32) as i32 as isize != 0 {
            /*libogg reported a hole (a gap in the page sequence numbers).
            Drain the packets from the page anyway.
            If we don't, they'll still be there when we fetch the next page.
            Then, when we go to pull out packets, we might get more than 255,
             which would overrun our packet buffer.*/
            total_duration = op_collect_audio_packets(_of, durations.as_mut_ptr());
            if _ignore_holes == 0 {
                /*Report the hole to the caller after we finish timestamping the
                packets.*/
                report_hole = 1 as i32;
                /*We had lost or damaged pages, so reset our granule position
                 tracking.
                This makes holes behave the same as a small raw seek.
                If the next page is the EOS page, we'll discard it (because we
                 can't perform end trimming properly), and we'll always discard at
                 least 80 ms of audio (to allow decoder state to re-converge).
                We could try to fill in the gap with PLC by looking at timestamps
                 in the non-EOS case, but that's complicated and error prone and we
                 can't rely on the timestamps being valid.*/
                (*_of).prev_packet_gp = -(1 as i32) as ogg_int64_t
            }
        }
        op_count = (*_of).op_count;
        /*If we found at least one audio data packet, compute per-packet granule
        positions for them.*/
        if op_count > 0 as i32 {
            let mut diff: ogg_int64_t = 0;
            let mut prev_packet_gp: ogg_int64_t = 0;
            let mut cur_packet_gp: ogg_int64_t = 0;
            let mut cur_page_gp: ogg_int64_t = 0;
            let mut cur_page_eos: i32 = 0;
            let mut pi: i32 = 0;
            cur_page_gp = (*_of).op[(op_count - 1 as i32) as usize].granulepos;
            cur_page_eos = (*_of).op[(op_count - 1 as i32) as usize].e_o_s as i32;
            prev_packet_gp = (*_of).prev_packet_gp;
            if (prev_packet_gp == -(1 as i32) as isize) as i32 as isize != 0 {
                let mut cur_discard_count: opus_int32 = 0;
                /*This is the first call after a raw seek.
                Try to reconstruct prev_packet_gp from scratch.*/
                if (cur_page_eos != 0) as i32 as isize != 0 {
                    /*If the first page we hit after our seek was the EOS page, and
                     we didn't start from data_offset or before, we don't have
                     enough information to do end-trimming.
                    Proceed to the next link, rather than risk playing back some
                     samples that shouldn't have been played.*/
                    (*_of).op_count = 0 as i32;
                    if report_hole != 0 {
                        return -(3 as i32);
                    }
                    continue;
                } else {
                    /*By default discard 80 ms of data after a seek, unless we seek
                    into the pre-skip region.*/
                    cur_discard_count = 80 as i32 * 48 as i32;
                    cur_page_gp = (*_of).op[(op_count - 1 as i32) as usize].granulepos;
                    /*Try to initialize prev_packet_gp.
                    If the current page had packets but didn't have a granule
                     position, or the granule position it had was too small (both
                     illegal), just use the starting granule position for the link.*/
                    prev_packet_gp = (*links.offset(cur_link as isize)).pcm_start;
                    if (cur_page_gp != -(1 as i32) as isize) as i32 as isize != 0 {
                        op_granpos_add(&mut prev_packet_gp, cur_page_gp, -total_duration);
                    }
                    if (op_granpos_diff(
                        &mut diff,
                        prev_packet_gp,
                        (*links.offset(cur_link as isize)).pcm_start,
                    ) == 0) as i32 as isize
                        != 0
                    {
                        let mut pre_skip: opus_int32 = 0;
                        /*If we start at the beginning of the pre-skip region, or we're
                         at least 80 ms from the end of the pre-skip region, we discard
                         to the end of the pre-skip region.
                        Otherwise, we still use the 80 ms default, which will discard
                         past the end of the pre-skip region.*/
                        pre_skip = (*links.offset(cur_link as isize)).head.pre_skip as opus_int32;
                        if diff >= 0 as i32 as isize
                            && diff
                                <= (if 0 as i32 > pre_skip - 80 as i32 * 48 as i32 {
                                    0 as i32
                                } else {
                                    (pre_skip) - 80 as i32 * 48 as i32
                                }) as isize
                        {
                            cur_discard_count = pre_skip - diff as i32
                        }
                    }
                    (*_of).cur_discard_count = cur_discard_count
                }
            }
            if (cur_page_gp == -(1 as i32) as isize) as i32 as isize != 0 {
                /*This page had completed packets but didn't have a valid granule
                 position.
                This is illegal, but we'll try to handle it by continuing to count
                 forwards from the previous page.*/
                if op_granpos_add(&mut cur_page_gp, prev_packet_gp, total_duration) < 0 as i32 {
                    /*The timestamp for this page overflowed.*/
                    cur_page_gp = (*links.offset(cur_link as isize)).pcm_end
                }
            }
            /*If we hit the last page, handle end-trimming.*/
            if (cur_page_eos != 0) as i32 as isize != 0
                && (op_granpos_diff(&mut diff, cur_page_gp, prev_packet_gp) == 0) as i32 as isize
                    != 0
                && (diff < total_duration as isize) as i32 as isize != 0
            {
                cur_packet_gp = prev_packet_gp;
                pi = 0 as i32;
                while pi < op_count {
                    diff = durations[pi as usize] as isize - diff;
                    /*If we have samples to trim...*/
                    if diff > 0 as i32 as isize {
                        /*If we trimmed the entire packet, stop (the spec says encoders
                        shouldn't do this, but we support it anyway).*/
                        if (diff > durations[pi as usize] as isize) as i32 as isize != 0 {
                            break;
                        }
                        cur_packet_gp = cur_page_gp;
                        /*Move the EOS flag to this packet, if necessary, so we'll trim
                        the samples during decode.*/
                        (*_of).op[pi as usize].e_o_s = 1 as i32 as isize
                    }
                    /*Update the granule position as normal.*/
                    (*_of).op[pi as usize].granulepos = cur_packet_gp;
                    pi += 1
                }
            } else {
                /*Propagate timestamps to earlier packets.
                op_granpos_add(&prev_packet_gp,prev_packet_gp,total_duration)
                 should succeed and give prev_packet_gp==cur_page_gp.
                But we don't bother to check that, as there isn't much we can do
                 if it's not true, and it actually will not be true on the first
                 page after a seek, if there was a continued packet.
                The only thing we guarantee is that the start and end granule
                 positions of the packets are valid, and that they are monotonic
                 within a page.
                They might be completely out of range for this link (we'll check
                 that elsewhere), or non-monotonic between pages.*/
                if (op_granpos_add(&mut prev_packet_gp, cur_page_gp, -total_duration) < 0 as i32)
                    as i32 as isize
                    != 0
                {
                    /*The starting timestamp for the first packet on this page
                     underflowed.
                    This is illegal, but we ignore it.*/
                    prev_packet_gp = 0 as i32 as ogg_int64_t
                }
                pi = 0 as i32;
                while pi < op_count {
                    if (op_granpos_add(&mut cur_packet_gp, cur_page_gp, -total_duration) < 0 as i32)
                        as i32 as isize
                        != 0
                    {
                        /*The start timestamp for this packet underflowed.
                        This is illegal, but we ignore it.*/
                        cur_packet_gp = 0 as i32 as ogg_int64_t
                    }
                    total_duration -= durations[pi as usize];
                    (*_of).op[pi as usize].granulepos = cur_packet_gp;
                    pi += 1
                }
            }
            (*_of).prev_packet_gp = prev_packet_gp;
            (*_of).prev_page_offset = _page_offset;
            op_count = pi;
            (*_of).op_count = op_count
        }
        if report_hole != 0 {
            return -(3 as i32);
        }
        /*If end-trimming didn't trim all the packets, we're done.*/
        if op_count > 0 as i32 {
            return 0 as i32;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn op_raw_seek(mut _of: *mut OggOpusFile, mut _pos: i64) -> i32 {
    let mut ret: i32 = 0;
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    /*Don't dump the decoder state if we can't seek.*/
    if ((*_of).seekable == 0) as i32 as isize != 0 {
        return -(138 as i32);
    }
    if (_pos < 0 as i32 as i64) as i32 as isize != 0 || (_pos > (*_of).end) as i32 as isize != 0 {
        return -(131 as i32);
    }
    /*Clear out any buffered, decoded data.*/
    op_decode_clear(_of);
    (*_of).bytes_tracked = 0 as i32 as i64;
    (*_of).samples_tracked = 0 as i32 as ogg_int64_t;
    ret = op_seek_helper(_of, _pos);
    if (ret < 0 as i32) as i32 as isize != 0 {
        return -(128 as i32);
    }
    ret = op_fetch_and_process_page(
        _of,
        std::ptr::null_mut(),
        -(1 as i32) as i64,
        1 as i32,
        1 as i32,
    );
    /*If we hit EOF, op_fetch_and_process_page() leaves us uninitialized.
    Instead, jump to the end.*/
    if ret == -(2 as i32) {
        let mut cur_link: i32 = 0;
        op_decode_clear(_of);
        cur_link = (*_of).nlinks - 1 as i32;
        (*_of).cur_link = cur_link;
        (*_of).prev_packet_gp = (*(*_of).links.offset(cur_link as isize)).pcm_end;
        (*_of).cur_discard_count = 0 as i32;
        ret = 0 as i32
    }
    return ret;
}
/*Convert a PCM offset relative to the start of the whole stream to a granule
position in an individual link.*/

unsafe extern "C" fn op_get_granulepos(
    mut _of: *const OggOpusFile,
    mut _pcm_offset: ogg_int64_t,
    mut _li: *mut i32,
) -> ogg_int64_t {
    let mut links: *const OggOpusLink = std::ptr::null();
    let mut duration: ogg_int64_t = 0 as i32 as ogg_int64_t;
    let mut pcm_start: ogg_int64_t = 0;
    let mut _pre_skip: opus_int32 = 0;
    let mut nlinks: i32 = 0;
    let mut li_lo: i32 = 0;
    let mut li_hi: i32 = 0;
    nlinks = (*_of).nlinks;
    links = (*_of).links;
    li_lo = 0 as i32;
    li_hi = nlinks;
    loop {
        let mut li: i32 = 0;
        li = li_lo + (li_hi - li_lo >> 1 as i32);
        if (*links.offset(li as isize)).pcm_file_offset <= _pcm_offset {
            li_lo = li
        } else {
            li_hi = li
        }
        if !(li_hi - li_lo > 1 as i32) {
            break;
        }
    }
    _pcm_offset -= (*links.offset(li_lo as isize)).pcm_file_offset;
    pcm_start = (*links.offset(li_lo as isize)).pcm_start;
    _pre_skip = (*links.offset(li_lo as isize)).head.pre_skip as opus_int32;
    duration -= _pre_skip as isize;
    if _pcm_offset >= duration {
        return -(1 as i32) as ogg_int64_t;
    }
    _pcm_offset += _pre_skip as isize;
    if (pcm_start
        > (2 as i32 as isize * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
            | 1 as i32 as isize)
            - _pcm_offset) as i32 as isize
        != 0
    {
        /*Adding this amount to the granule position would overflow the positive
         half of its 64-bit range.
        Since signed overflow is undefined in C, do it in a way the compiler
         isn't allowed to screw up.*/
        _pcm_offset -= (2 as i32 as isize
            * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
            | 1 as i32 as isize)
            - pcm_start
            + 1 as i32 as isize;
        pcm_start = -(2 as i32 as isize
            * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
            | 1 as i32 as isize)
            - 1 as i32 as isize
    }
    pcm_start += _pcm_offset;
    *_li = li_lo;
    return pcm_start;
}
/*A small helper to determine if an Ogg page contains data that continues onto
a subsequent page.*/

unsafe extern "C" fn op_page_continues(mut _og: *const ogg_page) -> i32 {
    let mut nlacing: i32 = 0;
    nlacing = *(*_og).header.offset(26 as i32 as isize) as i32;
    /*This also correctly handles the (unlikely) case of nlacing==0, because
    0!=255.*/
    return (*(*_og)
        .header
        .offset((27 as i32 + nlacing - 1 as i32) as isize) as i32
        == 255 as i32) as i32;
}
/*A small helper to buffer the continued packet data from a page.*/

unsafe extern "C" fn op_buffer_continued_data(mut _of: *mut OggOpusFile, mut _og: *mut ogg_page) {
    let mut op: ogg_packet = ogg_packet {
        packet: std::ptr::null_mut(),
        bytes: 0,
        b_o_s: 0,
        e_o_s: 0,
        granulepos: 0,
        packetno: 0,
    };
    ogg_stream_pagein(
        &mut (*_of).os as *mut _ as *mut ogg_stream_state,
        _og as *mut ogg_page,
    );
    /*Drain any packets that did end on this page (and ignore holes).
    We only care about the continued packet data.*/
    while ogg_stream_packetout(
        &mut (*_of).os as *mut _ as *mut ogg_stream_state,
        &mut op as *mut _ as *mut ogg_packet,
    ) != 0
    {}
}
/*Note: The OP_SMALL_FOOTPRINT #define doesn't (currently) save much code size,
but it's meant to serve as documentation for portions of the seeking
algorithm that are purely optional, to aid others learning from/porting this
code to other contexts.*/
/*#define OP_SMALL_FOOTPRINT (1)*/
/*Search within link _li for the page with the highest granule position
 preceding (or equal to) _target_gp.
There is a danger here: missing pages or incorrect frame number information
 in the bitstream could make our task impossible.
Account for that (and report it as an error condition).*/

unsafe extern "C" fn op_pcm_seek_page(
    mut _of: *mut OggOpusFile,
    mut _target_gp: ogg_int64_t,
    mut _li: i32,
) -> i32 {
    let mut link: *const OggOpusLink = std::ptr::null();
    let mut og: ogg_page = ogg_page {
        header: std::ptr::null_mut(),
        header_len: 0,
        body: std::ptr::null_mut(),
        body_len: 0,
    };
    let mut pcm_pre_skip: ogg_int64_t = 0 as i32 as ogg_int64_t;
    let mut pcm_start: ogg_int64_t = 0;
    let mut pcm_end: ogg_int64_t = 0;
    let mut best_gp: ogg_int64_t = 0;
    let mut diff: ogg_int64_t = 0;
    let mut serialno: ogg_uint32_t = 0;
    let mut _pre_skip: opus_int32 = 0;
    let mut begin: i64 = 0;
    let mut end: i64 = 0;
    let mut boundary: i64 = 0;
    let mut best: i64 = 0;
    let mut best_start: i64 = 0;
    let mut page_offset: i64 = 0;
    let mut d0: i64 = 0;
    let mut d1: i64 = 0;
    let mut d2: i64 = 0;
    let mut force_bisect: i32 = 0;
    let mut buffering: i32 = 0;
    let mut ret: i32 = 0;
    (*_of).bytes_tracked = 0 as i32 as i64;
    (*_of).samples_tracked = 0 as i32 as ogg_int64_t;
    link = (*_of).links.offset(_li as isize);
    pcm_start = (*link).pcm_start;
    best_gp = pcm_start;
    pcm_end = (*link).pcm_end;
    serialno = (*link).serialno;
    begin = (*link).data_offset;
    best_start = begin;
    best = best_start;
    page_offset = -(1 as i32) as i64;
    buffering = 0 as i32;
    /*We discard the first 80 ms of data after a seek, so seek back that much
     farther.
    If we can't, simply seek to the beginning of the link.*/
    if (op_granpos_add(&mut _target_gp, _target_gp, -(80 as i32) * 48 as i32) < 0 as i32) as i32
        as isize
        != 0
        || (op_granpos_cmp(_target_gp, pcm_start) < 0 as i32) as i32 as isize != 0
    {
        _target_gp = pcm_start
    }
    /*Special case seeking to the start of the link.*/
    _pre_skip = (*link).head.pre_skip as opus_int32;
    if op_granpos_cmp(_target_gp, pcm_pre_skip) < 0 as i32 {
        boundary = begin;
        end = boundary
    } else {
        boundary = (*link).end_offset;
        end = boundary;
        /*If we were decoding from this link, we can narrow the range a bit.*/
        if _li == (*_of).cur_link && (*_of).ready_state >= 4 as i32 {
            let mut offset: i64 = 0;
            let mut op_count: i32 = 0;
            op_count = (*_of).op_count;
            /*The only way the offset can be invalid _and_ we can fail the granule
             position checks below is if someone changed the contents of the last
             page since we read it.
            We'd be within our rights to just return OP_EBADLINK in that case, but
             we'll simply ignore the current position instead.*/
            offset = (*_of).offset;
            if op_count > 0 as i32 && (offset <= end) as i32 as isize != 0 {
                let mut gp: ogg_int64_t = 0;
                /*Make sure the timestamp is valid.
                The granule position might be -1 if we collected the packets from a
                 page without a granule position after reporting a hole.*/
                gp = (*_of).op[(op_count - 1 as i32) as usize].granulepos;
                if (gp != -(1 as i32) as isize) as i32 as isize != 0
                    && (op_granpos_cmp(pcm_start, gp) < 0 as i32) as i32 as isize != 0
                    && (op_granpos_cmp(pcm_end, gp) > 0 as i32) as i32 as isize != 0
                {
                    /*We only actually use the current time if either
                    a) We can cut off at least half the range, or
                    b) We're seeking sufficiently close to the current position that
                        it's likely to be informative.
                    Otherwise it appears using the whole link range to estimate the
                     first seek location gives better results, on average.*/
                    if diff < 0 as i32 as isize {
                        if offset - begin >= end - begin >> 1 as i32
                            || diff > -(120 as i32 * 48 as i32 * 1000 as i32) as isize
                        {
                            begin = offset;
                            best = begin;
                            pcm_start = gp;
                            best_gp = pcm_start;
                            /*If we have buffered data from a continued packet, remember the
                             offset of the previous page's start, so that if we do wind up
                             having to seek back here later, we can prime the stream with
                             the continued packet data.
                            With no continued packet, we remember the end of the page.*/
                            best_start = if (*_of).os.body_returned < (*_of).os.body_fill {
                                (*_of).prev_page_offset
                            } else {
                                best
                            };
                            /*If there's completed packets and data in the stream state,
                            prev_page_offset should always be set.*/
                            /*Buffer any continued packet data starting from here.*/
                            buffering = 1 as i32
                        }
                    } else {
                        let mut prev_page_gp: ogg_int64_t = 0 as i32 as ogg_int64_t;
                        /*We might get lucky and already have the packet with the target
                         buffered.
                        Worth checking.
                        For very small files (with all of the data in a single page,
                         generally 1 second or less), we can loop them continuously
                         without seeking at all.*/
                        if op_granpos_cmp(prev_page_gp, _target_gp) <= 0 as i32 {
                            /*Don't call op_decode_clear(), because it will dump our
                            packets.*/
                            (*_of).op_pos = 0 as i32;
                            (*_of).od_buffer_size = 0 as i32;
                            (*_of).prev_packet_gp = prev_page_gp;
                            /*_of->prev_page_offset already points to the right place.*/
                            (*_of).ready_state = 3 as i32;
                            return op_make_decode_ready(_of);
                        }
                        /*No such luck.
                        Check if we can cut off at least half the range, though.*/
                        if offset - begin <= end - begin >> 1 as i32
                            || diff < (120 as i32 * 48 as i32 * 1000 as i32) as isize
                        {
                            /*We really want the page start here, but this will do.*/
                            boundary = offset;
                            end = boundary;
                            pcm_end = gp
                        }
                    }
                }
            }
        }
    }
    /*This code was originally based on the "new search algorithm by HB (Nicholas
     Vinen)" from libvorbisfile.
    It has been modified substantially since.*/
    op_decode_clear(_of);
    if buffering == 0 {
        ogg_stream_reset_serialno(
            &mut (*_of).os as *mut _ as *mut ogg_stream_state,
            serialno as i32,
        );
    }
    (*_of).cur_link = _li;
    (*_of).ready_state = 3 as i32;
    /*Initialize the interval size history.*/
    d0 = end - begin;
    d1 = d0;
    d2 = d1;
    force_bisect = 0 as i32;
    while begin < end {
        let mut bisect: i64 = 0;
        let mut next_boundary: i64 = 0;
        let mut chunk_size: opus_int32 = 0;
        if end - begin < 65536 as i32 as i64 {
            bisect = begin
        } else {
            /*Update the interval size history.*/
            d0 = d1 >> 1 as i32;
            d1 = d2 >> 1 as i32;
            d2 = end - begin >> 1 as i32;
            if force_bisect != 0 {
                bisect = begin + (end - begin >> 1 as i32)
            } else {
                let mut diff2: ogg_int64_t = 0;
                /*Take a (pretty decent) guess.*/
                bisect = begin + op_rescale64(diff as i64, diff2 as i64, end - begin)
                    - 65536 as i32 as i64
            }
            if (bisect - 65536 as i32 as i64) < begin {
                bisect = begin
            }
            force_bisect = 0 as i32
        }
        if bisect != (*_of).offset {
            /*Discard any buffered continued packet data.*/
            if buffering != 0 {
                ogg_stream_reset(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
            }
            buffering = 0 as i32;
            page_offset = -(1 as i32) as i64;
            ret = op_seek_helper(_of, bisect);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        chunk_size = 65536 as i32;
        next_boundary = boundary;
        /*Now scan forward and figure out where we landed.
        In the ideal case, we will see a page with a granule position at or
         before our target, followed by a page with a granule position after our
         target (or the end of the search interval).
        Then we can just drop out and will have all of the data we need with no
         additional seeking.
        If we landed too far before, or after, we'll break out and do another
         bisection.*/
        while begin < end {
            page_offset = op_get_next_page(_of, &mut og, boundary);
            if page_offset < 0 as i32 as i64 {
                if page_offset < -(1 as i32) as i64 {
                    return page_offset as i32;
                }
                /*There are no more pages in our interval from our stream with a valid
                timestamp that start at position bisect or later.*/
                /*If we scanned the whole interval, we're done.*/
                if bisect <= begin + 1 as i32 as i64 {
                    end = begin
                } else {
                    /*Otherwise, back up one chunk.
                    First, discard any data from a continued packet.*/
                    if buffering != 0 {
                        ogg_stream_reset(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
                    }
                    buffering = 0 as i32;
                    bisect = if bisect - chunk_size as i64 > begin {
                        (bisect) - chunk_size as i64
                    } else {
                        begin
                    };
                    ret = op_seek_helper(_of, bisect);
                    if (ret < 0 as i32) as i32 as isize != 0 {
                        return ret;
                    }
                    /*Bump up the chunk size.*/
                    chunk_size = if 2 as i32 * chunk_size < 1024 as i32 * 1024 as i32 {
                        (2 as i32) * chunk_size
                    } else {
                        (1024 as i32) * 1024 as i32
                    };
                    /*If we did find a page from another stream or without a timestamp,
                    don't read past it.*/
                    boundary = next_boundary
                }
            } else {
                let mut gp_0: ogg_int64_t = 0;
                let mut has_packets: i32 = 0;
                /*Save the offset of the first page we found after the seek, regardless
                of the stream it came from or whether or not it has a timestamp.*/
                next_boundary = if page_offset < next_boundary {
                    page_offset
                } else {
                    next_boundary
                };
                if serialno
                    != ogg_page_serialno(&mut og as *mut _ as *const ogg_page) as ogg_uint32_t
                {
                    continue;
                }
                has_packets =
                    (ogg_page_packets(&mut og as *mut _ as *const ogg_page) > 0 as i32) as i32;
                /*Force the gp to -1 (as it should be per spec) if no packets end on
                 this page.
                Otherwise we might get confused when we try to pull out a packet
                 with that timestamp and can't find it.*/
                gp_0 = if has_packets != 0 {
                    ogg_page_granulepos(&mut og as *mut _ as *const ogg_page)
                } else {
                    -(1 as i32) as isize
                };
                if gp_0 == -(1 as i32) as isize {
                    if buffering != 0 {
                        if (has_packets == 0) as i32 as isize != 0 {
                            ogg_stream_pagein(
                                &mut (*_of).os as *mut _ as *mut ogg_stream_state,
                                &mut og as *mut _ as *mut ogg_page,
                            );
                        } else {
                            /*If packets did end on this page, but we still didn't have a
                             valid granule position (in violation of the spec!), stop
                             buffering continued packet data.
                            Otherwise we might continue past the packet we actually
                             wanted.*/
                            ogg_stream_reset(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
                            buffering = 0 as i32
                        }
                    }
                } else if op_granpos_cmp(gp_0, _target_gp) < 0 as i32 {
                    /*We found a page that ends before our target.
                    Advance to the raw offset of the next page.*/
                    begin = (*_of).offset;
                    if (op_granpos_cmp(pcm_start, gp_0) > 0 as i32) as i32 as isize != 0
                        || (op_granpos_cmp(pcm_end, gp_0) < 0 as i32) as i32 as isize != 0
                    {
                        break;
                    }
                    /*Save the byte offset of the end of the page with this granule
                    position.*/
                    best_start = begin;
                    best = best_start;
                    /*Buffer any data from a continued packet, if necessary.
                    This avoids the need to seek back here if the next timestamp we
                     encounter while scanning forward lies after our target.*/
                    if buffering != 0 {
                        ogg_stream_reset(&mut (*_of).os as *mut _ as *mut ogg_stream_state);
                    }
                    if op_page_continues(&mut og) != 0 {
                        op_buffer_continued_data(_of, &mut og);
                        /*If we have a continued packet, remember the offset of this
                         page's start, so that if we do wind up having to seek back here
                         later, we can prime the stream with the continued packet data.
                        With no continued packet, we remember the end of the page.*/
                        best_start = page_offset
                    }
                    /*Then force buffering on, so that if a packet starts (but does not
                    end) on the next page, we still avoid the extra seek back.*/
                    buffering = 1 as i32;
                    pcm_start = gp_0;
                    best_gp = pcm_start;
                    /*If we're more than a second away from our target, break out and
                    do another bisection.*/
                    if diff > 48000 as i32 as isize {
                        break;
                    }
                    /*Otherwise, keep scanning forward (do NOT use begin+1).*/
                    bisect = begin
                } else if bisect <= begin + 1 as i32 as i64 {
                    end = begin
                } else {
                    end = bisect;
                    /*We found a page that ends after our target.*/
                    /*If we scanned the whole interval before we found it, we're done.*/
                    /*In later iterations, don't read past the first page we found.*/
                    boundary = next_boundary;
                    /*If we're not making much progress shrinking the interval size,
                    start forcing straight bisection to limit the worst case.*/
                    force_bisect = (end - begin > d0 * 2 as i32 as i64) as i32;
                    /*Don't let pcm_end get out of range!
                    That could happen with an invalid timestamp.*/
                    if (op_granpos_cmp(pcm_end, gp_0) > 0 as i32) as i32 as isize != 0
                        && (op_granpos_cmp(pcm_start, gp_0) <= 0 as i32) as i32 as isize != 0
                    {
                        pcm_end = gp_0
                    }
                    break;
                }
            }
        }
    }
    /*Found our page.*/
    /*Seek, if necessary.
    If we were buffering data from a continued packet, we should be able to
     continue to scan forward to get the rest of the data (even if
     page_offset==-1).
    Otherwise, we need to seek back to best_start.*/
    if buffering == 0 {
        if best_start != page_offset {
            page_offset = -(1 as i32) as i64;
            ret = op_seek_helper(_of, best_start);
            if (ret < 0 as i32) as i32 as isize != 0 {
                return ret;
            }
        }
        if best_start < best {
            /*Retrieve the page at best_start, if we do not already have it.*/
            if page_offset < 0 as i32 as i64 {
                page_offset = op_get_next_page(_of, &mut og, (*link).end_offset);
                if (page_offset < -(1 as i32) as i64) as i32 as isize != 0 {
                    return page_offset as i32;
                }
                if (page_offset != best_start) as i32 as isize != 0 {
                    return -(137 as i32);
                }
            }
            op_buffer_continued_data(_of, &mut og);
            page_offset = -(1 as i32) as i64
        }
    }
    /*Update prev_packet_gp to allow per-packet granule position assignment.*/
    (*_of).prev_packet_gp = best_gp;
    (*_of).prev_page_offset = best_start;
    ret = op_fetch_and_process_page(
        _of,
        if page_offset < 0 as i32 as i64 {
            std::ptr::null_mut()
        } else {
            &mut og
        },
        page_offset,
        0 as i32,
        1 as i32,
    );
    if (ret < 0 as i32) as i32 as isize != 0 {
        return -(137 as i32);
    }
    /*Verify result.*/
    if (op_granpos_cmp((*_of).prev_packet_gp, _target_gp) > 0 as i32) as i32 as isize != 0 {
        return -(137 as i32);
    }
    /*Our caller will set cur_discard_count to handle pre-roll.*/
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn op_pcm_seek(
    mut _of: *mut OggOpusFile,
    mut _pcm_offset: ogg_int64_t,
) -> i32 {
    let mut link: *const OggOpusLink = std::ptr::null();
    let mut pcm_start: ogg_int64_t = 0;
    let mut target_gp: ogg_int64_t = 0;
    let mut prev_packet_gp: ogg_int64_t = 0;
    let mut skip: ogg_int64_t = 0;
    let mut diff: ogg_int64_t = 0;
    let mut op_count: i32 = 0;
    let mut op_pos: i32 = 0;
    let mut ret: i32 = 0;
    let mut li: i32 = 0;
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    if ((*_of).seekable == 0) as i32 as isize != 0 {
        return -(138 as i32);
    }
    if (_pcm_offset < 0 as i32 as isize) as i32 as isize != 0 {
        return -(131 as i32);
    }
    target_gp = op_get_granulepos(_of, _pcm_offset, &mut li);
    if (target_gp == -(1 as i32) as isize) as i32 as isize != 0 {
        return -(131 as i32);
    }
    link = (*_of).links.offset(li as isize);
    pcm_start = (*link).pcm_start;
    /*For small (90 ms or less) forward seeks within the same link, just decode
     forward.
    This also optimizes the case of seeking to the current position.*/
    if li == (*_of).cur_link && (*_of).ready_state >= 4 as i32 {
        let mut gp: ogg_int64_t = 0;
        gp = (*_of).prev_packet_gp;
        if (gp != -(1 as i32) as isize) as i32 as isize != 0 {
            let mut _nbuffered: i32 = 0;
            _nbuffered = if (*_of).od_buffer_size - (*_of).od_buffer_pos > 0 as i32 {
                ((*_of).od_buffer_size) - (*_of).od_buffer_pos
            } else {
                0 as i32
            };
            /*We do _not_ add cur_discard_count to gp.
            Otherwise the total amount to discard could grow without bound, and it
             would be better just to do a full seek.*/
            if (op_granpos_diff(&mut diff, gp, pcm_start) == 0) as i32 as isize != 0 {
                let mut discard_count: ogg_int64_t = 0;
                discard_count = _pcm_offset - diff;
                /*We use a threshold of 90 ms instead of 80, since 80 ms is the
                 _minimum_ we would have discarded after a full seek.
                Assuming 20 ms frames (the default), we'd discard 90 ms on average.*/
                if discard_count >= 0 as i32 as isize
                    && (discard_count < (90 as i32 * 48 as i32) as isize) as i32 as isize != 0
                {
                    (*_of).cur_discard_count = discard_count as opus_int32;
                    return 0 as i32;
                }
            }
        }
    }
    ret = op_pcm_seek_page(_of, target_gp, li);
    if (ret < 0 as i32) as i32 as isize != 0 {
        return ret;
    }
    /*Now skip samples until we actually get to our target.*/
    /*Figure out where we should skip to.*/
    if _pcm_offset <= (*link).head.pre_skip as isize {
        skip = 0 as i32 as ogg_int64_t
    } else {
        skip = if _pcm_offset - (80 as i32 * 48 as i32) as isize > 0 as i32 as isize {
            (_pcm_offset) - (80 as i32 * 48 as i32) as isize
        } else {
            0 as i32 as isize
        }
    }
    loop
    /*Skip packets until we find one with samples past our skip target.*/
    {
        op_count = (*_of).op_count;
        prev_packet_gp = (*_of).prev_packet_gp;
        op_pos = (*_of).op_pos;
        while op_pos < op_count {
            let mut cur_packet_gp: ogg_int64_t = 0;
            cur_packet_gp = (*_of).op[op_pos as usize].granulepos;
            if (op_granpos_diff(&mut diff, cur_packet_gp, pcm_start) == 0) as i32 as isize != 0
                && diff > skip
            {
                break;
            }
            prev_packet_gp = cur_packet_gp;
            op_pos += 1
        }
        (*_of).prev_packet_gp = prev_packet_gp;
        (*_of).op_pos = op_pos;
        if op_pos < op_count {
            break;
        }
        /*We skipped all the packets on this page.
        Fetch another.*/
        ret = op_fetch_and_process_page(
            _of,
            std::ptr::null_mut(),
            -(1 as i32) as i64,
            0 as i32,
            1 as i32,
        );
        if (ret < 0 as i32) as i32 as isize != 0 {
            return -(137 as i32);
        }
    }
    /*We skipped too far.
    Either the timestamps were illegal or there was a hole in the data.*/
    if diff > skip {
        return -(137 as i32);
    }
    /*TODO: If there are further holes/illegal timestamps, we still won't decode
     to the correct sample.
    However, at least op_pcm_tell() will report the correct value immediately
     after returning.*/
    (*_of).cur_discard_count = (_pcm_offset - diff) as opus_int32;
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn op_raw_tell(mut _of: *const OggOpusFile) -> i64 {
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32) as i64;
    }
    return (*_of).offset;
}
/*Convert a granule position from a given link to a PCM offset relative to the
 start of the whole stream.
For unseekable sources, this gets reset to 0 at the beginning of each link.*/

unsafe extern "C" fn op_get_pcm_offset(
    mut _of: *const OggOpusFile,
    mut _gp: ogg_int64_t,
    mut _li: i32,
) -> ogg_int64_t {
    let mut links: *const OggOpusLink = std::ptr::null();
    let mut pcm_offset: ogg_int64_t = 0;
    links = (*_of).links;
    pcm_offset = (*links.offset(_li as isize)).pcm_file_offset;
    if (*_of).seekable != 0
        && (op_granpos_cmp(_gp, (*links.offset(_li as isize)).pcm_end) > 0 as i32) as i32 as isize
            != 0
    {
        _gp = (*links.offset(_li as isize)).pcm_end
    }
    if (op_granpos_cmp(_gp, (*links.offset(_li as isize)).pcm_start) > 0 as i32) as i32 as isize
        != 0
    {
        let mut delta: ogg_int64_t = 0;
        if (op_granpos_diff(&mut delta, _gp, (*links.offset(_li as isize)).pcm_start) < 0 as i32)
            as i32 as isize
            != 0
        {
            /*This means an unseekable stream claimed to have a page from more than
            2 billion days after we joined.*/
            return 2 as i32 as isize
                * (((1 as i32 as ogg_int64_t) << 62 as i32) - 1 as i32 as isize)
                | 1 as i32 as isize;
        }
        if delta < (*links.offset(_li as isize)).head.pre_skip as isize {
            delta = 0 as i32 as ogg_int64_t
        } else {
            delta -= (*links.offset(_li as isize)).head.pre_skip as isize
        }
        /*In the seekable case, _gp was limited by pcm_end.
        In the unseekable case, pcm_offset should be 0.*/
        pcm_offset += delta
    }
    return pcm_offset;
}
#[no_mangle]

pub unsafe extern "C" fn op_pcm_tell(mut _of: *const OggOpusFile) -> ogg_int64_t {
    let mut gp: ogg_int64_t = 0;
    let mut _nbuffered: i32 = 0;
    let mut li: i32 = 0;
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32) as ogg_int64_t;
    }
    gp = (*_of).prev_packet_gp;
    if gp == -(1 as i32) as isize {
        return 0 as i32 as ogg_int64_t;
    }
    _nbuffered = if (*_of).od_buffer_size - (*_of).od_buffer_pos > 0 as i32 {
        ((*_of).od_buffer_size) - (*_of).od_buffer_pos
    } else {
        0 as i32
    };
    li = if (*_of).seekable != 0 {
        (*_of).cur_link
    } else {
        0 as i32
    };
    if op_granpos_add(&mut gp, gp, (*_of).cur_discard_count) < 0 as i32 {
        gp = (*(*_of).links.offset(li as isize)).pcm_end
    }
    return op_get_pcm_offset(_of, gp, li);
}
#[no_mangle]

pub unsafe extern "C" fn op_set_decode_callback(
    mut _of: *mut OggOpusFile,
    mut _decode_cb: crate::src::opusfile_0_9::src::opusfile::op_decode_cb_func,
    mut _ctx: *mut libc::c_void,
) {
    (*_of).decode_cb = _decode_cb;
    (*_of).decode_cb_ctx = _ctx;
}
#[no_mangle]

pub unsafe extern "C" fn op_set_gain_offset(
    mut _of: *mut OggOpusFile,
    mut _gain_type: i32,
    mut _gain_offset_q8: opus_int32,
) -> i32 {
    if _gain_type != 0 as i32
        && _gain_type != 3007 as i32
        && _gain_type != 3008 as i32
        && _gain_type != 3009 as i32
    {
        return -(131 as i32);
    }
    (*_of).gain_type = _gain_type;
    /*The sum of header gain and track gain lies in the range [-65536,65534].
    These bounds allow the offset to set the final value to anywhere in the
     range [-32768,32767], which is what we'll clamp it to before applying.*/
    (*_of).gain_offset_q8 = if -(98302 as i32)
        > (if _gain_offset_q8 < 98303 as i32 {
            _gain_offset_q8
        } else {
            98303 as i32
        }) {
        -(98302 as i32)
    } else if _gain_offset_q8 < 98303 as i32 {
        _gain_offset_q8
    } else {
        98303 as i32
    };
    op_update_gain(_of);
    return 0 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn op_set_dither_enabled(mut _of: *mut OggOpusFile, mut _enabled: i32) {
    (*_of).dither_disabled = (_enabled == 0) as i32;
    if _enabled == 0 {
        (*_of).dither_mute = 65 as i32
    };
}
/*Allocate the decoder scratch buffer.
This is done lazily, since if the user provides large enough buffers, we'll
 never need it.*/

unsafe extern "C" fn op_init_buffer(mut _of: *mut OggOpusFile) -> i32 {
    let mut nchannels_max: i32 = 0;
    if (*_of).seekable != 0 {
        let mut links: *const OggOpusLink = std::ptr::null();
        let mut nlinks: i32 = 0;
        let mut li: i32 = 0;
        links = (*_of).links;
        nlinks = (*_of).nlinks;
        nchannels_max = 1 as i32;
        li = 0 as i32;
        while li < nlinks {
            nchannels_max = if nchannels_max > (*links.offset(li as isize)).head.channel_count {
                nchannels_max
            } else {
                (*links.offset(li as isize)).head.channel_count
            };
            li += 1
        }
    } else {
        nchannels_max = 8 as i32
    }
    (*_of).od_buffer = crate::stdlib::malloc(
        (::std::mem::size_of::<op_sample>() as usize)
            .wrapping_mul(nchannels_max as usize)
            .wrapping_mul(120 as i32 as usize)
            .wrapping_mul(48 as i32 as usize),
    ) as *mut op_sample;
    if (*_of).od_buffer.is_null() {
        return -(129 as i32);
    }
    return 0 as i32;
}
/*Decode a single packet into the target buffer.*/

unsafe extern "C" fn op_decode(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut op_sample,
    mut _op: *const ogg_packet,
    mut _nsamples: i32,
    mut _nchannels: i32,
) -> i32 {
    let mut ret: i32 = 0;
    /*First we try using the application-provided decode callback.*/
    if (*_of).decode_cb.is_some() {
        ret = Some((*_of).decode_cb.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*_of).decode_cb_ctx,
            (*_of).od,
            _pcm as *mut libc::c_void,
            _op,
            _nsamples,
            _nchannels,
            7040 as i32,
            (*_of).cur_link,
        )
    } else {
        ret = 6720 as i32
    }
    /*If the application didn't want to handle decoding, do it ourselves.*/
    if ret == 6720 as i32 {
        ret = crate::src::opus_1_2_1::src::opus_multistream_decoder::opus_multistream_decode_float(
            (*_of).od,
            (*_op).packet,
            (*_op).bytes as opus_int32,
            _pcm,
            _nsamples,
            0 as i32,
        )
    } else if (ret > 0 as i32) as i32 as isize != 0 {
        return -(136 as i32);
    }
    if (ret < 0 as i32) as i32 as isize != 0 {
        return -(136 as i32);
    }
    return ret;
}
/*If the application returned a positive value other than 0 or
OP_DEC_USE_DEFAULT, fail.*/
/*Read more samples from the stream, using the same API as op_read() or
op_read_float().*/

unsafe extern "C" fn op_read_native(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut op_sample,
    mut _buf_size: i32,
    mut _li: *mut i32,
) -> i32 {
    if ((*_of).ready_state < 2 as i32) as i32 as isize != 0 {
        return -(131 as i32);
    }
    loop
    /*Don't grab another page yet.
    This one might have more packets, or might have buffered data now.*/
    {
        let mut ret: i32 = 0;
        if ((*_of).ready_state >= 4 as i32) as i32 as isize != 0 {
            let mut nchannels: i32 = 0;
            let mut od_buffer_pos: i32 = 0;
            let mut nsamples: i32 = 0;
            let mut op_pos: i32 = 0;
            nchannels = (*(*_of).links.offset(if (*_of).seekable != 0 {
                (*_of).cur_link
            } else {
                0 as i32
            } as isize))
            .head
            .channel_count;
            od_buffer_pos = (*_of).od_buffer_pos;
            nsamples = (*_of).od_buffer_size - od_buffer_pos;
            /*If we have buffered samples, return them.*/
            if nsamples > 0 as i32 {
                if nsamples * nchannels > _buf_size {
                    nsamples = _buf_size / nchannels
                }
                crate::stdlib::memcpy(
                    _pcm as *mut libc::c_void,
                    (*_of)
                        .od_buffer
                        .offset((nchannels * od_buffer_pos) as isize)
                        as *const libc::c_void,
                    (::std::mem::size_of::<op_sample>() as usize)
                        .wrapping_mul(nchannels as usize)
                        .wrapping_mul(nsamples as usize),
                );
                od_buffer_pos += nsamples;
                (*_of).od_buffer_pos = od_buffer_pos;
                if !_li.is_null() {
                    *_li = (*_of).cur_link
                }
                return nsamples;
            }
            /*If we have buffered packets, decode one.*/
            op_pos = (*_of).op_pos;
            if (op_pos < (*_of).op_count) as i32 as isize != 0 {
                let mut pop: *const ogg_packet = std::ptr::null();
                let mut diff: ogg_int64_t = 0;
                let mut cur_discard_count: opus_int32 = 0;
                let mut duration: i32 = 0;
                let mut trimmed_duration: i32 = 0;
                let fresh3 = op_pos;
                op_pos = op_pos + 1;
                pop = (*_of).op.as_mut_ptr().offset(fresh3 as isize);
                (*_of).op_pos = op_pos;
                cur_discard_count = (*_of).cur_discard_count;
                duration = op_get_packet_duration((*pop).packet, (*pop).bytes as i32);
                /*We don't buffer packets with an invalid TOC sequence.*/
                trimmed_duration = duration;
                /*Perform end-trimming.*/
                if ((*pop).e_o_s != 0) as i32 as isize != 0 {
                    if (op_granpos_cmp((*pop).granulepos, (*_of).prev_packet_gp) <= 0 as i32) as i32
                        as isize
                        != 0
                    {
                        trimmed_duration = 0 as i32
                    } else if (op_granpos_diff(&mut diff, (*pop).granulepos, (*_of).prev_packet_gp)
                        == 0) as i32 as isize
                        != 0
                    {
                        trimmed_duration = if diff < trimmed_duration as isize {
                            diff
                        } else {
                            trimmed_duration as isize
                        } as i32
                    }
                }
                (*_of).prev_packet_gp = (*pop).granulepos;
                if (duration * nchannels > _buf_size) as i32 as isize != 0 {
                    let mut buf: *mut op_sample = std::ptr::null_mut();
                    /*If the user's buffer is too small, decode into a scratch buffer.*/
                    buf = (*_of).od_buffer;
                    if buf.is_null() as i32 as isize != 0 {
                        ret = op_init_buffer(_of);
                        if (ret < 0 as i32) as i32 as isize != 0 {
                            return ret;
                        }
                        buf = (*_of).od_buffer
                    }
                    ret = op_decode(_of, buf, pop, duration, nchannels);
                    if (ret < 0 as i32) as i32 as isize != 0 {
                        return ret;
                    }
                    /*Perform pre-skip/pre-roll.*/
                    od_buffer_pos = if trimmed_duration < cur_discard_count {
                        trimmed_duration
                    } else {
                        cur_discard_count
                    };
                    cur_discard_count -= od_buffer_pos;
                    (*_of).cur_discard_count = cur_discard_count;
                    (*_of).od_buffer_pos = od_buffer_pos;
                    (*_of).od_buffer_size = trimmed_duration;
                    /*Update bitrate tracking based on the actual samples we used from
                    what was decoded.*/
                    (*_of).bytes_tracked += (*pop).bytes as i64;
                    (*_of).samples_tracked += (trimmed_duration - od_buffer_pos) as isize
                } else {
                    /*Otherwise decode directly into the user's buffer.*/
                    ret = op_decode(_of, _pcm, pop, duration, nchannels);
                    if (ret < 0 as i32) as i32 as isize != 0 {
                        return ret;
                    }
                    if (trimmed_duration > 0 as i32) as i32 as isize != 0 {
                        /*Perform pre-skip/pre-roll.*/
                        od_buffer_pos = if trimmed_duration < cur_discard_count {
                            trimmed_duration
                        } else {
                            cur_discard_count
                        };
                        cur_discard_count -= od_buffer_pos;
                        (*_of).cur_discard_count = cur_discard_count;
                        trimmed_duration -= od_buffer_pos;
                        if (trimmed_duration > 0 as i32) as i32 as isize != 0
                            && (od_buffer_pos > 0 as i32) as i32 as isize != 0
                        {
                            crate::stdlib::memmove(
                                _pcm as *mut libc::c_void,
                                _pcm.offset((od_buffer_pos * nchannels) as isize)
                                    as *const libc::c_void,
                                (::std::mem::size_of::<op_sample>() as usize)
                                    .wrapping_mul(trimmed_duration as usize)
                                    .wrapping_mul(nchannels as usize),
                            );
                        }
                        /*Update bitrate tracking based on the actual samples we used from
                        what was decoded.*/
                        (*_of).bytes_tracked += (*pop).bytes as i64;
                        (*_of).samples_tracked += trimmed_duration as isize;
                        if (trimmed_duration > 0 as i32) as i32 as isize != 0 {
                            if !_li.is_null() {
                                *_li = (*_of).cur_link
                            }
                            return trimmed_duration;
                        }
                    }
                }
                continue;
            }
        }
        /*Suck in another page.*/
        ret = op_fetch_and_process_page(
            _of,
            std::ptr::null_mut(),
            -(1 as i32) as i64,
            1 as i32,
            0 as i32,
        );
        if (ret == -(2 as i32)) as i32 as isize != 0 {
            if !_li.is_null() {
                *_li = (*_of).cur_link
            }
            return 0 as i32;
        }
        if (ret < 0 as i32) as i32 as isize != 0 {
            return ret;
        }
    }
}
/*Decode some samples and then apply a custom filter to them.
This is used to convert to different output formats.*/

unsafe extern "C" fn op_filter_read_native(
    mut _of: *mut OggOpusFile,
    mut _dst: *mut libc::c_void,
    mut _dst_sz: i32,
    mut _filter: op_read_filter_func,
    mut _li: *mut i32,
) -> i32 {
    let mut ret: i32 = 0;
    /*Ensure we have some decoded samples in our buffer.*/
    ret = op_read_native(_of, std::ptr::null_mut(), 0 as i32, _li);
    /*Now apply the filter to them.*/
    if (ret >= 0 as i32) as i32 as isize != 0
        && ((*_of).ready_state >= 4 as i32) as i32 as isize != 0
    {
        let mut od_buffer_pos: i32 = 0;
        od_buffer_pos = (*_of).od_buffer_pos;
        ret = (*_of).od_buffer_size - od_buffer_pos;
        if (ret > 0 as i32) as i32 as isize != 0 {
            let mut nchannels: i32 = 0;
            nchannels = (*(*_of).links.offset(if (*_of).seekable != 0 {
                (*_of).cur_link
            } else {
                0 as i32
            } as isize))
            .head
            .channel_count;
            ret = Some(_filter.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                _of,
                _dst,
                _dst_sz,
                (*_of)
                    .od_buffer
                    .offset((nchannels * od_buffer_pos) as isize),
                ret,
                nchannels,
            );
            od_buffer_pos += ret;
            (*_of).od_buffer_pos = od_buffer_pos
        }
    }
    return ret;
}
/*Matrices for downmixing from the supported channel counts to stereo.
The matrices with 5 or more channels are normalized to a total volume of 2.0,
 since most mixes sound too quiet if normalized to 1.0 (as there is generally
 little volume in the side/rear channels).*/

static mut OP_STEREO_DOWNMIX: [[[f32; 2]; 8]; 6] = [
    [
        [0.5858f32, 0.0f32],
        [0.4142f32, 0.4142f32],
        [0.0f32, 0.5858f32],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
    ],
    [
        [0.4226f32, 0.0f32],
        [0.0f32, 0.4226f32],
        [0.366f32, 0.2114f32],
        [0.2114f32, 0.336f32],
        [0.; 2],
        [0.; 2],
        [0.; 2],
        [0.; 2],
    ],
    [
        [0.651f32, 0.0f32],
        [0.46f32, 0.46f32],
        [0.0f32, 0.651f32],
        [0.5636f32, 0.3254f32],
        [0.3254f32, 0.5636f32],
        [0.; 2],
        [0.; 2],
        [0.; 2],
    ],
    [
        [0.529f32, 0.0f32],
        [0.3741f32, 0.3741f32],
        [0.0f32, 0.529f32],
        [0.4582f32, 0.2645f32],
        [0.2645f32, 0.4582f32],
        [0.3741f32, 0.3741f32],
        [0.; 2],
        [0.; 2],
    ],
    [
        [0.4553f32, 0.0f32],
        [0.322f32, 0.322f32],
        [0.0f32, 0.4553f32],
        [0.3943f32, 0.2277f32],
        [0.2277f32, 0.3943f32],
        [0.2788f32, 0.2788f32],
        [0.322f32, 0.322f32],
        [0.; 2],
    ],
    [
        [0.3886f32, 0.0f32],
        [0.2748f32, 0.2748f32],
        [0.0f32, 0.3886f32],
        [0.3366f32, 0.1943f32],
        [0.1943f32, 0.3366f32],
        [0.3366f32, 0.1943f32],
        [0.1943f32, 0.3366f32],
        [0.2748f32, 0.2748f32],
    ],
];
/*The dithering code here is adapted from opusdec, part of opus-tools.
It was originally written by Greg Maxwell.*/

unsafe extern "C" fn op_rand(mut _seed: opus_uint32) -> opus_uint32 {
    return _seed
        .wrapping_mul(96314165 as i32 as u32)
        .wrapping_add(907633515 as i32 as u32)
        & 0xffffffff as u32;
}
/*48 kHz noise shaping filter, sd=2.34.*/

static mut OP_FCOEF_B: [f32; 4] = [2.2374f32, -0.7339f32, -0.1251f32, -0.6033f32];

static mut OP_FCOEF_A: [f32; 4] = [0.9030f32, 0.0116f32, -0.5853f32, -0.2571f32];

unsafe extern "C" fn op_float2short_filter(
    mut _of: *mut OggOpusFile,
    mut _dst: *mut libc::c_void,
    mut _dst_sz: i32,
    mut _src: *mut f32,
    mut _nsamples: i32,
    mut _nchannels: i32,
) -> i32 {
    let mut dst: *mut opus_int16 = std::ptr::null_mut();
    let mut ci: i32 = 0;
    let mut i: i32 = 0;
    dst = _dst as *mut opus_int16;
    if (_nsamples * _nchannels > _dst_sz) as i32 as isize != 0 {
        _nsamples = _dst_sz / _nchannels
    }
    if (*_of).state_channel_count != _nchannels {
        ci = 0 as i32;
        while ci < _nchannels {
            (*_of).clip_state[ci as usize] = 0 as i32 as f32;
            ci += 1
        }
    }
    crate::src::opus_1_2_1::src::opus::opus_pcm_soft_clip(
        _src,
        _nsamples,
        _nchannels,
        (*_of).clip_state.as_mut_ptr(),
    );
    if (*_of).dither_disabled != 0 {
        i = 0 as i32;
        while i < _nchannels * _nsamples {
            *dst.offset(i as isize) = ((if -(32768 as i32) as f32
                > (if 32768.0f32 * *_src.offset(i as isize) < 32767 as i32 as f32 {
                    (32768.0f32) * *_src.offset(i as isize)
                } else {
                    32767 as i32 as f32
                }) {
                -(32768 as i32) as f32
            } else {
                (if 32768.0f32 * *_src.offset(i as isize) < 32767 as i32 as f32 {
                    (32768.0f32) * *_src.offset(i as isize)
                } else {
                    32767 as i32 as f32
                })
            }) + (if (if -(32768 as i32) as f32
                > (if 32768.0f32 * *_src.offset(i as isize) < 32767 as i32 as f32 {
                    (32768.0f32) * *_src.offset(i as isize)
                } else {
                    32767 as i32 as f32
                }) {
                -(32768 as i32) as f32
            } else {
                (if 32768.0f32 * *_src.offset(i as isize) < 32767 as i32 as f32 {
                    (32768.0f32) * *_src.offset(i as isize)
                } else {
                    32767 as i32 as f32
                })
            }) < 0 as i32 as f32
            {
                -0.5f32
            } else {
                0.5f32
            })) as i32 as opus_int16;
            i += 1
        }
    } else {
        let mut seed: opus_uint32 = 0;
        let mut mute: i32 = 0;
        seed = (*_of).dither_seed;
        mute = (*_of).dither_mute;
        if (*_of).state_channel_count != _nchannels {
            mute = 65 as i32
        }
        /*In order to avoid replacing digital silence with quiet dither noise, we
        mute if the output has been silent for a while.*/
        if mute > 64 as i32 {
            crate::stdlib::memset(
                (*_of).dither_a.as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                (::std::mem::size_of::<f32>() as usize)
                    .wrapping_mul(4 as i32 as usize)
                    .wrapping_mul(_nchannels as usize),
            );
        }
        i = 0 as i32;
        while i < _nsamples {
            let mut silent: i32 = 0;
            silent = 1 as i32;
            ci = 0 as i32;
            while ci < _nchannels {
                let mut r: f32 = 0.;
                let mut s: f32 = 0.;
                let mut err: f32 = 0.;
                let mut si: i32 = 0;
                let mut j: i32 = 0;
                s = *_src.offset((_nchannels * i + ci) as isize);
                silent &= (s == 0 as i32 as f32) as i32;
                s *= 32753.0f32;
                err = 0 as i32 as f32;
                j = 0 as i32;
                while j < 4 as i32 {
                    err += OP_FCOEF_B[j as usize] * (*_of).dither_b[(ci * 4 as i32 + j) as usize]
                        - OP_FCOEF_A[j as usize] * (*_of).dither_a[(ci * 4 as i32 + j) as usize];
                    j += 1
                }
                j = 3 as i32;
                loop {
                    let fresh4 = j;
                    j = j - 1;
                    if !(fresh4 > 0 as i32) {
                        break;
                    }
                    (*_of).dither_a[(ci * 4 as i32 + j + 1 as i32) as usize] =
                        (*_of).dither_a[(ci * 4 as i32 + j) as usize]
                }
                j = 3 as i32;
                loop {
                    let fresh5 = j;
                    j = j - 1;
                    if !(fresh5 > 0 as i32) {
                        break;
                    }
                    (*_of).dither_b[(ci * 4 as i32 + j + 1 as i32) as usize] =
                        (*_of).dither_b[(ci * 4 as i32 + j) as usize]
                }
                (*_of).dither_a[(ci * 4 as i32) as usize] = err;
                s -= err;
                if mute > 16 as i32 {
                    r = 0 as i32 as f32
                } else {
                    seed = op_rand(seed);
                    r = seed as f32 * (1.0f32 / 0xffffffff as u32 as f32);
                    seed = op_rand(seed);
                    r -= seed as f32 * (1.0f32 / 0xffffffff as u32 as f32)
                }
                /*Clamp in float out of paranoia that the input will be > 96 dBFS and
                wrap if the integer is clamped.*/
                si = ((if -(32768 as i32) as f32
                    > (if s + r < 32767 as i32 as f32 {
                        (s) + r
                    } else {
                        32767 as i32 as f32
                    }) {
                    -(32768 as i32) as f32
                } else {
                    (if s + r < 32767 as i32 as f32 {
                        (s) + r
                    } else {
                        32767 as i32 as f32
                    })
                }) + (if (if -(32768 as i32) as f32
                    > (if s + r < 32767 as i32 as f32 {
                        (s) + r
                    } else {
                        32767 as i32 as f32
                    }) {
                    -(32768 as i32) as f32
                } else {
                    (if s + r < 32767 as i32 as f32 {
                        (s) + r
                    } else {
                        32767 as i32 as f32
                    })
                }) < 0 as i32 as f32
                {
                    -0.5f32
                } else {
                    0.5f32
                })) as i32;
                *dst.offset((_nchannels * i + ci) as isize) = si as opus_int16;
                /*Including clipping in the noise shaping is generally disastrous: the
                 futile effort to restore the clipped energy results in more clipping.
                However, small amounts---at the level which could normally be created
                 by dither and rounding---are harmless and can even reduce clipping
                 somewhat due to the clipping sometimes reducing the dither + rounding
                 error.*/
                (*_of).dither_b[(ci * 4 as i32) as usize] = if mute > 16 as i32 {
                    0 as i32 as f32
                } else if -1.5f32
                    > (if si as f32 - s < 1.5f32 {
                        (si as f32) - s
                    } else {
                        1.5f32
                    })
                {
                    -1.5f32
                } else if si as f32 - s < 1.5f32 {
                    (si as f32) - s
                } else {
                    1.5f32
                };
                ci += 1
            }
            mute += 1;
            if silent == 0 {
                mute = 0 as i32
            }
            i += 1
        }
        (*_of).dither_mute = if mute < 65 as i32 { mute } else { 65 as i32 };
        (*_of).dither_seed = seed
    }
    (*_of).state_channel_count = _nchannels;
    return _nsamples;
}
#[no_mangle]

pub unsafe extern "C" fn op_read(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut opus_int16,
    mut _buf_size: i32,
    mut _li: *mut i32,
) -> i32 {
    return op_filter_read_native(
        _of,
        _pcm as *mut libc::c_void,
        _buf_size,
        Some(
            op_float2short_filter
                as unsafe extern "C" fn(
                    _: *mut OggOpusFile,
                    _: *mut libc::c_void,
                    _: i32,
                    _: *mut f32,
                    _: i32,
                    _: i32,
                ) -> i32,
        ),
        _li,
    );
}
#[no_mangle]

pub unsafe extern "C" fn op_read_float(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut f32,
    mut _buf_size: i32,
    mut _li: *mut i32,
) -> i32 {
    (*_of).state_channel_count = 0 as i32;
    return op_read_native(_of, _pcm, _buf_size, _li);
}

unsafe extern "C" fn op_stereo_filter(
    mut _of: *mut OggOpusFile,
    mut _dst: *mut libc::c_void,
    mut _dst_sz: i32,
    mut _src: *mut op_sample,
    mut _nsamples: i32,
    mut _nchannels: i32,
) -> i32 {
    _nsamples = if _nsamples < _dst_sz >> 1 as i32 {
        _nsamples
    } else {
        (_dst_sz) >> 1 as i32
    };
    if _nchannels == 2 as i32 {
        crate::stdlib::memcpy(
            _dst,
            _src as *const libc::c_void,
            ((_nsamples * 2 as i32) as usize)
                .wrapping_mul(::std::mem::size_of::<op_sample>() as usize),
        );
    } else {
        let mut dst: *mut f32 = std::ptr::null_mut();
        let mut i: i32 = 0;
        dst = _dst as *mut f32;
        if _nchannels == 1 as i32 {
            i = 0 as i32;
            while i < _nsamples {
                let ref mut fresh6 = *dst.offset((2 as i32 * i + 1 as i32) as isize);
                *fresh6 = *_src.offset(i as isize);
                *dst.offset((2 as i32 * i + 0 as i32) as isize) = *fresh6;
                i += 1
            }
        } else {
            i = 0 as i32;
            while i < _nsamples {
                let mut l: f32 = 0.;
                let mut r: f32 = 0.;
                let mut ci: i32 = 0;
                r = 0 as i32 as f32;
                l = r;
                ci = 0 as i32;
                while ci < _nchannels {
                    l += OP_STEREO_DOWNMIX[(_nchannels - 3 as i32) as usize][ci as usize]
                        [0 as i32 as usize]
                        * *_src.offset((_nchannels * i + ci) as isize);
                    r += OP_STEREO_DOWNMIX[(_nchannels - 3 as i32) as usize][ci as usize]
                        [1 as i32 as usize]
                        * *_src.offset((_nchannels * i + ci) as isize);
                    ci += 1
                }
                *dst.offset((2 as i32 * i + 0 as i32) as isize) = l;
                *dst.offset((2 as i32 * i + 1 as i32) as isize) = r;
                i += 1
            }
        }
    }
    return _nsamples;
}

unsafe extern "C" fn op_float2short_stereo_filter(
    mut _of: *mut OggOpusFile,
    mut _dst: *mut libc::c_void,
    mut _dst_sz: i32,
    mut _src: *mut op_sample,
    mut _nsamples: i32,
    mut _nchannels: i32,
) -> i32 {
    let mut dst: *mut opus_int16 = std::ptr::null_mut();
    dst = _dst as *mut opus_int16;
    if _nchannels == 1 as i32 {
        let mut i: i32 = 0;
        _nsamples = op_float2short_filter(
            _of,
            dst as *mut libc::c_void,
            _dst_sz >> 1 as i32,
            _src,
            _nsamples,
            1 as i32,
        );
        i = _nsamples;
        loop {
            let fresh7 = i;
            i = i - 1;
            if !(fresh7 > 0 as i32) {
                break;
            }
            let ref mut fresh8 = *dst.offset((2 as i32 * i + 1 as i32) as isize);
            *fresh8 = *dst.offset(i as isize);
            *dst.offset((2 as i32 * i + 0 as i32) as isize) = *fresh8
        }
    } else {
        if _nchannels > 2 as i32 {
            _nsamples = if _nsamples < _dst_sz >> 1 as i32 {
                _nsamples
            } else {
                (_dst_sz) >> 1 as i32
            };
            _nsamples = op_stereo_filter(
                _of,
                _src as *mut libc::c_void,
                _nsamples * 2 as i32,
                _src,
                _nsamples,
                _nchannels,
            )
        }
        _nsamples = op_float2short_filter(
            _of,
            dst as *mut libc::c_void,
            _dst_sz,
            _src,
            _nsamples,
            2 as i32,
        )
    }
    return _nsamples;
}
#[no_mangle]

pub unsafe extern "C" fn op_read_stereo(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut opus_int16,
    mut _buf_size: i32,
) -> i32 {
    return op_filter_read_native(
        _of,
        _pcm as *mut libc::c_void,
        _buf_size,
        Some(
            op_float2short_stereo_filter
                as unsafe extern "C" fn(
                    _: *mut OggOpusFile,
                    _: *mut libc::c_void,
                    _: i32,
                    _: *mut op_sample,
                    _: i32,
                    _: i32,
                ) -> i32,
        ),
        std::ptr::null_mut(),
    );
}
/* *Sets the packet decode callback function.
If set, this is called once for each packet that needs to be decoded.
This can be used by advanced applications to do additional processing on the
 compressed or uncompressed data.
For example, an application might save the final entropy coder state for
 debugging and testing purposes, or it might apply additional filters
 before the downmixing, dithering, or soft-clipping performed by
 <tt>libopusfile</tt>, so long as these filters do not introduce any
 latency.

A call to this function is no guarantee that the audio will eventually be
 delivered to the application.
<tt>libopusfile</tt> may discard some or all of the decoded audio data
 (i.e., at the beginning or end of a link, or after a seek), however the
 callback is still required to provide all of it.
\param _of        The \c OggOpusFile on which to set the decode callback.
\param _decode_cb The callback function to call.
                  This may be <code>NULL</code> to disable calling the
                   callback.
\param _ctx       The application-provided context pointer to pass to the
                   callback on each call.*/
/* *Gain offset type that indicates that the provided offset is relative to the
 header gain.
This is the default.*/
/* *Gain offset type that indicates that the provided offset is relative to the
R128_ALBUM_GAIN value (if any), in addition to the header gain.*/
/* *Gain offset type that indicates that the provided offset is relative to the
R128_TRACK_GAIN value (if any), in addition to the header gain.*/
/* *Gain offset type that indicates that the provided offset should be used as
the gain directly, without applying any the header or track gains.*/
/* *Sets the gain to be used for decoded output.
By default, the gain in the header is applied with no additional offset.
The total gain (including header gain and/or track gain, if applicable, and
 this offset), will be clamped to [-32768,32767]/256 dB.
This is more than enough to saturate or underflow 16-bit PCM.
\note The new gain will not be applied to any already buffered, decoded
 output.
This means you cannot change it sample-by-sample, as at best it will be
 updated packet-by-packet.
It is meant for setting a target volume level, rather than applying smooth
 fades, etc.
\param _of             The \c OggOpusFile on which to set the gain offset.
\param _gain_type      One of #OP_HEADER_GAIN, #OP_ALBUM_GAIN,
                        #OP_TRACK_GAIN, or #OP_ABSOLUTE_GAIN.
\param _gain_offset_q8 The gain offset to apply, in 1/256ths of a dB.
\return 0 on success or a negative value on error.
\retval #OP_EINVAL The \a _gain_type was unrecognized.*/
/* *Sets whether or not dithering is enabled for 16-bit decoding.
By default, when <tt>libopusfile</tt> is compiled to use floating-point
 internally, calling op_read() or op_read_stereo() will first decode to
 float, and then convert to fixed-point using noise-shaping dithering.
This flag can be used to disable that dithering.
When the application uses op_read_float() or op_read_float_stereo(), or when
 the library has been compiled to decode directly to fixed point, this flag
 has no effect.
\param _of      The \c OggOpusFile on which to enable or disable dithering.
\param _enabled A non-zero value to enable dithering, or 0 to disable it.*/
/* *Reads more samples from the stream.
\note Although \a _buf_size must indicate the total number of values that
 can be stored in \a _pcm, the return value is the number of samples
 <em>per channel</em>.
This is done because
<ol>
<li>The channel count cannot be known a priori (reading more samples might
     advance us into the next link, with a different channel count), so
     \a _buf_size cannot also be in units of samples per channel,</li>
<li>Returning the samples per channel matches the <code>libopus</code> API
     as closely as we're able,</li>
<li>Returning the total number of values instead of samples per channel
     would mean the caller would need a division to compute the samples per
     channel, and might worry about the possibility of getting back samples
     for some channels and not others, and</li>
<li>This approach is relatively fool-proof: if an application passes too
     small a value to \a _buf_size, they will simply get fewer samples back,
     and if they assume the return value is the total number of values, then
     they will simply read too few (rather than reading too many and going
     off the end of the buffer).</li>
</ol>
\param      _of       The \c OggOpusFile from which to read.
\param[out] _pcm      A buffer in which to store the output PCM samples, as
                       signed native-endian 16-bit values at 48&nbsp;kHz
                       with a nominal range of <code>[-32768,32767)</code>.
                      Multiple channels are interleaved using the
                       <a href="http://www.xiph.org/vorbis/doc/Vorbis_I_spec.html#x1-800004.3.9">Vorbis
                       channel ordering</a>.
                      This must have room for at least \a _buf_size values.
\param      _buf_size The number of values that can be stored in \a _pcm.
                      It is recommended that this be large enough for at
                       least 120 ms of data at 48 kHz per channel (5760
                       values per channel).
                      Smaller buffers will simply return less data, possibly
                       consuming more memory to buffer the data internally.
                      <tt>libopusfile</tt> may return less data than
                       requested.
                      If so, there is no guarantee that the remaining data
                       in \a _pcm will be unmodified.
\param[out] _li       The index of the link this data was decoded from.
                      You may pass <code>NULL</code> if you do not need this
                       information.
                      If this function fails (returning a negative value),
                       this parameter is left unset.
\return The number of samples read per channel on success, or a negative
         value on failure.
        The channel count can be retrieved on success by calling
         <code>op_head(_of,*_li)</code>.
        The number of samples returned may be 0 if the buffer was too small
         to store even a single sample for all channels, or if end-of-file
         was reached.
        The list of possible failure codes follows.
        Most of them can only be returned by unseekable, chained streams
         that encounter a new link.
\retval #OP_HOLE          There was a hole in the data, and some samples
                           may have been skipped.
                          Call this function again to continue decoding
                           past the hole.
\retval #OP_EREAD         An underlying read operation failed.
                          This may signal a truncation attack from an
                           <https:> source.
\retval #OP_EFAULT        An internal memory allocation failed.
\retval #OP_EIMPL         An unseekable stream encountered a new link that
                           used a feature that is not implemented, such as
                           an unsupported channel family.
\retval #OP_EINVAL        The stream was only partially open.
\retval #OP_ENOTFORMAT    An unseekable stream encountered a new link that
                           did not have any logical Opus streams in it.
\retval #OP_EBADHEADER    An unseekable stream encountered a new link with a
                           required header packet that was not properly
                           formatted, contained illegal values, or was
                           missing altogether.
\retval #OP_EVERSION      An unseekable stream encountered a new link with
                           an ID header that contained an unrecognized
                           version number.
\retval #OP_EBADPACKET    Failed to properly decode the next packet.
\retval #OP_EBADLINK      We failed to find data we had seen before.
\retval #OP_EBADTIMESTAMP An unseekable stream encountered a new link with
                           a starting timestamp that failed basic validity
                           checks.*/
/* *Reads more samples from the stream.
\note Although \a _buf_size must indicate the total number of values that
 can be stored in \a _pcm, the return value is the number of samples
 <em>per channel</em>.
<ol>
<li>The channel count cannot be known a priori (reading more samples might
     advance us into the next link, with a different channel count), so
     \a _buf_size cannot also be in units of samples per channel,</li>
<li>Returning the samples per channel matches the <code>libopus</code> API
     as closely as we're able,</li>
<li>Returning the total number of values instead of samples per channel
     would mean the caller would need a division to compute the samples per
     channel, and might worry about the possibility of getting back samples
     for some channels and not others, and</li>
<li>This approach is relatively fool-proof: if an application passes too
     small a value to \a _buf_size, they will simply get fewer samples back,
     and if they assume the return value is the total number of values, then
     they will simply read too few (rather than reading too many and going
     off the end of the buffer).</li>
</ol>
\param      _of       The \c OggOpusFile from which to read.
\param[out] _pcm      A buffer in which to store the output PCM samples as
                       signed floats at 48&nbsp;kHz with a nominal range of
                       <code>[-1.0,1.0]</code>.
                      Multiple channels are interleaved using the
                       <a href="http://www.xiph.org/vorbis/doc/Vorbis_I_spec.html#x1-800004.3.9">Vorbis
                       channel ordering</a>.
                      This must have room for at least \a _buf_size floats.
\param      _buf_size The number of floats that can be stored in \a _pcm.
                      It is recommended that this be large enough for at
                       least 120 ms of data at 48 kHz per channel (5760
                       samples per channel).
                      Smaller buffers will simply return less data, possibly
                       consuming more memory to buffer the data internally.
                      If less than \a _buf_size values are returned,
                       <tt>libopusfile</tt> makes no guarantee that the
                       remaining data in \a _pcm will be unmodified.
\param[out] _li       The index of the link this data was decoded from.
                      You may pass <code>NULL</code> if you do not need this
                       information.
                      If this function fails (returning a negative value),
                       this parameter is left unset.
\return The number of samples read per channel on success, or a negative
         value on failure.
        The channel count can be retrieved on success by calling
         <code>op_head(_of,*_li)</code>.
        The number of samples returned may be 0 if the buffer was too small
         to store even a single sample for all channels, or if end-of-file
         was reached.
        The list of possible failure codes follows.
        Most of them can only be returned by unseekable, chained streams
         that encounter a new link.
\retval #OP_HOLE          There was a hole in the data, and some samples
                           may have been skipped.
                          Call this function again to continue decoding
                           past the hole.
\retval #OP_EREAD         An underlying read operation failed.
                          This may signal a truncation attack from an
                           <https:> source.
\retval #OP_EFAULT        An internal memory allocation failed.
\retval #OP_EIMPL         An unseekable stream encountered a new link that
                           used a feature that is not implemented, such as
                           an unsupported channel family.
\retval #OP_EINVAL        The stream was only partially open.
\retval #OP_ENOTFORMAT    An unseekable stream encountered a new link that
                           did not have any logical Opus streams in it.
\retval #OP_EBADHEADER    An unseekable stream encountered a new link with a
                           required header packet that was not properly
                           formatted, contained illegal values, or was
                           missing altogether.
\retval #OP_EVERSION      An unseekable stream encountered a new link with
                           an ID header that contained an unrecognized
                           version number.
\retval #OP_EBADPACKET    Failed to properly decode the next packet.
\retval #OP_EBADLINK      We failed to find data we had seen before.
\retval #OP_EBADTIMESTAMP An unseekable stream encountered a new link with
                           a starting timestamp that failed basic validity
                           checks.*/
/* *Reads more samples from the stream and downmixes to stereo, if necessary.
This function is intended for simple players that want a uniform output
 format, even if the channel count changes between links in a chained
 stream.
\note \a _buf_size indicates the total number of values that can be stored
 in \a _pcm, while the return value is the number of samples <em>per
 channel</em>, even though the channel count is known, for consistency with
 op_read().
\param      _of       The \c OggOpusFile from which to read.
\param[out] _pcm      A buffer in which to store the output PCM samples, as
                       signed native-endian 16-bit values at 48&nbsp;kHz
                       with a nominal range of <code>[-32768,32767)</code>.
                      The left and right channels are interleaved in the
                       buffer.
                      This must have room for at least \a _buf_size values.
\param      _buf_size The number of values that can be stored in \a _pcm.
                      It is recommended that this be large enough for at
                       least 120 ms of data at 48 kHz per channel (11520
                       values total).
                      Smaller buffers will simply return less data, possibly
                       consuming more memory to buffer the data internally.
                      If less than \a _buf_size values are returned,
                       <tt>libopusfile</tt> makes no guarantee that the
                       remaining data in \a _pcm will be unmodified.
\return The number of samples read per channel on success, or a negative
         value on failure.
        The number of samples returned may be 0 if the buffer was too small
         to store even a single sample for both channels, or if end-of-file
         was reached.
        The list of possible failure codes follows.
        Most of them can only be returned by unseekable, chained streams
         that encounter a new link.
\retval #OP_HOLE          There was a hole in the data, and some samples
                           may have been skipped.
                          Call this function again to continue decoding
                           past the hole.
\retval #OP_EREAD         An underlying read operation failed.
                          This may signal a truncation attack from an
                           <https:> source.
\retval #OP_EFAULT        An internal memory allocation failed.
\retval #OP_EIMPL         An unseekable stream encountered a new link that
                           used a feature that is not implemented, such as
                           an unsupported channel family.
\retval #OP_EINVAL        The stream was only partially open.
\retval #OP_ENOTFORMAT    An unseekable stream encountered a new link that
                           did not have any logical Opus streams in it.
\retval #OP_EBADHEADER    An unseekable stream encountered a new link with a
                           required header packet that was not properly
                           formatted, contained illegal values, or was
                           missing altogether.
\retval #OP_EVERSION      An unseekable stream encountered a new link with
                           an ID header that contained an unrecognized
                           version number.
\retval #OP_EBADPACKET    Failed to properly decode the next packet.
\retval #OP_EBADLINK      We failed to find data we had seen before.
\retval #OP_EBADTIMESTAMP An unseekable stream encountered a new link with
                           a starting timestamp that failed basic validity
                           checks.*/
/* *Reads more samples from the stream and downmixes to stereo, if necessary.
This function is intended for simple players that want a uniform output
 format, even if the channel count changes between links in a chained
 stream.
\note \a _buf_size indicates the total number of values that can be stored
 in \a _pcm, while the return value is the number of samples <em>per
 channel</em>, even though the channel count is known, for consistency with
 op_read_float().
\param      _of       The \c OggOpusFile from which to read.
\param[out] _pcm      A buffer in which to store the output PCM samples, as
                       signed floats at 48&nbsp;kHz with a nominal range of
                       <code>[-1.0,1.0]</code>.
                      The left and right channels are interleaved in the
                       buffer.
                      This must have room for at least \a _buf_size values.
\param      _buf_size The number of values that can be stored in \a _pcm.
                      It is recommended that this be large enough for at
                       least 120 ms of data at 48 kHz per channel (11520
                       values total).
                      Smaller buffers will simply return less data, possibly
                       consuming more memory to buffer the data internally.
                      If less than \a _buf_size values are returned,
                       <tt>libopusfile</tt> makes no guarantee that the
                       remaining data in \a _pcm will be unmodified.
\return The number of samples read per channel on success, or a negative
         value on failure.
        The number of samples returned may be 0 if the buffer was too small
         to store even a single sample for both channels, or if end-of-file
         was reached.
        The list of possible failure codes follows.
        Most of them can only be returned by unseekable, chained streams
         that encounter a new link.
\retval #OP_HOLE          There was a hole in the data, and some samples
                           may have been skipped.
                          Call this function again to continue decoding
                           past the hole.
\retval #OP_EREAD         An underlying read operation failed.
                          This may signal a truncation attack from an
                           <https:> source.
\retval #OP_EFAULT        An internal memory allocation failed.
\retval #OP_EIMPL         An unseekable stream encountered a new link that
                           used a feature that is not implemented, such as
                           an unsupported channel family.
\retval #OP_EINVAL        The stream was only partially open.
\retval #OP_ENOTFORMAT    An unseekable stream encountered a new link that
                           that did not have any logical Opus streams in it.
\retval #OP_EBADHEADER    An unseekable stream encountered a new link with a
                           required header packet that was not properly
                           formatted, contained illegal values, or was
                           missing altogether.
\retval #OP_EVERSION      An unseekable stream encountered a new link with
                           an ID header that contained an unrecognized
                           version number.
\retval #OP_EBADPACKET    Failed to properly decode the next packet.
\retval #OP_EBADLINK      We failed to find data we had seen before.
\retval #OP_EBADTIMESTAMP An unseekable stream encountered a new link with
                           a starting timestamp that failed basic validity
                           checks.*/
#[no_mangle]

pub unsafe extern "C" fn op_read_float_stereo(
    mut _of: *mut OggOpusFile,
    mut _pcm: *mut f32,
    mut _buf_size: i32,
) -> i32 {
    (*_of).state_channel_count = 0 as i32;
    return op_filter_read_native(
        _of,
        _pcm as *mut libc::c_void,
        _buf_size,
        Some(
            op_stereo_filter
                as unsafe extern "C" fn(
                    _: *mut OggOpusFile,
                    _: *mut libc::c_void,
                    _: i32,
                    _: *mut op_sample,
                    _: i32,
                    _: i32,
                ) -> i32,
        ),
        std::ptr::null_mut(),
    );
}

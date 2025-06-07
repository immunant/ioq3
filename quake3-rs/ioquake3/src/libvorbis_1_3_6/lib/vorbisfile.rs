// =============== BEGIN vorbisfile_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov_callbacks {
    pub read_func: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: crate::stddef_h::size_t,
            _: crate::stddef_h::size_t,
            _: *mut libc::c_void,
        ) -> crate::stddef_h::size_t,
    >,
    pub seek_func: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: crate::config_types_h::ogg_int64_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub close_func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
    pub tell_func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OggVorbis_File {
    pub datasource: *mut libc::c_void,
    pub seekable: libc::c_int,
    pub offset: crate::config_types_h::ogg_int64_t,
    pub end: crate::config_types_h::ogg_int64_t,
    pub oy: crate::ogg_h::ogg_sync_state,
    pub links: libc::c_int,
    pub offsets: *mut crate::config_types_h::ogg_int64_t,
    pub dataoffsets: *mut crate::config_types_h::ogg_int64_t,
    pub serialnos: *mut libc::c_long,
    pub pcmlengths: *mut crate::config_types_h::ogg_int64_t,
    pub vi: *mut crate::codec_h::vorbis_info,
    pub vc: *mut crate::codec_h::vorbis_comment,
    pub pcm_offset: crate::config_types_h::ogg_int64_t,
    pub ready_state: libc::c_int,
    pub current_serialno: libc::c_long,
    pub current_link: libc::c_int,
    pub bittrack: libc::c_double,
    pub samptrack: libc::c_double,
    pub os: crate::ogg_h::ogg_stream_state,
    pub vd: crate::codec_h::vorbis_dsp_state,
    pub vb: crate::codec_h::vorbis_block,
    pub callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks,
}
use ::libc;

pub mod os_h {

    /* *******************************************************************
    *                                                                  *
    * THIS FILE IS PART OF THE OggVorbis SOFTWARE CODEC SOURCE CODE.   *
    * USE, DISTRIBUTION AND REPRODUCTION OF THIS LIBRARY SOURCE IS     *
    * GOVERNED BY A BSD-STYLE SOURCE LICENSE INCLUDED WITH THIS SOURCE *
    * IN 'COPYING'. PLEASE READ THESE TERMS BEFORE DISTRIBUTING.       *
    *                                                                  *
    * THE OggVorbis SOURCE CODE IS (C) COPYRIGHT 1994-2015             *
    * by the Xiph.Org Foundation http://www.xiph.org/                  *
    *                                                                  *
    ********************************************************************

    function: #ifdef jail to whip a few platforms into the UNIX ideal.

    ********************************************************************/
    /* Special i386 GCC implementation */
    /* Special i386 GCC implementation */
    /* MSVC inline assembly. 32 bit only; inline ASM isn't implemented in the
     * 64 bit compiler and doesn't work on arm. */
    /* Special MSVC 32 bit implementation */
    /* Optimized code path for x86_64 builds. Uses SSE2 intrinsics. This can be
    done safely because all x86_64 CPUs supports SSE2. */
    #[inline]

    pub unsafe extern "C" fn vorbis_fpu_setround(mut _fpu: *mut crate::os_h::vorbis_fpu_control) {}
    #[inline]

    pub unsafe extern "C" fn vorbis_ftoi(mut f: libc::c_double) -> libc::c_int {
        return _mm_cvtsd_si32(_mm_load_sd(&mut f));
    }
    #[inline]

    pub unsafe extern "C" fn vorbis_fpu_restore(mut _fpu: crate::os_h::vorbis_fpu_control) {}

    use ::std::arch::x86_64::_mm_cvtsd_si32;
    use ::std::arch::x86_64::_mm_load_sd;
    /* _OS_H */
    /* default implementation */
    /* If no special implementation was found for the current compiler / platform,
    use the default implementation here: */
    /* Special MSVC x64 implementation */
}

pub use crate::stddef_h::size_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_comment;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::config_types_h::ogg_int16_t;
pub use crate::config_types_h::ogg_int32_t;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::emmintrin_h::__mm_load_sd_struct;
pub use crate::ogg_h::ogg_packet;
pub use crate::ogg_h::ogg_page;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
pub use crate::ogg_h::oggpack_buffer;
pub use crate::os_h::vorbis_fpu_control;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_bos;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_continued;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_eos;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos;
pub use crate::src::libogg_1_3_3::src::framing::ogg_page_serialno;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_clear;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_init;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_packetpeek;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_reset;
pub use crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_buffer;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_clear;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_init;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_pageseek;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_reset;
pub use crate::src::libogg_1_3_3::src::framing::ogg_sync_wrote;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_block_clear;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_block_init;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_dsp_clear;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_blockin;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_init;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_lapout;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_read;
pub use crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_restart;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_clear;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_init;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_info_clear;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_info_init;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_synthesis_headerin;
pub use crate::src::libvorbis_1_3_6::lib::info::vorbis_synthesis_idheader;
pub use crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_packet_blocksize;
pub use crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis;
pub use crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate;
pub use crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p;
pub use crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_trackonly;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::os_h::vorbis_fpu_restore;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::os_h::vorbis_fpu_setround;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::os_h::vorbis_ftoi;

extern "C" {
    #[no_mangle]
    pub fn vorbis_window(
        v: *mut crate::codec_h::vorbis_dsp_state,
        W: libc::c_int,
    ) -> *const libc::c_float;
}
/* a smaller read size is needed for low-rate streaming. */

unsafe extern "C" fn _get_data(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_long {
    *::libc::__errno_location() = 0 as libc::c_int;
    if (*vf).callbacks.read_func.is_none() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if !(*vf).datasource.is_null() {
        let mut buffer: *mut libc::c_char = crate::src::libogg_1_3_3::src::framing::ogg_sync_buffer(
            &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
            2048 as libc::c_int as libc::c_long,
        );
        let mut bytes: libc::c_long = (*vf)
            .callbacks
            .read_func
            .expect("non-null function pointer")(
            buffer as *mut libc::c_void,
            1 as libc::c_int as crate::stddef_h::size_t,
            2048 as libc::c_int as crate::stddef_h::size_t,
            (*vf).datasource,
        ) as libc::c_long;
        if bytes > 0 as libc::c_int as libc::c_long {
            crate::src::libogg_1_3_3::src::framing::ogg_sync_wrote(
                &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
                bytes,
            );
        }
        if bytes == 0 as libc::c_int as libc::c_long && *::libc::__errno_location() != 0 {
            return -(1 as libc::c_int) as libc::c_long;
        }
        return bytes;
    } else {
        return 0 as libc::c_int as libc::c_long;
    };
}
/* save a tiny smidge of verbosity to make the code more readable */

unsafe extern "C" fn _seek_helper(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut offset: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    if !(*vf).datasource.is_null() {
        /* only seek if the file position isn't already there */
        if (*vf).offset != offset {
            if (*vf).callbacks.seek_func.is_none()
                || (*vf)
                    .callbacks
                    .seek_func
                    .expect("non-null function pointer")(
                    (*vf).datasource, offset, 0 as libc::c_int
                ) == -(1 as libc::c_int)
            {
                return -(128 as libc::c_int);
            }
            (*vf).offset = offset;
            crate::src::libogg_1_3_3::src::framing::ogg_sync_reset(
                &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
            );
        }
    } else {
        /* shouldn't happen unless someone writes a broken callback */
        return -(129 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* The read/seek functions track absolute position within the stream */
/* from the head of the stream, get the next page.  boundary specifies
if the function is allowed to fetch more data from the stream (and
how much) or only use internally buffered data.

boundary: -1) unbounded search
           0) read no additional data; use cached only
           n) search for a new page beginning for n bytes

return:   <0) did not find a page (OV_FALSE, OV_EOF, OV_EREAD)
           n) found a page at absolute offset n */

unsafe extern "C" fn _get_next_page(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut og: *mut crate::ogg_h::ogg_page,
    mut boundary: crate::config_types_h::ogg_int64_t,
) -> crate::config_types_h::ogg_int64_t {
    if boundary > 0 as libc::c_int as libc::c_long {
        boundary += (*vf).offset
    }
    loop {
        let mut more: libc::c_long = 0;
        if boundary > 0 as libc::c_int as libc::c_long && (*vf).offset >= boundary {
            return -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        }
        more = crate::src::libogg_1_3_3::src::framing::ogg_sync_pageseek(
            &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
            og as *mut crate::ogg_h::ogg_page,
        );
        if more < 0 as libc::c_int as libc::c_long {
            /* skipped n bytes */
            (*vf).offset -= more
        } else if more == 0 as libc::c_int as libc::c_long {
            /* send more paramedics */
            if boundary == 0 {
                return -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            }
            let mut ret: libc::c_long = _get_data(vf);
            if ret == 0 as libc::c_int as libc::c_long {
                return -(2 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            }
            if ret < 0 as libc::c_int as libc::c_long {
                return -(128 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            }
        } else {
            /* got a page.  Return the offset at the page beginning,
            advance the internal offset past the page end */
            let mut ret_0: crate::config_types_h::ogg_int64_t = (*vf).offset;
            (*vf).offset += more;
            return ret_0;
        }
    }
}
/* find the latest page beginning before the passed in position. Much
dirtier than the above as Ogg doesn't have any backward search
linkage.  no 'readp' as it will certainly have to read. */
/* returns offset or OV_EREAD, OV_FAULT */

unsafe extern "C" fn _get_prev_page(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut begin: crate::config_types_h::ogg_int64_t,
    mut og: *mut crate::ogg_h::ogg_page,
) -> crate::config_types_h::ogg_int64_t {
    let mut end: crate::config_types_h::ogg_int64_t = begin;
    let mut ret: crate::config_types_h::ogg_int64_t = 0;
    let mut offset: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    while offset == -(1 as libc::c_int) as libc::c_long {
        begin -= 65536 as libc::c_int as libc::c_long;
        if begin < 0 as libc::c_int as libc::c_long {
            begin = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
        }
        ret = _seek_helper(vf, begin) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret;
        }
        while (*vf).offset < end {
            crate::stdlib::memset(
                og as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<crate::ogg_h::ogg_page>() as libc::c_ulong,
            );
            ret = _get_next_page(vf, og, end - (*vf).offset);
            if ret == -(128 as libc::c_int) as libc::c_long {
                return -(128 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            }
            if ret < 0 as libc::c_int as libc::c_long {
                break;
            }
            offset = ret
        }
    }
    /* In a fully compliant, non-multiplexed stream, we'll still be
    holding the last page.  In multiplexed (or noncompliant streams),
    we will probably have to re-read the last page we saw */
    if (*og).header_len == 0 as libc::c_int as libc::c_long {
        ret = _seek_helper(vf, offset) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret;
        }
        ret = _get_next_page(
            vf,
            og,
            65536 as libc::c_int as crate::config_types_h::ogg_int64_t,
        );
        if ret < 0 as libc::c_int as libc::c_long {
            /* this shouldn't be possible */
            return -(129 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        }
    }
    return offset;
}

unsafe extern "C" fn _add_serialno(
    mut og: *mut crate::ogg_h::ogg_page,
    mut serialno_list: *mut *mut libc::c_long,
    mut n: *mut libc::c_int,
) {
    let mut s: libc::c_long = crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
        og as *const crate::ogg_h::ogg_page,
    ) as libc::c_long;
    *n += 1;
    if !(*serialno_list).is_null() {
        *serialno_list = crate::stdlib::realloc(
            *serialno_list as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(*n as libc::c_ulong),
        ) as *mut libc::c_long
    } else {
        *serialno_list =
            crate::stdlib::malloc(::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                as *mut libc::c_long
    }
    *(*serialno_list).offset((*n - 1 as libc::c_int) as isize) = s;
}
/* returns nonzero if found */

unsafe extern "C" fn _lookup_serialno(
    mut s: libc::c_long,
    mut serialno_list: *mut libc::c_long,
    mut n: libc::c_int,
) -> libc::c_int {
    if !serialno_list.is_null() {
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 != 0) {
                break;
            }
            if *serialno_list == s {
                return 1 as libc::c_int;
            }
            serialno_list = serialno_list.offset(1)
        }
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn _lookup_page_serialno(
    mut og: *mut crate::ogg_h::ogg_page,
    mut serialno_list: *mut libc::c_long,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_long = crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
        og as *const crate::ogg_h::ogg_page,
    ) as libc::c_long;
    return _lookup_serialno(s, serialno_list, n);
}
/* performs the same search as _get_prev_page, but prefers pages of
the specified serial number. If a page of the specified serialno is
spotted during the seek-back-and-read-forward, it will return the
info of last page of the matching serial number instead of the very
last page.  If no page of the specified serialno is seen, it will
return the info of last page and alter *serialno.  */

unsafe extern "C" fn _get_prev_page_serial(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut begin: crate::config_types_h::ogg_int64_t,
    mut serial_list: *mut libc::c_long,
    mut serial_n: libc::c_int,
    mut serialno: *mut libc::c_int,
    mut granpos: *mut crate::config_types_h::ogg_int64_t,
) -> crate::config_types_h::ogg_int64_t {
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    };
    let mut end: crate::config_types_h::ogg_int64_t = begin;
    let mut ret: crate::config_types_h::ogg_int64_t = 0;
    let mut prefoffset: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut offset: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut ret_serialno: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut ret_gran: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    while offset == -(1 as libc::c_int) as libc::c_long {
        begin -= 65536 as libc::c_int as libc::c_long;
        if begin < 0 as libc::c_int as libc::c_long {
            begin = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
        }
        ret = _seek_helper(vf, begin) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret;
        }
        while (*vf).offset < end {
            ret = _get_next_page(vf, &mut og, end - (*vf).offset);
            if ret == -(128 as libc::c_int) as libc::c_long {
                return -(128 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            }
            if ret < 0 as libc::c_int as libc::c_long {
                break;
            }
            ret_serialno = crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
            ) as crate::config_types_h::ogg_int64_t;
            ret_gran = crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos(
                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
            );
            offset = ret;
            if ret_serialno == *serialno as libc::c_long {
                prefoffset = ret;
                *granpos = ret_gran
            }
            if _lookup_serialno(ret_serialno, serial_list, serial_n) == 0 {
                /* we fell off the end of the link, which means we seeked
                back too far and shouldn't have been looking in that link
                to begin with.  If we found the preferred serial number,
                forget that we saw it. */
                prefoffset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t
            }
        }
    }
    /* we're not interested in the page... just the serialno and granpos. */
    if prefoffset >= 0 as libc::c_int as libc::c_long {
        return prefoffset;
    }
    *serialno = ret_serialno as libc::c_int;
    *granpos = ret_gran;
    return offset;
}
/* uses the local ogg_stream storage in vf; this is important for
non-streaming input sources */

unsafe extern "C" fn _fetch_headers(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut vi: *mut crate::codec_h::vorbis_info,
    mut vc: *mut crate::codec_h::vorbis_comment,
    mut serialno_list: *mut *mut libc::c_long,
    mut serialno_n: *mut libc::c_int,
    mut og_ptr: *mut crate::ogg_h::ogg_page,
) -> libc::c_int {
    let mut current_block: u64;
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    };
    let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
        packet: 0 as *mut libc::c_uchar,
        bytes: 0,
        b_o_s: 0,
        e_o_s: 0,
        granulepos: 0,
        packetno: 0,
    };
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut allbos: libc::c_int = 0 as libc::c_int;
    if og_ptr.is_null() {
        let mut llret: crate::config_types_h::ogg_int64_t = _get_next_page(
            vf,
            &mut og,
            65536 as libc::c_int as crate::config_types_h::ogg_int64_t,
        );
        if llret == -(128 as libc::c_int) as libc::c_long {
            return -(128 as libc::c_int);
        }
        if llret < 0 as libc::c_int as libc::c_long {
            return -(132 as libc::c_int);
        }
        og_ptr = &mut og
    }
    crate::src::libvorbis_1_3_6::lib::info::vorbis_info_init(
        vi as *mut crate::codec_h::vorbis_info,
    );
    crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_init(
        vc as *mut crate::codec_h::vorbis_comment,
    );
    (*vf).ready_state = 2 as libc::c_int;
    loop
    /* extract the serialnos of all BOS pages + the first set of vorbis
    headers we see in the link */
    {
        if !(crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
            og_ptr as *const crate::ogg_h::ogg_page,
        ) != 0)
        {
            current_block = 7746103178988627676;
            break;
        }
        if !serialno_list.is_null() {
            if _lookup_page_serialno(og_ptr, *serialno_list, *serialno_n) != 0 {
                /* a dupe serialnumber in an initial header packet set == invalid stream */
                if !(*serialno_list).is_null() {
                    ::libc::free(*serialno_list as *mut libc::c_void);
                }
                *serialno_list = 0 as *mut libc::c_long;
                *serialno_n = 0 as libc::c_int;
                ret = -(133 as libc::c_int);
                current_block = 5963935241184096755;
                break;
            } else {
                _add_serialno(og_ptr, serialno_list, serialno_n);
            }
        }
        if (*vf).ready_state < 3 as libc::c_int {
            /* we don't have a vorbis stream in this link yet, so begin
            prospective stream setup. We need a stream to get packets */
            crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                    og_ptr as *const crate::ogg_h::ogg_page,
                ),
            );
            crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                og_ptr as *mut crate::ogg_h::ogg_page,
            );
            if crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
            ) > 0 as libc::c_int
                && crate::src::libvorbis_1_3_6::lib::info::vorbis_synthesis_idheader(
                    &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                ) != 0
            {
                /* vorbis header; continue setup */
                (*vf).ready_state = 3 as libc::c_int;
                ret = crate::src::libvorbis_1_3_6::lib::info::vorbis_synthesis_headerin(
                    vi as *mut crate::codec_h::vorbis_info,
                    vc as *mut crate::codec_h::vorbis_comment,
                    &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                );
                if ret != 0 {
                    ret = -(133 as libc::c_int);
                    current_block = 5963935241184096755;
                    break;
                }
            }
        }
        /* get next page */
        let mut llret_0: crate::config_types_h::ogg_int64_t = _get_next_page(
            vf,
            og_ptr,
            65536 as libc::c_int as crate::config_types_h::ogg_int64_t,
        );
        if llret_0 == -(128 as libc::c_int) as libc::c_long {
            ret = -(128 as libc::c_int);
            current_block = 5963935241184096755;
            break;
        } else if llret_0 < 0 as libc::c_int as libc::c_long {
            ret = -(132 as libc::c_int);
            current_block = 5963935241184096755;
            break;
        } else {
            /* if this page also belongs to our vorbis stream, submit it and break */
            if !((*vf).ready_state == 3 as libc::c_int
                && (*vf).os.serialno
                    == crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                        og_ptr as *const crate::ogg_h::ogg_page,
                    ) as libc::c_long)
            {
                continue;
            }
            crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                og_ptr as *mut crate::ogg_h::ogg_page,
            );
            current_block = 7746103178988627676;
            break;
        }
    }
    match current_block {
        7746103178988627676 => {
            if (*vf).ready_state != 3 as libc::c_int {
                ret = -(132 as libc::c_int)
            } else {
                i = 0 as libc::c_int;
                's_219: loop {
                    if !(i < 2 as libc::c_int) {
                        current_block = 307447392441238883;
                        break;
                    }
                    /* get a page loop */
                    while i < 2 as libc::c_int {
                        /* get a packet loop */
                        let mut result: libc::c_int =
                            crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                            );
                        if result == 0 as libc::c_int {
                            break;
                        }
                        if result == -(1 as libc::c_int) {
                            ret = -(133 as libc::c_int);
                            current_block = 5963935241184096755;
                            break 's_219;
                        } else {
                            ret = crate::src::libvorbis_1_3_6::lib::info::vorbis_synthesis_headerin(
                                vi as *mut crate::codec_h::vorbis_info,
                                vc as *mut crate::codec_h::vorbis_comment,
                                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                            );
                            if ret != 0 {
                                current_block = 5963935241184096755;
                                break 's_219;
                            }
                            i += 1
                        }
                    }
                    while i < 2 as libc::c_int {
                        if _get_next_page(
                            vf,
                            og_ptr,
                            65536 as libc::c_int as crate::config_types_h::ogg_int64_t,
                        ) < 0 as libc::c_int as libc::c_long
                        {
                            ret = -(133 as libc::c_int);
                            current_block = 5963935241184096755;
                            break 's_219;
                        } else if (*vf).os.serialno
                            == crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                                og_ptr as *const crate::ogg_h::ogg_page,
                            ) as libc::c_long
                        {
                            crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                og_ptr as *mut crate::ogg_h::ogg_page,
                            );
                            break;
                        } else {
                            /* if this page belongs to the correct stream, go parse it */
                            /* if we never see the final vorbis headers before the link
                            ends, abort */
                            if !(crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
                                og_ptr as *const crate::ogg_h::ogg_page,
                            ) != 0)
                            {
                                continue;
                            }
                            if allbos != 0 {
                                ret = -(133 as libc::c_int);
                                current_block = 5963935241184096755;
                                break 's_219;
                            } else {
                                allbos = 1 as libc::c_int
                            }
                        }
                        /* otherwise, keep looking */
                    }
                }
                match current_block {
                    5963935241184096755 => {}
                    _ => return 0 as libc::c_int,
                }
            }
        }
        _ => {}
    }
    crate::src::libvorbis_1_3_6::lib::info::vorbis_info_clear(
        vi as *mut crate::codec_h::vorbis_info,
    );
    crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_clear(
        vc as *mut crate::codec_h::vorbis_comment,
    );
    (*vf).ready_state = 2 as libc::c_int;
    return ret;
}
/* Starting from current cursor position, get initial PCM offset of
next page.  Consumes the page in the process without decoding
audio, however this is only called during stream parsing upon
seekable open. */

unsafe extern "C" fn _initial_pcmoffset(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut vi: *mut crate::codec_h::vorbis_info,
) -> crate::config_types_h::ogg_int64_t {
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    }; /* should not be possible unless the file is truncated/mangled */
    let mut accumulated: crate::config_types_h::ogg_int64_t =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    let mut lastblock: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    let mut result: libc::c_int = 0;
    let mut serialno: libc::c_int = (*vf).os.serialno as libc::c_int;
    loop {
        let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
            packet: 0 as *mut libc::c_uchar,
            bytes: 0,
            b_o_s: 0,
            e_o_s: 0,
            granulepos: 0,
            packetno: 0,
        };
        if _get_next_page(
            vf,
            &mut og,
            -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
        ) < 0 as libc::c_int as libc::c_long
        {
            break;
        }
        if crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
            &mut og as *mut _ as *const crate::ogg_h::ogg_page,
        ) != 0
        {
            break;
        }
        if crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
            &mut og as *mut _ as *const crate::ogg_h::ogg_page,
        ) != serialno
        {
            continue;
        }
        /* count blocksizes of all frames in the page */
        crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
            &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
        );
        loop {
            result = crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
            );
            if !(result != 0) {
                break;
            }
            if result > 0 as libc::c_int {
                /* ignore holes */
                let mut thisblock: libc::c_long =
                    crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_packet_blocksize(
                        vi as *mut crate::codec_h::vorbis_info,
                        &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                    );
                if thisblock >= 0 as libc::c_int as libc::c_long {
                    if lastblock != -(1 as libc::c_int) as libc::c_long {
                        accumulated += lastblock + thisblock >> 2 as libc::c_int
                    }
                    lastblock = thisblock
                }
            }
        }
        if !(crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos(
            &mut og as *mut _ as *const crate::ogg_h::ogg_page,
        ) != -(1 as libc::c_int) as libc::c_long)
        {
            continue;
        }
        /* pcm offset of last packet on the first audio page */
        accumulated = crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos(
            &mut og as *mut _ as *const crate::ogg_h::ogg_page,
        ) - accumulated;
        break;
    }
    /* less than zero?  Either a corrupt file or a stream with samples
    trimmed off the beginning, a normal occurrence; in both cases set
    the offset to zero */
    if accumulated < 0 as libc::c_int as libc::c_long {
        accumulated = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
    }
    return accumulated;
}
/* finds each bitstream link one at a time using a bisection search
(has to begin by knowing the offset of the lb's initial page).
Recurses for each link so it can alloc the link storage after
finding them all, then unroll and fill the cache at the same time */

unsafe extern "C" fn _bisect_forward_serialno(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut begin: crate::config_types_h::ogg_int64_t,
    mut searched: crate::config_types_h::ogg_int64_t,
    mut end: crate::config_types_h::ogg_int64_t,
    mut endgran: crate::config_types_h::ogg_int64_t,
    mut endserial: libc::c_int,
    mut currentno_list: *mut libc::c_long,
    mut currentnos: libc::c_int,
    mut m: libc::c_long,
) -> libc::c_int {
    let mut pcmoffset: crate::config_types_h::ogg_int64_t = 0;
    let mut dataoffset: crate::config_types_h::ogg_int64_t = searched;
    let mut endsearched: crate::config_types_h::ogg_int64_t = end;
    let mut next: crate::config_types_h::ogg_int64_t = end;
    let mut searchgran: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    };
    let mut ret: crate::config_types_h::ogg_int64_t = 0;
    let mut last: crate::config_types_h::ogg_int64_t = 0;
    let mut serialno: libc::c_int = (*vf).os.serialno as libc::c_int;
    /* invariants:
       we have the headers and serialnos for the link beginning at 'begin'
       we have the offset and granpos of the last page in the file (potentially
         not a page we care about)
    */
    /* Is the last page in our list of current serialnumbers? */
    if _lookup_serialno(endserial as libc::c_long, currentno_list, currentnos) != 0 {
        /* last page is in the starting serialno list, so we've bisected
        down to (or just started with) a single link.  Now we need to
        find the last vorbis page belonging to the first vorbis stream
        for this link. */
        searched = end;
        while endserial != serialno {
            endserial = serialno;
            searched = _get_prev_page_serial(
                vf,
                searched,
                currentno_list,
                currentnos,
                &mut endserial,
                &mut endgran,
            )
        }
        (*vf).links = (m + 1 as libc::c_int as libc::c_long) as libc::c_int;
        if !(*vf).offsets.is_null() {
            ::libc::free((*vf).offsets as *mut libc::c_void);
        }
        if !(*vf).serialnos.is_null() {
            ::libc::free((*vf).serialnos as *mut libc::c_void);
        }
        if !(*vf).dataoffsets.is_null() {
            ::libc::free((*vf).dataoffsets as *mut libc::c_void);
        }
        (*vf).offsets = crate::stdlib::malloc(
            (((*vf).links + 1 as libc::c_int) as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
            ),
        ) as *mut crate::config_types_h::ogg_int64_t;
        (*vf).vi =
            crate::stdlib::realloc(
                (*vf).vi as *mut libc::c_void,
                ((*vf).links as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                    crate::codec_h::vorbis_info,
                >() as libc::c_ulong),
            ) as *mut crate::codec_h::vorbis_info;
        (*vf).vc = crate::stdlib::realloc(
            (*vf).vc as *mut libc::c_void,
            ((*vf).links as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                crate::codec_h::vorbis_comment,
            >() as libc::c_ulong),
        ) as *mut crate::codec_h::vorbis_comment;
        (*vf).serialnos = crate::stdlib::malloc(
            ((*vf).links as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        ) as *mut libc::c_long;
        (*vf).dataoffsets = crate::stdlib::malloc(((*vf).links as libc::c_ulong).wrapping_mul(
            ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
        )) as *mut crate::config_types_h::ogg_int64_t;
        (*vf).pcmlengths = crate::stdlib::malloc(
            (((*vf).links * 2 as libc::c_int) as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
            ),
        ) as *mut crate::config_types_h::ogg_int64_t;
        *(*vf)
            .offsets
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = end;
        *(*vf).offsets.offset(m as isize) = begin;
        *(*vf).pcmlengths.offset(
            (m * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long) as isize,
        ) = if endgran < 0 as libc::c_int as libc::c_long {
            0 as libc::c_int as libc::c_long
        } else {
            endgran
        }
    } else {
        /* last page is not in the starting stream's serial number list,
        so we have multiple links.  Find where the stream that begins
        our bisection ends. */
        let mut next_serialno_list: *mut libc::c_long = 0 as *mut libc::c_long;
        let mut next_serialnos: libc::c_int = 0 as libc::c_int;
        let mut vi: crate::codec_h::vorbis_info = crate::codec_h::vorbis_info {
            version: 0,
            channels: 0,
            rate: 0,
            bitrate_upper: 0,
            bitrate_nominal: 0,
            bitrate_lower: 0,
            bitrate_window: 0,
            codec_setup: 0 as *mut libc::c_void,
        };
        let mut vc: crate::codec_h::vorbis_comment = crate::codec_h::vorbis_comment {
            user_comments: 0 as *mut *mut libc::c_char,
            comment_lengths: 0 as *mut libc::c_int,
            comments: 0,
            vendor: 0 as *mut libc::c_char,
        };
        let mut testserial: libc::c_int = serialno + 1 as libc::c_int;
        /* the below guards against garbage seperating the last and
        first pages of two links. */
        while searched < endsearched {
            let mut bisect: crate::config_types_h::ogg_int64_t = 0;
            if endsearched - searched < 65536 as libc::c_int as libc::c_long {
                bisect = searched
            } else {
                bisect = (searched + endsearched) / 2 as libc::c_int as libc::c_long
            }
            ret = _seek_helper(vf, bisect) as crate::config_types_h::ogg_int64_t;
            if ret != 0 {
                return ret as libc::c_int;
            }
            last = _get_next_page(
                vf,
                &mut og,
                -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
            );
            if last == -(128 as libc::c_int) as libc::c_long {
                return -(128 as libc::c_int);
            }
            if last < 0 as libc::c_int as libc::c_long
                || _lookup_page_serialno(&mut og, currentno_list, currentnos) == 0
            {
                endsearched = bisect;
                if last >= 0 as libc::c_int as libc::c_long {
                    next = last
                }
            } else {
                searched = (*vf).offset
            }
        }
        /* Bisection point found */
        /* for the time being, fetch end PCM offset the simple way */
        searched = next;
        while testserial != serialno {
            testserial = serialno;
            searched = _get_prev_page_serial(
                vf,
                searched,
                currentno_list,
                currentnos,
                &mut testserial,
                &mut searchgran,
            )
        }
        ret = _seek_helper(vf, next) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret as libc::c_int;
        }
        ret = _fetch_headers(
            vf,
            &mut vi,
            &mut vc,
            &mut next_serialno_list,
            &mut next_serialnos,
            0 as *mut crate::ogg_h::ogg_page,
        ) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret as libc::c_int;
        }
        serialno = (*vf).os.serialno as libc::c_int;
        dataoffset = (*vf).offset;
        /* this will consume a page, however the next bisection always
        starts with a raw seek */
        pcmoffset = _initial_pcmoffset(vf, &mut vi);
        ret = _bisect_forward_serialno(
            vf,
            next,
            (*vf).offset,
            end,
            endgran,
            endserial,
            next_serialno_list,
            next_serialnos,
            m + 1 as libc::c_int as libc::c_long,
        ) as crate::config_types_h::ogg_int64_t;
        if ret != 0 {
            return ret as libc::c_int;
        }
        if !next_serialno_list.is_null() {
            ::libc::free(next_serialno_list as *mut libc::c_void);
        }
        *(*vf)
            .offsets
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = next;
        *(*vf)
            .serialnos
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = serialno as libc::c_long;
        *(*vf)
            .dataoffsets
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = dataoffset;
        *(*vf)
            .vi
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = vi;
        *(*vf)
            .vc
            .offset((m + 1 as libc::c_int as libc::c_long) as isize) = vc;
        *(*vf).pcmlengths.offset(
            (m * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long) as isize,
        ) = searchgran;
        *(*vf).pcmlengths.offset(
            (m * 2 as libc::c_int as libc::c_long + 2 as libc::c_int as libc::c_long) as isize,
        ) = pcmoffset;
        let ref mut fresh1 = *(*vf).pcmlengths.offset(
            (m * 2 as libc::c_int as libc::c_long + 3 as libc::c_int as libc::c_long) as isize,
        );
        *fresh1 -= pcmoffset;
        if *(*vf).pcmlengths.offset(
            (m * 2 as libc::c_int as libc::c_long + 3 as libc::c_int as libc::c_long) as isize,
        ) < 0 as libc::c_int as libc::c_long
        {
            *(*vf).pcmlengths.offset(
                (m * 2 as libc::c_int as libc::c_long + 3 as libc::c_int as libc::c_long) as isize,
            ) = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
        }
    }
    return 0 as libc::c_int;
}

unsafe extern "C" fn _make_decode_ready(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    if (*vf).ready_state > 3 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*vf).ready_state < 3 as libc::c_int {
        return -(129 as libc::c_int);
    }
    if (*vf).seekable != 0 {
        if crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_init(
            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
            (*vf).vi.offset((*vf).current_link as isize) as *mut crate::codec_h::vorbis_info,
        ) != 0
        {
            return -(137 as libc::c_int);
        }
    } else if crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_init(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        (*vf).vi as *mut crate::codec_h::vorbis_info,
    ) != 0
    {
        return -(137 as libc::c_int);
    }
    crate::src::libvorbis_1_3_6::lib::block::vorbis_block_init(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
    );
    (*vf).ready_state = 4 as libc::c_int;
    (*vf).bittrack = 0.0f32 as libc::c_double;
    (*vf).samptrack = 0.0f32 as libc::c_double;
    return 0 as libc::c_int;
}

unsafe extern "C" fn _open_seekable2(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    let mut dataoffset: crate::config_types_h::ogg_int64_t =
        *(*vf).dataoffsets.offset(0 as libc::c_int as isize);
    let mut end: crate::config_types_h::ogg_int64_t = 0;
    let mut endgran: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut endserial: libc::c_int = (*vf).os.serialno as libc::c_int;
    let mut serialno: libc::c_int = (*vf).os.serialno as libc::c_int;
    /* we're partially open and have a first link header state in
    storage in vf */
    /* fetch initial PCM offset */
    let mut pcmoffset: crate::config_types_h::ogg_int64_t = _initial_pcmoffset(vf, (*vf).vi);
    /* we can seek, so set out learning all about this file */
    if (*vf).callbacks.seek_func.is_some() && (*vf).callbacks.tell_func.is_some() {
        (*vf)
            .callbacks
            .seek_func
            .expect("non-null function pointer")(
            (*vf).datasource,
            0 as libc::c_int as crate::config_types_h::ogg_int64_t,
            2 as libc::c_int,
        );
        (*vf).end = (*vf)
            .callbacks
            .tell_func
            .expect("non-null function pointer")((*vf).datasource);
        (*vf).offset = (*vf).end
    } else {
        (*vf).end = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        (*vf).offset = (*vf).end
    }
    /* If seek_func is implemented, tell_func must also be implemented */
    if (*vf).end == -(1 as libc::c_int) as libc::c_long {
        return -(131 as libc::c_int);
    }
    /* Get the offset of the last page of the physical bitstream, or, if
    we're lucky the last vorbis page of this link as most OggVorbis
    files will contain a single logical bitstream */
    end = _get_prev_page_serial(
        vf,
        (*vf).end,
        (*vf).serialnos.offset(2 as libc::c_int as isize),
        *(*vf).serialnos.offset(1 as libc::c_int as isize) as libc::c_int,
        &mut endserial,
        &mut endgran,
    );
    if end < 0 as libc::c_int as libc::c_long {
        return end as libc::c_int;
    }
    /* now determine bitstream structure recursively */
    if _bisect_forward_serialno(
        vf,
        0 as libc::c_int as crate::config_types_h::ogg_int64_t,
        dataoffset,
        end,
        endgran,
        endserial,
        (*vf).serialnos.offset(2 as libc::c_int as isize),
        *(*vf).serialnos.offset(1 as libc::c_int as isize) as libc::c_int,
        0 as libc::c_int as libc::c_long,
    ) < 0 as libc::c_int
    {
        return -(128 as libc::c_int);
    }
    *(*vf).offsets.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    *(*vf).serialnos.offset(0 as libc::c_int as isize) = serialno as libc::c_long;
    *(*vf).dataoffsets.offset(0 as libc::c_int as isize) = dataoffset;
    *(*vf).pcmlengths.offset(0 as libc::c_int as isize) = pcmoffset;
    let ref mut fresh2 = *(*vf).pcmlengths.offset(1 as libc::c_int as isize);
    *fresh2 -= pcmoffset;
    if *(*vf).pcmlengths.offset(1 as libc::c_int as isize) < 0 as libc::c_int as libc::c_long {
        *(*vf).pcmlengths.offset(1 as libc::c_int as isize) =
            0 as libc::c_int as crate::config_types_h::ogg_int64_t
    }
    return ov_raw_seek(vf, dataoffset);
}
/* clear out the current logical bitstream decoder */

unsafe extern "C" fn _decode_clear(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) {
    crate::src::libvorbis_1_3_6::lib::block::vorbis_dsp_clear(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
    );
    crate::src::libvorbis_1_3_6::lib::block::vorbis_block_clear(
        &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
    );
    (*vf).ready_state = 2 as libc::c_int;
}
/* fetch and process a packet.  Handles the case where we're at a
   bitstream boundary and dumps the decoding machine.  If the decoding
   machine is unloaded, it loads it.  It also keeps pcm_offset up to
   date (seek and read both use this.  seek uses a special hack with
   readp).

   return: <0) error, OV_HOLE (lost packet) or OV_EOF
            0) need more data (only if readp==0)
            1) got a packet
*/

unsafe extern "C" fn _fetch_and_process_packet(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut op_in: *mut crate::ogg_h::ogg_packet,
    mut readp: libc::c_int,
    mut spanp: libc::c_int,
) -> libc::c_int {
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    };
    loop
    /* handle one packet.  Try to fetch it from current stream state */
    /* extract packets from page */
    {
        if (*vf).ready_state == 3 as libc::c_int {
            let mut ret: libc::c_int = _make_decode_ready(vf);
            if ret < 0 as libc::c_int {
                return ret;
            }
        }
        /* process a packet if we can. */
        if (*vf).ready_state == 4 as libc::c_int {
            let mut hs: libc::c_int =
                crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p(
                    (*vf).vi as *mut crate::codec_h::vorbis_info,
                ); /* hole in the data. */
            loop {
                let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
                    packet: 0 as *mut libc::c_uchar,
                    bytes: 0,
                    b_o_s: 0,
                    e_o_s: 0,
                    granulepos: 0,
                    packetno: 0,
                };
                let mut op_ptr: *mut crate::ogg_h::ogg_packet =
                    if !op_in.is_null() { op_in } else { &mut op };
                let mut result: libc::c_int =
                    crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                        &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                        op_ptr as *mut crate::ogg_h::ogg_packet,
                    );
                let mut granulepos: crate::config_types_h::ogg_int64_t = 0;
                op_in = 0 as *mut crate::ogg_h::ogg_packet;
                if result == -(1 as libc::c_int) {
                    return -(3 as libc::c_int);
                }
                if !(result > 0 as libc::c_int) {
                    break;
                }
                /* got a packet.  process it */
                granulepos = (*op_ptr).granulepos;
                if crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis(
                    &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
                    op_ptr as *mut crate::ogg_h::ogg_packet,
                ) == 0
                {
                    /* lazy check for lazy
                    header handling.  The
                    header packets aren't
                    audio, so if/when we
                    submit them,
                    vorbis_synthesis will
                    reject them */
                    /* suck in the synthesis data and track bitrate */
                    let mut oldsamples: libc::c_int =
                        crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                            0 as *mut *mut *mut libc::c_float,
                        );
                    /* for proper use of libvorbis within libvorbisfile,
                    oldsamples will always be zero. */
                    if oldsamples != 0 {
                        return -(129 as libc::c_int);
                    }
                    crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_blockin(
                        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                        &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
                    );
                    (*vf).samptrack +=
                        (crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                            0 as *mut *mut *mut libc::c_float,
                        ) << hs) as libc::c_double;
                    (*vf).bittrack +=
                        ((*op_ptr).bytes * 8 as libc::c_int as libc::c_long) as libc::c_double;
                    /* update the pcm offset. */
                    if granulepos != -(1 as libc::c_int) as libc::c_long && (*op_ptr).e_o_s == 0 {
                        let mut link: libc::c_int = if (*vf).seekable != 0 {
                            (*vf).current_link
                        } else {
                            0 as libc::c_int
                        };
                        let mut i: libc::c_int = 0;
                        let mut samples: libc::c_int = 0;
                        /* this packet has a pcm_offset on it (the last packet
                        completed on a page carries the offset) After processing
                        (above), we know the pcm position of the *last* sample
                        ready to be returned. Find the offset of the *first*

                        As an aside, this trick is inaccurate if we begin
                        reading anew right at the last page; the end-of-stream
                        granulepos declares the last frame in the stream, and the
                        last packet of the last page may be a partial frame.
                        So, we need a previous granulepos from an in-sequence page
                        to have a reference point.  Thus the !op_ptr->e_o_s clause
                        above */
                        if (*vf).seekable != 0 && link > 0 as libc::c_int {
                            granulepos -=
                                *(*vf).pcmlengths.offset((link * 2 as libc::c_int) as isize)
                        } /* actually, this
                          shouldn't be possible
                          here unless the stream
                          is very broken */
                        if granulepos < 0 as libc::c_int as libc::c_long {
                            granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
                        }
                        samples = crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                            0 as *mut *mut *mut libc::c_float,
                        ) << hs;
                        granulepos -= samples as libc::c_long;
                        i = 0 as libc::c_int;
                        while i < link {
                            granulepos += *(*vf)
                                .pcmlengths
                                .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
                            i += 1
                        }
                        (*vf).pcm_offset = granulepos
                    }
                    return 1 as libc::c_int;
                }
            }
        }
        if (*vf).ready_state >= 2 as libc::c_int {
            let mut ret_0: crate::config_types_h::ogg_int64_t = 0;
            loop {
                /* the loop is not strictly necessary, but there's no sense in
                doing the extra checks of the larger loop for the common
                case in a multiplexed bistream where the page is simply
                part of a different logical bitstream; keep reading until
                we get one with the correct serialno */
                if readp == 0 {
                    return 0 as libc::c_int;
                }
                ret_0 = _get_next_page(
                    vf,
                    &mut og,
                    -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
                );
                if ret_0 < 0 as libc::c_int as libc::c_long {
                    return -(2 as libc::c_int);
                    /* eof. leave unitialized */
                }
                /* bitrate tracking; add the header's bytes here, the body bytes
                are done by packet above */
                (*vf).bittrack +=
                    (og.header_len * 8 as libc::c_int as libc::c_long) as libc::c_double;
                if !((*vf).ready_state == 4 as libc::c_int) {
                    break;
                }
                if !((*vf).current_serialno
                    != crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                        &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                    ) as libc::c_long)
                {
                    break;
                }
                /* two possibilities:
                1) our decoding just traversed a bitstream boundary
                2) another stream is multiplexed into this logical section */
                if !(crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
                    &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                ) != 0)
                {
                    continue;
                }
                /* boundary case */
                if spanp == 0 {
                    return -(2 as libc::c_int);
                }
                _decode_clear(vf);
                if (*vf).seekable == 0 {
                    crate::src::libvorbis_1_3_6::lib::info::vorbis_info_clear(
                        (*vf).vi as *mut crate::codec_h::vorbis_info,
                    );
                    crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_clear(
                        (*vf).vc as *mut crate::codec_h::vorbis_comment,
                    );
                }
                break;
                /* possibility #2 */
            }
        }
        /* Do we need to load a new machine before submitting the page? */
        /* This is different in the seekable and non-seekable cases.

           In the seekable case, we already have all the header
           information loaded and cached; we just initialize the machine
           with it and continue on our merry way.

           In the non-seekable (streaming) case, we'll only be at a
           boundary if we just left the previous logical bitstream and
           we're now nominally at the header of the next bitstream
        */
        if (*vf).ready_state != 4 as libc::c_int {
            let mut link_0: libc::c_int = 0;
            if (*vf).ready_state < 3 as libc::c_int {
                if (*vf).seekable != 0 {
                    let mut serialno: libc::c_long =
                        crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                            &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                        ) as libc::c_long;
                    /* match the serialno to bitstream section.  We use this rather than
                    offset positions to avoid problems near logical bitstream
                    boundaries */
                    link_0 = 0 as libc::c_int; /* not the desired Vorbis
                                               bitstream section; keep
                                               trying */
                    while link_0 < (*vf).links {
                        if *(*vf).serialnos.offset(link_0 as isize) == serialno {
                            break;
                        }
                        link_0 += 1
                    }
                    if link_0 == (*vf).links {
                        continue;
                    }
                    (*vf).current_serialno = serialno;
                    (*vf).current_link = link_0;
                    crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                        &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                        (*vf).current_serialno as libc::c_int,
                    );
                    (*vf).ready_state = 3 as libc::c_int
                } else {
                    /* we're streaming */
                    /* fetch the three header packets, build the info struct */
                    let mut ret_1: libc::c_int = _fetch_headers(
                        vf,
                        (*vf).vi,
                        (*vf).vc,
                        0 as *mut *mut libc::c_long,
                        0 as *mut libc::c_int,
                        &mut og,
                    );
                    if ret_1 != 0 {
                        return ret_1;
                    }
                    (*vf).current_serialno = (*vf).os.serialno;
                    (*vf).current_link += 1;
                    link_0 = 0 as libc::c_int
                }
            }
        }
        /* the buffered page is the data we want, and we're ready for it;
        add it to the stream state */
        crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
            &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
        );
    }
}
/* if, eg, 64 bit stdio is configured by default, this will build with
fseek64 */

unsafe extern "C" fn _fseek64_wrap(
    mut f: *mut crate::stdlib::FILE,
    mut off: crate::config_types_h::ogg_int64_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    return crate::stdlib::fseek(f, off, whence);
}

unsafe extern "C" fn _ov_open1(
    mut f: *mut libc::c_void,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut initial: *const libc::c_char,
    mut ibytes: libc::c_long,
    mut callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks,
) -> libc::c_int {
    let mut offsettest: libc::c_int = if !f.is_null() && callbacks.seek_func.is_some() {
        callbacks.seek_func.expect("non-null function pointer")(
            f,
            0 as libc::c_int as crate::config_types_h::ogg_int64_t,
            1 as libc::c_int,
        )
    } else {
        -(1 as libc::c_int)
    };
    let mut serialno_list: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut serialno_list_size: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0;
    crate::stdlib::memset(
        vf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File>()
            as libc::c_ulong,
    );
    (*vf).datasource = f;
    (*vf).callbacks = callbacks;
    /* init the framing state */
    crate::src::libogg_1_3_3::src::framing::ogg_sync_init(
        &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
    );
    /* perhaps some data was previously read into a buffer for testing
    against other stream types.  Allow initialization from this
    previously read data (especially as we may be reading from a
    non-seekable stream) */
    if !initial.is_null() {
        let mut buffer: *mut libc::c_char = crate::src::libogg_1_3_3::src::framing::ogg_sync_buffer(
            &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
            ibytes,
        );
        crate::stdlib::memcpy(
            buffer as *mut libc::c_void,
            initial as *const libc::c_void,
            ibytes as libc::c_ulong,
        );
        crate::src::libogg_1_3_3::src::framing::ogg_sync_wrote(
            &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
            ibytes,
        );
    }
    /* can we seek? Stevens suggests the seek test was portable */
    if offsettest != -(1 as libc::c_int) {
        (*vf).seekable = 1 as libc::c_int
    }
    /* No seeking yet; Set up a 'single' (current) logical bitstream
    entry for partial open */
    (*vf).links = 1 as libc::c_int; /* fill in the serialno later */
    (*vf).vi = crate::stdlib::calloc(
        (*vf).links as libc::c_ulong,
        ::std::mem::size_of::<crate::codec_h::vorbis_info>() as libc::c_ulong,
    ) as *mut crate::codec_h::vorbis_info;
    (*vf).vc = crate::stdlib::calloc(
        (*vf).links as libc::c_ulong,
        ::std::mem::size_of::<crate::codec_h::vorbis_comment>() as libc::c_ulong,
    ) as *mut crate::codec_h::vorbis_comment;
    crate::src::libogg_1_3_3::src::framing::ogg_stream_init(
        &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        -(1 as libc::c_int),
    );
    /* Fetch all BOS pages, store the vorbis header and all seen serial
    numbers, load subsequent vorbis setup headers */
    ret = _fetch_headers(
        vf,
        (*vf).vi,
        (*vf).vc,
        &mut serialno_list,
        &mut serialno_list_size,
        0 as *mut crate::ogg_h::ogg_page,
    );
    if ret < 0 as libc::c_int {
        (*vf).datasource = 0 as *mut libc::c_void;
        ov_clear(vf);
    } else {
        /* serial number list for first link needs to be held somewhere
        for second stage of seekable stream open; this saves having to
        seek/reread first link's serialnumber data then. */
        (*vf).serialnos = crate::stdlib::calloc(
            (serialno_list_size + 2 as libc::c_int) as libc::c_ulong,
            ::std::mem::size_of::<libc::c_long>() as libc::c_ulong,
        ) as *mut libc::c_long;
        (*vf).current_serialno = (*vf).os.serialno;
        *(*vf).serialnos.offset(0 as libc::c_int as isize) = (*vf).current_serialno;
        *(*vf).serialnos.offset(1 as libc::c_int as isize) = serialno_list_size as libc::c_long;
        crate::stdlib::memcpy(
            (*vf).serialnos.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            serialno_list as *const libc::c_void,
            (serialno_list_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        );
        (*vf).offsets = crate::stdlib::calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
        ) as *mut crate::config_types_h::ogg_int64_t;
        (*vf).dataoffsets = crate::stdlib::calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<crate::config_types_h::ogg_int64_t>() as libc::c_ulong,
        ) as *mut crate::config_types_h::ogg_int64_t;
        *(*vf).offsets.offset(0 as libc::c_int as isize) =
            0 as libc::c_int as crate::config_types_h::ogg_int64_t;
        *(*vf).dataoffsets.offset(0 as libc::c_int as isize) = (*vf).offset;
        (*vf).ready_state = 1 as libc::c_int
    }
    if !serialno_list.is_null() {
        ::libc::free(serialno_list as *mut libc::c_void);
    }
    return ret;
}

unsafe extern "C" fn _ov_open2(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    if (*vf).ready_state != 1 as libc::c_int {
        return -(131 as libc::c_int);
    }
    (*vf).ready_state = 2 as libc::c_int;
    if (*vf).seekable != 0 {
        let mut ret: libc::c_int = _open_seekable2(vf);
        if ret != 0 {
            (*vf).datasource = 0 as *mut libc::c_void;
            ov_clear(vf);
        }
        return ret;
    } else {
        (*vf).ready_state = 3 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* clear out the OggVorbis_File struct */
#[no_mangle]

pub unsafe extern "C" fn ov_clear(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    if !vf.is_null() {
        crate::src::libvorbis_1_3_6::lib::block::vorbis_block_clear(
            &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
        );
        crate::src::libvorbis_1_3_6::lib::block::vorbis_dsp_clear(
            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        );
        crate::src::libogg_1_3_3::src::framing::ogg_stream_clear(
            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        );
        if !(*vf).vi.is_null() && (*vf).links != 0 {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*vf).links {
                crate::src::libvorbis_1_3_6::lib::info::vorbis_info_clear(
                    (*vf).vi.offset(i as isize) as *mut crate::codec_h::vorbis_info,
                );
                crate::src::libvorbis_1_3_6::lib::info::vorbis_comment_clear(
                    (*vf).vc.offset(i as isize) as *mut crate::codec_h::vorbis_comment,
                );
                i += 1
            }
            ::libc::free((*vf).vi as *mut libc::c_void);
            ::libc::free((*vf).vc as *mut libc::c_void);
        }
        if !(*vf).dataoffsets.is_null() {
            ::libc::free((*vf).dataoffsets as *mut libc::c_void);
        }
        if !(*vf).pcmlengths.is_null() {
            ::libc::free((*vf).pcmlengths as *mut libc::c_void);
        }
        if !(*vf).serialnos.is_null() {
            ::libc::free((*vf).serialnos as *mut libc::c_void);
        }
        if !(*vf).offsets.is_null() {
            ::libc::free((*vf).offsets as *mut libc::c_void);
        }
        crate::src::libogg_1_3_3::src::framing::ogg_sync_clear(
            &mut (*vf).oy as *mut _ as *mut crate::ogg_h::ogg_sync_state,
        );
        if !(*vf).datasource.is_null() && (*vf).callbacks.close_func.is_some() {
            (*vf)
                .callbacks
                .close_func
                .expect("non-null function pointer")((*vf).datasource);
        }
        crate::stdlib::memset(
            vf as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File>()
                as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
/* inspects the OggVorbis file and finds/documents all the logical
   bitstreams contained in it.  Tries to be tolerant of logical
   bitstream sections that are truncated/woogie.

   return: -1) error
            0) OK
*/
#[no_mangle]

pub unsafe extern "C" fn ov_open_callbacks(
    mut f: *mut libc::c_void,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut initial: *const libc::c_char,
    mut ibytes: libc::c_long,
    mut callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks,
) -> libc::c_int {
    let mut ret: libc::c_int = _ov_open1(f, vf, initial, ibytes, callbacks);
    if ret != 0 {
        return ret;
    }
    return _ov_open2(vf);
}
#[no_mangle]

pub unsafe extern "C" fn ov_open(
    mut f: *mut crate::stdlib::FILE,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut initial: *const libc::c_char,
    mut ibytes: libc::c_long,
) -> libc::c_int {
    let mut callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks = {
        let mut init = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks {
            read_func: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: libc::c_ulong,
                        _: libc::c_ulong,
                        _: *mut crate::stdlib::FILE,
                    ) -> libc::c_ulong,
                >,
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: crate::stddef_h::size_t,
                        _: crate::stddef_h::size_t,
                        _: *mut libc::c_void,
                    ) -> crate::stddef_h::size_t,
                >,
            >(Some(
                crate::stdlib::fread
                    as unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: libc::c_ulong,
                        _: libc::c_ulong,
                        _: *mut crate::stdlib::FILE,
                    ) -> libc::c_ulong,
            )),
            seek_func: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        _: *mut crate::stdlib::FILE,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
                >,
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
                >,
            >(Some(
                _fseek64_wrap
                    as unsafe extern "C" fn(
                        _: *mut crate::stdlib::FILE,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
            )),
            close_func: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_int>,
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            >(Some(
                crate::stdlib::fclose
                    as unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_int,
            )),
            tell_func: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_long>,
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>,
            >(Some(
                crate::stdlib::ftell
                    as unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_long,
            )),
        };
        init
    };
    return ov_open_callbacks(f as *mut libc::c_void, vf, initial, ibytes, callbacks);
}
#[no_mangle]

pub unsafe extern "C" fn ov_fopen(
    mut path: *const libc::c_char,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut f: *mut crate::stdlib::FILE =
        crate::stdlib::fopen(path, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    ret = ov_open(
        f,
        vf,
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_long,
    );
    if ret != 0 {
        crate::stdlib::fclose(f);
    }
    return ret;
}
/* cheap hack for game usage where downsampling is desirable; there's
no need for SRC as we can just do it cheaply in libvorbis. */
#[no_mangle]

pub unsafe extern "C" fn ov_halfrate(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut flag: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*vf).vi.is_null() {
        return -(131 as libc::c_int);
    }
    if (*vf).ready_state > 3 as libc::c_int {
        /* clear out stream state; dumping the decode machine is needed to
        reinit the MDCT lookups. */
        crate::src::libvorbis_1_3_6::lib::block::vorbis_dsp_clear(
            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        ); /* make sure the pos is dumped if unseekable */
        crate::src::libvorbis_1_3_6::lib::block::vorbis_block_clear(
            &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
        );
        (*vf).ready_state = 3 as libc::c_int;
        if (*vf).pcm_offset >= 0 as libc::c_int as libc::c_long {
            let mut pos: crate::config_types_h::ogg_int64_t = (*vf).pcm_offset;
            (*vf).pcm_offset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
            ov_pcm_seek(vf, pos);
        }
    }
    i = 0 as libc::c_int;
    while i < (*vf).links {
        if crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate(
            (*vf).vi.offset(i as isize) as *mut crate::codec_h::vorbis_info,
            flag,
        ) != 0
        {
            if flag != 0 {
                ov_halfrate(vf, 0 as libc::c_int);
            }
            return -(131 as libc::c_int);
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ov_halfrate_p(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    if (*vf).vi.is_null() {
        return -(131 as libc::c_int);
    }
    return crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p(
        (*vf).vi as *mut crate::codec_h::vorbis_info,
    );
}
/* Only partially open the vorbis file; test for Vorbisness, and load
the headers for the first chain.  Do not seek (although test for
seekability).  Use ov_test_open to finish opening the file, else
ov_clear to close/free it. Same return codes as open.

Note that vorbisfile does _not_ take ownership of the file if the
call fails; the calling applicaiton is responsible for closing the file
if this call returns an error. */
#[no_mangle]

pub unsafe extern "C" fn ov_test_callbacks(
    mut f: *mut libc::c_void,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut initial: *const libc::c_char,
    mut ibytes: libc::c_long,
    mut callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks,
) -> libc::c_int {
    return _ov_open1(f, vf, initial, ibytes, callbacks);
}
#[no_mangle]

pub unsafe extern "C" fn ov_test(
    mut f: *mut crate::stdlib::FILE,
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut initial: *const libc::c_char,
    mut ibytes: libc::c_long,
) -> libc::c_int {
    let mut callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks = {
        let mut init = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks {
            read_func: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: libc::c_ulong,
                        _: libc::c_ulong,
                        _: *mut crate::stdlib::FILE,
                    ) -> libc::c_ulong,
                >,
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: crate::stddef_h::size_t,
                        _: crate::stddef_h::size_t,
                        _: *mut libc::c_void,
                    ) -> crate::stddef_h::size_t,
                >,
            >(Some(
                crate::stdlib::fread
                    as unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: libc::c_ulong,
                        _: libc::c_ulong,
                        _: *mut crate::stdlib::FILE,
                    ) -> libc::c_ulong,
            )),
            seek_func: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        _: *mut crate::stdlib::FILE,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
                >,
                Option<
                    unsafe extern "C" fn(
                        _: *mut libc::c_void,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
                >,
            >(Some(
                _fseek64_wrap
                    as unsafe extern "C" fn(
                        _: *mut crate::stdlib::FILE,
                        _: crate::config_types_h::ogg_int64_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
            )),
            close_func: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_int>,
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
            >(Some(
                crate::stdlib::fclose
                    as unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_int,
            )),
            tell_func: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_long>,
                Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>,
            >(Some(
                crate::stdlib::ftell
                    as unsafe extern "C" fn(_: *mut crate::stdlib::FILE) -> libc::c_long,
            )),
        };
        init
    };
    return ov_test_callbacks(f as *mut libc::c_void, vf, initial, ibytes, callbacks);
}
#[no_mangle]

pub unsafe extern "C" fn ov_test_open(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    if (*vf).ready_state != 1 as libc::c_int {
        return -(131 as libc::c_int);
    }
    return _ov_open2(vf);
}
/* How many logical bitstreams in this physical bitstream? */
#[no_mangle]

pub unsafe extern "C" fn ov_streams(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_long {
    return (*vf).links as libc::c_long;
}
/* Is the FILE * associated with vf seekable? */
#[no_mangle]

pub unsafe extern "C" fn ov_seekable(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_long {
    return (*vf).seekable as libc::c_long;
}
/* returns the bitrate for a given logical bitstream or the entire
physical bitstream.  If the file is open for random access, it will
find the *actual* average bitrate.  If the file is streaming, it
returns the nominal bitrate (if set) else the average of the
upper/lower bounds (if set) else -1 (unset).

If you want the actual bitrate field settings, get them from the
vorbis_info structs */
#[no_mangle]

pub unsafe extern "C" fn ov_bitrate(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut i: libc::c_int,
) -> libc::c_long {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_long;
    }
    if i >= (*vf).links {
        return -(131 as libc::c_int) as libc::c_long;
    }
    if (*vf).seekable == 0 && i != 0 as libc::c_int {
        return ov_bitrate(vf, 0 as libc::c_int);
    }
    if i < 0 as libc::c_int {
        let mut bits: crate::config_types_h::ogg_int64_t =
            0 as libc::c_int as crate::config_types_h::ogg_int64_t;
        let mut i_0: libc::c_int = 0;
        let mut br: libc::c_float = 0.;
        i_0 = 0 as libc::c_int;
        while i_0 < (*vf).links {
            bits += (*(*vf).offsets.offset((i_0 + 1 as libc::c_int) as isize)
                - *(*vf).dataoffsets.offset(i_0 as isize))
                * 8 as libc::c_int as libc::c_long;
            i_0 += 1
        }
        /* This once read: return(rint(bits/ov_time_total(vf,-1)));
         * gcc 3.x on x86 miscompiled this at optimisation level 2 and above,
         * so this is slightly transformed to make it work.
         */
        br = (bits as libc::c_double / ov_time_total(vf, -(1 as libc::c_int))) as libc::c_float;
        return crate::stdlib::rint(br as libc::c_double) as libc::c_long;
    } else if (*vf).seekable != 0 {
        /* return the actual bitrate */
        return crate::stdlib::rint(
            ((*(*vf).offsets.offset((i + 1 as libc::c_int) as isize)
                - *(*vf).dataoffsets.offset(i as isize))
                * 8 as libc::c_int as libc::c_long) as libc::c_double
                / ov_time_total(vf, i),
        ) as libc::c_long;
    } else if (*(*vf).vi.offset(i as isize)).bitrate_nominal > 0 as libc::c_int as libc::c_long {
        return (*(*vf).vi.offset(i as isize)).bitrate_nominal;
    } else {
        if (*(*vf).vi.offset(i as isize)).bitrate_upper > 0 as libc::c_int as libc::c_long {
            if (*(*vf).vi.offset(i as isize)).bitrate_lower > 0 as libc::c_int as libc::c_long {
                return ((*(*vf).vi.offset(i as isize)).bitrate_upper
                    + (*(*vf).vi.offset(i as isize)).bitrate_lower)
                    / 2 as libc::c_int as libc::c_long;
            } else {
                return (*(*vf).vi.offset(i as isize)).bitrate_upper;
            }
        }
        return -(1 as libc::c_int) as libc::c_long;
    };
}
/* return nominal if set */
/* returns the actual bitrate since last call.  returns -1 if no
   additional data to offer since last call (or at beginning of stream),
   EINVAL if stream is only partially open
*/
#[no_mangle]

pub unsafe extern "C" fn ov_bitrate_instant(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_long {
    let mut link: libc::c_int = if (*vf).seekable != 0 {
        (*vf).current_link
    } else {
        0 as libc::c_int
    };
    let mut ret: libc::c_long = 0;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_long;
    }
    if (*vf).samptrack == 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int) as libc::c_long;
    }
    ret = ((*vf).bittrack / (*vf).samptrack
        * (*(*vf).vi.offset(link as isize)).rate as libc::c_double
        + 0.5f64) as libc::c_long;
    (*vf).bittrack = 0.0f32 as libc::c_double;
    (*vf).samptrack = 0.0f32 as libc::c_double;
    return ret;
}
/* Guess */
#[no_mangle]

pub unsafe extern "C" fn ov_serialnumber(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut i: libc::c_int,
) -> libc::c_long {
    if i >= (*vf).links {
        return ov_serialnumber(vf, (*vf).links - 1 as libc::c_int);
    }
    if (*vf).seekable == 0 && i >= 0 as libc::c_int {
        return ov_serialnumber(vf, -(1 as libc::c_int));
    }
    if i < 0 as libc::c_int {
        return (*vf).current_serialno;
    } else {
        return *(*vf).serialnos.offset(i as isize);
    };
}
/* returns: total raw (compressed) length of content if i==-1
            raw (compressed) length of that logical bitstream for i==0 to n
            OV_EINVAL if the stream is not seekable (we can't know the length)
            or if stream is only partially open
*/
#[no_mangle]

pub unsafe extern "C" fn ov_raw_total(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut i: libc::c_int,
) -> crate::config_types_h::ogg_int64_t {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    if (*vf).seekable == 0 || i >= (*vf).links {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    if i < 0 as libc::c_int {
        let mut acc: crate::config_types_h::ogg_int64_t =
            0 as libc::c_int as crate::config_types_h::ogg_int64_t;
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*vf).links {
            acc += ov_raw_total(vf, i_0);
            i_0 += 1
        }
        return acc;
    } else {
        return *(*vf).offsets.offset((i + 1 as libc::c_int) as isize)
            - *(*vf).offsets.offset(i as isize);
    };
}
/* returns: total PCM length (samples) of content if i==-1 PCM length
            (samples) of that logical bitstream for i==0 to n
            OV_EINVAL if the stream is not seekable (we can't know the
            length) or only partially open
*/
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_total(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut i: libc::c_int,
) -> crate::config_types_h::ogg_int64_t {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    if (*vf).seekable == 0 || i >= (*vf).links {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    if i < 0 as libc::c_int {
        let mut acc: crate::config_types_h::ogg_int64_t =
            0 as libc::c_int as crate::config_types_h::ogg_int64_t;
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*vf).links {
            acc += ov_pcm_total(vf, i_0);
            i_0 += 1
        }
        return acc;
    } else {
        return *(*vf)
            .pcmlengths
            .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
    };
}
/* returns: total seconds of content if i==-1
            seconds in that logical bitstream for i==0 to n
            OV_EINVAL if the stream is not seekable (we can't know the
            length) or only partially open
*/
#[no_mangle]

pub unsafe extern "C" fn ov_time_total(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut i: libc::c_int,
) -> libc::c_double {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_double;
    }
    if (*vf).seekable == 0 || i >= (*vf).links {
        return -(131 as libc::c_int) as libc::c_double;
    }
    if i < 0 as libc::c_int {
        let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*vf).links {
            acc += ov_time_total(vf, i_0);
            i_0 += 1
        }
        return acc;
    } else {
        return *(*vf)
            .pcmlengths
            .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_double
            / (*(*vf).vi.offset(i as isize)).rate as libc::c_double;
    };
}
/* seek to an offset relative to the *compressed* data. This also
scans packets to update the PCM cursor. It will cross a logical
bitstream boundary, but only if it can't get any packets out of the
tail of the bitstream we seek to (so no surprises).

returns zero on success, nonzero on failure */
#[no_mangle]

pub unsafe extern "C" fn ov_raw_seek(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    let mut work_os: crate::ogg_h::ogg_stream_state = crate::ogg_h::ogg_stream_state {
        body_data: 0 as *mut libc::c_uchar,
        body_storage: 0,
        body_fill: 0,
        body_returned: 0,
        lacing_vals: 0 as *mut libc::c_int,
        granule_vals: 0 as *mut crate::config_types_h::ogg_int64_t,
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
    }; /* don't dump machine if we can't seek */
    let mut ret: libc::c_int = 0;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    if (*vf).seekable == 0 {
        return -(138 as libc::c_int);
    }
    if pos < 0 as libc::c_int as libc::c_long || pos > (*vf).end {
        return -(131 as libc::c_int);
    }
    /* is the seek position outside our current link [if any]? */
    if (*vf).ready_state >= 3 as libc::c_int {
        if pos < *(*vf).offsets.offset((*vf).current_link as isize)
            || pos
                >= *(*vf)
                    .offsets
                    .offset(((*vf).current_link + 1 as libc::c_int) as isize)
        {
            _decode_clear(vf);
        }
        /* clear out stream state */
    }
    /* don't yet clear out decoding machine (if it's initialized), in
    the case we're in the same link.  Restart the decode lapping, and
    let _fetch_and_process_packet deal with a potential bitstream
    boundary */
    (*vf).pcm_offset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t; /* must set serialno */
    crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
        &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        (*vf).current_serialno as libc::c_int,
    );
    crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_restart(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
    );
    ret = _seek_helper(vf, pos);
    if ret != 0 {
        /* dump the machine so we're in a known state */
        (*vf).pcm_offset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        crate::src::libogg_1_3_3::src::framing::ogg_stream_clear(
            &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        );
        _decode_clear(vf);
        return -(137 as libc::c_int);
    } else {
        /* we need to make sure the pcm_offset is set, but we don't want to
           advance the raw cursor past good packets just to get to the first
           with a granulepos.  That's not equivalent behavior to beginning
           decoding as immediately after the seek position as possible.

           So, a hack.  We use two stream states; a local scratch state and
           the shared vf->os stream state.  We use the local state to
           scan, and the shared state as a buffer for later decode.

           Unfortuantely, on the last page we still advance to last packet
           because the granulepos on the last page is not necessarily on a
           packet boundary, and we need to make sure the granpos is
           correct.
        */
        let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
            header: 0 as *mut libc::c_uchar,
            header_len: 0,
            body: 0 as *mut libc::c_uchar,
            body_len: 0,
        }; /* get the memory ready */
        let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
            packet: 0 as *mut libc::c_uchar,
            bytes: 0,
            b_o_s: 0,
            e_o_s: 0,
            granulepos: 0,
            packetno: 0,
        }; /* eliminate the spurious OV_HOLE
           return from not necessarily
           starting from the beginning */
        let mut lastblock: libc::c_int = 0 as libc::c_int;
        let mut accblock: libc::c_int = 0 as libc::c_int;
        let mut thisblock: libc::c_int = 0 as libc::c_int;
        let mut lastflag: libc::c_int = 0 as libc::c_int;
        let mut firstflag: libc::c_int = 0 as libc::c_int;
        let mut pagepos: crate::config_types_h::ogg_int64_t =
            -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
        crate::src::libogg_1_3_3::src::framing::ogg_stream_init(
            &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
            (*vf).current_serialno as libc::c_int,
        );
        crate::src::libogg_1_3_3::src::framing::ogg_stream_reset(
            &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        );
        loop {
            if (*vf).ready_state >= 3 as libc::c_int {
                /* snarf/scan a packet if we can */
                let mut result: libc::c_int =
                    crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                        &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                        &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                    );
                if result > 0 as libc::c_int {
                    if !(*(*vf).vi.offset((*vf).current_link as isize))
                        .codec_setup
                        .is_null()
                    {
                        thisblock =
                            crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_packet_blocksize(
                                (*vf).vi.offset((*vf).current_link as isize)
                                    as *mut crate::codec_h::vorbis_info,
                                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                            ) as libc::c_int;
                        if thisblock < 0 as libc::c_int {
                            crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                0 as *mut crate::ogg_h::ogg_packet as *mut crate::ogg_h::ogg_packet,
                            );
                            thisblock = 0 as libc::c_int
                        } else if lastflag != 0 && firstflag == 0 {
                            crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                0 as *mut crate::ogg_h::ogg_packet as *mut crate::ogg_h::ogg_packet,
                            );
                        } else if lastblock != 0 {
                            accblock += lastblock + thisblock >> 2 as libc::c_int
                        }
                        if op.granulepos != -(1 as libc::c_int) as libc::c_long {
                            let mut i: libc::c_int = 0;
                            let mut link: libc::c_int = (*vf).current_link;
                            let mut granulepos: crate::config_types_h::ogg_int64_t = op.granulepos
                                - *(*vf).pcmlengths.offset((link * 2 as libc::c_int) as isize);
                            if granulepos < 0 as libc::c_int as libc::c_long {
                                granulepos = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
                            }
                            i = 0 as libc::c_int;
                            while i < link {
                                granulepos += *(*vf)
                                    .pcmlengths
                                    .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
                                i += 1
                            }
                            (*vf).pcm_offset = granulepos - accblock as libc::c_long;
                            if (*vf).pcm_offset < 0 as libc::c_int as libc::c_long {
                                (*vf).pcm_offset =
                                    0 as libc::c_int as crate::config_types_h::ogg_int64_t
                            }
                            break;
                        } else {
                            lastblock = thisblock;
                            continue;
                        }
                    } else {
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            0 as *mut crate::ogg_h::ogg_packet as *mut crate::ogg_h::ogg_packet,
                        );
                    }
                }
            }
            if lastblock == 0 {
                pagepos = _get_next_page(
                    vf,
                    &mut og,
                    -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
                );
                if pagepos < 0 as libc::c_int as libc::c_long {
                    (*vf).pcm_offset = ov_pcm_total(vf, -(1 as libc::c_int));
                    break;
                } else {
                    /* We can't get a guaranteed correct pcm position out of the
                    last page in a stream because it might have a 'short'
                    granpos, which can only be detected in the presence of a
                    preceding page.  However, if the last page is also the first
                    page, the granpos rules of a first page take precedence.  Not
                    only that, but for first==last, the EOS page must be treated
                    as if its a normal first page for the stream to open/play. */
                    /* has our decoding just traversed a bitstream boundary? */
                    if (*vf).ready_state >= 3 as libc::c_int {
                        if (*vf).current_serialno
                            != crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                            ) as libc::c_long
                        {
                            /* two possibilities:
                            1) our decoding just traversed a bitstream boundary
                            2) another stream is multiplexed into this logical section? */
                            if crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
                                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                            ) != 0
                            {
                                /* we traversed */
                                _decode_clear(vf); /* clear out stream state */
                                crate::src::libogg_1_3_3::src::framing::ogg_stream_clear(
                                    &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                );
                            }
                            /* else, do nothing; next loop will scoop another page */
                        }
                    } /* not the desired Vorbis
                      bitstream section; keep
                      trying */
                    if (*vf).ready_state < 3 as libc::c_int {
                        let mut link_0: libc::c_int = 0;
                        let mut serialno: libc::c_long =
                            crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                            ) as libc::c_long;
                        link_0 = 0 as libc::c_int;
                        while link_0 < (*vf).links {
                            if *(*vf).serialnos.offset(link_0 as isize) == serialno {
                                break;
                            }
                            link_0 += 1
                        }
                        if link_0 == (*vf).links {
                            continue;
                        }
                        (*vf).current_link = link_0;
                        (*vf).current_serialno = serialno;
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            serialno as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                            &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            serialno as libc::c_int,
                        );
                        (*vf).ready_state = 3 as libc::c_int;
                        firstflag =
                            (pagepos <= *(*vf).dataoffsets.offset(link_0 as isize)) as libc::c_int
                    }
                    crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                        &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                        &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
                    );
                    crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                        &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                        &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
                    );
                    lastflag = crate::src::libogg_1_3_3::src::framing::ogg_page_eos(
                        &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                    )
                }
            } else {
                /* huh?  Bogus stream with packets but no granulepos */
                (*vf).pcm_offset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
                break;
            }
        }
        crate::src::libogg_1_3_3::src::framing::ogg_stream_clear(
            &mut work_os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
        );
        (*vf).bittrack = 0.0f32 as libc::c_double;
        (*vf).samptrack = 0.0f32 as libc::c_double;
        return 0 as libc::c_int;
    };
}
/* Page granularity seek (faster than sample granularity because we
don't do the last bit of decode to find a specific sample).

Seek to the last [granule marked] page preceding the specified pos
location, such that decoding past the returned point will quickly
arrive at the requested position. */
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_seek_page(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut link: libc::c_int = -(1 as libc::c_int);
    let mut result: crate::config_types_h::ogg_int64_t =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    let mut total: crate::config_types_h::ogg_int64_t = ov_pcm_total(vf, -(1 as libc::c_int));
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    if (*vf).seekable == 0 {
        return -(138 as libc::c_int);
    }
    if pos < 0 as libc::c_int as libc::c_long || pos > total {
        return -(131 as libc::c_int);
    }
    /* which bitstream section does this pcm offset occur in? */
    link = (*vf).links - 1 as libc::c_int;
    while link >= 0 as libc::c_int {
        total -= *(*vf)
            .pcmlengths
            .offset((link * 2 as libc::c_int + 1 as libc::c_int) as isize);
        if pos >= total {
            break;
        }
        link -= 1
    }
    /* Search within the logical bitstream for the page with the highest
    pcm_pos preceding pos.  If we're looking for a position on the
    first page, bisection will halt without finding our position as
    it's before the first explicit granulepos fencepost. That case is
    handled separately below.

    There is a danger here; missing pages or incorrect frame number
    information in the bitstream could make our task impossible.
    Account for that (it would be an error condition) */
    /* new search algorithm originally by HB (Nicholas Vinen) */
    let mut end: crate::config_types_h::ogg_int64_t =
        *(*vf).offsets.offset((link + 1 as libc::c_int) as isize);
    let mut begin: crate::config_types_h::ogg_int64_t = *(*vf).dataoffsets.offset(link as isize);
    let mut begintime: crate::config_types_h::ogg_int64_t =
        *(*vf).pcmlengths.offset((link * 2 as libc::c_int) as isize);
    let mut endtime: crate::config_types_h::ogg_int64_t = *(*vf)
        .pcmlengths
        .offset((link * 2 as libc::c_int + 1 as libc::c_int) as isize)
        + begintime;
    let mut target: crate::config_types_h::ogg_int64_t = pos - total + begintime;
    let mut best: crate::config_types_h::ogg_int64_t =
        -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    let mut got_page: libc::c_int = 0 as libc::c_int;
    let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
        header: 0 as *mut libc::c_uchar,
        header_len: 0,
        body: 0 as *mut libc::c_uchar,
        body_len: 0,
    };
    /* if we have only one page, there will be no bisection.  Grab the page here */
    if begin == end {
        result = _seek_helper(vf, begin) as crate::config_types_h::ogg_int64_t;
        if result != 0 {
            current_block = 11555952146732080064;
        } else {
            result = _get_next_page(
                vf,
                &mut og,
                1 as libc::c_int as crate::config_types_h::ogg_int64_t,
            );
            if result < 0 as libc::c_int as libc::c_long {
                current_block = 11555952146732080064;
            } else {
                got_page = 1 as libc::c_int;
                current_block = 11307063007268554308;
            }
        }
    } else {
        current_block = 11307063007268554308;
    }
    's_118: loop {
        match current_block {
            11555952146732080064 => {
                /* dump machine so we're in a known state */
                (*vf).pcm_offset = -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
                _decode_clear(vf);
                return result as libc::c_int;
            }
            _ =>
            /* bisection loop */
            {
                if begin < end {
                    let mut bisect: crate::config_types_h::ogg_int64_t = 0;
                    if end - begin < 65536 as libc::c_int as libc::c_long {
                        bisect = begin
                    } else {
                        /* take a (pretty decent) guess. */
                        bisect = begin
                            + ((target - begintime) as libc::c_double
                                * (end - begin) as libc::c_double
                                / (endtime - begintime) as libc::c_double)
                                as crate::config_types_h::ogg_int64_t
                            - 65536 as libc::c_int as libc::c_long;
                        if bisect < begin + 65536 as libc::c_int as libc::c_long {
                            bisect = begin
                        }
                    }
                    result = _seek_helper(vf, bisect) as crate::config_types_h::ogg_int64_t;
                    if result != 0 {
                        current_block = 11555952146732080064;
                        continue;
                    }
                    loop
                    /* read loop within the bisection loop */
                    {
                        if !(begin < end) {
                            current_block = 11307063007268554308;
                            break;
                        }
                        result = _get_next_page(vf, &mut og, end - (*vf).offset);
                        if result == -(128 as libc::c_int) as libc::c_long {
                            current_block = 11555952146732080064;
                            break;
                        }
                        if result < 0 as libc::c_int as libc::c_long {
                            /* there is no next page! */
                            if bisect <= begin + 1 as libc::c_int as libc::c_long {
                                /* No bisection left to perform.  We've either found the
                                best candidate already or failed. Exit loop. */
                                end = begin
                            } else {
                                /* We tried to load a fraction of the last page; back up a
                                bit and try to get the whole last page */
                                if bisect == 0 as libc::c_int as libc::c_long {
                                    current_block = 11555952146732080064;
                                    break;
                                }
                                bisect -= 65536 as libc::c_int as libc::c_long;
                                /* don't repeat/loop on a read we've already performed */
                                if bisect <= begin {
                                    bisect = begin + 1 as libc::c_int as libc::c_long
                                }
                                /* seek and cntinue bisection */
                                result =
                                    _seek_helper(vf, bisect) as crate::config_types_h::ogg_int64_t;
                                if result != 0 {
                                    current_block = 11555952146732080064;
                                    break;
                                }
                            }
                        } else {
                            let mut granulepos: crate::config_types_h::ogg_int64_t = 0;
                            got_page = 1 as libc::c_int;
                            /* got a page. analyze it */
                            /* only consider pages from primary vorbis stream */
                            if crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                            ) as libc::c_long
                                != *(*vf).serialnos.offset(link as isize)
                            {
                                continue;
                            }
                            /* only consider pages with the granulepos set */
                            granulepos =
                                crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos(
                                    &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                                );
                            if granulepos == -(1 as libc::c_int) as libc::c_long {
                                continue;
                            }
                            if granulepos < target {
                                /* this page is a successful candidate! Set state */
                                best = result; /* raw offset of packet with granulepos */
                                /* *not* begin + 1 as above */
                                begin = (*vf).offset; /* raw offset of next page */
                                begintime = granulepos;
                                /* if we're before our target but within a short distance,
                                don't bisect; read forward */
                                if target - begintime > 44100 as libc::c_int as libc::c_long {
                                    current_block = 11307063007268554308;
                                    break;
                                }
                                bisect = begin
                            } else if bisect <= begin + 1 as libc::c_int as libc::c_long {
                                /* This is one of our pages, but the granpos is
                                post-target; it is not a bisection return
                                candidate. (The only way we'd use it is if it's the
                                first page in the stream; we handle that case later
                                outside the bisection) */
                                /* No bisection left to perform.  We've either found the
                                best candidate already or failed. Exit loop. */
                                end = begin
                            } else if end == (*vf).offset {
                                /* bisection read to the end; use the known page
                                boundary (result) to update bisection, back up a
                                little bit, and try again */
                                end = result;
                                bisect -= 65536 as libc::c_int as libc::c_long;
                                if bisect <= begin {
                                    bisect = begin + 1 as libc::c_int as libc::c_long
                                }
                                result =
                                    _seek_helper(vf, bisect) as crate::config_types_h::ogg_int64_t;
                                if result != 0 {
                                    current_block = 11555952146732080064;
                                    break;
                                }
                            } else {
                                /* Normal bisection */
                                end = bisect;
                                endtime = granulepos;
                                current_block = 11307063007268554308;
                                break;
                            }
                        }
                    }
                } else {
                    /* Out of bisection: did it 'fail?' */
                    if best == -(1 as libc::c_int) as libc::c_long {
                        /* Check the 'looking for data in first page' special case;
                        bisection would 'fail' because our search target was before the
                        first PCM granule position fencepost. */
                        if !(got_page != 0
                            && begin == *(*vf).dataoffsets.offset(link as isize)
                            && crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                            ) as libc::c_long
                                == *(*vf).serialnos.offset(link as isize))
                        {
                            current_block = 11555952146732080064;
                            continue;
                        }
                        /* Yes, this is the beginning-of-stream case. We already have
                        our page, right at the beginning of PCM data.  Set state
                        and return. */
                        (*vf).pcm_offset = total;
                        if link != (*vf).current_link {
                            /* Different link; dump entire decode machine */
                            _decode_clear(vf);
                            (*vf).current_link = link;
                            (*vf).current_serialno = *(*vf).serialnos.offset(link as isize);
                            (*vf).ready_state = 3 as libc::c_int
                        } else {
                            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_restart(
                                &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                            );
                        }
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            (*vf).current_serialno as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
                        );
                    } else {
                        /* Bisection found our page. seek to it, update pcm offset. Easier case than
                        raw_seek, don't keep packets preceding granulepos. */
                        let mut og_0: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
                            header: 0 as *mut libc::c_uchar,
                            header_len: 0,
                            body: 0 as *mut libc::c_uchar,
                            body_len: 0,
                        };
                        let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
                            packet: 0 as *mut libc::c_uchar,
                            bytes: 0,
                            b_o_s: 0,
                            e_o_s: 0,
                            granulepos: 0,
                            packetno: 0,
                        };
                        /* seek */
                        result = _seek_helper(vf, best) as crate::config_types_h::ogg_int64_t;
                        (*vf).pcm_offset =
                            -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t;
                        if result != 0 {
                            current_block = 11555952146732080064;
                            continue;
                        }
                        result = _get_next_page(
                            vf,
                            &mut og_0,
                            -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
                        );
                        if result < 0 as libc::c_int as libc::c_long {
                            current_block = 11555952146732080064;
                            continue;
                        }
                        if link != (*vf).current_link {
                            /* Different link; dump entire decode machine */
                            _decode_clear(vf);
                            (*vf).current_link = link;
                            (*vf).current_serialno = *(*vf).serialnos.offset(link as isize);
                            (*vf).ready_state = 3 as libc::c_int
                        } else {
                            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_restart(
                                &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                            );
                        }
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            (*vf).current_serialno as libc::c_int,
                        );
                        crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                            &mut og_0 as *mut _ as *mut crate::ogg_h::ogg_page,
                        );
                        loop
                        /* pull out all but last packet; the one with granulepos */
                        {
                            result = crate::src::libogg_1_3_3::src::framing::ogg_stream_packetpeek(
                                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                            )
                                as crate::config_types_h::ogg_int64_t;
                            if result == 0 as libc::c_int as libc::c_long {
                                /* No packet returned; we exited the bisection with 'best'
                                pointing to a page with a granule position, so the packet
                                finishing this page ('best') originated on a preceding
                                page. Keep fetching previous pages until we get one with
                                a granulepos or without the 'continued' flag set.  Then
                                just use raw_seek for simplicity. */
                                /* Do not rewind past the beginning of link data; if we do,
                                it's either a bug or a broken stream */
                                result = best;
                                while result > *(*vf).dataoffsets.offset(link as isize) {
                                    result = _get_prev_page(vf, result, &mut og_0);
                                    if result < 0 as libc::c_int as libc::c_long {
                                        current_block = 11555952146732080064;
                                        continue 's_118;
                                    }
                                    if crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(&mut og_0 as *mut _ as *const crate::ogg_h::ogg_page) as
                                               libc::c_long ==
                                               (*vf).current_serialno &&
                                               (crate::src::libogg_1_3_3::src::framing::ogg_page_granulepos(&mut og_0 as *mut _ as *const crate::ogg_h::ogg_page)
                                                    >
                                                    -(1 as libc::c_int) as
                                                        libc::c_long ||
                                                    crate::src::libogg_1_3_3::src::framing::ogg_page_continued(&mut og_0 as *mut _ as *const crate::ogg_h::ogg_page)
                                                        == 0) {
                                            return ov_raw_seek(vf, result)
                                        }
                                }
                            }
                            if result < 0 as libc::c_int as libc::c_long {
                                result =
                                    -(136 as libc::c_int) as crate::config_types_h::ogg_int64_t;
                                current_block = 11555952146732080064;
                                continue 's_118;
                            } else if op.granulepos != -(1 as libc::c_int) as libc::c_long {
                                (*vf).pcm_offset = op.granulepos
                                    - *(*vf)
                                        .pcmlengths
                                        .offset(((*vf).current_link * 2 as libc::c_int) as isize);
                                if (*vf).pcm_offset < 0 as libc::c_int as libc::c_long {
                                    (*vf).pcm_offset =
                                        0 as libc::c_int as crate::config_types_h::ogg_int64_t
                                }
                                (*vf).pcm_offset += total;
                                break;
                            } else {
                                result =
                                    crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                                        &mut (*vf).os as *mut _
                                            as *mut crate::ogg_h::ogg_stream_state,
                                        0 as *mut crate::ogg_h::ogg_packet
                                            as *mut crate::ogg_h::ogg_packet,
                                    )
                                        as crate::config_types_h::ogg_int64_t
                            }
                        }
                    }
                    /* verify result */
                    if (*vf).pcm_offset > pos || pos > ov_pcm_total(vf, -(1 as libc::c_int)) {
                        result = -(129 as libc::c_int) as crate::config_types_h::ogg_int64_t;
                        current_block = 11555952146732080064;
                    } else {
                        (*vf).bittrack = 0.0f32 as libc::c_double;
                        (*vf).samptrack = 0.0f32 as libc::c_double;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
}
/* seek to a sample offset relative to the decompressed pcm stream
returns zero on success, nonzero on failure */
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_seek(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    let mut thisblock: libc::c_int = 0;
    let mut lastblock: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = ov_pcm_seek_page(vf, pos);
    if ret < 0 as libc::c_int {
        return ret;
    }
    ret = _make_decode_ready(vf);
    if ret != 0 {
        return ret;
    }
    loop
    /* discard leading packets we don't need for the lapping of the
    position we want; don't decode them */
    {
        let mut op: crate::ogg_h::ogg_packet = crate::ogg_h::ogg_packet {
            packet: 0 as *mut libc::c_uchar,
            bytes: 0,
            b_o_s: 0,
            e_o_s: 0,
            granulepos: 0,
            packetno: 0,
        };
        let mut og: crate::ogg_h::ogg_page = crate::ogg_h::ogg_page {
            header: 0 as *mut libc::c_uchar,
            header_len: 0,
            body: 0 as *mut libc::c_uchar,
            body_len: 0,
        };
        let mut ret_0: libc::c_int = crate::src::libogg_1_3_3::src::framing::ogg_stream_packetpeek(
            &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
            &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
        );
        if ret_0 > 0 as libc::c_int {
            thisblock = crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_packet_blocksize(
                (*vf).vi.offset((*vf).current_link as isize) as *mut crate::codec_h::vorbis_info,
                &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
            ) as libc::c_int;
            if thisblock < 0 as libc::c_int {
                crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                    &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                    0 as *mut crate::ogg_h::ogg_packet as *mut crate::ogg_h::ogg_packet,
                );
            /* non audio packet */
            } else {
                if lastblock != 0 {
                    (*vf).pcm_offset += (lastblock + thisblock >> 2 as libc::c_int) as libc::c_long
                }
                if (*vf).pcm_offset
                    + (thisblock
                        + crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
                            (*vf).vi as *mut crate::codec_h::vorbis_info,
                            1 as libc::c_int,
                        )
                        >> 2 as libc::c_int) as libc::c_long
                    >= pos
                {
                    break;
                }
                /* remove the packet from packet queue and track its granulepos */
                crate::src::libogg_1_3_3::src::framing::ogg_stream_packetout(
                    &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                    0 as *mut crate::ogg_h::ogg_packet as *mut crate::ogg_h::ogg_packet,
                ); /* set up a vb with
                   only tracking, no
                   pcm_decode */
                crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_trackonly(
                    &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
                    &mut op as *mut _ as *mut crate::ogg_h::ogg_packet,
                );
                crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_blockin(
                    &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                    &mut (*vf).vb as *mut _ as *mut crate::codec_h::vorbis_block,
                );
                /* end of logical stream case is hard, especially with exact
                length positioning. */
                if op.granulepos > -(1 as libc::c_int) as libc::c_long {
                    let mut i: libc::c_int = 0;
                    /* always believe the stream markers */
                    (*vf).pcm_offset = op.granulepos
                        - *(*vf)
                            .pcmlengths
                            .offset(((*vf).current_link * 2 as libc::c_int) as isize);
                    if (*vf).pcm_offset < 0 as libc::c_int as libc::c_long {
                        (*vf).pcm_offset = 0 as libc::c_int as crate::config_types_h::ogg_int64_t
                    }
                    i = 0 as libc::c_int;
                    while i < (*vf).current_link {
                        (*vf).pcm_offset += *(*vf)
                            .pcmlengths
                            .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize);
                        i += 1
                    }
                }
                lastblock = thisblock
            }
        } else {
            if ret_0 < 0 as libc::c_int && ret_0 != -(3 as libc::c_int) {
                break;
            }
            /* suck in a new page */
            if _get_next_page(
                vf,
                &mut og,
                -(1 as libc::c_int) as crate::config_types_h::ogg_int64_t,
            ) < 0 as libc::c_int as libc::c_long
            {
                break;
            }
            if crate::src::libogg_1_3_3::src::framing::ogg_page_bos(
                &mut og as *mut _ as *const crate::ogg_h::ogg_page,
            ) != 0
            {
                _decode_clear(vf);
            }
            if (*vf).ready_state < 3 as libc::c_int {
                let mut serialno: libc::c_long =
                    crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                        &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                    ) as libc::c_long;
                let mut link: libc::c_int = 0;
                link = 0 as libc::c_int;
                while link < (*vf).links {
                    if *(*vf).serialnos.offset(link as isize) == serialno {
                        break;
                    }
                    link += 1
                }
                if link == (*vf).links {
                    continue;
                }
                (*vf).current_link = link;
                (*vf).ready_state = 3 as libc::c_int;
                (*vf).current_serialno = crate::src::libogg_1_3_3::src::framing::ogg_page_serialno(
                    &mut og as *mut _ as *const crate::ogg_h::ogg_page,
                ) as libc::c_long;
                crate::src::libogg_1_3_3::src::framing::ogg_stream_reset_serialno(
                    &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                    serialno as libc::c_int,
                );
                ret_0 = _make_decode_ready(vf);
                if ret_0 != 0 {
                    return ret_0;
                }
                lastblock = 0 as libc::c_int
            }
            crate::src::libogg_1_3_3::src::framing::ogg_stream_pagein(
                &mut (*vf).os as *mut _ as *mut crate::ogg_h::ogg_stream_state,
                &mut og as *mut _ as *mut crate::ogg_h::ogg_page,
            );
        }
    }
    (*vf).bittrack = 0.0f32 as libc::c_double;
    (*vf).samptrack = 0.0f32 as libc::c_double;
    /* discard samples until we reach the desired position. Crossing a
    logical bitstream boundary with abandon is OK. */
    /* note that halfrate could be set differently in each link, but
    vorbisfile encoforces all links are set or unset */
    let mut hs: libc::c_int =
        crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p(
            (*vf).vi as *mut crate::codec_h::vorbis_info,
        );
    while (*vf).pcm_offset < pos >> hs << hs {
        let mut target: crate::config_types_h::ogg_int64_t = pos - (*vf).pcm_offset >> hs;
        let mut samples: libc::c_long =
            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                0 as *mut *mut *mut libc::c_float,
            ) as libc::c_long;
        if samples > target {
            samples = target
        }
        crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_read(
            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
            samples as libc::c_int,
        );
        (*vf).pcm_offset += samples << hs;
        if samples < target {
            if _fetch_and_process_packet(
                vf,
                0 as *mut crate::ogg_h::ogg_packet,
                1 as libc::c_int,
                1 as libc::c_int,
            ) <= 0 as libc::c_int
            {
                (*vf).pcm_offset = ov_pcm_total(vf, -(1 as libc::c_int))
            }
        }
        /* eof */
    }
    return 0 as libc::c_int;
}
/* seek to a playback time relative to the decompressed pcm stream
returns zero on success, nonzero on failure */
#[no_mangle]

pub unsafe extern "C" fn ov_time_seek(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut seconds: libc::c_double,
) -> libc::c_int {
    /* translate time to PCM position and call ov_pcm_seek */
    let mut link: libc::c_int = -(1 as libc::c_int);
    let mut pcm_total: crate::config_types_h::ogg_int64_t =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    let mut time_total: libc::c_double = 0.0f64;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    if (*vf).seekable == 0 {
        return -(138 as libc::c_int);
    }
    if seconds < 0 as libc::c_int as libc::c_double {
        return -(131 as libc::c_int);
    }
    /* which bitstream section does this time offset occur in? */
    link = 0 as libc::c_int;
    while link < (*vf).links {
        let mut addsec: libc::c_double = ov_time_total(vf, link);
        if seconds < time_total + addsec {
            break;
        }
        time_total += addsec;
        pcm_total += *(*vf)
            .pcmlengths
            .offset((link * 2 as libc::c_int + 1 as libc::c_int) as isize);
        link += 1
    }
    if link == (*vf).links {
        return -(131 as libc::c_int);
    }
    /* enough information to convert time offset to pcm offset */
    let mut target: crate::config_types_h::ogg_int64_t = (pcm_total as libc::c_double
        + (seconds - time_total) * (*(*vf).vi.offset(link as isize)).rate as libc::c_double)
        as crate::config_types_h::ogg_int64_t;
    return ov_pcm_seek(vf, target);
}
/* page-granularity version of ov_time_seek
returns zero on success, nonzero on failure */
#[no_mangle]

pub unsafe extern "C" fn ov_time_seek_page(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut seconds: libc::c_double,
) -> libc::c_int {
    /* translate time to PCM position and call ov_pcm_seek */
    let mut link: libc::c_int = -(1 as libc::c_int);
    let mut pcm_total: crate::config_types_h::ogg_int64_t =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    let mut time_total: libc::c_double = 0.0f64;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    if (*vf).seekable == 0 {
        return -(138 as libc::c_int);
    }
    if seconds < 0 as libc::c_int as libc::c_double {
        return -(131 as libc::c_int);
    }
    /* which bitstream section does this time offset occur in? */
    link = 0 as libc::c_int;
    while link < (*vf).links {
        let mut addsec: libc::c_double = ov_time_total(vf, link);
        if seconds < time_total + addsec {
            break;
        }
        time_total += addsec;
        pcm_total += *(*vf)
            .pcmlengths
            .offset((link * 2 as libc::c_int + 1 as libc::c_int) as isize);
        link += 1
    }
    if link == (*vf).links {
        return -(131 as libc::c_int);
    }
    /* enough information to convert time offset to pcm offset */
    let mut target: crate::config_types_h::ogg_int64_t = (pcm_total as libc::c_double
        + (seconds - time_total) * (*(*vf).vi.offset(link as isize)).rate as libc::c_double)
        as crate::config_types_h::ogg_int64_t;
    return ov_pcm_seek_page(vf, target);
}
/* tell the current stream offset cursor.  Note that seek followed by
tell will likely not give the set offset due to caching */
#[no_mangle]

pub unsafe extern "C" fn ov_raw_tell(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> crate::config_types_h::ogg_int64_t {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    return (*vf).offset;
}
/* return PCM offset (sample) of next PCM sample to be read */
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_tell(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> crate::config_types_h::ogg_int64_t {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as crate::config_types_h::ogg_int64_t;
    }
    return (*vf).pcm_offset;
}
/* return time offset (seconds) of next PCM sample to be read */
#[no_mangle]

pub unsafe extern "C" fn ov_time_tell(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_double {
    let mut link: libc::c_int = 0 as libc::c_int;
    let mut pcm_total: crate::config_types_h::ogg_int64_t =
        0 as libc::c_int as crate::config_types_h::ogg_int64_t;
    let mut time_total: libc::c_double = 0.0f32 as libc::c_double;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_double;
    }
    if (*vf).seekable != 0 {
        pcm_total = ov_pcm_total(vf, -(1 as libc::c_int));
        time_total = ov_time_total(vf, -(1 as libc::c_int));
        /* which bitstream section does this time offset occur in? */
        link = (*vf).links - 1 as libc::c_int;
        while link >= 0 as libc::c_int {
            pcm_total -= *(*vf)
                .pcmlengths
                .offset((link * 2 as libc::c_int + 1 as libc::c_int) as isize);
            time_total -= ov_time_total(vf, link);
            if (*vf).pcm_offset >= pcm_total {
                break;
            }
            link -= 1
        }
    }
    return time_total
        + ((*vf).pcm_offset - pcm_total) as libc::c_double
            / (*(*vf).vi.offset(link as isize)).rate as libc::c_double;
}
/*  link:   -1) return the vorbis_info struct for the bitstream section
            currently being decoded
       0-n) to request information for a specific bitstream section

In the case of a non-seekable bitstream, any call returns the
current bitstream.  NULL in the case that the machine is not
initialized */
#[no_mangle]

pub unsafe extern "C" fn ov_info(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut link: libc::c_int,
) -> *mut crate::codec_h::vorbis_info {
    if (*vf).seekable != 0 {
        if link < 0 as libc::c_int {
            if (*vf).ready_state >= 3 as libc::c_int {
                return (*vf).vi.offset((*vf).current_link as isize);
            } else {
                return (*vf).vi;
            }
        } else if link >= (*vf).links {
            return 0 as *mut crate::codec_h::vorbis_info;
        } else {
            return (*vf).vi.offset(link as isize);
        }
    } else {
        return (*vf).vi;
    };
}
/* grr, strong typing, grr, no templates/inheritence, grr */
#[no_mangle]

pub unsafe extern "C" fn ov_comment(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut link: libc::c_int,
) -> *mut crate::codec_h::vorbis_comment {
    if (*vf).seekable != 0 {
        if link < 0 as libc::c_int {
            if (*vf).ready_state >= 3 as libc::c_int {
                return (*vf).vc.offset((*vf).current_link as isize);
            } else {
                return (*vf).vc;
            }
        } else if link >= (*vf).links {
            return 0 as *mut crate::codec_h::vorbis_comment;
        } else {
            return (*vf).vc.offset(link as isize);
        }
    } else {
        return (*vf).vc;
    }; /* deadbeef */
}

unsafe extern "C" fn host_is_big_endian() -> libc::c_int {
    let mut pattern: crate::config_types_h::ogg_int32_t =
        0xfeedface as libc::c_uint as crate::config_types_h::ogg_int32_t;
    let mut bytewise: *mut libc::c_uchar =
        &mut pattern as *mut crate::config_types_h::ogg_int32_t as *mut libc::c_uchar;
    if *bytewise.offset(0 as libc::c_int as isize) as libc::c_int == 0xfe as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* up to this point, everything could more or less hide the multiple
logical bitstream nature of chaining from the toplevel application
if the toplevel application didn't particularly care.  However, at
the point that we actually read audio back, the multiple-section
nature must surface: Multiple bitstream sections do not necessarily
have to have the same number of channels or sampling rate.

ov_read returns the sequential logical bitstream number currently
being decoded along with the PCM data in order that the toplevel
application can take action on channel/sample rate changes.  This
number will be incremented even for streamed (non-seekable) streams
(for seekable streams, it represents the actual logical bitstream
index within the physical bitstream.  Note that the accessor
functions above are aware of this dichotomy).

ov_read_filter is exactly the same as ov_read except that it processes
the decoded audio data through a filter before packing it into the
requested format. This gives greater accuracy than applying a filter
after the audio has been converted into integral PCM.

input values: buffer) a buffer to hold packed PCM data for return
              length) the byte length requested to be placed into buffer
              bigendianp) should the data be packed LSB first (0) or
                          MSB first (1)
              word) word size for output.  currently 1 (byte) or
                    2 (16 bit short)

return values: <0) error/hole in data (OV_HOLE), partial open (OV_EINVAL)
                0) EOF
                n) number of bytes of PCM actually returned.  The
                below works on a packet-by-packet basis, so the
                return length is not related to the 'length' passed
                in, just guaranteed to fit.

         *section) set to the logical bitstream number */
#[no_mangle]

pub unsafe extern "C" fn ov_read_filter(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut buffer: *mut libc::c_char,
    mut length: libc::c_int,
    mut bigendianp: libc::c_int,
    mut word: libc::c_int,
    mut sgned: libc::c_int,
    mut bitstream: *mut libc::c_int,
    mut filter: Option<
        unsafe extern "C" fn(
            _: *mut *mut libc::c_float,
            _: libc::c_long,
            _: libc::c_long,
            _: *mut libc::c_void,
        ) -> (),
    >,
    mut filter_param: *mut libc::c_void,
) -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut host_endian: libc::c_int = host_is_big_endian();
    let mut hs: libc::c_int = 0;
    let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut samples: libc::c_long = 0;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_long;
    }
    loop {
        if (*vf).ready_state == 4 as libc::c_int {
            samples = crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                &mut pcm,
            ) as libc::c_long;
            if samples != 0 {
                break;
            }
        }
        /* suck in another packet */
        let mut ret: libc::c_int = _fetch_and_process_packet(
            vf,
            0 as *mut crate::ogg_h::ogg_packet,
            1 as libc::c_int,
            1 as libc::c_int,
        );
        if ret == -(2 as libc::c_int) {
            return 0 as libc::c_int as libc::c_long;
        }
        if ret <= 0 as libc::c_int {
            return ret as libc::c_long;
        }
    }
    if samples > 0 as libc::c_int as libc::c_long {
        /* yay! proceed to pack data into the byte buffer */
        let mut channels: libc::c_long =
            (*ov_info(vf, -(1 as libc::c_int))).channels as libc::c_long;
        let mut bytespersample: libc::c_long = word as libc::c_long * channels;
        let mut fpu: crate::os_h::vorbis_fpu_control = 0;
        if samples > length as libc::c_long / bytespersample {
            samples = length as libc::c_long / bytespersample
        }
        if samples <= 0 as libc::c_int as libc::c_long {
            return -(131 as libc::c_int) as libc::c_long;
        }
        /* Here. */
        if filter.is_some() {
            filter.expect("non-null function pointer")(pcm, channels, samples, filter_param);
        }
        /* a tight loop to pack each size */
        let mut val: libc::c_int = 0;
        if word == 1 as libc::c_int {
            let mut off: libc::c_int = if sgned != 0 {
                0 as libc::c_int
            } else {
                128 as libc::c_int
            };
            vorbis_fpu_setround(&mut fpu);
            j = 0 as libc::c_int;
            while (j as libc::c_long) < samples {
                i = 0 as libc::c_int;
                while (i as libc::c_long) < channels {
                    val = vorbis_ftoi(
                        (*(*pcm.offset(i as isize)).offset(j as isize) * 128.0f32)
                            as libc::c_double,
                    );
                    if val > 127 as libc::c_int {
                        val = 127 as libc::c_int
                    } else if val < -(128 as libc::c_int) {
                        val = -(128 as libc::c_int)
                    }
                    let fresh3 = buffer;
                    buffer = buffer.offset(1);
                    *fresh3 = (val + off) as libc::c_char;
                    i += 1
                }
                j += 1
            }
            vorbis_fpu_restore(fpu);
        } else {
            let mut off_0: libc::c_int = if sgned != 0 {
                0 as libc::c_int
            } else {
                32768 as libc::c_int
            };
            if host_endian == bigendianp {
                if sgned != 0 {
                    vorbis_fpu_setround(&mut fpu);
                    i = 0 as libc::c_int;
                    while (i as libc::c_long) < channels {
                        /* It's faster in this order */
                        let mut src: *mut libc::c_float = *pcm.offset(i as isize);
                        let mut dest: *mut libc::c_short =
                            (buffer as *mut libc::c_short).offset(i as isize);
                        j = 0 as libc::c_int;
                        while (j as libc::c_long) < samples {
                            val = vorbis_ftoi(
                                (*src.offset(j as isize) * 32768.0f32) as libc::c_double,
                            );
                            if val > 32767 as libc::c_int {
                                val = 32767 as libc::c_int
                            } else if val < -(32768 as libc::c_int) {
                                val = -(32768 as libc::c_int)
                            }
                            *dest = val as libc::c_short;
                            dest = dest.offset(channels as isize);
                            j += 1
                        }
                        i += 1
                    }
                    vorbis_fpu_restore(fpu);
                } else {
                    vorbis_fpu_setround(&mut fpu);
                    i = 0 as libc::c_int;
                    while (i as libc::c_long) < channels {
                        let mut src_0: *mut libc::c_float = *pcm.offset(i as isize);
                        let mut dest_0: *mut libc::c_short =
                            (buffer as *mut libc::c_short).offset(i as isize);
                        j = 0 as libc::c_int;
                        while (j as libc::c_long) < samples {
                            val = vorbis_ftoi(
                                (*src_0.offset(j as isize) * 32768.0f32) as libc::c_double,
                            );
                            if val > 32767 as libc::c_int {
                                val = 32767 as libc::c_int
                            } else if val < -(32768 as libc::c_int) {
                                val = -(32768 as libc::c_int)
                            }
                            *dest_0 = (val + off_0) as libc::c_short;
                            dest_0 = dest_0.offset(channels as isize);
                            j += 1
                        }
                        i += 1
                    }
                    vorbis_fpu_restore(fpu);
                }
            } else if bigendianp != 0 {
                vorbis_fpu_setround(&mut fpu);
                j = 0 as libc::c_int;
                while (j as libc::c_long) < samples {
                    i = 0 as libc::c_int;
                    while (i as libc::c_long) < channels {
                        val = vorbis_ftoi(
                            (*(*pcm.offset(i as isize)).offset(j as isize) * 32768.0f32)
                                as libc::c_double,
                        );
                        if val > 32767 as libc::c_int {
                            val = 32767 as libc::c_int
                        } else if val < -(32768 as libc::c_int) {
                            val = -(32768 as libc::c_int)
                        }
                        val += off_0;
                        let fresh4 = buffer;
                        buffer = buffer.offset(1);
                        *fresh4 = (val >> 8 as libc::c_int) as libc::c_char;
                        let fresh5 = buffer;
                        buffer = buffer.offset(1);
                        *fresh5 = (val & 0xff as libc::c_int) as libc::c_char;
                        i += 1
                    }
                    j += 1
                }
                vorbis_fpu_restore(fpu);
            } else {
                let mut val_0: libc::c_int = 0;
                vorbis_fpu_setround(&mut fpu);
                j = 0 as libc::c_int;
                while (j as libc::c_long) < samples {
                    i = 0 as libc::c_int;
                    while (i as libc::c_long) < channels {
                        val_0 = vorbis_ftoi(
                            (*(*pcm.offset(i as isize)).offset(j as isize) * 32768.0f32)
                                as libc::c_double,
                        );
                        if val_0 > 32767 as libc::c_int {
                            val_0 = 32767 as libc::c_int
                        } else if val_0 < -(32768 as libc::c_int) {
                            val_0 = -(32768 as libc::c_int)
                        }
                        val_0 += off_0;
                        let fresh6 = buffer;
                        buffer = buffer.offset(1);
                        *fresh6 = (val_0 & 0xff as libc::c_int) as libc::c_char;
                        let fresh7 = buffer;
                        buffer = buffer.offset(1);
                        *fresh7 = (val_0 >> 8 as libc::c_int) as libc::c_char;
                        i += 1
                    }
                    j += 1
                }
                vorbis_fpu_restore(fpu);
            }
        }
        crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_read(
            &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
            samples as libc::c_int,
        );
        hs = crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p(
            (*vf).vi as *mut crate::codec_h::vorbis_info,
        );
        (*vf).pcm_offset += samples << hs;
        if !bitstream.is_null() {
            *bitstream = (*vf).current_link
        }
        return samples * bytespersample;
    } else {
        return samples;
    };
}
#[no_mangle]

pub unsafe extern "C" fn ov_read(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut buffer: *mut libc::c_char,
    mut length: libc::c_int,
    mut bigendianp: libc::c_int,
    mut word: libc::c_int,
    mut sgned: libc::c_int,
    mut bitstream: *mut libc::c_int,
) -> libc::c_long {
    return ov_read_filter(
        vf,
        buffer,
        length,
        bigendianp,
        word,
        sgned,
        bitstream,
        None,
        0 as *mut libc::c_void,
    );
}
/* input values: pcm_channels) a float vector per channel of output
              length) the sample length being read by the app

return values: <0) error/hole in data (OV_HOLE), partial open (OV_EINVAL)
                0) EOF
                n) number of samples of PCM actually returned.  The
                below works on a packet-by-packet basis, so the
                return length is not related to the 'length' passed
                in, just guaranteed to fit.

         *section) set to the logical bitstream number */
#[no_mangle]

pub unsafe extern "C" fn ov_read_float(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pcm_channels: *mut *mut *mut libc::c_float,
    mut length: libc::c_int,
    mut bitstream: *mut libc::c_int,
) -> libc::c_long {
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int) as libc::c_long;
    }
    loop {
        if (*vf).ready_state == 4 as libc::c_int {
            let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
            let mut samples: libc::c_long =
                crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                    &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                    &mut pcm,
                ) as libc::c_long;
            if samples != 0 {
                let mut hs: libc::c_int =
                    crate::src::libvorbis_1_3_6::lib::synthesis::vorbis_synthesis_halfrate_p(
                        (*vf).vi as *mut crate::codec_h::vorbis_info,
                    );
                if !pcm_channels.is_null() {
                    *pcm_channels = pcm
                }
                if samples > length as libc::c_long {
                    samples = length as libc::c_long
                }
                crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_read(
                    &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                    samples as libc::c_int,
                );
                (*vf).pcm_offset += samples << hs;
                if !bitstream.is_null() {
                    *bitstream = (*vf).current_link
                }
                return samples;
            }
        }
        /* suck in another packet */
        let mut ret: libc::c_int = _fetch_and_process_packet(
            vf,
            0 as *mut crate::ogg_h::ogg_packet,
            1 as libc::c_int,
            1 as libc::c_int,
        );
        if ret == -(2 as libc::c_int) {
            return 0 as libc::c_int as libc::c_long;
        }
        if ret <= 0 as libc::c_int {
            return ret as libc::c_long;
        }
    }
}

unsafe extern "C" fn _ov_splice(
    mut pcm: *mut *mut libc::c_float,
    mut lappcm: *mut *mut libc::c_float,
    mut n1: libc::c_int,
    mut n2: libc::c_int,
    mut ch1: libc::c_int,
    mut ch2: libc::c_int,
    mut w1: *const libc::c_float,
    mut w2: *const libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: *const libc::c_float = w1;
    let mut n: libc::c_int = n1;
    if n1 > n2 {
        n = n2;
        w = w2
    }
    /* splice */
    j = 0 as libc::c_int;
    while j < ch1 && j < ch2 {
        let mut s: *mut libc::c_float = *lappcm.offset(j as isize);
        let mut d: *mut libc::c_float = *pcm.offset(j as isize);
        i = 0 as libc::c_int;
        while i < n {
            let mut wd: libc::c_float = *w.offset(i as isize) * *w.offset(i as isize);
            let mut ws: libc::c_float = (1.0f64 - wd as libc::c_double) as libc::c_float;
            *d.offset(i as isize) = *d.offset(i as isize) * wd + *s.offset(i as isize) * ws;
            i += 1
        }
        j += 1
    }
    /* window from zero */
    while j < ch2 {
        let mut d_0: *mut libc::c_float = *pcm.offset(j as isize);
        i = 0 as libc::c_int;
        while i < n {
            let mut wd_0: libc::c_float = *w.offset(i as isize) * *w.offset(i as isize);
            *d_0.offset(i as isize) = *d_0.offset(i as isize) * wd_0;
            i += 1
        }
        j += 1
    }
}
/* make sure vf is INITSET */

unsafe extern "C" fn _ov_initset(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    while !((*vf).ready_state == 4 as libc::c_int) {
        /* suck in another packet */
        let mut ret: libc::c_int = _fetch_and_process_packet(
            vf,
            0 as *mut crate::ogg_h::ogg_packet,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        if ret < 0 as libc::c_int && ret != -(3 as libc::c_int) {
            return ret;
        }
    }
    return 0 as libc::c_int;
}
/* make sure vf is INITSET and that we have a primed buffer; if
we're crosslapping at a stream section boundary, this also makes
sure we're sanity checking against the right stream information */

unsafe extern "C" fn _ov_initprime(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    let mut vd: *mut crate::codec_h::vorbis_dsp_state = &mut (*vf).vd;
    loop {
        if (*vf).ready_state == 4 as libc::c_int {
            if crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                vd as *mut crate::codec_h::vorbis_dsp_state,
                0 as *mut *mut *mut libc::c_float,
            ) != 0
            {
                break;
            }
        }
        /* suck in another packet */
        let mut ret: libc::c_int = _fetch_and_process_packet(
            vf,
            0 as *mut crate::ogg_h::ogg_packet,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        if ret < 0 as libc::c_int && ret != -(3 as libc::c_int) {
            return ret;
        }
    }
    return 0 as libc::c_int;
}
/* grab enough data for lapping from vf; this may be in the form of
unreturned, already-decoded pcm, remaining PCM we will need to
decode, or synthetic postextrapolation from last packets. */

unsafe extern "C" fn _ov_getlap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut vi: *mut crate::codec_h::vorbis_info,
    mut vd: *mut crate::codec_h::vorbis_dsp_state,
    mut lappcm: *mut *mut libc::c_float,
    mut lapsize: libc::c_int,
) {
    let mut lapcount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    /* try first to decode the lapping data */
    while lapcount < lapsize {
        let mut samples: libc::c_int =
            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_pcmout(
                vd as *mut crate::codec_h::vorbis_dsp_state,
                &mut pcm,
            );
        if samples != 0 {
            if samples > lapsize - lapcount {
                samples = lapsize - lapcount
            }
            i = 0 as libc::c_int;
            while i < (*vi).channels {
                crate::stdlib::memcpy(
                    (*lappcm.offset(i as isize)).offset(lapcount as isize) as *mut libc::c_void,
                    *pcm.offset(i as isize) as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(samples as libc::c_ulong),
                );
                i += 1
            }
            lapcount += samples;
            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_read(
                vd as *mut crate::codec_h::vorbis_dsp_state,
                samples,
            );
        } else {
            /* suck in another packet */
            let mut ret: libc::c_int = _fetch_and_process_packet(
                vf,
                0 as *mut crate::ogg_h::ogg_packet,
                1 as libc::c_int,
                0 as libc::c_int,
            ); /* do *not* span */
            if ret == -(2 as libc::c_int) {
                break;
            }
        }
    }
    if lapcount < lapsize {
        /* failed to get lapping data from normal decode; pry it from the
        postextrapolation buffering, or the second half of the MDCT
        from the last packet */
        let mut samples_0: libc::c_int =
            crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_lapout(
                &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
                &mut pcm,
            );
        if samples_0 == 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < (*vi).channels {
                crate::stdlib::memset(
                    (*lappcm.offset(i as isize)).offset(lapcount as isize) as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(lapsize as libc::c_ulong)
                        .wrapping_sub(lapcount as libc::c_ulong),
                );
                i += 1
            }
            lapcount = lapsize
        } else {
            if samples_0 > lapsize - lapcount {
                samples_0 = lapsize - lapcount
            }
            i = 0 as libc::c_int;
            while i < (*vi).channels {
                crate::stdlib::memcpy(
                    (*lappcm.offset(i as isize)).offset(lapcount as isize) as *mut libc::c_void,
                    *pcm.offset(i as isize) as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(samples_0 as libc::c_ulong),
                );
                i += 1
            }
            lapcount += samples_0
        }
    };
}
/* this sets up crosslapping of a sample by using trailing data from
sample 1 and lapping it into the windowing buffer of sample 2 */
#[no_mangle]

pub unsafe extern "C" fn ov_crosslap(
    mut vf1: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut vf2: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
) -> libc::c_int {
    let mut vi1: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info; /* degenerate case */
    let mut vi2: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info;
    let mut lappcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut w1: *const libc::c_float = 0 as *const libc::c_float;
    let mut w2: *const libc::c_float = 0 as *const libc::c_float;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut hs1: libc::c_int = 0;
    let mut hs2: libc::c_int = 0;
    if vf1 == vf2 {
        return 0 as libc::c_int;
    }
    if (*vf1).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    if (*vf2).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    /* the relevant overlap buffers must be pre-checked and pre-primed
    before looking at settings in the event that priming would cross
    a bitstream boundary.  So, do it now */
    ret = _ov_initset(vf1);
    if ret != 0 {
        return ret;
    }
    ret = _ov_initprime(vf2);
    if ret != 0 {
        return ret;
    }
    vi1 = ov_info(vf1, -(1 as libc::c_int));
    vi2 = ov_info(vf2, -(1 as libc::c_int));
    hs1 = ov_halfrate_p(vf1);
    hs2 = ov_halfrate_p(vf2);
    let mut fresh8 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            .wrapping_mul((*vi1).channels as libc::c_ulong) as usize,
    );
    lappcm = fresh8.as_mut_ptr() as *mut *mut libc::c_float;
    n1 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi1 as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs1;
    n2 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi2 as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs2;
    w1 = vorbis_window(&mut (*vf1).vd, 0 as libc::c_int);
    w2 = vorbis_window(&mut (*vf2).vd, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*vi1).channels {
        let mut fresh9 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(n1 as libc::c_ulong) as usize,
        );
        let ref mut fresh10 = *lappcm.offset(i as isize);
        *fresh10 = fresh9.as_mut_ptr() as *mut libc::c_float;
        i += 1
    }
    _ov_getlap(vf1, vi1, &mut (*vf1).vd, lappcm, n1);
    /* have a lapping buffer from vf1; now to splice it into the lapping
    buffer of vf2 */
    /* consolidate and expose the buffer. */
    crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_lapout(
        &mut (*vf2).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        &mut pcm,
    );
    /* splice */
    _ov_splice(
        pcm,
        lappcm,
        n1,
        n2,
        (*vi1).channels,
        (*vi2).channels,
        w1,
        w2,
    );
    /* done */
    return 0 as libc::c_int; /* window arrays from libvorbis are
                             persistent; even if the decode state
                             from this link gets dumped, this
                             window array continues to exist */
}

unsafe extern "C" fn _ov_64_seek_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
    mut localseek: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
            _: crate::config_types_h::ogg_int64_t,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut vi: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info;
    let mut lappcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut w1: *const libc::c_float = 0 as *const libc::c_float;
    let mut w2: *const libc::c_float = 0 as *const libc::c_float;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ch1: libc::c_int = 0;
    let mut ch2: libc::c_int = 0;
    let mut hs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    ret = _ov_initset(vf);
    if ret != 0 {
        return ret;
    }
    vi = ov_info(vf, -(1 as libc::c_int));
    hs = ov_halfrate_p(vf);
    ch1 = (*vi).channels;
    n1 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs;
    w1 = vorbis_window(&mut (*vf).vd, 0 as libc::c_int);
    let mut fresh11 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ch1 as libc::c_ulong) as usize,
    );
    lappcm = fresh11.as_mut_ptr() as *mut *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < ch1 {
        let mut fresh12 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(n1 as libc::c_ulong) as usize,
        );
        let ref mut fresh13 = *lappcm.offset(i as isize);
        *fresh13 = fresh12.as_mut_ptr() as *mut libc::c_float;
        i += 1
    }
    _ov_getlap(vf, vi, &mut (*vf).vd, lappcm, n1);
    /* have lapping data; seek and prime the buffer */
    ret = localseek.expect("non-null function pointer")(vf, pos);
    if ret != 0 {
        return ret;
    }
    ret = _ov_initprime(vf);
    if ret != 0 {
        return ret;
    }
    /* Guard against cross-link changes; they're perfectly legal */
    vi = ov_info(vf, -(1 as libc::c_int));
    ch2 = (*vi).channels;
    n2 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs;
    w2 = vorbis_window(&mut (*vf).vd, 0 as libc::c_int);
    /* consolidate and expose the buffer. */
    crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_lapout(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        &mut pcm,
    );
    /* splice */
    _ov_splice(pcm, lappcm, n1, n2, ch1, ch2, w1, w2);
    /* done */
    return 0 as libc::c_int; /* window arrays from libvorbis are
                             persistent; even if the decode state
                             from this link gets dumped, this
                             window array continues to exist */
}
#[no_mangle]

pub unsafe extern "C" fn ov_raw_seek_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    return _ov_64_seek_lap(
        vf,
        pos,
        Some(
            ov_raw_seek
                as unsafe extern "C" fn(
                    _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
                    _: crate::config_types_h::ogg_int64_t,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_seek_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    return _ov_64_seek_lap(
        vf,
        pos,
        Some(
            ov_pcm_seek
                as unsafe extern "C" fn(
                    _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
                    _: crate::config_types_h::ogg_int64_t,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn ov_pcm_seek_page_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: crate::config_types_h::ogg_int64_t,
) -> libc::c_int {
    return _ov_64_seek_lap(
        vf,
        pos,
        Some(
            ov_pcm_seek_page
                as unsafe extern "C" fn(
                    _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
                    _: crate::config_types_h::ogg_int64_t,
                ) -> libc::c_int,
        ),
    );
}

unsafe extern "C" fn _ov_d_seek_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: libc::c_double,
    mut localseek: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
            _: libc::c_double,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut vi: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info;
    let mut lappcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut pcm: *mut *mut libc::c_float = 0 as *mut *mut libc::c_float;
    let mut w1: *const libc::c_float = 0 as *const libc::c_float;
    let mut w2: *const libc::c_float = 0 as *const libc::c_float;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ch1: libc::c_int = 0;
    let mut ch2: libc::c_int = 0;
    let mut hs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if (*vf).ready_state < 2 as libc::c_int {
        return -(131 as libc::c_int);
    }
    ret = _ov_initset(vf);
    if ret != 0 {
        return ret;
    }
    vi = ov_info(vf, -(1 as libc::c_int));
    hs = ov_halfrate_p(vf);
    ch1 = (*vi).channels;
    n1 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs;
    w1 = vorbis_window(&mut (*vf).vd, 0 as libc::c_int);
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ch1 as libc::c_ulong) as usize,
    );
    lappcm = fresh14.as_mut_ptr() as *mut *mut libc::c_float;
    i = 0 as libc::c_int;
    while i < ch1 {
        let mut fresh15 = ::std::vec::from_elem(
            0,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(n1 as libc::c_ulong) as usize,
        );
        let ref mut fresh16 = *lappcm.offset(i as isize);
        *fresh16 = fresh15.as_mut_ptr() as *mut libc::c_float;
        i += 1
    }
    _ov_getlap(vf, vi, &mut (*vf).vd, lappcm, n1);
    /* have lapping data; seek and prime the buffer */
    ret = localseek.expect("non-null function pointer")(vf, pos);
    if ret != 0 {
        return ret;
    }
    ret = _ov_initprime(vf);
    if ret != 0 {
        return ret;
    }
    /* Guard against cross-link changes; they're perfectly legal */
    vi = ov_info(vf, -(1 as libc::c_int));
    ch2 = (*vi).channels;
    n2 = crate::src::libvorbis_1_3_6::lib::info::vorbis_info_blocksize(
        vi as *mut crate::codec_h::vorbis_info,
        0 as libc::c_int,
    ) >> 1 as libc::c_int + hs;
    w2 = vorbis_window(&mut (*vf).vd, 0 as libc::c_int);
    /* consolidate and expose the buffer. */
    crate::src::libvorbis_1_3_6::lib::block::vorbis_synthesis_lapout(
        &mut (*vf).vd as *mut _ as *mut crate::codec_h::vorbis_dsp_state,
        &mut pcm,
    );
    /* splice */
    _ov_splice(pcm, lappcm, n1, n2, ch1, ch2, w1, w2);
    /* done */
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn ov_time_seek_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: libc::c_double,
) -> libc::c_int {
    return _ov_d_seek_lap(
        vf,
        pos,
        Some(
            ov_time_seek
                as unsafe extern "C" fn(
                    _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
                    _: libc::c_double,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]

pub unsafe extern "C" fn ov_time_seek_page_lap(
    mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    mut pos: libc::c_double,
) -> libc::c_int {
    return _ov_d_seek_lap(
        vf,
        pos,
        Some(
            ov_time_seek_page
                as unsafe extern "C" fn(
                    _: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
                    _: libc::c_double,
                ) -> libc::c_int,
        ),
    );
}

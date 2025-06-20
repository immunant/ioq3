use ::libc;

pub use crate::config_types_h::ogg_int64_t;
pub use crate::stddef_h::ptrdiff_t;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::opusfile_0_9::src::opusfile::op_close_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_read_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_seek_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_tell_func;
pub use crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks;

/*The context information needed to read from a block of memory as if it were a
file.*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpusMemStream {
    pub data: *const u8,
    pub size: ptrdiff_t,
    pub pos: ptrdiff_t,
}

unsafe extern "C" fn op_fread(
    mut _stream: *mut libc::c_void,
    mut _ptr: *mut u8,
    mut _buf_size: i32,
) -> i32 {
    let mut stream: *mut FILE = std::ptr::null_mut();
    let mut ret: size_t = 0;
    /*Check for empty read.*/
    if _buf_size <= 0 as i32 {
        return 0 as i32;
    }
    stream = _stream as *mut FILE;
    ret = crate::stdlib::fread(
        _ptr as *mut libc::c_void,
        1 as i32 as usize,
        _buf_size as usize,
        stream,
    );
    /*If ret==0 and !feof(stream), there was a read error.*/
    return if ret > 0 as i32 as usize || crate::stdlib::feof(stream) != 0 {
        ret as i32
    } else {
        -(128 as i32)
    };
}

unsafe extern "C" fn op_fseek(
    mut _stream: *mut libc::c_void,
    mut _offset: i64,
    mut _whence: i32,
) -> i32 {
    /*This function actually conforms to the SUSv2 and POSIX.1-2001, so we prefer
    it except on Windows.*/
    return crate::stdlib::fseeko(_stream as *mut FILE, _offset as off_t, _whence);
}

unsafe extern "C" fn op_ftell(mut _stream: *mut libc::c_void) -> i64 {
    /*This function actually conforms to the SUSv2 and POSIX.1-2001, so we prefer
    it except on Windows.*/
    return crate::stdlib::ftello(_stream as *mut FILE) as i64;
}

static mut OP_FILE_CALLBACKS: OpusFileCallbacks = unsafe {
    {
        let mut init = OpusFileCallbacks {
            read: Some(
                op_fread as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut u8, _: i32) -> i32,
            ),
            seek: Some(
                op_fseek as unsafe extern "C" fn(_: *mut libc::c_void, _: i64, _: i32) -> i32,
            ),
            tell: Some(op_ftell as unsafe extern "C" fn(_: *mut libc::c_void) -> i64),
            close: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut FILE) -> i32>,
                op_close_func,
            >(Some(
                crate::stdlib::fclose as unsafe extern "C" fn(_: *mut FILE) -> i32,
            )),
        };
        init
    }
};
#[no_mangle]

pub unsafe extern "C" fn op_fopen(
    mut _cb: *mut OpusFileCallbacks,
    mut _path: *const libc::c_char,
    mut _mode: *const libc::c_char,
) -> *mut libc::c_void {
    let mut fp: *mut FILE = std::ptr::null_mut();
    fp = crate::stdlib::fopen(_path, _mode);
    if !fp.is_null() {
        *_cb = OP_FILE_CALLBACKS
    }
    return fp as *mut libc::c_void;
}
#[no_mangle]

pub unsafe extern "C" fn op_fdopen(
    mut _cb: *mut OpusFileCallbacks,
    mut _fd: i32,
    mut _mode: *const libc::c_char,
) -> *mut libc::c_void {
    let mut fp: *mut FILE = std::ptr::null_mut();
    fp = crate::stdlib::fdopen(_fd, _mode);
    if !fp.is_null() {
        *_cb = OP_FILE_CALLBACKS
    }
    return fp as *mut libc::c_void;
}
#[no_mangle]

pub unsafe extern "C" fn op_freopen(
    mut _cb: *mut OpusFileCallbacks,
    mut _path: *const libc::c_char,
    mut _mode: *const libc::c_char,
    mut _stream: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut fp: *mut FILE = std::ptr::null_mut();
    fp = crate::stdlib::freopen(_path, _mode, _stream as *mut FILE);
    if !fp.is_null() {
        *_cb = OP_FILE_CALLBACKS
    }
    return fp as *mut libc::c_void;
}

unsafe extern "C" fn op_mem_read(
    mut _stream: *mut libc::c_void,
    mut _ptr: *mut u8,
    mut _buf_size: i32,
) -> i32 {
    let mut stream: *mut OpusMemStream = std::ptr::null_mut();
    let mut size: ptrdiff_t = 0;
    let mut pos: ptrdiff_t = 0;
    stream = _stream as *mut OpusMemStream;
    /*Check for empty read.*/
    if _buf_size <= 0 as i32 {
        return 0 as i32;
    }
    size = (*stream).size;
    pos = (*stream).pos;
    /*Check for EOF.*/
    if pos >= size {
        return 0 as i32;
    }
    /*Check for a short read.*/
    _buf_size = if size - pos < _buf_size as isize {
        (size) - pos
    } else {
        _buf_size as isize
    } as i32;
    crate::stdlib::memcpy(
        _ptr as *mut libc::c_void,
        (*stream).data.offset(pos as isize) as *const libc::c_void,
        _buf_size as usize,
    );
    pos += _buf_size as isize;
    (*stream).pos = pos;
    return _buf_size;
}

unsafe extern "C" fn op_mem_seek(
    mut _stream: *mut libc::c_void,
    mut _offset: i64,
    mut _whence: i32,
) -> i32 {
    let mut stream: *mut OpusMemStream = std::ptr::null_mut();
    let mut pos: ptrdiff_t = 0;
    stream = _stream as *mut OpusMemStream;
    pos = (*stream).pos;
    match _whence {
        0 => {
            /*Check for overflow:*/
            if _offset < 0 as i32 as i64
                || _offset > (!(0 as i32 as size_t) >> 1 as i32) as ptrdiff_t as i64
            {
                return -(1 as i32);
            }
            pos = _offset as ptrdiff_t
        }
        1 => {
            /*Check for overflow:*/
            if _offset < -pos as i64
                || _offset > ((!(0 as i32 as size_t) >> 1 as i32) as ptrdiff_t - pos) as i64
            {
                return -(1 as i32);
            }
            pos = (pos as i64 + _offset) as ptrdiff_t
        }
        2 => {
            let mut size: ptrdiff_t = 0;
            size = (*stream).size;
            /*Check for overflow:*/
            if _offset > size as i64
                || _offset < (size - (!(0 as i32 as size_t) >> 1 as i32) as ptrdiff_t) as i64
            {
                return -(1 as i32);
            }
            pos = (size as i64 - _offset) as ptrdiff_t
        }
        _ => return -(1 as i32),
    }
    (*stream).pos = pos;
    return 0 as i32;
}

unsafe extern "C" fn op_mem_tell(mut _stream: *mut libc::c_void) -> i64 {
    let mut stream: *mut OpusMemStream = std::ptr::null_mut();
    stream = _stream as *mut OpusMemStream;
    return (*stream).pos as i64;
}

unsafe extern "C" fn op_mem_close(mut _stream: *mut libc::c_void) -> i32 {
    libc::free(_stream);
    return 0 as i32;
}

static mut OP_MEM_CALLBACKS: OpusFileCallbacks = {
    let mut init = OpusFileCallbacks {
        read: Some(
            op_mem_read as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut u8, _: i32) -> i32,
        ),
        seek: Some(
            op_mem_seek as unsafe extern "C" fn(_: *mut libc::c_void, _: i64, _: i32) -> i32,
        ),
        tell: Some(op_mem_tell as unsafe extern "C" fn(_: *mut libc::c_void) -> i64),
        close: Some(op_mem_close as unsafe extern "C" fn(_: *mut libc::c_void) -> i32),
    };
    init
};
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
/*@}*/
/*@}*/
/* *\defgroup url_options URL Reading Options*/
/*@{*/
/* *\name URL reading options
Options for op_url_stream_create() and associated functions.
These allow you to provide proxy configuration parameters, skip SSL
 certificate checks, etc.
Options are processed in order, and if the same option is passed multiple
 times, only the value specified by the last occurrence has an effect
 (unless otherwise specified).
They may be expanded in the future.*/
/*@{*/
/* *@cond PRIVATE*/
/*These are the raw numbers used to define the request codes.
They should not be used directly.*/
/*These macros trigger compilation errors or warnings if the wrong types are
provided to one of the URL options.*/
/* *@endcond*/
/* *HTTP/Shoutcast/Icecast server information associated with a URL.*/
/* *The name of the server (icy-name/ice-name).
This is <code>NULL</code> if there was no <code>icy-name</code> or
 <code>ice-name</code> header.*/
/* *A short description of the server (icy-description/ice-description).
This is <code>NULL</code> if there was no <code>icy-description</code> or
 <code>ice-description</code> header.*/
/* *The genre the server falls under (icy-genre/ice-genre).
This is <code>NULL</code> if there was no <code>icy-genre</code> or
 <code>ice-genre</code> header.*/
/* *The homepage for the server (icy-url/ice-url).
This is <code>NULL</code> if there was no <code>icy-url</code> or
 <code>ice-url</code> header.*/
/* *The software used by the origin server (Server).
This is <code>NULL</code> if there was no <code>Server</code> header.*/
/* *The media type of the entity sent to the recepient (Content-Type).
This is <code>NULL</code> if there was no <code>Content-Type</code>
 header.*/
/* *The nominal stream bitrate in kbps (icy-br/ice-bitrate).
This is <code>-1</code> if there was no <code>icy-br</code> or
 <code>ice-bitrate</code> header.*/
/* *Flag indicating whether the server is public (<code>1</code>) or not
 (<code>0</code>) (icy-pub/ice-public).
This is <code>-1</code> if there was no <code>icy-pub</code> or
 <code>ice-public</code> header.*/
/* *Flag indicating whether the server is using HTTPS instead of HTTP.
This is <code>0</code> unless HTTPS is being used.
This may not match the protocol used in the original URL if there were
 redirections.*/
/* *Initializes an #OpusServerInfo structure.
All fields are set as if the corresponding header was not available.
\param _info The #OpusServerInfo structure to initialize.
\note If you use this function, you must link against <tt>libopusurl</tt>.*/
/* *Clears the #OpusServerInfo structure.
This should be called on an #OpusServerInfo structure after it is no longer
 needed.
It will free all memory used by the structure members.
\param _info The #OpusServerInfo structure to clear.
\note If you use this function, you must link against <tt>libopusurl</tt>.*/
/* *Skip the certificate check when connecting via TLS/SSL (https).
\param _b <code>opus_int32</code>: Whether or not to skip the certificate
           check.
          The check will be skipped if \a _b is non-zero, and will not be
           skipped if \a _b is zero.
\hideinitializer*/
/* *Proxy connections through the given host.
If no port is specified via #OP_HTTP_PROXY_PORT, the port number defaults
 to 8080 (http-alt).
All proxy parameters are ignored for non-http and non-https URLs.
\param _host <code>const char *</code>: The proxy server hostname.
             This may be <code>NULL</code> to disable the use of a proxy
              server.
\hideinitializer*/
/* *Use the given port when proxying connections.
This option only has an effect if #OP_HTTP_PROXY_HOST is specified with a
 non-<code>NULL</code> \a _host.
If this option is not provided, the proxy port number defaults to 8080
 (http-alt).
All proxy parameters are ignored for non-http and non-https URLs.
\param _port <code>opus_int32</code>: The proxy server port.
             This must be in the range 0...65535 (inclusive), or the
              URL function this is passed to will fail.
\hideinitializer*/
/* *Use the given user name for authentication when proxying connections.
All proxy parameters are ignored for non-http and non-https URLs.
\param _user const char *: The proxy server user name.
                           This may be <code>NULL</code> to disable proxy
                            authentication.
                           A non-<code>NULL</code> value only has an effect
                            if #OP_HTTP_PROXY_HOST and #OP_HTTP_PROXY_PASS
                            are also specified with non-<code>NULL</code>
                            arguments.
\hideinitializer*/
/* *Use the given password for authentication when proxying connections.
All proxy parameters are ignored for non-http and non-https URLs.
\param _pass const char *: The proxy server password.
                           This may be <code>NULL</code> to disable proxy
                            authentication.
                           A non-<code>NULL</code> value only has an effect
                            if #OP_HTTP_PROXY_HOST and #OP_HTTP_PROXY_USER
                            are also specified with non-<code>NULL</code>
                            arguments.
\hideinitializer*/
/* *Parse information about the streaming server (if any) and return it.
Very little validation is done.
In particular, OpusServerInfo::url may not be a valid URL,
 OpusServerInfo::bitrate_kbps may not really be in kbps, and
 OpusServerInfo::content_type may not be a valid MIME type.
The character set of the string fields is not specified anywhere, and should
 not be assumed to be valid UTF-8.
\param _info OpusServerInfo *: Returns information about the server.
                               If there is any error opening the stream, the
                                contents of this structure remain
                                unmodified.
                               On success, fills in the structure with the
                                server information that was available, if
                                any.
                               After a successful return, the contents of
                                this structure should be freed by calling
                                opus_server_info_clear().
\hideinitializer*/
/*@}*/
/*@}*/
/* *\defgroup stream_callbacks Abstract Stream Reading Interface*/
/*@{*/
/* *\name Functions for reading from streams
These functions define the interface used to read from and seek in a stream
 of data.
A stream does not need to implement seeking, but the decoder will not be
 able to seek if it does not do so.
These functions also include some convenience routines for working with
 standard <code>FILE</code> pointers, complete streams stored in a single
 block of memory, or URLs.*/
/*@{*/
/* *Reads up to \a _nbytes bytes of data from \a _stream.
\param      _stream The stream to read from.
\param[out] _ptr    The buffer to store the data in.
\param      _nbytes The maximum number of bytes to read.
                    This function may return fewer, though it will not
                     return zero unless it reaches end-of-file.
\return The number of bytes successfully read, or a negative value on
         error.*/
/* *Sets the position indicator for \a _stream.
The new position, measured in bytes, is obtained by adding \a _offset
 bytes to the position specified by \a _whence.
If \a _whence is set to <code>SEEK_SET</code>, <code>SEEK_CUR</code>, or
 <code>SEEK_END</code>, the offset is relative to the start of the stream,
 the current position indicator, or end-of-file, respectively.
\retval 0  Success.
\retval -1 Seeking is not supported or an error occurred.
           <code>errno</code> need not be set.*/
/* *Obtains the current value of the position indicator for \a _stream.
\return The current position indicator.*/
/* *Closes the underlying stream.
\retval 0   Success.
\retval EOF An error occurred.
            <code>errno</code> need not be set.*/
/* *The callbacks used to access non-<code>FILE</code> stream resources.
The function prototypes are basically the same as for the stdio functions
 <code>fread()</code>, <code>fseek()</code>, <code>ftell()</code>, and
 <code>fclose()</code>.
The differences are that the <code>FILE *</code> arguments have been
 replaced with a <code>void *</code>, which is to be used as a pointer to
 whatever internal data these functions might need, that #seek and #tell
 take and return 64-bit offsets, and that #seek <em>must</em> return -1 if
 the stream is unseekable.*/
/* *Used to read data from the stream.
This must not be <code>NULL</code>.*/
/* *Used to seek in the stream.
This may be <code>NULL</code> if seeking is not implemented.*/
/* *Used to return the current read position in the stream.
This may be <code>NULL</code> if seeking is not implemented.*/
/* *Used to close the stream when the decoder is freed.
This may be <code>NULL</code> to leave the stream open.*/
/* *Opens a stream with <code>fopen()</code> and fills in a set of callbacks
 that can be used to access it.
This is useful to avoid writing your own portable 64-bit seeking wrappers,
 and also avoids cross-module linking issues on Windows, where a
 <code>FILE *</code> must be accessed by routines defined in the same module
 that opened it.
\param[out] _cb   The callbacks to use for this file.
                  If there is an error opening the file, nothing will be
                   filled in here.
\param      _path The path to the file to open.
                  On Windows, this string must be UTF-8 (to allow access to
                   files whose names cannot be represented in the current
                   MBCS code page).
                  All other systems use the native character encoding.
\param      _mode The mode to open the file in.
\return A stream handle to use with the callbacks, or <code>NULL</code> on
         error.*/
/* *Opens a stream with <code>fdopen()</code> and fills in a set of callbacks
 that can be used to access it.
This is useful to avoid writing your own portable 64-bit seeking wrappers,
 and also avoids cross-module linking issues on Windows, where a
 <code>FILE *</code> must be accessed by routines defined in the same module
 that opened it.
\param[out] _cb   The callbacks to use for this file.
                  If there is an error opening the file, nothing will be
                   filled in here.
\param      _fd   The file descriptor to open.
\param      _mode The mode to open the file in.
\return A stream handle to use with the callbacks, or <code>NULL</code> on
         error.*/
/* *Opens a stream with <code>freopen()</code> and fills in a set of callbacks
 that can be used to access it.
This is useful to avoid writing your own portable 64-bit seeking wrappers,
 and also avoids cross-module linking issues on Windows, where a
 <code>FILE *</code> must be accessed by routines defined in the same module
 that opened it.
\param[out] _cb     The callbacks to use for this file.
                    If there is an error opening the file, nothing will be
                     filled in here.
\param      _path   The path to the file to open.
                    On Windows, this string must be UTF-8 (to allow access
                     to files whose names cannot be represented in the
                     current MBCS code page).
                    All other systems use the native character encoding.
\param      _mode   The mode to open the file in.
\param      _stream A stream previously returned by op_fopen(), op_fdopen(),
                     or op_freopen().
\return A stream handle to use with the callbacks, or <code>NULL</code> on
         error.*/
/* *Creates a stream that reads from the given block of memory.
This block of memory must contain the complete stream to decode.
This is useful for caching small streams (e.g., sound effects) in RAM.
\param[out] _cb   The callbacks to use for this stream.
                  If there is an error creating the stream, nothing will be
                   filled in here.
\param      _data The block of memory to read from.
\param      _size The size of the block of memory.
\return A stream handle to use with the callbacks, or <code>NULL</code> on
         error.*/
#[no_mangle]

pub unsafe extern "C" fn op_mem_stream_create(
    mut _cb: *mut OpusFileCallbacks,
    mut _data: *const u8,
    mut _size: size_t,
) -> *mut libc::c_void {
    let mut stream: *mut OpusMemStream = std::ptr::null_mut();
    if _size > !(0 as i32 as size_t) >> 1 as i32 {
        return std::ptr::null_mut();
    }
    stream = crate::stdlib::malloc(::std::mem::size_of::<OpusMemStream>() as usize)
        as *mut OpusMemStream;
    if !stream.is_null() {
        *_cb = OP_MEM_CALLBACKS;
        (*stream).data = _data;
        (*stream).size = _size as ptrdiff_t;
        (*stream).pos = 0 as i32 as ptrdiff_t
    }
    return stream as *mut libc::c_void;
}

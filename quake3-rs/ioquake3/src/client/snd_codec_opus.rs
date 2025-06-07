use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::uint32_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::internal_h::OggOpusFile;
pub use crate::opus_types_h::opus_int16;
pub use crate::opus_types_h::opus_uint32;
pub use crate::src::client::snd_codec::snd_codec_s;
pub use crate::src::client::snd_codec::snd_codec_t;
pub use crate::src::client::snd_codec::snd_info_s;
pub use crate::src::client::snd_codec::snd_info_t;
pub use crate::src::client::snd_codec::snd_stream_s;
pub use crate::src::client::snd_codec::snd_stream_t;
pub use crate::src::client::snd_codec::S_CodecUtilClose;
pub use crate::src::client::snd_codec::S_CodecUtilOpen;
pub use crate::src::client::snd_codec::CODEC_CLOSE;
pub use crate::src::client::snd_codec::CODEC_LOAD;
pub use crate::src::client::snd_codec::CODEC_OPEN;
pub use crate::src::client::snd_codec::CODEC_READ;
pub use crate::src::opusfile_0_9::src::opusfile::op_close_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_free;
pub use crate::src::opusfile_0_9::src::opusfile::op_head;
pub use crate::src::opusfile_0_9::src::opusfile::op_open_callbacks;
pub use crate::src::opusfile_0_9::src::opusfile::op_pcm_total;
pub use crate::src::opusfile_0_9::src::opusfile::op_read;
pub use crate::src::opusfile_0_9::src::opusfile::op_read_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_seek_func;
pub use crate::src::opusfile_0_9::src::opusfile::op_seekable;
pub use crate::src::opusfile_0_9::src::opusfile::op_tell_func;
pub use crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks;
pub use crate::src::opusfile_0_9::src::opusfile::OpusHead;
pub use crate::src::qcommon::common::Com_Printf;

pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;

// Q3 Ogg Opus codec
#[no_mangle]

pub static mut opus_codec: crate::src::client::snd_codec::snd_codec_t = {
    {
        let mut init = crate::src::client::snd_codec::snd_codec_s {
            ext: b"opus\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            load: Some(
                S_OggOpus_CodecLoad
                    as unsafe extern "C" fn(
                        _: *const libc::c_char,
                        _: *mut crate::src::client::snd_codec::snd_info_t,
                    ) -> *mut libc::c_void,
            ),
            open: Some(
                S_OggOpus_CodecOpenStream
                    as unsafe extern "C" fn(
                        _: *const libc::c_char,
                    )
                        -> *mut crate::src::client::snd_codec::snd_stream_t,
            ),
            read: Some(
                S_OggOpus_CodecReadStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                        _: libc::c_int,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            close: Some(
                S_OggOpus_CodecCloseStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                    ) -> (),
            ),
            next: 0 as *const crate::src::client::snd_codec::snd_codec_t
                as *mut crate::src::client::snd_codec::snd_codec_t,
        };
        init
    }
};
// callbacks for opusfile
// fread() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_Callback_read(
    mut datasource: *mut libc::c_void,
    mut ptr: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut bytesRead: libc::c_int = 0 as libc::c_int;
    // check if input is valid
    if ptr.is_null() {
        *::libc::__errno_location() = 14 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if size == 0 {
        // It's not an error, caller just wants zero bytes!
        *::libc::__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    if size < 0 as libc::c_int {
        *::libc::__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if datasource.is_null() {
        *::libc::__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    // we use a snd_stream_t in the generic pointer to pass around
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    // read it with the Q3 function FS_Read()
    bytesRead = crate::src::qcommon::files::FS_Read(ptr as *mut libc::c_void, size, (*stream).file);
    // update the file position
    (*stream).pos += bytesRead;
    return bytesRead;
}
// fseek() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_Callback_seek(
    mut datasource: *mut libc::c_void,
    mut offset: libc::c_longlong,
    mut whence: libc::c_int,
) -> libc::c_int {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut retVal: libc::c_int = 0 as libc::c_int;
    // check if input is valid
    if datasource.is_null() {
        *::libc::__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int);
    }
    // snd_stream_t in the generic pointer
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    // we must map the whence to its Q3 counterpart
    match whence {
        0 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset as libc::c_long,
                crate::src::qcommon::q_shared::FS_SEEK_SET as libc::c_int,
            );
            // something has gone wrong, so we return here
            if retVal < 0 as libc::c_int {
                return retVal;
            }
            // keep track of file position
            (*stream).pos = offset as libc::c_int
        }
        1 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset as libc::c_long,
                crate::src::qcommon::q_shared::FS_SEEK_CUR as libc::c_int,
            );
            // something has gone wrong, so we return here
            if retVal < 0 as libc::c_int {
                return retVal;
            }
            // keep track of file position
            (*stream).pos += offset as libc::c_int
        }
        2 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset as libc::c_long,
                crate::src::qcommon::q_shared::FS_SEEK_END as libc::c_int,
            );
            // something has gone wrong, so we return here
            if retVal < 0 as libc::c_int {
                return retVal;
            }
            // keep track of file position
            (*stream).pos = (*stream).length + offset as libc::c_int
        }
        _ => {
            // unknown whence, so we return an error
            *::libc::__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    // stream->pos shouldn't be smaller than zero or bigger than the filesize
    (*stream).pos = if (*stream).pos < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (*stream).pos
    };
    (*stream).pos = if (*stream).pos > (*stream).length {
        (*stream).length
    } else {
        (*stream).pos
    };
    return 0 as libc::c_int;
}
// fclose() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_Callback_close(
    mut _datasource: *mut libc::c_void,
) -> libc::c_int {
    // we do nothing here and close all things manually in S_OggOpus_CodecCloseStream()
    return 0 as libc::c_int;
}
// ftell() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_Callback_tell(
    mut datasource: *mut libc::c_void,
) -> libc::c_longlong {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // check if input is valid
    if datasource.is_null() {
        *::libc::__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    // snd_stream_t in the generic pointer
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    return crate::src::qcommon::files::FS_FTell((*stream).file) as libc::c_longlong;
}
// the callback structure
#[no_mangle]

pub static mut S_OggOpus_Callbacks: crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks = {
    let mut init = crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks {
        read: Some(
            S_OggOpus_Callback_read
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: *mut libc::c_uchar,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        seek: Some(
            S_OggOpus_Callback_seek
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_longlong,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        tell: Some(
            S_OggOpus_Callback_tell
                as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_longlong,
        ),
        close: Some(
            S_OggOpus_Callback_close as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int,
        ),
    };
    init
};
/*
=================
S_OggOpus_CodecOpenStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_CodecOpenStream(
    mut filename: *const libc::c_char,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // Opus codec control structure
    let mut of: *mut crate::internal_h::OggOpusFile = 0 as *mut crate::internal_h::OggOpusFile;
    // some variables used to get informations about the file
    let mut opusInfo: *const crate::src::opusfile_0_9::src::opusfile::OpusHead =
        0 as *const crate::src::opusfile_0_9::src::opusfile::OpusHead;
    let mut numSamples: crate::config_types_h::ogg_int64_t = 0;
    // check if input is valid
    if filename.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Open the stream
    stream = crate::src::client::snd_codec::S_CodecUtilOpen(
        filename,
        &mut opus_codec as *mut _ as *mut crate::src::client::snd_codec::snd_codec_s,
    ) as *mut crate::src::client::snd_codec::snd_stream_s;
    if stream.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // open the codec with our callbacks and stream as the generic pointer
    of = crate::src::opusfile_0_9::src::opusfile::op_open_callbacks(
        stream as *mut libc::c_void,
        &S_OggOpus_Callbacks as *const _
            as *const crate::src::opusfile_0_9::src::opusfile::OpusFileCallbacks,
        0 as *const libc::c_uchar,
        0 as libc::c_int as crate::stddef_h::size_t,
        0 as *mut libc::c_int,
    );
    if of.is_null() {
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // the stream must be seekable
    if crate::src::opusfile_0_9::src::opusfile::op_seekable(of) == 0 {
        crate::src::opusfile_0_9::src::opusfile::op_free(of);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the info about channels and rate
    opusInfo = crate::src::opusfile_0_9::src::opusfile::op_head(of, -(1 as libc::c_int))
        as *const crate::src::opusfile_0_9::src::opusfile::OpusHead;
    if opusInfo.is_null() {
        crate::src::opusfile_0_9::src::opusfile::op_free(of);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    if (*opusInfo).stream_count != 1 as libc::c_int {
        crate::src::opusfile_0_9::src::opusfile::op_free(of);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        crate::src::qcommon::common::Com_Printf(
            b"Only Ogg Opus files with one stream are support\n\x00" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    if (*opusInfo).channel_count != 1 as libc::c_int
        && (*opusInfo).channel_count != 2 as libc::c_int
    {
        crate::src::opusfile_0_9::src::opusfile::op_free(of);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        crate::src::qcommon::common::Com_Printf(
            b"Only mono and stereo Ogg Opus files are supported\n\x00" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the number of sample-frames in the file
    numSamples = crate::src::opusfile_0_9::src::opusfile::op_pcm_total(of, -(1 as libc::c_int));
    // fill in the info-structure in the stream
    (*stream).info.rate = 48000 as libc::c_int;
    (*stream).info.width = 2 as libc::c_int;
    (*stream).info.channels = (*opusInfo).channel_count;
    (*stream).info.samples = numSamples as libc::c_int;
    (*stream).info.size = (*stream).info.samples * (*stream).info.channels * (*stream).info.width;
    (*stream).info.dataofs = 0 as libc::c_int;
    // We use stream->pos for the file pointer in the compressed ogg file
    (*stream).pos = 0 as libc::c_int;
    // We use the generic pointer in stream for the opus codec control structure
    (*stream).ptr = of as *mut libc::c_void;
    return stream;
}
/*
=================
S_OggOpus_CodecCloseStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_CodecCloseStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
) {
    // check if input is valid
    if stream.is_null() {
        return;
    }
    // let the opus codec cleanup its stuff
    crate::src::opusfile_0_9::src::opusfile::op_free(
        (*stream).ptr as *mut crate::internal_h::OggOpusFile,
    );
    // close the stream
    crate::src::client::snd_codec::S_CodecUtilClose(
        &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
    );
}
/*
=================
S_OggOpus_CodecReadStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_CodecReadStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
    mut bytes: libc::c_int,
    mut buffer: *mut libc::c_void,
) -> libc::c_int {
    // buffer handling
    let mut samplesRead: libc::c_int = 0;
    let mut samplesLeft: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bufPtr: *mut crate::opus_types_h::opus_int16 =
        0 as *mut crate::opus_types_h::opus_int16;
    // check if input is valid
    if !(!stream.is_null() && !buffer.is_null()) {
        return 0 as libc::c_int;
    }
    if bytes <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    samplesRead = 0 as libc::c_int;
    samplesLeft = bytes / (*stream).info.channels / (*stream).info.width;
    bufPtr = buffer as *mut crate::opus_types_h::opus_int16;
    if samplesLeft <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    // cycle until we have the requested or all available bytes read
    while -(1 as libc::c_int) != 0 {
        // read some samples from the opus codec
        c = crate::src::opusfile_0_9::src::opusfile::op_read(
            (*stream).ptr as *mut crate::internal_h::OggOpusFile,
            bufPtr.offset((samplesRead * (*stream).info.channels) as isize),
            samplesLeft * (*stream).info.channels,
            0 as *mut libc::c_int,
        );
        // no more samples are left
        if c <= 0 as libc::c_int {
            break;
        }
        samplesRead += c;
        samplesLeft -= c;
        // we have enough samples
        if samplesLeft <= 0 as libc::c_int {
            break;
        }
    }
    return samplesRead * (*stream).info.channels * (*stream).info.width;
}
// USE_CODEC_VORBIS
// Ogg Opus codec
/*
=====================================================================
S_OggOpus_CodecLoad

We handle S_OggOpus_CodecLoad as a special case of the streaming functions
where we read the whole stream at once.
======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OggOpus_CodecLoad(
    mut filename: *const libc::c_char,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> *mut libc::c_void {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bytesRead: libc::c_int = 0;
    // check if input is valid
    if !(!filename.is_null() && !info.is_null()) {
        return 0 as *mut libc::c_void;
    }
    // open the file as a stream
    stream = S_OggOpus_CodecOpenStream(filename);
    if stream.is_null() {
        return 0 as *mut libc::c_void;
    }
    // copy over the info
    (*info).rate = (*stream).info.rate;
    (*info).width = (*stream).info.width;
    (*info).channels = (*stream).info.channels;
    (*info).samples = (*stream).info.samples;
    (*info).size = (*stream).info.size;
    (*info).dataofs = (*stream).info.dataofs;
    // allocate a buffer
    // this buffer must be free-ed by the caller of this function
    buffer = crate::src::qcommon::common::Hunk_AllocateTempMemory((*info).size)
        as *mut crate::src::qcommon::q_shared::byte;
    if buffer.is_null() {
        S_OggOpus_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    // fill the buffer
    bytesRead = S_OggOpus_CodecReadStream(stream, (*info).size, buffer as *mut libc::c_void);
    // we don't even have read a single byte
    if bytesRead <= 0 as libc::c_int {
        crate::src::qcommon::common::Hunk_FreeTempMemory(buffer as *mut libc::c_void);
        S_OggOpus_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    S_OggOpus_CodecCloseStream(stream);
    return buffer as *mut libc::c_void;
}
// USE_CODEC_OPUS

use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::config_types_h::ogg_int64_t;
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
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_info;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_open_callbacks;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_pcm_total;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_read;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_seekable;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_streams;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;

pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_comment;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
pub use crate::ogg_h::oggpack_buffer;

// Q3 OGG codec
#[no_mangle]

pub static mut ogg_codec: crate::src::client::snd_codec::snd_codec_t = unsafe {
    {
        let mut init = crate::src::client::snd_codec::snd_codec_s {
            ext: b"ogg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            load: Some(
                S_OGG_CodecLoad
                    as unsafe extern "C" fn(
                        _: *const libc::c_char,
                        _: *mut crate::src::client::snd_codec::snd_info_t,
                    ) -> *mut libc::c_void,
            ),
            open: Some(
                S_OGG_CodecOpenStream
                    as unsafe extern "C" fn(
                        _: *const libc::c_char,
                    )
                        -> *mut crate::src::client::snd_codec::snd_stream_t,
            ),
            read: Some(
                S_OGG_CodecReadStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                        _: libc::c_int,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            close: Some(
                S_OGG_CodecCloseStream
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
// callbacks for vobisfile
// fread() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_read(
    mut ptr: *mut libc::c_void,
    mut size: crate::stddef_h::size_t,
    mut nmemb: crate::stddef_h::size_t,
    mut datasource: *mut libc::c_void,
) -> crate::stddef_h::size_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut byteSize: libc::c_int = 0 as libc::c_int;
    let mut bytesRead: libc::c_int = 0 as libc::c_int;
    let mut nMembRead: crate::stddef_h::size_t = 0 as libc::c_int as crate::stddef_h::size_t;
    // check if input is valid
    if ptr.is_null() {
        *::libc::__errno_location() = 14 as libc::c_int;
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    if !(size != 0 && nmemb != 0) {
        // It's not an error, caller just wants zero bytes!
        *::libc::__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    if datasource.is_null() {
        *::libc::__errno_location() = 9 as libc::c_int;
        return 0 as libc::c_int as crate::stddef_h::size_t;
    }
    // we use a snd_stream_t in the generic pointer to pass around
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    // FS_Read does not support multi-byte elements
    byteSize = nmemb.wrapping_mul(size) as libc::c_int;
    // read it with the Q3 function FS_Read()
    bytesRead = crate::src::qcommon::files::FS_Read(ptr, byteSize, (*stream).file);
    // update the file position
    (*stream).pos += bytesRead;
    // this function returns the number of elements read not the number of bytes
    nMembRead = (bytesRead as libc::c_ulong).wrapping_div(size);
    // even if the last member is only read partially
    // it is counted as a whole in the return value
    if (bytesRead as libc::c_ulong).wrapping_rem(size) != 0 {
        nMembRead = nMembRead.wrapping_add(1)
    }
    return nMembRead;
}
// fseek() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_seek(
    mut datasource: *mut libc::c_void,
    mut offset: crate::config_types_h::ogg_int64_t,
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
                offset,
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
                offset,
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
                offset,
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

pub unsafe extern "C" fn S_OGG_Callback_close(mut _datasource: *mut libc::c_void) -> libc::c_int {
    // we do nothing here and close all things manually in S_OGG_CodecCloseStream()
    return 0 as libc::c_int;
}
// ftell() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_tell(mut datasource: *mut libc::c_void) -> libc::c_long {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // check if input is valid
    if datasource.is_null() {
        *::libc::__errno_location() = 9 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_long;
    }
    // snd_stream_t in the generic pointer
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    return crate::src::qcommon::files::FS_FTell((*stream).file) as libc::c_long;
}
// the callback structure
#[no_mangle]

pub static mut S_OGG_Callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks = {
    let mut init = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks {
        read_func: Some(
            S_OGG_Callback_read
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: crate::stddef_h::size_t,
                    _: crate::stddef_h::size_t,
                    _: *mut libc::c_void,
                ) -> crate::stddef_h::size_t,
        ),
        seek_func: Some(
            S_OGG_Callback_seek
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: crate::config_types_h::ogg_int64_t,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        close_func: Some(
            S_OGG_Callback_close as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int,
        ),
        tell_func: Some(
            S_OGG_Callback_tell as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long,
        ),
    };
    init
};
/*
=================
S_OGG_CodecOpenStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecOpenStream(
    mut filename: *const libc::c_char,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // OGG codec control structure
    let mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File =
        0 as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
    // some variables used to get informations about the OGG
    let mut OGGInfo: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info;
    let mut numSamples: crate::config_types_h::ogg_int64_t = 0;
    // check if input is valid
    if filename.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Open the stream
    stream = crate::src::client::snd_codec::S_CodecUtilOpen(
        filename,
        &mut ogg_codec as *mut _ as *mut crate::src::client::snd_codec::snd_codec_s,
    ) as *mut crate::src::client::snd_codec::snd_stream_s;
    if stream.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // alloctate the OggVorbis_File
    vf = crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<
        crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    >() as libc::c_ulong as libc::c_int)
        as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
    if vf.is_null() {
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // open the codec with our callbacks and stream as the generic pointer
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_open_callbacks(
        stream as *mut libc::c_void,
        vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        0 as *const libc::c_char,
        0 as libc::c_int as libc::c_long,
        S_OGG_Callbacks as crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks,
    ) != 0 as libc::c_int
    {
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // the stream must be seekable
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_seekable(
        vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    ) == 0
    {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(
            vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        );
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // we only support OGGs with one substream
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_streams(
        vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    ) != 1 as libc::c_int as libc::c_long
    {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(
            vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        );
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the info about channels and rate
    OGGInfo = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_info(
        vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        0 as libc::c_int,
    ) as *mut crate::codec_h::vorbis_info;
    if OGGInfo.is_null() {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(
            vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        );
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(
            &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
        );
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the number of sample-frames in the OGG
    numSamples = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_pcm_total(
        vf as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
        0 as libc::c_int,
    );
    // fill in the info-structure in the stream
    (*stream).info.rate = (*OGGInfo).rate as libc::c_int;
    (*stream).info.width = 2 as libc::c_int;
    (*stream).info.channels = (*OGGInfo).channels;
    (*stream).info.samples = numSamples as libc::c_int;
    (*stream).info.size = (*stream).info.samples * (*stream).info.channels * (*stream).info.width;
    (*stream).info.dataofs = 0 as libc::c_int;
    // We use stream->pos for the file pointer in the compressed ogg file
    (*stream).pos = 0 as libc::c_int;
    // We use the generic pointer in stream for the OGG codec control structure
    (*stream).ptr = vf as *mut libc::c_void;
    return stream;
}
/*
=================
S_OGG_CodecCloseStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecCloseStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
) {
    // check if input is valid
    if stream.is_null() {
        return;
    }
    // let the OGG codec cleanup its stuff
    crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(
        (*stream).ptr as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File
            as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    );
    // free the OGG codec control struct
    crate::src::qcommon::common::Z_Free((*stream).ptr);
    // close the stream
    crate::src::client::snd_codec::S_CodecUtilClose(
        &mut stream as *mut _ as *mut *mut crate::src::client::snd_codec::snd_stream_s,
    );
}
/*
=================
S_OGG_CodecReadStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecReadStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
    mut bytes: libc::c_int,
    mut buffer: *mut libc::c_void,
) -> libc::c_int {
    // buffer handling
    let mut bytesRead: libc::c_int = 0;
    let mut bytesLeft: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bufPtr: *mut libc::c_char = 0 as *mut libc::c_char;
    // Bitstream for the decoder
    let mut BS: libc::c_int = 0 as libc::c_int;
    // big endian machines want their samples in big endian order
    let mut IsBigEndian: libc::c_int = 0 as libc::c_int;
    // Q3_BIG_ENDIAN
    // check if input is valid
    if !(!stream.is_null() && !buffer.is_null()) {
        return 0 as libc::c_int;
    }
    if bytes <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    bytesRead = 0 as libc::c_int;
    bytesLeft = bytes;
    bufPtr = buffer as *mut libc::c_char;
    // cycle until we have the requested or all available bytes read
    while -(1 as libc::c_int) != 0 {
        // read some bytes from the OGG codec
        c = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_read(
            (*stream).ptr as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File
                as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
            bufPtr,
            bytesLeft,
            IsBigEndian,
            2 as libc::c_int,
            1 as libc::c_int,
            &mut BS,
        ) as libc::c_int;
        // no more bytes are left
        if c <= 0 as libc::c_int {
            break;
        }
        bytesRead += c;
        bytesLeft -= c;
        bufPtr = bufPtr.offset(c as isize);
        // we have enough bytes
        if bytesLeft <= 0 as libc::c_int {
            break;
        }
    }
    return bytesRead;
}
// Ogg Vorbis codec
/*
=====================================================================
S_OGG_CodecLoad

We handle S_OGG_CodecLoad as a special case of the streaming functions
where we read the whole stream at once.
======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecLoad(
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
    stream = S_OGG_CodecOpenStream(filename);
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
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    // fill the buffer
    bytesRead = S_OGG_CodecReadStream(stream, (*info).size, buffer as *mut libc::c_void);
    // we don't even have read a single byte
    if bytesRead <= 0 as libc::c_int {
        crate::src::qcommon::common::Hunk_FreeTempMemory(buffer as *mut libc::c_void);
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    S_OGG_CodecCloseStream(stream);
    return buffer as *mut libc::c_void;
}
// USE_CODEC_VORBIS

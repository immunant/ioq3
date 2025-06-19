use ::libc;

pub mod stdlib_h {

    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> i32 {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as i32,
        ) as i32;
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128 as i32) && __c < 256 as i32 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::VMI_BYTECODE;
pub use crate::qcommon_h::VMI_COMPILED;
pub use crate::qcommon_h::VMI_NATIVE;
pub use crate::src::client::snd_main::S_ClearSoundBuffer;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_RemoveCommand;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::common::com_basegame;
pub use crate::src::qcommon::common::com_fullyInitialized;
pub use crate::src::qcommon::common::com_journal;
pub use crate::src::qcommon::common::com_journalDataFile;
pub use crate::src::qcommon::common::com_legacyprotocol;
pub use crate::src::qcommon::common::com_protocol;
pub use crate::src::qcommon::common::com_standalone;
pub use crate::src::qcommon::common::demo_protocols;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_FilterPath;
pub use crate::src::qcommon::common::Com_GameRestart;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_SafeMode;
pub use crate::src::qcommon::common::Com_StartupVariable;
pub use crate::src::qcommon::common::CopyString;
pub use crate::src::qcommon::common::Hunk_AllocateTempMemory;
pub use crate::src::qcommon::common::Hunk_ClearTempMemory;
pub use crate::src::qcommon::common::Hunk_FreeTempMemory;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::cvar::Cvar_ForceReset;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::files::ctype_h::tolower;
pub use crate::src::qcommon::files::stdlib_h::atoi;
pub use crate::src::qcommon::md4::Com_BlockChecksum;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::COM_CompareExtension;
pub use crate::src::qcommon::q_shared::COM_GetExtension;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_stricmpn;
pub use crate::src::qcommon::q_shared::Q_strlwr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::unzip::tm_unz;
pub use crate::src::qcommon::unzip::tm_unz_s;
pub use crate::src::qcommon::unzip::unzClose;
pub use crate::src::qcommon::unzip::unzCloseCurrentFile;
pub use crate::src::qcommon::unzip::unzFile;
pub use crate::src::qcommon::unzip::unzGetCurrentFileInfo;
pub use crate::src::qcommon::unzip::unzGetGlobalInfo;
pub use crate::src::qcommon::unzip::unzGetOffset;
pub use crate::src::qcommon::unzip::unzGoToFirstFile;
pub use crate::src::qcommon::unzip::unzGoToNextFile;
pub use crate::src::qcommon::unzip::unzOpen;
pub use crate::src::qcommon::unzip::unzOpenCurrentFile;
pub use crate::src::qcommon::unzip::unzReadCurrentFile;
pub use crate::src::qcommon::unzip::unzSetOffset;
pub use crate::src::qcommon::unzip::unz_file_info;
pub use crate::src::qcommon::unzip::unz_file_info_s;
pub use crate::src::qcommon::unzip::unz_global_info;
pub use crate::src::qcommon::unzip::unz_global_info_s;
pub use crate::src::qcommon::unzip::unztell;
pub use crate::src::sys::sys_main::Sys_DefaultInstallPath;
pub use crate::src::sys::sys_main::Sys_InitPIDFile;
pub use crate::src::sys::sys_main::Sys_RemovePIDFile;
pub use crate::src::sys::sys_unix::Sys_DefaultHomePath;
pub use crate::src::sys::sys_unix::Sys_DllExtension;
pub use crate::src::sys::sys_unix::Sys_FOpen;
pub use crate::src::sys::sys_unix::Sys_FreeFileList;
pub use crate::src::sys::sys_unix::Sys_GogPath;
pub use crate::src::sys::sys_unix::Sys_ListFiles;
pub use crate::src::sys::sys_unix::Sys_Mkdir;
pub use crate::src::sys::sys_unix::Sys_Mkfifo;
pub use crate::src::sys::sys_unix::Sys_SteamPath;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fflush;
pub use crate::stdlib::fread;
pub use crate::stdlib::fseek;
pub use crate::stdlib::ftell;
pub use crate::stdlib::fwrite;

pub use crate::stdlib::off_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::setvbuf;

pub use crate::stdlib::vsnprintf;
pub use crate::zconf_h::uInt;
pub use crate::zconf_h::uLong;
pub use crate::zconf_h::voidp;
pub use ::libc::remove;
pub use ::libc::rename;

pub use ::libc::strtol;
extern "C" {
    #[no_mangle]
    pub fn Com_AppendCDKey(filename: *const libc::c_char);
    #[no_mangle]
    pub fn Com_ReadCDKey(filename: *const libc::c_char);
}

pub type searchpath_t = searchpath_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct searchpath_s {
    pub next: *mut searchpath_s,
    pub pack: *mut pack_t,
    pub dir: *mut directory_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct directory_t {
    pub path: [libc::c_char; 4096],
    pub fullpath: [libc::c_char; 4096],
    pub gamedir: [libc::c_char; 4096],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pack_t {
    pub pakPathname: [libc::c_char; 4096],
    pub pakFilename: [libc::c_char; 4096],
    pub pakBasename: [libc::c_char; 4096],
    pub pakGamename: [libc::c_char; 4096],
    pub handle: crate::src::qcommon::unzip::unzFile,
    pub checksum: i32,
    pub pure_checksum: i32,
    pub numfiles: i32,
    pub referenced: i32,
    pub hashSize: i32,
    pub hashTable: *mut *mut fileInPack_t,
    pub buildBuffer: *mut fileInPack_t,
}

pub type fileInPack_t = fileInPack_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileInPack_s {
    pub name: *mut libc::c_char,
    pub pos: libc::c_ulong,
    pub len: libc::c_ulong,
    pub next: *mut fileInPack_s,
}

pub type qfile_gut = qfile_gus;

#[repr(C)]
#[derive(Copy, Clone)]
pub union qfile_gus {
    pub o: *mut crate::stdlib::FILE,
    pub z: crate::src::qcommon::unzip::unzFile,
}

pub type qfile_ut = qfile_us;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qfile_us {
    pub file: qfile_gut,
    pub unique: crate::src::qcommon::q_shared::qboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileHandleData_t {
    pub handleFiles: qfile_ut,
    pub handleSync: crate::src::qcommon::q_shared::qboolean,
    pub fileSize: i32,
    pub zipFilePos: i32,
    pub zipFileLen: i32,
    pub zipFile: crate::src::qcommon::q_shared::qboolean,
    pub name: [libc::c_char; 256],
}

static mut pak_checksums: [u32; 9] = [
    1566731103 as u32,
    298122907 as u32,
    412165236 as u32,
    2991495316 as u32,
    1197932710 as u32,
    4087071573 as u32,
    3709064859 as u32,
    908855077 as u32,
    977125798 as u32,
];

static mut missionpak_checksums: [u32; 4] = [
    2430342401 as u32,
    511014160 as u32,
    2662638993 as u32,
    1438664554 as u32,
];

static mut fs_gamedir: [libc::c_char; 4096] = [0; 4096];
// only one of pack / dir will be non NULL
// name of the file
// file info position in zip
// uncompress file size
// next file in the hash
// this will be a single file name with no separators

static mut fs_debug: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_homepath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_steampath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_gogpath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_basepath: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_basegame: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_gamedirvar: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;

static mut fs_searchpaths: *mut searchpath_t = 0 as *const searchpath_t as *mut searchpath_t;

static mut fs_readCount: i32 = 0;
// total bytes read

static mut fs_loadCount: i32 = 0;
// total files read

static mut fs_loadStack: i32 = 0;
// total files in memory

static mut fs_packFiles: i32 = 0 as i32;
// total number of files in packs

static mut fs_checksumFeed: i32 = 0;

static mut fsh: [fileHandleData_t; 64] = [fileHandleData_t {
    handleFiles: qfile_ut {
        file: qfile_gus {
            o: 0 as *const crate::stdlib::FILE as *mut crate::stdlib::FILE,
        },
        unique: crate::src::qcommon::q_shared::qfalse,
    },
    handleSync: crate::src::qcommon::q_shared::qfalse,
    fileSize: 0,
    zipFilePos: 0,
    zipFileLen: 0,
    zipFile: crate::src::qcommon::q_shared::qfalse,
    name: [0; 256],
}; 64];
// TTimo - https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=540
// wether we did a reorder on the current search path when joining the server

static mut fs_reordered: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
// never load anything from pk3 files that are not present at the server when pure

static mut fs_numServerPaks: i32 = 0 as i32;

static mut fs_serverPaks: [i32; 4096] = [0; 4096];
// checksums

static mut fs_serverPakNames: [*mut libc::c_char; 4096] =
    [0 as *const libc::c_char as *mut libc::c_char; 4096];
// pk3 names
// only used for autodownload, to make sure the client has at least
// all the pk3 files that are referenced at the server side

static mut fs_numServerReferencedPaks: i32 = 0;

static mut fs_serverReferencedPaks: [i32; 4096] = [0; 4096];
// checksums

static mut fs_serverReferencedPakNames: [*mut libc::c_char; 4096] =
    [0 as *const libc::c_char as *mut libc::c_char; 4096];
// pk3 names
// last valid game folder used
#[no_mangle]

pub static mut lastValidBase: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]

pub static mut lastValidComBaseGame: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]

pub static mut lastValidFsBaseGame: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]

pub static mut lastValidGame: [libc::c_char; 4096] = [0; 4096];
/* C99 defines __func__ */
/*
==============
FS_Initialized
==============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Initialized() -> crate::src::qcommon::q_shared::qboolean {
    return (fs_searchpaths != 0 as *mut libc::c_void as *mut searchpath_t) as i32
        as crate::src::qcommon::q_shared::qboolean;
}
/*
=================
FS_PakIsPure
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_PakIsPure(
    mut pack: *mut pack_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    if fs_numServerPaks != 0 {
        i = 0 as i32;
        while i < fs_numServerPaks {
            // FIXME: also use hashed file names
            // NOTE TTimo: a pk3 with same checksum but different name would be validated too
            //   I don't see this as allowing for any exploit, it would only happen if the client does manips of its file names 'not a bug'
            if (*pack).checksum == fs_serverPaks[i as usize] {
                return crate::src::qcommon::q_shared::qtrue;
                // on the aproved list
            }
            i += 1
        }
        return crate::src::qcommon::q_shared::qfalse;
        // not on the pure server pak list
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
FS_LoadStack
return load stack
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_LoadStack() -> i32 {
    return fs_loadStack;
}
/*
================
return a hash value for the filename
================
*/

unsafe extern "C" fn FS_HashFileName(
    mut fname: *const libc::c_char,
    mut hashSize: i32,
) -> libc::c_long {
    let mut i: i32 = 0; // don't include extension
    let mut hash: libc::c_long = 0; // damn path names
    let mut letter: libc::c_char = 0; // damn path names
    hash = 0 as i32 as libc::c_long;
    i = 0 as i32;
    while *fname.offset(i as isize) as i32 != '\u{0}' as i32 {
        letter = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as i32 as libc::c_ulong {
                if 0 != 0 {
                    let mut __c: i32 = *fname.offset(i as isize) as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*fname.offset(i as isize) as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc())
                    .offset(*fname.offset(i as isize) as i32 as isize)
            }
            __res
        }) as libc::c_char;
        if letter as i32 == '.' as i32 {
            break;
        }
        if letter as i32 == '\\' as i32 {
            letter = '/' as i32 as libc::c_char
        }
        if letter as i32 == '/' as i32 {
            letter = '/' as i32 as libc::c_char
        }
        hash += letter as libc::c_long * (i + 119 as i32) as libc::c_long;
        i += 1
    }
    hash = hash ^ hash >> 10 as i32 ^ hash >> 20 as i32;
    hash &= (hashSize - 1 as i32) as libc::c_long;
    return hash;
}

unsafe extern "C" fn FS_HandleForFile() -> crate::src::qcommon::q_shared::fileHandle_t {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < 64 as i32 {
        if fsh[i as usize].handleFiles.file.o.is_null() {
            return i;
        }
        i += 1
    }
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"FS_HandleForFile: none free\x00" as *const u8 as *const libc::c_char,
    );
}

unsafe extern "C" fn FS_FileForHandle(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) -> *mut crate::stdlib::FILE {
    if f < 1 as i32 || f >= 64 as i32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"FS_FileForHandle: out of range\x00" as *const u8 as *const libc::c_char,
        );
    }
    if fsh[f as usize].zipFile as u32 == crate::src::qcommon::q_shared::qtrue as i32 as u32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"FS_FileForHandle: can\'t get FILE on zip file\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if fsh[f as usize].handleFiles.file.o.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"FS_FileForHandle: NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    return fsh[f as usize].handleFiles.file.o;
}
#[no_mangle]

pub unsafe extern "C" fn FS_ForceFlush(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    let mut file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    file = FS_FileForHandle(f);
    crate::stdlib::setvbuf(
        file,
        0 as *mut libc::c_char,
        2 as i32,
        0 as i32 as crate::stddef_h::size_t,
    );
}
/*
================
FS_fplength
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_fplength(mut h: *mut crate::stdlib::FILE) -> libc::c_long {
    let mut pos: libc::c_long = 0;
    let mut end: libc::c_long = 0;
    pos = crate::stdlib::ftell(h);
    crate::stdlib::fseek(h, 0 as i32 as libc::c_long, 2 as i32);
    end = crate::stdlib::ftell(h);
    crate::stdlib::fseek(h, pos, 0 as i32);
    return end;
}
/*
================
FS_filelength

If this is called on a non-unique FILE (from a pak file),
it will return the size of the pak file, not the expected
size of the file.
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_filelength(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) -> libc::c_long {
    let mut h: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    h = FS_FileForHandle(f);
    if h.is_null() {
        return -(1 as i32) as libc::c_long;
    } else {
        return FS_fplength(h);
    };
}
/*
====================
FS_ReplaceSeparators

Fix things up differently for win/unix/mac
====================
*/

unsafe extern "C" fn FS_ReplaceSeparators(mut path: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastCharWasSep: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    s = path;
    while *s != 0 {
        if *s as i32 == '/' as i32 || *s as i32 == '\\' as i32 {
            if lastCharWasSep as u64 == 0 {
                *s = '/' as i32 as libc::c_char;
                lastCharWasSep = crate::src::qcommon::q_shared::qtrue
            } else {
                crate::stdlib::memmove(
                    s as *mut libc::c_void,
                    s.offset(1 as i32 as isize) as *const libc::c_void,
                    crate::stdlib::strlen(s),
                );
            }
        } else {
            lastCharWasSep = crate::src::qcommon::q_shared::qfalse
        }
        s = s.offset(1)
    }
}
/*
===================
FS_BuildOSPath

Qpath may have either forward or backwards slashes
===================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_BuildOSPath(
    mut base: *const libc::c_char,
    mut game: *const libc::c_char,
    mut qpath: *const libc::c_char,
) -> *mut libc::c_char {
    let mut temp: [libc::c_char; 4096] = [0; 4096]; // flip-flop to allow two returns without clash
    static mut ospath: [[libc::c_char; 4096]; 2] = [[0; 4096]; 2];
    static mut toggle: i32 = 0;
    toggle ^= 1 as i32;
    if game.is_null() || *game.offset(0 as i32 as isize) == 0 {
        game = fs_gamedir.as_mut_ptr()
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        temp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
        b"/%s/%s\x00" as *const u8 as *const libc::c_char,
        game,
        qpath,
    );
    FS_ReplaceSeparators(temp.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        ospath[toggle as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        base,
        temp.as_mut_ptr(),
    );
    return ospath[toggle as usize].as_mut_ptr();
}
/*
============
FS_CreatePath

Creates any directories needed to store the given filename
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_CreatePath(
    mut OSPath: *mut libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ofs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    // make absolutely sure that it can't back up the path
    // FIXME: is c: allowed???
    if !::libc::strstr(OSPath, b"..\x00" as *const u8 as *const libc::c_char).is_null()
        || !::libc::strstr(OSPath, b"::\x00" as *const u8 as *const libc::c_char).is_null()
    {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: refusing to create relative path \"%s\"\n\x00" as *const u8
                as *const libc::c_char,
            OSPath,
        );
        return crate::src::qcommon::q_shared::qtrue;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        path.as_mut_ptr(),
        OSPath,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    FS_ReplaceSeparators(path.as_mut_ptr());
    // Skip creation of the root directory as it will always be there
    ofs = ::libc::strchr(path.as_mut_ptr(), '/' as i32);
    if !ofs.is_null() {
        ofs = ofs.offset(1)
    }
    while !ofs.is_null() && *ofs as i32 != 0 {
        if *ofs as i32 == '/' as i32 {
            // create the directory
            *ofs = 0 as i32 as libc::c_char;
            if crate::src::sys::sys_unix::Sys_Mkdir(path.as_mut_ptr()) as u64 == 0 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as i32,
                    b"FS_CreatePath: failed to create path \"%s\"\x00" as *const u8
                        as *const libc::c_char,
                    path.as_mut_ptr(),
                );
            }
            *ofs = '/' as i32 as libc::c_char
        }
        ofs = ofs.offset(1)
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
FS_CheckFilenameIsMutable

ERR_FATAL if trying to maniuplate a file with the platform library, QVM, or pk3 extension
=================
 */

unsafe extern "C" fn FS_CheckFilenameIsMutable(
    mut filename: *const libc::c_char,
    mut function: *const libc::c_char,
) {
    // Check if the filename ends with the library, QVM, or pk3 extension
    if crate::src::sys::sys_unix::Sys_DllExtension(filename) as u32 != 0
        || crate::src::qcommon::q_shared::COM_CompareExtension(
            filename,
            b".qvm\x00" as *const u8 as *const libc::c_char,
        ) as u32
            != 0
        || crate::src::qcommon::q_shared::COM_CompareExtension(
            filename,
            b".pk3\x00" as *const u8 as *const libc::c_char,
        ) as u32
            != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"%s: Not allowed to manipulate \'%s\' due to %s extension\x00" as *const u8
                as *const libc::c_char,
            function,
            filename,
            crate::src::qcommon::q_shared::COM_GetExtension(filename),
        );
    };
}
/*
===========
FS_Remove

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Remove(mut osPath: *const libc::c_char) {
    FS_CheckFilenameIsMutable(
        osPath,
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"FS_Remove\x00")).as_ptr(),
    );
    ::libc::remove(osPath);
}
/*
===========
FS_HomeRemove

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_HomeRemove(mut homePath: *const libc::c_char) {
    FS_CheckFilenameIsMutable(
        homePath,
        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"FS_HomeRemove\x00")).as_ptr(),
    );
    ::libc::remove(FS_BuildOSPath(
        (*fs_homepath).string,
        fs_gamedir.as_mut_ptr(),
        homePath,
    ));
}
/*
================
FS_FileInPathExists

Tests if path and file exists
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FileInPathExists(
    mut testpath: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut filep: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    filep = crate::src::sys::sys_unix::Sys_FOpen(
        testpath,
        b"rb\x00" as *const u8 as *const libc::c_char,
    ) as *mut crate::stdlib::_IO_FILE;
    if !filep.is_null() {
        crate::stdlib::fclose(filep);
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
================
FS_FileExists

Tests if the file exists in the current gamedir, this DOES NOT
search the paths.  This is to determine if opening a file to write
(which always goes into the current gamedir) will cause any overwrites.
NOTE TTimo: this goes with FS_FOpenFileWrite for opening the file afterwards
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FileExists(
    mut file: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    return FS_FileInPathExists(FS_BuildOSPath(
        (*fs_homepath).string,
        fs_gamedir.as_mut_ptr(),
        file,
    ));
}
/*
================
FS_SV_FileExists

Tests if the file exists
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_SV_FileExists(
    mut file: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut testpath: *mut libc::c_char = 0 as *mut libc::c_char;
    testpath = FS_BuildOSPath(
        (*fs_homepath).string,
        file,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    *testpath
        .offset(crate::stdlib::strlen(testpath).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return FS_FileInPathExists(testpath);
}
/*
===========
FS_SV_FOpenFileWrite

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_SV_FOpenFileWrite(
    mut filename: *const libc::c_char,
) -> crate::src::qcommon::q_shared::fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    ospath = FS_BuildOSPath(
        (*fs_homepath).string,
        filename,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    *ospath
        .offset(crate::stdlib::strlen(ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_SV_FOpenFileWrite: %s\n\x00" as *const u8 as *const libc::c_char,
            ospath,
        );
    }
    FS_CheckFilenameIsMutable(
        ospath,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"FS_SV_FOpenFileWrite\x00"))
            .as_ptr(),
    );
    if FS_CreatePath(ospath) as u64 != 0 {
        return 0 as i32;
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"writing to: %s\n\x00" as *const u8 as *const libc::c_char,
        ospath,
    );
    fsh[f as usize].handleFiles.file.o =
        crate::src::sys::sys_unix::Sys_FOpen(ospath, b"wb\x00" as *const u8 as *const libc::c_char)
            as *mut crate::stdlib::_IO_FILE;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fsh[f as usize].name.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() {
        f = 0 as i32
    }
    return f;
}
/*
===========
FS_SV_FOpenFileRead

Search for a file somewhere below the home path then base path
in that order
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_SV_FOpenFileRead(
    mut filename: *const libc::c_char,
    mut fp: *mut crate::src::qcommon::q_shared::fileHandle_t,
) -> libc::c_long {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0 as i32;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fsh[f as usize].name.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    // don't let sound stutter
    crate::src::client::snd_main::S_ClearSoundBuffer();
    // search homepath
    ospath = FS_BuildOSPath(
        (*fs_homepath).string,
        filename,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    // remove trailing slash
    *ospath
        .offset(crate::stdlib::strlen(ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_SV_FOpenFileRead (fs_homepath): %s\n\x00" as *const u8 as *const libc::c_char,
            ospath,
        );
    }
    fsh[f as usize].handleFiles.file.o =
        crate::src::sys::sys_unix::Sys_FOpen(ospath, b"rb\x00" as *const u8 as *const libc::c_char)
            as *mut crate::stdlib::_IO_FILE;
    fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() {
        // If fs_homepath == fs_basepath, don't bother
        if crate::src::qcommon::q_shared::Q_stricmp((*fs_homepath).string, (*fs_basepath).string)
            != 0
        {
            // search basepath
            ospath = FS_BuildOSPath(
                (*fs_basepath).string,
                filename,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            *ospath.offset(
                crate::stdlib::strlen(ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
            if (*fs_debug).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"FS_SV_FOpenFileRead (fs_basepath): %s\n\x00" as *const u8
                        as *const libc::c_char,
                    ospath,
                );
            }
            fsh[f as usize].handleFiles.file.o = crate::src::sys::sys_unix::Sys_FOpen(
                ospath,
                b"rb\x00" as *const u8 as *const libc::c_char,
            ) as *mut crate::stdlib::_IO_FILE;
            fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse
        }
        // Check fs_steampath
        if fsh[f as usize].handleFiles.file.o.is_null()
            && *(*fs_steampath).string.offset(0 as i32 as isize) as i32 != 0
        {
            ospath = FS_BuildOSPath(
                (*fs_steampath).string,
                filename,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            *ospath.offset(
                crate::stdlib::strlen(ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
            if (*fs_debug).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"FS_SV_FOpenFileRead (fs_steampath): %s\n\x00" as *const u8
                        as *const libc::c_char,
                    ospath,
                );
            }
            fsh[f as usize].handleFiles.file.o = crate::src::sys::sys_unix::Sys_FOpen(
                ospath,
                b"rb\x00" as *const u8 as *const libc::c_char,
            ) as *mut crate::stdlib::_IO_FILE;
            fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse
        }
        // Check fs_gogpath
        if fsh[f as usize].handleFiles.file.o.is_null()
            && *(*fs_gogpath).string.offset(0 as i32 as isize) as i32 != 0
        {
            ospath = FS_BuildOSPath(
                (*fs_gogpath).string,
                filename,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            *ospath.offset(
                crate::stdlib::strlen(ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
            if (*fs_debug).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"FS_SV_FOpenFileRead (fs_gogpath): %s\n\x00" as *const u8
                        as *const libc::c_char,
                    ospath,
                );
            }
            fsh[f as usize].handleFiles.file.o = crate::src::sys::sys_unix::Sys_FOpen(
                ospath,
                b"rb\x00" as *const u8 as *const libc::c_char,
            ) as *mut crate::stdlib::_IO_FILE;
            fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse
        }
        if fsh[f as usize].handleFiles.file.o.is_null() {
            f = 0 as i32
        }
    }
    *fp = f;
    if f != 0 {
        return FS_filelength(f);
    }
    return -(1 as i32) as libc::c_long;
}
/*
===========
FS_SV_Rename

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_SV_Rename(
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut safe: crate::src::qcommon::q_shared::qboolean,
) {
    let mut from_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    // don't let sound stutter
    crate::src::client::snd_main::S_ClearSoundBuffer();
    from_ospath = FS_BuildOSPath(
        (*fs_homepath).string,
        from,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    to_ospath = FS_BuildOSPath(
        (*fs_homepath).string,
        to,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    *from_ospath.offset(
        crate::stdlib::strlen(from_ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize,
    ) = '\u{0}' as i32 as libc::c_char;
    *to_ospath.offset(
        crate::stdlib::strlen(to_ospath).wrapping_sub(1 as i32 as libc::c_ulong) as isize,
    ) = '\u{0}' as i32 as libc::c_char;
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_SV_Rename: %s --> %s\n\x00" as *const u8 as *const libc::c_char,
            from_ospath,
            to_ospath,
        );
    }
    if safe as u64 != 0 {
        FS_CheckFilenameIsMutable(
            to_ospath,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"FS_SV_Rename\x00"))
                .as_ptr(),
        );
    }
    ::libc::rename(from_ospath, to_ospath);
}
/*
===========
FS_Rename

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Rename(mut from: *const libc::c_char, mut to: *const libc::c_char) {
    let mut from_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    // don't let sound stutter
    crate::src::client::snd_main::S_ClearSoundBuffer();
    from_ospath = FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), from);
    to_ospath = FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), to);
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_Rename: %s --> %s\n\x00" as *const u8 as *const libc::c_char,
            from_ospath,
            to_ospath,
        );
    }
    FS_CheckFilenameIsMutable(
        to_ospath,
        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"FS_Rename\x00")).as_ptr(),
    );
    ::libc::rename(from_ospath, to_ospath);
}
/*
==============
FS_FCloseFile

If the FILE pointer is an open pak file, leave it open.

For some reason, other dll's can't just cal fclose()
on files returned by FS_FOpenFile...
==============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FCloseFile(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if fsh[f as usize].zipFile as u32 == crate::src::qcommon::q_shared::qtrue as i32 as u32 {
        crate::src::qcommon::unzip::unzCloseCurrentFile(fsh[f as usize].handleFiles.file.z);
        if fsh[f as usize].handleFiles.unique as u64 != 0 {
            crate::src::qcommon::unzip::unzClose(fsh[f as usize].handleFiles.file.z);
        }
        crate::stdlib::memset(
            &mut *fsh.as_mut_ptr().offset(f as isize) as *mut fileHandleData_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<fileHandleData_t>() as libc::c_ulong,
        );
        return;
    }
    // we didn't find it as a pak, so close it as a unique file
    if !fsh[f as usize].handleFiles.file.o.is_null() {
        crate::stdlib::fclose(fsh[f as usize].handleFiles.file.o);
    }
    crate::stdlib::memset(
        &mut *fsh.as_mut_ptr().offset(f as isize) as *mut fileHandleData_t as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<fileHandleData_t>() as libc::c_ulong,
    );
}
/*
===========
FS_FOpenFileWrite

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FOpenFileWrite(
    mut filename: *const libc::c_char,
) -> crate::src::qcommon::q_shared::fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
    ospath = FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), filename);
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_FOpenFileWrite: %s\n\x00" as *const u8 as *const libc::c_char,
            ospath,
        );
    }
    FS_CheckFilenameIsMutable(
        ospath,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"FS_FOpenFileWrite\x00"))
            .as_ptr(),
    );
    if FS_CreatePath(ospath) as u64 != 0 {
        return 0 as i32;
    }
    // enabling the following line causes a recursive function call loop
    // when running with +set logfile 1 +set developer 1
    //Com_DPrintf( "writing to: %s\n", ospath );
    fsh[f as usize].handleFiles.file.o =
        crate::src::sys::sys_unix::Sys_FOpen(ospath, b"wb\x00" as *const u8 as *const libc::c_char)
            as *mut crate::stdlib::_IO_FILE;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fsh[f as usize].name.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() {
        f = 0 as i32
    }
    return f;
}
/*
===========
FS_FOpenFileAppend

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FOpenFileAppend(
    mut filename: *const libc::c_char,
) -> crate::src::qcommon::q_shared::fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fsh[f as usize].name.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    // don't let sound stutter
    crate::src::client::snd_main::S_ClearSoundBuffer();
    ospath = FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), filename);
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_FOpenFileAppend: %s\n\x00" as *const u8 as *const libc::c_char,
            ospath,
        );
    }
    FS_CheckFilenameIsMutable(
        ospath,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"FS_FOpenFileAppend\x00"))
            .as_ptr(),
    );
    if FS_CreatePath(ospath) as u64 != 0 {
        return 0 as i32;
    }
    fsh[f as usize].handleFiles.file.o =
        crate::src::sys::sys_unix::Sys_FOpen(ospath, b"ab\x00" as *const u8 as *const libc::c_char)
            as *mut crate::stdlib::_IO_FILE;
    fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() {
        f = 0 as i32
    }
    return f;
}
/*
===========
FS_FCreateOpenPipeFile

===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FCreateOpenPipeFile(
    mut filename: *const libc::c_char,
) -> crate::src::qcommon::q_shared::fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fifo: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::q_shared::Q_strncpyz(
        fsh[f as usize].name.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
    );
    // don't let sound stutter
    crate::src::client::snd_main::S_ClearSoundBuffer();
    ospath = FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), filename);
    if (*fs_debug).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"FS_FCreateOpenPipeFile: %s\n\x00" as *const u8 as *const libc::c_char,
            ospath,
        );
    }
    FS_CheckFilenameIsMutable(
        ospath,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"FS_FCreateOpenPipeFile\x00"))
            .as_ptr(),
    );
    fifo = crate::src::sys::sys_unix::Sys_Mkfifo(ospath) as *mut crate::stdlib::_IO_FILE;
    if !fifo.is_null() {
        fsh[f as usize].handleFiles.file.o = fifo;
        fsh[f as usize].handleSync = crate::src::qcommon::q_shared::qfalse
    } else {
        crate::src::qcommon::common::Com_Printf(b"^3WARNING: Could not create new com_pipefile at %s. com_pipefile will not be used.\n\x00"
                       as *const u8 as *const libc::c_char, ospath);
        f = 0 as i32
    }
    return f;
}
/*
===========
FS_FilenameCompare

Ignore case and seprator char distinctions
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FilenameCompare(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    loop {
        let fresh0 = s1;
        s1 = s1.offset(1);
        c1 = *fresh0 as i32;
        let fresh1 = s2;
        s2 = s2.offset(1);
        c2 = *fresh1 as i32;
        if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
            c1 -= 'a' as i32 - 'A' as i32
        }
        if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
            c2 -= 'a' as i32 - 'A' as i32
        }
        if c1 == '\\' as i32 || c1 == ':' as i32 {
            c1 = '/' as i32
        }
        if c2 == '\\' as i32 || c2 == ':' as i32 {
            c2 = '/' as i32
        }
        if c1 != c2 {
            return crate::src::qcommon::q_shared::qtrue;
            // strings not equal
        }
        if !(c1 != 0) {
            break;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
    // strings are equal
}
/*
===========
FS_IsExt

Return qtrue if ext matches file extension filename
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_IsExt(
    mut filename: *const libc::c_char,
    mut ext: *const libc::c_char,
    mut namelen: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut extlen: i32 = 0;
    extlen = crate::stdlib::strlen(ext) as i32;
    if extlen > namelen {
        return crate::src::qcommon::q_shared::qfalse;
    }
    filename = filename.offset((namelen - extlen) as isize);
    return (crate::src::qcommon::q_shared::Q_stricmp(filename, ext) == 0) as i32
        as crate::src::qcommon::q_shared::qboolean;
}
/*
===========
FS_IsDemoExt

Return qtrue if filename has a demo extension
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_IsDemoExt(
    mut filename: *const libc::c_char,
    mut _namelen: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ext_test: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: i32 = 0;
    let mut protocol: i32 = 0;
    ext_test = ::libc::strrchr(filename, '.' as i32);
    if !ext_test.is_null()
        && crate::src::qcommon::q_shared::Q_stricmpn(
            ext_test.offset(1 as i32 as isize),
            b"dm_\x00" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as i32 as libc::c_ulong) as i32,
        ) == 0
    {
        protocol = atoi(
            ext_test.offset(
                (::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    as isize,
            ),
        );
        if protocol == (*crate::src::qcommon::common::com_protocol).integer {
            return crate::src::qcommon::q_shared::qtrue;
        }
        if protocol == (*crate::src::qcommon::common::com_legacyprotocol).integer {
            return crate::src::qcommon::q_shared::qtrue;
        }
        index = 0 as i32;
        while *crate::src::qcommon::common::demo_protocols
            .as_mut_ptr()
            .offset(index as isize)
            != 0
        {
            if *crate::src::qcommon::common::demo_protocols
                .as_mut_ptr()
                .offset(index as isize)
                == protocol
            {
                return crate::src::qcommon::q_shared::qtrue;
            }
            index += 1
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===========
FS_FOpenFileReadDir

Tries opening file "filename" in searchpath "search"
Returns filesize and an open FILE pointer.
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FOpenFileReadDir(
    mut filename: *const libc::c_char,
    mut search: *mut searchpath_t,
    mut file: *mut crate::src::qcommon::q_shared::fileHandle_t,
    mut uniqueFILE: crate::src::qcommon::q_shared::qboolean,
    mut unpure: crate::src::qcommon::q_shared::qboolean,
) -> libc::c_long {
    let mut hash: libc::c_long = 0;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut pakFile: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut dir: *mut directory_t = 0 as *mut directory_t;
    let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filep: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut len: i32 = 0;
    if filename.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FS_FOpenFileRead: NULL \'filename\' parameter passed\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // qpaths are not supposed to have a leading slash
    if *filename.offset(0 as i32 as isize) as i32 == '/' as i32
        || *filename.offset(0 as i32 as isize) as i32 == '\\' as i32
    {
        filename = filename.offset(1)
    }
    // make absolutely sure that it can't back up the path.
    // The searchpaths do guarantee that something will always
    // be prepended, so we don't need to worry about "c:" or "//limbo"
    if !::libc::strstr(filename, b"..\x00" as *const u8 as *const libc::c_char).is_null()
        || !::libc::strstr(filename, b"::\x00" as *const u8 as *const libc::c_char).is_null()
    {
        if file.is_null() {
            return crate::src::qcommon::q_shared::qfalse as i32 as libc::c_long;
        }
        *file = 0 as i32;
        return -(1 as i32) as libc::c_long;
    }
    // make sure the q3key file is only readable by the quake3.exe at initialization
    // any other time the key should only be accessed in memory using the provided functions
    if crate::src::qcommon::common::com_fullyInitialized as u32 != 0
        && !::libc::strstr(filename, b"q3key\x00" as *const u8 as *const libc::c_char).is_null()
    {
        if file.is_null() {
            return crate::src::qcommon::q_shared::qfalse as i32 as libc::c_long;
        }
        *file = 0 as i32;
        return -(1 as i32) as libc::c_long;
    }
    if file.is_null() {
        // just wants to see if file is there
        // is the element a pak file?
        if !(*search).pack.is_null() {
            hash = FS_HashFileName(filename, (*(*search).pack).hashSize);
            if !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null() {
                // look through all the pak file elements
                pak = (*search).pack;
                pakFile = *(*pak).hashTable.offset(hash as isize);
                loop {
                    // case and separator insensitive comparisons
                    if FS_FilenameCompare((*pakFile).name, filename) as u64 == 0 {
                        // found it!
                        if (*pakFile).len != 0 {
                            return (*pakFile).len as libc::c_long;
                        } else {
                            // It's not nice, but legacy code depends
                            // on positive value if file exists no matter
                            // what size
                            return 1 as i32 as libc::c_long;
                        }
                    }
                    pakFile = (*pakFile).next;
                    if pakFile.is_null() {
                        break;
                    }
                }
            }
        } else if !(*search).dir.is_null() {
            dir = (*search).dir;
            netpath = FS_BuildOSPath(
                (*dir).path.as_mut_ptr(),
                (*dir).gamedir.as_mut_ptr(),
                filename,
            );
            filep = crate::src::sys::sys_unix::Sys_FOpen(
                netpath,
                b"rb\x00" as *const u8 as *const libc::c_char,
            ) as *mut crate::stdlib::_IO_FILE;
            if !filep.is_null() {
                len = FS_fplength(filep) as i32;
                crate::stdlib::fclose(filep);
                if len != 0 {
                    return len as libc::c_long;
                } else {
                    return 1 as i32 as libc::c_long;
                }
            }
        }
        return 0 as i32 as libc::c_long;
    }
    *file = FS_HandleForFile();
    fsh[*file as usize].handleFiles.unique = uniqueFILE;
    // is the element a pak file?
    if !(*search).pack.is_null() {
        hash = FS_HashFileName(filename, (*(*search).pack).hashSize);
        if !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null() {
            // disregard if it doesn't match one of the allowed pure pak files
            if unpure as u64 == 0 && FS_PakIsPure((*search).pack) as u64 == 0 {
                *file = 0 as i32;
                return -(1 as i32) as libc::c_long;
            }
            // look through all the pak file elements
            pak = (*search).pack;
            pakFile = *(*pak).hashTable.offset(hash as isize);
            loop {
                // case and separator insensitive comparisons
                if FS_FilenameCompare((*pakFile).name, filename) as u64 == 0 {
                    // found it!
                    // mark the pak as having been referenced and mark specifics on cgame and ui
                    // shaders, txt, arena files  by themselves do not count as a reference as
                    // these are loaded from all pk3s
                    // from every pk3 file..
                    len = crate::stdlib::strlen(filename) as i32;
                    if (*pak).referenced & 0x1 as i32 == 0 {
                        if FS_IsExt(
                            filename,
                            b".shader\x00" as *const u8 as *const libc::c_char,
                            len,
                        ) as u64
                            == 0
                            && FS_IsExt(
                                filename,
                                b".txt\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && FS_IsExt(
                                filename,
                                b".cfg\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && FS_IsExt(
                                filename,
                                b".config\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && FS_IsExt(
                                filename,
                                b".bot\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && FS_IsExt(
                                filename,
                                b".arena\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && FS_IsExt(
                                filename,
                                b".menu\x00" as *const u8 as *const libc::c_char,
                                len,
                            ) as u64
                                == 0
                            && crate::src::qcommon::q_shared::Q_stricmp(
                                filename,
                                b"vm/qagame.qvm\x00" as *const u8 as *const libc::c_char,
                            ) != 0 as i32
                            && ::libc::strstr(
                                filename,
                                b"levelshots\x00" as *const u8 as *const libc::c_char,
                            )
                            .is_null()
                        {
                            (*pak).referenced |= 0x1 as i32
                        }
                    }
                    if !::libc::strstr(
                        filename,
                        b"cgame.qvm\x00" as *const u8 as *const libc::c_char,
                    )
                    .is_null()
                    {
                        (*pak).referenced |= 0x4 as i32
                    }
                    if !::libc::strstr(filename, b"ui.qvm\x00" as *const u8 as *const libc::c_char)
                        .is_null()
                    {
                        (*pak).referenced |= 0x2 as i32
                    }
                    if uniqueFILE as u64 != 0 {
                        // open a new file on the pakfile
                        fsh[*file as usize].handleFiles.file.z =
                            crate::src::qcommon::unzip::unzOpen((*pak).pakFilename.as_mut_ptr());
                        if fsh[*file as usize].handleFiles.file.z.is_null() {
                            crate::src::qcommon::common::Com_Error(
                                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                                b"Couldn\'t open %s\x00" as *const u8 as *const libc::c_char,
                                (*pak).pakFilename.as_mut_ptr(),
                            );
                        }
                    } else {
                        fsh[*file as usize].handleFiles.file.z = (*pak).handle
                    }
                    crate::src::qcommon::q_shared::Q_strncpyz(
                        fsh[*file as usize].name.as_mut_ptr(),
                        filename,
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
                    );
                    fsh[*file as usize].zipFile = crate::src::qcommon::q_shared::qtrue;
                    // set the file position in the zip file (also sets the current file info)
                    crate::src::qcommon::unzip::unzSetOffset(
                        fsh[*file as usize].handleFiles.file.z,
                        (*pakFile).pos,
                    );
                    // open the file in the zip
                    crate::src::qcommon::unzip::unzOpenCurrentFile(
                        fsh[*file as usize].handleFiles.file.z,
                    );
                    fsh[*file as usize].zipFilePos = (*pakFile).pos as i32;
                    fsh[*file as usize].zipFileLen = (*pakFile).len as i32;
                    if (*fs_debug).integer != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"FS_FOpenFileRead: %s (found in \'%s\')\n\x00" as *const u8
                                as *const libc::c_char,
                            filename,
                            (*pak).pakFilename.as_mut_ptr(),
                        );
                    }
                    return (*pakFile).len as libc::c_long;
                }
                pakFile = (*pakFile).next;
                if pakFile.is_null() {
                    break;
                }
            }
        }
    } else if !(*search).dir.is_null() {
        // check a file in the directory tree
        // if we are running restricted, the only files we
        // will allow to come from the directory are .cfg files
        len = crate::stdlib::strlen(filename) as i32;
        // FIXME TTimo I'm not sure about the fs_numServerPaks test
        // if you are using FS_ReadFile to find out if a file exists,
        //   this test can make the search fail although the file is in the directory
        // I had the problem on https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=8
        // turned out I used FS_FileExists instead
        if unpure as u64 == 0 && fs_numServerPaks != 0 {
            if FS_IsExt(
                filename,
                b".cfg\x00" as *const u8 as *const libc::c_char,
                len,
            ) as u64
                == 0
                && FS_IsExt(
                    filename,
                    b".menu\x00" as *const u8 as *const libc::c_char,
                    len,
                ) as u64
                    == 0
                && FS_IsExt(
                    filename,
                    b".game\x00" as *const u8 as *const libc::c_char,
                    len,
                ) as u64
                    == 0
                && FS_IsExt(
                    filename,
                    b".dat\x00" as *const u8 as *const libc::c_char,
                    len,
                ) as u64
                    == 0
                && FS_IsDemoExt(filename, len) as u64 == 0
            {
                // demos
                *file = 0 as i32;
                return -(1 as i32) as libc::c_long;
            }
        }
        dir = (*search).dir;
        netpath = FS_BuildOSPath(
            (*dir).path.as_mut_ptr(),
            (*dir).gamedir.as_mut_ptr(),
            filename,
        );
        filep = crate::src::sys::sys_unix::Sys_FOpen(
            netpath,
            b"rb\x00" as *const u8 as *const libc::c_char,
        ) as *mut crate::stdlib::_IO_FILE;
        if filep.is_null() {
            *file = 0 as i32;
            return -(1 as i32) as libc::c_long;
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            fsh[*file as usize].name.as_mut_ptr(),
            filename,
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as i32,
        );
        fsh[*file as usize].zipFile = crate::src::qcommon::q_shared::qfalse;
        if (*fs_debug).integer != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"FS_FOpenFileRead: %s (found in \'%s%c%s\')\n\x00" as *const u8
                    as *const libc::c_char,
                filename,
                (*dir).path.as_mut_ptr(),
                '/' as i32,
                (*dir).gamedir.as_mut_ptr(),
            );
        }
        fsh[*file as usize].handleFiles.file.o = filep;
        return FS_fplength(filep);
    }
    *file = 0 as i32;
    return -(1 as i32) as libc::c_long;
}
/*
===========
FS_FOpenFileRead

Finds the file in the search path.
Returns filesize and an open FILE pointer.
Used for streaming data out of either a
separate file or a ZIP file.
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FOpenFileRead(
    mut filename: *const libc::c_char,
    mut file: *mut crate::src::qcommon::q_shared::fileHandle_t,
    mut uniqueFILE: crate::src::qcommon::q_shared::qboolean,
) -> libc::c_long {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut len: libc::c_long = 0;
    let mut isLocalConfig: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    isLocalConfig = (::libc::strcmp(
        filename,
        b"autoexec.cfg\x00" as *const u8 as *const libc::c_char,
    ) == 0
        || ::libc::strcmp(
            filename,
            b"q3config.cfg\x00" as *const u8 as *const libc::c_char,
        ) == 0) as i32 as crate::src::qcommon::q_shared::qboolean;
    search = fs_searchpaths;
    while !search.is_null() {
        // autoexec.cfg and q3config.cfg can only be loaded outside of pk3 files.
        if !(isLocalConfig as u32 != 0 && !(*search).pack.is_null()) {
            len = FS_FOpenFileReadDir(
                filename,
                search,
                file,
                uniqueFILE,
                crate::src::qcommon::q_shared::qfalse,
            );
            if file.is_null() {
                if len > 0 as i32 as libc::c_long {
                    return len;
                }
            } else if len >= 0 as i32 as libc::c_long && *file != 0 {
                return len;
            }
        }
        search = (*search).next
    }
    if !file.is_null() {
        *file = 0 as i32;
        return -(1 as i32) as libc::c_long;
    } else {
        // When file is NULL, we're querying the existence of the file
        // If we've got here, it doesn't exist
        return 0 as i32 as libc::c_long;
    };
}
/*
=================
FS_FindVM

Find a suitable VM file in search path order.

In each searchpath try:
 - open DLL file if DLL loading enabled
 - open QVM file

Enable search for DLL by setting enableDll to FSVM_ENABLEDLL

write found DLL or QVM to "found" and return VMI_NATIVE if DLL, VMI_COMPILED if QVM
Return the searchpath in "startSearch".
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FindVM(
    mut startSearch: *mut *mut libc::c_void,
    mut found: *mut libc::c_char,
    mut foundlen: i32,
    mut name: *const libc::c_char,
    mut enableDll: i32,
) -> i32 {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut lastSearch: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut dir: *mut directory_t = 0 as *mut directory_t;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut dllName: [libc::c_char; 4096] = [0; 4096];
    let mut qvmName: [libc::c_char; 4096] = [0; 4096];
    let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if enableDll != 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            dllName.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
            b"%sx86_64.so\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        qvmName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
        b"vm/%s.qvm\x00" as *const u8 as *const libc::c_char,
        name,
    );
    lastSearch = *startSearch as *mut searchpath_t;
    if (*startSearch).is_null() {
        search = fs_searchpaths
    } else {
        search = (*lastSearch).next
    }
    while !search.is_null() {
        if !(*search).dir.is_null() && fs_numServerPaks == 0 {
            dir = (*search).dir;
            if enableDll != 0 {
                netpath = FS_BuildOSPath(
                    (*dir).path.as_mut_ptr(),
                    (*dir).gamedir.as_mut_ptr(),
                    dllName.as_mut_ptr(),
                );
                if FS_FileInPathExists(netpath) as u64 != 0 {
                    crate::src::qcommon::q_shared::Q_strncpyz(found, netpath, foundlen);
                    *startSearch = search as *mut libc::c_void;
                    return crate::qcommon_h::VMI_NATIVE as i32;
                }
            }
            if FS_FOpenFileReadDir(
                qvmName.as_mut_ptr(),
                search,
                0 as *mut crate::src::qcommon::q_shared::fileHandle_t,
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qfalse,
            ) > 0 as i32 as libc::c_long
            {
                *startSearch = search as *mut libc::c_void;
                return crate::qcommon_h::VMI_COMPILED as i32;
            }
        } else if !(*search).pack.is_null() {
            pack = (*search).pack;
            if !lastSearch.is_null() && !(*lastSearch).pack.is_null() {
                // make sure we only try loading one VM file per game dir
                // i.e. if VM from pak7.pk3 fails we won't try one from pak6.pk3
                if FS_FilenameCompare(
                    (*(*lastSearch).pack).pakPathname.as_mut_ptr(),
                    (*pack).pakPathname.as_mut_ptr(),
                ) as u64
                    == 0
                {
                    search = (*search).next;
                    continue;
                }
            }
            if FS_FOpenFileReadDir(
                qvmName.as_mut_ptr(),
                search,
                0 as *mut crate::src::qcommon::q_shared::fileHandle_t,
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qfalse,
            ) > 0 as i32 as libc::c_long
            {
                *startSearch = search as *mut libc::c_void;
                return crate::qcommon_h::VMI_COMPILED as i32;
            }
        }
        search = (*search).next
    }
    return -(1 as i32);
}
/*
=================
FS_Read

Properly handles partial reads
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Read(
    mut buffer: *mut libc::c_void,
    mut len: i32,
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
) -> i32 {
    let mut block: i32 = 0;
    let mut remaining: i32 = 0;
    let mut read: i32 = 0;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut tries: i32 = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if f == 0 {
        return 0 as i32;
    }
    buf = buffer as *mut crate::src::qcommon::q_shared::byte;
    fs_readCount += len;
    if fsh[f as usize].zipFile as u32 == crate::src::qcommon::q_shared::qfalse as i32 as u32 {
        remaining = len;
        tries = 0 as i32;
        while remaining != 0 {
            block = remaining;
            read = crate::stdlib::fread(
                buf as *mut libc::c_void,
                1 as i32 as libc::c_ulong,
                block as libc::c_ulong,
                fsh[f as usize].handleFiles.file.o,
            ) as i32;
            if read == 0 as i32 {
                // we might have been trying to read from a CD, which
                // sometimes returns a 0 read on windows
                if tries == 0 {
                    tries = 1 as i32
                } else {
                    return len - remaining;
                    //Com_Error (ERR_FATAL, "FS_Read: 0 bytes read");
                }
            }
            if read == -(1 as i32) {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as i32,
                    b"FS_Read: -1 bytes read\x00" as *const u8 as *const libc::c_char,
                );
            }
            remaining -= read;
            buf = buf.offset(read as isize)
        }
        return len;
    } else {
        return crate::src::qcommon::unzip::unzReadCurrentFile(
            fsh[f as usize].handleFiles.file.z,
            buffer,
            len as u32,
        );
    };
}
/*
=================
FS_Write

Properly handles partial writes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Write(
    mut buffer: *const libc::c_void,
    mut len: i32,
    mut h: crate::src::qcommon::q_shared::fileHandle_t,
) -> i32 {
    let mut block: i32 = 0;
    let mut remaining: i32 = 0;
    let mut written: i32 = 0;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut tries: i32 = 0;
    let mut f: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if h == 0 {
        return 0 as i32;
    }
    f = FS_FileForHandle(h);
    buf = buffer as *mut crate::src::qcommon::q_shared::byte;
    remaining = len;
    tries = 0 as i32;
    while remaining != 0 {
        block = remaining;
        written = crate::stdlib::fwrite(
            buf as *const libc::c_void,
            1 as i32 as libc::c_ulong,
            block as libc::c_ulong,
            f,
        ) as i32;
        if written == 0 as i32 {
            if tries == 0 {
                tries = 1 as i32
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"FS_Write: 0 bytes written\n\x00" as *const u8 as *const libc::c_char,
                );
                return 0 as i32;
            }
        }
        if written == -(1 as i32) {
            crate::src::qcommon::common::Com_Printf(
                b"FS_Write: -1 bytes written\n\x00" as *const u8 as *const libc::c_char,
            );
            return 0 as i32;
        }
        remaining -= written;
        buf = buf.offset(written as isize)
    }
    if fsh[h as usize].handleSync as u64 != 0 {
        crate::stdlib::fflush(f);
    }
    return len;
}
#[no_mangle]

pub unsafe extern "C" fn FS_Printf(
    mut h: crate::src::qcommon::q_shared::fileHandle_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        fmt,
        argptr.as_va_list(),
    );
    FS_Write(
        msg.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(msg.as_mut_ptr()) as i32,
        h,
    );
}
/*
=================
FS_Seek

=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Seek(
    mut f: crate::src::qcommon::q_shared::fileHandle_t,
    mut offset: libc::c_long,
    mut origin: i32,
) -> i32 {
    let mut _origin: i32 = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if fsh[f as usize].zipFile as u32 == crate::src::qcommon::q_shared::qtrue as i32 as u32 {
        //FIXME: this is really, really crappy
        //(but better than what was here before)
        let mut buffer: [crate::src::qcommon::q_shared::byte; 65536] = [0; 65536];
        let mut remainder: i32 = 0;
        let mut currentPosition: i32 = FS_FTell(f);
        // change negative offsets into FS_SEEK_SET
        if offset < 0 as i32 as libc::c_long {
            match origin {
                1 => remainder = (fsh[f as usize].zipFileLen as libc::c_long + offset) as i32,
                0 => remainder = (currentPosition as libc::c_long + offset) as i32,
                2 | _ => remainder = 0 as i32,
            }
            if remainder < 0 as i32 {
                remainder = 0 as i32
            }
            origin = crate::src::qcommon::q_shared::FS_SEEK_SET as i32
        } else if origin == crate::src::qcommon::q_shared::FS_SEEK_END as i32 {
            remainder =
                ((fsh[f as usize].zipFileLen - currentPosition) as libc::c_long + offset) as i32
        } else {
            remainder = offset as i32
        }
        match origin {
            2 => {
                if remainder == currentPosition {
                    return offset as i32;
                }
                crate::src::qcommon::unzip::unzSetOffset(
                    fsh[f as usize].handleFiles.file.z,
                    fsh[f as usize].zipFilePos as crate::zconf_h::uLong,
                );
                crate::src::qcommon::unzip::unzOpenCurrentFile(fsh[f as usize].handleFiles.file.z);
            }
            1 | 0 => {}
            _ => {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as i32,
                    b"Bad origin in FS_Seek\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        //fallthrough
        while remainder > 65536 as i32 {
            FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, 65536 as i32, f);
            remainder -= 65536 as i32
        }
        FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, remainder, f);
        return offset as i32;
    } else {
        let mut file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
        file = FS_FileForHandle(f);
        match origin {
            0 => _origin = 1 as i32,
            1 => _origin = 2 as i32,
            2 => _origin = 0 as i32,
            _ => {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as i32,
                    b"Bad origin in FS_Seek\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        return crate::stdlib::fseek(file, offset, _origin);
    };
}
/*
======================================================================================

CONVENIENCE FUNCTIONS FOR ENTIRE FILES

======================================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FileIsInPAK(
    mut filename: *const libc::c_char,
    mut pChecksum: *mut i32,
) -> i32 {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut pakFile: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut hash: libc::c_long = 0 as i32 as libc::c_long;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if filename.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FS_FOpenFileRead: NULL \'filename\' parameter passed\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // qpaths are not supposed to have a leading slash
    if *filename.offset(0 as i32 as isize) as i32 == '/' as i32
        || *filename.offset(0 as i32 as isize) as i32 == '\\' as i32
    {
        filename = filename.offset(1)
    }
    // make absolutely sure that it can't back up the path.
    // The searchpaths do guarantee that something will always
    // be prepended, so we don't need to worry about "c:" or "//limbo"
    if !::libc::strstr(filename, b"..\x00" as *const u8 as *const libc::c_char).is_null()
        || !::libc::strstr(filename, b"::\x00" as *const u8 as *const libc::c_char).is_null()
    {
        return -(1 as i32);
    }
    //
    // search through the path, one element at a time
    //
    search = fs_searchpaths;
    while !search.is_null() {
        //
        if !(*search).pack.is_null() {
            hash = FS_HashFileName(filename, (*(*search).pack).hashSize)
        }
        // is the element a pak file?
        if !(*search).pack.is_null()
            && !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null()
        {
            // disregard if it doesn't match one of the allowed pure pak files
            if !(FS_PakIsPure((*search).pack) as u64 == 0) {
                // look through all the pak file elements
                pak = (*search).pack;
                pakFile = *(*pak).hashTable.offset(hash as isize);
                loop {
                    // case and separator insensitive comparisons
                    if FS_FilenameCompare((*pakFile).name, filename) as u64 == 0 {
                        if !pChecksum.is_null() {
                            *pChecksum = (*pak).pure_checksum
                        }
                        return 1 as i32;
                    }
                    pakFile = (*pakFile).next;
                    if pakFile.is_null() {
                        break;
                    }
                }
            }
        }
        search = (*search).next
    }
    return -(1 as i32);
}
/*
============
FS_ReadFileDir

Filename are relative to the quake search path
a null buffer will just return the file length without loading
If searchPath is non-NULL search only in that specific search path
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ReadFileDir(
    mut qpath: *const libc::c_char,
    mut searchPath: *mut libc::c_void,
    mut unpure: crate::src::qcommon::q_shared::qboolean,
    mut buffer: *mut *mut libc::c_void,
) -> libc::c_long {
    let mut h: crate::src::qcommon::q_shared::fileHandle_t = 0; // quiet compiler warning
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut isConfig: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut len: libc::c_long = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if qpath.is_null() || *qpath.offset(0 as i32 as isize) == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FS_ReadFile with empty name\x00" as *const u8 as *const libc::c_char,
        );
    }
    buf = 0 as *mut crate::src::qcommon::q_shared::byte;
    // if this is a .cfg file and we are playing back a journal, read
    // it from the journal file
    if !::libc::strstr(qpath, b".cfg\x00" as *const u8 as *const libc::c_char).is_null() {
        isConfig = crate::src::qcommon::q_shared::qtrue;
        if !crate::src::qcommon::common::com_journal.is_null()
            && (*crate::src::qcommon::common::com_journal).integer == 2 as i32
        {
            let mut r: i32 = 0;
            crate::src::qcommon::common::Com_DPrintf(
                b"Loading %s from journal file.\n\x00" as *const u8 as *const libc::c_char,
                qpath,
            );
            r = FS_Read(
                &mut len as *mut libc::c_long as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as i32,
                crate::src::qcommon::common::com_journalDataFile,
            );
            if r as libc::c_ulong != ::std::mem::size_of::<libc::c_long>() as libc::c_ulong {
                if !buffer.is_null() {
                    *buffer = 0 as *mut libc::c_void
                }
                return -(1 as i32) as libc::c_long;
            }
            // if the file didn't exist when the journal was created
            if len == 0 {
                if buffer.is_null() {
                    return 1 as i32 as libc::c_long;
                    // hack for old journal files
                }
                *buffer = 0 as *mut libc::c_void;
                return -(1 as i32) as libc::c_long;
            }
            if buffer.is_null() {
                return len;
            }
            buf = crate::src::qcommon::common::Hunk_AllocateTempMemory(
                (len + 1 as i32 as libc::c_long) as i32,
            ) as *mut crate::src::qcommon::q_shared::byte;
            *buffer = buf as *mut libc::c_void;
            r = FS_Read(
                buf as *mut libc::c_void,
                len as i32,
                crate::src::qcommon::common::com_journalDataFile,
            );
            if r as libc::c_long != len {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_FATAL as i32,
                    b"Read from journalDataFile failed\x00" as *const u8 as *const libc::c_char,
                );
            }
            fs_loadCount += 1;
            fs_loadStack += 1;
            // guarantee that it will have a trailing 0 for string operations
            *buf.offset(len as isize) = 0 as i32 as crate::src::qcommon::q_shared::byte;
            return len;
        }
    } else {
        isConfig = crate::src::qcommon::q_shared::qfalse
    }
    search = searchPath as *mut searchpath_t;
    if search.is_null() {
        // look for it in the filesystem or pack files
        len = FS_FOpenFileRead(qpath, &mut h, crate::src::qcommon::q_shared::qfalse)
    } else {
        // look for it in a specific search path only
        len = FS_FOpenFileReadDir(
            qpath,
            search,
            &mut h,
            crate::src::qcommon::q_shared::qfalse,
            unpure,
        )
    }
    if h == 0 as i32 {
        if !buffer.is_null() {
            *buffer = 0 as *mut libc::c_void
        }
        // if we are journalling and it is a config file, write a zero to the journal file
        if isConfig as u32 != 0
            && !crate::src::qcommon::common::com_journal.is_null()
            && (*crate::src::qcommon::common::com_journal).integer == 1 as i32
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"Writing zero for %s to journal file.\n\x00" as *const u8 as *const libc::c_char,
                qpath,
            );
            len = 0 as i32 as libc::c_long;
            FS_Write(
                &mut len as *mut libc::c_long as *const libc::c_void,
                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as i32,
                crate::src::qcommon::common::com_journalDataFile,
            );
            FS_Flush(crate::src::qcommon::common::com_journalDataFile);
        }
        return -(1 as i32) as libc::c_long;
    }
    if buffer.is_null() {
        if isConfig as u32 != 0
            && !crate::src::qcommon::common::com_journal.is_null()
            && (*crate::src::qcommon::common::com_journal).integer == 1 as i32
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"Writing len for %s to journal file.\n\x00" as *const u8 as *const libc::c_char,
                qpath,
            );
            FS_Write(
                &mut len as *mut libc::c_long as *const libc::c_void,
                ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as i32,
                crate::src::qcommon::common::com_journalDataFile,
            );
            FS_Flush(crate::src::qcommon::common::com_journalDataFile);
        }
        FS_FCloseFile(h);
        return len;
    }
    fs_loadCount += 1;
    fs_loadStack += 1;
    buf = crate::src::qcommon::common::Hunk_AllocateTempMemory(
        (len + 1 as i32 as libc::c_long) as i32,
    ) as *mut crate::src::qcommon::q_shared::byte;
    *buffer = buf as *mut libc::c_void;
    FS_Read(buf as *mut libc::c_void, len as i32, h);
    // guarantee that it will have a trailing 0 for string operations
    *buf.offset(len as isize) = 0 as i32 as crate::src::qcommon::q_shared::byte;
    FS_FCloseFile(h);
    // if we are journalling and it is a config file, write it to the journal file
    if isConfig as u32 != 0
        && !crate::src::qcommon::common::com_journal.is_null()
        && (*crate::src::qcommon::common::com_journal).integer == 1 as i32
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"Writing %s to journal file.\n\x00" as *const u8 as *const libc::c_char,
            qpath,
        );
        FS_Write(
            &mut len as *mut libc::c_long as *const libc::c_void,
            ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as i32,
            crate::src::qcommon::common::com_journalDataFile,
        );
        FS_Write(
            buf as *const libc::c_void,
            len as i32,
            crate::src::qcommon::common::com_journalDataFile,
        );
        FS_Flush(crate::src::qcommon::common::com_journalDataFile);
    }
    return len;
}
/*
============
FS_ReadFile

Filename are relative to the quake search path
a null buffer will just return the file length without loading
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ReadFile(
    mut qpath: *const libc::c_char,
    mut buffer: *mut *mut libc::c_void,
) -> libc::c_long {
    return FS_ReadFileDir(
        qpath,
        0 as *mut libc::c_void,
        crate::src::qcommon::q_shared::qfalse,
        buffer,
    );
}
/*
=============
FS_FreeFile
=============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FreeFile(mut buffer: *mut libc::c_void) {
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if buffer.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FS_FreeFile( NULL )\x00" as *const u8 as *const libc::c_char,
        );
    }
    fs_loadStack -= 1;
    crate::src::qcommon::common::Hunk_FreeTempMemory(buffer);
    // if all of our temp files are free, clear all of our space
    if fs_loadStack == 0 as i32 {
        crate::src::qcommon::common::Hunk_ClearTempMemory();
    };
}
/*
============
FS_WriteFile

Filename are relative to the quake search path
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_WriteFile(
    mut qpath: *const libc::c_char,
    mut buffer: *const libc::c_void,
    mut size: i32,
) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if qpath.is_null() || buffer.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FS_WriteFile: NULL parameter\x00" as *const u8 as *const libc::c_char,
        );
    }
    f = FS_FOpenFileWrite(qpath);
    if f == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Failed to open %s\n\x00" as *const u8 as *const libc::c_char,
            qpath,
        );
        return;
    }
    FS_Write(buffer, size, f);
    FS_FCloseFile(f);
}
/*
==========================================================================

ZIP FILE LOADING

==========================================================================
*/
/*
=================
FS_LoadZipFile

Creates a new pak_t in the search chain for the contents
of a zip file.
=================
*/

unsafe extern "C" fn FS_LoadZipFile(
    mut zipfile: *const libc::c_char,
    mut basename: *const libc::c_char,
) -> *mut pack_t {
    let mut buildBuffer: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut uf: crate::src::qcommon::unzip::unzFile = 0 as *mut libc::c_void;
    let mut err: i32 = 0;
    let mut gi: crate::src::qcommon::unzip::unz_global_info =
        crate::src::qcommon::unzip::unz_global_info {
            number_entry: 0,
            size_comment: 0,
        };
    let mut filename_inzip: [libc::c_char; 256] = [0; 256];
    let mut file_info: crate::src::qcommon::unzip::unz_file_info =
        crate::src::qcommon::unzip::unz_file_info {
            version: 0,
            version_needed: 0,
            flag: 0,
            compression_method: 0,
            dosDate: 0,
            crc: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            size_filename: 0,
            size_file_extra: 0,
            size_file_comment: 0,
            disk_num_start: 0,
            internal_fa: 0,
            external_fa: 0,
            tmu_date: crate::src::qcommon::unzip::tm_unz {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
            },
        };
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut hash: libc::c_long = 0;
    let mut fs_numHeaderLongs: i32 = 0;
    let mut fs_headerLongs: *mut i32 = 0 as *mut i32;
    let mut namePtr: *mut libc::c_char = 0 as *mut libc::c_char;
    fs_numHeaderLongs = 0 as i32;
    uf = crate::src::qcommon::unzip::unzOpen(zipfile);
    err = crate::src::qcommon::unzip::unzGetGlobalInfo(
        uf,
        &mut gi as *mut _ as *mut crate::src::qcommon::unzip::unz_global_info_s,
    );
    if err != 0 as i32 {
        return 0 as *mut pack_t;
    }
    len = 0 as i32;
    crate::src::qcommon::unzip::unzGoToFirstFile(uf);
    i = 0 as i32;
    while (i as libc::c_ulong) < gi.number_entry {
        err = crate::src::qcommon::unzip::unzGetCurrentFileInfo(
            uf,
            &mut file_info as *mut _ as *mut crate::src::qcommon::unzip::unz_file_info_s,
            filename_inzip.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            0 as *mut libc::c_void,
            0 as i32 as crate::zconf_h::uLong,
            0 as *mut libc::c_char,
            0 as i32 as crate::zconf_h::uLong,
        );
        if err != 0 as i32 {
            break;
        }
        len = (len as libc::c_ulong).wrapping_add(
            crate::stdlib::strlen(filename_inzip.as_mut_ptr())
                .wrapping_add(1 as i32 as libc::c_ulong),
        ) as i32 as i32;
        crate::src::qcommon::unzip::unzGoToNextFile(uf);
        i += 1
    }
    buildBuffer = crate::src::qcommon::common::Z_Malloc(
        gi.number_entry
            .wrapping_mul(::std::mem::size_of::<fileInPack_t>() as libc::c_ulong)
            .wrapping_add(len as libc::c_ulong) as i32,
    ) as *mut fileInPack_t;
    namePtr = (buildBuffer as *mut libc::c_char).offset(
        gi.number_entry
            .wrapping_mul(::std::mem::size_of::<fileInPack_t>() as libc::c_ulong) as isize,
    );
    fs_headerLongs = crate::src::qcommon::common::Z_Malloc(
        gi.number_entry
            .wrapping_add(1 as i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<i32>() as libc::c_ulong) as i32,
    ) as *mut i32;
    let fresh2 = fs_numHeaderLongs;
    fs_numHeaderLongs = fs_numHeaderLongs + 1;
    *fs_headerLongs.offset(fresh2 as isize) = fs_checksumFeed;
    // get the hash table size from the number of files in the zip
    // because lots of custom pk3 files have less than 32 or 64 files
    i = 1 as i32;
    while i <= 1024 as i32 {
        if i as libc::c_ulong > gi.number_entry {
            break;
        }
        i <<= 1 as i32
    }
    pack = crate::src::qcommon::common::Z_Malloc(
        (::std::mem::size_of::<pack_t>() as libc::c_ulong).wrapping_add(
            (i as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut fileInPack_t>() as libc::c_ulong),
        ) as i32,
    ) as *mut pack_t;
    (*pack).hashSize = i;
    (*pack).hashTable = (pack as *mut libc::c_char)
        .offset(::std::mem::size_of::<pack_t>() as libc::c_ulong as isize)
        as *mut *mut fileInPack_t;
    i = 0 as i32;
    while i < (*pack).hashSize {
        let ref mut fresh3 = *(*pack).hashTable.offset(i as isize);
        *fresh3 = 0 as *mut fileInPack_t;
        i += 1
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*pack).pakFilename.as_mut_ptr(),
        zipfile,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*pack).pakBasename.as_mut_ptr(),
        basename,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    // strip .pk3 if needed
    if crate::stdlib::strlen((*pack).pakBasename.as_mut_ptr()) > 4 as i32 as libc::c_ulong
        && crate::src::qcommon::q_shared::Q_stricmp(
            (*pack)
                .pakBasename
                .as_mut_ptr()
                .offset(crate::stdlib::strlen((*pack).pakBasename.as_mut_ptr()) as isize)
                .offset(-(4 as i32 as isize)),
            b".pk3\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        (*pack).pakBasename[crate::stdlib::strlen((*pack).pakBasename.as_mut_ptr())
            .wrapping_sub(4 as i32 as libc::c_ulong) as usize] = 0 as i32 as libc::c_char
    }
    (*pack).handle = uf;
    (*pack).numfiles = gi.number_entry as i32;
    crate::src::qcommon::unzip::unzGoToFirstFile(uf);
    i = 0 as i32;
    while (i as libc::c_ulong) < gi.number_entry {
        err = crate::src::qcommon::unzip::unzGetCurrentFileInfo(
            uf,
            &mut file_info as *mut _ as *mut crate::src::qcommon::unzip::unz_file_info_s,
            filename_inzip.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            0 as *mut libc::c_void,
            0 as i32 as crate::zconf_h::uLong,
            0 as *mut libc::c_char,
            0 as i32 as crate::zconf_h::uLong,
        );
        if err != 0 as i32 {
            break;
        }
        if file_info.uncompressed_size > 0 as i32 as libc::c_ulong {
            let fresh4 = fs_numHeaderLongs;
            fs_numHeaderLongs = fs_numHeaderLongs + 1;
            *fs_headerLongs.offset(fresh4 as isize) = file_info.crc as i32
        }
        crate::src::qcommon::q_shared::Q_strlwr(filename_inzip.as_mut_ptr());
        hash = FS_HashFileName(filename_inzip.as_mut_ptr(), (*pack).hashSize);
        let ref mut fresh5 = (*buildBuffer.offset(i as isize)).name;
        *fresh5 = namePtr;
        ::libc::strcpy(
            (*buildBuffer.offset(i as isize)).name,
            filename_inzip.as_mut_ptr(),
        );
        namePtr = namePtr.offset(
            crate::stdlib::strlen(filename_inzip.as_mut_ptr())
                .wrapping_add(1 as i32 as libc::c_ulong) as isize,
        );
        // store the file position in the zip
        (*buildBuffer.offset(i as isize)).pos = crate::src::qcommon::unzip::unzGetOffset(uf);
        (*buildBuffer.offset(i as isize)).len = file_info.uncompressed_size;
        let ref mut fresh6 = (*buildBuffer.offset(i as isize)).next;
        *fresh6 = *(*pack).hashTable.offset(hash as isize);
        let ref mut fresh7 = *(*pack).hashTable.offset(hash as isize);
        *fresh7 = &mut *buildBuffer.offset(i as isize) as *mut fileInPack_t;
        crate::src::qcommon::unzip::unzGoToNextFile(uf);
        i += 1
    }
    (*pack).checksum = crate::src::qcommon::md4::Com_BlockChecksum(
        &mut *fs_headerLongs.offset(1 as i32 as isize) as *mut i32 as *const libc::c_void,
        (::std::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul((fs_numHeaderLongs - 1 as i32) as libc::c_ulong) as i32,
    ) as i32;
    (*pack).pure_checksum = crate::src::qcommon::md4::Com_BlockChecksum(
        fs_headerLongs as *const libc::c_void,
        (::std::mem::size_of::<i32>() as libc::c_ulong)
            .wrapping_mul(fs_numHeaderLongs as libc::c_ulong) as i32,
    ) as i32;
    (*pack).checksum = (*pack).checksum;
    (*pack).pure_checksum = (*pack).pure_checksum;
    crate::src::qcommon::common::Z_Free(fs_headerLongs as *mut libc::c_void);
    (*pack).buildBuffer = buildBuffer;
    return pack;
}
/*
=================
FS_FreePak

Frees a pak structure and releases all associated resources
=================
*/

unsafe extern "C" fn FS_FreePak(mut thepak: *mut pack_t) {
    crate::src::qcommon::unzip::unzClose((*thepak).handle);
    crate::src::qcommon::common::Z_Free((*thepak).buildBuffer as *mut libc::c_void);
    crate::src::qcommon::common::Z_Free(thepak as *mut libc::c_void);
}
/*
=================
FS_GetZipChecksum

Compares whether the given pak file matches a referenced checksum
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_CompareZipChecksum(
    mut zipfile: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut thepak: *mut pack_t = 0 as *mut pack_t;
    let mut index: i32 = 0;
    let mut checksum: i32 = 0;
    thepak = FS_LoadZipFile(zipfile, b"\x00" as *const u8 as *const libc::c_char);
    if thepak.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    checksum = (*thepak).checksum;
    FS_FreePak(thepak);
    index = 0 as i32;
    while index < fs_numServerReferencedPaks {
        if checksum == fs_serverReferencedPaks[index as usize] {
            return crate::src::qcommon::q_shared::qtrue;
        }
        index += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
}

unsafe extern "C" fn FS_ReturnPath(
    mut zname: *const libc::c_char,
    mut zpath: *mut libc::c_char,
    mut depth: *mut i32,
) -> i32 {
    let mut len: i32 = 0;
    let mut at: i32 = 0;
    let mut newdep: i32 = 0;
    newdep = 0 as i32;
    *zpath.offset(0 as i32 as isize) = 0 as i32 as libc::c_char;
    len = 0 as i32;
    at = 0 as i32;
    while *zname.offset(at as isize) as i32 != 0 as i32 {
        if *zname.offset(at as isize) as i32 == '/' as i32
            || *zname.offset(at as isize) as i32 == '\\' as i32
        {
            len = at;
            newdep += 1
        }
        at += 1
    }
    ::libc::strcpy(zpath, zname);
    *zpath.offset(len as isize) = 0 as i32 as libc::c_char;
    *depth = newdep;
    return len;
}
/*
==================
FS_AddFileToList
==================
*/

unsafe extern "C" fn FS_AddFileToList(
    mut name: *mut libc::c_char,
    mut list: *mut *mut libc::c_char,
    mut nfiles: i32,
) -> i32 {
    let mut i: i32 = 0;
    if nfiles == 0x1000 as i32 - 1 as i32 {
        return nfiles;
    }
    i = 0 as i32;
    while i < nfiles {
        if crate::src::qcommon::q_shared::Q_stricmp(name, *list.offset(i as isize)) == 0 {
            return nfiles;
            // already in list
        }
        i += 1
    }
    let ref mut fresh8 = *list.offset(nfiles as isize);
    *fresh8 = crate::src::qcommon::common::CopyString(name);
    nfiles += 1;
    return nfiles;
}
/*
===============
FS_ListFilteredFiles

Returns a uniqued list of files that match the given criteria
from all search paths
===============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ListFilteredFiles(
    mut path: *const libc::c_char,
    mut extension: *const libc::c_char,
    mut filter: *mut libc::c_char,
    mut numfiles: *mut i32,
    mut allowNonPureFilesOnDisk: crate::src::qcommon::q_shared::qboolean,
) -> *mut *mut libc::c_char {
    let mut nfiles: i32 = 0;
    let mut listCopy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut list: [*mut libc::c_char; 4096] = [0 as *mut libc::c_char; 4096];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: i32 = 0;
    let mut pathLength: i32 = 0;
    let mut extensionLength: i32 = 0;
    let mut length: i32 = 0;
    let mut pathDepth: i32 = 0;
    let mut temp: i32 = 0;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut buildBuffer: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut zpath: [libc::c_char; 256] = [0; 256];
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if path.is_null() {
        *numfiles = 0 as i32;
        return 0 as *mut *mut libc::c_char;
    }
    if extension.is_null() {
        extension = b"\x00" as *const u8 as *const libc::c_char
    }
    pathLength = crate::stdlib::strlen(path) as i32;
    if *path.offset((pathLength - 1 as i32) as isize) as i32 == '\\' as i32
        || *path.offset((pathLength - 1 as i32) as isize) as i32 == '/' as i32
    {
        pathLength -= 1
    }
    extensionLength = crate::stdlib::strlen(extension) as i32;
    nfiles = 0 as i32;
    FS_ReturnPath(path, zpath.as_mut_ptr(), &mut pathDepth);
    //
    // search through the path, one element at a time, adding to list
    //
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            //ZOID:  If we are pure, don't search for files on paks that
            // aren't on the pure list
            if !(FS_PakIsPure((*search).pack) as u64 == 0) {
                // look through all the pak file elements
                pak = (*search).pack;
                buildBuffer = (*pak).buildBuffer;
                i = 0 as i32;
                while i < (*pak).numfiles {
                    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut zpathLen: i32 = 0;
                    let mut depth: i32 = 0;
                    // check for directory match
                    name = (*buildBuffer.offset(i as isize)).name;
                    //
                    if !filter.is_null() {
                        // case insensitive
                        if !(crate::src::qcommon::common::Com_FilterPath(
                            filter,
                            name,
                            crate::src::qcommon::q_shared::qfalse as i32,
                        ) == 0)
                        {
                            // unique the match
                            nfiles = FS_AddFileToList(name, list.as_mut_ptr(), nfiles)
                        }
                    } else {
                        zpathLen = FS_ReturnPath(name, zpath.as_mut_ptr(), &mut depth);
                        if !(depth - pathDepth > 2 as i32
                            || pathLength > zpathLen
                            || crate::src::qcommon::q_shared::Q_stricmpn(name, path, pathLength)
                                != 0)
                        {
                            // check for extension match
                            length = crate::stdlib::strlen(name) as i32;
                            if !(length < extensionLength) {
                                if !(crate::src::qcommon::q_shared::Q_stricmp(
                                    name.offset(length as isize)
                                        .offset(-(extensionLength as isize)),
                                    extension,
                                ) != 0)
                                {
                                    // unique the match
                                    temp = pathLength;
                                    if pathLength != 0 {
                                        temp += 1
                                        // include the '/'
                                    }
                                    nfiles = FS_AddFileToList(
                                        name.offset(temp as isize),
                                        list.as_mut_ptr(),
                                        nfiles,
                                    )
                                }
                            }
                        }
                    }
                    i += 1
                }
            }
        } else if !(*search).dir.is_null() {
            // scan for files in the filesystem
            let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut numSysFiles: i32 = 0;
            let mut sysFiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
            // don't scan directories for files if we are pure or restricted
            if !(fs_numServerPaks != 0 && allowNonPureFilesOnDisk as u64 == 0) {
                netpath = FS_BuildOSPath(
                    (*(*search).dir).path.as_mut_ptr(),
                    (*(*search).dir).gamedir.as_mut_ptr(),
                    path,
                );
                sysFiles = crate::src::sys::sys_unix::Sys_ListFiles(
                    netpath,
                    extension,
                    filter,
                    &mut numSysFiles,
                    crate::src::qcommon::q_shared::qfalse,
                );
                i = 0 as i32;
                while i < numSysFiles {
                    // unique the match
                    name_0 = *sysFiles.offset(i as isize);
                    nfiles = FS_AddFileToList(name_0, list.as_mut_ptr(), nfiles);
                    i += 1
                }
                crate::src::sys::sys_unix::Sys_FreeFileList(sysFiles);
            }
        }
        search = (*search).next
    }
    // return a copy of the list
    *numfiles = nfiles;
    if nfiles == 0 {
        return 0 as *mut *mut libc::c_char;
    }
    listCopy = crate::src::qcommon::common::Z_Malloc(
        ((nfiles + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as i32,
    ) as *mut *mut libc::c_char;
    i = 0 as i32;
    while i < nfiles {
        let ref mut fresh9 = *listCopy.offset(i as isize);
        *fresh9 = list[i as usize];
        i += 1
    }
    let ref mut fresh10 = *listCopy.offset(i as isize);
    *fresh10 = 0 as *mut libc::c_char;
    return listCopy;
}
/*
=================
FS_ListFiles
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ListFiles(
    mut path: *const libc::c_char,
    mut extension: *const libc::c_char,
    mut numfiles: *mut i32,
) -> *mut *mut libc::c_char {
    return FS_ListFilteredFiles(
        path,
        extension,
        0 as *mut libc::c_char,
        numfiles,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
=================
FS_FreeFileList
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FreeFileList(mut list: *mut *mut libc::c_char) {
    let mut i: i32 = 0;
    if fs_searchpaths.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Filesystem call made without initialization\x00" as *const u8 as *const libc::c_char,
        );
    }
    if list.is_null() {
        return;
    }
    i = 0 as i32;
    while !(*list.offset(i as isize)).is_null() {
        crate::src::qcommon::common::Z_Free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    crate::src::qcommon::common::Z_Free(list as *mut libc::c_void);
}
/*
================
FS_GetFileList
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_GetFileList(
    mut path: *const libc::c_char,
    mut extension: *const libc::c_char,
    mut listbuf: *mut libc::c_char,
    mut bufsize: i32,
) -> i32 {
    let mut nFiles: i32 = 0;
    let mut i: i32 = 0;
    let mut nTotal: i32 = 0;
    let mut nLen: i32 = 0;
    let mut pFiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *listbuf = 0 as i32 as libc::c_char;
    nFiles = 0 as i32;
    nTotal = 0 as i32;
    if crate::src::qcommon::q_shared::Q_stricmp(
        path,
        b"$modlist\x00" as *const u8 as *const libc::c_char,
    ) == 0 as i32
    {
        return FS_GetModList(listbuf, bufsize);
    }
    pFiles = FS_ListFiles(path, extension, &mut nFiles);
    i = 0 as i32;
    while i < nFiles {
        nLen = crate::stdlib::strlen(*pFiles.offset(i as isize))
            .wrapping_add(1 as i32 as libc::c_ulong) as i32;
        if (nTotal + nLen + 1 as i32) < bufsize {
            ::libc::strcpy(listbuf, *pFiles.offset(i as isize));
            listbuf = listbuf.offset(nLen as isize);
            nTotal += nLen;
            i += 1
        } else {
            nFiles = i;
            break;
        }
    }
    FS_FreeFileList(pFiles);
    return nFiles;
}
/*
=======================
Sys_ConcatenateFileLists

mkv: Naive implementation. Concatenates three lists into a
     new list, and frees the old lists from the heap.
bk001129 - from cvs1.17 (mkv)

FIXME TTimo those two should move to common.c next to Sys_ListFiles
=======================
 */

unsafe extern "C" fn Sys_CountFileList(mut list: *mut *mut libc::c_char) -> u32 {
    let mut i: i32 = 0 as i32;
    if !list.is_null() {
        while !(*list).is_null() {
            list = list.offset(1);
            i += 1
        }
    }
    return i as u32;
}

unsafe extern "C" fn Sys_ConcatenateFileLists(
    mut list0: *mut *mut libc::c_char,
    mut list1: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    let mut totalLength: i32 = 0 as i32;
    let mut cat: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut dst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut src: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    totalLength = (totalLength as u32).wrapping_add(Sys_CountFileList(list0)) as i32 as i32;
    totalLength = (totalLength as u32).wrapping_add(Sys_CountFileList(list1)) as i32 as i32;
    /* Create new list. */
    cat = crate::src::qcommon::common::Z_Malloc(
        ((totalLength + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as i32,
    ) as *mut *mut libc::c_char;
    dst = cat;
    /* Copy over lists. */
    if !list0.is_null() {
        src = list0;
        while !(*src).is_null() {
            *dst = *src;
            src = src.offset(1);
            dst = dst.offset(1)
        }
    }
    if !list1.is_null() {
        src = list1;
        while !(*src).is_null() {
            *dst = *src;
            src = src.offset(1);
            dst = dst.offset(1)
        }
    }
    // Terminate the list
    *dst = 0 as *mut libc::c_char;
    // Free our old lists.
    // NOTE: not freeing their content, it's been merged in dst and still being used
    if !list0.is_null() {
        crate::src::qcommon::common::Z_Free(list0 as *mut libc::c_void);
    }
    if !list1.is_null() {
        crate::src::qcommon::common::Z_Free(list1 as *mut libc::c_void);
    }
    return cat;
}
/*
================
FS_GetModDescription
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_GetModDescription(
    mut modDir: *const libc::c_char,
    mut description: *mut libc::c_char,
    mut descriptionLen: i32,
) {
    let mut descHandle: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut descPath: [libc::c_char; 64] = [0; 64];
    let mut nDescLen: i32 = 0;
    let mut file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    crate::src::qcommon::q_shared::Com_sprintf(
        descPath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b"%s%cdescription.txt\x00" as *const u8 as *const libc::c_char,
        modDir,
        '/' as i32,
    );
    nDescLen = FS_SV_FOpenFileRead(descPath.as_mut_ptr(), &mut descHandle) as i32;
    if nDescLen > 0 as i32 {
        file = FS_FileForHandle(descHandle);
        crate::stdlib::memset(
            description as *mut libc::c_void,
            0 as i32,
            descriptionLen as libc::c_ulong,
        );
        nDescLen = crate::stdlib::fread(
            description as *mut libc::c_void,
            1 as i32 as libc::c_ulong,
            descriptionLen as libc::c_ulong,
            file,
        ) as i32;
        if nDescLen >= 0 as i32 {
            *description.offset(nDescLen as isize) = '\u{0}' as i32 as libc::c_char
        }
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(description, modDir, descriptionLen);
    }
    if descHandle != 0 {
        FS_FCloseFile(descHandle);
    };
}
/*
================
FS_GetModList

Returns a list of mod directory names
A mod directory is a peer to baseq3 with a pk3 or pk3dir in it
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_GetModList(mut listbuf: *mut libc::c_char, mut bufsize: i32) -> i32 {
    let mut nMods: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nTotal: i32 = 0;
    let mut nLen: i32 = 0;
    let mut nPaks: i32 = 0;
    let mut nDirs: i32 = 0;
    let mut nPakDirs: i32 = 0;
    let mut nPotential: i32 = 0;
    let mut nDescLen: i32 = 0;
    let mut pFiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pPaks: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pDirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut description: [libc::c_char; 4096] = [0; 4096];
    let mut dummy: i32 = 0;
    let mut pFiles0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut bDrop: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    // paths to search for mods
    let paths: [*const libc::c_char; 4] = [
        (*fs_basepath).string as *const libc::c_char,
        (*fs_homepath).string as *const libc::c_char,
        (*fs_steampath).string as *const libc::c_char,
        (*fs_gogpath).string as *const libc::c_char,
    ];
    *listbuf = 0 as i32 as libc::c_char;
    nTotal = 0 as i32;
    nMods = nTotal;
    // iterate through paths and get list of potential mods
    i = 0 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        pFiles0 = crate::src::sys::sys_unix::Sys_ListFiles(
            paths[i as usize],
            0 as *const libc::c_char,
            0 as *mut libc::c_char,
            &mut dummy,
            crate::src::qcommon::q_shared::qtrue,
        );
        // Sys_ConcatenateFileLists frees the lists so Sys_FreeFileList isn't required
        pFiles = Sys_ConcatenateFileLists(pFiles, pFiles0);
        i += 1
    }
    nPotential = Sys_CountFileList(pFiles) as i32;
    i = 0 as i32;
    while i < nPotential {
        name = *pFiles.offset(i as isize);
        // NOTE: cleaner would involve more changes
        // ignore duplicate mod directories
        if i != 0 as i32 {
            bDrop = crate::src::qcommon::q_shared::qfalse;
            j = 0 as i32;
            while j < i {
                if crate::src::qcommon::q_shared::Q_stricmp(*pFiles.offset(j as isize), name)
                    == 0 as i32
                {
                    // this one can be dropped
                    bDrop = crate::src::qcommon::q_shared::qtrue;
                    break;
                } else {
                    j += 1
                }
            }
        }
        // we also drop "baseq3" "." and ".."
        if !(bDrop as u32 != 0
            || crate::src::qcommon::q_shared::Q_stricmp(
                name,
                (*crate::src::qcommon::common::com_basegame).string,
            ) == 0 as i32
            || crate::src::qcommon::q_shared::Q_stricmpn(
                name,
                b".\x00" as *const u8 as *const libc::c_char,
                1 as i32,
            ) == 0 as i32)
        {
            // in order to be a valid mod the directory must contain at least one .pk3 or .pk3dir
            // we didn't keep the information when we merged the directory names, as to what OS Path it was found under
            // so we will try each of them here
            j = 0 as i32;
            while (j as libc::c_ulong)
                < (::std::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            {
                path = FS_BuildOSPath(
                    paths[j as usize],
                    name,
                    b"\x00" as *const u8 as *const libc::c_char,
                );
                nPakDirs = 0 as i32;
                nDirs = nPakDirs;
                nPaks = nDirs;
                pPaks = crate::src::sys::sys_unix::Sys_ListFiles(
                    path,
                    b".pk3\x00" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                    &mut nPaks,
                    crate::src::qcommon::q_shared::qfalse,
                );
                pDirs = crate::src::sys::sys_unix::Sys_ListFiles(
                    path,
                    b"/\x00" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_char,
                    &mut nDirs,
                    crate::src::qcommon::q_shared::qfalse,
                );
                k = 0 as i32;
                while k < nDirs {
                    // we only want to count directories ending with ".pk3dir"
                    if FS_IsExt(
                        *pDirs.offset(k as isize),
                        b".pk3dir\x00" as *const u8 as *const libc::c_char,
                        crate::stdlib::strlen(*pDirs.offset(k as isize)) as i32,
                    ) as u64
                        != 0
                    {
                        nPakDirs += 1
                    }
                    k += 1
                }
                // we only use Sys_ListFiles to check whether files are present
                crate::src::sys::sys_unix::Sys_FreeFileList(pPaks);
                crate::src::sys::sys_unix::Sys_FreeFileList(pDirs);
                if nPaks > 0 as i32 || nPakDirs > 0 as i32 {
                    break;
                }
                j += 1
            }
            if nPaks > 0 as i32 || nPakDirs > 0 as i32 {
                nLen = crate::stdlib::strlen(name).wrapping_add(1 as i32 as libc::c_ulong) as i32;
                // nLen is the length of the mod path
                // we need to see if there is a description available
                FS_GetModDescription(
                    name,
                    description.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                );
                nDescLen = crate::stdlib::strlen(description.as_mut_ptr())
                    .wrapping_add(1 as i32 as libc::c_ulong) as i32;
                if !((nTotal + nLen + 1 as i32 + nDescLen + 1 as i32) < bufsize) {
                    break;
                }
                ::libc::strcpy(listbuf, name);
                listbuf = listbuf.offset(nLen as isize);
                ::libc::strcpy(listbuf, description.as_mut_ptr());
                listbuf = listbuf.offset(nDescLen as isize);
                nTotal += nLen + nDescLen;
                nMods += 1
            }
        }
        i += 1
    }
    crate::src::sys::sys_unix::Sys_FreeFileList(pFiles);
    return nMods;
}
//============================================================================
/*
================
FS_Dir_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Dir_f() {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ndirs: i32 = 0;
    let mut i: i32 = 0;
    if crate::src::qcommon::cmd::Cmd_Argc() < 2 as i32
        || crate::src::qcommon::cmd::Cmd_Argc() > 3 as i32
    {
        crate::src::qcommon::common::Com_Printf(
            b"usage: dir <directory> [extension]\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if crate::src::qcommon::cmd::Cmd_Argc() == 2 as i32 {
        path = crate::src::qcommon::cmd::Cmd_Argv(1 as i32);
        extension = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        path = crate::src::qcommon::cmd::Cmd_Argv(1 as i32);
        extension = crate::src::qcommon::cmd::Cmd_Argv(2 as i32)
    }
    crate::src::qcommon::common::Com_Printf(
        b"Directory of %s %s\n\x00" as *const u8 as *const libc::c_char,
        path,
        extension,
    );
    crate::src::qcommon::common::Com_Printf(
        b"---------------\n\x00" as *const u8 as *const libc::c_char,
    );
    dirnames = FS_ListFiles(path, extension, &mut ndirs);
    i = 0 as i32;
    while i < ndirs {
        crate::src::qcommon::common::Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            *dirnames.offset(i as isize),
        );
        i += 1
    }
    FS_FreeFileList(dirnames);
}
/*
===========
FS_ConvertPath
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ConvertPath(mut s: *mut libc::c_char) {
    while *s != 0 {
        if *s as i32 == '\\' as i32 || *s as i32 == ':' as i32 {
            *s = '/' as i32 as libc::c_char
        }
        s = s.offset(1)
    }
}
/*
===========
FS_PathCmp

Ignore case and seprator char distinctions
===========
*/
#[no_mangle]

pub unsafe extern "C" fn FS_PathCmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    loop {
        let fresh11 = s1;
        s1 = s1.offset(1);
        c1 = *fresh11 as i32;
        let fresh12 = s2;
        s2 = s2.offset(1);
        c2 = *fresh12 as i32;
        if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
            c1 -= 'a' as i32 - 'A' as i32
        }
        if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
            c2 -= 'a' as i32 - 'A' as i32
        }
        if c1 == '\\' as i32 || c1 == ':' as i32 {
            c1 = '/' as i32
        }
        if c2 == '\\' as i32 || c2 == ':' as i32 {
            c2 = '/' as i32
        }
        if c1 < c2 {
            return -(1 as i32);
            // strings not equal
        }
        if c1 > c2 {
            return 1 as i32;
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0 as i32;
    // strings are equal
}
/*
================
FS_SortFileList
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_SortFileList(mut filelist: *mut *mut libc::c_char, mut numfiles: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut numsortedfiles: i32 = 0;
    let mut sortedlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    sortedlist = crate::src::qcommon::common::Z_Malloc(
        ((numfiles + 1 as i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as i32,
    ) as *mut *mut libc::c_char;
    let ref mut fresh13 = *sortedlist.offset(0 as i32 as isize);
    *fresh13 = 0 as *mut libc::c_char;
    numsortedfiles = 0 as i32;
    i = 0 as i32;
    while i < numfiles {
        j = 0 as i32;
        while j < numsortedfiles {
            if FS_PathCmp(*filelist.offset(i as isize), *sortedlist.offset(j as isize)) < 0 as i32 {
                break;
            }
            j += 1
        }
        k = numsortedfiles;
        while k > j {
            let ref mut fresh14 = *sortedlist.offset(k as isize);
            *fresh14 = *sortedlist.offset((k - 1 as i32) as isize);
            k -= 1
        }
        let ref mut fresh15 = *sortedlist.offset(j as isize);
        *fresh15 = *filelist.offset(i as isize);
        numsortedfiles += 1;
        i += 1
    }
    crate::stdlib::memcpy(
        filelist as *mut libc::c_void,
        sortedlist as *const libc::c_void,
        (numfiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    crate::src::qcommon::common::Z_Free(sortedlist as *mut libc::c_void);
}
/*
================
FS_NewDir_f
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_NewDir_f() {
    let mut filter: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ndirs: i32 = 0;
    let mut i: i32 = 0;
    if crate::src::qcommon::cmd::Cmd_Argc() < 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"usage: fdir <filter>\n\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::common::Com_Printf(
            b"example: fdir *q3dm*.bsp\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    filter = crate::src::qcommon::cmd::Cmd_Argv(1 as i32);
    crate::src::qcommon::common::Com_Printf(
        b"---------------\n\x00" as *const u8 as *const libc::c_char,
    );
    dirnames = FS_ListFilteredFiles(
        b"\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        filter,
        &mut ndirs,
        crate::src::qcommon::q_shared::qfalse,
    );
    FS_SortFileList(dirnames, ndirs);
    i = 0 as i32;
    while i < ndirs {
        FS_ConvertPath(*dirnames.offset(i as isize));
        crate::src::qcommon::common::Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            *dirnames.offset(i as isize),
        );
        i += 1
    }
    crate::src::qcommon::common::Com_Printf(
        b"%d files listed\n\x00" as *const u8 as *const libc::c_char,
        ndirs,
    );
    FS_FreeFileList(dirnames);
}
/*
============
FS_Path_f

============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Path_f() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: i32 = 0;
    crate::src::qcommon::common::Com_Printf(
        b"We are looking in the current search path:\n\x00" as *const u8 as *const libc::c_char,
    );
    s = fs_searchpaths;
    while !s.is_null() {
        if !(*s).pack.is_null() {
            crate::src::qcommon::common::Com_Printf(
                b"%s (%i files)\n\x00" as *const u8 as *const libc::c_char,
                (*(*s).pack).pakFilename.as_mut_ptr(),
                (*(*s).pack).numfiles,
            );
            if fs_numServerPaks != 0 {
                if FS_PakIsPure((*s).pack) as u64 == 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"    not on the pure list\n\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    crate::src::qcommon::common::Com_Printf(
                        b"    on the pure list\n\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"%s%c%s\n\x00" as *const u8 as *const libc::c_char,
                (*(*s).dir).path.as_mut_ptr(),
                '/' as i32,
                (*(*s).dir).gamedir.as_mut_ptr(),
            );
        }
        s = (*s).next
    }
    crate::src::qcommon::common::Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    i = 1 as i32;
    while i < 64 as i32 {
        if !fsh[i as usize].handleFiles.file.o.is_null() {
            crate::src::qcommon::common::Com_Printf(
                b"handle %i: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                fsh[i as usize].name.as_mut_ptr(),
            );
        }
        i += 1
    }
}
/*
============
FS_TouchFile_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_TouchFile_f() {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"Usage: touchFile <file>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    FS_FOpenFileRead(
        crate::src::qcommon::cmd::Cmd_Argv(1 as i32),
        &mut f,
        crate::src::qcommon::q_shared::qfalse,
    );
    if f != 0 {
        FS_FCloseFile(f);
    };
}
/*
============
FS_Which
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Which(
    mut filename: *const libc::c_char,
    mut searchPath: *mut libc::c_void,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut search: *mut searchpath_t = searchPath as *mut searchpath_t;
    if FS_FOpenFileReadDir(
        filename,
        search,
        0 as *mut crate::src::qcommon::q_shared::fileHandle_t,
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qfalse,
    ) > 0 as i32 as libc::c_long
    {
        if !(*search).pack.is_null() {
            crate::src::qcommon::common::Com_Printf(
                b"File \"%s\" found in \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                filename,
                (*(*search).pack).pakFilename.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qtrue;
        } else {
            if !(*search).dir.is_null() {
                crate::src::qcommon::common::Com_Printf(
                    b"File \"%s\" found at \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                    filename,
                    (*(*search).dir).fullpath.as_mut_ptr(),
                );
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
============
FS_Which_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Which_f() {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    filename = crate::src::qcommon::cmd::Cmd_Argv(1 as i32);
    if *filename.offset(0 as i32 as isize) == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Usage: which <file>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // qpaths are not supposed to have a leading slash
    if *filename.offset(0 as i32 as isize) as i32 == '/' as i32
        || *filename.offset(0 as i32 as isize) as i32 == '\\' as i32
    {
        filename = filename.offset(1)
    }
    // just wants to see if file is there
    search = fs_searchpaths;
    while !search.is_null() {
        if FS_Which(filename, search as *mut libc::c_void) as u64 != 0 {
            return;
        }
        search = (*search).next
    }
    crate::src::qcommon::common::Com_Printf(
        b"File not found: \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        filename,
    );
}
//===========================================================================

unsafe extern "C" fn paksort(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    let mut aa: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bb: *mut libc::c_char = 0 as *mut libc::c_char;
    aa = *(a as *mut *mut libc::c_char);
    bb = *(b as *mut *mut libc::c_char);
    return FS_PathCmp(aa, bb);
}
/*
================
FS_AddGameDirectory

Sets fs_gamedir, adds the directory to the head of the path,
then loads the zip headers
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_AddGameDirectory(
    mut path: *const libc::c_char,
    mut dir: *const libc::c_char,
) {
    let mut sp: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut curpath: [libc::c_char; 4097] = [0; 4097];
    let mut pakfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numfiles: i32 = 0;
    let mut pakfiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakfilesi: i32 = 0;
    let mut pakfilestmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut numdirs: i32 = 0;
    let mut pakdirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakdirsi: i32 = 0;
    let mut pakdirstmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakwhich: i32 = 0;
    let mut len: i32 = 0;
    // Unique
    sp = fs_searchpaths;
    while !sp.is_null() {
        if !(*sp).dir.is_null()
            && crate::src::qcommon::q_shared::Q_stricmp((*(*sp).dir).path.as_mut_ptr(), path) == 0
            && crate::src::qcommon::q_shared::Q_stricmp((*(*sp).dir).gamedir.as_mut_ptr(), dir) == 0
        {
            return;
            // we've already got this one
        }
        sp = (*sp).next
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        fs_gamedir.as_mut_ptr(),
        dir,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    // find all pak files in this directory
    crate::src::qcommon::q_shared::Q_strncpyz(
        curpath.as_mut_ptr(),
        FS_BuildOSPath(path, dir, b"\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong as i32,
    ); // strip the trailing slash
    curpath[crate::stdlib::strlen(curpath.as_mut_ptr()).wrapping_sub(1 as i32 as libc::c_ulong)
        as usize] = '\u{0}' as i32 as libc::c_char;
    // Get .pk3 files
    pakfiles = crate::src::sys::sys_unix::Sys_ListFiles(
        curpath.as_mut_ptr(),
        b".pk3\x00" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_char,
        &mut numfiles,
        crate::src::qcommon::q_shared::qfalse,
    );
    crate::stdlib::qsort(
        pakfiles as *mut libc::c_void,
        numfiles as crate::stddef_h::size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            paksort as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    if fs_numServerPaks != 0 {
        numdirs = 0 as i32;
        pakdirs = 0 as *mut *mut libc::c_char
    } else {
        // Get top level directories (we'll filter them later since the Sys_ListFiles filtering is terrible)
        pakdirs = crate::src::sys::sys_unix::Sys_ListFiles(
            curpath.as_mut_ptr(),
            b"/\x00" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_char,
            &mut numdirs,
            crate::src::qcommon::q_shared::qfalse,
        );
        crate::stdlib::qsort(
            pakdirs as *mut libc::c_void,
            numdirs as crate::stddef_h::size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                paksort
                    as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
            ),
        );
    }
    pakfilesi = 0 as i32;
    pakdirsi = 0 as i32;
    while pakfilesi < numfiles || pakdirsi < numdirs {
        // Check if a pakfile or pakdir comes next
        if pakfilesi >= numfiles {
            // We've used all the pakfiles, it must be a pakdir.
            pakwhich = 0 as i32
        } else if pakdirsi >= numdirs {
            // We've used all the pakdirs, it must be a pakfile.
            pakwhich = 1 as i32
        } else {
            // Could be either, compare to see which name comes first
            // Need tmp variables for appropriate indirection for paksort()
            pakfilestmp = &mut *pakfiles.offset(pakfilesi as isize) as *mut *mut libc::c_char;
            pakdirstmp = &mut *pakdirs.offset(pakdirsi as isize) as *mut *mut libc::c_char;
            pakwhich = (paksort(
                pakfilestmp as *const libc::c_void,
                pakdirstmp as *const libc::c_void,
            ) < 0 as i32) as i32
        }
        if pakwhich != 0 {
            // The next .pk3 file is before the next .pk3dir
            pakfile = FS_BuildOSPath(path, dir, *pakfiles.offset(pakfilesi as isize));
            pak = FS_LoadZipFile(pakfile, *pakfiles.offset(pakfilesi as isize));
            if pak.is_null() {
                // This isn't a .pk3! Next!
                pakfilesi += 1
            } else {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    (*pak).pakPathname.as_mut_ptr(),
                    curpath.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                );
                // store the game name for downloading
                crate::src::qcommon::q_shared::Q_strncpyz(
                    (*pak).pakGamename.as_mut_ptr(),
                    dir,
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                );
                fs_packFiles += (*pak).numfiles;
                search =
                    crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<searchpath_t>()
                        as libc::c_ulong
                        as i32) as *mut searchpath_t;
                (*search).pack = pak;
                (*search).next = fs_searchpaths;
                fs_searchpaths = search;
                pakfilesi += 1
            }
        } else {
            // The next .pk3dir is before the next .pk3 file
            // But wait, this could be any directory, we're filtering to only ending with ".pk3dir" here.
            len = crate::stdlib::strlen(*pakdirs.offset(pakdirsi as isize)) as i32;
            if FS_IsExt(
                *pakdirs.offset(pakdirsi as isize),
                b".pk3dir\x00" as *const u8 as *const libc::c_char,
                len,
            ) as u64
                == 0
            {
                // This isn't a .pk3dir! Next!
                pakdirsi += 1
            } else {
                pakfile = FS_BuildOSPath(path, dir, *pakdirs.offset(pakdirsi as isize));
                // add the directory to the search path
                search =
                    crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<searchpath_t>()
                        as libc::c_ulong
                        as i32) as *mut searchpath_t; // c:\quake3\baseq3
                (*search).dir =
                    crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<directory_t>()
                        as libc::c_ulong
                        as i32) as *mut directory_t; // c:\quake3\baseq3\mypak.pk3dir
                crate::src::qcommon::q_shared::Q_strncpyz(
                    (*(*search).dir).path.as_mut_ptr(),
                    curpath.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                ); // mypak.pk3dir
                crate::src::qcommon::q_shared::Q_strncpyz(
                    (*(*search).dir).fullpath.as_mut_ptr(),
                    pakfile,
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                );
                crate::src::qcommon::q_shared::Q_strncpyz(
                    (*(*search).dir).gamedir.as_mut_ptr(),
                    *pakdirs.offset(pakdirsi as isize),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
                );
                (*search).next = fs_searchpaths;
                fs_searchpaths = search;
                pakdirsi += 1
            }
        }
    }
    // done
    crate::src::sys::sys_unix::Sys_FreeFileList(pakfiles);
    crate::src::sys::sys_unix::Sys_FreeFileList(pakdirs);
    //
    // add the directory to the search path
    //
    search = crate::src::qcommon::common::Z_Malloc(
        ::std::mem::size_of::<searchpath_t>() as libc::c_ulong as i32
    ) as *mut searchpath_t;
    (*search).dir = crate::src::qcommon::common::Z_Malloc(
        ::std::mem::size_of::<directory_t>() as libc::c_ulong as i32
    ) as *mut directory_t;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*(*search).dir).path.as_mut_ptr(),
        path,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*(*search).dir).fullpath.as_mut_ptr(),
        curpath.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*(*search).dir).gamedir.as_mut_ptr(),
        dir,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    (*search).next = fs_searchpaths;
    fs_searchpaths = search;
}
/*
================
FS_idPak
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_idPak(
    mut pak: *mut libc::c_char,
    mut base: *mut libc::c_char,
    mut numPaks: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 9 as i32 {
        if FS_FilenameCompare(
            pak,
            crate::src::qcommon::q_shared::va(
                b"%s/pak%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                base,
                i,
            ),
        ) as u64
            == 0
        {
            break;
        }
        i += 1
    }
    if i < numPaks {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
================
FS_CheckDirTraversal

Check whether the string contains stuff like "../" to prevent directory traversal bugs
and return qtrue if it does.
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_CheckDirTraversal(
    mut checkdir: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if !::libc::strstr(checkdir, b"../\x00" as *const u8 as *const libc::c_char).is_null()
        || !::libc::strstr(checkdir, b"..\\\x00" as *const u8 as *const libc::c_char).is_null()
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
================
FS_InvalidGameDir

return true if path is a reference to current directory or directory traversal
or a sub-directory
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_InvalidGameDir(
    mut gamedir: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if ::libc::strcmp(gamedir, b".\x00" as *const u8 as *const libc::c_char) == 0
        || ::libc::strcmp(gamedir, b"..\x00" as *const u8 as *const libc::c_char) == 0
        || !::libc::strchr(gamedir, '/' as i32).is_null()
        || !::libc::strchr(gamedir, '\\' as i32).is_null()
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
================
FS_ComparePaks

----------------
dlstring == qtrue

Returns a list of pak files that we should download from the server. They all get stored
in the current gamedir and an FS_Restart will be fired up after we download them all.

The string is the format:

@remotename@localname [repeat]

static int		fs_numServerReferencedPaks;
static int		fs_serverReferencedPaks[MAX_SEARCH_PATHS];
static char		*fs_serverReferencedPakNames[MAX_SEARCH_PATHS];

----------------
dlstring == qfalse

we are not interested in a download string format, we want something human-readable
(this is used for diagnostics while connecting to a pure server)

================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ComparePaks(
    mut neededpaks: *mut libc::c_char,
    mut len: i32,
    mut dlstring: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut sp: *mut searchpath_t = 0 as *mut searchpath_t; // Server didn't send any pack information along
    let mut havepak: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut origpos: *mut libc::c_char = neededpaks;
    let mut i: i32 = 0;
    if fs_numServerReferencedPaks == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *neededpaks = 0 as i32 as libc::c_char;
    i = 0 as i32;
    while i < fs_numServerReferencedPaks {
        // Ok, see if we have this pak file
        havepak = crate::src::qcommon::q_shared::qfalse;
        // never autodownload any of the id paks
        if !(FS_idPak(
            fs_serverReferencedPakNames[i as usize],
            b"baseq3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            9 as i32,
        ) as u32
            != 0
            || FS_idPak(
                fs_serverReferencedPakNames[i as usize],
                b"missionpack\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                4 as i32,
            ) as u32
                != 0)
        {
            // Make sure the server cannot make us write to non-quake3 directories.
            if FS_CheckDirTraversal(fs_serverReferencedPakNames[i as usize]) as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"WARNING: Invalid download name %s\n\x00" as *const u8 as *const libc::c_char,
                    fs_serverReferencedPakNames[i as usize],
                ); // This is it!
            } else {
                sp = fs_searchpaths;
                while !sp.is_null() {
                    if !(*sp).pack.is_null()
                        && (*(*sp).pack).checksum == fs_serverReferencedPaks[i as usize]
                    {
                        havepak = crate::src::qcommon::q_shared::qtrue;
                        break;
                    } else {
                        sp = (*sp).next
                    }
                }
                if havepak as u64 == 0
                    && !fs_serverReferencedPakNames[i as usize].is_null()
                    && *fs_serverReferencedPakNames[i as usize] as i32 != 0
                {
                    // Don't got it
                    if dlstring as u64 != 0 {
                        // We need this to make sure we won't hit the end of the buffer or the server could
                        // overwrite non-pk3 files on clients by writing so much crap into neededpaks that
                        // Q_strcat cuts off the .pk3 extension.
                        origpos = origpos.offset(crate::stdlib::strlen(origpos) as isize);
                        // Remote name
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            b"@\x00" as *const u8 as *const libc::c_char,
                        );
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            fs_serverReferencedPakNames[i as usize],
                        );
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            b".pk3\x00" as *const u8 as *const libc::c_char,
                        );
                        // Local name
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            b"@\x00" as *const u8 as *const libc::c_char,
                        );
                        // Do we have one with the same name?
                        if FS_SV_FileExists(crate::src::qcommon::q_shared::va(
                            b"%s.pk3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fs_serverReferencedPakNames[i as usize],
                        )) as u64
                            != 0
                        {
                            let mut st: [libc::c_char; 256] = [0; 256];
                            // We already have one called this, we need to download it to another name
                            // Make something up with the checksum in it
                            crate::src::qcommon::q_shared::Com_sprintf(
                                st.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                    as i32,
                                b"%s.%08x.pk3\x00" as *const u8 as *const libc::c_char,
                                fs_serverReferencedPakNames[i as usize],
                                fs_serverReferencedPaks[i as usize],
                            );
                            crate::src::qcommon::q_shared::Q_strcat(
                                neededpaks,
                                len,
                                st.as_mut_ptr(),
                            );
                        } else {
                            crate::src::qcommon::q_shared::Q_strcat(
                                neededpaks,
                                len,
                                fs_serverReferencedPakNames[i as usize],
                            );
                            crate::src::qcommon::q_shared::Q_strcat(
                                neededpaks,
                                len,
                                b".pk3\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                        // Find out whether it might have overflowed the buffer and don't add this file to the
                        // list if that is the case.
                        if crate::stdlib::strlen(origpos).wrapping_add(
                            origpos.offset_from(neededpaks) as libc::c_long as libc::c_ulong,
                        ) >= (len - 1 as i32) as libc::c_ulong
                        {
                            *origpos = '\u{0}' as i32 as libc::c_char;
                            break;
                        }
                    } else {
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            fs_serverReferencedPakNames[i as usize],
                        );
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            b".pk3\x00" as *const u8 as *const libc::c_char,
                        );
                        // Do we have one with the same name?
                        if FS_SV_FileExists(crate::src::qcommon::q_shared::va(
                            b"%s.pk3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fs_serverReferencedPakNames[i as usize],
                        )) as u64
                            != 0
                        {
                            crate::src::qcommon::q_shared::Q_strcat(
                                neededpaks,
                                len,
                                b" (local file exists with wrong checksum)\x00" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        crate::src::qcommon::q_shared::Q_strcat(
                            neededpaks,
                            len,
                            b"\n\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
        i += 1
    }
    if *neededpaks != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
    // We have them all
}
/*
================
FS_Shutdown

Frees all resources.
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Shutdown(mut _closemfp: crate::src::qcommon::q_shared::qboolean) {
    let mut p: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut next: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 64 as i32 {
        if fsh[i as usize].fileSize != 0 {
            FS_FCloseFile(i);
        }
        i += 1
    }
    // free everything
    p = fs_searchpaths;
    while !p.is_null() {
        next = (*p).next;
        if !(*p).pack.is_null() {
            FS_FreePak((*p).pack);
        }
        if !(*p).dir.is_null() {
            crate::src::qcommon::common::Z_Free((*p).dir as *mut libc::c_void);
        }
        crate::src::qcommon::common::Z_Free(p as *mut libc::c_void);
        p = next
    }
    // any FS_ calls will now be an error until reinitialized
    fs_searchpaths = 0 as *mut searchpath_t;
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"path\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"dir\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"fdir\x00" as *const u8 as *const libc::c_char);
    crate::src::qcommon::cmd::Cmd_RemoveCommand(
        b"touchFile\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cmd::Cmd_RemoveCommand(b"which\x00" as *const u8 as *const libc::c_char);
}
/*
================
FS_ReorderPurePaks
NOTE TTimo: the reordering that happens here is not reflected in the cvars (\cvarlist *pak*)
  this can lead to misleading situations, see https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=540
================
*/

unsafe extern "C" fn FS_ReorderPurePaks() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t; // when doing the scan
    let mut i: i32 = 0;
    let mut p_insert_index: *mut *mut searchpath_t = 0 as *mut *mut searchpath_t;
    let mut p_previous: *mut *mut searchpath_t = 0 as *mut *mut searchpath_t;
    fs_reordered = crate::src::qcommon::q_shared::qfalse;
    // only relevant when connected to pure server
    if fs_numServerPaks == 0 {
        return;
    } // we insert in order at the beginning of the list
    p_insert_index = &mut fs_searchpaths; // track the pointer-to-current-item
    i = 0 as i32;
    while i < fs_numServerPaks {
        p_previous = p_insert_index;
        s = *p_insert_index;
        while !s.is_null() {
            // the part of the list before p_insert_index has been sorted already
            if !(*s).pack.is_null() && fs_serverPaks[i as usize] == (*(*s).pack).checksum {
                fs_reordered = crate::src::qcommon::q_shared::qtrue;
                // iterate to next server pack
                // move this element to the insert list
                *p_previous = (*s).next;
                (*s).next = *p_insert_index;
                *p_insert_index = s;
                // increment insert list
                p_insert_index = &mut (*s).next;
                break;
            } else {
                p_previous = &mut (*s).next;
                s = (*s).next
            }
        }
        i += 1
    }
}
/*
================
FS_Startup
================
*/

unsafe extern "C" fn FS_Startup(mut gameName: *const libc::c_char) {
    let mut homePath: *const libc::c_char = 0 as *const libc::c_char;
    crate::src::qcommon::common::Com_Printf(
        b"----- FS_Startup -----\n\x00" as *const u8 as *const libc::c_char,
    );
    fs_packFiles = 0 as i32;
    fs_debug = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_debug\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    fs_basepath = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_basepath\x00" as *const u8 as *const libc::c_char,
        crate::src::sys::sys_main::Sys_DefaultInstallPath(),
        0x10 as i32 | 0x2000 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    fs_basegame = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_basegame\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    homePath = crate::src::sys::sys_unix::Sys_DefaultHomePath();
    if homePath.is_null() || *homePath.offset(0 as i32 as isize) == 0 {
        homePath = (*fs_basepath).string
    }
    fs_homepath = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_homepath\x00" as *const u8 as *const libc::c_char,
        homePath,
        0x10 as i32 | 0x2000 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    fs_gamedirvar = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as i32 | 0x8 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if *gameName.offset(0 as i32 as isize) == 0 {
        crate::src::qcommon::cvar::Cvar_ForceReset(
            b"com_basegame\x00" as *const u8 as *const libc::c_char,
        );
    }
    if FS_FilenameCompare((*fs_gamedirvar).string, gameName) as u64 == 0 {
        // This is the standard base game. Servers and clients should
        // use "" and not the standard basegame name because this messes
        // up pak file negotiation and lots of other stuff
        crate::src::qcommon::cvar::Cvar_ForceReset(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        );
    }
    if FS_InvalidGameDir(gameName) as u64 != 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Invalid com_basegame \'%s\'\x00" as *const u8 as *const libc::c_char,
            gameName,
        );
    }
    if FS_InvalidGameDir((*fs_basegame).string) as u64 != 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Invalid fs_basegame \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*fs_basegame).string,
        );
    }
    if FS_InvalidGameDir((*fs_gamedirvar).string) as u64 != 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Invalid fs_game \'%s\'\x00" as *const u8 as *const libc::c_char,
            (*fs_gamedirvar).string,
        );
    }
    // add search path elements in reverse priority order
    fs_gogpath = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_gogpath\x00" as *const u8 as *const libc::c_char,
        crate::src::sys::sys_unix::Sys_GogPath(),
        0x10 as i32 | 0x2000 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if *(*fs_gogpath).string.offset(0 as i32 as isize) != 0 {
        FS_AddGameDirectory((*fs_gogpath).string, gameName);
    }
    fs_steampath = crate::src::qcommon::cvar::Cvar_Get(
        b"fs_steampath\x00" as *const u8 as *const libc::c_char,
        crate::src::sys::sys_unix::Sys_SteamPath(),
        0x10 as i32 | 0x2000 as i32,
    ) as *mut crate::src::qcommon::q_shared::cvar_s;
    if *(*fs_steampath).string.offset(0 as i32 as isize) != 0 {
        FS_AddGameDirectory((*fs_steampath).string, gameName);
    }
    if *(*fs_basepath).string.offset(0 as i32 as isize) != 0 {
        FS_AddGameDirectory((*fs_basepath).string, gameName);
    }
    // fs_homepath is somewhat particular to *nix systems, only add if relevant
    // NOTE: same filtering below for mods and basegame
    if *(*fs_homepath).string.offset(0 as i32 as isize) as i32 != 0
        && crate::src::qcommon::q_shared::Q_stricmp((*fs_homepath).string, (*fs_basepath).string)
            != 0
    {
        FS_CreatePath((*fs_homepath).string);
        FS_AddGameDirectory((*fs_homepath).string, gameName);
    }
    // check for additional base game so mods can be based upon other mods
    if *(*fs_basegame).string.offset(0 as i32 as isize) as i32 != 0
        && crate::src::qcommon::q_shared::Q_stricmp((*fs_basegame).string, gameName) != 0
    {
        if *(*fs_gogpath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_gogpath).string, (*fs_basegame).string);
        }
        if *(*fs_steampath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_steampath).string, (*fs_basegame).string);
        }
        if *(*fs_basepath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_basepath).string, (*fs_basegame).string);
        }
        if *(*fs_homepath).string.offset(0 as i32 as isize) as i32 != 0
            && crate::src::qcommon::q_shared::Q_stricmp(
                (*fs_homepath).string,
                (*fs_basepath).string,
            ) != 0
        {
            FS_AddGameDirectory((*fs_homepath).string, (*fs_basegame).string);
        }
    }
    // check for additional game folder for mods
    if *(*fs_gamedirvar).string.offset(0 as i32 as isize) as i32 != 0
        && crate::src::qcommon::q_shared::Q_stricmp((*fs_gamedirvar).string, gameName) != 0
    {
        if *(*fs_gogpath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_gogpath).string, (*fs_gamedirvar).string);
        }
        if *(*fs_steampath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_steampath).string, (*fs_gamedirvar).string);
        }
        if *(*fs_basepath).string.offset(0 as i32 as isize) != 0 {
            FS_AddGameDirectory((*fs_basepath).string, (*fs_gamedirvar).string);
        }
        if *(*fs_homepath).string.offset(0 as i32 as isize) as i32 != 0
            && crate::src::qcommon::q_shared::Q_stricmp(
                (*fs_homepath).string,
                (*fs_basepath).string,
            ) != 0
        {
            FS_AddGameDirectory((*fs_homepath).string, (*fs_gamedirvar).string);
        }
    }
    if (*crate::src::qcommon::common::com_standalone).integer == 0 {
        Com_ReadCDKey(b"baseq3\x00" as *const u8 as *const libc::c_char);
        if *(*fs_gamedirvar).string.offset(0 as i32 as isize) != 0 {
            Com_AppendCDKey((*fs_gamedirvar).string);
        }
    }
    // add our commands
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"path\x00" as *const u8 as *const libc::c_char,
        Some(FS_Path_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"dir\x00" as *const u8 as *const libc::c_char,
        Some(FS_Dir_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"fdir\x00" as *const u8 as *const libc::c_char,
        Some(FS_NewDir_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"touchFile\x00" as *const u8 as *const libc::c_char,
        Some(FS_TouchFile_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"which\x00" as *const u8 as *const libc::c_char,
        Some(FS_Which_f as unsafe extern "C" fn() -> ()),
    );
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=506
    // reorder the pure pk3 files according to server order
    FS_ReorderPurePaks();
    // print the current search paths
    FS_Path_f(); // We just loaded, it's not modified
    (*fs_gamedirvar).modified = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Com_Printf(
        b"----------------------\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_Printf(
        b"%d files in pk3 files\n\x00" as *const u8 as *const libc::c_char,
        fs_packFiles,
    );
}
/*
===================
FS_CheckPak0

Check whether any of the original id pak files is present,
and start up in standalone mode, if there are none and a
different com_basegame was set.
Note: If you're building a game that doesn't depend on the
Q3 media pak0.pk3, you'll want to remove this by defining
STANDALONE in q_shared.h
===================
*/

unsafe extern "C" fn FS_CheckPak0() {
    let mut path: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut curpack: *mut pack_t = 0 as *mut pack_t;
    let mut pakBasename: *const libc::c_char = 0 as *const libc::c_char;
    let mut founddemo: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut foundPak: u32 = 0 as i32 as u32;
    let mut foundTA: u32 = 0 as i32 as u32;
    path = fs_searchpaths;
    while !path.is_null() {
        if !(*path).pack.is_null() {
            curpack = (*path).pack;
            pakBasename = (*curpack).pakBasename.as_mut_ptr();
            if crate::src::qcommon::q_shared::Q_stricmpn(
                (*curpack).pakGamename.as_mut_ptr(),
                b"demoq3\x00" as *const u8 as *const libc::c_char,
                4096 as i32,
            ) == 0
                && crate::src::qcommon::q_shared::Q_stricmpn(
                    pakBasename,
                    b"pak0\x00" as *const u8 as *const libc::c_char,
                    4096 as i32,
                ) == 0
            {
                if (*curpack).checksum as u32 == 2985612116 as u32 {
                    founddemo = crate::src::qcommon::q_shared::qtrue
                }
            } else if crate::src::qcommon::q_shared::Q_stricmpn(
                (*curpack).pakGamename.as_mut_ptr(),
                b"baseq3\x00" as *const u8 as *const libc::c_char,
                4096 as i32,
            ) == 0
                && crate::stdlib::strlen(pakBasename) == 4 as i32 as libc::c_ulong
                && crate::src::qcommon::q_shared::Q_stricmpn(
                    pakBasename,
                    b"pak\x00" as *const u8 as *const libc::c_char,
                    3 as i32,
                ) == 0
                && *pakBasename.offset(3 as i32 as isize) as i32 >= '0' as i32
                && *pakBasename.offset(3 as i32 as isize) as i32 <= '0' as i32 + 9 as i32 - 1 as i32
            {
                if (*curpack).checksum as u32
                    != pak_checksums
                        [(*pakBasename.offset(3 as i32 as isize) as i32 - '0' as i32) as usize]
                {
                    if *pakBasename.offset(3 as i32 as isize) as i32 == '0' as i32 {
                        crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: baseq3/pak0.pk3 is present but its checksum (%u)\nis not correct. Please re-copy pak0.pk3 from your\nlegitimate Q3 CDROM.\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).checksum);
                    } else {
                        crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: baseq3/pak%d.pk3 is present but its checksum (%u)\nis not correct. Please re-install the point release\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   *pakBasename.offset(3 as i32 as
                                                           isize) as
                                       i32 - '0' as i32,
                                   (*curpack).checksum);
                    }
                }
                foundPak |= ((1 as i32)
                    << *pakBasename.offset(3 as i32 as isize) as i32 - '0' as i32)
                    as u32
            } else if crate::src::qcommon::q_shared::Q_stricmpn(
                (*curpack).pakGamename.as_mut_ptr(),
                b"missionpack\x00" as *const u8 as *const libc::c_char,
                4096 as i32,
            ) == 0
                && crate::stdlib::strlen(pakBasename) == 4 as i32 as libc::c_ulong
                && crate::src::qcommon::q_shared::Q_stricmpn(
                    pakBasename,
                    b"pak\x00" as *const u8 as *const libc::c_char,
                    3 as i32,
                ) == 0
                && *pakBasename.offset(3 as i32 as isize) as i32 >= '0' as i32
                && *pakBasename.offset(3 as i32 as isize) as i32 <= '0' as i32 + 4 as i32 - 1 as i32
            {
                if (*curpack).checksum as u32
                    != missionpak_checksums
                        [(*pakBasename.offset(3 as i32 as isize) as i32 - '0' as i32) as usize]
                {
                    crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: missionpack/pak%d.pk3 is present but its checksum (%u)\nis not correct. Please re-install Team Arena\n**************************************************\n\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               *pakBasename.offset(3 as i32 as isize)
                                   as i32 - '0' as i32,
                               (*curpack).checksum);
                }
                foundTA |= ((1 as i32)
                    << *pakBasename.offset(3 as i32 as isize) as i32 - '0' as i32)
                    as u32
            } else {
                let mut index: i32 = 0;
                // Finally check whether this pak's checksum is listed because the user tried
                // to trick us by renaming the file, and set foundPak's highest bit to indicate this case.
                index = 0 as i32;
                while (index as libc::c_ulong)
                    < (::std::mem::size_of::<[u32; 9]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<u32>() as libc::c_ulong)
                {
                    if (*curpack).checksum as u32 == pak_checksums[index as usize] {
                        crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: %s is renamed pak file %s%cpak%d.pk3\nRunning in standalone mode won\'t work\nPlease rename, or remove this file\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).pakFilename.as_mut_ptr(),
                                   b"baseq3\x00" as *const u8 as
                                       *const libc::c_char, '/' as i32,
                                   index);
                        foundPak |= 0x80000000 as u32
                    }
                    index += 1
                }
                index = 0 as i32;
                while (index as libc::c_ulong)
                    < (::std::mem::size_of::<[u32; 4]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<u32>() as libc::c_ulong)
                {
                    if (*curpack).checksum as u32 == missionpak_checksums[index as usize] {
                        crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: %s is renamed pak file %s%cpak%d.pk3\nRunning in standalone mode won\'t work\nPlease rename, or remove this file\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).pakFilename.as_mut_ptr(),
                                   b"missionpack\x00" as *const u8 as
                                       *const libc::c_char, '/' as i32,
                                   index);
                        foundTA |= 0x80000000 as u32
                    }
                    index += 1
                }
            }
        }
        path = (*path).next
    }
    if foundPak == 0
        && foundTA == 0
        && crate::src::qcommon::q_shared::Q_stricmp(
            (*crate::src::qcommon::common::com_basegame).string,
            b"baseq3\x00" as *const u8 as *const libc::c_char,
        ) != 0
    {
        crate::src::qcommon::cvar::Cvar_Set(
            b"com_standalone\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        crate::src::qcommon::cvar::Cvar_Set(
            b"com_standalone\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*crate::src::qcommon::common::com_standalone).integer == 0 {
        if foundPak & 0x1 as i32 as u32 == 0 {
            if founddemo as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(b"\n\n**************************************************\nWARNING: It looks like you\'re using pak0.pk3\nfrom the demo. This may work fine, but it is not\nguaranteed or supported.\n**************************************************\n\n\n\x00"
                               as *const u8 as *const libc::c_char);
                foundPak |= 0x1 as i32 as u32
            }
        }
    }
    if (*crate::src::qcommon::common::com_standalone).integer == 0
        && foundPak & 0x1ff as i32 as u32 != 0x1ff as i32 as u32
    {
        let mut errorText: [libc::c_char; 1024] =
            *::std::mem::transmute::<&[u8; 1024],
                                     &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        if foundPak & 0x1 as i32 as u32 != 0x1 as i32 as u32 {
            crate::src::qcommon::q_shared::Q_strcat(
                errorText.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                b"\"pak0.pk3\" is missing. Please copy it from your legitimate Q3 CDROM. \x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        if foundPak & 0x1fe as i32 as u32 != 0x1fe as i32 as u32 {
            crate::src::qcommon::q_shared::Q_strcat(
                errorText.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
                b"Point Release files are missing. Please re-install the 1.32 point release. \x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        crate::src::qcommon::q_shared::Q_strcat(errorText.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as i32,
                 crate::src::qcommon::q_shared::va(b"Also check that your ioq3 executable is in the correct place and that every file in the \"%s\" directory is present and readable\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                    b"baseq3\x00" as *const u8 as *const libc::c_char));
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            errorText.as_mut_ptr(),
        );
    }
    if (*crate::src::qcommon::common::com_standalone).integer == 0
        && foundTA != 0
        && foundTA & 0xf as i32 as u32 != 0xf as i32 as u32
    {
        let mut errorText_0: [libc::c_char; 1024] =
            *::std::mem::transmute::<&[u8; 1024],
                                     &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        if foundTA & 0x1 as i32 as u32 != 0x1 as i32 as u32 {
            crate::src::qcommon::q_shared::Com_sprintf(errorText_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as i32,
                        b"\"missionpack%cpak0.pk3\" is missing. Please copy it from your legitimate Quake 3 Team Arena CDROM. \x00"
                            as *const u8 as *const libc::c_char, '/' as i32);
        }
        if foundTA & 0xe as i32 as u32 != 0xe as i32 as u32 {
            crate::src::qcommon::q_shared::Q_strcat(errorText_0.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as i32,
                     b"Team Arena Point Release files are missing. Please re-install the latest Team Arena point release.\x00"
                         as *const u8 as *const libc::c_char);
        }
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"%s\x00" as *const u8 as *const libc::c_char,
            errorText_0.as_mut_ptr(),
        );
    };
}
/*
=====================
FS_LoadedPakChecksums

Returns a space separated string containing the checksums of all loaded pk3 files.
Servers with sv_pure set will get this string and pass it to clients.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_LoadedPakChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            crate::src::qcommon::q_shared::Q_strcat(
                info.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                crate::src::qcommon::q_shared::va(
                    b"%i \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*(*search).pack).checksum,
                ),
            );
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
/*
=====================
FS_LoadedPakNames

Returns a space separated string containing the names of all loaded pk3 files.
Servers with sv_pure set will get this string and pass it to clients.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_LoadedPakNames() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            if *info.as_mut_ptr() != 0 {
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            crate::src::qcommon::q_shared::Q_strcat(
                info.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                (*(*search).pack).pakBasename.as_mut_ptr(),
            );
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
/*
=====================
FS_LoadedPakPureChecksums

Returns a space separated string containing the pure checksums of all loaded pk3 files.
Servers with sv_pure use these checksums to compare with the checksums the clients send
back to the server.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_LoadedPakPureChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            crate::src::qcommon::q_shared::Q_strcat(
                info.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                crate::src::qcommon::q_shared::va(
                    b"%i \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*(*search).pack).pure_checksum,
                ),
            );
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
/*
=====================
FS_ReferencedPakChecksums

Returns a space separated string containing the checksums of all referenced pk3 files.
The server will send this to the clients so they can check which files should be auto-downloaded.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ReferencedPakChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            if (*(*search).pack).referenced != 0
                || crate::src::qcommon::q_shared::Q_stricmpn(
                    (*(*search).pack).pakGamename.as_mut_ptr(),
                    (*crate::src::qcommon::common::com_basegame).string,
                    crate::stdlib::strlen((*crate::src::qcommon::common::com_basegame).string)
                        as i32,
                ) != 0
            {
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    crate::src::qcommon::q_shared::va(
                        b"%i \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*(*search).pack).checksum,
                    ),
                );
            }
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
/*
=====================
FS_ReferencedPakPureChecksums

Returns a space separated string containing the pure checksums of all referenced pk3 files.
Servers with sv_pure set will get this string back from clients for pure validation

The string has a specific order, "cgame ui @ ref1 ref2 ref3 ..."
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ReferencedPakPureChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut nFlags: i32 = 0;
    let mut numPaks: i32 = 0;
    let mut checksum: i32 = 0;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    checksum = fs_checksumFeed;
    numPaks = 0 as i32;
    nFlags = 0x4 as i32;
    while nFlags != 0 {
        if nFlags & 0x1 as i32 != 0 {
            // add a delimter between must haves and general refs
            //Q_strcat(info, sizeof(info), "@ ");
            info[crate::stdlib::strlen(info.as_mut_ptr()).wrapping_add(1 as i32 as libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_char;
            info[crate::stdlib::strlen(info.as_mut_ptr()).wrapping_add(2 as i32 as libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_char;
            info[crate::stdlib::strlen(info.as_mut_ptr()) as usize] = '@' as i32 as libc::c_char;
            info[crate::stdlib::strlen(info.as_mut_ptr()) as usize] = ' ' as i32 as libc::c_char
        }
        search = fs_searchpaths;
        while !search.is_null() {
            // is the element a pak file and has it been referenced based on flag?
            if !(*search).pack.is_null() && (*(*search).pack).referenced & nFlags != 0 {
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    crate::src::qcommon::q_shared::va(
                        b"%i \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*(*search).pack).pure_checksum,
                    ),
                );
                if nFlags & (0x4 as i32 | 0x2 as i32) != 0 {
                    break;
                }
                checksum ^= (*(*search).pack).pure_checksum;
                numPaks += 1
            }
            search = (*search).next
        }
        nFlags = nFlags >> 1 as i32
    }
    // last checksum is the encoded number of referenced pk3s
    checksum ^= numPaks;
    crate::src::qcommon::q_shared::Q_strcat(
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
        crate::src::qcommon::q_shared::va(
            b"%i \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            checksum,
        ),
    );
    return info.as_mut_ptr();
}
/*
=====================
FS_ReferencedPakNames

Returns a space separated string containing the names of all referenced pk3 files.
The server will send this to the clients so they can check which files should be auto-downloaded.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ReferencedPakNames() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0 as i32 as usize] = 0 as i32 as libc::c_char;
    // we want to return ALL pk3's from the fs_game path
    // and referenced one's from baseq3
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            if (*(*search).pack).referenced != 0
                || crate::src::qcommon::q_shared::Q_stricmpn(
                    (*(*search).pack).pakGamename.as_mut_ptr(),
                    (*crate::src::qcommon::common::com_basegame).string,
                    crate::stdlib::strlen((*crate::src::qcommon::common::com_basegame).string)
                        as i32,
                ) != 0
            {
                if *info.as_mut_ptr() != 0 {
                    crate::src::qcommon::q_shared::Q_strcat(
                        info.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                        b" \x00" as *const u8 as *const libc::c_char,
                    );
                }
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    (*(*search).pack).pakGamename.as_mut_ptr(),
                );
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    b"/\x00" as *const u8 as *const libc::c_char,
                );
                crate::src::qcommon::q_shared::Q_strcat(
                    info.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
                    (*(*search).pack).pakBasename.as_mut_ptr(),
                );
            }
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
/*
=====================
FS_ClearPakReferences
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ClearPakReferences(mut flags: i32) {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    if flags == 0 {
        flags = -(1 as i32)
    }
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file and has it been referenced?
        if !(*search).pack.is_null() {
            (*(*search).pack).referenced &= !flags
        }
        search = (*search).next
    }
}
/*
=====================
FS_PureServerSetLoadedPaks

If the string is empty, all data sources will be allowed.
If not empty, only pk3 files that match one of the space
separated checksums will be checked for files, with the
exception of .cfg and .dat files.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_PureServerSetLoadedPaks(
    mut pakSums: *const libc::c_char,
    mut pakNames: *const libc::c_char,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    crate::src::qcommon::cmd::Cmd_TokenizeString(pakSums);
    c = crate::src::qcommon::cmd::Cmd_Argc();
    if c > 4096 as i32 {
        c = 4096 as i32
    }
    fs_numServerPaks = c;
    i = 0 as i32;
    while i < c {
        fs_serverPaks[i as usize] = atoi(crate::src::qcommon::cmd::Cmd_Argv(i));
        i += 1
    }
    if fs_numServerPaks != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"Connected to a pure server.\n\x00" as *const u8 as *const libc::c_char,
        );
    } else if fs_reordered as u64 != 0 {
        // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=540
        // force a restart to make sure the search order will be correct
        crate::src::qcommon::common::Com_DPrintf(
            b"FS search reorder is required\n\x00" as *const u8 as *const libc::c_char,
        );
        FS_Restart(fs_checksumFeed);
        return;
    }
    i = 0 as i32;
    while i < c {
        if !fs_serverPakNames[i as usize].is_null() {
            crate::src::qcommon::common::Z_Free(fs_serverPakNames[i as usize] as *mut libc::c_void);
        }
        fs_serverPakNames[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    if !pakNames.is_null() && *pakNames as i32 != 0 {
        crate::src::qcommon::cmd::Cmd_TokenizeString(pakNames);
        d = crate::src::qcommon::cmd::Cmd_Argc();
        if d > 4096 as i32 {
            d = 4096 as i32
        }
        i = 0 as i32;
        while i < d {
            fs_serverPakNames[i as usize] =
                crate::src::qcommon::common::CopyString(crate::src::qcommon::cmd::Cmd_Argv(i));
            i += 1
        }
    };
}
/*
=====================
FS_PureServerSetReferencedPaks

The checksums and names of the pk3 files referenced at the server
are sent to the client and stored here. The client will use these
checksums to see if any pk3 files need to be auto-downloaded.
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_PureServerSetReferencedPaks(
    mut pakSums: *const libc::c_char,
    mut pakNames: *const libc::c_char,
) {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0 as i32;
    crate::src::qcommon::cmd::Cmd_TokenizeString(pakSums);
    c = crate::src::qcommon::cmd::Cmd_Argc();
    if c > 4096 as i32 {
        c = 4096 as i32
    }
    i = 0 as i32;
    while i < c {
        fs_serverReferencedPaks[i as usize] = atoi(crate::src::qcommon::cmd::Cmd_Argv(i));
        i += 1
    }
    i = 0 as i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if !fs_serverReferencedPakNames[i as usize].is_null() {
            crate::src::qcommon::common::Z_Free(
                fs_serverReferencedPakNames[i as usize] as *mut libc::c_void,
            );
        }
        fs_serverReferencedPakNames[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    if !pakNames.is_null() && *pakNames as i32 != 0 {
        crate::src::qcommon::cmd::Cmd_TokenizeString(pakNames);
        d = crate::src::qcommon::cmd::Cmd_Argc();
        if d > c {
            d = c
        }
        i = 0 as i32;
        while i < d {
            fs_serverReferencedPakNames[i as usize] =
                crate::src::qcommon::common::CopyString(crate::src::qcommon::cmd::Cmd_Argv(i));
            i += 1
        }
    }
    // ensure that there are as many checksums as there are pak names.
    if d < c {
        c = d
    }
    fs_numServerReferencedPaks = c;
}
/*
================
FS_InitFilesystem

Called only at initial startup, not when the filesystem
is resetting due to a game change
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_InitFilesystem() {
    // allow command line parms to override our defaults
    // we have to specially handle this, because normal command
    // line variable sets don't happen until after the filesystem
    // has already been initialized
    crate::src::qcommon::common::Com_StartupVariable(
        b"fs_basepath\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_StartupVariable(
        b"fs_homepath\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_StartupVariable(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
    );
    if FS_FilenameCompare(
        crate::src::qcommon::cvar::Cvar_VariableString(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
        ),
        (*crate::src::qcommon::common::com_basegame).string,
    ) as u64
        == 0
    {
        crate::src::qcommon::cvar::Cvar_Set(
            b"fs_game\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    // try to start up normally
    FS_Startup((*crate::src::qcommon::common::com_basegame).string);
    FS_CheckPak0();
    // if we can't find default.cfg, assume that the paths are
    // busted and error out now, rather than getting an unreadable
    // graphics screen when the font fails to load
    if FS_ReadFile(
        b"default.cfg\x00" as *const u8 as *const libc::c_char,
        0 as *mut *mut libc::c_void,
    ) <= 0 as i32 as libc::c_long
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Couldn\'t load default.cfg\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidBase.as_mut_ptr(),
        (*fs_basepath).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidComBaseGame.as_mut_ptr(),
        (*crate::src::qcommon::common::com_basegame).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidFsBaseGame.as_mut_ptr(),
        (*fs_basegame).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidGame.as_mut_ptr(),
        (*fs_gamedirvar).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
}
/*
================
FS_Restart
================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_Restart(mut checksumFeed: i32) {
    let mut lastGameDir: *const libc::c_char = 0 as *const libc::c_char;
    // free anything we currently have loaded
    FS_Shutdown(crate::src::qcommon::q_shared::qfalse);
    // set the checksum feed
    fs_checksumFeed = checksumFeed;
    // clear pak references
    FS_ClearPakReferences(0 as i32);
    // try to start up normally
    FS_Startup((*crate::src::qcommon::common::com_basegame).string);
    FS_CheckPak0();
    // if we can't find default.cfg, assume that the paths are
    // busted and error out now, rather than getting an unreadable
    // graphics screen when the font fails to load
    if FS_ReadFile(
        b"default.cfg\x00" as *const u8 as *const libc::c_char,
        0 as *mut *mut libc::c_void,
    ) <= 0 as i32 as libc::c_long
    {
        // this might happen when connecting to a pure server not using BASEGAME/pak0.pk3
        // (for instance a TA demo server)
        if lastValidBase[0 as i32 as usize] != 0 {
            FS_PureServerSetLoadedPaks(
                b"\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"fs_basepath\x00" as *const u8 as *const libc::c_char,
                lastValidBase.as_mut_ptr(),
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"com_basegame\x00" as *const u8 as *const libc::c_char,
                lastValidComBaseGame.as_mut_ptr(),
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"fs_basegame\x00" as *const u8 as *const libc::c_char,
                lastValidFsBaseGame.as_mut_ptr(),
            );
            crate::src::qcommon::cvar::Cvar_Set(
                b"fs_game\x00" as *const u8 as *const libc::c_char,
                lastValidGame.as_mut_ptr(),
            );
            lastValidBase[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            lastValidComBaseGame[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            lastValidFsBaseGame[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            lastValidGame[0 as i32 as usize] = '\u{0}' as i32 as libc::c_char;
            FS_Restart(checksumFeed);
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"Invalid game folder\x00" as *const u8 as *const libc::c_char,
            );
        }
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Couldn\'t load default.cfg\x00" as *const u8 as *const libc::c_char,
        );
    }
    lastGameDir = if lastValidGame[0 as i32 as usize] as i32 != 0 {
        lastValidGame.as_mut_ptr()
    } else {
        lastValidComBaseGame.as_mut_ptr()
    };
    if crate::src::qcommon::q_shared::Q_stricmp(FS_GetCurrentGameDir(), lastGameDir) != 0 {
        crate::src::sys::sys_main::Sys_RemovePIDFile(lastGameDir);
        crate::src::sys::sys_main::Sys_InitPIDFile(FS_GetCurrentGameDir());
        // skip the q3config.cfg if "safe" is on the command line
        if crate::src::qcommon::common::Com_SafeMode() as u64 == 0 {
            crate::src::qcommon::cmd::Cbuf_AddText(
                b"exec q3config.cfg\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidBase.as_mut_ptr(),
        (*fs_basepath).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidComBaseGame.as_mut_ptr(),
        (*crate::src::qcommon::common::com_basegame).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidFsBaseGame.as_mut_ptr(),
        (*fs_basegame).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        lastValidGame.as_mut_ptr(),
        (*fs_gamedirvar).string,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as i32,
    );
}
/*
=================
FS_ConditionalRestart

Restart if necessary
Return qtrue if restarting due to game directory changed, qfalse otherwise
=================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_ConditionalRestart(
    mut checksumFeed: i32,
    mut disconnect: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*fs_gamedirvar).modified as u64 != 0 {
        if FS_FilenameCompare(lastValidGame.as_mut_ptr(), (*fs_gamedirvar).string) as u32 != 0
            && (*lastValidGame.as_mut_ptr() as i32 != 0
                || FS_FilenameCompare(
                    (*fs_gamedirvar).string,
                    (*crate::src::qcommon::common::com_basegame).string,
                ) as u32
                    != 0)
            && (*(*fs_gamedirvar).string as i32 != 0
                || FS_FilenameCompare(
                    lastValidGame.as_mut_ptr(),
                    (*crate::src::qcommon::common::com_basegame).string,
                ) as u32
                    != 0)
        {
            crate::src::qcommon::common::Com_GameRestart(checksumFeed, disconnect);
            return crate::src::qcommon::q_shared::qtrue;
        } else {
            (*fs_gamedirvar).modified = crate::src::qcommon::q_shared::qfalse
        }
    }
    if checksumFeed != fs_checksumFeed {
        FS_Restart(checksumFeed);
    } else if fs_numServerPaks != 0 && fs_reordered as u64 == 0 {
        FS_ReorderPurePaks();
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
========================================================================================

Handle based file calls for virtual machines

========================================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn FS_FOpenFileByMode(
    mut qpath: *const libc::c_char,
    mut f: *mut crate::src::qcommon::q_shared::fileHandle_t,
    mut mode: crate::src::qcommon::q_shared::fsMode_t,
) -> i32 {
    let mut r: i32 = 0;
    let mut sync: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    sync = crate::src::qcommon::q_shared::qfalse;
    let mut current_block_14: u64;
    match mode as u32 {
        0 => {
            r = FS_FOpenFileRead(qpath, f, crate::src::qcommon::q_shared::qtrue) as i32;
            current_block_14 = 17833034027772472439;
        }
        1 => {
            *f = FS_FOpenFileWrite(qpath);
            r = 0 as i32;
            if *f == 0 as i32 {
                r = -(1 as i32)
            }
            current_block_14 = 17833034027772472439;
        }
        3 => {
            sync = crate::src::qcommon::q_shared::qtrue;
            current_block_14 = 4670568916356103642;
        }
        2 => {
            current_block_14 = 4670568916356103642;
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"FS_FOpenFileByMode: bad mode\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    match current_block_14 {
        4670568916356103642 => {
            *f = FS_FOpenFileAppend(qpath);
            r = 0 as i32;
            if *f == 0 as i32 {
                r = -(1 as i32)
            }
        }
        _ => {}
    }
    if f.is_null() {
        return r;
    }
    if *f != 0 {
        fsh[*f as usize].fileSize = r
    }
    fsh[*f as usize].handleSync = sync;
    return r;
}
#[no_mangle]

pub unsafe extern "C" fn FS_FTell(mut f: crate::src::qcommon::q_shared::fileHandle_t) -> i32 {
    let mut pos: i32 = 0;
    if fsh[f as usize].zipFile as u32 == crate::src::qcommon::q_shared::qtrue as i32 as u32 {
        pos = crate::src::qcommon::unzip::unztell(fsh[f as usize].handleFiles.file.z) as i32
    } else {
        pos = crate::stdlib::ftell(fsh[f as usize].handleFiles.file.o) as i32
    }
    return pos;
}
#[no_mangle]

pub unsafe extern "C" fn FS_Flush(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    crate::stdlib::fflush(fsh[f as usize].handleFiles.file.o);
}
#[no_mangle]

pub unsafe extern "C" fn FS_FilenameCompletion(
    mut dir: *const libc::c_char,
    mut ext: *const libc::c_char,
    mut stripExt: crate::src::qcommon::q_shared::qboolean,
    mut callback: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
    mut allowNonPureFilesOnDisk: crate::src::qcommon::q_shared::qboolean,
) {
    let mut filenames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nfiles: i32 = 0;
    let mut i: i32 = 0;
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    filenames = FS_ListFilteredFiles(
        dir,
        ext,
        0 as *mut libc::c_char,
        &mut nfiles,
        allowNonPureFilesOnDisk,
    );
    FS_SortFileList(filenames, nfiles);
    i = 0 as i32;
    while i < nfiles {
        FS_ConvertPath(*filenames.offset(i as isize));
        crate::src::qcommon::q_shared::Q_strncpyz(
            filename.as_mut_ptr(),
            *filenames.offset(i as isize),
            1024 as i32,
        );
        if stripExt as u64 != 0 {
            crate::src::qcommon::q_shared::COM_StripExtension(
                filename.as_mut_ptr(),
                filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as i32,
            );
        }
        callback.expect("non-null function pointer")(filename.as_mut_ptr());
        i += 1
    }
    FS_FreeFileList(filenames);
}
#[no_mangle]

pub unsafe extern "C" fn FS_GetCurrentGameDir() -> *const libc::c_char {
    if *(*fs_gamedirvar).string.offset(0 as i32 as isize) != 0 {
        return (*fs_gamedirvar).string;
    }
    return (*crate::src::qcommon::common::com_basegame).string;
}

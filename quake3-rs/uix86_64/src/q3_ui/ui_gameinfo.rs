use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::Com_Printf;
pub use crate::src::q3_ui::ui_gameinfo::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_ReInit;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Register;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_FS_FCloseFile;
pub use crate::src::ui::ui_syscalls::trap_FS_FOpenFile;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_FS_Read;
pub use crate::src::ui::ui_syscalls::trap_Print;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_local_h::AWARD_ACCURACY;
pub use crate::ui_local_h::AWARD_EXCELLENT;
pub use crate::ui_local_h::AWARD_FRAGS;
pub use crate::ui_local_h::AWARD_GAUNTLET;
pub use crate::ui_local_h::AWARD_IMPRESSIVE;
pub use crate::ui_local_h::AWARD_PERFECT;

pub use ::libc::strtol;
#[no_mangle]

pub static mut ui_numBots: libc::c_int = 0;

static mut ui_botInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];

static mut ui_numArenas: libc::c_int = 0;

static mut ui_arenaInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];

static mut ui_numSinglePlayerArenas: libc::c_int = 0;

static mut ui_numSpecialSinglePlayerArenas: libc::c_int = 0;

static mut memoryPool: [libc::c_char; 131072] = [0; 131072];

static mut allocPoint: libc::c_int = 0;

static mut outOfMemory: libc::c_int = 0;
/*
===============
UI_Alloc
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Alloc(mut size: libc::c_int) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if allocPoint + size > 128 as libc::c_int * 1024 as libc::c_int {
        outOfMemory = crate::src::qcommon::q_shared::qtrue as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    p = &mut *memoryPool.as_mut_ptr().offset(allocPoint as isize) as *mut libc::c_char;
    allocPoint += size + 31 as libc::c_int & !(31 as libc::c_int);
    return p as *mut libc::c_void;
}
/*
===============
UI_InitMemory
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_InitMemory() {
    allocPoint = 0 as libc::c_int;
    outOfMemory = crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
/*
===============
UI_ParseInfos
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ParseInfos(
    mut buf: *mut libc::c_char,
    mut max: libc::c_int,
    mut infos: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    count = 0 as libc::c_int;
    loop {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut buf);
        if *token.offset(0 as libc::c_int as isize) == 0 {
            break;
        }
        if ::libc::strcmp(token, b"{\x00" as *const u8 as *const libc::c_char) != 0 {
            crate::src::q3_ui::ui_atoms::Com_Printf(
                b"Missing { in info file\n\x00" as *const u8 as *const libc::c_char,
            );
            break;
        } else if count == max {
            crate::src::q3_ui::ui_atoms::Com_Printf(
                b"Max infos exceeded\n\x00" as *const u8 as *const libc::c_char,
            );
            break;
        } else {
            info[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            loop {
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    &mut buf,
                    crate::src::qcommon::q_shared::qtrue,
                );
                if *token.offset(0 as libc::c_int as isize) == 0 {
                    crate::src::q3_ui::ui_atoms::Com_Printf(
                        b"Unexpected end of info file\n\x00" as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    if ::libc::strcmp(token, b"}\x00" as *const u8 as *const libc::c_char) == 0 {
                        break;
                    }
                    crate::src::qcommon::q_shared::Q_strncpyz(
                        key.as_mut_ptr(),
                        token,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    token = crate::src::qcommon::q_shared::COM_ParseExt(
                        &mut buf,
                        crate::src::qcommon::q_shared::qfalse,
                    );
                    if *token.offset(0 as libc::c_int as isize) == 0 {
                        ::libc::strcpy(token, b"<NULL>\x00" as *const u8 as *const libc::c_char);
                    }
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        info.as_mut_ptr(),
                        key.as_mut_ptr(),
                        token,
                    );
                }
            }
            //NOTE: extra space for arena number
            let ref mut fresh0 = *infos.offset(count as isize);
            *fresh0 = UI_Alloc(
                crate::stdlib::strlen(info.as_mut_ptr())
                    .wrapping_add(crate::stdlib::strlen(
                        b"\\num\\\x00" as *const u8 as *const libc::c_char,
                    ))
                    .wrapping_add(crate::stdlib::strlen(crate::src::qcommon::q_shared::va(
                        b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1024 as libc::c_int,
                    )))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_char;
            if !(*infos.offset(count as isize)).is_null() {
                ::libc::strcpy(*infos.offset(count as isize), info.as_mut_ptr());
                count += 1
            }
        }
    }
    return count;
}
/*
===============
UI_LoadArenasFromFile
===============
*/

unsafe extern "C" fn UI_LoadArenasFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = crate::src::ui::ui_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192 as libc::c_int,
        ));
        crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
        return;
    }
    crate::src::ui::ui_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
    ui_numArenas += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024 as libc::c_int - ui_numArenas,
        &mut *ui_arenaInfos.as_mut_ptr().offset(ui_numArenas as isize),
    );
}
/*
===============
UI_LoadArenas
===============
*/

unsafe extern "C" fn UI_LoadArenas() {
    let mut numdirs: libc::c_int = 0;
    let mut arenasFile: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 4096] = [0; 4096];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut singlePlayerNum: libc::c_int = 0;
    let mut specialNum: libc::c_int = 0;
    let mut otherNum: libc::c_int = 0;
    ui_numArenas = 0 as libc::c_int;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        &mut arenasFile as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"g_arenasFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int | 0x40 as libc::c_int,
    );
    if *arenasFile.string.as_mut_ptr() != 0 {
        UI_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        UI_LoadArenasFromFile(
            b"scripts/arenas.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    // get all arenas from .arena files
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".arena\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        4096 as libc::c_int,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as libc::c_int;
        ::libc::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        ::libc::strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1 as libc::c_int) as isize)
    }
    crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i arenas parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ui_numArenas,
    ));
    if outOfMemory != 0 {
        crate::src::ui::ui_syscalls::trap_Print(
            b"^3WARNING: not enough memory in pool to load all arenas\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // set initial numbers
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
        );
        n += 1
    }
    // go through and count single players levels
    ui_numSinglePlayerArenas = 0 as libc::c_int;
    ui_numSpecialSinglePlayerArenas = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        // determine type
        type_0 = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const libc::c_char,
        );
        // if no type specified, it will be treated as "ffa"
        if !(*type_0 == 0) {
            if !::libc::strstr(type_0, b"single\x00" as *const u8 as *const libc::c_char).is_null()
            {
                // check for special single player arenas (training, final)
                tag = crate::src::qcommon::q_shared::Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const libc::c_char,
                );
                if *tag != 0 {
                    ui_numSpecialSinglePlayerArenas += 1
                } else {
                    ui_numSinglePlayerArenas += 1
                }
            }
        }
        n += 1
    }
    n = ui_numSinglePlayerArenas % 4 as libc::c_int;
    if n != 0 as libc::c_int {
        ui_numSinglePlayerArenas -= n;
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"%i arenas ignored to make count divisible by %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            n,
            4 as libc::c_int,
        ));
    }
    // go through once more and assign number to the levels
    singlePlayerNum = 0 as libc::c_int;
    specialNum = singlePlayerNum + ui_numSinglePlayerArenas;
    otherNum = specialNum + ui_numSpecialSinglePlayerArenas;
    let mut current_block_44: u64;
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        // determine type
        type_0 = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const libc::c_char,
        );
        // if no type specified, it will be treated as "ffa"
        if *type_0 != 0 {
            if !::libc::strstr(type_0, b"single\x00" as *const u8 as *const libc::c_char).is_null()
            {
                // check for special single player arenas (training, final)
                tag = crate::src::qcommon::q_shared::Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const libc::c_char,
                );
                if *tag != 0 {
                    let fresh1 = specialNum;
                    specialNum = specialNum + 1;
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const libc::c_char,
                        crate::src::qcommon::q_shared::va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fresh1,
                        ),
                    );
                } else {
                    let fresh2 = singlePlayerNum;
                    singlePlayerNum = singlePlayerNum + 1;
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const libc::c_char,
                        crate::src::qcommon::q_shared::va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fresh2,
                        ),
                    );
                }
                current_block_44 = 14775119014532381840;
            } else {
                current_block_44 = 16415152177862271243;
            }
        } else {
            current_block_44 = 16415152177862271243;
        }
        match current_block_44 {
            16415152177862271243 => {
                let fresh3 = otherNum;
                otherNum = otherNum + 1;
                crate::src::qcommon::q_shared::Info_SetValueForKey(
                    ui_arenaInfos[n as usize],
                    b"num\x00" as *const u8 as *const libc::c_char,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        fresh3,
                    ),
                );
            }
            _ => {}
        }
        n += 1
    }
}
/*
===============
UI_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetArenaInfoByNumber(mut num: libc::c_int) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if num < 0 as libc::c_int || num >= ui_numArenas {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Invalid arena number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *const libc::c_char;
    }
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
        );
        if *value as libc::c_int != 0 && atoi(value) == num {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
/*
===============
UI_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetArenaInfoByMap(mut map: *const libc::c_char) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const libc::c_char,
            ),
            map,
        ) == 0 as libc::c_int
        {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
/*
===============
UI_GetSpecialArenaInfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetSpecialArenaInfo(
    mut tag: *const libc::c_char,
) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < ui_numArenas {
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"special\x00" as *const u8 as *const libc::c_char,
            ),
            tag,
        ) == 0 as libc::c_int
        {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
/*
===============
UI_LoadBotsFromFile
===============
*/

unsafe extern "C" fn UI_LoadBotsFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = crate::src::ui::ui_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192 as libc::c_int,
        ));
        crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
        return;
    }
    crate::src::ui::ui_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0 as libc::c_int as libc::c_char;
    crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
    ui_numBots += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024 as libc::c_int - ui_numBots,
        &mut *ui_botInfos.as_mut_ptr().offset(ui_numBots as isize),
    );
    if outOfMemory != 0 {
        crate::src::ui::ui_syscalls::trap_Print(
            b"^3WARNING: not enough memory in pool to load all bots\n\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
/*
===============
UI_LoadBots
===============
*/

unsafe extern "C" fn UI_LoadBots() {
    let mut botsFile: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    let mut numdirs: libc::c_int = 0;
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    ui_numBots = 0 as libc::c_int;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        &mut botsFile as *mut _ as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"g_botsFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10 as libc::c_int | 0x40 as libc::c_int,
    );
    if *botsFile.string.as_mut_ptr() != 0 {
        UI_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        UI_LoadBotsFromFile(
            b"scripts/bots.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    // get all bots from .bot files
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".bot\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024 as libc::c_int,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as libc::c_int;
        ::libc::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        ::libc::strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1 as libc::c_int) as isize)
    }
    crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i bots parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ui_numBots,
    ));
}
/*
===============
UI_GetBotInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBotInfoByNumber(mut num: libc::c_int) -> *mut libc::c_char {
    if num < 0 as libc::c_int || num >= ui_numBots {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *mut libc::c_char;
    }
    return ui_botInfos[num as usize];
}
/*
===============
UI_GetBotInfoByName
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBotInfoByName(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    n = 0 as libc::c_int;
    while n < ui_numBots {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_botInfos[n as usize],
            b"name\x00" as *const u8 as *const libc::c_char,
        );
        if crate::src::qcommon::q_shared::Q_stricmp(value, name) == 0 {
            return ui_botInfos[n as usize];
        }
        n += 1
    }
    return 0 as *mut libc::c_char;
}
//
// single player game info
//
/*
===============
UI_GetBestScore

Returns the player's best finish on a given level, 0 if the have not played the level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBestScore(
    mut level: libc::c_int,
    mut score: *mut libc::c_int,
    mut skill: *mut libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut skillScore: libc::c_int = 0;
    let mut bestScore: libc::c_int = 0;
    let mut bestScoreSkill: libc::c_int = 0;
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    if score.is_null() || skill.is_null() {
        return;
    }
    if level < 0 as libc::c_int || level > ui_numArenas {
        return;
    }
    bestScore = 0 as libc::c_int;
    bestScoreSkill = 0 as libc::c_int;
    n = 1 as libc::c_int;
    while n <= 5 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            crate::src::qcommon::q_shared::va(
                b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
            scores.as_mut_ptr(),
            1024 as libc::c_int,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"l%i\x00" as *const u8 as *const libc::c_char,
            level,
        );
        skillScore = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            scores.as_mut_ptr(),
            arenaKey.as_mut_ptr(),
        ));
        if !(skillScore < 1 as libc::c_int || skillScore > 8 as libc::c_int) {
            if bestScore == 0 || skillScore <= bestScore {
                bestScore = skillScore;
                bestScoreSkill = n
            }
        }
        n += 1
    }
    *score = bestScore;
    *skill = bestScoreSkill;
}
/*
===============
UI_SetBestScore

Set the player's best finish for a level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SetBestScore(mut level: libc::c_int, mut score: libc::c_int) {
    let mut skill: libc::c_int = 0;
    let mut oldScore: libc::c_int = 0;
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    // validate score
    if score < 1 as libc::c_int || score > 8 as libc::c_int {
        return;
    }
    // validate skill
    skill = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"g_spSkill\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if skill < 1 as libc::c_int || skill > 5 as libc::c_int {
        return;
    }
    // get scores
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        crate::src::qcommon::q_shared::va(
            b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill,
        ),
        scores.as_mut_ptr(),
        1024 as libc::c_int,
    );
    // see if this is better
    crate::src::qcommon::q_shared::Com_sprintf(
        arenaKey.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"l%i\x00" as *const u8 as *const libc::c_char,
        level,
    );
    oldScore = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        scores.as_mut_ptr(),
        arenaKey.as_mut_ptr(),
    ));
    if oldScore != 0 && oldScore <= score {
        return;
    }
    // update scores
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        scores.as_mut_ptr(),
        arenaKey.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            score,
        ),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        crate::src::qcommon::q_shared::va(
            b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill,
        ),
        scores.as_mut_ptr(),
    );
}
/*
===============
UI_LogAwardData
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LogAwardData(mut award: libc::c_int, mut data: libc::c_int) {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    let mut oldValue: libc::c_int = 0;
    if data == 0 as libc::c_int {
        return;
    }
    if award > crate::ui_local_h::AWARD_PERFECT as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Bad award %i in UI_LogAwardData\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            award,
        ));
        return;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"a%i\x00" as *const u8 as *const libc::c_char,
        award,
    );
    oldValue = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
    ));
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            oldValue + data,
        ),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
    );
}
/*
===============
UI_GetAwardLevel
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetAwardLevel(mut award: libc::c_int) -> libc::c_int {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"a%i\x00" as *const u8 as *const libc::c_char,
        award,
    );
    return atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
    ));
}
/*
===============
UI_TierCompleted
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TierCompleted(mut levelWon: libc::c_int) -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tier: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut skill: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    tier = levelWon / 4 as libc::c_int;
    level = tier * 4 as libc::c_int;
    if tier == UI_GetNumSPTiers() {
        info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
        if levelWon
            == atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"num\x00" as *const u8 as *const libc::c_char,
            ))
        {
            return 0 as libc::c_int;
        }
        info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
        if info.is_null()
            || levelWon
                == atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"num\x00" as *const u8 as *const libc::c_char,
                ))
        {
            return tier + 1 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    n = 0 as libc::c_int;
    while n < 4 as libc::c_int {
        UI_GetBestScore(level, &mut score, &mut skill);
        if score != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        n += 1;
        level += 1
    }
    return tier + 1 as libc::c_int;
}
/*
===============
UI_ShowTierVideo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ShowTierVideo(
    mut tier: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut videos: [libc::c_char; 1024] = [0; 1024];
    if tier <= 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"tier%i\x00" as *const u8 as *const libc::c_char,
        tier,
    );
    if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
    )) != 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        ),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
UI_CanShowTierVideo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CanShowTierVideo(
    mut tier: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut videos: [libc::c_char; 1024] = [0; 1024];
    if tier == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::q3_ui::ui_atoms::uis.demoversion as libc::c_uint != 0 && tier != 8 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"tier%i\x00" as *const u8 as *const libc::c_char,
        tier,
    );
    if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
    )) != 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
UI_GetCurrentGame

Returns the next level the player has not won
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetCurrentGame() -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut rank: libc::c_int = 0 as libc::c_int;
    let mut skill: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
    if !info.is_null() {
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_GetBestScore(level, &mut rank, &mut skill);
        if rank == 0 || rank > 1 as libc::c_int {
            return level;
        }
    }
    level = 0 as libc::c_int;
    while level < ui_numSinglePlayerArenas {
        UI_GetBestScore(level, &mut rank, &mut skill);
        if rank == 0 || rank > 1 as libc::c_int {
            return level;
        }
        level += 1
    }
    info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
    if info.is_null() {
        return -(1 as libc::c_int);
    }
    return atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"num\x00" as *const u8 as *const libc::c_char,
    ));
}
/*
===============
UI_NewGame

Clears the scores and sets the difficutly level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_NewGame() {
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores2\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores3\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores4\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores5\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===============
UI_GetNumArenas
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumArenas() -> libc::c_int {
    return ui_numArenas;
}
/*
===============
UI_GetNumSPArenas
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumSPArenas() -> libc::c_int {
    return ui_numSinglePlayerArenas;
}
/*
===============
UI_GetNumSPTiers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumSPTiers() -> libc::c_int {
    return ui_numSinglePlayerArenas / 4 as libc::c_int;
}
/*
===============
UI_GetNumBots
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumBots() -> libc::c_int {
    return ui_numBots;
}
/*
===============
UI_SPUnlock_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPUnlock_f() {
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    let mut level: libc::c_int = 0;
    let mut tier: libc::c_int = 0;
    // get scores for skill 1
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        scores.as_mut_ptr(),
        1024 as libc::c_int,
    );
    // update scores
    level = 0 as libc::c_int;
    while level < ui_numSinglePlayerArenas + ui_numSpecialSinglePlayerArenas {
        crate::src::qcommon::q_shared::Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"l%i\x00" as *const u8 as *const libc::c_char,
            level,
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            scores.as_mut_ptr(),
            arenaKey.as_mut_ptr(),
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        level += 1
    }
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        scores.as_mut_ptr(),
    );
    // unlock cinematics
    tier = 1 as libc::c_int;
    while tier <= 8 as libc::c_int {
        UI_ShowTierVideo(tier);
        tier += 1
    }
    crate::src::ui::ui_syscalls::trap_Print(
        b"All levels unlocked at skill level 1\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_ReInit();
}
/*
===============
UI_SPUnlockMedals_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPUnlockMedals_f() {
    let mut n: libc::c_int = 0;
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        1024 as libc::c_int,
    );
    n = 0 as libc::c_int;
    while n < 6 as libc::c_int {
        crate::src::qcommon::q_shared::Com_sprintf(
            key.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"a%i\x00" as *const u8 as *const libc::c_char,
            n,
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            awardData.as_mut_ptr(),
            key.as_mut_ptr(),
            b"100\x00" as *const u8 as *const libc::c_char,
        );
        n += 1
    }
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Print(
        b"All awards unlocked at 100\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
//
// ui_spSkill.c
//
//
// ui_syscalls.c
//
// don't use EXEC_NOW!
// fsOrigin_t
//
// ui_addbots.c
//
//
// ui_removebots.c
//
//
// ui_teamorders.c
//
//
// ui_loadconfig.c
//
//
// ui_saveconfig.c
//
//
// ui_display.c
//
//
// ui_sound.c
//
//
// ui_network.c
//
//
// ui_gameinfo.c
//
/*
===============
UI_InitGameinfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_InitGameinfo() {
    UI_InitMemory();
    UI_LoadArenas();
    UI_LoadBots();
    crate::src::q3_ui::ui_atoms::uis.demoversion = crate::src::qcommon::q_shared::qfalse;
}

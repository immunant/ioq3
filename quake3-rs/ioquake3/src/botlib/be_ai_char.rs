use ::libc;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;

pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::FreeSource;
pub use crate::src::botlib::l_precomp::LoadSourceFile;
pub use crate::src::botlib::l_precomp::PC_ExpectAnyToken;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenType;
pub use crate::src::botlib::l_precomp::PC_ReadToken;
pub use crate::src::botlib::l_precomp::PC_SetBaseFolder;
pub use crate::src::botlib::l_precomp::SourceError;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

//a bot character

pub type bot_character_t = bot_character_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_character_s {
    pub filename: [libc::c_char; 64],
    pub skill: f32,
    pub c: [bot_characteristic_t; 1],
}
//variable sized
//a characteristic

pub type bot_characteristic_t = bot_characteristic_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_characteristic_s {
    pub type_0: libc::c_char,
    pub value: cvalue,
}
//characteristic type
//characteristic value
//characteristic value

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvalue {
    pub integer: i32,
    pub _float: f32,
    pub string: *mut libc::c_char,
}
#[no_mangle]

pub static mut botcharacters: [*mut bot_character_t; 65] = [std::ptr::null_mut(); 65];
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCharacterFromHandle(mut handle: i32) -> *mut bot_character_t {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"character handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return std::ptr::null_mut();
    } //end if
    if botcharacters[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid character %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            handle,
        );
        return std::ptr::null_mut();
    }
    return botcharacters[handle as usize];
}
//end of the function BotCharacterFromHandle
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpCharacter(mut ch: *mut bot_character_t) {
    let mut i: i32 = 0; //end for
    crate::src::botlib::l_log::Log_Write(
        b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*ch).filename.as_mut_ptr(),
    );
    crate::src::botlib::l_log::Log_Write(
        b"skill %.1f\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*ch).skill as f64,
    );
    crate::src::botlib::l_log::Log_Write(
        b"{\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as i32;
    while i < 80 as i32 {
        match (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 {
            1 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.integer,
                );
            }
            2 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %f\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value._float as f64,
                );
            }
            3 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string,
                );
            }
            _ => {}
        }
        i += 1
        //end case
    }
    crate::src::botlib::l_log::Log_Write(
        b"}\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
//end of the function BotDumpCharacter
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacterStrings(mut ch: *mut bot_character_t) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 80 as i32 {
        if (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 as i32 {
            crate::src::botlib::l_memory::FreeMemory(
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string as *mut libc::c_void,
            );
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotFreeCharacterStrings
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacter2(mut handle: i32) {
    if handle <= 0 as i32 || handle > 64 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"character handle %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            handle,
        ); //end if
        return;
    } //end if
    if botcharacters[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4 as i32,
            b"invalid character %d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            handle,
        );
        return;
    }
    BotFreeCharacterStrings(botcharacters[handle as usize]);
    crate::src::botlib::l_memory::FreeMemory(botcharacters[handle as usize] as *mut libc::c_void);
    botcharacters[handle as usize] = std::ptr::null_mut();
}
//frees a bot character
//end of the function BotFreeCharacter2
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacter(mut handle: i32) {
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        return;
    }
    BotFreeCharacter2(handle);
}
//end of the function BotFreeCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDefaultCharacteristics(
    mut ch: *mut bot_character_t,
    mut defaultch: *mut bot_character_t,
) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < 80 as i32 {
        if !((*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 != 0) {
            //
            if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2 as i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 2 as i32 as libc::c_char; //end if
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value._float =
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        ._float
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 1 as i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 1 as i32 as libc::c_char; //end else if
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value.integer =
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        .integer
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 as i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 3 as i32 as libc::c_char;
                let ref mut fresh0 = (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string;
                *fresh0 = crate::src::botlib::l_memory::GetMemory(
                    crate::stdlib::strlen(
                        (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                            .value
                            .string,
                    )
                    .wrapping_add(1 as i32 as usize),
                ) as *mut libc::c_char;
                libc::strcpy(
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string,
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        .string,
                );
            }
        }
        i += 1
        //end else if
    }
    //end for
}
//end of the function BotDefaultCharacteristics
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacterFromFile(
    mut charfile: *mut libc::c_char,
    mut skill: i32,
) -> *mut bot_character_t {
    let mut indent: i32 = 0;
    let mut index: i32 = 0;
    let mut foundcharacter: i32 = 0;
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    let mut source: *mut source_t = std::ptr::null_mut();
    let mut token: token_t = token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: std::ptr::null_mut(),
        endwhitespace_p: std::ptr::null_mut(),
        line: 0,
        linescrossed: 0,
        next: std::ptr::null_mut(),
    };
    foundcharacter = qfalse as i32;
    //a bot character is parsed in two phases
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char); //end if
    source = LoadSourceFile(charfile) as *mut source_s;
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"counldn\'t load %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            charfile,
        );
        return std::ptr::null_mut();
    }
    ch = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<bot_character_t>() as usize).wrapping_add(
            (80 as i32 as usize)
                .wrapping_mul(::std::mem::size_of::<bot_characteristic_t>() as usize),
        ),
    ) as *mut bot_character_t;
    libc::strcpy((*ch).filename.as_mut_ptr(), charfile);
    //end else
    while PC_ReadToken(
        source as *mut source_s,
        &mut token as *mut _ as *mut token_s,
    ) != 0
    {
        if libc::strcmp(
            token.string.as_mut_ptr(),
            b"skill\x00" as *const u8 as *const libc::c_char,
        ) == 0
        {
            //end while
            if PC_ExpectTokenType(
                source as *mut source_s,
                3 as i32,
                0 as i32,
                &mut token as *mut _ as *mut token_s,
            ) == 0
            {
                FreeSource(source as *mut source_s); //end if
                BotFreeCharacterStrings(ch);
                crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                return std::ptr::null_mut();
            }
            //end else
            if PC_ExpectTokenString(
                source as *mut source_s,
                b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
            {
                FreeSource(source as *mut source_s); //end if
                BotFreeCharacterStrings(ch);
                crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                return std::ptr::null_mut();
            }
            if skill < 0 as i32 || token.intvalue == skill as usize {
                //if it's the correct skill
                foundcharacter = qtrue as i32; //end if
                (*ch).skill = token.intvalue as f32;
                while PC_ExpectAnyToken(
                    source as *mut source_s,
                    &mut token as *mut _ as *mut token_s,
                ) != 0
                {
                    if libc::strcmp(
                        token.string.as_mut_ptr(),
                        b"}\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        break;
                        //end else
                    } //end if
                    if token.type_0 != 3 as i32 || token.subtype & 0x1000 as i32 == 0 {
                        SourceError(
                            source as *mut source_s,
                            b"expected integer index, found %s\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            token.string.as_mut_ptr(),
                        ); //end if
                        FreeSource(source as *mut source_s); //end if
                        BotFreeCharacterStrings(ch); //end if
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void); //end if
                        return std::ptr::null_mut();
                    } //end if
                    index = token.intvalue as i32;
                    if index < 0 as i32 || index > 80 as i32 {
                        SourceError(
                            source as *mut source_s,
                            b"characteristic index out of range [0, %d]\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            80 as i32,
                        );
                        FreeSource(source as *mut source_s);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return std::ptr::null_mut();
                    }
                    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 != 0 {
                        SourceError(
                            source as *mut source_s,
                            b"characteristic %d already initialized\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            index,
                        );
                        FreeSource(source as *mut source_s);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return std::ptr::null_mut();
                    }
                    if PC_ExpectAnyToken(
                        source as *mut source_s,
                        &mut token as *mut _ as *mut token_s,
                    ) == 0
                    {
                        FreeSource(source as *mut source_s);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return std::ptr::null_mut();
                    }
                    if token.type_0 == 3 as i32 {
                        if token.subtype & 0x800 as i32 != 0 {
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float =
                                token.floatvalue;
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 =
                                2 as i32 as libc::c_char
                        } else {
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer =
                                token.intvalue as i32;
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 =
                                1 as i32 as libc::c_char
                        }
                    //end else
                    } else if token.type_0 == 1 as i32 {
                        StripDoubleQuotes(token.string.as_mut_ptr()); //end else if
                        let ref mut fresh1 =
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string;
                        *fresh1 = crate::src::botlib::l_memory::GetMemory(
                            crate::stdlib::strlen(token.string.as_mut_ptr())
                                .wrapping_add(1 as i32 as usize),
                        ) as *mut libc::c_char;
                        libc::strcpy(
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string,
                            token.string.as_mut_ptr(),
                        );
                        (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 =
                            3 as i32 as libc::c_char
                    } else {
                        SourceError(
                            source as *mut source_s,
                            b"expected integer, float or string, found %s\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            token.string.as_mut_ptr(),
                        );
                        FreeSource(source as *mut source_s);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return std::ptr::null_mut();
                    }
                }
                break;
            } else {
                indent = 1 as i32;
                while indent != 0 {
                    //end while
                    if PC_ExpectAnyToken(
                        source as *mut source_s,
                        &mut token as *mut _ as *mut token_s,
                    ) == 0
                    {
                        FreeSource(source as *mut source_s); //end if
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return std::ptr::null_mut();
                    }
                    if libc::strcmp(
                        token.string.as_mut_ptr(),
                        b"{\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        indent += 1
                    } else if libc::strcmp(
                        token.string.as_mut_ptr(),
                        b"}\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        indent -= 1
                    }
                }
            }
        } else {
            SourceError(
                source as *mut source_s,
                b"unknown definition %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            FreeSource(source as *mut source_s);
            BotFreeCharacterStrings(ch);
            crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    FreeSource(source as *mut source_s);
    //
    if foundcharacter == 0 {
        BotFreeCharacterStrings(ch); //end if
        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
        return std::ptr::null_mut();
    }
    return ch;
}
//end of the function BotLoadCharacterFromFile
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFindCachedCharacter(
    mut charfile: *mut libc::c_char,
    mut skill: f32,
) -> i32 {
    let mut handle: i32 = 0; //end for
    handle = 1 as i32;
    while handle <= 64 as i32 {
        if !botcharacters[handle as usize].is_null() {
            if libc::strcmp(
                (*botcharacters[handle as usize]).filename.as_mut_ptr(),
                charfile,
            ) == 0 as i32
                && (skill < 0 as i32 as f32
                    || crate::stdlib::fabs(
                        ((*botcharacters[handle as usize]).skill - skill) as f64,
                    ) < 0.01f64)
            {
                return handle;
            }
        }
        handle += 1
        //end if
    }
    return 0 as i32;
}
//end of the function BotFindCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCachedCharacter(
    mut charfile: *mut libc::c_char,
    mut skill: f32,
    mut reload: i32,
) -> i32 {
    let mut handle: i32 = 0;
    let mut cachedhandle: i32 = 0;
    let mut intskill: i32 = 0;
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    //DEBUG
    //find a free spot for a character
    handle = 1 as i32; //end for
    while handle <= 64 as i32 {
        if botcharacters[handle as usize].is_null() {
            break;
        }
        handle += 1
    }
    if handle > 64 as i32 {
        return 0 as i32;
    }
    //try to load a cached character with the given skill
    if reload == 0 {
        cachedhandle = BotFindCachedCharacter(charfile, skill);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"loaded cached skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skill as f64,
                charfile,
            ); //end else
            return cachedhandle;
        }
        //end if
    }
    //
    intskill = (skill as f64 + 0.5f64) as i32;
    //try to load the character with the given skill
    ch = BotLoadCharacterFromFile(charfile, intskill); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        //
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"loaded skill %d from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            intskill,
            charfile,
        );
        //DEBUG
        return handle;
    }
    //
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2 as i32,
        b"couldn\'t find skill %d in %s\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        intskill,
        charfile,
    );
    //
    if reload == 0 {
        //end if
        //try to load a cached default character with the given skill
        cachedhandle = BotFindCachedCharacter(
            b"bots/default_c.c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill,
        );
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"loaded cached default skill %d from %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                intskill,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load the default character with the given skill
    ch = BotLoadCharacterFromFile(
        b"bots/default_c.c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        intskill,
    ); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"loaded default skill %d from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            intskill,
            charfile,
        );
        return handle;
    }
    //
    if reload == 0 {
        //end if
        //try to load a cached character with any skill
        cachedhandle = BotFindCachedCharacter(charfile, -(1 as i32) as f32);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"loaded cached skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*botcharacters[cachedhandle as usize]).skill as f64,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load a character with any skill
    ch = BotLoadCharacterFromFile(charfile, -(1 as i32)); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"loaded skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ch).skill as f64,
            charfile,
        );
        return handle;
    }
    //
    if reload == 0 {
        //end if
        //try to load a cached character with any skill
        cachedhandle = BotFindCachedCharacter(
            b"bots/default_c.c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            -(1 as i32) as f32,
        );
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1 as i32,
                b"loaded cached default skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*botcharacters[cachedhandle as usize]).skill as f64,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load a character with any skill
    ch = BotLoadCharacterFromFile(
        b"bots/default_c.c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        -(1 as i32),
    ); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"loaded default skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ch).skill as f64,
            charfile,
        );
        return handle;
    }
    //
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2 as i32,
        b"couldn\'t load any skill from %s\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        charfile,
    );
    //couldn't load any character
    return 0 as i32;
}
//end of the function BotLoadCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacterSkill(
    mut charfile: *mut libc::c_char,
    mut skill: f32,
) -> i32 {
    let mut ch: i32 = 0; //end if
    let mut defaultch: i32 = 0;
    defaultch = BotLoadCachedCharacter(
        b"bots/default_c.c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        skill,
        qfalse as i32,
    );
    ch = BotLoadCachedCharacter(
        charfile,
        skill,
        crate::src::botlib::l_libvar::LibVarGetValue(
            b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
        ) as i32,
    );
    if defaultch != 0 && ch != 0 {
        BotDefaultCharacteristics(
            botcharacters[ch as usize],
            botcharacters[defaultch as usize],
        );
    }
    return ch;
}
//end of the function BotLoadCharacterSkill
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInterpolateCharacters(
    mut handle1: i32,
    mut handle2: i32,
    mut desiredskill: f32,
) -> i32 {
    let mut ch1: *mut bot_character_t = std::ptr::null_mut();
    let mut ch2: *mut bot_character_t = std::ptr::null_mut();
    let mut out: *mut bot_character_t = std::ptr::null_mut();
    let mut i: i32 = 0;
    let mut handle: i32 = 0;
    let mut scale: f32 = 0.;
    ch1 = BotCharacterFromHandle(handle1);
    ch2 = BotCharacterFromHandle(handle2);
    if ch1.is_null() || ch2.is_null() {
        return 0 as i32;
    }
    //find a free spot for a character
    handle = 1 as i32; //end for
    while handle <= 64 as i32 {
        if botcharacters[handle as usize].is_null() {
            break; //end for
        }
        handle += 1
    }
    if handle > 64 as i32 {
        return 0 as i32;
    }
    out = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<bot_character_t>() as usize).wrapping_add(
            (80 as i32 as usize)
                .wrapping_mul(::std::mem::size_of::<bot_characteristic_t>() as usize),
        ),
    ) as *mut bot_character_t;
    (*out).skill = desiredskill;
    libc::strcpy((*out).filename.as_mut_ptr(), (*ch1).filename.as_mut_ptr());
    botcharacters[handle as usize] = out;
    scale = (desiredskill - (*ch1).skill) / ((*ch2).skill - (*ch1).skill);
    i = 0 as i32;
    while i < 80 as i32 {
        //
        if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2 as i32
            && (*(*ch2).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2 as i32
        {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 2 as i32 as libc::c_char; //end if
            (*(*out).c.as_mut_ptr().offset(i as isize)).value._float =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value._float
                    + ((*(*ch2).c.as_mut_ptr().offset(i as isize)).value._float
                        - (*(*ch1).c.as_mut_ptr().offset(i as isize)).value._float)
                        * scale
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 1 as i32 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 1 as i32 as libc::c_char; //end else if
            (*(*out).c.as_mut_ptr().offset(i as isize)).value.integer =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.integer
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 as i32 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 3 as i32 as libc::c_char;
            let ref mut fresh2 = (*(*out).c.as_mut_ptr().offset(i as isize)).value.string;
            *fresh2 = crate::src::botlib::l_memory::GetMemory(
                crate::stdlib::strlen((*(*ch1).c.as_mut_ptr().offset(i as isize)).value.string)
                    .wrapping_add(1 as i32 as usize),
            ) as *mut libc::c_char;
            libc::strcpy(
                (*(*out).c.as_mut_ptr().offset(i as isize)).value.string,
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.string,
            );
        }
        i += 1
        //end else if
    }
    return handle;
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
/* ****************************************************************************
 * name:		be_ai_char.h
 *
 * desc:		bot characters
 *
 * $Archive: /source/code/botlib/be_ai_char.h $
 *
 *****************************************************************************/
//loads a bot character from a file
//end of the function BotInterpolateCharacters
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacter(mut charfile: *mut libc::c_char, mut skill: f32) -> i32 {
    let mut firstskill: i32 = 0;
    let mut secondskill: i32 = 0;
    let mut handle: i32 = 0;
    //make sure the skill is in the valid range
    if (skill as f64) < 1.0f64 {
        skill = 1.0f64 as f32
    } else if skill as f64 > 5.0f64 {
        skill = 5.0f64 as f32
    }
    //skill 1, 4 and 5 should be available in the character files
    if skill as f64 == 1.0f64 || skill as f64 == 4.0f64 || skill as f64 == 5.0f64 {
        return BotLoadCharacterSkill(charfile, skill);
    } //end if
      //check if there's a cached skill
    handle = BotFindCachedCharacter(charfile, skill); //end if
    if handle != 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as i32,
            b"loaded cached skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            skill as f64,
            charfile,
        ); //end else
        return handle;
    } //end if
    if (skill as f64) < 4.0f64 {
        //load skill 1 and 4
        firstskill = BotLoadCharacterSkill(charfile, 1 as i32 as f32);
        if firstskill == 0 {
            return 0 as i32;
        }
        secondskill = BotLoadCharacterSkill(charfile, 4 as i32 as f32);
        if secondskill == 0 {
            return firstskill;
        }
    } else {
        //load skill 4 and 5
        firstskill = BotLoadCharacterSkill(charfile, 4 as i32 as f32);
        if firstskill == 0 {
            return 0 as i32;
        }
        secondskill = BotLoadCharacterSkill(charfile, 5 as i32 as f32);
        if secondskill == 0 {
            return firstskill;
        }
    }
    //interpolate between the two skills
    handle = BotInterpolateCharacters(firstskill, secondskill, skill);
    if handle == 0 {
        return 0 as i32;
    }
    //write the character to the log file
    BotDumpCharacter(botcharacters[handle as usize]);
    //
    return handle;
}
//end of the function BotLoadCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CheckCharacteristicIndex(mut character: i32, mut index: i32) -> i32 {
    let mut ch: *mut bot_character_t = std::ptr::null_mut(); //end if
    ch = BotCharacterFromHandle(character); //end if
    if ch.is_null() {
        return qfalse as i32;
    }
    if index < 0 as i32 || index >= 80 as i32 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"characteristic %d does not exist\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
        return qfalse as i32;
    }
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"characteristic %d is not initialized\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
        return qfalse as i32;
    }
    return qtrue as i32;
}
//returns a float characteristic
//end of the function CheckCharacteristicIndex
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_Float(mut character: i32, mut index: i32) -> f32 {
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0 as i32 as f32;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return 0 as i32 as f32;
    }
    //an integer will be converted to a float
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 1 as i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer as f32;
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 2 as i32 {
        //end if
        //floats are just returned
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float;
    } else {
        //end else if
        //cannot convert a string pointer to a float
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"characteristic %d is not a float\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
        return 0 as i32 as f32;
    };
    //end else if
    //	return 0;
}
//returns a bounded float characteristic
//end of the function Characteristic_Float
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_BFloat(
    mut character: i32,
    mut index: i32,
    mut min: f32,
    mut max: f32,
) -> f32 {
    let mut value: f32 = 0.; //end if
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0 as i32 as f32;
    }
    if min > max {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"cannot bound characteristic %d between %f and %f\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            index,
            min as f64,
            max as f64,
        );
        return 0 as i32 as f32;
    }
    value = Characteristic_Float(character, index);
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
//returns an integer characteristic
//end of the function Characteristic_BFloat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_Integer(mut character: i32, mut index: i32) -> i32 {
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0 as i32;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return 0 as i32;
    }
    //an integer will just be returned
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 1 as i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer;
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 2 as i32 {
        //end if
        //floats are casted to integers
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float as i32;
    } else {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"characteristic %d is not an integer\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        ); //end else if
        return 0 as i32;
    };
    //end else if
    //	return 0;
}
//returns a bounded integer characteristic
//end of the function Characteristic_Integer
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_BInteger(
    mut character: i32,
    mut index: i32,
    mut min: i32,
    mut max: i32,
) -> i32 {
    let mut value: i32 = 0; //end if
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0 as i32;
    }
    if min > max {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"cannot bound characteristic %d between %d and %d\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            index,
            min,
            max,
        );
        return 0 as i32;
    }
    value = Characteristic_Integer(character, index);
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
//returns a string characteristic
//end of the function Characteristic_BInteger
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_String(
    mut character: i32,
    mut index: i32,
    mut buf: *mut libc::c_char,
    mut size: i32,
) {
    let mut ch: *mut bot_character_t = std::ptr::null_mut();
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return;
    }
    //an integer will be converted to a float
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 3 as i32 {
        crate::stdlib::strncpy(
            buf,
            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string,
            (size - 1 as i32) as usize,
        ); //end if
        *buf.offset((size - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as i32,
            b"characteristic %d is not a string\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
    };
    //end else if
}
//free cached bot characters
//end of the function Characteristic_String
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownCharacters() {
    let mut handle: i32 = 0;
    handle = 1 as i32;
    while handle <= 64 as i32 {
        if !botcharacters[handle as usize].is_null() {
            BotFreeCharacter2(handle);
        }
        handle += 1
        //end if
    }
    //end for
}
//end of the function BotShutdownCharacters

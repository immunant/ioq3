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

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::src::client::cl_cgame::CL_GameCommand;
pub use crate::src::client::cl_main::CL_ForwardCommandToServer;
pub use crate::src::client::cl_ui::UI_GameCommand;
pub use crate::src::qcommon::cmd::stdlib_h::atoi;
pub use crate::src::qcommon::common::com_cl_running;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Filter;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::CopyString;
pub use crate::src::qcommon::common::Field_CompleteFilename;
pub use crate::src::qcommon::common::S_Malloc;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::cvar::Cvar_Command;
pub use crate::src::qcommon::cvar::Cvar_CompleteCvarName;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::files::FS_FreeFile;
pub use crate::src::qcommon::files::FS_ReadFile;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::COM_DefaultExtension;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::server::sv_game::SV_GameCommand;

pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_t {
    pub data: *mut crate::src::qcommon::q_shared::byte,
    pub maxsize: i32,
    pub cursize: i32,
}
/*
=============================================================================

                    COMMAND EXECUTION

=============================================================================
*/

pub type cmd_function_t = cmd_function_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_function_s {
    pub next: *mut cmd_function_s,
    pub name: *mut libc::c_char,
    pub function: crate::qcommon_h::xcommand_t,
    pub complete: crate::qcommon_h::completionFunc_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_118 {
    pub c: *mut libc::c_char,
    pub v: *mut libc::c_void,
}
#[no_mangle]

pub static mut cmd_wait: i32 = 0;
#[no_mangle]

pub static mut cmd_text: cmd_t = cmd_t {
    data: 0 as *const crate::src::qcommon::q_shared::byte
        as *mut crate::src::qcommon::q_shared::byte,
    maxsize: 0,
    cursize: 0,
};
#[no_mangle]

pub static mut cmd_text_buf: [crate::src::qcommon::q_shared::byte; 131072] = [0; 131072];
//=============================================================================
/*
============
Cmd_Wait_f

Causes execution of the remainder of the command buffer to be delayed until
next frame.  This allows commands like:
bind g "cmd use rocket ; +attack ; wait ; -attack ; cmd use blaster"
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Wait_f() {
    if Cmd_Argc() == 2 as i32 {
        cmd_wait = atoi(Cmd_Argv(1 as i32));
        if cmd_wait < 0 as i32 {
            cmd_wait = 1 as i32
        }
    // ignore the argument
    } else {
        cmd_wait = 1 as i32
    };
}
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
/*
=============================================================================

                        COMMAND BUFFER

=============================================================================
*/
/*
============
Cbuf_Init
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cbuf_Init() {
    cmd_text.data = cmd_text_buf.as_mut_ptr();
    cmd_text.maxsize = 128 as i32 * 1024 as i32;
    cmd_text.cursize = 0 as i32;
}
// allocates an initial text buffer that will grow as needed
/*
============
Cbuf_AddText

Adds command text at the end of the buffer, does NOT add a final \n
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cbuf_AddText(mut text: *const libc::c_char) {
    let mut l: i32 = 0;
    l = crate::stdlib::strlen(text) as i32;
    if cmd_text.cursize + l >= cmd_text.maxsize {
        crate::src::qcommon::common::Com_Printf(
            b"Cbuf_AddText: overflow\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    crate::stdlib::memcpy(
        &mut *cmd_text.data.offset(cmd_text.cursize as isize)
            as *mut crate::src::qcommon::q_shared::byte as *mut libc::c_void,
        text as *const libc::c_void,
        l as libc::c_ulong,
    );
    cmd_text.cursize += l;
}
/*
============
Cbuf_InsertText

Adds command text immediately after the current command
Adds a \n to the text
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cbuf_InsertText(mut text: *const libc::c_char) {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    len = crate::stdlib::strlen(text).wrapping_add(1 as i32 as libc::c_ulong) as i32;
    if len + cmd_text.cursize > cmd_text.maxsize {
        crate::src::qcommon::common::Com_Printf(
            b"Cbuf_InsertText overflowed\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    // move the existing command text
    i = cmd_text.cursize - 1 as i32;
    while i >= 0 as i32 {
        *cmd_text.data.offset((i + len) as isize) = *cmd_text.data.offset(i as isize);
        i -= 1
    }
    // copy the new text in
    crate::stdlib::memcpy(
        cmd_text.data as *mut libc::c_void,
        text as *const libc::c_void,
        (len - 1 as i32) as libc::c_ulong,
    );
    // add a \n
    *cmd_text.data.offset((len - 1 as i32) as isize) =
        '\n' as i32 as crate::src::qcommon::q_shared::byte;
    cmd_text.cursize += len;
}
// Adds command text at the end of the buffer, does NOT add a final \n
/*
============
Cbuf_ExecuteText
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cbuf_ExecuteText(mut exec_when: i32, mut text: *const libc::c_char) {
    match exec_when {
        0 => {
            if !text.is_null() && crate::stdlib::strlen(text) > 0 as i32 as libc::c_ulong {
                crate::src::qcommon::common::Com_DPrintf(
                    b"^3EXEC_NOW %s\n\x00" as *const u8 as *const libc::c_char,
                    text,
                );
                Cmd_ExecuteString(text);
            } else {
                Cbuf_Execute();
                crate::src::qcommon::common::Com_DPrintf(
                    b"^3EXEC_NOW %s\n\x00" as *const u8 as *const libc::c_char,
                    cmd_text.data,
                );
            }
        }
        1 => {
            Cbuf_InsertText(text);
        }
        2 => {
            Cbuf_AddText(text);
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"Cbuf_ExecuteText: bad exec_when\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
/*
============
Cbuf_Execute
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cbuf_Execute() {
    let mut i: i32 = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut quotes: i32 = 0;
    // This will keep // style comments all on one line by not breaking on
    // a semicolon.  It will keep /* ... */ style comments all on one line by not
    // breaking it for semicolon or newline.
    let mut in_star_comment: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut in_slash_comment: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    while cmd_text.cursize != 0 {
        if cmd_wait > 0 as i32 {
            // skip out while text still remains in buffer, leaving it
            // for next frame
            cmd_wait -= 1;
            break;
        } else {
            // find a \n or ; line break or comment: // or /* */
            text = cmd_text.data as *mut libc::c_char;
            quotes = 0 as i32;
            i = 0 as i32;
            while i < cmd_text.cursize {
                if *text.offset(i as isize) as i32 == '\"' as i32 {
                    quotes += 1
                }
                if quotes & 1 as i32 == 0 {
                    if i < cmd_text.cursize - 1 as i32 {
                        if in_star_comment as u64 == 0
                            && *text.offset(i as isize) as i32 == '/' as i32
                            && *text.offset((i + 1 as i32) as isize) as i32 == '/' as i32
                        {
                            in_slash_comment = crate::src::qcommon::q_shared::qtrue
                        } else if in_slash_comment as u64 == 0
                            && *text.offset(i as isize) as i32 == '/' as i32
                            && *text.offset((i + 1 as i32) as isize) as i32 == '*' as i32
                        {
                            in_star_comment = crate::src::qcommon::q_shared::qtrue
                        } else if in_star_comment as u32 != 0
                            && *text.offset(i as isize) as i32 == '*' as i32
                            && *text.offset((i + 1 as i32) as isize) as i32 == '/' as i32
                        {
                            in_star_comment = crate::src::qcommon::q_shared::qfalse;
                            // If we are in a star comment, then the part after it is valid
                            // Note: This will cause it to NUL out the terminating '/'
                            // but ExecuteString doesn't require it anyway.
                            i += 1;
                            break;
                        }
                    }
                    if in_slash_comment as u64 == 0
                        && in_star_comment as u64 == 0
                        && *text.offset(i as isize) as i32 == ';' as i32
                    {
                        break;
                    }
                }
                if in_star_comment as u64 == 0
                    && (*text.offset(i as isize) as i32 == '\n' as i32
                        || *text.offset(i as isize) as i32 == '\r' as i32)
                {
                    in_slash_comment = crate::src::qcommon::q_shared::qfalse;
                    break;
                } else {
                    i += 1
                }
            }
            if i >= 1024 as i32 - 1 as i32 {
                i = 1024 as i32 - 1 as i32
            }
            crate::stdlib::memcpy(
                line.as_mut_ptr() as *mut libc::c_void,
                text as *const libc::c_void,
                i as libc::c_ulong,
            );
            line[i as usize] = 0 as i32 as libc::c_char;
            // delete the text from the command buffer and move remaining commands down
            // this is necessary because commands (exec) can insert data at the
            // beginning of the text buffer
            if i == cmd_text.cursize {
                cmd_text.cursize = 0 as i32
            } else {
                i += 1;
                cmd_text.cursize -= i;
                crate::stdlib::memmove(
                    text as *mut libc::c_void,
                    text.offset(i as isize) as *const libc::c_void,
                    cmd_text.cursize as libc::c_ulong,
                );
            }
            // execute the command line
            Cmd_ExecuteString(line.as_mut_ptr());
        }
    }
}
/*
==============================================================================

                        SCRIPT COMMANDS

==============================================================================
*/
/*
===============
Cmd_Exec_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Exec_f() {
    let mut quiet: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut f: C2RustUnnamed_118 = C2RustUnnamed_118 {
        c: 0 as *mut libc::c_char,
    };
    let mut filename: [libc::c_char; 64] = [0; 64];
    quiet = (crate::src::qcommon::q_shared::Q_stricmp(
        Cmd_Argv(0 as i32),
        b"execq\x00" as *const u8 as *const libc::c_char,
    ) == 0) as i32 as crate::src::qcommon::q_shared::qboolean;
    if Cmd_Argc() != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"exec%s <filename> : execute a script file%s\n\x00" as *const u8
                as *const libc::c_char,
            if quiet as u32 != 0 {
                b"q\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
            if quiet as u32 != 0 {
                b" without notification\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        filename.as_mut_ptr(),
        Cmd_Argv(1 as i32),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
    );
    crate::src::qcommon::q_shared::COM_DefaultExtension(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as i32,
        b".cfg\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::files::FS_ReadFile(filename.as_mut_ptr(), &mut f.v);
    if f.c.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"couldn\'t exec %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return;
    }
    if quiet as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"execing %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    Cbuf_InsertText(f.c);
    crate::src::qcommon::files::FS_FreeFile(f.v);
}
/*
===============
Cmd_Vstr_f

Inserts the current value of a variable as command text
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Vstr_f() {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() != 2 as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"vstr <variablename> : execute a variable command\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    v = crate::src::qcommon::cvar::Cvar_VariableString(Cmd_Argv(1 as i32));
    Cbuf_InsertText(crate::src::qcommon::q_shared::va(
        b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        v,
    ));
}
/*
===============
Cmd_Echo_f

Just prints the rest of the line to the console
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Echo_f() {
    crate::src::qcommon::common::Com_Printf(
        b"%s\n\x00" as *const u8 as *const libc::c_char,
        Cmd_Args(),
    );
}

static mut cmd_argc: i32 = 0;

static mut cmd_argv: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
// points into cmd_tokenized

static mut cmd_tokenized: [libc::c_char; 9216] = [0; 9216];
// will have 0 bytes inserted

static mut cmd_cmd: [libc::c_char; 8192] = [0; 8192];
// the original command we received (no token processing)

static mut cmd_functions: *mut cmd_function_t = 0 as *const cmd_function_t as *mut cmd_function_t;
// possible commands to execute
/*
============
Cmd_Argc
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Argc() -> i32 {
    return cmd_argc;
}
/*
============
Cmd_Argv
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Argv(mut arg: i32) -> *mut libc::c_char {
    if arg as u32 >= cmd_argc as u32 {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return cmd_argv[arg as usize];
}
/*
============
Cmd_ArgvBuffer

The interpreted versions use this because
they can't have pointers returned to them
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_ArgvBuffer(
    mut arg: i32,
    mut buffer: *mut libc::c_char,
    mut bufferLength: i32,
) {
    crate::src::qcommon::q_shared::Q_strncpyz(buffer, Cmd_Argv(arg), bufferLength);
}
/*
============
Cmd_Args

Returns a single string containing argv(1) to argv(argc()-1)
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Args() -> *mut libc::c_char {
    static mut cmd_args: [libc::c_char; 1024] = [0; 1024];
    let mut i: i32 = 0;
    cmd_args[0 as i32 as usize] = 0 as i32 as libc::c_char;
    i = 1 as i32;
    while i < cmd_argc {
        ::libc::strcat(cmd_args.as_mut_ptr(), cmd_argv[i as usize]);
        if i != cmd_argc - 1 as i32 {
            ::libc::strcat(
                cmd_args.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
        i += 1
    }
    return cmd_args.as_mut_ptr();
}
/*
============
Cmd_Args

Returns a single string containing argv(arg) to argv(argc()-1)
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_ArgsFrom(mut arg: i32) -> *mut libc::c_char {
    static mut cmd_args: [libc::c_char; 8192] = [0; 8192];
    let mut i: i32 = 0;
    cmd_args[0 as i32 as usize] = 0 as i32 as libc::c_char;
    if arg < 0 as i32 {
        arg = 0 as i32
    }
    i = arg;
    while i < cmd_argc {
        ::libc::strcat(cmd_args.as_mut_ptr(), cmd_argv[i as usize]);
        if i != cmd_argc - 1 as i32 {
            ::libc::strcat(
                cmd_args.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
        i += 1
    }
    return cmd_args.as_mut_ptr();
}
/*
============
Cmd_ArgsBuffer

The interpreted versions use this because
they can't have pointers returned to them
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_ArgsBuffer(mut buffer: *mut libc::c_char, mut bufferLength: i32) {
    crate::src::qcommon::q_shared::Q_strncpyz(buffer, Cmd_Args(), bufferLength);
}
/*
============
Cmd_Cmd

Retrieve the unmodified command string
For rcon use when you want to transmit without altering quoting
https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=543
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Cmd() -> *mut libc::c_char {
    return cmd_cmd.as_mut_ptr();
}
/*
   Replace command separators with space to prevent interpretation
   This is a hack to protect buggy qvms
   https://bugzilla.icculus.org/show_bug.cgi?id=3593
   https://bugzilla.icculus.org/show_bug.cgi?id=4769
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Args_Sanitize() {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < cmd_argc {
        let mut c: *mut libc::c_char = cmd_argv[i as usize];
        if crate::stdlib::strlen(c) > (256 as i32 - 1 as i32) as libc::c_ulong {
            *c.offset((256 as i32 - 1 as i32) as isize) = '\u{0}' as i32 as libc::c_char
        }
        loop {
            c = ::libc::strpbrk(c, b"\n\r;\x00" as *const u8 as *const libc::c_char);
            if c.is_null() {
                break;
            }
            *c = ' ' as i32 as libc::c_char;
            c = c.offset(1)
        }
        i += 1
    }
}
/*
============
Cmd_TokenizeString

Parses the given string into command line tokens.
The text is copied to a separate buffer and 0 characters
are inserted in the appropriate place, The argv array
will point into this temporary buffer.
============
*/
// NOTE TTimo define that to track tokenization issues
//#define TKN_DBG

unsafe extern "C" fn Cmd_TokenizeString2(
    mut text_in: *const libc::c_char,
    mut ignoreQuotes: crate::src::qcommon::q_shared::qboolean,
) {
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut textOut: *mut libc::c_char = 0 as *mut libc::c_char;
    // clear previous args
    cmd_argc = 0 as i32;
    if text_in.is_null() {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        cmd_cmd.as_mut_ptr(),
        text_in,
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as i32,
    );
    text = text_in;
    textOut = cmd_tokenized.as_mut_ptr();
    loop {
        if cmd_argc == 1024 as i32 {
            return;
            // this is usually something malicious
        }
        loop {
            // skip whitespace
            while *text as i32 != 0 && *text as i32 <= ' ' as i32 {
                text = text.offset(1)
            }
            if *text == 0 {
                return;
                // all tokens parsed
            }
            // skip // comments
            if *text.offset(0 as i32 as isize) as i32 == '/' as i32
                && *text.offset(1 as i32 as isize) as i32 == '/' as i32
            {
                return;
                // all tokens parsed
            }
            // skip /* */ comments
            if !(*text.offset(0 as i32 as isize) as i32 == '/' as i32
                && *text.offset(1 as i32 as isize) as i32 == '*' as i32)
            {
                break;
            }
            while *text as i32 != 0
                && (*text.offset(0 as i32 as isize) as i32 != '*' as i32
                    || *text.offset(1 as i32 as isize) as i32 != '/' as i32)
            {
                text = text.offset(1)
            }
            if *text == 0 {
                return;
                // all tokens parsed
            }
            text = text.offset(2 as i32 as isize)
        }
        // handle quoted strings
        // NOTE TTimo this doesn't handle \" escaping
        if ignoreQuotes as u64 == 0 && *text as i32 == '\"' as i32 {
            cmd_argv[cmd_argc as usize] = textOut;
            cmd_argc += 1;
            text = text.offset(1);
            while *text as i32 != 0 && *text as i32 != '\"' as i32 {
                let fresh0 = text;
                text = text.offset(1);
                let fresh1 = textOut;
                textOut = textOut.offset(1);
                *fresh1 = *fresh0
            }
            let fresh2 = textOut;
            textOut = textOut.offset(1);
            *fresh2 = 0 as i32 as libc::c_char;
            if *text == 0 {
                return;
                // all tokens parsed
            }
            text = text.offset(1)
        } else {
            // regular token
            cmd_argv[cmd_argc as usize] = textOut;
            cmd_argc += 1;
            // skip until whitespace, quote, or command
            while *text as i32 > ' ' as i32 {
                if ignoreQuotes as u64 == 0 && *text.offset(0 as i32 as isize) as i32 == '\"' as i32
                {
                    break;
                }
                if *text.offset(0 as i32 as isize) as i32 == '/' as i32
                    && *text.offset(1 as i32 as isize) as i32 == '/' as i32
                {
                    break;
                }
                // skip /* */ comments
                if *text.offset(0 as i32 as isize) as i32 == '/' as i32
                    && *text.offset(1 as i32 as isize) as i32 == '*' as i32
                {
                    break;
                }
                let fresh3 = text;
                text = text.offset(1);
                let fresh4 = textOut;
                textOut = textOut.offset(1);
                *fresh4 = *fresh3
            }
            let fresh5 = textOut;
            textOut = textOut.offset(1);
            *fresh5 = 0 as i32 as libc::c_char;
            if *text == 0 {
                return;
                // all tokens parsed
            }
        }
    }
}
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
/*
============
Cmd_TokenizeString
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_TokenizeString(mut text_in: *const libc::c_char) {
    Cmd_TokenizeString2(text_in, crate::src::qcommon::q_shared::qfalse);
}
/*
============
Cmd_TokenizeStringIgnoreQuotes
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_TokenizeStringIgnoreQuotes(mut text_in: *const libc::c_char) {
    Cmd_TokenizeString2(text_in, crate::src::qcommon::q_shared::qtrue);
}
/*
============
Cmd_FindCommand
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_FindCommand(mut cmd_name: *const libc::c_char) -> *mut cmd_function_t {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(cmd_name, (*cmd).name) == 0 {
            return cmd;
        }
        cmd = (*cmd).next
    }
    return 0 as *mut cmd_function_t;
}
/*
============
Cmd_AddCommand
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_AddCommand(
    mut cmd_name: *const libc::c_char,
    mut function: crate::qcommon_h::xcommand_t,
) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    // fail if the command already exists
    if !Cmd_FindCommand(cmd_name).is_null() {
        // allow completion-only commands to be silently doubled
        if function.is_some() {
            crate::src::qcommon::common::Com_Printf(
                b"Cmd_AddCommand: %s already defined\n\x00" as *const u8 as *const libc::c_char,
                cmd_name,
            );
        }
        return;
    }
    // use a small malloc to avoid zone fragmentation
    cmd = crate::src::qcommon::common::S_Malloc(
        ::std::mem::size_of::<cmd_function_t>() as libc::c_ulong as i32
    ) as *mut cmd_function_t;
    (*cmd).name = crate::src::qcommon::common::CopyString(cmd_name);
    (*cmd).function = function;
    (*cmd).complete = None;
    (*cmd).next = cmd_functions;
    cmd_functions = cmd;
}
// callback with each valid string
/*
============
Cmd_SetCommandCompletionFunc
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_SetCommandCompletionFunc(
    mut command: *const libc::c_char,
    mut complete: crate::qcommon_h::completionFunc_t,
) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(command, (*cmd).name) == 0 {
            (*cmd).complete = complete;
            return;
        }
        cmd = (*cmd).next
    }
}
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
/*
============
Cmd_RemoveCommand
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_RemoveCommand(mut cmd_name: *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut back: *mut *mut cmd_function_t = 0 as *mut *mut cmd_function_t;
    back = &mut cmd_functions;
    loop {
        cmd = *back;
        if cmd.is_null() {
            // command wasn't active
            return;
        }
        if ::libc::strcmp(cmd_name, (*cmd).name) == 0 {
            *back = (*cmd).next;
            crate::src::qcommon::common::Z_Free((*cmd).name as *mut libc::c_void);
            crate::src::qcommon::common::Z_Free(cmd as *mut libc::c_void);
            return;
        }
        back = &mut (*cmd).next
    }
}
// don't allow VMs to remove system commands
/*
============
Cmd_RemoveCommandSafe

Only remove commands with no associated function
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_RemoveCommandSafe(mut cmd_name: *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = Cmd_FindCommand(cmd_name);
    if cmd.is_null() {
        return;
    }
    if (*cmd).function.is_some() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Restricted source tried to remove system command \"%s\"\x00" as *const u8
                as *const libc::c_char,
            cmd_name,
        );
    }
    Cmd_RemoveCommand(cmd_name);
}
/*
============
Cmd_CommandCompletion
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CommandCompletion(
    mut callback: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        callback.expect("non-null function pointer")((*cmd).name);
        cmd = (*cmd).next
    }
}
/*
============
Cmd_CompleteArgument
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CompleteArgument(
    mut command: *const libc::c_char,
    mut args: *mut libc::c_char,
    mut argNum: i32,
) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(command, (*cmd).name) == 0 {
            if (*cmd).complete.is_some() {
                (*cmd).complete.expect("non-null function pointer")(args, argNum);
            }
            return;
        }
        cmd = (*cmd).next
    }
}
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
/*
============
Cmd_ExecuteString

A complete command line has been parsed, so try to execute it
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_ExecuteString(mut text: *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut prev: *mut *mut cmd_function_t = 0 as *mut *mut cmd_function_t;
    // execute the command line
    Cmd_TokenizeString(text);
    if Cmd_Argc() == 0 {
        return;
        // no tokens
    }
    // check registered command functions
    prev = &mut cmd_functions;
    while !(*prev).is_null() {
        cmd = *prev;
        if crate::src::qcommon::q_shared::Q_stricmp(cmd_argv[0 as i32 as usize], (*cmd).name) == 0 {
            // rearrange the links so that the command will be
            // near the head of the list next time it is used
            *prev = (*cmd).next;
            (*cmd).next = cmd_functions;
            cmd_functions = cmd;
            // perform the action
            if (*cmd).function.is_none() {
                break;
            }
            (*cmd).function.expect("non-null function pointer")();
            return;
        } else {
            prev = &mut (*cmd).next
        }
    }
    // check cvars
    if crate::src::qcommon::cvar::Cvar_Command() as u64 != 0 {
        return;
    }
    // check client game commands
    if !crate::src::qcommon::common::com_cl_running.is_null()
        && (*crate::src::qcommon::common::com_cl_running).integer != 0
        && crate::src::client::cl_cgame::CL_GameCommand() as u32 != 0
    {
        return;
    }
    // check server game commands
    if !crate::src::qcommon::common::com_sv_running.is_null()
        && (*crate::src::qcommon::common::com_sv_running).integer != 0
        && crate::src::server::sv_game::SV_GameCommand() as u32 != 0
    {
        return;
    }
    // check ui commands
    if !crate::src::qcommon::common::com_cl_running.is_null()
        && (*crate::src::qcommon::common::com_cl_running).integer != 0
        && crate::src::client::cl_ui::UI_GameCommand() as u32 != 0
    {
        return;
    }
    // send it as a server command if we are connected
    // this will usually result in a chat message
    crate::src::client::cl_main::CL_ForwardCommandToServer(text);
}
/*
============
Cmd_List_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_List_f() {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut i: i32 = 0;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() > 1 as i32 {
        match_0 = Cmd_Argv(1 as i32)
    } else {
        match_0 = 0 as *mut libc::c_char
    }
    i = 0 as i32;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if !(!match_0.is_null()
            && crate::src::qcommon::common::Com_Filter(
                match_0,
                (*cmd).name,
                crate::src::qcommon::q_shared::qfalse as i32,
            ) == 0)
        {
            crate::src::qcommon::common::Com_Printf(
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                (*cmd).name,
            );
            i += 1
        }
        cmd = (*cmd).next
    }
    crate::src::qcommon::common::Com_Printf(
        b"%i commands\n\x00" as *const u8 as *const libc::c_char,
        i,
    );
}
/*
==================
Cmd_CompleteCfgName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_CompleteCfgName(mut _args: *mut libc::c_char, mut argNum: i32) {
    if argNum == 2 as i32 {
        crate::src::qcommon::common::Field_CompleteFilename(
            b"\x00" as *const u8 as *const libc::c_char,
            b"cfg\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qtrue,
        );
    };
}
/*
============
Cmd_Init
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cmd_Init() {
    Cmd_AddCommand(
        b"cmdlist\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_List_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"exec\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_Exec_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"execq\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_Exec_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"exec\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_CompleteCfgName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"execq\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_CompleteCfgName as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> ()),
    );
    Cmd_AddCommand(
        b"vstr\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_Vstr_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_SetCommandCompletionFunc(
        b"vstr\x00" as *const u8 as *const libc::c_char,
        Some(
            crate::src::qcommon::cvar::Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: i32) -> (),
        ),
    );
    Cmd_AddCommand(
        b"echo\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_Echo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"wait\x00" as *const u8 as *const libc::c_char,
        Some(Cmd_Wait_f as unsafe extern "C" fn() -> ()),
    );
}

// =============== BEGIN l_precomp_h ================
pub type define_t = crate::src::botlib::l_precomp::define_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct define_s {
    pub name: *mut libc::c_char,
    pub flags: libc::c_int,
    pub builtin: libc::c_int,
    pub numparms: libc::c_int,
    pub parms: *mut crate::src::botlib::l_script::token_t,
    pub tokens: *mut crate::src::botlib::l_script::token_t,
    pub next: *mut crate::src::botlib::l_precomp::define_s,
    pub hashnext: *mut crate::src::botlib::l_precomp::define_s,
}

pub type indent_t = crate::src::botlib::l_precomp::indent_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct indent_s {
    pub type_0: libc::c_int,
    pub skip: libc::c_int,
    pub script: *mut crate::src::botlib::l_script::script_t,
    pub next: *mut crate::src::botlib::l_precomp::indent_s,
}

pub type source_t = crate::src::botlib::l_precomp::source_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct source_s {
    pub filename: [libc::c_char; 1024],
    pub includepath: [libc::c_char; 1024],
    pub punctuations: *mut crate::src::botlib::l_script::punctuation_t,
    pub scriptstack: *mut crate::src::botlib::l_script::script_t,
    pub tokens: *mut crate::src::botlib::l_script::token_t,
    pub defines: *mut crate::src::botlib::l_precomp::define_t,
    pub definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
    pub indentstack: *mut crate::src::botlib::l_precomp::indent_t,
    pub skip: libc::c_int,
    pub token: crate::src::botlib::l_script::token_t,
}
use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::time_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::EndOfScript;
pub use crate::src::botlib::l_script::FreeScript;
pub use crate::src::botlib::l_script::LoadScriptFile;
pub use crate::src::botlib::l_script::LoadScriptMemory;
pub use crate::src::botlib::l_script::PS_ReadToken;
pub use crate::src::botlib::l_script::PS_SetBaseFolder;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct operator_s {
    pub operator: libc::c_int,
    pub priority: libc::c_int,
    pub parentheses: libc::c_int,
    pub prev: *mut operator_s,
    pub next: *mut operator_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_s {
    pub intvalue: libc::c_long,
    pub floatvalue: libc::c_float,
    pub parentheses: libc::c_int,
    pub prev: *mut value_s,
    pub next: *mut value_s,
}

pub type value_t = value_s;
//end of the function PC_Directive_endif
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================

pub type operator_t = operator_s;
//directive name with parse function

pub type directive_t = directive_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct directive_s {
    pub name: *mut libc::c_char,
    pub func: Option<
        unsafe extern "C" fn(_: *mut crate::src::botlib::l_precomp::source_t) -> libc::c_int,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct builtin {
    pub string: *mut libc::c_char,
    pub builtin: libc::c_int,
}
#[no_mangle]

pub static mut numtokens: libc::c_int = 0;
/*
int tokenheapinitialized;				//true when the token heap is initialized
token_t token_heap[TOKEN_HEAP_SIZE];	//heap with tokens
token_t *freetokens;					//free tokens from the heap
*/
//list with global defines added to every source loaded
#[no_mangle]

pub static mut globaldefines: *mut crate::src::botlib::l_precomp::define_t = 0
    as *const crate::src::botlib::l_precomp::define_t
    as *mut crate::src::botlib::l_precomp::define_t;
//print a source error
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn SourceError(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        3 as libc::c_int,
        b"file %s, line %d: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*(*source).scriptstack).filename.as_mut_ptr(),
        (*(*source).scriptstack).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//print a source warning
//end of the function SourceError
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn SourceWarning(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut str: *mut libc::c_char,
    mut args: ...
) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2 as libc::c_int,
        b"file %s, line %d: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*(*source).scriptstack).filename.as_mut_ptr(),
        (*(*source).scriptstack).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//end of the function ScriptWarning
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PushIndent(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: libc::c_int,
    mut skip: libc::c_int,
) {
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    indent = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::indent_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::indent_t;
    (*indent).type_0 = type_0;
    (*indent).script = (*source).scriptstack;
    (*indent).skip = (skip != 0 as libc::c_int) as libc::c_int;
    (*source).skip += (*indent).skip;
    (*indent).next = (*source).indentstack;
    (*source).indentstack = indent;
}
//end of the function PC_PushIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PopIndent(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: *mut libc::c_int,
    mut skip: *mut libc::c_int,
) {
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    *type_0 = 0 as libc::c_int;
    *skip = 0 as libc::c_int;
    indent = (*source).indentstack;
    if indent.is_null() {
        return;
    }
    //must be an indent from the current script
    if (*(*source).indentstack).script != (*source).scriptstack {
        return;
    }
    *type_0 = (*indent).type_0;
    *skip = (*indent).skip;
    (*source).indentstack = (*(*source).indentstack).next;
    (*source).skip -= (*indent).skip;
    crate::src::botlib::l_memory::FreeMemory(indent as *mut libc::c_void);
}
//end of the function PC_PopIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PushScript(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut script: *mut crate::src::botlib::l_script::script_t,
) {
    let mut s: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t; //end for
    s = (*source).scriptstack;
    while !s.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            (*s).filename.as_mut_ptr(),
            (*script).filename.as_mut_ptr(),
        ) == 0
        {
            SourceError(
                source,
                b"%s recursively included\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*script).filename.as_mut_ptr(),
            );
            return;
        }
        s = (*s).next
        //end if
    }
    //push the script on the script stack
    (*script).next = (*source).scriptstack;
    (*source).scriptstack = script;
}
//end of the function PC_PushScript
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_InitTokenHeap() {
    /*
    int i;

    if (tokenheapinitialized) return;
    freetokens = NULL;
    for (i = 0; i < TOKEN_HEAP_SIZE; i++)
    {
        token_heap[i].next = freetokens;
        freetokens = &token_heap[i];
    } //end for
    tokenheapinitialized = qtrue;
    */
}
//end of the function PC_InitTokenHeap
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CopyToken(
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> *mut crate::src::botlib::l_script::token_t {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    //	t = (token_t *) malloc(sizeof(token_t));
    t = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_script::token_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_script::token_t;
    //	t = freetokens;
    if t.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"out of token space\x00" as *const u8 as *const libc::c_char,
        ); //end if
    }
    //	freetokens = freetokens->next;
    crate::stdlib::memcpy(
        t as *mut libc::c_void,
        token as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>() as libc::c_ulong,
    );
    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
    numtokens += 1;
    return t;
}
//end of the function PC_CopyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeToken(mut token: *mut crate::src::botlib::l_script::token_t) {
    //free(token);
    crate::src::botlib::l_memory::FreeMemory(token as *mut libc::c_void);
    //	token->next = freetokens;
    //	freetokens = token;
    numtokens -= 1;
}
//end of the function PC_FreeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadSourceToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    //if there's no token already available
    while (*source).tokens.is_null() {
        //end while
        //if there's a token to read from the script
        if crate::src::botlib::l_script::PS_ReadToken(
            (*source).scriptstack as *mut crate::src::botlib::l_script::script_s,
            token as *mut crate::src::botlib::l_script::token_s,
        ) != 0
        {
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
        //if at the end of the script
        if crate::src::botlib::l_script::EndOfScript(
            (*source).scriptstack as *mut crate::src::botlib::l_script::script_s,
        ) != 0
        {
            //end if
            //remove all indents of the script
            while !(*source).indentstack.is_null()
                && (*(*source).indentstack).script == (*source).scriptstack
            {
                SourceWarning(
                    source,
                    b"missing #endif\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                PC_PopIndent(source, &mut type_0, &mut skip);
            }
            //end if
        }
        //if this was the initial script
        if (*(*source).scriptstack).next.is_null() {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //remove the script and return to the last one
        script = (*source).scriptstack;
        (*source).scriptstack = (*(*source).scriptstack).next;
        crate::src::botlib::l_script::FreeScript(
            script as *mut crate::src::botlib::l_script::script_s,
        );
    }
    //copy the already available token
    crate::stdlib::memcpy(
        token as *mut libc::c_void,
        (*source).tokens as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>() as libc::c_ulong,
    );
    //free the read token
    t = (*source).tokens;
    (*source).tokens = (*(*source).tokens).next;
    PC_FreeToken(t);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_ReadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadSourceToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    t = PC_CopyToken(token);
    (*t).next = (*source).tokens;
    (*source).tokens = t;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_UnreadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDefineParms(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut parms: *mut *mut crate::src::botlib::l_script::token_t,
    mut maxparms: libc::c_int,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut i: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut lastcomma: libc::c_int = 0;
    let mut numparms: libc::c_int = 0;
    let mut indent: libc::c_int = 0;
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"define %s missing parms\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*define).name,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    if (*define).numparms > maxparms {
        SourceError(
            source,
            b"define with more than %d parameters\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            maxparms,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    i = 0 as libc::c_int;
    while i < (*define).numparms {
        let ref mut fresh0 = *parms.offset(i as isize);
        *fresh0 = 0 as *mut crate::src::botlib::l_script::token_t;
        i += 1
    }
    //if no leading "("
    if ::libc::strcmp(
        token.string.as_mut_ptr(),
        b"(\x00" as *const u8 as *const libc::c_char,
    ) != 0
    {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"define %s missing parms\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*define).name,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //read the define parameters
    done = 0 as libc::c_int; //end for
    numparms = 0 as libc::c_int; //end if
    indent = 0 as libc::c_int; //end if
    while done == 0 {
        if numparms >= maxparms {
            SourceError(
                source,
                b"define %s with too many parms\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*define).name,
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if numparms >= (*define).numparms {
            SourceWarning(
                source,
                b"define %s has too many parms\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*define).name,
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        let ref mut fresh1 = *parms.offset(numparms as isize);
        *fresh1 = 0 as *mut crate::src::botlib::l_script::token_t;
        lastcomma = 1 as libc::c_int;
        last = 0 as *mut crate::src::botlib::l_script::token_t;
        //end if
        while done == 0 {
            if PC_ReadSourceToken(source, &mut token) == 0 {
                SourceError(
                    source,
                    b"define %s incomplete\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*define).name,
                ); //end while
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            //
            //end if
            //
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b",\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if indent <= 0 as libc::c_int {
                    if lastcomma != 0 {
                        SourceWarning(
                            source,
                            b"too many comma\'s\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                    }
                    break;
                }
                //end if
            }
            lastcomma = 0 as libc::c_int;
            if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"(\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                //
                //end if
                indent += 1
            } else {
                if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b")\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    indent -= 1;
                    if indent <= 0 as libc::c_int {
                        //end if
                        //end if
                        if (*parms.offset(((*define).numparms - 1 as libc::c_int) as isize))
                            .is_null()
                        {
                            SourceWarning(
                                source,
                                b"too few define parms\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ); //end if
                        }
                        done = 1 as libc::c_int;
                        break;
                    }
                }
                //
                if numparms < (*define).numparms {
                    //
                    t = PC_CopyToken(&mut token);
                    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                    if !last.is_null() {
                        (*last).next = t
                    } else {
                        let ref mut fresh2 = *parms.offset(numparms as isize);
                        *fresh2 = t
                    }
                    last = t
                }
            }
        }
        numparms += 1
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_ReadDefineParms
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_StringizeTokens(
    mut tokens: *mut crate::src::botlib::l_script::token_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end for
    (*token).type_0 = 1 as libc::c_int;
    (*token).whitespace_p = 0 as *mut libc::c_char;
    (*token).endwhitespace_p = 0 as *mut libc::c_char;
    (*token).string[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    ::libc::strcat(
        (*token).string.as_mut_ptr(),
        b"\"\x00" as *const u8 as *const libc::c_char,
    );
    t = tokens;
    while !t.is_null() {
        crate::stdlib::strncat(
            (*token).string.as_mut_ptr(),
            (*t).string.as_mut_ptr(),
            (1024 as libc::c_int as libc::c_ulong)
                .wrapping_sub(crate::stdlib::strlen((*token).string.as_mut_ptr()))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        t = (*t).next
    }
    crate::stdlib::strncat(
        (*token).string.as_mut_ptr(),
        b"\"\x00" as *const u8 as *const libc::c_char,
        (1024 as libc::c_int as libc::c_ulong)
            .wrapping_sub(crate::stdlib::strlen((*token).string.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_StringizeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_MergeTokens(
    mut t1: *mut crate::src::botlib::l_script::token_t,
    mut t2: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    //merging of a name with a name or number
    if (*t1).type_0 == 4 as libc::c_int
        && ((*t2).type_0 == 4 as libc::c_int || (*t2).type_0 == 3 as libc::c_int)
    {
        ::libc::strcat((*t1).string.as_mut_ptr(), (*t2).string.as_mut_ptr()); //end if
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //merging of two strings
    if (*t1).type_0 == 1 as libc::c_int && (*t2).type_0 == 1 as libc::c_int {
        //end if
        //remove trailing double quote
        (*t1).string[crate::stdlib::strlen((*t1).string.as_mut_ptr())
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
            '\u{0}' as i32 as libc::c_char;
        //concat without leading double quote
        ::libc::strcat(
            (*t1).string.as_mut_ptr(),
            &mut *(*t2).string.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //FIXME: merging of two number of the same sub type
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_MergeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
/*
void PC_PrintDefine(define_t *define)
{
    printf("define->name = %s\n", define->name);
    printf("define->flags = %d\n", define->flags);
    printf("define->builtin = %d\n", define->builtin);
    printf("define->numparms = %d\n", define->numparms);
//	token_t *parms;					//define parameters
//	token_t *tokens;					//macro tokens (possibly containing parm tokens)
//	struct define_s *next;			//next defined macro in a list
} //end of the function PC_PrintDefine*/
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PrintDefineHashTable(
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
) {
    let mut i: libc::c_int = 0; //end for
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        crate::src::botlib::l_log::Log_Write(
            b"%4d:\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
        );
        d = *definehash.offset(i as isize);
        while !d.is_null() {
            crate::src::botlib::l_log::Log_Write(
                b" %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*d).name,
            );
            d = (*d).hashnext
        }
        crate::src::botlib::l_log::Log_Write(
            b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        i += 1
    }
    //end for
}
//end of the function PC_PrintDefineHashTable
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
//char primes[16] = {1, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 41, 43, 47};
#[no_mangle]

pub unsafe extern "C" fn PC_NameHash(mut name: *mut libc::c_char) -> libc::c_int {
    let mut hash: libc::c_int = 0; //end while
    let mut i: libc::c_int = 0;
    hash = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *name.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        hash += *name.offset(i as isize) as libc::c_int * (119 as libc::c_int + i);
        i += 1
        //hash += (name[i] << 7) + i;
        //hash += (name[i] << (i&15));
    }
    hash = (hash ^ hash >> 10 as libc::c_int ^ hash >> 20 as libc::c_int)
        & 1024 as libc::c_int - 1 as libc::c_int;
    return hash;
}
//end of the function PC_NameHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddDefineToHash(
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
) {
    let mut hash: libc::c_int = 0;
    hash = PC_NameHash((*define).name);
    (*define).hashnext = *definehash.offset(hash as isize);
    let ref mut fresh3 = *definehash.offset(hash as isize);
    *fresh3 = define;
}
//end of the function PC_AddDefineToHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindHashedDefine(
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut libc::c_char,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end for
    let mut hash: libc::c_int = 0;
    hash = PC_NameHash(name);
    d = *definehash.offset(hash as isize);
    while !d.is_null() {
        if ::libc::strcmp((*d).name, name) == 0 {
            return d;
        }
        d = (*d).hashnext
    }
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//end of the function PC_FindHashedDefine
//DEFINEHASHING
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindDefine(
    mut defines: *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut libc::c_char,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end for
    d = defines;
    while !d.is_null() {
        if ::libc::strcmp((*d).name, name) == 0 {
            return d;
        }
        d = (*d).next
    }
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//end of the function PC_FindDefine
//============================================================================
//
// Parameter:				-
// Returns:					number of the parm
//								if no parm found with the given name -1 is returned
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindDefineParm(
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end for
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    p = (*define).parms;
    while !p.is_null() {
        if ::libc::strcmp((*p).string.as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1;
        p = (*p).next
    }
    return -(1 as libc::c_int);
}
//end of the function PC_FindDefineParm
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeDefine(mut define: *mut crate::src::botlib::l_precomp::define_t) {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut next: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    //free the define parameters
    t = (*define).parms; //end for
    while !t.is_null() {
        next = (*t).next;
        PC_FreeToken(t);
        t = next
    }
    //free the define tokens
    t = (*define).tokens; //end for
    while !t.is_null() {
        next = (*t).next;
        PC_FreeToken(t);
        t = next
    }
    //free the define
    crate::src::botlib::l_memory::FreeMemory((*define).name as *mut libc::c_void);
    crate::src::botlib::l_memory::FreeMemory(define as *mut libc::c_void);
}
//add builtin defines
//end of the function PC_FreeDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddBuiltinDefines(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    let mut i: libc::c_int = 0;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut builtin_0: [builtin; 5] = [
        {
            let mut init = builtin {
                string: b"__LINE__\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                builtin: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__FILE__\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                builtin: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__DATE__\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                builtin: 3 as libc::c_int,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__TIME__\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                builtin: 4 as libc::c_int,
            };
            init
        },
        {
            let mut init = builtin {
                string: 0 as *mut libc::c_char,
                builtin: 0 as libc::c_int,
            };
            init
        },
    ];
    i = 0 as libc::c_int;
    while !builtin_0[i as usize].string.is_null() {
        define = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
            crate::src::botlib::l_precomp::define_t,
        >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::define_t;
        crate::stdlib::memset(
            define as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<crate::src::botlib::l_precomp::define_t>() as libc::c_ulong,
        );
        (*define).name = crate::src::botlib::l_memory::GetMemory(
            crate::stdlib::strlen(builtin_0[i as usize].string)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        ::libc::strcpy((*define).name, builtin_0[i as usize].string);
        (*define).flags |= 0x1 as libc::c_int;
        (*define).builtin = builtin_0[i as usize].builtin;
        //DEFINEHASHING
        PC_AddDefineToHash(define, (*source).definehash);
        i += 1
    }
    //add the define to the source
    //end for
}
//end of the function PC_AddBuiltinDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandBuiltinDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut firsttoken: *mut *mut crate::src::botlib::l_script::token_t,
    mut lasttoken: *mut *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end switch
    let mut t: crate::stdlib::time_t = 0;
    let mut curtime: *mut libc::c_char = 0 as *mut libc::c_char;
    token = PC_CopyToken(deftoken);
    match (*define).builtin {
        1 => {
            ::libc::sprintf(
                (*token).string.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*deftoken).line,
            );
            (*token).intvalue = (*deftoken).line as libc::c_ulong;
            (*token).floatvalue = (*deftoken).line as libc::c_float;
            //end case
            //NUMBERVALUE
            (*token).type_0 = 3 as libc::c_int; //end case
            (*token).subtype = 0x8 as libc::c_int | 0x1000 as libc::c_int; //end case
            *firsttoken = token; //end case
            *lasttoken = token
        }
        2 => {
            ::libc::strcpy(
                (*token).string.as_mut_ptr(),
                (*(*source).scriptstack).filename.as_mut_ptr(),
            );
            (*token).type_0 = 4 as libc::c_int;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        3 => {
            t = ::libc::time(0 as *mut crate::stdlib::time_t);
            curtime = crate::stdlib::ctime(&mut t);
            ::libc::strcpy(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
            crate::stdlib::strncat(
                (*token).string.as_mut_ptr(),
                curtime.offset(4 as libc::c_int as isize),
                7 as libc::c_int as libc::c_ulong,
            );
            crate::stdlib::strncat(
                (*token)
                    .string
                    .as_mut_ptr()
                    .offset(7 as libc::c_int as isize),
                curtime.offset(20 as libc::c_int as isize),
                4 as libc::c_int as libc::c_ulong,
            );
            ::libc::strcat(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
            ::libc::free(curtime as *mut libc::c_void);
            (*token).type_0 = 4 as libc::c_int;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        4 => {
            t = ::libc::time(0 as *mut crate::stdlib::time_t);
            curtime = crate::stdlib::ctime(&mut t);
            ::libc::strcpy(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
            crate::stdlib::strncat(
                (*token).string.as_mut_ptr(),
                curtime.offset(11 as libc::c_int as isize),
                8 as libc::c_int as libc::c_ulong,
            );
            ::libc::strcat(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
            ::libc::free(curtime as *mut libc::c_void);
            (*token).type_0 = 4 as libc::c_int;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        5 | _ => {
            *firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
            *lasttoken = 0 as *mut crate::src::botlib::l_script::token_t
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_ExpandBuiltinDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut firsttoken: *mut *mut crate::src::botlib::l_script::token_t,
    mut lasttoken: *mut *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut parms: [*mut crate::src::botlib::l_script::token_t; 128] = [
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
    ];
    let mut dt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut pt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t1: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t2: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut first: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nextpt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut parmnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    //if it is a builtin define
    if (*define).builtin != 0 {
        return PC_ExpandBuiltinDefine(source, deftoken, define, firsttoken, lasttoken);
    } //end if
      //if the define has parameters
    if (*define).numparms != 0 {
        if PC_ReadDefineParms(source, define, parms.as_mut_ptr(), 128 as libc::c_int) == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        } //end if
          //DEBUG_EVAL
    }
    //empty list at first
    first = 0 as *mut crate::src::botlib::l_script::token_t;
    last = 0 as *mut crate::src::botlib::l_script::token_t;
    let mut current_block_41: u64;
    //create a list with tokens of the expanded define
    dt = (*define).tokens; //end for
    while !dt.is_null() {
        parmnum = -(1 as libc::c_int);
        //end else
        //if the token is a name, it could be a define parameter
        if (*dt).type_0 == 4 as libc::c_int {
            parmnum = PC_FindDefineParm(define, (*dt).string.as_mut_ptr())
        } //end if
          //if it is a define parameter
        if parmnum >= 0 as libc::c_int {
            pt = parms[parmnum as usize];
            while !pt.is_null() {
                t = PC_CopyToken(pt);
                //add the token to the list
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !last.is_null() {
                    (*last).next = t
                } else {
                    first = t
                }
                last = t;
                pt = (*pt).next
            }
        //end for
        } else {
            //if stringizing operator
            if (*dt).string[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                && (*dt).string[1 as libc::c_int as usize] as libc::c_int == '\u{0}' as i32
            {
                //end else
                //the stringizing operator must be followed by a define parameter
                if !(*dt).next.is_null() {
                    parmnum = PC_FindDefineParm(define, (*(*dt).next).string.as_mut_ptr())
                } else {
                    parmnum = -(1 as libc::c_int)
                }
                //end if
                if parmnum >= 0 as libc::c_int {
                    //
                    dt = (*dt).next; //end if
                                     //step over the stringizing operator
                                     //stringize the define parameter tokens
                    if PC_StringizeTokens(parms[parmnum as usize], &mut token) == 0 {
                        SourceError(
                            source,
                            b"can\'t stringize tokens\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    t = PC_CopyToken(&mut token);
                    current_block_41 = 13131896068329595644;
                } else {
                    SourceWarning(
                        source,
                        b"stringizing operator without define parameter\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    current_block_41 = 1917311967535052937;
                }
            } else {
                t = PC_CopyToken(dt);
                current_block_41 = 13131896068329595644;
            }
            match current_block_41 {
                1917311967535052937 => {}
                _ => {
                    //add the token to the list
                    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                    if !last.is_null() {
                        (*last).next = t
                    } else {
                        first = t
                    }
                    last = t
                }
            }
        }
        dt = (*dt).next
    }
    //check for the merging operator
    t = first; //end for
    while !t.is_null() {
        if !(*t).next.is_null() {
            //end if
            //if the merging operator
            if (*(*t).next).string[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                && (*(*t).next).string[1 as libc::c_int as usize] as libc::c_int == '#' as i32
            {
                t1 = t;
                t2 = (*(*t).next).next;
                if !t2.is_null() {
                    //end if
                    if PC_MergeTokens(t1, t2) == 0 {
                        SourceError(
                            source,
                            b"can\'t merge %s with %s\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*t1).string.as_mut_ptr(),
                            (*t2).string.as_mut_ptr(),
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    PC_FreeToken((*t1).next);
                    (*t1).next = (*t2).next;
                    if t2 == last {
                        last = t1
                    }
                    PC_FreeToken(t2);
                    continue;
                }
            }
            //end if
        }
        t = (*t).next
    }
    //store the first and last token of the list
    *firsttoken = first;
    *lasttoken = last;
    //free all the parameter tokens
    i = 0 as libc::c_int; //end for
    while i < (*define).numparms {
        pt = parms[i as usize];
        while !pt.is_null() {
            nextpt = (*pt).next;
            PC_FreeToken(pt);
            pt = nextpt
        }
        i += 1
        //end for
    }
    //
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_ExpandDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandDefineIntoSource(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
) -> libc::c_int {
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end if
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    if PC_ExpandDefine(source, deftoken, define, &mut firsttoken, &mut lasttoken) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if !firsttoken.is_null() && !lasttoken.is_null() {
        (*lasttoken).next = (*source).tokens;
        (*source).tokens = firsttoken;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_ExpandDefineIntoSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ConvertPath(mut path: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    //remove double path seperators
    ptr = path; //end while
    while *ptr != 0 {
        if (*ptr as libc::c_int == '\\' as i32 || *ptr as libc::c_int == '/' as i32)
            && (*ptr.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                || *ptr.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            crate::stdlib::memmove(
                ptr as *mut libc::c_void,
                ptr.offset(1 as libc::c_int as isize) as *const libc::c_void,
                crate::stdlib::strlen(ptr),
            ); //end if
        } else {
            ptr = ptr.offset(1)
        }
        //end else
    }
    //set OS dependent path seperators
    ptr = path;
    while *ptr != 0 {
        if *ptr as libc::c_int == '/' as i32 || *ptr as libc::c_int == '\\' as i32 {
            *ptr = '/' as i32 as libc::c_char
        }
        ptr = ptr.offset(1)
    }
    //end while
}
//end of the function PC_ConvertPath
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_include(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut path: [libc::c_char; 64] = [0; 64];
    //QUAKE
    if (*source).skip > 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if token.linescrossed > 0 as libc::c_int {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end else
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if token.type_0 == 1 as libc::c_int {
        crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
        PC_ConvertPath(token.string.as_mut_ptr());
        script = crate::src::botlib::l_script::LoadScriptFile(token.string.as_mut_ptr())
            as *mut crate::src::botlib::l_script::script_s;
        if script.is_null() {
            crate::src::qcommon::q_shared::Q_strncpyz(
                path.as_mut_ptr(),
                (*source).includepath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            crate::src::qcommon::q_shared::Q_strcat(
                path.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                token.string.as_mut_ptr(),
            );
            script = crate::src::botlib::l_script::LoadScriptFile(path.as_mut_ptr())
                as *mut crate::src::botlib::l_script::script_s
        }
    //end if
    } else if token.type_0 == 5 as libc::c_int
        && *token.string.as_mut_ptr() as libc::c_int == '<' as i32
    {
        crate::src::qcommon::q_shared::Q_strncpyz(
            path.as_mut_ptr(),
            (*source).includepath.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ); //end if
        while PC_ReadSourceToken(source, &mut token) != 0 {
            //end while
            if token.linescrossed > 0 as libc::c_int {
                PC_UnreadSourceToken(source, &mut token); //end if
                break; //end if
            } else {
                if token.type_0 == 5 as libc::c_int
                    && *token.string.as_mut_ptr() as libc::c_int == '>' as i32
                {
                    break; //end if
                }
                crate::src::qcommon::q_shared::Q_strcat(
                    path.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    token.string.as_mut_ptr(),
                );
            }
        }
        if *token.string.as_mut_ptr() as libc::c_int != '>' as i32 {
            SourceWarning(
                source,
                b"#include missing trailing >\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if crate::stdlib::strlen(path.as_mut_ptr()) == 0 {
            SourceError(
                source,
                b"#include without file name between < >\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        PC_ConvertPath(path.as_mut_ptr());
        script = crate::src::botlib::l_script::LoadScriptFile(path.as_mut_ptr())
            as *mut crate::src::botlib::l_script::script_s
    } else {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //QUAKE
    if script.is_null() {
        SourceError(
            source,
            b"file %s not found\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            path.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int; //end if
                                                                     //SCREWUP
    }
    PC_PushScript(source, script);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//read a token only if on the same line, lines are concatenated with a slash
//end of the function PC_Directive_include
//============================================================================
// reads a token from the current line, continues reading on the next
// line only if a backslash '\' is encountered.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadLine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut crossline: libc::c_int = 0; //end if
    crossline = 0 as libc::c_int;
    loop {
        if PC_ReadSourceToken(source, token) == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if (*token).linescrossed > crossline {
            PC_UnreadSourceToken(source, token);
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        crossline = 1 as libc::c_int;
        if !(::libc::strcmp(
            (*token).string.as_mut_ptr(),
            b"\\\x00" as *const u8 as *const libc::c_char,
        ) == 0)
        {
            break;
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//returns true if there was a white space in front of the token
//end of the function PC_ReadLine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_WhiteSpaceBeforeToken(
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    return ((*token)
        .endwhitespace_p
        .wrapping_offset_from((*token).whitespace_p) as libc::c_long
        > 0 as libc::c_int as libc::c_long) as libc::c_int;
}
//end of the function PC_WhiteSpaceBeforeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ClearTokenWhiteSpace(
    mut token: *mut crate::src::botlib::l_script::token_t,
) {
    (*token).whitespace_p = 0 as *mut libc::c_char;
    (*token).endwhitespace_p = 0 as *mut libc::c_char;
    (*token).linescrossed = 0 as libc::c_int;
}
//end of the function PC_ClearTokenWhiteSpace
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_undef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut lastdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut hash: libc::c_int = 0;
    if (*source).skip > 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(
            source,
            b"undef without name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if token.type_0 != 4 as libc::c_int {
        PC_UnreadSourceToken(source, &mut token); //end for
        SourceError(
            source,
            b"expected name, found %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token.string.as_mut_ptr(),
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end else
    hash = PC_NameHash(token.string.as_mut_ptr()); //end if
    lastdefine = 0 as *mut crate::src::botlib::l_precomp::define_t;
    define = *(*source).definehash.offset(hash as isize);
    while !define.is_null() {
        if ::libc::strcmp((*define).name, token.string.as_mut_ptr()) == 0 {
            if (*define).flags & 0x1 as libc::c_int != 0 {
                SourceWarning(
                    source,
                    b"can\'t undef %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr(),
                );
            } else {
                if !lastdefine.is_null() {
                    (*lastdefine).hashnext = (*define).hashnext
                } else {
                    let ref mut fresh4 = *(*source).definehash.offset(hash as isize);
                    *fresh4 = (*define).hashnext
                }
                PC_FreeDefine(define);
            }
            break;
        } else {
            lastdefine = define;
            define = (*define).hashnext
        }
    }
    //DEFINEHASHING
    //DEFINEHASHING
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_undef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_define(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    if (*source).skip > 0 as libc::c_int {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(
            source,
            b"#define without name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if token.type_0 != 4 as libc::c_int {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(
            source,
            b"expected name after #define, found %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //check if the define already exists
    define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    //DEFINEHASHING
    if !define.is_null() {
        //end if
        if (*define).flags & 0x1 as libc::c_int != 0 {
            SourceError(
                source,
                b"can\'t redefine %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end if
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        SourceWarning(
            source,
            b"redefinition of %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            token.string.as_mut_ptr(),
        );
        //unread the define name before executing the #undef directive
        PC_UnreadSourceToken(source, &mut token);
        if PC_Directive_undef(source) == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
    }
    //allocate define
    define = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::define_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::define_t;
    crate::stdlib::memset(
        define as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::define_t>() as libc::c_ulong,
    );
    (*define).name = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen(token.string.as_mut_ptr())
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    ::libc::strcpy((*define).name, token.string.as_mut_ptr());
    //add the define to the source
    PC_AddDefineToHash(define, (*source).definehash);
    //DEFINEHASHING
    //DEFINEHASHING
    //if nothing is defined, just return
    if PC_ReadLine(source, &mut token) == 0 {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //if it is a define with parameters
    if PC_WhiteSpaceBeforeToken(&mut token) == 0
        && ::libc::strcmp(
            token.string.as_mut_ptr(),
            b"(\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        //end if
        //read the define parameters
        last = 0 as *mut crate::src::botlib::l_script::token_t; //end if
        if PC_CheckTokenString(
            source,
            b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            loop {
                if PC_ReadLine(source, &mut token) == 0 {
                    SourceError(
                        source,
                        b"expected define parameter\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                //end if
                //if it isn't a name
                if token.type_0 != 4 as libc::c_int {
                    SourceError(
                        source,
                        b"invalid define parameter\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                //
                if PC_FindDefineParm(define, token.string.as_mut_ptr()) >= 0 as libc::c_int {
                    SourceError(
                        source,
                        b"two the same define parameters\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                //add the define parm
                t = PC_CopyToken(&mut token);
                PC_ClearTokenWhiteSpace(t);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !last.is_null() {
                    (*last).next = t
                } else {
                    (*define).parms = t
                }
                last = t;
                (*define).numparms += 1;
                //read next token
                if PC_ReadLine(source, &mut token) == 0 {
                    SourceError(
                        source,
                        b"define parameters not terminated\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                //
                if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b")\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    break;
                }
                if ::libc::strcmp(
                    token.string.as_mut_ptr(),
                    b",\x00" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    SourceError(
                        source,
                        b"define not terminated\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
            }
            //then it must be a comma
            //end while
        }
        if PC_ReadLine(source, &mut token) == 0 {
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    //read the defined stuff
    last = 0 as *mut crate::src::botlib::l_script::token_t; //end if
    loop {
        t = PC_CopyToken(&mut token);
        if (*t).type_0 == 4 as libc::c_int
            && ::libc::strcmp((*t).string.as_mut_ptr(), (*define).name) == 0
        {
            SourceError(
                source,
                b"recursive define (removed recursion)\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            PC_ClearTokenWhiteSpace(t);
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !last.is_null() {
                (*last).next = t
            } else {
                (*define).tokens = t
            }
            last = t
        }
        if !(PC_ReadLine(source, &mut token) != 0) {
            break;
        }
    }
    //
    if !last.is_null() {
        //end if
        //check for merge operators at the beginning or end
        if ::libc::strcmp(
            (*(*define).tokens).string.as_mut_ptr(),
            b"##\x00" as *const u8 as *const libc::c_char,
        ) == 0
            || ::libc::strcmp(
                (*last).string.as_mut_ptr(),
                b"##\x00" as *const u8 as *const libc::c_char,
            ) == 0
        {
            SourceError(
                source,
                b"define with misplaced ##\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_define
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DefineFromString(
    mut string: *mut libc::c_char,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut src: crate::src::botlib::l_precomp::source_t =
        crate::src::botlib::l_precomp::source_t {
            filename: [0; 1024],
            includepath: [0; 1024],
            punctuations: 0 as *mut crate::src::botlib::l_script::punctuation_t,
            scriptstack: 0 as *mut crate::src::botlib::l_script::script_t,
            tokens: 0 as *mut crate::src::botlib::l_script::token_t,
            defines: 0 as *mut crate::src::botlib::l_precomp::define_t,
            definehash: 0 as *mut *mut crate::src::botlib::l_precomp::define_t,
            indentstack: 0 as *mut crate::src::botlib::l_precomp::indent_t,
            skip: 0,
            token: crate::src::botlib::l_script::token_t {
                string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut crate::src::botlib::l_script::token_s,
            },
        };
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut def: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptMemory(
        string,
        crate::stdlib::strlen(string) as libc::c_int,
        b"*extern\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut crate::src::botlib::l_script::script_s;
    //create a new source
    crate::stdlib::memset(
        &mut src as *mut crate::src::botlib::l_precomp::source_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>() as libc::c_ulong,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        src.filename.as_mut_ptr(),
        b"*extern\x00" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    src.scriptstack = script;
    src.definehash = crate::src::botlib::l_memory::GetClearedMemory(
        (1024 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            *mut crate::src::botlib::l_precomp::define_t,
        >() as libc::c_ulong),
    ) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    //create a define from the source
    res = PC_Directive_define(&mut src);
    //free any tokens if left
    t = src.tokens; //end for
    while !t.is_null() {
        src.tokens = (*src.tokens).next; //end for
        PC_FreeToken(t);
        t = src.tokens
    }
    def = 0 as *mut crate::src::botlib::l_precomp::define_t;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !(*src.definehash.offset(i as isize)).is_null() {
            def = *src.definehash.offset(i as isize);
            break;
        } else {
            i += 1
        }
        //end if
    }
    //DEFINEHASHING
    //
    crate::src::botlib::l_memory::FreeMemory(src.definehash as *mut libc::c_void);
    //DEFINEHASHING
    //
    crate::src::botlib::l_script::FreeScript(script as *mut crate::src::botlib::l_script::script_s);
    //if the define was created successfully
    if res > 0 as libc::c_int {
        return def;
    }
    //free the define is created
    if !src.defines.is_null() {
        PC_FreeDefine(def);
    }
    //
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//add a define to the source
//end of the function PC_DefineFromString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut libc::c_char,
) -> libc::c_int {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = PC_DefineFromString(string);
    if define.is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    PC_AddDefineToHash(define, (*source).definehash);
    //DEFINEHASHING
    //DEFINEHASHING
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//add a globals define that will be added to all opened sources
//end of the function PC_AddDefine
//============================================================================
// add a globals define that will be added to all opened sources
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddGlobalDefine(mut string: *mut libc::c_char) -> libc::c_int {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = PC_DefineFromString(string);
    if define.is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    (*define).next = globaldefines;
    globaldefines = define;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//remove the given global define
//end of the function PC_AddGlobalDefine
//============================================================================
// remove the given global define
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_RemoveGlobalDefine(mut name: *mut libc::c_char) -> libc::c_int {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end if
    define = PC_FindDefine(globaldefines, name);
    if !define.is_null() {
        PC_FreeDefine(define);
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//remove all globals defines
//end of the function PC_RemoveGlobalDefine
//============================================================================
// remove all globals defines
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_RemoveAllGlobalDefines() {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = globaldefines;
    while !define.is_null() {
        globaldefines = (*globaldefines).next;
        PC_FreeDefine(define);
        define = globaldefines
    }
    //end for
}
//end of the function PC_RemoveAllGlobalDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CopyDefine(
    mut _source: *mut crate::src::botlib::l_precomp::source_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut newdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut newtoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    newdefine = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::define_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::define_t;
    //copy the define name
    (*newdefine).name = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen((*define).name).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    ::libc::strcpy((*newdefine).name, (*define).name);
    (*newdefine).flags = (*define).flags;
    (*newdefine).builtin = (*define).builtin;
    (*newdefine).numparms = (*define).numparms;
    //the define is not linked
    (*newdefine).next = 0 as *mut crate::src::botlib::l_precomp::define_s;
    (*newdefine).hashnext = 0 as *mut crate::src::botlib::l_precomp::define_s;
    //copy the define tokens
    (*newdefine).tokens = 0 as *mut crate::src::botlib::l_script::token_t; //end for
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    token = (*define).tokens;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut crate::src::botlib::l_script::token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else {
            (*newdefine).tokens = newtoken
        }
        lasttoken = newtoken;
        token = (*token).next
    }
    //copy the define parameters
    (*newdefine).parms = 0 as *mut crate::src::botlib::l_script::token_t; //end for
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    token = (*define).parms;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut crate::src::botlib::l_script::token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else {
            (*newdefine).parms = newtoken
        }
        lasttoken = newtoken;
        token = (*token).next
    }
    return newdefine;
}
//end of the function PC_CopyDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddGlobalDefinesToSource(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut newdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = globaldefines;
    while !define.is_null() {
        newdefine = PC_CopyDefine(source, define);
        PC_AddDefineToHash(newdefine, (*source).definehash);
        define = (*define).next
        //DEFINEHASHING
        //DEFINEHASHING
    }
    //end for
}
//end of the function PC_AddGlobalDefinesToSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_if_def(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end if
    let mut skip: libc::c_int = 0;
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(
            source,
            b"#ifdef without name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if token.type_0 != 4 as libc::c_int {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(
            source,
            b"expected name after #ifdef, found %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    d = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    //DEFINEHASHING
    skip = ((type_0 == 0x8 as libc::c_int) as libc::c_int
        == (d == 0 as *mut libc::c_void as *mut crate::src::botlib::l_precomp::define_t)
            as libc::c_int) as libc::c_int;
    PC_PushIndent(source, type_0, skip);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directiveif_def
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_ifdef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    return PC_Directive_if_def(source, 0x8 as libc::c_int);
}
//end of the function PC_Directive_ifdef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_ifndef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    return PC_Directive_if_def(source, 0x10 as libc::c_int);
}
//end of the function PC_Directive_ifndef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_else(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0; //end if
    let mut skip: libc::c_int = 0; //end if
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 {
        SourceError(
            source,
            b"misplaced #else\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if type_0 == 0x2 as libc::c_int {
        SourceError(
            source,
            b"#else after #else\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    PC_PushIndent(source, 0x2 as libc::c_int, (skip == 0) as libc::c_int);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_else
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_endif(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut type_0: libc::c_int = 0; //end if
    let mut skip: libc::c_int = 0; //end switch
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 {
        SourceError(
            source,
            b"misplaced #endif\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn PC_OperatorPriority(mut op: libc::c_int) -> libc::c_int {
    match op {
        26 => return 15 as libc::c_int,
        27 => return 15 as libc::c_int,
        28 => return 15 as libc::c_int,
        29 => return 14 as libc::c_int,
        30 => return 14 as libc::c_int,
        5 => return 7 as libc::c_int,
        6 => return 6 as libc::c_int,
        7 => return 12 as libc::c_int,
        8 => return 12 as libc::c_int,
        9 => return 11 as libc::c_int,
        10 => return 11 as libc::c_int,
        36 => return 16 as libc::c_int,
        37 => return 12 as libc::c_int,
        38 => return 12 as libc::c_int,
        21 => return 13 as libc::c_int,
        22 => return 13 as libc::c_int,
        32 => return 10 as libc::c_int,
        33 => return 8 as libc::c_int,
        34 => return 9 as libc::c_int,
        35 => return 16 as libc::c_int,
        42 => return 5 as libc::c_int,
        43 => return 5 as libc::c_int,
        _ => {}
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//
#[no_mangle]

pub unsafe extern "C" fn PC_EvaluateTokens(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut tokens: *mut crate::src::botlib::l_script::token_t,
    mut intvalue: *mut libc::c_long,
    mut floatvalue: *mut libc::c_float,
    mut integer: libc::c_int,
) -> libc::c_int {
    let mut o: *mut operator_t = 0 as *mut operator_t;
    let mut firstoperator: *mut operator_t = 0 as *mut operator_t;
    let mut lastoperator: *mut operator_t = 0 as *mut operator_t;
    let mut v: *mut value_t = 0 as *mut value_t;
    let mut firstvalue: *mut value_t = 0 as *mut value_t;
    let mut lastvalue: *mut value_t = 0 as *mut value_t;
    let mut v1: *mut value_t = 0 as *mut value_t;
    let mut v2: *mut value_t = 0 as *mut value_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut brace: libc::c_int = 0 as libc::c_int;
    let mut parentheses: libc::c_int = 0 as libc::c_int;
    let mut error: libc::c_int = 0 as libc::c_int;
    let mut lastwasvalue: libc::c_int = 0 as libc::c_int;
    let mut negativevalue: libc::c_int = 0 as libc::c_int;
    let mut questmarkintvalue: libc::c_int = 0 as libc::c_int;
    let mut questmarkfloatvalue: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut gotquestmarkvalue: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    //
    let mut operator_heap: [operator_t; 64] = [operator_t {
        operator: 0,
        priority: 0,
        parentheses: 0,
        prev: 0 as *mut operator_s,
        next: 0 as *mut operator_s,
    }; 64]; //end for
    let mut numoperators: libc::c_int = 0 as libc::c_int; //end switch
    let mut value_heap: [value_t; 64] = [value_t {
        intvalue: 0,
        floatvalue: 0.,
        parentheses: 0,
        prev: 0 as *mut value_s,
        next: 0 as *mut value_s,
    }; 64]; //end if
    let mut numvalues: libc::c_int = 0 as libc::c_int; //end if
    lastoperator = 0 as *mut operator_t; //end if
    firstoperator = lastoperator; //end if
    lastvalue = 0 as *mut value_t;
    firstvalue = lastvalue;
    if !intvalue.is_null() {
        *intvalue = 0 as libc::c_int as libc::c_long
    }
    if !floatvalue.is_null() {
        *floatvalue = 0 as libc::c_int as libc::c_float
    }
    t = tokens;
    while !t.is_null() {
        let mut current_block_97: u64;
        match (*t).type_0 {
            4 => {
                if lastwasvalue != 0 || negativevalue != 0 {
                    SourceError(
                        source,
                        b"syntax error in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    error = 1 as libc::c_int
                } else if ::libc::strcmp(
                    (*t).string.as_mut_ptr(),
                    b"defined\x00" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    SourceError(
                        source,
                        b"undefined name %s in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*t).string.as_mut_ptr(),
                    );
                    error = 1 as libc::c_int
                } else {
                    t = (*t).next;
                    if ::libc::strcmp(
                        (*t).string.as_mut_ptr(),
                        b"(\x00" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        brace = crate::src::qcommon::q_shared::qtrue as libc::c_int;
                        t = (*t).next
                    }
                    if t.is_null() || (*t).type_0 != 4 as libc::c_int {
                        SourceError(
                            source,
                            b"defined without name in #if/#elif\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        error = 1 as libc::c_int
                    } else if numvalues >= 64 as libc::c_int {
                        SourceError(
                            source,
                            b"out of value space\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        error = 1 as libc::c_int
                    } else {
                        let fresh5 = numvalues;
                        numvalues = numvalues + 1;
                        v = &mut *value_heap.as_mut_ptr().offset(fresh5 as isize) as *mut value_t;
                        //v = (value_t *) GetClearedMemory(sizeof(value_t));
                        if !PC_FindHashedDefine((*source).definehash, (*t).string.as_mut_ptr())
                            .is_null()
                        {
                            //end else
                            //DEFINEHASHING
                            (*v).intvalue = 1 as libc::c_int as libc::c_long; //end if
                            (*v).floatvalue = 1 as libc::c_int as libc::c_float
                        } else {
                            (*v).intvalue = 0 as libc::c_int as libc::c_long; //end if
                            (*v).floatvalue = 0 as libc::c_int as libc::c_float
                        }
                        (*v).parentheses = parentheses;
                        (*v).next = 0 as *mut value_s;
                        (*v).prev = lastvalue;
                        if !lastvalue.is_null() {
                            (*lastvalue).next = v
                        } else {
                            firstvalue = v
                        }
                        lastvalue = v;
                        if brace != 0 {
                            t = (*t).next;
                            if t.is_null()
                                || ::libc::strcmp(
                                    (*t).string.as_mut_ptr(),
                                    b")\x00" as *const u8 as *const libc::c_char,
                                ) != 0
                            {
                                SourceError(
                                    source,
                                    b"defined without ) in #if/#elif\x00" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                error = 1 as libc::c_int;
                                current_block_97 = 2473505634946569239;
                            } else {
                                current_block_97 = 1854459640724737493;
                            }
                        //end if
                        } else {
                            current_block_97 = 1854459640724737493;
                        }
                        match current_block_97 {
                            2473505634946569239 => {}
                            _ => {
                                brace = crate::src::qcommon::q_shared::qfalse as libc::c_int;
                                // defined() creates a value
                                lastwasvalue = 1 as libc::c_int
                            }
                        }
                    }
                }
            }
            3 => {
                if lastwasvalue != 0 {
                    SourceError(
                        source,
                        b"syntax error in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    error = 1 as libc::c_int
                } else if numvalues >= 64 as libc::c_int {
                    SourceError(
                        source,
                        b"out of value space\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    error = 1 as libc::c_int
                } else {
                    let fresh6 = numvalues;
                    numvalues = numvalues + 1;
                    v = &mut *value_heap.as_mut_ptr().offset(fresh6 as isize) as *mut value_t;
                    //v = (value_t *) GetClearedMemory(sizeof(value_t));
                    if negativevalue != 0 {
                        //end else
                        (*v).intvalue = -((*t).intvalue as libc::c_int) as libc::c_long; //end if
                        (*v).floatvalue = -(*t).floatvalue
                    } else {
                        (*v).intvalue = (*t).intvalue as libc::c_long;
                        (*v).floatvalue = (*t).floatvalue
                    }
                    (*v).parentheses = parentheses;
                    (*v).next = 0 as *mut value_s;
                    (*v).prev = lastvalue;
                    if !lastvalue.is_null() {
                        (*lastvalue).next = v
                    } else {
                        firstvalue = v
                    }
                    lastvalue = v;
                    //last token was a value
                    lastwasvalue = 1 as libc::c_int;
                    //
                    negativevalue = 0 as libc::c_int
                }
            }
            5 => {
                if negativevalue != 0 {
                    SourceError(
                        source,
                        b"misplaced minus sign in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    error = 1 as libc::c_int
                } else if (*t).subtype == 44 as libc::c_int {
                    //end else if
                    parentheses += 1
                } else if (*t).subtype == 45 as libc::c_int {
                    parentheses -= 1; //end if
                    if parentheses < 0 as libc::c_int {
                        SourceError(
                            source,
                            b"too many ) in #if/#elsif\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ); //end if
                        error = 1 as libc::c_int
                    }
                } else {
                    //check for invalid operators on floating point values
                    if integer == 0 {
                        if (*t).subtype == 35 as libc::c_int
                            || (*t).subtype == 28 as libc::c_int
                            || (*t).subtype == 21 as libc::c_int
                            || (*t).subtype == 22 as libc::c_int
                            || (*t).subtype == 32 as libc::c_int
                            || (*t).subtype == 33 as libc::c_int
                            || (*t).subtype == 34 as libc::c_int
                        {
                            SourceError(
                                source,
                                b"illigal operator %s on floating point operands\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                (*t).string.as_mut_ptr(),
                            ); //end if
                            error = 1 as libc::c_int;
                            current_block_97 = 2473505634946569239;
                        } else {
                            current_block_97 = 6712462580143783635;
                        }
                    //end if
                    } else {
                        current_block_97 = 6712462580143783635; //end switch
                    } //end if
                    match current_block_97 {
                        2473505634946569239 => {}
                        _ => {
                            let mut current_block_80: u64; //end case
                            match (*t).subtype {
                                36 | 35 => {
                                    if lastwasvalue != 0 {
                                        SourceError(
                                            source,
                                            b"! or ~ after value in #if/#elif\x00" as *const u8
                                                as *const libc::c_char
                                                as *mut libc::c_char,
                                        ); //end case
                                        error = 1 as libc::c_int;
                                        current_block_80 = 10248984122780841972;
                                    } else {
                                        current_block_80 = 10248984122780841972;
                                    }
                                }
                                16 | 17 => {
                                    SourceError(
                                        source,
                                        b"++ or -- used in #if/#elif\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    current_block_80 = 10248984122780841972;
                                }
                                30 => {
                                    if lastwasvalue == 0 {
                                        negativevalue = 1 as libc::c_int;
                                        current_block_80 = 10248984122780841972;
                                    } else {
                                        current_block_80 = 5706227035632243100;
                                    }
                                    //end if
                                }
                                26 | 27 | 28 | 29 | 5 | 6 | 7 | 8 | 9 | 10 | 37 | 38 | 21 | 22
                                | 32 | 33 | 34 | 42 | 43 => {
                                    current_block_80 = 5706227035632243100; //end if
                                }
                                _ => {
                                    SourceError(
                                        source,
                                        b"invalid operator %s in #if/#elif\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                        (*t).string.as_mut_ptr(),
                                    ); //end if
                                    error = 1 as libc::c_int;
                                    current_block_80 = 10248984122780841972;
                                }
                            }
                            match current_block_80 {
                                5706227035632243100 => {
                                    if lastwasvalue == 0 {
                                        SourceError(
                                            source,
                                            b"operator %s after operator in #if/#elif\x00"
                                                as *const u8
                                                as *const libc::c_char
                                                as *mut libc::c_char,
                                            (*t).string.as_mut_ptr(),
                                        );
                                        error = 1 as libc::c_int
                                    }
                                }
                                _ => {}
                            }
                            if error == 0 && negativevalue == 0 {
                                //o = (operator_t *) GetClearedMemory(sizeof(operator_t));
                                if numoperators >= 64 as libc::c_int {
                                    SourceError(
                                        source,
                                        b"out of operator space\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    ); //end if
                                    error = 1 as libc::c_int
                                } else {
                                    let fresh7 = numoperators; //end if
                                    numoperators = numoperators + 1;
                                    o = &mut *operator_heap.as_mut_ptr().offset(fresh7 as isize)
                                        as *mut operator_t;
                                    (*o).operator = (*t).subtype;
                                    (*o).priority = PC_OperatorPriority((*t).subtype);
                                    (*o).parentheses = parentheses;
                                    (*o).next = 0 as *mut operator_s;
                                    (*o).prev = lastoperator;
                                    if !lastoperator.is_null() {
                                        (*lastoperator).next = o
                                    } else {
                                        firstoperator = o
                                    }
                                    lastoperator = o;
                                    lastwasvalue = 0 as libc::c_int
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                SourceError(
                    source,
                    b"unknown %s in #if/#elif\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*t).string.as_mut_ptr(),
                );
                error = 1 as libc::c_int
            }
        }
        if error != 0 {
            break;
        }
        t = (*t).next
    }
    if error == 0 {
        if lastwasvalue == 0 {
            SourceError(
                source,
                b"trailing operator in #if/#elif\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            error = 1 as libc::c_int
        } else if parentheses != 0 {
            SourceError(
                source,
                b"too many ( in #if/#elif\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            error = 1 as libc::c_int
        }
        //end else if
    }
    //
    gotquestmarkvalue = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    questmarkintvalue = 0 as libc::c_int;
    questmarkfloatvalue = 0 as libc::c_int as libc::c_float;
    //while there are operators
    while error == 0 && !firstoperator.is_null() {
        v = firstvalue; //end while
        o = firstoperator; //end for
        while !(*o).next.is_null() {
            //if the current operator is nested deeper in parentheses
            //than the next operator
            if (*o).parentheses > (*(*o).next).parentheses {
                break;
            }
            //if the current and next operator are nested equally deep in parentheses
            if (*o).parentheses == (*(*o).next).parentheses {
                //end if
                //if the priority of the current operator is equal or higher
                //than the priority of the next operator
                if (*o).priority >= (*(*o).next).priority {
                    break;
                }
            }
            //if the arity of the operator isn't equal to 1
            if (*o).operator != 36 as libc::c_int && (*o).operator != 35 as libc::c_int {
                v = (*v).next
            }
            //if there's no value or no next value
            if v.is_null() {
                SourceError(
                    source,
                    b"mising values in #if/#elif\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                error = 1 as libc::c_int;
                break;
            } else {
                o = (*o).next
            }
            //end if
        }
        if error != 0 {
            break;
        }
        v1 = v;
        v2 = (*v).next;
        //DEBUG_EVAL
        match (*o).operator {
            36 => {
                (*v1).intvalue = ((*v1).intvalue == 0) as libc::c_int as libc::c_long;
                (*v1).floatvalue = ((*v1).floatvalue == 0.) as libc::c_int as libc::c_float
                //end switch
                //end if
            }
            35 => (*v1).intvalue = !(*v1).intvalue,
            26 => {
                (*v1).intvalue *= (*v2).intvalue; //end if
                (*v1).floatvalue *= (*v2).floatvalue
            }
            27 => {
                if (*v2).intvalue == 0 || (*v2).floatvalue == 0. {
                    SourceError(
                        source,
                        b"divide by zero in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end else
                    error = 1 as libc::c_int
                } else {
                    (*v1).intvalue /= (*v2).intvalue; //end if
                    (*v1).floatvalue /= (*v2).floatvalue
                }
            }
            28 => {
                if (*v2).intvalue == 0 {
                    SourceError(
                        source,
                        b"divide by zero in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                    error = 1 as libc::c_int
                } else {
                    (*v1).intvalue %= (*v2).intvalue
                }
            }
            29 => {
                (*v1).intvalue += (*v2).intvalue;
                (*v1).floatvalue += (*v2).floatvalue
            }
            30 => {
                (*v1).intvalue -= (*v2).intvalue;
                (*v1).floatvalue -= (*v2).floatvalue
            }
            5 => {
                (*v1).intvalue =
                    ((*v1).intvalue != 0 && (*v2).intvalue != 0) as libc::c_int as libc::c_long;
                (*v1).floatvalue = ((*v1).floatvalue != 0. && (*v2).floatvalue != 0.) as libc::c_int
                    as libc::c_float
            }
            6 => {
                (*v1).intvalue =
                    ((*v1).intvalue != 0 || (*v2).intvalue != 0) as libc::c_int as libc::c_long;
                (*v1).floatvalue = ((*v1).floatvalue != 0. || (*v2).floatvalue != 0.) as libc::c_int
                    as libc::c_float
            }
            7 => {
                (*v1).intvalue = ((*v1).intvalue >= (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue >= (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            8 => {
                (*v1).intvalue = ((*v1).intvalue <= (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue <= (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            9 => {
                (*v1).intvalue = ((*v1).intvalue == (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue == (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            10 => {
                (*v1).intvalue = ((*v1).intvalue != (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue != (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            37 => {
                (*v1).intvalue = ((*v1).intvalue > (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue > (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            38 => {
                (*v1).intvalue = ((*v1).intvalue < (*v2).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue < (*v2).floatvalue) as libc::c_int as libc::c_float
            }
            21 => (*v1).intvalue >>= (*v2).intvalue,
            22 => (*v1).intvalue <<= (*v2).intvalue,
            32 => (*v1).intvalue &= (*v2).intvalue,
            33 => (*v1).intvalue |= (*v2).intvalue,
            34 => (*v1).intvalue ^= (*v2).intvalue,
            42 => {
                if gotquestmarkvalue == 0 {
                    SourceError(
                        source,
                        b": without ? in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    error = 1 as libc::c_int
                } else {
                    if integer != 0 {
                        if questmarkintvalue == 0 {
                            (*v1).intvalue = (*v2).intvalue
                        }
                    } else if questmarkfloatvalue == 0. {
                        (*v1).floatvalue = (*v2).floatvalue
                    }
                    gotquestmarkvalue = crate::src::qcommon::q_shared::qfalse as libc::c_int
                }
            }
            43 => {
                if gotquestmarkvalue != 0 {
                    SourceError(
                        source,
                        b"? after ? in #if/#elif\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    error = 1 as libc::c_int
                } else {
                    questmarkintvalue = (*v1).intvalue as libc::c_int;
                    questmarkfloatvalue = (*v1).floatvalue;
                    gotquestmarkvalue = crate::src::qcommon::q_shared::qtrue as libc::c_int
                }
            }
            _ => {}
        }
        //DEBUG_EVAL
        if error != 0 {
            break;
        }
        //if not an operator with arity 1
        if (*o).operator != 36 as libc::c_int && (*o).operator != 35 as libc::c_int {
            //end if
            //remove the second value if not question mark operator
            if (*o).operator != 43 as libc::c_int {
                v = (*v).next
            }
            //
            if !v.is_null() {
                if !(*v).prev.is_null() {
                    (*(*v).prev).next = (*v).next
                } else {
                    firstvalue = (*v).next
                }
                if !(*v).next.is_null() {
                    (*(*v).next).prev = (*v).prev
                }
            }
        }
        //remove the operator
        if !(*o).prev.is_null() {
            (*(*o).prev).next = (*o).next
        } else {
            firstoperator = (*o).next
        } //end if
        if !(*o).next.is_null() {
            (*(*o).next).prev = (*o).prev
        }
    } //end for
    if !firstvalue.is_null() {
        if !intvalue.is_null() {
            *intvalue = (*firstvalue).intvalue
        } //end for
        if !floatvalue.is_null() {
            *floatvalue = (*firstvalue).floatvalue
        }
    }
    o = firstoperator;
    while !o.is_null() {
        lastoperator = (*o).next;
        o = lastoperator
    }
    v = firstvalue;
    while !v.is_null() {
        lastvalue = (*v).next;
        v = lastvalue
    }
    if error == 0 {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    if !intvalue.is_null() {
        *intvalue = 0 as libc::c_int as libc::c_long
    }
    if !floatvalue.is_null() {
        *floatvalue = 0 as libc::c_int as libc::c_float
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_EvaluateTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Evaluate(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut intvalue: *mut libc::c_long,
    mut floatvalue: *mut libc::c_float,
    mut integer: libc::c_int,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nexttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut defined: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    if !intvalue.is_null() {
        *intvalue = 0 as libc::c_int as libc::c_long
    }
    if !floatvalue.is_null() {
        *floatvalue = 0 as libc::c_int as libc::c_float
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(
            source,
            b"no value after #if/#elif\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    loop {
        //if the token is a name
        if token.type_0 == 4 as libc::c_int {
            if defined != 0 {
                defined = crate::src::qcommon::q_shared::qfalse as libc::c_int; //end if
                t = PC_CopyToken(&mut token); //end if
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"defined\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                defined = crate::src::qcommon::q_shared::qtrue as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else {
                //then it must be a define
                define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
                //DEFINEHASHING
                if define.is_null() {
                    SourceError(
                        source,
                        b"can\'t evaluate %s, not defined\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        token.string.as_mut_ptr(),
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                if PC_ExpandDefineIntoSource(source, &mut token, define) == 0 {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
            }
        //end else
        } else if token.type_0 == 3 as libc::c_int || token.type_0 == 5 as libc::c_int {
            //if the token is a number or a punctuation
            t = PC_CopyToken(&mut token); //end else
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else {
                firsttoken = t
            }
            lasttoken = t
        } else {
            //can't evaluate the token
            SourceError(
                source,
                b"can\'t evaluate %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if !(PC_ReadLine(source, &mut token) != 0) {
            break;
        }
        //end else
    }
    //
    if PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue, integer) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    //DEBUG_EVAL
    t = firsttoken; //end for
    while !t.is_null() {
        //DEBUG_EVAL
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    //DEBUG_EVAL
    //
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Evaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarEvaluate(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut intvalue: *mut libc::c_long,
    mut floatvalue: *mut libc::c_float,
    mut integer: libc::c_int,
) -> libc::c_int {
    let mut indent: libc::c_int = 0;
    let mut defined: libc::c_int = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nexttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    if !intvalue.is_null() {
        *intvalue = 0 as libc::c_int as libc::c_long
    }
    if !floatvalue.is_null() {
        *floatvalue = 0 as libc::c_int as libc::c_float
    }
    //
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"no leading ( after $evalint/$evalfloat\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end if
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"nothing to evaluate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    indent = 1 as libc::c_int;
    firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    loop
    //if the token is a name
    {
        if token.type_0 == 4 as libc::c_int {
            if defined != 0 {
                defined = crate::src::qcommon::q_shared::qfalse as libc::c_int; //end if
                t = PC_CopyToken(&mut token); //end if
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else if ::libc::strcmp(
                token.string.as_mut_ptr(),
                b"defined\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                defined = crate::src::qcommon::q_shared::qtrue as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else {
                //then it must be a define
                define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
                //DEFINEHASHING
                if define.is_null() {
                    SourceError(
                        source,
                        b"can\'t evaluate %s, not defined\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        token.string.as_mut_ptr(),
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                if PC_ExpandDefineIntoSource(source, &mut token, define) == 0 {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
            }
        //end else
        } else if token.type_0 == 3 as libc::c_int || token.type_0 == 5 as libc::c_int {
            //if the token is a number or a punctuation
            if *token.string.as_mut_ptr() as libc::c_int == '(' as i32 {
                indent += 1
            } else if *token.string.as_mut_ptr() as libc::c_int == ')' as i32 {
                indent -= 1
            } //end else
            if indent <= 0 as libc::c_int {
                break;
            }
            t = PC_CopyToken(&mut token);
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else {
                firsttoken = t
            }
            lasttoken = t
        } else {
            //can't evaluate the token
            SourceError(
                source,
                b"can\'t evaluate %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if !(PC_ReadSourceToken(source, &mut token) != 0) {
            break;
        }
        //end else
    }
    //
    if PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue, integer) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    //DEBUG_EVAL
    t = firsttoken; //end for
    while !t.is_null() {
        //DEBUG_EVAL
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    //DEBUG_EVAL
    //
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_DollarEvaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_elif(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_long = 0; //end if
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 || type_0 == 0x2 as libc::c_int {
        SourceError(
            source,
            b"misplaced #elif\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut libc::c_float,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    skip = (value == 0 as libc::c_int as libc::c_long) as libc::c_int;
    PC_PushIndent(source, 0x4 as libc::c_int, skip);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_elif
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_if(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_long = 0;
    let mut skip: libc::c_int = 0;
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut libc::c_float,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    skip = (value == 0 as libc::c_int as libc::c_long) as libc::c_int;
    PC_PushIndent(source, 0x1 as libc::c_int, skip);
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_line(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    SourceError(
        source,
        b"#line directive not supported\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_Directive_line
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_error(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    ::libc::strcpy(
        token.string.as_mut_ptr(),
        b"\x00" as *const u8 as *const libc::c_char,
    );
    PC_ReadSourceToken(source, &mut token);
    SourceError(
        source,
        b"#error directive: %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_Directive_error
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_pragma(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    SourceWarning(
        source,
        b"#pragma directive not supported\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    while PC_ReadLine(source, &mut token) != 0 {}
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_pragma
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn UnreadSignToken(mut source: *mut crate::src::botlib::l_precomp::source_t) {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0 as libc::c_int;
    ::libc::strcpy(
        token.string.as_mut_ptr(),
        b"-\x00" as *const u8 as *const libc::c_char,
    );
    token.type_0 = 5 as libc::c_int;
    token.subtype = 30 as libc::c_int;
    PC_UnreadSourceToken(source, &mut token);
}
//end of the function UnreadSignToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_eval(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_long = 0;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut libc::c_float,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0 as libc::c_int;
    ::libc::sprintf(
        token.string.as_mut_ptr(),
        b"%ld\x00" as *const u8 as *const libc::c_char,
        ::libc::labs(value),
    );
    token.type_0 = 3 as libc::c_int;
    token.subtype = 0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x8 as libc::c_int;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 as libc::c_int as libc::c_long {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_eval
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_evalfloat(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_float = 0.;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_Evaluate(
        source,
        0 as *mut libc::c_long,
        &mut value,
        crate::src::qcommon::q_shared::qfalse as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0 as libc::c_int;
    ::libc::sprintf(
        token.string.as_mut_ptr(),
        b"%1.2f\x00" as *const u8 as *const libc::c_char,
        crate::stdlib::fabs(value as libc::c_double),
    );
    token.type_0 = 3 as libc::c_int;
    token.subtype = 0x800 as libc::c_int | 0x2000 as libc::c_int | 0x8 as libc::c_int;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 as libc::c_int as libc::c_float {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_Directive_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub static mut directives: [directive_t; 20] = {
    [
        {
            let mut init = directive_s {
                name: b"if\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_if
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"ifdef\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_ifdef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"ifndef\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_ifndef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"elif\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_elif
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"else\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_else
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"endif\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_endif
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"include\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_include
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"define\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_define
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"undef\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_undef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"line\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_line
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"error\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_error
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"pragma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_pragma
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"eval\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_eval
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"evalfloat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_Directive_evalfloat
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                func: None,
            };
            init
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
    ]
};
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDirective(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut i: libc::c_int = 0;
    //read the directive name
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"found # without name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //directive name must be on the same line
    if token.linescrossed > 0 as libc::c_int {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"found # at end of line\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if if is a name
    if token.type_0 == 4 as libc::c_int {
        //end if
        //find the precompiler directive
        i = 0 as libc::c_int;
        while !directives[i as usize].name.is_null() {
            if ::libc::strcmp(directives[i as usize].name, token.string.as_mut_ptr()) == 0 {
                return directives[i as usize]
                    .func
                    .expect("non-null function pointer")(source);
            }
            i += 1
            //end if
        }
        //end for
    }
    SourceError(
        source,
        b"unknown precompiler directive %s\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function PC_ReadDirective
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarDirective_evalint(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_long = 0;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_DollarEvaluate(
        source,
        &mut value,
        0 as *mut libc::c_float,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0 as libc::c_int;
    ::libc::sprintf(
        token.string.as_mut_ptr(),
        b"%ld\x00" as *const u8 as *const libc::c_char,
        ::libc::labs(value),
    );
    token.type_0 = 3 as libc::c_int;
    token.subtype = 0x1000 as libc::c_int | 0x2000 as libc::c_int | 0x8 as libc::c_int;
    token.intvalue = ::libc::labs(value) as libc::c_ulong;
    token.floatvalue = token.intvalue as libc::c_float;
    //NUMBERVALUE
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 as libc::c_int as libc::c_long {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_DollarDirective_evalint
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarDirective_evalfloat(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut value: libc::c_float = 0.;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_DollarEvaluate(
        source,
        0 as *mut libc::c_long,
        &mut value,
        crate::src::qcommon::q_shared::qfalse as libc::c_int,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0 as libc::c_int;
    ::libc::sprintf(
        token.string.as_mut_ptr(),
        b"%1.2f\x00" as *const u8 as *const libc::c_char,
        crate::stdlib::fabs(value as libc::c_double),
    );
    token.type_0 = 3 as libc::c_int;
    token.subtype = 0x800 as libc::c_int | 0x2000 as libc::c_int | 0x8 as libc::c_int;
    token.floatvalue = crate::stdlib::fabs(value as libc::c_double) as libc::c_float;
    token.intvalue = token.floatvalue as libc::c_ulong;
    //NUMBERVALUE
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 as libc::c_int as libc::c_float {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_DollarDirective_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub static mut dollardirectives: [directive_t; 20] = {
    [
        {
            let mut init = directive_s {
                name: b"evalint\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_DollarDirective_evalint
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"evalfloat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(
                    PC_DollarDirective_evalfloat
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                func: None,
            };
            init
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
        directive_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            func: None,
        },
    ]
};
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDollarDirective(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut i: libc::c_int = 0;
    //read the directive name
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"found $ without name\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //directive name must be on the same line
    if token.linescrossed > 0 as libc::c_int {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"found $ at end of line\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if if is a name
    if token.type_0 == 4 as libc::c_int {
        //end if
        //find the precompiler directive
        i = 0 as libc::c_int;
        while !dollardirectives[i as usize].name.is_null() {
            if ::libc::strcmp(dollardirectives[i as usize].name, token.string.as_mut_ptr()) == 0 {
                return dollardirectives[i as usize]
                    .func
                    .expect("non-null function pointer")(source);
            }
            i += 1
            //end if
        }
        //end for
    }
    PC_UnreadSourceToken(source, &mut token);
    SourceError(
        source,
        b"unknown precompiler directive %s\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//read a token from the source
//end of the function PC_ReadDirective
//QUAKEC
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    loop {
        if PC_ReadSourceToken(source, token) == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //check for precompiler directives
        if (*token).type_0 == 5 as libc::c_int
            && *(*token).string.as_mut_ptr() as libc::c_int == '#' as i32
        {
            //end if
            //QUAKC
            //read the precompiler directive
            if PC_ReadDirective(source) == 0 {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        //end if
        } else if (*token).type_0 == 5 as libc::c_int
            && *(*token).string.as_mut_ptr() as libc::c_int == '$' as i32
        {
            //end if
            //QUAKEC
            //read the precompiler directive
            if PC_ReadDollarDirective(source) == 0 {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        //end if
        } else {
            // recursively concatenate strings that are behind each other still resolving defines
            if (*token).type_0 == 1 as libc::c_int {
                let mut newtoken: crate::src::botlib::l_script::token_t =
                    crate::src::botlib::l_script::token_t {
                        string: [0; 1024],
                        type_0: 0,
                        subtype: 0,
                        intvalue: 0,
                        floatvalue: 0.,
                        whitespace_p: 0 as *mut libc::c_char,
                        endwhitespace_p: 0 as *mut libc::c_char,
                        line: 0,
                        linescrossed: 0,
                        next: 0 as *mut crate::src::botlib::l_script::token_s,
                    }; //end if
                if PC_ReadToken(source, &mut newtoken) != 0 {
                    if newtoken.type_0 == 1 as libc::c_int {
                        (*token).string[crate::stdlib::strlen((*token).string.as_mut_ptr())
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as usize] = '\u{0}' as i32 as libc::c_char;
                        if crate::stdlib::strlen((*token).string.as_mut_ptr())
                            .wrapping_add(crate::stdlib::strlen(
                                newtoken
                                    .string
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize),
                            ))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            >= 1024 as libc::c_int as libc::c_ulong
                        {
                            SourceError(
                                source,
                                b"string longer than MAX_TOKEN %d\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                1024 as libc::c_int,
                            );
                            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                        }
                        ::libc::strcat(
                            (*token).string.as_mut_ptr(),
                            newtoken
                                .string
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize),
                        );
                    } else {
                        PC_UnreadToken(source, &mut newtoken);
                    }
                }
            }
            //if skipping source because of conditional compilation
            if (*source).skip != 0 {
                continue;
            }
            //if the token is a name
            if (*token).type_0 == 4 as libc::c_int {
                //end if
                //check if the name is a define macro
                define = PC_FindHashedDefine((*source).definehash, (*token).string.as_mut_ptr());
                //end if
                if !define.is_null() {
                    //DEFINEHASHING
                    //if it is a define macro
                    //expand the defined macro
                    if PC_ExpandDefineIntoSource(source, token, define) == 0 {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    continue;
                }
            }
            //copy token for unreading
            crate::stdlib::memcpy(
                &mut (*source).token as *mut crate::src::botlib::l_script::token_t
                    as *mut libc::c_void,
                token as *const libc::c_void,
                ::std::mem::size_of::<crate::src::botlib::l_script::token_t>() as libc::c_ulong,
            );
            //found a token
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    //end while
}
//expect a certain token
//end of the function PC_ReadToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectTokenString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut libc::c_char,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    if PC_ReadToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"couldn\'t find expected %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            string,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if ::libc::strcmp(token.string.as_mut_ptr(), string) != 0 {
        SourceError(
            source,
            b"expected %s, found %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            string,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//expect a certain token type
//end of the function PC_ExpectTokenString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectTokenType(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: libc::c_int,
    mut subtype: libc::c_int,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut str: [libc::c_char; 1024] = [0; 1024]; //end if
    if PC_ReadToken(source, token) == 0 {
        SourceError(
            source,
            b"couldn\'t read expected token\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } //end else if
    if (*token).type_0 != type_0 {
        ::libc::strcpy(
            str.as_mut_ptr(),
            b"\x00" as *const u8 as *const libc::c_char,
        );
        if type_0 == 1 as libc::c_int {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"string\x00" as *const u8 as *const libc::c_char,
            );
        }
        if type_0 == 2 as libc::c_int {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"literal\x00" as *const u8 as *const libc::c_char,
            );
        }
        if type_0 == 3 as libc::c_int {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"number\x00" as *const u8 as *const libc::c_char,
            );
        }
        if type_0 == 4 as libc::c_int {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"name\x00" as *const u8 as *const libc::c_char,
            );
        }
        if type_0 == 5 as libc::c_int {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"punctuation\x00" as *const u8 as *const libc::c_char,
            );
        }
        SourceError(
            source,
            b"expected a %s, found %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            str.as_mut_ptr(),
            (*token).string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if (*token).type_0 == 3 as libc::c_int {
        if (*token).subtype & subtype != subtype {
            ::libc::strcpy(
                str.as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
            );
            if subtype & 0x8 as libc::c_int != 0 {
                ::libc::strcpy(
                    str.as_mut_ptr(),
                    b"decimal\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x100 as libc::c_int != 0 {
                ::libc::strcpy(
                    str.as_mut_ptr(),
                    b"hex\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x200 as libc::c_int != 0 {
                ::libc::strcpy(
                    str.as_mut_ptr(),
                    b"octal\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x400 as libc::c_int != 0 {
                ::libc::strcpy(
                    str.as_mut_ptr(),
                    b"binary\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x2000 as libc::c_int != 0 {
                ::libc::strcat(
                    str.as_mut_ptr(),
                    b" long\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x4000 as libc::c_int != 0 {
                ::libc::strcat(
                    str.as_mut_ptr(),
                    b" unsigned\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x800 as libc::c_int != 0 {
                ::libc::strcat(
                    str.as_mut_ptr(),
                    b" float\x00" as *const u8 as *const libc::c_char,
                );
            }
            if subtype & 0x1000 as libc::c_int != 0 {
                ::libc::strcat(
                    str.as_mut_ptr(),
                    b" integer\x00" as *const u8 as *const libc::c_char,
                );
            }
            SourceError(
                source,
                b"expected %s, found %s\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                str.as_mut_ptr(),
                (*token).string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
    //end if
    } else if (*token).type_0 == 5 as libc::c_int {
        if (*token).subtype != subtype {
            SourceError(
                source,
                b"found %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*token).string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//expect a token
//end of the function PC_ExpectTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectAnyToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    if PC_ReadToken(source, token) == 0 {
        SourceError(
            source,
            b"couldn\'t read expected token\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    } else {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    };
    //end else
}
//returns true when the token is available
//end of the function PC_ExpectAnyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckTokenString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut libc::c_char,
) -> libc::c_int {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_ReadToken(source, &mut tok) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if the token is available
    if ::libc::strcmp(tok.string.as_mut_ptr(), string) == 0 {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    PC_UnreadSourceToken(source, &mut tok);
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//returns true and reads the token when a token with the given type is available
//end of the function PC_CheckTokenString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckTokenType(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: libc::c_int,
    mut subtype: libc::c_int,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> libc::c_int {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_ReadToken(source, &mut tok) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if the type matches
    if tok.type_0 == type_0 && tok.subtype & subtype == subtype {
        crate::stdlib::memcpy(
            token as *mut libc::c_void,
            &mut tok as *mut crate::src::botlib::l_script::token_t as *const libc::c_void,
            ::std::mem::size_of::<crate::src::botlib::l_script::token_t>() as libc::c_ulong,
        ); //end if
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    //
    PC_UnreadSourceToken(source, &mut tok);
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//skip tokens until the given token string is read
//end of the function PC_CheckTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SkipUntilString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut libc::c_char,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end while
    while PC_ReadToken(source, &mut token) != 0 {
        if ::libc::strcmp(token.string.as_mut_ptr(), string) == 0 {
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//unread the last token read from the script
//end of the function PC_SkipUntilString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadLastToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    PC_UnreadSourceToken(source, &mut (*source).token);
}
//unread the given token
//end of the function PC_UnreadLastToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) {
    PC_UnreadSourceToken(source, token);
}
//set the source include path
//end of the function PC_UnreadToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetIncludePath(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut path: *mut libc::c_char,
) {
    let mut len: crate::stddef_h::size_t = 0;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).includepath.as_mut_ptr(),
        path,
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    len = crate::stdlib::strlen((*source).includepath.as_mut_ptr());
    //add trailing path seperator
    if len > 0 as libc::c_int as libc::c_ulong
        && (*source).includepath[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_int
            != '\\' as i32
        && (*source).includepath[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
            as libc::c_int
            != '/' as i32
    {
        ::libc::strcat(
            (*source).includepath.as_mut_ptr(),
            b"/\x00" as *const u8 as *const libc::c_char,
        );
    };
    //end if
}
//set the punction set
//end of the function PC_SetIncludePath
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetPunctuations(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut p: *mut crate::src::botlib::l_script::punctuation_t,
) {
    (*source).punctuations = p;
}
//load a source file
//end of the function PC_SetPunctuations
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadSourceFile(
    mut filename: *const libc::c_char,
) -> *mut crate::src::botlib::l_precomp::source_t {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptFile(filename)
        as *mut crate::src::botlib::l_script::script_s;
    if script.is_null() {
        return 0 as *mut crate::src::botlib::l_precomp::source_t;
    }
    (*script).next = 0 as *mut crate::src::botlib::l_script::script_s;
    source = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::source_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::source_t;
    crate::stdlib::memset(
        source as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>() as libc::c_ulong,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).filename.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut crate::src::botlib::l_script::token_t;
    (*source).defines = 0 as *mut crate::src::botlib::l_precomp::define_t;
    (*source).indentstack = 0 as *mut crate::src::botlib::l_precomp::indent_t;
    (*source).skip = 0 as libc::c_int;
    (*source).definehash = crate::src::botlib::l_memory::GetClearedMemory(
        (1024 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            *mut crate::src::botlib::l_precomp::define_t,
        >() as libc::c_ulong),
    ) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//load a source from memory
//end of the function LoadSourceFile
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadSourceMemory(
    mut ptr: *mut libc::c_char,
    mut length: libc::c_int,
    mut name: *mut libc::c_char,
) -> *mut crate::src::botlib::l_precomp::source_t {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptMemory(ptr, length, name)
        as *mut crate::src::botlib::l_script::script_s;
    if script.is_null() {
        return 0 as *mut crate::src::botlib::l_precomp::source_t;
    }
    (*script).next = 0 as *mut crate::src::botlib::l_script::script_s;
    source = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::source_t,
    >() as libc::c_ulong) as *mut crate::src::botlib::l_precomp::source_t;
    crate::stdlib::memset(
        source as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>() as libc::c_ulong,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).filename.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut crate::src::botlib::l_script::token_t;
    (*source).defines = 0 as *mut crate::src::botlib::l_precomp::define_t;
    (*source).indentstack = 0 as *mut crate::src::botlib::l_precomp::indent_t;
    (*source).skip = 0 as libc::c_int;
    (*source).definehash = crate::src::botlib::l_memory::GetClearedMemory(
        (1024 as libc::c_int as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            *mut crate::src::botlib::l_precomp::define_t,
        >() as libc::c_ulong),
    ) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//free the given source
//end of the function LoadSourceMemory
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeSource(mut source: *mut crate::src::botlib::l_precomp::source_t) {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    let mut i: libc::c_int = 0;
    //PC_PrintDefineHashTable(source->definehash);
    //free all the scripts
    while !(*source).scriptstack.is_null() {
        script = (*source).scriptstack; //end for
        (*source).scriptstack = (*(*source).scriptstack).next;
        crate::src::botlib::l_script::FreeScript(
            script as *mut crate::src::botlib::l_script::script_s,
        );
    }
    //free all the tokens
    while !(*source).tokens.is_null() {
        token = (*source).tokens; //end for
        (*source).tokens = (*(*source).tokens).next; //end for
        PC_FreeToken(token);
    }
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        while !(*(*source).definehash.offset(i as isize)).is_null() {
            define = *(*source).definehash.offset(i as isize);
            let ref mut fresh8 = *(*source).definehash.offset(i as isize);
            *fresh8 = (**(*source).definehash.offset(i as isize)).hashnext;
            PC_FreeDefine(define);
        }
        i += 1
        //end while
    }
    //DEFINEHASHING
    //DEFINEHASHING
    //free all indents
    while !(*source).indentstack.is_null() {
        indent = (*source).indentstack; //end for
        (*source).indentstack = (*(*source).indentstack).next;
        crate::src::botlib::l_memory::FreeMemory(indent as *mut libc::c_void);
    }
    //
    if !(*source).definehash.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*source).definehash as *mut libc::c_void);
    }
    //DEFINEHASHING
    //free the source itself
    crate::src::botlib::l_memory::FreeMemory(source as *mut libc::c_void);
}
#[no_mangle]

pub static mut sourceFiles: [*mut crate::src::botlib::l_precomp::source_t; 64] =
    [0 as *const crate::src::botlib::l_precomp::source_t
        as *mut crate::src::botlib::l_precomp::source_t; 64];
//BSPC
//
#[no_mangle]

pub unsafe extern "C" fn PC_LoadSourceHandle(mut filename: *const libc::c_char) -> libc::c_int {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t; //end for
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 64 as libc::c_int {
        if sourceFiles[i as usize].is_null() {
            break;
        }
        i += 1
    }
    if i >= 64 as libc::c_int {
        return 0 as libc::c_int;
    }
    crate::src::botlib::l_script::PS_SetBaseFolder(
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    source = LoadSourceFile(filename);
    if source.is_null() {
        return 0 as libc::c_int;
    }
    sourceFiles[i as usize] = source;
    return i;
}
//end of the function PC_LoadSourceHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeSourceHandle(mut handle: libc::c_int) -> libc::c_int {
    if handle < 1 as libc::c_int || handle >= 64 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if sourceFiles[handle as usize].is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    FreeSource(sourceFiles[handle as usize]);
    sourceFiles[handle as usize] = 0 as *mut crate::src::botlib::l_precomp::source_t;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function PC_FreeSourceHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadTokenHandle(
    mut handle: libc::c_int,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> libc::c_int {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut ret: libc::c_int = 0;
    if handle < 1 as libc::c_int || handle >= 64 as libc::c_int {
        return 0 as libc::c_int;
    }
    if sourceFiles[handle as usize].is_null() {
        return 0 as libc::c_int;
    }
    ret = PC_ReadToken(sourceFiles[handle as usize], &mut token);
    ::libc::strcpy((*pc_token).string.as_mut_ptr(), token.string.as_mut_ptr());
    (*pc_token).type_0 = token.type_0;
    (*pc_token).subtype = token.subtype;
    (*pc_token).intvalue = token.intvalue as libc::c_int;
    (*pc_token).floatvalue = token.floatvalue;
    if (*pc_token).type_0 == 1 as libc::c_int {
        crate::src::botlib::l_script::StripDoubleQuotes((*pc_token).string.as_mut_ptr());
    }
    return ret;
}
//end of the function PC_ReadTokenHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SourceFileAndLine(
    mut handle: libc::c_int,
    mut filename: *mut libc::c_char,
    mut line: *mut libc::c_int,
) -> libc::c_int {
    if handle < 1 as libc::c_int || handle >= 64 as libc::c_int {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if sourceFiles[handle as usize].is_null() {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    ::libc::strcpy(
        filename,
        (*sourceFiles[handle as usize]).filename.as_mut_ptr(),
    );
    if !(*sourceFiles[handle as usize]).scriptstack.is_null() {
        *line = (*(*sourceFiles[handle as usize]).scriptstack).line
    } else {
        *line = 0 as libc::c_int
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//set the base folder to load files from
//end of the function PC_SourceFileAndLine
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetBaseFolder(mut path: *mut libc::c_char) {
    crate::src::botlib::l_script::PS_SetBaseFolder(path);
}
//end of the function PC_SetBaseFolder
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckOpenSourceHandles() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 64 as libc::c_int {
        if !sourceFiles[i as usize].is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"file %s still open in precompiler\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*(*sourceFiles[i as usize]).scriptstack)
                    .filename
                    .as_mut_ptr(),
            );
            //BOTLIB
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function PC_CheckOpenSourceHandles
